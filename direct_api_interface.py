#!/usr/bin/env python3
"""
直接API接口 - 不使用MCP客户端，直接调用函数
避免启动新的Python进程
"""

import sys
import os
from pathlib import Path
from typing import List, Dict, Any

# 导入直接API模式的MCP桥接模块
import importlib.util
spec = importlib.util.spec_from_file_location("binja_mcp_bridge_direct", Path(__file__).parent / "binja_mcp_bridge_direct.py")
binja_mcp_direct = importlib.util.module_from_spec(spec)
spec.loader.exec_module(binja_mcp_direct)

class DirectAPIInterface:
    """直接API接口类，提供与MCP工具相同的功能但不启动新进程"""
    
    def __init__(self, binary_view):
        """初始化接口并设置当前的BinaryView"""
        self.bv = binary_view
        binja_mcp_direct.set_current_binary_view(binary_view)
    
    def list_methods(self, offset: int = 0, limit: int = 100) -> List[str]:
        """列出所有可用的方法"""
        return binja_mcp_direct.list_methods(offset, limit)
    
    def decompile_function(self, function_name: str) -> List[str]:
        """反编译指定函数"""
        return binja_mcp_direct.decompile_function(function_name)
    
    def fetch_disassembly(self, function_name: str) -> List[str]:
        """获取函数的反汇编代码"""
        return binja_mcp_direct.fetch_disassembly(function_name)
    
    def rename_function(self, old_name: str, new_name: str) -> List[str]:
        """重命名函数"""
        return binja_mcp_direct.rename_function(old_name, new_name)
    
    def set_comment(self, address: str, comment: str) -> List[str]:
        """在指定地址设置注释"""
        return binja_mcp_direct.set_comment(address, comment)
    
    def set_function_comment(self, function_name: str, comment: str) -> List[str]:
        """为函数设置注释"""
        return binja_mcp_direct.set_function_comment(function_name, comment)
    
    def get_comment(self, address: str) -> List[str]:
        """获取指定地址的注释"""
        return binja_mcp_direct.get_comment(address)
    
    def get_function_comment(self, function_name: str) -> List[str]:
        """获取函数注释"""
        return binja_mcp_direct.get_function_comment(function_name)
    
    def list_segments(self, offset: int = 0, limit: int = 100) -> List[str]:
        """列出内存段"""
        return binja_mcp_direct.list_segments(offset, limit)
    
    def list_imports(self, offset: int = 0, limit: int = 100) -> List[str]:
        """列出导入符号"""
        return binja_mcp_direct.list_imports(offset, limit)
    
    def list_exports(self, offset: int = 0, limit: int = 100) -> List[str]:
        """列出导出符号"""
        return binja_mcp_direct.list_exports(offset, limit)
    
    def search_functions_by_name(self, query: str, offset: int = 0, limit: int = 100) -> List[str]:
        """按名称搜索函数"""
        return binja_mcp_direct.search_functions_by_name(query, offset, limit)
    
    def get_binary_status(self) -> List[str]:
        """获取二进制文件状态"""
        return binja_mcp_direct.get_binary_status()
    
    def function_at(self, address: str) -> List[str]:
        """获取指定地址的函数"""
        return binja_mcp_direct.function_at(address)
    
    def code_references(self, function_name: str) -> List[str]:
        """获取函数的代码引用"""
        return binja_mcp_direct.code_references(function_name)
    
    def retype_variable(self, function_name: str, variable_name: str, type_str: str) -> List[str]:
        """重新类型化变量"""
        return binja_mcp_direct.retype_variable(function_name, variable_name, type_str)
    
    def rename_variable(self, function_name: str, variable_name: str, new_name: str) -> List[str]:
        """重命名变量"""
        return binja_mcp_direct.rename_variable(function_name, variable_name, new_name)
    
    def get_binary_status(self) -> List[str]:
        """获取二进制文件状态"""
        return binja_mcp_direct.get_binary_status()
    
    def get_available_tools(self) -> List[Dict[str, Any]]:
        """获取所有可用工具的列表（模拟MCP工具列表）"""
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
        """调用指定的工具（模拟MCP工具调用）"""
        if hasattr(self, tool_name):
            method = getattr(self, tool_name)
            try:
                return method(**kwargs)
            except Exception as e:
                return [f"Error calling {tool_name}: {str(e)}"]
        else:
            return [f"Error: Tool '{tool_name}' not found"]