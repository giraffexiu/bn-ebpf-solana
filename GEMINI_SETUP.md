# Gemini MCP Integration Setup Guide

本插件现在支持Google Gemini作为LLM提供商，与现有的Claude支持并行工作。

## 功能特性

- 支持Claude和Gemini两种LLM提供商
- 使用Model Context Protocol (MCP) 进行工具集成
- 自动工具调用和函数执行
- 统一的用户界面和配置管理

## 安装要求

### 1. 安装依赖

确保安装了所有必需的Python包：

```bash
pip install -r requirements.txt
```

新增的依赖包括：
- `google-genai`: Google Gemini API客户端

### 2. 获取API密钥

#### Gemini API密钥
1. 访问 [Google AI Studio](https://aistudio.google.com/app/apikey)
2. 创建新的API密钥
3. 保存密钥以备后用

#### Claude API密钥（可选）
1. 访问 [Anthropic Console](https://console.anthropic.com/)
2. 创建API密钥
3. 保存密钥

## 配置设置

在Binary Ninja中配置插件：

1. 打开 **Edit → Preferences → Settings**
2. 导航到 **MCP settings** 部分
3. 配置以下设置：

### 必需设置

- **LLM Provider**: 选择 `claude` 或 `gemini`
- **Gemini API Key**: 输入您的Gemini API密钥（如果选择Gemini）
- **Anthropic API Key**: 输入您的Claude API密钥（如果选择Claude）

### 可选设置

- **Context for Solana MCP**: IDL文件的绝对路径，用于提供额外上下文

## 使用方法

1. **选择LLM提供商**：在设置中选择 `gemini` 或 `claude`
2. **设置API密钥**：根据选择的提供商输入相应的API密钥
3. **打开Solana二进制文件**：在Binary Ninja中加载eBPF/Solana程序
4. **查看侧边栏**："AI reconstructed Rust (Claude/Gemini)" 面板将显示重构的Rust代码
5. **导航函数**：点击不同函数时，AI将自动分析并生成改进的代码

## 支持的模型

### Gemini模型
- `gemini-2.0-flash` (默认): 快速、高效的模型
- 支持MCP工具集成
- 优秀的代码生成能力

### Claude模型
- `claude-3-5-sonnet-20241022`: 高质量的代码分析和生成

## 故障排除

### 常见问题

1. **"Please set your Gemini API key"**
   - 确保在设置中正确配置了Gemini API密钥
   - 验证密钥格式正确

2. **"MCP bridge error"**
   - 确保 `binary_ninja_mcp` 插件已正确安装
   - 检查MCP服务器路径是否正确

3. **"LLM disabled: authentication"**
   - 验证API密钥是否有效
   - 检查网络连接

### 测试安装

运行测试脚本验证Gemini集成：

```bash
export GEMINI_API_KEY="your-api-key-here"
python test_gemini.py
```

## 性能对比

| 特性 | Claude | Gemini |
|------|--------|--------|
| 代码生成质量 | 优秀 | 优秀 |
| 响应速度 | 中等 | 快速 |
| MCP集成 | 手动实现 | 原生支持 |
| 成本 | 中等 | 较低 |
| 工具调用 | 支持 | 自动化 |

## 开发说明

### 架构概述

- `LLMRunner`: 基础抽象类
- `ClaudeRunner`: Claude API实现
- `GeminiRunner`: Gemini API实现
- 统一的MCP客户端集成
- 动态提供商选择

### 扩展支持

要添加新的LLM提供商：

1. 继承 `LLMRunner` 类
2. 实现 `run()` 和相应的API调用方法
3. 在设置中添加新的提供商选项
4. 更新 `_update()` 方法中的提供商选择逻辑

## 许可证

本扩展遵循与原插件相同的许可证条款。