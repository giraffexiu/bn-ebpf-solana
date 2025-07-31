import os
import json
import asyncio
from binaryninja import (
    Settings,
    BackgroundTaskThread,
    execute_on_main_thread
)
import binaryninja as bn
# Use Binary Ninja's built-in Qt without importing binaryninjaui to avoid version conflicts
from PySide6.QtCore import Qt, QRectF
from PySide6.QtGui import QImage, QPainter, QFont, QColor
from PySide6.QtWidgets import QTextEdit, QVBoxLayout, QPushButton, QHBoxLayout, QFileDialog, QMessageBox, QTabWidget, QWidget, QLabel, QComboBox, QProgressBar
from binaryninjaui import (
    SidebarWidget, SidebarWidgetType, Sidebar, UIActionHandler,
    SidebarWidgetLocation, SidebarContextSensitivity
)
from pygments import highlight
from pygments.lexers import RustLexer
from pygments.formatters import HtmlFormatter
from typing import Any, Dict
# Removed Claude/Anthropic imports - using Gemini only
from google import genai
from google.genai import types as genai_types

import sys


from .direct_api_interface import DirectAPIInterface
from .batch_rust_decompiler import CacheManager, BatchRustDecompilerWorker

# Fix stdout/stderr for fastmcp compatibility
def _safe_fd():
    return getattr(_safe_fd, "fd", os.open(os.devnull, os.O_RDWR))

for stream_name in ("stdout", "stderr"):
    stream = getattr(sys, stream_name, None)
    if stream is not None and not hasattr(stream, "fileno"):
        stream.fileno = _safe_fd  


from fastmcp.client import Client
from fastmcp.client.transports import PythonStdioTransport
from pathlib import Path

import zlib, sys
from solders.pubkey import Pubkey
from solana.rpc.async_api import AsyncClient

from .mcp_utils import *

DEFAULT_RPC = "https://api.mainnet-beta.solana.com"


# Settings configuration

settings = Settings()
# Simplified settings - Gemini only
setting_props_context = f'{{"title" : "Context for Solana MCP", "description" : "Absolute path to extra context to be provided, like an IDL", "type" : "string", "default" : ""}}'
setting_props_gemini = f'{{"title" : "Gemini API Key", "description" : "Your Google Gemini API key for LLM requests", "type" : "string", "default" : ""}}'

settings.register_group("bn-ebpf-solana", "Gemini Decompiler Settings")
settings.register_setting(
    "bn-ebpf-solana.context",
    setting_props_context
)
settings.register_setting(
    "bn-ebpf-solana.gemini_api_key",
    setting_props_gemini
)


class LLMRunner(BackgroundTaskThread):
    def __init__(self, bar):
        provider = settings.get_string("bn-ebpf-solana.llm_provider")
        super().__init__(f"Prompting {provider.title()} for Rust (running)…", can_cancel=True)
        self.bar = bar
        self.func        = bar.f
        self.provider = provider
        self._cancelled = False
    
    def cancel(self):
        """Cancel the current task"""
        self._cancelled = True
        super().cancel()
    
    def is_cancelled(self):
        """Check if the task has been cancelled"""
        return self._cancelled
    
    def cleanup_task(self):
        """Clean up task state in the sidebar widget"""
        if self.bar.current_task == self:
            self.bar.current_task = None
            self.bar.pending_function = None

# Claude runner removed - using Gemini only

# Claude extract_rust method removed - using Gemini only

# Claude API call method removed - using Gemini only

