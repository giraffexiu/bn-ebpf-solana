#!/usr/bin/env python3

import asyncio
import os
import json
import hashlib
from pathlib import Path
from typing import Optional, Dict, List, Any, Tuple

from PySide6.QtWidgets import (
    QDialog, QVBoxLayout, QHBoxLayout, QLabel, QPushButton, 
    QProgressBar, QTextEdit, QCheckBox, QMessageBox, QFileDialog, 
    QGridLayout, QGroupBox
)
from PySide6.QtCore import QThread, Signal, Qt
from PySide6.QtGui import QFont

import binaryninja as bn
from binaryninja import Settings

from google import genai
from google.genai import types as genai_types
from pygments import highlight
from pygments.lexers import RustLexer
from pygments.formatters import HtmlFormatter

from .direct_api_interface import DirectAPIInterface


settings = Settings()

class CacheManager:
    """Local cache manager for Rust decompilation results"""
    
    def __init__(self, cache_dir: str = None):
        if cache_dir is None:
            # Use plugin directory's cache subdirectory as default
            plugin_dir = Path(__file__).parent
            cache_dir = plugin_dir / "cache"
        else:
            cache_dir = Path(cache_dir)
        
        self.cache_dir = cache_dir
        self.cache_dir.mkdir(parents=True, exist_ok=True)
        
    def _get_function_hash(self, binary_view: bn.BinaryView, func: bn.Function) -> str:
        """Generate unique hash for function based on binary and function content"""
        # Use binary file path, function name, and function content for hash
        binary_path = binary_view.file.filename or "unknown"
        func_content = ""
        
        try:
            # Get function disassembly as content identifier
            func_content = str(func.llil)
        except:
            func_content = func.name
            
        content = f"{binary_path}:{func.name}:{func_content}"
        return hashlib.md5(content.encode()).hexdigest()
    
    def get_cached_result(self, binary_view: bn.BinaryView, func: bn.Function) -> Optional[str]:
        """Get cached Rust code for function"""
        try:
            func_hash = self._get_function_hash(binary_view, func)
            cache_file = self.cache_dir / f"{func_hash}.rs"
            
            if cache_file.exists():
                with open(cache_file, 'r', encoding='utf-8') as f:
                    return f.read()
        except Exception:
            pass
        
        return None
    
    def cache_result(self, binary_view: bn.BinaryView, func: bn.Function, rust_code: str):
        """Cache Rust code for function"""
        try:
            func_hash = self._get_function_hash(binary_view, func)
            cache_file = self.cache_dir / f"{func_hash}.rs"
            
            with open(cache_file, 'w', encoding='utf-8') as f:
                f.write(f"// Cached: {func.name}\n")
                f.write(rust_code)
                
        except Exception:
            pass
    
    def clear_cache(self):
        """Clear all cached results"""
        try:
            for cache_file in self.cache_dir.glob("*.rs"):
                cache_file.unlink()
        except Exception:
            pass
    
    def get_cache_stats(self) -> dict:
        """Get cache statistics"""
        try:
            cache_files = list(self.cache_dir.glob("*.rs"))
            total_size = sum(f.stat().st_size for f in cache_files)
            return {
                "count": len(cache_files),
                "size_bytes": total_size,
                "size_mb": round(total_size / (1024 * 1024), 2)
            }
        except Exception:
            return {"count": 0, "size_bytes": 0, "size_mb": 0}

