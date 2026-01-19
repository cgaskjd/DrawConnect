# DrawConnect 插件开发指南

## 插件结构

一个有效的 DrawConnect 插件必须包含以下结构：

```
my-plugin/
├── manifest.json    # 必需：插件清单文件
├── main.js          # 必需：JavaScript 入口文件
├── README.md        # 可选：说明文档
└── icon.png         # 可选：插件图标 (64x64)
```

## manifest.json 必需字段

| 字段 | 类型 | 说明 |
|------|------|------|
| `id` | string | 唯一标识符，如 "com.example.my-plugin" |
| `name` | string | 插件显示名称 |
| `version` | string | 语义版本号，如 "1.0.0" |
| `apiVersion` | string | API 版本，目前为 "1.0" |
| `description` | string | 插件描述 |
| `author` | object | 作者信息 `{ "name": "作者名" }` |
| `license` | string | 许可证，如 "MIT" |
| `type` | string | 插件类型: "brush", "filter", "tool", "mixed" |
| `runtime` | string | 运行时: "javascript", "wasm", "hybrid" |
| `main` | string | JavaScript 入口文件路径 |
| `capabilities` | object | 插件提供的功能 |

## 插件类型

### 滤镜插件 (filter)

```json
{
  "type": "filter",
  "capabilities": {
    "filters": [
      { "id": "my-filter", "name": "我的滤镜", "category": "颜色" }
    ]
  }
}
```

### 笔刷插件 (brush)

```json
{
  "type": "brush",
  "capabilities": {
    "brushes": [
      { "id": "my-brush", "name": "我的笔刷", "category": "艺术" }
    ]
  }
}
```

### 工具插件 (tool)

```json
{
  "type": "tool",
  "capabilities": {
    "tools": [
      { "id": "my-tool", "name": "我的工具" }
    ]
  }
}
```

## 安装插件

### 方法 1: 从文件夹安装
1. 打开插件管理器
2. 点击「从文件夹安装」
3. 选择包含 manifest.json 的插件目录

### 方法 2: 从压缩包安装
1. 将插件目录打包为 .zip 或 .dcplugin 文件
2. 打开插件管理器
3. 点击「安装插件」
4. 选择压缩包文件

**注意**：压缩包内 manifest.json 必须在根目录或一级子目录中。

## 常见错误

### "No manifest.json found in plugin archive"
- 确保 manifest.json 存在于压缩包根目录
- 检查压缩包结构，避免嵌套过深
- 不要将整个项目目录打包，只打包插件文件夹内容

### "Plugin ID cannot be empty"
- 确保 manifest.json 中的 `id` 字段不为空

### "Invalid version format"
- 版本号必须是语义版本格式，如 "1.0.0" 或 "1.0"

### "Filter plugins must define at least one filter"
- 如果 `type` 为 "filter"，必须在 capabilities.filters 中定义至少一个滤镜
