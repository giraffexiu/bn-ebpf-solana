#!/usr/bin/env python3

import sys
import os
from pathlib import Path
from typing import List, Dict, Any

# Import MCP bridge module for direct API mode
import importlib.util
spec = importlib.util.spec_from_file_location("binja_mcp_bridge_direct", Path(__file__).parent / "binja_mcp_bridge_direct.py")
binja_mcp_direct = importlib.util.module_from_spec(spec)
spec.loader.exec_module(binja_mcp_direct)

class DirectAPIInterface:

    
    def __init__(self, binary_view):
        """Initialize interface with current BinaryView."""
        self.bv = binary_view
        binja_mcp_direct.set_current_binary_view(binary_view)
    
    def list_methods(self, offset: int = 0, limit: int = 100) -> List[str]:
        """List all available methods."""
        return binja_mcp_direct.list_methods(offset, limit)
    
    def decompile_function(self, function_name: str) -> List[str]:
        """Decompile specified function."""
        return binja_mcp_direct.decompile_function(function_name)
    
    def fetch_disassembly(self, function_name: str) -> List[str]:
        """Get function disassembly."""
        return binja_mcp_direct.fetch_disassembly(function_name)
    
    def rename_function(self, old_name: str, new_name: str) -> List[str]:
        """Rename function."""
        return binja_mcp_direct.rename_function(old_name, new_name)
    
    def set_comment(self, address: str, comment: str) -> List[str]:
        """Set comment at specified address."""
        return binja_mcp_direct.set_comment(address, comment)
    
    def set_function_comment(self, function_name: str, comment: str) -> List[str]:
        """Set function comment."""
        return binja_mcp_direct.set_function_comment(function_name, comment)
    
    def get_comment(self, address: str) -> List[str]:
        """Get comment at specified address."""
        return binja_mcp_direct.get_comment(address)
    
    def get_function_comment(self, function_name: str) -> List[str]:
        """Get function comment."""
        return binja_mcp_direct.get_function_comment(function_name)
    
    def list_segments(self, offset: int = 0, limit: int = 100) -> List[str]:
        """List memory segments."""
        return binja_mcp_direct.list_segments(offset, limit)
    
    def list_imports(self, offset: int = 0, limit: int = 100) -> List[str]:
        """List imported symbols."""
        return binja_mcp_direct.list_imports(offset, limit)
    
    def list_exports(self, offset: int = 0, limit: int = 100) -> List[str]:
        """List exported symbols."""
        return binja_mcp_direct.list_exports(offset, limit)
    
    def search_functions_by_name(self, query: str, offset: int = 0, limit: int = 100) -> List[str]:
        """Search functions by name."""
        return binja_mcp_direct.search_functions_by_name(query, offset, limit)
    
    def get_binary_status(self) -> List[str]:
        """Get binary status."""
        return binja_mcp_direct.get_binary_status()
    
    def function_at(self, address: str) -> List[str]:
        """Get function at specified address."""
        return binja_mcp_direct.function_at(address)
    
    def code_references(self, function_name: str) -> List[str]:
        """Get function code references."""
        return binja_mcp_direct.code_references(function_name)
    
    def retype_variable(self, function_name: str, variable_name: str, type_str: str) -> List[str]:
        """Retype variable."""
        return binja_mcp_direct.retype_variable(function_name, variable_name, type_str)
    
    def rename_variable(self, function_name: str, variable_name: str, new_name: str) -> List[str]:
        """Rename variable."""
        return binja_mcp_direct.rename_variable(function_name, variable_name, new_name)
    
    def get_binary_status(self) -> List[str]:
        """Get binary status."""
        return binja_mcp_direct.get_binary_status()
    
    def get_available_tools(self) -> List[Dict[str, Any]]:
        """Get list of available tools (simulating MCP tool list)."""
        tools = [
            {"name": "list_methods", "description": "List all available methods in the binary"},
            {"name": "decompile_function", "description": "Decompile a function to high-level representation"},
            {"name": "fetch_disassembly", "description": "Get disassembly for a function"},
            {"name": "rename_function", "description": "Rename a function"},
            {"name": "set_comment", "description": "Set a comment at an address"},
            {"name": "set_function_comment", "description": "Set a comment for a function"},
            {"name": "get_comment", "description": "Get comment at an address"},
            {"name": "get_function_comment", "description": "Get comment for a function"},
            {"name": "list_segments", "description": "List memory segments"},
            {"name": "list_imports", "description": "List imported symbols"},
            {"name": "list_exports", "description": "List exported symbols"},
            {"name": "search_functions_by_name", "description": "Search functions by name"},
            {"name": "get_binary_status", "description": "Get binary status information"},
            {"name": "function_at", "description": "Get function at address"},
            {"name": "code_references", "description": "Get code references to a function"},
            {"name": "retype_variable", "description": "Retype a variable"},
            {"name": "rename_variable", "description": "Rename a variable"}
        ]
        return tools
    
    def call_tool(self, tool_name: str, **kwargs) -> List[str]:
        """Call specified tool (simulating MCP tool call)."""
        if hasattr(self, tool_name):
            method = getattr(self, tool_name)
            try:
                return method(**kwargs)
            except Exception as e:
                return [f"Error calling {tool_name}: {str(e)}"]
        else:
            return [f"Error: Tool '{tool_name}' not found"]