class BatchRustDecompilerWorker(QThread):
    """Batch Rust decompiler worker thread based on sidebar_ui.py GeminiRunner"""
    
    progress_updated = Signal(int, int, str)  # current, total, message
    function_completed = Signal(str, str)  # function_name, rust_code
    finished = Signal(dict)  # results
    error_occurred = Signal(str)  # error message
    
    def __init__(self, binary_view: bn.BinaryView, use_cache: bool = True):
        super().__init__()
        self.binary_view = binary_view
        self.use_cache = use_cache
        self.results = {}
        self._should_stop = False
        self.cache_manager = CacheManager()
    
    
    def run(self):
        """Main worker thread execution"""
        try:
            self._should_stop = False
            
            # Get all functions to process
            functions = list(self.binary_view.functions)
            
            total_functions = len(functions)
            self.progress_updated.emit(0, total_functions, f"Starting Rust decompilation of {total_functions} functions...")
            
            # Process each function
            for i, func in enumerate(functions):
                if self._should_stop:
                    break
                
                self.progress_updated.emit(i, total_functions, f"Processing {func.name}...")
                
                try:
                    rust_code = None
                    
                    # Check cache first if enabled
                    if self.use_cache:
                        rust_code = self.cache_manager.get_cached_result(self.binary_view, func)
                        if rust_code:
                            self.progress_updated.emit(i, total_functions, f"Using cached result for {func.name}")
                    
                    # If no cached result, call API
                    if not rust_code:
                        self.progress_updated.emit(i, total_functions, f"Decompiling {func.name} with Gemini...")
                        rust_code = asyncio.run(self._extract_rust_gemini(func))
                        
                        # Cache the result if successful and cache is enabled
                        if self.use_cache and rust_code and not rust_code.startswith("Error:") and rust_code != "Task stopped":
                            self.cache_manager.cache_result(self.binary_view, func, rust_code)
                    
                    self.results[func.name] = rust_code
                    self.function_completed.emit(func.name, rust_code)
                    
                except Exception as e:
                    self.results[func.name] = f"Error: {str(e)}"
                    self.function_completed.emit(func.name, f"Error: {str(e)}")
            
            # Emit final results
            if not self._should_stop:
                self.progress_updated.emit(total_functions, total_functions, "Rust decompilation completed!")
                self.finished.emit(self.results)
            
        except Exception as e:
            error_msg = f"Batch Rust decompilation failed: {str(e)}"
            self.error_occurred.emit(error_msg)
    
    async def _extract_rust_gemini(self, func: bn.Function) -> str:
        """Extract Rust code using Gemini API - based on sidebar_ui.py implementation"""
        func_name = func.name if func else "unknown"
        
        if not func:
            return ""

        # Check for stop request
        if self._should_stop:
            return "Task stopped"

        api_key = settings.get_string("bn-ebpf-solana.gemini_api_key")

        if not api_key:
            print(f"[ERROR] No Gemini API key found in settings")
            return "Please set your Gemini API key in the plugin settings"

        try:
            gemini_client = genai.Client(api_key=api_key)
        except Exception as e:
            print(f"[ERROR] Failed to create Gemini client: {e}")
            return f"Failed to create Gemini client: {e}"

        # Get extra context
        extra_context_path = settings.get_string("bn-ebpf-solana.context")
        extra_context = ""
        if extra_context_path != "":
            try:
                with open(extra_context_path) as f:
                    extra_context = f.read()
            except Exception as e:
                print(f"[WARNING] Failed to load extra context: {e}")

        try:
            system_prompt = open(Path(__file__).parent / "system.txt").read() + extra_context
        except Exception as e:
            print(f"[WARNING] Failed to load system prompt: {e}")
            system_prompt = "You are a helpful assistant for decompiling binary code to Rust."

        # Build the prompt
        prompt_content = f"Improve the quality of decompilation inside binary ninja of {func.name} using all tools at your disposal"

        # Use direct API interface
        api_interface = DirectAPIInterface(self.binary_view)
        
        # Check for stop request before API call
        if self._should_stop:
            return "Task stopped"
        
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
        except Exception as e:
            print(f"[ERROR] Gemini API call failed for function {func_name}: {e}")
            return f"Gemini API call failed: {e}"
        
        # Check for stop request after API call
        if self._should_stop:
            return "Task stopped"
        
        # Extract text from response
        if hasattr(response, 'text') and response.text:
            final = response.text
            
            # Clean up response similar to sidebar_ui.py
            if "```" in final:
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
                        return extracted
                    else:
                        extracted = final[code_start:].strip()
                        return extracted
                else:
                    extracted = final[start_idx + 3:].strip()
                    return extracted
            if "pub fn" in final:
                extracted = final[final.index("pub fn"):]
                return extracted
            return final
        else:
            print(f"[ERROR] No text response from Gemini API")
            return "No response generated from Gemini"
    
    def stop(self):
        """Stop the decompilation process"""
        self._should_stop = True


