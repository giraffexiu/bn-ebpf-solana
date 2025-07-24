import os
from binaryninja import (
    Settings,
    BackgroundTaskThread,
    execute_on_main_thread
)
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
from anthropic import Anthropic, AuthenticationError, RateLimitError, APIStatusError
from fastmcp import exceptions as mcp_exc
from tenacity import retry, stop_after_attempt, wait_random_exponential
from google import genai
from google.genai import types as genai_types

import sys, os

from .idl_utils import fetch_idl_anchorpy
from .direct_api_interface import DirectAPIInterface

# binja screws up stdout and stderr, fastmcp doesnt like that
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


# 使用直接API接口替代MCP客户端
# SERVER_PATH = Path(__file__).parent / "binja_mcp_bridge_direct.py"  # 不再需要

# 不再需要动态导入MCP桥接模块
# import importlib.util
# spec = importlib.util.spec_from_file_location("binja_mcp_bridge_direct", SERVER_PATH)
# binja_mcp_direct = importlib.util.module_from_spec(spec)
# spec.loader.exec_module(binja_mcp_direct)

#settings

settings = Settings()
setting_props = properties = f'{{"title" : "Anthropic API Key", "description" : "Your Anthropic API key for LLM requests", "type" : "string", "default" : ""}}'
setting_props_2 = properties = f'{{"title" : "Context for Solana MCP", "description" : "Absolute path to extra context to be provided, like an IDL", "type" : "string", "default" : ""}}'
setting_props_3 = properties = f'{{"title" : "Gemini API Key", "description" : "Your Google Gemini API key for LLM requests", "type" : "string", "default" : ""}}'
setting_props_4 = properties = f'{{"title" : "LLM Provider", "description" : "Choose between Claude and Gemini", "type" : "string", "default" : "claude", "enum" : ["claude", "gemini"]}}'
settings.register_group("bn-ebpf-solana", "MCP settings")
settings.register_setting(
    "bn-ebpf-solana.context",           # identifier
    setting_props_2
)
settings.register_setting(
    "bn-ebpf-solana.anthropic_api_key",           # identifier
    setting_props
)
settings.register_setting(
    "bn-ebpf-solana.gemini_api_key",           # identifier
    setting_props_3
)
settings.register_setting(
    "bn-ebpf-solana.llm_provider",           # identifier
    setting_props_4
)


class LLMRunner(BackgroundTaskThread):
    def __init__(self, bar):
        provider = settings.get_string("bn-ebpf-solana.llm_provider")
        super().__init__(f"Prompting {provider.title()} for Rust (running)…", can_cancel=True)
        self.bar = bar
        self.func        = bar.f
        self.idl = bar.idl
        self.provider = provider

