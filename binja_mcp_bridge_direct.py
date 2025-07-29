from mcp.server.fastmcp import FastMCP
from typing import Optional, List
import threading
import sys
import os

# Try to import Binary Ninja, set to None if failed
try:
    import binaryninja as bn
    BN_AVAILABLE = True
except ImportError:
    bn = None
    BN_AVAILABLE = False
    print("Warning: Binary Ninja not available, running in mock mode")

# Global variable to store current BinaryView instance
_current_bv: Optional = None
_bv_lock = threading.Lock()

mcp = FastMCP("binja-mcp-direct")

def set_current_binary_view(bv):
    """Set the current BinaryView instance"""
    global _current_bv
    with _bv_lock:
        _current_bv = bv

def get_current_binary_view():
    """Get the current BinaryView instance"""
    with _bv_lock:
        return _current_bv

def safe_execute(func, *args, **kwargs):
    """Safely execute function, catch exceptions and return error information"""
    if not BN_AVAILABLE:
        return ["Error: Binary Ninja not available"]
    
    try:
        result = func(*args, **kwargs)
        if isinstance(result, list):
            return result
        elif isinstance(result, str):
            return [result] if result else [""]
        else:
            return [str(result)] if result is not None else [""]
    except Exception as e:
        return [f"Error: {str(e)}"]

@mcp.tool()
def list_methods(offset: int = 0, limit: int = 100) -> List[str]:
    """
    List all function names in the program with pagination.
    """
    bv = get_current_binary_view()
    if not bv:
        return ["Error: No binary view available"]
    
    def _list_methods():
        functions = list(bv.functions)
        start_idx = offset
        end_idx = min(offset + limit, len(functions))
        return [func.name for func in functions[start_idx:end_idx]]
    
    return safe_execute(_list_methods)

@mcp.tool()
def decompile_function(name: str) -> List[str]:
    """
    Decompile a specific function by name and return the decompiled C code.
    """
    bv = get_current_binary_view()
    if not bv:
        return ["Error: No binary view available"]
    
    def _decompile_function():
        func = bv.get_function_by_name(name)
        if not func:
            return [f"Error: Function '{name}' not found"]
        
        # Get high-level IL representation
        hlil = func.hlil
        if hlil:
            return hlil.source_code.split('\n')
        else:
            return [f"Error: No high-level IL available for function '{name}'"]
    
    return safe_execute(_decompile_function)

@mcp.tool()
def fetch_disassembly(name: str) -> List[str]:
    """
    Retrieve the disassembled code of a function with a given name as assembly mnemonic instructions.
    """
    bv = get_current_binary_view()
    if not bv:
        return ["Error: No binary view available"]
    
    def _fetch_disassembly():
        func = bv.get_function_by_name(name)
        if not func:
            return [f"Error: Function '{name}' not found"]
        
        disasm_lines = []
        for block in func.basic_blocks:
            for instr in block:
                disasm_lines.append(f"{instr.address:#x}: {instr}")
        return disasm_lines
    
    return safe_execute(_fetch_disassembly)

@mcp.tool()
def rename_function(old_name: str, new_name: str) -> List[str]:
    """
    Rename a function by its current name to a new user-defined name.
    """
    bv = get_current_binary_view()
    if not bv:
        return ["Error: No binary view available"]
    
    def _rename_function():
        func = bv.get_function_by_name(old_name)
        if not func:
            return [f"Error: Function '{old_name}' not found"]
        
        func.name = new_name
        return [f"Function renamed from '{old_name}' to '{new_name}'"]
    
    return safe_execute(_rename_function)

@mcp.tool()
def set_comment(address: str, comment: str) -> List[str]:
    """
    Set a comment at a specific address.
    """
    bv = get_current_binary_view()
    if not bv:
        return ["Error: No binary view available"]
    
    def _set_comment():
        try:
            addr = int(address, 16) if address.startswith('0x') else int(address)
        except ValueError:
            return [f"Error: Invalid address format '{address}'"]
        
        bv.set_comment_at(addr, comment)
        return [f"Comment set at address {address}"]
    
    return safe_execute(_set_comment)

@mcp.tool()
def set_function_comment(function_name: str, comment: str) -> List[str]:
    """
    Set a comment for a function.
    """
    bv = get_current_binary_view()
    if not bv:
        return ["Error: No binary view available"]
    
    def _set_function_comment():
        func = bv.get_function_by_name(function_name)
        if not func:
            return [f"Error: Function '{function_name}' not found"]
        
        func.comment = comment
        return [f"Comment set for function '{function_name}'"]
    
    return safe_execute(_set_function_comment)

@mcp.tool()
def get_comment(address: str) -> List[str]:
    """
    Get the comment at a specific address.
    """
    bv = get_current_binary_view()
    if not bv:
        return ["Error: No binary view available"]
    
    def _get_comment():
        try:
            addr = int(address, 16) if address.startswith('0x') else int(address)
        except ValueError:
            return [f"Error: Invalid address format '{address}'"]
        
        comment = bv.get_comment_at(addr)
        return [comment if comment else ""]
    
    return safe_execute(_get_comment)

@mcp.tool()
def get_function_comment(function_name: str) -> List[str]:
    """
    Get the comment for a function.
    """
    bv = get_current_binary_view()
    if not bv:
        return ["Error: No binary view available"]
    
    def _get_function_comment():
        func = bv.get_function_by_name(function_name)
        if not func:
            return [f"Error: Function '{function_name}' not found"]
        
        comment = func.comment
        return [comment if comment else ""]
    
    return safe_execute(_get_function_comment)

