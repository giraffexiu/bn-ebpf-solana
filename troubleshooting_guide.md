# eBPF/Solana Binary Analysis Troubleshooting Guide

## Common Issues and Solutions

### Issue: "ELF architecture 247 is not supported"

**Problem**: Binary Ninja's default ELF parser doesn't recognize eBPF architecture (ID 247/0xf7).

**Solution**: This plugin now includes improved error handling:

1. **Automatic Fallback**: When LIEF ELF parsing fails, the plugin automatically falls back to minimal initialization mode.

2. **Minimal Mode Features**:
   - Basic eBPF instruction disassembly
   - Common Solana syscall recognition
   - Memory layout setup (stack, heap, input regions)
   - Entry point detection

3. **Expected Behavior**:
   ```
   [Default] ELF architecture 247 is not supported
   [Default] BinaryView of type 'ELF' failed to initialize!
   [ScriptingProvider] init
   Detected eBPF binary (arch: 247)
   LIEF ELF parsing failed: ...
   This is expected for eBPF binaries with architecture 247
   Falling back to minimal initialization...
   Initializing with minimal eBPF support...
   Minimal eBPF initialization completed
   ```

### Issue: Sections outside address space

**Problem**: ELF sections like `.dynamic`, `.data.rel.ro`, etc. are reported as outside the address space.

**Solution**: 
- This is normal for eBPF binaries due to their specialized memory layout
- The plugin creates custom memory regions for Solana-specific areas
- Essential functionality is preserved through minimal initialization

### Issue: Virtual address conversion failures

**Problem**: Messages like "Can't convert DT_SYMTAB.virtual_address into an offset"

**Solution**:
- These warnings are expected for eBPF binaries
- The plugin provides alternative symbol resolution
- Core analysis functionality remains available

## Usage Tips

1. **Loading eBPF Files**:
   - Open `.so` files directly in Binary Ninja
   - The plugin will automatically detect eBPF/SBPF architecture
   - Ignore initial ELF parsing warnings - they're expected

2. **Analysis Features**:
   - Use the AI-powered sidebar for advanced analysis
   - Batch Rust decompiler for large programs
   - Automatic Rust symbol demangling

3. **Troubleshooting**:
   - Check Binary Ninja's log for detailed error messages
   - Ensure all required Python dependencies are installed
   - Restart Binary Ninja after plugin updates

## Technical Details

### Architecture Support
- eBPF (architecture ID 247/0xf7)
- SBPF (architecture ID 263/0x107)
- Little-endian byte order
- 64-bit address space

### Memory Layout
- Program code: `0x100000000+`
- Stack region: `0x200000000` (32KB)
- Heap region: `0x300000000` (32KB)
- Input region: `0x400000000` (32KB)
- Extern/syscalls: `0x1000-0x3000`

### Fallback Initialization
When full ELF parsing fails, the plugin:
1. Creates basic memory segments
2. Sets up Solana-specific regions
3. Adds common syscall stubs
4. Attempts entry point detection
5. Enables eBPF instruction analysis

This ensures that even problematic eBPF binaries can be analyzed effectively.