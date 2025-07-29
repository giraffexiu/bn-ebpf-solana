# bn-ebpf-solana

### Overview

A comprehensive Binary Ninja plugin for analyzing eBPF and Solana programs. This plugin provides advanced disassembly, decompilation, and analysis capabilities specifically designed for Solana blockchain programs and eBPF bytecode.

### Features

#### Core Architecture Support
- **eBPF Architecture**: Complete instruction decoding and lifting to Binary Ninja's intermediate language
- **Solana Platform**: Specialized support for Solana program binaries with ELF parsing
- **Custom Binary View**: Enhanced Solana-specific binary analysis with memory mapping and symbol handling

#### Advanced Analysis Capabilities
- **Instruction Lifting**: Converts eBPF instructions to Binary Ninja's Low Level IL (LLIL)
- **Symbol Demangling**: Automatic Rust symbol demangling for better readability
- **Program ID Detection**: Automatically identifies Solana program IDs from entry functions
- **Syscall Handling**: Maps eBPF syscalls to readable function names
- **Memory Mapping**: Creates proper memory segments for Solana program analysis

#### IDL Integration
- **Anchor IDL Support**: Fetches and parses Interface Definition Language files
- **On-chain IDL Retrieval**: Downloads IDL directly from Solana blockchain
- **Type System Integration**: Loads custom Solana types for enhanced analysis

#### AI-Powered Analysis
- **LLM Integration**: Built-in support for Anthropic Claude and Google Gemini
- **Smart Decompilation**: AI-assisted code analysis and explanation
- **Interactive Sidebar**: User-friendly interface for AI-powered reverse engineering

#### Developer Tools
- **MCP Bridge**: Model Context Protocol integration for external tool communication
- **Direct API Interface**: Programmatic access to Binary Ninja functionality
- **Custom UI Components**: Specialized sidebar for Solana program analysis

### Installation

1. Clone this repository to your Binary Ninja plugins directory:
   ```bash
   cd "~/Library/Application Support/Binary Ninja/plugins"
   git clone https://github.com/your-repo/bn-ebpf-solana.git
   ```

2. Install required Python dependencies in Binary Ninja:
   - Open Binary Ninja
   - Press `Cmd+P` (or `Ctrl+P` on Windows/Linux) to open the command palette
   - Type "Install python3 module" and select it
   - Install the following modules one by one:
     - `lief`
     - `anthropic`
     - `fastmcp`
     - `tenacity`
     - `rust_demangler`
     - `pygments`
     - `anchorpy`
     - `solana`
     - `solders`
     - `base58`
     - `google-genai`

3. Restart Binary Ninja to load the plugin

### Usage

#### Basic Analysis
1. Open a Solana program binary (.so file) in Binary Ninja
2. The plugin automatically detects and applies Solana-specific analysis
3. Use the sidebar panel for AI-assisted analysis

#### IDL Integration
- The plugin automatically attempts to fetch IDL files for known programs
- Custom types are loaded to enhance variable and function analysis

#### AI Analysis
1. Configure your API keys in the sidebar settings
2. Select functions or code regions for AI analysis
3. Get detailed explanations and suggestions for reverse engineering

### Configuration

#### API Keys
Configure your LLM provider API keys through the plugin settings:
- Anthropic Claude API key
- Google Gemini API key

#### Custom Types
The plugin includes predefined Solana types in `types.c` for enhanced analysis.

### File Structure

#### Core Plugin Files
- `__init__.py` - Plugin initialization and registration
- `ebpf.py` - eBPF architecture implementation
- `solana.py` - Solana platform definition
- `solanaview.py` - Solana binary view with ELF parsing
- `instr.py` - eBPF instruction handling and IL generation
- `sidebar_ui.py` - AI-powered analysis interface
- `idl_utils.py` - IDL fetching and parsing utilities
- `direct_api_interface.py` - Direct API access layer
- `mcp_utils.py` - MCP protocol utilities
- `types.c` - Solana-specific type definitions

#### Configuration Files
- `plugin.json` - Binary Ninja plugin metadata

#### AI/LLM Integration Files
- `system.txt` - System prompt for BinjaBot AI assistant with Solana/Anchor type definitions
- `context.txt` - MCP (Model Context Protocol) architecture documentation
- `binja_mcp_bridge_direct.py` - MCP bridge implementation for direct API access

### Contributing

Contributions are welcome! Please ensure all code follows the existing patterns and includes appropriate documentation.

### License

This project is licensed under the MIT License.
