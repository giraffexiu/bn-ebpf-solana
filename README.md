# bn-ebpf-solana

[English](#english) | [中文](#中文)

---

## English

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

---

## 中文

### 概述

一个用于分析 eBPF 和 Solana 程序的综合性 Binary Ninja 插件。该插件提供专为 Solana 区块链程序和 eBPF 字节码设计的高级反汇编、反编译和分析功能。

### 功能特性

#### 核心架构支持
- **eBPF 架构**：完整的指令解码和提升到 Binary Ninja 中间语言
- **Solana 平台**：专门支持 Solana 程序二进制文件的 ELF 解析
- **自定义二进制视图**：增强的 Solana 特定二进制分析，包含内存映射和符号处理

#### 高级分析能力
- **指令提升**：将 eBPF 指令转换为 Binary Ninja 的低级中间语言 (LLIL)
- **符号解码**：自动 Rust 符号解码以提高可读性
- **程序 ID 检测**：从入口函数自动识别 Solana 程序 ID
- **系统调用处理**：将 eBPF 系统调用映射为可读的函数名
- **内存映射**：为 Solana 程序分析创建适当的内存段

#### IDL 集成
- **Anchor IDL 支持**：获取和解析接口定义语言文件
- **链上 IDL 检索**：直接从 Solana 区块链下载 IDL
- **类型系统集成**：加载自定义 Solana 类型以增强分析

#### AI 驱动分析
- **LLM 集成**：内置支持 Anthropic Claude 和 Google Gemini
- **智能反编译**：AI 辅助代码分析和解释
- **交互式侧边栏**：用户友好的 AI 驱动逆向工程界面

#### 开发者工具
- **MCP 桥接**：模型上下文协议集成，用于外部工具通信
- **直接 API 接口**：对 Binary Ninja 功能的编程访问
- **自定义 UI 组件**：专门用于 Solana 程序分析的侧边栏

### 安装

1. 将此仓库克隆到您的 Binary Ninja 插件目录：
   ```bash
   cd "~/Library/Application Support/Binary Ninja/plugins"
   git clone https://github.com/your-repo/bn-ebpf-solana.git
   ```

2. 在 Binary Ninja 中安装所需的 Python 依赖：
   - 打开 Binary Ninja
   - 按 `Cmd+P`（Windows/Linux 上为 `Ctrl+P`）打开命令面板
   - 输入 "Install python3 module" 并选择
   - 逐个安装以下模块：
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

3. 重启 Binary Ninja 以加载插件

### 使用方法

#### 基础分析
1. 在 Binary Ninja 中打开 Solana 程序二进制文件（.so 文件）
2. 插件自动检测并应用 Solana 特定分析
3. 使用侧边栏面板进行 AI 辅助分析

#### IDL 集成
- 插件自动尝试为已知程序获取 IDL 文件
- 加载自定义类型以增强变量和函数分析

#### AI 分析
1. 在侧边栏设置中配置您的 API 密钥
2. 选择函数或代码区域进行 AI 分析
3. 获取详细的逆向工程解释和建议

### 配置

#### API 密钥
通过插件设置配置您的 LLM 提供商 API 密钥：
- Anthropic Claude API 密钥
- Google Gemini API 密钥

#### 自定义类型
插件在 `types.c` 中包含预定义的 Solana 类型以增强分析。

### 文件结构

#### 核心插件文件
- `__init__.py` - 插件初始化和注册
- `ebpf.py` - eBPF 架构实现
- `solana.py` - Solana 平台定义
- `solanaview.py` - 带 ELF 解析的 Solana 二进制视图
- `instr.py` - eBPF 指令处理和 IL 生成
- `sidebar_ui.py` - AI 驱动分析界面
- `idl_utils.py` - IDL 获取和解析工具
- `direct_api_interface.py` - 直接 API 访问层
- `mcp_utils.py` - MCP 协议工具
- `types.c` - Solana 特定类型定义

#### 配置文件
- `plugin.json` - Binary Ninja 插件元数据
- `requirements.txt` - Python 依赖列表

#### AI/LLM 集成文件
- `system.txt` - BinjaBot AI 助手的系统提示，包含 Solana/Anchor 类型定义
- `context.txt` - MCP（模型上下文协议）架构文档
- `binja_mcp_bridge_direct.py` - 用于直接 API 访问的 MCP 桥接实现

### 贡献

欢迎贡献！请确保所有代码遵循现有模式并包含适当的文档。

### 许可证

本项目采用 MIT 许可证。