class BatchRustDecompilerDialog(QDialog):
    """Batch Rust decompiler dialog using Gemini AI"""
    
    def __init__(self, binary_view: bn.BinaryView, parent=None):
        super().__init__(parent)
        self.binary_view = binary_view
        self.worker = None
        self.results = {}
        self.cache_manager = CacheManager()
        
        self.setWindowTitle("Batch Rust Decompiler (Gemini)")
        self.setMinimumSize(800, 600)
        self.resize(1000, 700)
        
        self._setup_ui()
        self._check_api_key()
        self._update_cache_stats()
    
    def _setup_ui(self):
        """Setup UI interface"""
        layout = QVBoxLayout(self)
        
        # Configuration area
        config_group = QGroupBox("Configuration")
        config_layout = QGridLayout(config_group)
        
        # Use cache
        config_layout.addWidget(QLabel("Use Local Cache:"), 0, 0)
        self.use_cache_checkbox = QCheckBox()
        self.use_cache_checkbox.setChecked(True)
        config_layout.addWidget(self.use_cache_checkbox, 0, 1)
        
        layout.addWidget(config_group)
        
        # Cache management area
        cache_group = QGroupBox("Cache Management")
        cache_layout = QGridLayout(cache_group)
        
        # Cache statistics
        self.cache_stats_label = QLabel()
        self.cache_stats_label.setFont(QFont("monospace"))
        cache_layout.addWidget(self.cache_stats_label, 0, 0, 1, 2)
        
        # Cache buttons
        self.clear_cache_button = QPushButton("Clear Cache")
        self.clear_cache_button.clicked.connect(self._clear_cache)
        cache_layout.addWidget(self.clear_cache_button, 1, 0)
        
        self.refresh_cache_button = QPushButton("Refresh Stats")
        self.refresh_cache_button.clicked.connect(self._update_cache_stats)
        cache_layout.addWidget(self.refresh_cache_button, 1, 1)
        
        layout.addWidget(cache_group)
        
        # Status area
        status_group = QGroupBox("Status")
        status_layout = QVBoxLayout(status_group)
        
        # API key status
        self.api_status_label = QLabel()
        self.api_status_label.setFont(QFont("monospace"))
        status_layout.addWidget(self.api_status_label)
        
        # Binary file information
        self.binary_info_label = QLabel()
        self.binary_info_label.setFont(QFont("monospace"))
        self._update_binary_info()
        status_layout.addWidget(self.binary_info_label)
        
        layout.addWidget(status_group)
        
        # Progress area
        progress_group = QGroupBox("Progress")
        progress_layout = QVBoxLayout(progress_group)
        
        self.progress_bar = QProgressBar()
        progress_layout.addWidget(self.progress_bar)
        
        self.progress_label = QLabel("Ready")
        progress_layout.addWidget(self.progress_label)
        
        layout.addWidget(progress_group)
        
        # Results preview area
        results_group = QGroupBox("Rust Code Preview")
        results_layout = QVBoxLayout(results_group)
        
        self.results_text = QTextEdit()
        self.results_text.setFont(QFont("monospace", 10))
        self.results_text.setReadOnly(True)
        results_layout.addWidget(self.results_text)
        
        layout.addWidget(results_group)
        
        # Button area
        button_layout = QHBoxLayout()
        
        self.start_button = QPushButton("Start Batch Decompilation")
        self.start_button.clicked.connect(self._start_decompilation)
        button_layout.addWidget(self.start_button)
        
        self.stop_button = QPushButton("Stop")
        self.stop_button.clicked.connect(self._stop_decompilation)
        self.stop_button.setEnabled(False)
        button_layout.addWidget(self.stop_button)
        
        self.export_button = QPushButton("Export Rust Code")
        self.export_button.clicked.connect(self._export_results)
        self.export_button.setEnabled(False)
        button_layout.addWidget(self.export_button)
        
        self.export_callgraph_button = QPushButton("Export Call Graph")
        self.export_callgraph_button.clicked.connect(self._export_call_graph)
        button_layout.addWidget(self.export_callgraph_button)
        
        self.close_button = QPushButton("Close")
        self.close_button.clicked.connect(self.close)
        button_layout.addWidget(self.close_button)
        
        button_layout.addStretch()
        layout.addLayout(button_layout)
    
    def _check_api_key(self):
        """Check API key status"""
        api_key = settings.get_string("bn-ebpf-solana.gemini_api_key")
        if api_key:
            self.api_status_label.setText("✓ Gemini API Key: Configured")
            self.api_status_label.setStyleSheet("color: green;")
        else:
            self.api_status_label.setText("✗ Gemini API Key: Not configured")
            self.api_status_label.setStyleSheet("color: red;")
    
    def _update_cache_stats(self):
        """Update cache statistics display"""
        stats = self.cache_manager.get_cache_stats()
        cache_dir = self.cache_manager.cache_dir
        self.cache_stats_label.setText(
            f"Cache Directory: {cache_dir}\n"
            f"Cached Functions: {stats['count']}\n"
            f"Cache Size: {stats['size_mb']} MB"
        )
    
    def _clear_cache(self):
        """Clear cache with confirmation"""
        reply = QMessageBox.question(
            self, "Clear Cache", 
            "Are you sure you want to clear all cached Rust decompilation results?",
            QMessageBox.StandardButton.Yes | QMessageBox.StandardButton.No,
            QMessageBox.StandardButton.No
        )
        
        if reply == QMessageBox.StandardButton.Yes:
            self.cache_manager.clear_cache()
            self._update_cache_stats()
            QMessageBox.information(self, "Cache Cleared", "All cached results have been cleared.")
    
    def _update_binary_info(self):
        """Update binary file information"""
        if self.binary_view:
            filename = self.binary_view.file.filename
            function_count = len(self.binary_view.functions)
            self.binary_info_label.setText(
                f"File: {filename}\n"
                f"Functions: {function_count}"
            )
    
    def _start_decompilation(self):
        """Start batch decompilation process"""
        if self.worker and self.worker.isRunning():
            QMessageBox.warning(self, "Warning", "Decompilation is already running!")
            return
        
        api_key = settings.get_string("bn-ebpf-solana.gemini_api_key")
        if not api_key:
            QMessageBox.warning(self, "API Key Required", 
                              "Please configure your Gemini API key in the plugin settings.")
            return
        
        use_cache = self.use_cache_checkbox.isChecked()
        
        # Create and start worker
        self.worker = BatchRustDecompilerWorker(self.binary_view, use_cache)
        self.worker.progress_updated.connect(self._on_progress)
        self.worker.function_completed.connect(self._on_function_completed)
        self.worker.finished.connect(self._on_finished)
        self.worker.error_occurred.connect(self._on_error)
        
        self.worker.start()
        
        # Update UI state
        self.start_button.setEnabled(False)
        self.stop_button.setEnabled(True)
        self.export_button.setEnabled(False)
        self.results_text.clear()
        self.results = {}
        self.progress_bar.setValue(0)
    
    def _stop_decompilation(self):
        """Stop decompilation process"""
        if self.worker and self.worker.isRunning():
            self.worker.stop()
            self.worker.wait(5000)  # Wait up to 5 seconds
            
            if self.worker.isRunning():
                self.worker.terminate()
                self.worker.wait()
        
        self._reset_ui_state()
        self.progress_label.setText("Stopped")
    
    def _on_progress(self, current: int, total: int, message: str):
        """Handle progress updates"""
        if total > 0:
            progress = int((current / total) * 100)
            self.progress_bar.setValue(progress)
        self.progress_label.setText(message)
    
    def _on_function_completed(self, function_name: str, rust_code: str):
        """Handle individual function completion"""
        self.results[function_name] = rust_code
        
        # Update preview with latest result
        if rust_code and not rust_code.startswith("Error:"):
            try:
                # Use syntax highlighting
                formatter = HtmlFormatter(
                    style="native",
                    noclasses=True,
                    cssstyles="background: transparent; font-family: monospace;"
                )
                highlighted = highlight(rust_code, RustLexer(), formatter)
                self.results_text.setHtml(highlighted)
            except Exception:
                self.results_text.setPlainText(rust_code)
        else:
            self.results_text.setPlainText(rust_code)
    
    def _on_finished(self, results: dict):
        """Handle completion of all functions"""
        self.results = results
        self._reset_ui_state()
        self.export_button.setEnabled(True)
        
        # Update cache stats after completion
        self._update_cache_stats()
        
        success_count = sum(1 for code in results.values() if not code.startswith("Error:"))
        total_count = len(results)
        
        QMessageBox.information(self, "Decompilation Complete", 
                              f"Batch decompilation completed!\n"
                              f"Successfully processed: {success_count}/{total_count} functions")
    
    def _on_error(self, error_message: str):
        """Handle errors"""
        self._reset_ui_state()
        QMessageBox.critical(self, "Error", f"Decompilation failed:\n{error_message}")
    
    def _reset_ui_state(self):
        """Reset UI to initial state"""
        self.start_button.setEnabled(True)
        self.stop_button.setEnabled(False)
    
    def _export_results(self):
        """Export results to file"""
        if not self.results:
            QMessageBox.warning(self, "No Results", "No results to export.")
            return
        
        filename, _ = QFileDialog.getSaveFileName(
            self, "Export Rust Code", "rust_decompiled.rs", "Rust Files (*.rs);;All Files (*)"
        )
        
        if filename:
            try:
                with open(filename, 'w', encoding='utf-8') as f:
                    f.write("// Batch Rust Decompilation Results\n")
                    f.write(f"// Generated from: {self.binary_view.file.filename}\n")
                    f.write("// Generated by: Batch Rust Decompiler\n\n")
                    
                    for func_name, rust_code in self.results.items():
                        f.write(f"// Function: {func_name}\n")
                        f.write(f"{rust_code}\n\n")
                
                QMessageBox.information(self, "Export Complete", f"Results exported to: {filename}")
            except Exception as e:
                QMessageBox.critical(self, "Export Error", f"Failed to export results:\n{str(e)}")
    

    
    def _serialize_for_json(self, obj):
        """Convert Binary Ninja specific types to JSON serializable types"""
        if hasattr(obj, 'value') and hasattr(obj, 'confidence'):
            # Handle BoolWithConfidence and similar types
            return obj.value
        elif hasattr(obj, '__bool__'):
            # Handle other boolean-like objects
            return bool(obj)
        return obj
    
    def _export_call_graph(self):
        """Export function call graph as JSON file"""
        try:
            # Show file save dialog
            file_path, _ = QFileDialog.getSaveFileName(
                self,
                "Export Function Call Graph",
                f"{self.binary_view.file.filename}_call_graph.json",
                "JSON Files (*.json);;All Files (*)"
            )
            
            if not file_path:
                return
            
            # Extract call graph
            self.progress_label.setText("Extracting function call relationships...")
            self.progress_bar.setRange(0, 0)  # Indeterminate progress
            
            if not self.binary_view:
                raise Exception("No binary view available")
            
            if len(self.binary_view.functions) == 0:
                self.binary_view.update_analysis_and_wait()
            
            # Build comprehensive call graph data structure
            call_graph = {
                "binary_info": {
                    "filename": self.binary_view.file.filename,
                    "arch": str(self.binary_view.arch),
                    "platform": str(self.binary_view.platform),
                    "entry_point": hex(self.binary_view.entry_point) if self.binary_view.entry_point else None,
                    "total_functions": len(self.binary_view.functions)
                },
                "functions": {},
                "statistics": {
                    "total_calls": 0,
                    "internal_calls": 0,
                    "external_calls": 0,
                    "recursive_calls": 0
                }
            }
            
            # Extract detailed function information and call relationships
            total_calls = 0
            internal_calls = 0
            external_calls = 0
            recursive_calls = 0
            
            for func in self.binary_view.functions:
                # Get function calls
                calls_to = []
                called_by = []
                
                # Analyze outgoing calls using callees property
                for callee in func.callees:
                    target_name = callee.name
                    internal_calls += 1
                    if callee == func:
                        recursive_calls += 1
                    
                    calls_to.append({
                        "target": target_name,
                        "address": f"0x{callee.start:x}"
                    })
                    total_calls += 1
                
                # Also check for external calls through call sites
                for call_site in func.call_sites:
                    refs_from = self.binary_view.get_code_refs_from(call_site.address)
                    for ref in refs_from:
                        # Check if this is an external symbol
                        symbol = self.binary_view.get_symbol_at(ref)
                        if symbol and not self.binary_view.get_function_at(ref):
                            external_calls += 1
                            calls_to.append({
                                "target": symbol.name,
                                "address": f"0x{ref:x}",
                                "call_site": f"0x{call_site.address:x}",
                                "external": True
                            })
                            total_calls += 1
                            break
                
                # Find incoming calls using callers property
                for caller in func.callers:
                    called_by.append({
                        "caller": caller.name,
                        "address": f"0x{caller.start:x}"
                    })
                
                # Build function info
                func_info = {
                    "address": f"0x{func.start:x}",
                    "size": int(func.total_bytes),
                    "name": func.name,
                    "symbol_type": str(func.symbol.type) if func.symbol else "SymbolType.FunctionSymbol",
                    "can_return": self._serialize_for_json(func.can_return),
                    "basic_blocks": len(func.basic_blocks),
                    "calls_to": calls_to,
                    "called_by": called_by
                }
                
                call_graph["functions"][func.name] = func_info
            
            # Update statistics
            call_graph["statistics"] = {
                "total_calls": total_calls,
                "internal_calls": internal_calls,
                "external_calls": external_calls,
                "recursive_calls": recursive_calls
            }
            
            # Save to JSON file
            with open(file_path, 'w', encoding='utf-8') as f:
                json.dump(call_graph, f, indent=2, ensure_ascii=False)
            
            # Reset progress
            self.progress_bar.setRange(0, 100)
            self.progress_bar.setValue(100)
            self.progress_label.setText("Call graph export completed")
            
            # Show success message with statistics
            stats = call_graph.get('statistics', {})
            binary_info = call_graph.get('binary_info', {})
            
            success_msg = f"""Function call graph exported successfully!
            
File: {file_path}
Format: JSON
            
Statistics:
            • Total Functions: {binary_info.get('total_functions', 0)}
            • Total Calls: {stats.get('total_calls', 0)}
            • Internal Calls: {stats.get('internal_calls', 0)}
            • External Calls: {stats.get('external_calls', 0)}
            • Recursive Calls: {stats.get('recursive_calls', 0)}
            
Architecture: {binary_info.get('arch', 'Unknown')}
Platform: {binary_info.get('platform', 'Unknown')}
Entry Point: {binary_info.get('entry_point', 'Unknown')}"""
            
            QMessageBox.information(
                self, "Call Graph Export Success", success_msg
            )
            
        except Exception as e:
            print(f"[ERROR] Call graph export failed: {e}")
            import traceback
            traceback.print_exc()
            
            # Reset progress on error
            self.progress_bar.setRange(0, 100)
            self.progress_bar.setValue(0)
            self.progress_label.setText("Ready")
            
            QMessageBox.critical(
                self, "Call Graph Export Error", 
                f"Failed to extract and export call graph:\n{str(e)}\n\nSee console for details."
            )
def show_batch_rust_decompiler(binary_view: bn.BinaryView):
    """Show the batch Rust decompiler dialog"""
    dialog = BatchRustDecompilerDialog(binary_view)
    dialog.exec()