class IDLExtractorRunner(BackgroundTaskThread):
    """Background task for extracting IDL data from Solana programs"""
    
    def __init__(self, sidebar_widget, output_dir, selected_model="gemini-2.5-flash"):
        super().__init__("Extracting IDL Data", True)
        self.sidebar_widget = sidebar_widget
        self.output_dir = output_dir
        self.selected_model = selected_model
        self.bv = sidebar_widget.bv
        self._cancelled = False
    
    def cancel(self):
        """Cancel the current task"""
        self._cancelled = True
        super().cancel()
    
    def is_cancelled(self):
        """Check if the task has been cancelled"""
        return self._cancelled
    
    def run(self):
        """Main execution method for IDL data extraction"""
        try:
            # Step 1: Generate Rust decompilation
            self.progress = "Step 1/2: Generating Rust decompilation..."
            self._update_status("Step 1/2: Generating Rust decompilation...")
            rust_code = self._generate_rust_code()
            
            if self._cancelled:
                return
            
            # Step 2: Extract IDL data using Gemini AI
            self.progress = "Step 2/2: Extracting IDL data..."
            self._update_status(f"Step 2/2: Extracting IDL with {self.selected_model}...")
            idl_data = asyncio.run(self._extract_idl_data(rust_code))
            
            if self._cancelled:
                return
            
            # Write IDL data to output directory
            self._write_idl_data(idl_data)
            
            # Notify completion
            execute_on_main_thread(lambda: self._on_completion_success())
            
        except Exception as e:
            import traceback
            error_msg = f"IDL extraction failed: {str(e)}"
            print(f"[DEBUG] Full error traceback: {traceback.format_exc()}")
            execute_on_main_thread(lambda: self._on_completion_error(error_msg))
    
    def _generate_rust_code(self):
        """Generate Rust decompilation using batch decompiler"""
        from .batch_rust_decompiler import BatchRustDecompilerWorker
        
        # Create temporary worker for batch decompilation
        worker = BatchRustDecompilerWorker(self.bv, use_cache=True)
        
        # Run synchronously in this thread
        worker.run()
        
        if self._cancelled:
            return None
        
        # Get results
        rust_results = worker.results
        
        # Combine all Rust code
        combined_rust = "\n\n".join([
            f"// Function: {name}\n{code}" 
            for name, code in rust_results.items() 
            if not code.startswith("Error:")
        ])
        
        return combined_rust
    

    
    async def _extract_idl_data(self, rust_code):
        """Use Gemini AI to extract IDL data from Rust code"""
        api_key = settings.get_string("bn-ebpf-solana.gemini_api_key")
        
        if not api_key:
            raise Exception("Gemini API key not configured. Please set it in Binary Ninja settings.")
        
        try:
            gemini_client = genai.Client(api_key=api_key)
        except Exception as e:
            raise Exception(f"Failed to create Gemini client: {e}")
        
        # Truncate code if too long
        max_code_length = 100000  # Much smaller limit for IDL extraction
        if len(rust_code) > max_code_length:
            rust_code = rust_code[:max_code_length] + "\n// ... (code truncated)"
        
        # Simple prompt focused on IDL extraction
        prompt_content = f"""Analyze this Solana program Rust code and extract IDL (Interface Definition Language) data:

{rust_code}

Extract and return ONLY a JSON object with the following structure:
{{
  "name": "program_name",
  "version": "0.1.0",
  "instructions": [
    {{
      "name": "instruction_name",
      "accounts": [
        {{
          "name": "account_name",
          "isMut": True,
          "isSigner": False
        }}
      ],
      "args": [
        {{
          "name": "arg_name",
          "type": "u64"
        }}
      ]
    }}
  ],
  "accounts": [
    {{
      "name": "AccountStruct",
      "type": {{
        "kind": "struct",
        "fields": [
          {{
            "name": "field_name",
            "type": "u64"
          }}
        ]
      }}
    }}
  ],
  "errors": [
    {{
      "code": 6000,
      "name": "ErrorName",
      "msg": "Error message"
    }}
  ]
}}

Focus on extracting:
1. Program instructions and their parameters
2. Account structures and their fields
3. Error codes and messages
4. Data types used

Return ONLY the JSON, no explanations."""
        
        try:
            config = genai_types.GenerateContentConfig(
                temperature=0.1,
                max_output_tokens=8000
            )
            
            response = await gemini_client.aio.models.generate_content(
                model=self.selected_model,
                contents=prompt_content,
                config=config
            )
            
            if not response.text:
                raise Exception("No response text generated from Gemini")
            
            # Parse JSON response
            response_text = response.text.strip()
            
            # Remove markdown code blocks if present
            if "```json" in response_text:
                start_idx = response_text.index("```json") + 7
                end_idx = response_text.find("```", start_idx)
                if end_idx != -1:
                    response_text = response_text[start_idx:end_idx].strip()
            elif "```" in response_text:
                start_idx = response_text.index("```") + 3
                end_idx = response_text.find("```", start_idx)
                if end_idx != -1:
                    response_text = response_text[start_idx:end_idx].strip()
            
            # Find JSON object boundaries
            start_brace = response_text.find('{')
            if start_brace == -1:
                raise json.JSONDecodeError("No opening brace found", response_text, 0)
            
            # Find the matching closing brace
            brace_count = 0
            end_brace = -1
            for i in range(start_brace, len(response_text)):
                if response_text[i] == '{':
                    brace_count += 1
                elif response_text[i] == '}':
                    brace_count -= 1
                    if brace_count == 0:
                        end_brace = i
                        break
            
            if end_brace == -1:
                raise json.JSONDecodeError("No matching closing brace found", response_text, start_brace)
            
            # Extract and parse JSON
            json_text = response_text[start_brace:end_brace + 1]
            idl_data = json.loads(json_text)
            
            print(f"[DEBUG] Successfully extracted IDL data")
            return idl_data
            
        except Exception as e:
            print(f"[DEBUG] IDL extraction failed: {e}")
            # Return fallback IDL structure
            return {
                "name": "extracted_program",
                "version": "0.1.0",
                "instructions": [
                    {
                        "name": "initialize",
                        "accounts": [
                            {
                                "name": "user",
                                "isMut": False,
                                "isSigner": True
                            }
                        ],
                        "args": []
                    }
                ],
                "accounts": [],
                "errors": []
            }
    
    def _write_idl_data(self, idl_data):
        """Write extracted IDL data to output directory"""
        import os
        from pathlib import Path
        
        print(f"[DEBUG] _write_idl_data called with idl_data type: {type(idl_data)}")
        print(f"[DEBUG] idl_data content: {idl_data}")
        
        output_path = Path(self.output_dir)
        
        # Create IDL output directory
        idl_dir = output_path / "extracted_idl"
        idl_dir.mkdir(exist_ok=True)
        
        # Build output preview content
        preview_content = f"🔍 IDL Extraction Results:\n"
        preview_content += f"📂 {idl_dir}\n\n"
        
        # Write IDL JSON file
        idl_file = idl_dir / "program.json"
        with open(idl_file, 'w', encoding='utf-8') as f:
            json.dump(idl_data, f, indent=2)
        
        # Add IDL summary to preview
        preview_content += "📊 IDL Summary:\n"
        preview_content += "═" * 50 + "\n"
        
        if "name" in idl_data:
            preview_content += f"📦 Program Name: {idl_data['name']}\n"
        
        if "version" in idl_data:
            preview_content += f"🏷️  Version: {idl_data['version']}\n"
        
        if "instructions" in idl_data:
            instructions = idl_data["instructions"]
            preview_content += f"⚡ Instructions: {len(instructions)}\n"
            for i, instruction in enumerate(instructions[:5]):  # Show first 5
                preview_content += f"   • {instruction.get('name', 'unknown')}\n"
                if "accounts" in instruction:
                    preview_content += f"     Accounts: {len(instruction['accounts'])}\n"
                if "args" in instruction:
                    preview_content += f"     Arguments: {len(instruction['args'])}\n"
            if len(instructions) > 5:
                preview_content += f"   ... and {len(instructions) - 5} more\n"
        
        if "accounts" in idl_data:
            accounts = idl_data["accounts"]
            preview_content += f"🏦 Account Types: {len(accounts)}\n"
            for account in accounts[:3]:  # Show first 3
                preview_content += f"   • {account.get('name', 'unknown')}\n"
            if len(accounts) > 3:
                preview_content += f"   ... and {len(accounts) - 3} more\n"
        
        if "errors" in idl_data:
            errors = idl_data["errors"]
            preview_content += f"❌ Error Codes: {len(errors)}\n"
            for error in errors[:3]:  # Show first 3
                preview_content += f"   • {error.get('name', 'unknown')} ({error.get('code', 'N/A')})\n"
            if len(errors) > 3:
                preview_content += f"   ... and {len(errors) - 3} more\n"
        
        preview_content += "\n"
        
        # Show IDL file content preview
        preview_content += "📄 IDL File Content:\n"
        preview_content += "═" * 50 + "\n"
        
        # Show formatted JSON (first 20 lines)
        json_lines = json.dumps(idl_data, indent=2).split('\n')[:20]
        preview_content += '\n'.join(json_lines)
        if len(json.dumps(idl_data, indent=2).split('\n')) > 20:
            preview_content += "\n... (truncated)"
        
        preview_content += f"\n\n📁 IDL file saved to: {idl_file}\n"
        
        # Update output preview
        self._update_output_preview(preview_content)
        
        self.final_project_path = str(idl_dir)
    

    
    def _update_status(self, message):
        """Update status label on main thread"""
        def update_ui():
            self.sidebar_widget.status_label.setText(message)
        execute_on_main_thread(update_ui)
    
    def _update_output_preview(self, content):
        """Update output preview on main thread"""
        def update_ui():
            self.sidebar_widget.output_preview.setPlainText(content)
        execute_on_main_thread(update_ui)
    
    def _on_completion_success(self):
        """Handle successful completion"""
        self.sidebar_widget.idl_button.setEnabled(True)
        self.sidebar_widget.idl_button.setText("Extract IDL Data")
        self.sidebar_widget.progress_bar.setVisible(False)
        self.sidebar_widget.status_label.setText("✅ IDL data extracted successfully!")
        
        QMessageBox.information(
            None, 
            "Success", 
            f"IDL data extracted successfully!\n\nOutput location: {self.final_project_path}"
        )
    
    def _on_completion_error(self, error_msg):
        """Handle error completion"""
        self.sidebar_widget.idl_button.setEnabled(True)
        self.sidebar_widget.idl_button.setText("Extract IDL Data")
        self.sidebar_widget.progress_bar.setVisible(False)
        self.sidebar_widget.status_label.setText(f"❌ Error: {error_msg}")
        
        QMessageBox.critical(None, "Error", error_msg)