class ClaudeRunner(LLMRunner):
    def __init__(self, bar):
        super().__init__(bar)

    def run(self):
        """
        BackgroundTaskThread entry-point.
        • Talks to fast-mcp / Claude in a private event loop.
        • If anything LLM-related fails, it prints a short notice instead of
          killing the sidebar or Binary Ninja.
        """
        pretty_text = ""

        try:
            # actual async work
            pretty_text = asyncio.run(self._extract_rust())

        # ───── expected user-side failures ───────────────────────────────────
        except AuthenticationError as exc:
            pretty_text = f"LLM disabled: bad Anthropic API key ({exc})"

        except mcp_exc.McpError as exc:
            pretty_text = f"LLM disabled: MCP bridge error ({exc})"

        except RuntimeError as exc:          # client not connected, etc.
            pretty_text = f"LLM disabled: {exc}"

        # ───── any other unexpected crash ───────────────────────────────────
        except Exception as exc:
            pretty_text = f"LLM disabled: {type(exc).__name__}: {exc}"

        # ───── always update the sidebar on UI thread ───────────────────────
        finally:
            execute_on_main_thread(
                lambda: self.bar.update_ui_func(self.func, pretty_text)
            )

    # ui stuff


    #llm pipeline
    async def _extract_rust(self) -> str:
        if not self.func:
            return ""

        api_key = settings.get_string("bn-ebpf-solana.anthropic_api_key")

        if not api_key.startswith("sk-"):
            return "Please set your Anthropic API key in the plugin settings"

        # 使用直接API接口
        api_interface = DirectAPIInterface(self.bar.bv)
        tools = api_interface.get_available_tools()
        specs = [mcp_to_anthropic(t) for t in tools]

        msgs: list[Dict[str, Any]] = [{
             "role":    "user",
             "content": f"Improve the quality of decompilation inside binary ninja of {self.func.name} using all tools at your disposal"
         },
         {
           "role": "user",
           "content": "Here is the IDL interface file : " + self.idl if self.idl else ""
         }
         ]

        for _ in range(50):                        # safety cap
             reply = await self._call_claude(specs, msgs, api_key)
             tool_calls = [b for b in reply.content if b.type == "tool_use"]

             # llm doesnt wanna call any tools anymore
             if not tool_calls:
                 final = "".join(b.text for b in reply.content if b.type == "text")
                 # claude is dumb and sometimes still inserts comments before code
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
                             return final[code_start:end_idx].strip()
                         else:
                             return final[code_start:].strip()
                     else:
                         return final[start_idx + 3:].strip()
                 if "pub fn" in final:
                     return final[final.index("pub fn"):]
                 return final

             # tool calls
             results: list[Dict[str, Any]] = []
             for call in tool_calls:
                 try:
                     res = api_interface.call_tool(call.name, **call.input)
                     payload = json.dumps(blocks_to_str(res), ensure_ascii=False)
                 except Exception as e:      # bad args, runtime error, etc.
                     payload = json.dumps({"error": str(e)}, ensure_ascii=False)

                 results.append({
                     "type":        "tool_result",
                     "tool_use_id": call.id,
                     "content":     payload,
                 })

             # ---------- feed results back to Claude ----------
             msgs.append({"role": "assistant", "content": reply.content})
             msgs.append({"role": "user",      "content": results})

        raise RuntimeError("LLM never produced final text")

    #wrap to circumvent rate limiting
    @retry(
        wait=wait_random_exponential(multiplier=1, min=10, max=60),  # 1 s → 30 s
        stop=stop_after_attempt(5),
        retry=(
            lambda exc: isinstance(exc, (RateLimitError, APIStatusError))
        ),
        reraise=False,
    )
    async def _call_claude(self, specs, msgs, api_key):
        """Single API call - automatically retried on 429 / 5xx."""
        claude = Anthropic(api_key=api_key)

        extra_context_path = settings.get_string("bn-ebpf-solana.context")
        extra_context = ""

        if(extra_context_path != ""):
            with open(extra_context_path) as f:
                extra_context = f.read()

        return claude.messages.create(
            model   = "claude-3-5-sonnet-20241022",
            system  = open(Path(__file__).parent / "system.txt").read() + extra_context, 
            messages     = msgs,
            tools        = specs,
            max_tokens   = 1_200,
            temperature  = 0.3,
        )

