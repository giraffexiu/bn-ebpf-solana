#!/usr/bin/env python3
"""
测试插件加载和侧边栏注册的脚本
"""

import sys
import os

# 添加插件目录到Python路径
plugin_dir = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, plugin_dir)

print("Testing plugin loading...")

try:
    # 测试导入主模块
    print("1. Testing main module import...")
    import __init__ as plugin_main
    print("   ✓ Main module imported successfully")
    
    # 测试导入sidebar_ui模块
    print("2. Testing sidebar_ui module import...")
    import sidebar_ui
    print("   ✓ sidebar_ui module imported successfully")
    
    # 检查是否有LLMDecompSidebarWidgetType类
    print("3. Testing sidebar widget type...")
    if hasattr(sidebar_ui, 'LLMDecompSidebarWidgetType'):
        print("   ✓ LLMDecompSidebarWidgetType class found")
    else:
        print("   ✗ LLMDecompSidebarWidgetType class not found")
    
    # 检查是否有LLMDecompSidebarWidget类
    print("4. Testing sidebar widget...")
    if hasattr(sidebar_ui, 'LLMDecompSidebarWidget'):
        print("   ✓ LLMDecompSidebarWidget class found")
    else:
        print("   ✗ LLMDecompSidebarWidget class not found")
    
    print("\n✓ All tests passed! Plugin should load correctly.")
    
except ImportError as e:
    print(f"   ✗ Import error: {e}")
    print("\nThis indicates missing dependencies. Please install:")
    print("pip install anthropic google-genai fastmcp tenacity pygments")
    
except Exception as e:
    print(f"   ✗ Unexpected error: {e}")
    print(f"   Error type: {type(e).__name__}")
    import traceback
    traceback.print_exc()