class GeminiRunner(LLMRunner):
    def __init__(self, bar, selected_model="gemini-2.5-flash"):
        super().__init__(bar)
        self.selected_model = selected_model

    def run(self):
        """BackgroundTaskThread entry-point for Gemini."""
        pretty_text = ""
        func_name = self.func.name if self.func else "unknown"
        print(f"[DEBUG] GeminiRunner.run() started for function: {func_name}")

        try:
            if self.is_cancelled():
                print(f"[DEBUG] Task cancelled before starting for function: {func_name}")
                self.cleanup_task()
                return
                
            print(f"[DEBUG] Starting async Gemini extraction for function: {func_name}")
            # actual async work
            pretty_text = asyncio.run(self._extract_rust_gemini())
            
            if self.is_cancelled():
                print(f"[DEBUG] Task cancelled after extraction for function: {func_name}")
                self.cleanup_task()
                return

            print(f"[DEBUG] Gemini extraction completed for function: {func_name}, result length: {len(pretty_text)}")
            if pretty_text and len(pretty_text) > 50:
                print(f"[DEBUG] First 100 chars of result: {pretty_text[:100]}...")

        # ───── expected user-side failures ───────────────────────────────────
        except Exception as exc:
            print(f"[DEBUG] Exception in GeminiRunner for function {func_name}: {type(exc).__name__}: {exc}")
            if "API_KEY" in str(exc) or "authentication" in str(exc).lower():
                pretty_text = f"LLM disabled: bad Gemini API key ({exc})"
            elif "mcp" in str(exc).lower():
                pretty_text = f"LLM disabled: MCP bridge error ({exc})"
            else:
                pretty_text = f"LLM disabled: {type(exc).__name__}: {exc}"

        # ───── always update the sidebar on UI thread ───────────────────────
        finally:
            if not self.is_cancelled():
                print(f"[DEBUG] Updating UI for function: {func_name}")
                execute_on_main_thread(
                    lambda: self.bar.update_ui_func(self.func, pretty_text)
                )
            else:
                print(f"[DEBUG] Task was cancelled, not updating UI for function: {func_name}")
            self.cleanup_task()

    async def _extract_rust_gemini(self) -> str:
        func_name = self.func.name if self.func else "unknown"
        print(f"[DEBUG] _extract_rust_gemini() started for function: {func_name}")
        
        if not self.func:
            print(f"[DEBUG] No function provided, returning empty string")
            return ""

        # Check for cancellation
        if self.is_cancelled():
            print(f"[DEBUG] Task cancelled at start of _extract_rust_gemini for function: {func_name}")
            return "Task cancelled"

        api_key = settings.get_string("bn-ebpf-solana.gemini_api_key")
        print(f"[DEBUG] API key configured: {bool(api_key)}")

        if not api_key:
            print(f"[DEBUG] No Gemini API key found in settings")
            return "Please set your Gemini API key in the plugin settings"

        try:
            gemini_client = genai.Client(api_key=api_key)
            print(f"[DEBUG] Gemini client created successfully")
        except Exception as e:
            print(f"[DEBUG] Failed to create Gemini client: {e}")
            return f"Failed to create Gemini client: {e}"

        # Get extra context
        extra_context_path = settings.get_string("bn-ebpf-solana.context")
        extra_context = ""
        if extra_context_path != "":
            try:
                with open(extra_context_path) as f:
                    extra_context = f.read()
                print(f"[DEBUG] Extra context loaded from: {extra_context_path}")
            except Exception as e:
                print(f"[DEBUG] Failed to load extra context: {e}")

        try:
            system_prompt = open(Path(__file__).parent / "system.txt").read() + extra_context
            print(f"[DEBUG] System prompt loaded, length: {len(system_prompt)}")
        except Exception as e:
            print(f"[DEBUG] Failed to load system prompt: {e}")
            system_prompt = "You are a helpful assistant for decompiling binary code to Rust."

        # Build the prompt
        prompt_content = f"Improve the quality of decompilation inside binary ninja of {self.func.name} using all tools at your disposal"
        
        print(f"[DEBUG] Prompt content length: {len(prompt_content)}")

        # Use direct API interface
        api_interface = DirectAPIInterface(self.bar.bv)
        print(f"[DEBUG] DirectAPIInterface created")
        
        # Check for cancellation before API call
        if self.is_cancelled():
            print(f"[DEBUG] Task cancelled before API call for function: {func_name}")
            return "Task cancelled"
        
        print(f"[DEBUG] Making Gemini API call for function: {func_name}")
        try:
            # Use direct API with Gemini
            # Optimize max_output_tokens based on model capabilities
            # Both Gemini 2.5 Flash and Pro support up to 65,535 output tokens
            max_tokens = 8000 if "flash" in self.selected_model.lower() else 12000
            
            response = await gemini_client.aio.models.generate_content(
                model=self.selected_model,
                contents=prompt_content,
                config=genai_types.GenerateContentConfig(
                    system_instruction=system_prompt,
                    temperature=0.3,
                    max_output_tokens=max_tokens
                )
            )
            print(f"[DEBUG] Gemini API call completed for function: {func_name}")
        except Exception as e:
            print(f"[DEBUG] Gemini API call failed for function {func_name}: {type(e).__name__}: {e}")
            return f"Gemini API call failed: {e}"
        
        # Check for cancellation after API call
        if self.is_cancelled():
            print(f"[DEBUG] Task cancelled after API call for function: {func_name}")
            return "Task cancelled"
        
        # Extract text from response
        if hasattr(response, 'text') and response.text:
            final = response.text
            print(f"[DEBUG] Response received, length: {len(final)}")
            print(f"[DEBUG] Response preview: {final[:200]}...")
            
            # Clean up response similar to Claude
            if "```" in final:
                print(f"[DEBUG] Found code blocks in response")
                # Extract code content without the ``` markers
                start_idx = final.index("```")
                # Find the end of the opening ``` line
                newline_idx = final.find("\n", start_idx)
                if newline_idx != -1:
                    code_start = newline_idx + 1
                    # Find the closing ```
                    end_idx = final.find("```", code_start)
                    if end_idx != -1:
                        extracted = final[code_start:end_idx].strip()
                        print(f"[DEBUG] Extracted code from markdown blocks, length: {len(extracted)}")
                        return extracted
                    else:
                        extracted = final[code_start:].strip()
                        print(f"[DEBUG] Extracted code from start to end, length: {len(extracted)}")
                        return extracted
                else:
                    extracted = final[start_idx + 3:].strip()
                    print(f"[DEBUG] Extracted code after ```, length: {len(extracted)}")
                    return extracted
            if "pub fn" in final:
                extracted = final[final.index("pub fn"):]
                print(f"[DEBUG] Extracted from 'pub fn', length: {len(extracted)}")
                return extracted
            print(f"[DEBUG] Returning full response as no code blocks found")
            return final
        else:
            print(f"[DEBUG] No text response from Gemini API")
            return "No response generated from Gemini"