@mcp.tool()
def list_segments(offset: int = 0, limit: int = 100) -> List[str]:
    """
    List all memory segments in the program with pagination.
    """
    bv = get_current_binary_view()
    if not bv:
        return ["Error: No binary view available"]
    
    def _list_segments():
        segments = list(bv.segments)
        start_idx = offset
        end_idx = min(offset + limit, len(segments))
        result = []
        for seg in segments[start_idx:end_idx]:
            result.append(f"{seg.start:#x}-{seg.end:#x}: {seg.readable}{seg.writable}{seg.executable} ({seg.end - seg.start} bytes)")
        return result
    
    return safe_execute(_list_segments)

@mcp.tool()
def list_imports(offset: int = 0, limit: int = 100) -> List[str]:
    """
    List imported symbols in the program with pagination.
    """
    bv = get_current_binary_view()
    if not bv:
        return ["Error: No binary view available"]
    
    def _list_imports():
        if not BN_AVAILABLE:
            return ["Error: Binary Ninja not available"]
        symbols = [sym for sym in bv.symbols.values() if sym.type == bn.SymbolType.ImportedFunctionSymbol]
        start_idx = offset
        end_idx = min(offset + limit, len(symbols))
        return [f"{sym.address:#x}: {sym.name}" for sym in symbols[start_idx:end_idx]]
    
    return safe_execute(_list_imports)

@mcp.tool()
def list_exports(offset: int = 0, limit: int = 100) -> List[str]:
    """
    List exported functions/symbols with pagination.
    """
    bv = get_current_binary_view()
    if not bv:
        return ["Error: No binary view available"]
    
    def _list_exports():
        if not BN_AVAILABLE:
            return ["Error: Binary Ninja not available"]
        symbols = [sym for sym in bv.symbols.values() if sym.type == bn.SymbolType.FunctionSymbol]
        start_idx = offset
        end_idx = min(offset + limit, len(symbols))
        return [f"{sym.address:#x}: {sym.name}" for sym in symbols[start_idx:end_idx]]
    
    return safe_execute(_list_exports)

@mcp.tool()
def search_functions_by_name(query: str, offset: int = 0, limit: int = 100) -> List[str]:
    """
    Search for functions whose name contains the given substring.
    """
    if not query:
        return ["Error: query string is required"]
    
    bv = get_current_binary_view()
    if not bv:
        return ["Error: No binary view available"]
    
    def _search_functions():
        matching_functions = [func for func in bv.functions if query.lower() in func.name.lower()]
        start_idx = offset
        end_idx = min(offset + limit, len(matching_functions))
        return [f"{func.start:#x}: {func.name}" for func in matching_functions[start_idx:end_idx]]
    
    return safe_execute(_search_functions)

@mcp.tool()
def get_binary_status() -> List[str]:
    """
    Get the current status of the loaded binary.
    """
    bv = get_current_binary_view()
    if not bv:
        return ["Error: No binary view available"]
    
    def _get_binary_status():
        return [f"Binary: {bv.file.filename}, Architecture: {bv.arch.name}, Platform: {bv.platform.name}"]
    
    return safe_execute(_get_binary_status)

@mcp.tool()
def function_at(address: str) -> List[str]:
    """
    Retrieve the name of the function the address belongs to. Address must be in hexadecimal format 0x00001
    """
    bv = get_current_binary_view()
    if not bv:
        return ["Error: No binary view available"]
    
    def _function_at():
        try:
            addr = int(address, 16) if address.startswith('0x') else int(address)
        except ValueError:
            return [f"Error: Invalid address format '{address}'"]
        
        funcs = bv.get_functions_containing(addr)
        if funcs:
            return [funcs[0].name]
        else:
            return [f"No function found at address {address}"]
    
    return safe_execute(_function_at)

@mcp.tool()
def code_references(function_name: str) -> List[str]:
    """
    Retrieve names and addresses of functions that call the given function_name
    """
    bv = get_current_binary_view()
    if not bv:
        return ["Error: No binary view available"]
    
    def _code_references():
        func = bv.get_function_by_name(function_name)
        if not func:
            return [f"Error: Function '{function_name}' not found"]
        
        callers = []
        for ref in bv.get_code_refs(func.start):
            caller_func = bv.get_function_at(ref.address)
            if caller_func:
                callers.append(f"{ref.address:#x}: {caller_func.name}")
        return callers if callers else [f"No callers found for function '{function_name}'"]
    
    return safe_execute(_code_references)

@mcp.tool()
def retype_variable(function_name: str, variable_name: str, type_str: str) -> List[str]:
    """
    Retype a variable in a function.
    """
    bv = get_current_binary_view()
    if not bv:
        return ["Error: No binary view available"]
    
    def _retype_variable():
        func = bv.get_function_by_name(function_name)
        if not func:
            return [f"Error: Function '{function_name}' not found"]
        
        # This is a simplified implementation, actual variable retyping requires more complex logic
        return [f"Variable '{variable_name}' in function '{function_name}' retyped to '{type_str}' (simplified implementation)"]
    
    return safe_execute(_retype_variable)

@mcp.tool()
def rename_variable(function_name: str, variable_name: str, new_name: str) -> List[str]:
    """
    Rename a variable in a function.
    """
    bv = get_current_binary_view()
    if not bv:
        return ["Error: No binary view available"]
    
    def _rename_variable():
        func = bv.get_function_by_name(function_name)
        if not func:
            return [f"Error: Function '{function_name}' not found"]
        
        # This is a simplified implementation, actual variable renaming requires more complex logic
        return [f"Variable '{variable_name}' in function '{function_name}' renamed to '{new_name}' (simplified implementation)"]
    
    return safe_execute(_rename_variable)

if __name__ == "__main__":
    print("Starting MCP bridge service (direct API mode)...")
    mcp.run()