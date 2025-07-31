#!/usr/bin/env python3
"""
Test script for bn-ebpf-solana plugin
Run this script to verify the plugin is working correctly
"""

import sys
import os

def test_plugin_imports():
    """Test if all plugin modules can be imported"""
    print("Testing plugin imports...")
    
    try:
        # Test core modules
        from . import ebpf
        print("✓ eBPF architecture module imported")
        
        from . import solana
        print("✓ Solana platform module imported")
        
        from . import solanaview
        print("✓ SolanaView binary view imported")
        
        from . import instr
        print("✓ Instruction handling module imported")
        
        # Test optional modules
        try:
            from . import sidebar_ui
            print("✓ Sidebar UI module imported")
        except ImportError as e:
            print(f"⚠ Sidebar UI module failed: {e}")
        
        try:
            from . import batch_rust_decompiler
            print("✓ Batch Rust decompiler imported")
        except ImportError as e:
            print(f"⚠ Batch Rust decompiler failed: {e}")
        
        return True
        
    except ImportError as e:
        print(f"✗ Import failed: {e}")
        return False

def test_dependencies():
    """Test if required dependencies are available"""
    print("\nTesting dependencies...")
    
    dependencies = [
        ('lief', 'ELF parsing'),
        ('rust_demangler', 'Rust symbol demangling'),
        ('binaryninja', 'Binary Ninja API')
    ]
    
    optional_deps = [
        ('google.genai', 'Google Gemini AI'),
        ('pygments', 'Syntax highlighting'),
        ('fastmcp', 'MCP protocol'),
        ('tenacity', 'Retry logic')
    ]
    
    all_good = True
    
    for dep, desc in dependencies:
        try:
            __import__(dep)
            print(f"✓ {dep} ({desc})")
        except ImportError:
            print(f"✗ {dep} ({desc}) - REQUIRED")
            all_good = False
    
    for dep, desc in optional_deps:
        try:
            __import__(dep)
            print(f"✓ {dep} ({desc})")
        except ImportError:
            print(f"⚠ {dep} ({desc}) - optional")
    
    return all_good

def test_architecture_registration():
    """Test if eBPF architecture is properly registered"""
    print("\nTesting architecture registration...")
    
    try:
        import binaryninja as bn
        
        # Check if eBPF architecture is registered
        if 'ebpf' in bn.Architecture:
            print("✓ eBPF architecture registered")
            arch = bn.Architecture['ebpf']
            print(f"  - Name: {arch.name}")
            print(f"  - Address size: {arch.address_size}")
            print(f"  - Max instruction length: {arch.max_instr_length}")
            return True
        else:
            print("✗ eBPF architecture not found")
            return False
            
    except Exception as e:
        print(f"✗ Architecture test failed: {e}")
        return False

def test_platform_registration():
    """Test if Solana platform is properly registered"""
    print("\nTesting platform registration...")
    
    try:
        import binaryninja as bn
        
        # Check if Solana platform is registered
        if 'Solana' in bn.Platform:
            print("✓ Solana platform registered")
            platform = bn.Platform['Solana']
            print(f"  - Name: {platform.name}")
            print(f"  - Architecture: {platform.arch.name if platform.arch else 'None'}")
            return True
        else:
            print("✗ Solana platform not found")
            return False
            
    except Exception as e:
        print(f"✗ Platform test failed: {e}")
        return False

def main():
    """Run all tests"""
    print("bn-ebpf-solana Plugin Test Suite")
    print("=" * 40)
    
    tests = [
        test_dependencies,
        test_plugin_imports,
        test_architecture_registration,
        test_platform_registration
    ]
    
    results = []
    for test in tests:
        try:
            result = test()
            results.append(result)
        except Exception as e:
            print(f"✗ Test {test.__name__} crashed: {e}")
            results.append(False)
    
    print("\n" + "=" * 40)
    print("Test Summary:")
    passed = sum(results)
    total = len(results)
    print(f"Passed: {passed}/{total}")
    
    if passed == total:
        print("🎉 All tests passed! Plugin should work correctly.")
    else:
        print("⚠ Some tests failed. Check the output above for details.")
        print("\nCommon solutions:")
        print("- Install missing dependencies using Binary Ninja's package manager")
        print("- Restart Binary Ninja after installing the plugin")
        print("- Check the troubleshooting guide for more help")

if __name__ == '__main__':
    main()