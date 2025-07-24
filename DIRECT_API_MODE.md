# Binary Ninja MCP 直接API模式使用说明

## 概述

本项目已从HTTP客户端模式改为直接API调用模式，这样可以避免依赖外部HTTP服务器，直接在当前Binary Ninja进程中运行MCP桥接。

## 主要改进

### 1. 消除HTTP依赖
- **之前**: 需要启动Binary Ninja HTTP服务器（端口9009）
- **现在**: 直接调用Binary Ninja Python API，无需HTTP服务器

### 2. 更好的稳定性
- **之前**: 容易出现"Connection closed"错误
- **现在**: 在同一进程中运行，连接更稳定

### 3. 更简单的部署
- **之前**: 需要配置和维护HTTP服务器
- **现在**: 插件自包含，无需额外配置

## 文件结构

```
bn-ebpf-solana/
├── binja_mcp_bridge_direct.py    # 新的直接API模式MCP桥接
├── sidebar_ui.py                 # 修改后的UI，使用直接API模式
├── test_direct_mcp.py            # 测试脚本
└── DIRECT_API_MODE.md            # 本说明文档
```

## 核心组件

### 1. binja_mcp_bridge_direct.py

这是新的MCP桥接文件，主要特点：

- **直接API调用**: 直接使用`binaryninja`模块的API
- **线程安全**: 使用锁机制保护BinaryView实例
- **错误处理**: 优雅处理Binary Ninja不可用的情况
- **完整功能**: 支持所有原有的MCP工具

### 2. 修改后的sidebar_ui.py

主要变更：

```python
# 之前的HTTP模式路径
# SERVER_PATH = base_path / "repositories" / "community" / "plugins" / "fosdickio_binary_ninja_mcp" / "bridge" / "binja_mcp_bridge.py"

# 新的直接API模式路径
SERVER_PATH = Path(__file__).parent / "binja_mcp_bridge_direct.py"

# 在MCP连接前设置BinaryView
binja_mcp_direct.set_current_binary_view(self.bar.bv)
```

## 支持的MCP工具

直接API模式支持以下17个工具：

1. **list_methods** - 列出所有函数名
2. **decompile_function** - 反编译指定函数
3. **fetch_disassembly** - 获取汇编代码
4. **rename_function** - 重命名函数
5. **set_comment** - 设置注释
6. **set_function_comment** - 设置函数注释
7. **get_comment** - 获取注释
8. **get_function_comment** - 获取函数注释
9. **list_segments** - 列出内存段
10. **list_imports** - 列出导入符号
11. **list_exports** - 列出导出符号
12. **search_functions_by_name** - 按名称搜索函数
13. **get_binary_status** - 获取二进制状态
14. **function_at** - 获取地址处的函数
15. **code_references** - 获取函数调用关系
16. **retype_variable** - 重新类型化变量
17. **rename_variable** - 重命名变量

## 使用方法

### 1. 在Binary Ninja中使用

当你在Binary Ninja中打开一个二进制文件时，侧边栏会自动使用新的直接API模式：

1. 打开Binary Ninja
2. 加载一个二进制文件
3. 侧边栏会自动检测并使用直接API模式
4. LLM（Claude/Gemini）会通过MCP工具分析二进制文件

### 2. 独立测试

使用提供的测试脚本：

```bash
cd '/Users/giraffe/Library/Application Support/Binary Ninja/plugins/bn-ebpf-solana'
python3 test_direct_mcp.py
```

预期输出：
```
开始测试直接API模式的MCP桥接...
✓ MCP客户端连接成功
✓ 可用工具数量: 17
✓ 工具调用测试: Error: No binary view available (预期错误)
✅ 直接API模式的MCP桥接测试成功！
```

## 错误处理

### 1. Binary Ninja不可用

当在没有Binary Ninja环境中运行时：
```python
Warning: Binary Ninja not available, running in mock mode
```

### 2. 没有加载二进制文件

当没有BinaryView实例时：
```
Error: No binary view available
```

### 3. 函数不存在

当请求的函数不存在时：
```
Error: Function 'function_name' not found
```

## 迁移指南

如果你之前使用HTTP模式，迁移到直接API模式很简单：

1. **无需启动HTTP服务器** - 删除任何启动HTTP服务器的代码
2. **更新路径配置** - 已自动更新为使用`binja_mcp_bridge_direct.py`
3. **无需端口配置** - 不再需要配置9009端口

## 性能优势

- **更快的响应** - 无HTTP网络开销
- **更低的资源使用** - 无需额外的HTTP服务器进程
- **更好的错误处理** - 直接异常传播，更容易调试

## 故障排除

### 问题：MCP连接失败

**解决方案**：
1. 确保`binja_mcp_bridge_direct.py`文件存在
2. 检查Python环境是否正确安装了依赖
3. 查看错误日志获取详细信息

### 问题：工具调用返回错误

**解决方案**：
1. 确保在Binary Ninja环境中运行
2. 确保已加载二进制文件
3. 检查函数名或地址是否正确

## 总结

直接API模式提供了更稳定、更高效的MCP桥接解决方案，消除了对外部HTTP服务器的依赖，同时保持了所有原有功能。这个改进大大简化了部署和维护过程。