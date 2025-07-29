import os
from binaryninja import (
    Settings,
    BackgroundTaskThread,
    execute_on_main_thread
)
# Use Binary Ninja's built-in Qt without importing binaryninjaui to avoid version conflicts
from PySide6.QtCore import Qt, QRectF
from PySide6.QtGui import QImage, QPainter, QFont, QColor
from PySide6.QtWidgets import QTextEdit, QVBoxLayout
from binaryninjaui import (
    SidebarWidget, SidebarWidgetType, Sidebar, UIActionHandler,
    SidebarWidgetLocation, SidebarContextSensitivity
)
from pygments import highlight
from pygments.lexers import RustLexer
from pygments.formatters import HtmlFormatter
import asyncio, json, os
import asyncio, json
from typing import Any, Dict
# Removed Claude/Anthropic imports - using Gemini only
from google import genai
from google.genai import types as genai_types

import sys, os

from .idl_utils import fetch_idl_anchorpy
from .direct_api_interface import DirectAPIInterface

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
        self.idl = bar.idl
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

class GeminiRunner(LLMRunner):
    def __init__(self, bar):
        super().__init__(bar)

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
        if self.idl:
            prompt_content += f"\n\nHere is the IDL interface file: {self.idl}"
            print(f"[DEBUG] IDL included in prompt")
        
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
            response = await gemini_client.aio.models.generate_content(
                model="gemini-2.5-flash",
                contents=prompt_content,
                config=genai_types.GenerateContentConfig(
                    system_instruction=system_prompt,
                    temperature=0.3,
                    max_output_tokens=1200
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
                self.idl = fetch_idl_anchorpy(self.bv, function)


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
        self.idl = None

        #current cursor
        self.here = 0
        #current function
        self.f = None

        #cache of llm responses
        self.cache = {}
        
        # Task management to prevent multiple concurrent LLM requests
        self.current_task = None
        self.pending_function = None

        # The editor will render HTML
        self.editor = QTextEdit()
        self.editor.setReadOnly(False)

        self.editor.setStyleSheet("""
            background:transparent;
            color:inherit;
            font-family: monospace;
        """)
        self.editor.setFrameStyle(0)

        layout = QVBoxLayout()
        layout.addWidget(self.editor)
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
            if self.idl is None:
                self.detect_id()
            self._update()

    # if the user moves around in the binary view, update UI
    def notifyOffsetChanged(self, offset):
        # Called as the cursor moves
        self._update()

    def update_ui_func(self, func, source):
        print(f"[DEBUG] update_ui_func called for function: {func.name}, source length: {len(source)}")
        
        if func.name not in self.cache:
            self.cache[func.name] = source
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
        if func.name in self.cache:
            self.update_ui_func(func, self.cache[func.name])
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
        task = GeminiRunner(self)
        
        self.current_task = task
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