class LLMDecompSidebarWidget(SidebarWidget):


    def detect_id(self):
        # analyze the entry func
        for function in self.bv.functions:
            if function.name.endswith("::entry") and "DebugList" not in function.name:
                pass  # IDL detection functionality removed


    def __init__(self, name, frame, data):
        super().__init__(name)
        self._formatter = HtmlFormatter(
            style="native",      
            noclasses=True,
            cssstyles="""
              background: transparent;
              font-family: monospace;
              /* override any default pre/span backgrounds: */
              pre { background: transparent !important; }
              span { background: transparent !important; }
            """
        )
        self.actionHandler = UIActionHandler()
        self.actionHandler.setupActionHandler(self)

        # Keep references for updates
        self.frame = frame
        self.bv = data

        #current cursor
        self.here = 0
        #current function
        self.f = None

        #cache of llm responses using CacheManager
        self.cache_manager = CacheManager()
        self.cache = {}  # Keep memory cache for quick access
        
        # Task management to prevent multiple concurrent LLM requests
        self.current_task = None
        self.pending_function = None

        # Create tab widget for two main functionalities
        self.tab_widget = QTabWidget()
        
        # ═══════════════════════════════════════════════════════════════════════
        # Tab 1: Cursor-based Decompilation (Original functionality)
        # ═══════════════════════════════════════════════════════════════════════
        self.decompile_tab = QWidget()
        
        # The editor will render HTML for decompiled code
        self.editor = QTextEdit()
        self.editor.setReadOnly(False)
        self.editor.setStyleSheet("""
            background:transparent;
            color:inherit;
            font-family: monospace;
        """)
        self.editor.setFrameStyle(0)
        
        # Layout for decompilation tab
        decompile_layout = QVBoxLayout()
        decompile_layout.addWidget(QLabel("Cursor-based Rust Decompilation:"))
        decompile_layout.addWidget(self.editor)
        self.decompile_tab.setLayout(decompile_layout)
        
        # ═══════════════════════════════════════════════════════════════════════
        # Tab 2: IDL Data Extraction
        # ═══════════════════════════════════════════════════════════════════════
        self.idl_tab = QWidget()
        
        # Model selection
        model_layout = QHBoxLayout()
        model_layout.addWidget(QLabel("Model:"))
        self.model_combo = QComboBox()
        self.model_combo.addItems(["gemini-2.5-flash", "gemini-2.5-pro"])
        self.model_combo.setCurrentText("gemini-2.5-flash")
        model_layout.addWidget(self.model_combo)
        model_layout.addStretch()
        
        # Extract button
        self.idl_button = QPushButton("Extract IDL Data")
        self.idl_button.clicked.connect(self._extract_idl_data_ui)
        self.idl_button.setToolTip("Extract IDL (Interface Definition Language) data from decompiled Solana program")
        
        # Output preview area
        self.output_preview = QTextEdit()
        self.output_preview.setReadOnly(True)
        self.output_preview.setStyleSheet("""
            background: #2b2b2b;
            color: #ffffff;
            font-family: monospace;
            border: 1px solid #555;
        """)
        self.output_preview.setPlaceholderText("Extracted IDL data will appear here...")
        
        # Progress bar
        self.progress_bar = QProgressBar()
        self.progress_bar.setVisible(False)
        
        # Status label
        self.status_label = QLabel("Ready to extract IDL data")
        self.status_label.setStyleSheet("color: #888; font-style: italic;")
        
        # Layout for IDL tab
        idl_layout = QVBoxLayout()
        idl_layout.addWidget(QLabel("IDL Data Extraction:"))
        idl_layout.addLayout(model_layout)
        idl_layout.addWidget(self.idl_button)
        idl_layout.addWidget(self.progress_bar)
        idl_layout.addWidget(self.status_label)
        idl_layout.addWidget(QLabel("Output Preview:"))
        idl_layout.addWidget(self.output_preview)
        self.idl_tab.setLayout(idl_layout)
        
        # ═══════════════════════════════════════════════════════════════════════
        # Add tabs to tab widget
        # ═══════════════════════════════════════════════════════════════════════
        self.tab_widget.addTab(self.decompile_tab, "Decompile")
        self.tab_widget.addTab(self.idl_tab, "IDL Extraction")
        
        # Main layout
        layout = QVBoxLayout()
        layout.addWidget(self.tab_widget)
        self.setLayout(layout)

        # If we already have a BinaryView, render immediately
        if self.bv:
            self._update()

    def notifyViewChanged(self, view_frame):
        # Called when the user opens or switches tabs
        self.frame = view_frame
        if view_frame is None:
            self.bv = None
            self.editor.setHtml("")
        else:
            iface = view_frame.getCurrentViewInterface()
            self.bv = iface.getData()
            # IDL detection functionality removed
            self._update()

    # if the user moves around in the binary view, update UI
    def notifyOffsetChanged(self, offset):
        # Called as the cursor moves
        self._update()

    def update_ui_func(self, func, source):
        print(f"[DEBUG] update_ui_func called for function: {func.name}, source length: {len(source)}")
        
        # Cache in both memory and persistent storage
        if func.name not in self.cache:
            self.cache[func.name] = source
            # Also cache to persistent storage if we have a binary view
            if hasattr(self, 'bv') and self.bv:
                self.cache_manager.cache_result(self.bv, func, source)
            print(f"[DEBUG] Cached result for function: {func.name}")
        else:
            print(f"[DEBUG] Function {func.name} already in cache, updating anyway")

        if source and len(source) > 10:
            print(f"[DEBUG] Source preview: {source[:100]}...")
            try:
                highlighted = highlight(source, RustLexer(), self._formatter)
                self.editor.setHtml(highlighted)
                print(f"[DEBUG] UI updated successfully for function: {func.name}")
            except Exception as e:
                print(f"[DEBUG] Failed to highlight code for function {func.name}: {e}")
                self.editor.setHtml(f"<pre>{source}</pre>")
        else:
            print(f"[DEBUG] Source is empty or too short for function: {func.name}")
            self.editor.setHtml(f"<pre>{source}</pre>")

    def _update(self):
        """
        Refresh sidebar whenever the cursor moves or the view changes.

        Fixes:
        ▸ Guard against cases where the current address is NOT inside any
          discovered function (bn.get_functions_containing returns []).
        ▸ Avoid repeated work if we're still in the same function.
        ▸ Prevent multiple concurrent LLM requests that cause UI freezing.
        """
        # ── Early-outs ────────────────────────────────────────────────────────────
        if not self.frame or not self.bv:
            return

        view = self.frame.getCurrentViewInterface()
        addr = view.getCurrentOffset()

        if addr == self.here:               # cursor hasn't moved
            return
        self.here = addr

        # ── Robust lookup ────────────────────────────────────────────────────────
        funcs = self.bv.get_functions_containing(addr)
        if not funcs:
            # Cursor is in padding / header / data region → just clear pane.
            self.editor.setHtml("")
            self.f = None
            # Cancel any pending task
            if self.current_task and self.current_task.is_alive():
                self.current_task.cancel()
                self.current_task = None
            self.pending_function = None
            return

        func = funcs[0]
        if self.f == func:                  # already displaying this function
            return
        self.f = func

        # ── Cached? ──────────────────────────────────────────────────────────────
        # First check memory cache
        if func.name in self.cache:
            self.update_ui_func(func, self.cache[func.name])
            return
        
        # Then check persistent cache
        cached_result = self.cache_manager.get_cached_result(self.bv, func)
        if cached_result:
            print(f"[DEBUG] Found persistent cache for function: {func.name}")
            self.cache[func.name] = cached_result  # Also store in memory cache
            self.update_ui_func(func, cached_result)
            return

        # ── Task management: prevent multiple concurrent requests ──────────────────
        # If there's already a task running, check if it's for the same function
        if self.current_task and self.current_task.is_alive():
            if self.pending_function == func:
                # Already processing this function, just wait
                return
            else:
                # Cancel the current task and start a new one
                self.current_task.cancel()
                self.current_task = None

        # ── Kick off background LLM job ───────────────────────────────────────
        self.pending_function = func
        print(f"[DEBUG] Starting Gemini decompilation for function: {func.name}")
        # Get selected model from UI
        selected_model = self.model_combo.currentText()
        task = GeminiRunner(self, selected_model)
        
        self.current_task = task
        task.start()

    def _extract_idl_data_ui(self):
        """Extract IDL data from decompiled Solana program"""
        if not self.bv:
            QMessageBox.warning(None, "Warning", "No binary view available")
            return
            
        # Check API key
        api_key = settings.get_string("bn-ebpf-solana.gemini_api_key")
        if not api_key:
            QMessageBox.warning(None, "API Key Required", 
                              "Please configure your Gemini API key in the plugin settings.")
            return
        
        # Step 1: Choose output directory
        output_dir = QFileDialog.getExistingDirectory(
            None, 
            "Select Output Directory for IDL Data",
            os.path.expanduser("~")
        )
        
        if not output_dir:
            return
            
        # Get selected model
        selected_model = self.model_combo.currentText()
        
        # Update UI state
        self.idl_button.setEnabled(False)
        self.idl_button.setText("Extracting...")
        self.progress_bar.setVisible(True)
        self.progress_bar.setRange(0, 0)  # Indeterminate progress
        self.status_label.setText(f"Starting IDL extraction with {selected_model}...")
        self.output_preview.clear()
        
        # Start the IDL extraction process in background
        task = IDLExtractorRunner(self, output_dir, selected_model)
        task.start()

    def contextMenuEvent(self, event):
        # if you want a right-click menu later
        self.m_contextMenuManager.show(self.m_menu, event.globalPos())


class LLMDecompSidebarWidgetType(SidebarWidgetType):
    def __init__(self):
        # Icons are 28×28 points; use 56×56px for HiDPI
        icon = QImage(56, 56, QImage.Format_RGB32)
        icon.fill(0)

        painter = QPainter(icon)
        painter.setFont(QFont("Open Sans", 56))
        painter.setPen(QColor(255, 255, 255, 255))
        # R for Rust!
        painter.drawText(QRectF(0, 0, 56, 56), Qt.AlignCenter, "R")
        painter.end()

        super().__init__(icon, "AI reconstructed Rust (Gemini)")

    def createWidget(self, frame, data):
        # frame: a ViewFrameRef, data: BinaryViewRef or None
        return LLMDecompSidebarWidget("LLM reconstructed Rust", frame, data)

    def defaultLocation(self):
        return SidebarWidgetLocation.RightContent

    def contextSensitivity(self):
        return SidebarContextSensitivity.SelfManagedSidebarContext

# Register the sidebar widget type
Sidebar.addSidebarWidgetType(LLMDecompSidebarWidgetType())
