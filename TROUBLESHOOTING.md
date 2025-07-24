# 插件故障排除指南

## 问题：看不到"AI reconstructed Rust (Claude/Gemini)"侧边栏面板

### 根本原因
经过检查，发现了以下问题：

1. **主要问题**: `__init__.py` 文件中缺少对 `sidebar_ui` 模块的导入
2. **依赖问题**: `plugin.json` 中缺少必要的依赖包
3. **插件类型**: `plugin.json` 中缺少 "ui" 类型声明

### 已修复的问题

#### 1. 修复了 `__init__.py` 文件
```python
# 在文件末尾添加了:
from . import sidebar_ui
```

#### 2. 修复了 `plugin.json` 文件
- 添加了缺失的依赖包: `anthropic`, `google-genai`
- 添加了 "ui" 插件类型

#### 3. 实现了直接API模式
- 创建了 `binja_mcp_bridge_direct.py` 文件
- 修改了 `sidebar_ui.py` 使用直接API调用

### 如何验证修复

1. **重启Binary Ninja**
   - 完全关闭Binary Ninja
   - 重新启动应用程序

2. **加载二进制文件**
   - 打开任意二进制文件
   - 等待分析完成

3. **查找侧边栏面板**
   - 在右侧侧边栏中寻找带有"R"图标的面板
   - 面板名称: "AI reconstructed Rust (Claude/Gemini)"

4. **配置API密钥**
   - 进入 Binary Ninja 设置
   - 找到 "MCP settings" 部分
   - 设置 Anthropic API Key 或 Gemini API Key
   - 选择 LLM Provider (claude 或 gemini)

### 如果仍然看不到面板

1. **检查Binary Ninja日志**
   - 打开 Binary Ninja 控制台
   - 查看是否有插件加载错误

2. **检查依赖安装**
   ```bash
   pip install anthropic google-genai fastmcp tenacity pygments
   ```

3. **手动重新加载插件**
   - 在Binary Ninja中: Tools → Manage Plugins
   - 找到 "bn-ebpf-solana" 插件
   - 禁用后重新启用

### 使用说明

1. **选择函数**
   - 在反汇编视图中点击任意函数
   - 侧边栏会自动开始分析

2. **查看结果**
   - 等待LLM分析完成
   - 结果会显示在侧边栏中
   - 支持语法高亮的Rust代码

3. **缓存机制**
   - 每个函数的分析结果会被缓存
   - 再次访问同一函数时会立即显示缓存结果

### 新的直接API模式优势

- ✅ 无需启动HTTP服务器
- ✅ 更稳定的连接
- ✅ 更快的响应速度
- ✅ 简化的部署过程
- ✅ 更好的错误处理

### 常见错误信息

- **"Please set your Anthropic API key"**: 需要在设置中配置API密钥
- **"LLM disabled: bad API key"**: API密钥无效或过期
- **"No binary view available"**: 当前没有加载二进制文件
- **"MCP bridge error"**: MCP连接问题，通常重启可解决

### 联系支持

如果问题仍然存在，请提供：
1. Binary Ninja版本
2. 操作系统版本
3. 插件版本
4. 错误日志
5. 具体的重现步骤