class GeminiRunner(LLMRunner):
    def __init__(self, bar):
        super().__init__(bar)

    def run(self):
        """BackgroundTaskThread entry-point for Gemini."""
        pretty_text = ""

        try:
            # actual async work
            pretty_text = asyncio.run(self._extract_rust_gemini())

        # ───── expected user-side failures ───────────────────────────────────
        except Exception as exc:
            if "API_KEY" in str(exc) or "authentication" in str(exc).lower():
                pretty_text = f"LLM disabled: bad Gemini API key ({exc})"
            elif "mcp" in str(exc).lower():
                pretty_text = f"LLM disabled: MCP bridge error ({exc})"
            else:
                pretty_text = f"LLM disabled: {type(exc).__name__}: {exc}"

        # ───── always update the sidebar on UI thread ───────────────────────
        finally:
            execute_on_main_thread(
                lambda: self.bar.update_ui_func(self.func, pretty_text)
            )

    async def _extract_rust_gemini(self) -> str:
        if not self.func:
            return ""

        api_key = settings.get_string("bn-ebpf-solana.gemini_api_key")

        if not api_key:
            return "Please set your Gemini API key in the plugin settings"

        gemini_client = genai.Client(api_key=api_key)

        # Get extra context
        extra_context_path = settings.get_string("bn-ebpf-solana.context")
        extra_context = ""
        if extra_context_path != "":
            with open(extra_context_path) as f:
                extra_context = f.read()

        system_prompt = open(Path(__file__).parent / "system.txt").read() + extra_context

        # Build the prompt
        prompt_content = f"Improve the quality of decompilation inside binary ninja of {self.func.name} using all tools at your disposal"
        if self.idl:
            prompt_content += f"\n\nHere is the IDL interface file: {self.idl}"

        # 使用直接API接口
        api_interface = DirectAPIInterface(self.bar.bv)
        
        # Use direct API with Gemini
        response = await gemini_client.aio.models.generate_content(
            model="gemini-2.0-flash",
            contents=prompt_content,
            config=genai_types.GenerateContentConfig(
                system_instruction=system_prompt,
                temperature=0.3,
                max_output_tokens=1200
            )
        )
        
        # Extract text from response
        if hasattr(response, 'text') and response.text:
            final = response.text
            # Clean up response similar to Claude
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
                        return final[code_start:end_idx].strip()
                    else:
                        return final[code_start:].strip()
                else:
                    return final[start_idx + 3:].strip()
            if "pub fn" in final:
                return final[final.index("pub fn"):]
            return final
        else:
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
        if func.name not in self.cache:
            self.cache[func.name] = source

        highlighted = highlight(source, RustLexer(), self._formatter)
        self.editor.setHtml(highlighted)

    def _update(self):
        """
        Refresh sidebar whenever the cursor moves or the view changes.

        Fixes:
        ▸ Guard against cases where the current address is NOT inside any
          discovered function (bn.get_functions_containing returns []).
        ▸ Avoid repeated work if we’re still in the same function.
        """
        # ── Early-outs ────────────────────────────────────────────────────────────
        if not self.frame or not self.bv:
            return

        view = self.frame.getCurrentViewInterface()
        addr = view.getCurrentOffset()

        if addr == self.here:               # cursor hasn’t moved
            return
        self.here = addr

        # ── Robust lookup ────────────────────────────────────────────────────────
        funcs = self.bv.get_functions_containing(addr)
        if not funcs:
            # Cursor is in padding / header / data region → just clear pane.
            self.editor.setHtml("")
            self.f = None
            return

        func = funcs[0]
        if self.f == func:                  # already displaying this function
            return
        self.f = func

        # ── Cached? ──────────────────────────────────────────────────────────────
        if func.name in self.cache:
            self.update_ui_func(func, self.cache[func.name])
            return

        # ── Kick off background LLM job ───────────────────────────────────────
        provider = settings.get_string("bn-ebpf-solana.llm_provider")
        if provider == "gemini":
            task = GeminiRunner(self)
        else:
            task = ClaudeRunner(self)
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

        super().__init__(icon, "AI reconstructed Rust (Claude/Gemini)")

    def createWidget(self, frame, data):
        # frame: a ViewFrameRef, data: BinaryViewRef or None
        return LLMDecompSidebarWidget("LLM reconstructed Rust", frame, data)

    def defaultLocation(self):
        return SidebarWidgetLocation.RightContent

    def contextSensitivity(self):
        return SidebarContextSensitivity.SelfManagedSidebarContext

# Register the sidebar widget type
Sidebar.addSidebarWidgetType(LLMDecompSidebarWidgetType())
