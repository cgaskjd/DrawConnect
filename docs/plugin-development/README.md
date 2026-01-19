# DrawConnect 插件开发指南

欢迎来到 DrawConnect 插件开发指南！本文档将帮助您创建自定义插件来扩展 DrawConnect 的功能。

## 目录

1. [快速开始](#快速开始)
2. [插件结构](#插件结构)
3. [插件类型](#插件类型)
4. [权限系统](#权限系统)
5. [API 参考](#api-参考)
6. [示例插件](#示例插件)
7. [调试与发布](#调试与发布)

---

## 快速开始

### 1. 创建插件目录

```bash
mkdir my-plugin
cd my-plugin
```

### 2. 创建 manifest.json

```json
{
  "id": "com.yourname.my-plugin",
  "name": "我的插件",
  "version": "1.0.0",
  "apiVersion": "1.0",
  "description": "这是我的第一个 DrawConnect 插件",
  "author": {
    "name": "您的名字",
    "email": "your@email.com"
  },
  "license": "MIT",
  "type": "filter",
  "runtime": "javascript",
  "main": "main.js",
  "permissions": [
    "canvas:read",
    "canvas:write",
    "filter:register"
  ],
  "capabilities": {
    "filters": [
      {
        "id": "my-filter",
        "name": "我的滤镜",
        "category": "自定义"
      }
    ]
  }
}
```

### 3. 创建 main.js

```javascript
/**
 * 插件初始化入口
 * @param {PluginAPI} api - DrawConnect 提供的 API 对象
 */
function initialize(api) {
  console.log('插件已加载');

  // 注册滤镜
  api.registerFilter({
    id: 'my-filter',
    name: '我的滤镜',
    apply: applyMyFilter
  });
}

/**
 * 滤镜处理函数
 * @param {ImageData} imageData - 图像数据
 * @param {Object} settings - 用户设置
 * @returns {ImageData} 处理后的图像数据
 */
function applyMyFilter(imageData, settings) {
  const data = imageData.data;

  // 遍历每个像素 (RGBA)
  for (let i = 0; i < data.length; i += 4) {
    const r = data[i];
    const g = data[i + 1];
    const b = data[i + 2];
    // Alpha 通道: data[i + 3]

    // 在这里处理像素...
    // 示例：简单的反色效果
    data[i] = 255 - r;
    data[i + 1] = 255 - g;
    data[i + 2] = 255 - b;
  }

  return imageData;
}

/**
 * 插件卸载时的清理函数
 */
function cleanup() {
  console.log('插件已卸载');
}

// 导出插件接口
module.exports = {
  initialize,
  cleanup
};
```

### 4. 安装插件

1. 打开 DrawConnect
2. 进入 **工具 → 插件管理**
3. 点击 **从文件夹安装**
4. 选择您的插件目录

---

## 插件结构

```
my-plugin/
├── manifest.json      # 必需：插件清单文件
├── main.js            # 必需：JavaScript 入口文件
├── README.md          # 可选：说明文档
├── icon.png           # 可选：插件图标 (64x64 推荐)
├── lib/               # 可选：依赖库目录
│   └── utils.js
└── assets/            # 可选：资源文件目录
    └── brushes/
```

### manifest.json 字段说明

| 字段 | 类型 | 必需 | 说明 |
|------|------|------|------|
| `id` | string | ✅ | 唯一标识符，推荐使用反向域名格式 |
| `name` | string | ✅ | 插件显示名称 |
| `version` | string | ✅ | 语义版本号 (x.y.z) |
| `apiVersion` | string | ✅ | API 版本，目前为 "1.0" |
| `description` | string | ✅ | 插件描述 |
| `author` | object | ✅ | 作者信息 |
| `license` | string | ✅ | 许可证 (MIT, GPL, etc.) |
| `type` | string | ✅ | 插件类型 |
| `runtime` | string | ✅ | 运行时类型 |
| `main` | string | ✅ | 入口文件路径 |
| `permissions` | array | ✅ | 所需权限列表 |
| `capabilities` | object | ✅ | 插件提供的功能 |
| `settings` | object | ❌ | 插件设置 schema |
| `locales` | array | ❌ | 支持的语言 |
| `keywords` | array | ❌ | 搜索关键词 |
| `category` | string | ❌ | 插件分类 |
| `homepage` | string | ❌ | 主页 URL |
| `repository` | string | ❌ | 代码仓库 URL |

---

## 插件类型

### 滤镜插件 (filter)

用于图像处理和特效。

```json
{
  "type": "filter",
  "capabilities": {
    "filters": [
      {
        "id": "unique-filter-id",
        "name": "滤镜显示名称",
        "category": "颜色调整"
      }
    ]
  }
}
```

### 笔刷插件 (brush)

用于自定义绘画笔刷。

```json
{
  "type": "brush",
  "capabilities": {
    "brushes": [
      {
        "id": "unique-brush-id",
        "name": "笔刷显示名称",
        "category": "艺术笔刷"
      }
    ]
  }
}
```

### 工具插件 (tool)

用于添加新的编辑工具。

```json
{
  "type": "tool",
  "capabilities": {
    "tools": [
      {
        "id": "unique-tool-id",
        "name": "工具显示名称"
      }
    ]
  }
}
```

### 混合插件 (mixed)

同时提供多种功能。

```json
{
  "type": "mixed",
  "capabilities": {
    "brushes": [...],
    "filters": [...],
    "tools": [...]
  }
}
```

---

## 权限系统

DrawConnect 使用细粒度权限系统保护用户数据安全。

### 画布权限

| 权限 | 说明 | 危险级别 |
|------|------|----------|
| `canvas:read` | 读取画布像素数据 | 低 |
| `canvas:write` | 修改画布像素数据 | ⚠️ 高 |

### 图层权限

| 权限 | 说明 |
|------|------|
| `layer:read` | 读取图层信息 |
| `layer:write` | 修改图层属性 |
| `layer:active` | 访问当前活跃图层 |
| `layer:pixels` | 访问图层像素数据 |

### 功能注册权限

| 权限 | 说明 |
|------|------|
| `filter:register` | 注册滤镜功能 |
| `filter:apply` | 应用滤镜效果 |
| `brush:register` | 注册自定义笔刷 |
| `brush:render` | 自定义笔刷渲染 |
| `tool:register` | 注册自定义工具 |

### UI 权限

| 权限 | 说明 |
|------|------|
| `ui:panel` | 添加 UI 面板 |
| `ui:menu` | 添加菜单项 |
| `ui:toolbar` | 添加工具栏按钮 |
| `ui:dialog` | 显示对话框 |

### 系统权限

| 权限 | 说明 | 危险级别 |
|------|------|----------|
| `fs:read` | 读取文件（沙箱内） | ⚠️ 高 |
| `fs:write` | 写入文件（沙箱内） | ⚠️ 高 |
| `network:fetch` | 网络请求 | ⚠️ 高 |
| `history:access` | 访问撤销/重做历史 | 低 |
| `selection:read` | 读取选区信息 | 低 |
| `selection:write` | 修改选区 | 中 |

---

## API 参考

### PluginAPI 对象

插件通过 `initialize(api)` 函数接收 API 对象。

#### 滤镜 API

```javascript
// 注册滤镜
api.registerFilter({
  id: 'filter-id',
  name: '滤镜名称',
  apply: function(imageData, settings) {
    // 处理图像
    return imageData;
  },
  // 可选：滤镜设置 UI
  settings: {
    intensity: { type: 'number', min: 0, max: 100, default: 50 }
  }
});
```

#### 笔刷 API

```javascript
// 注册笔刷
api.registerBrush({
  id: 'brush-id',
  name: '笔刷名称',
  render: function(context, stroke, settings) {
    // 渲染笔刷笔触
  },
  settings: {
    size: { type: 'number', min: 1, max: 500, default: 20 },
    opacity: { type: 'number', min: 0, max: 1, default: 1 },
    hardness: { type: 'number', min: 0, max: 1, default: 0.5 }
  }
});
```

#### 工具 API

```javascript
// 注册工具
api.registerTool({
  id: 'tool-id',
  name: '工具名称',
  icon: 'tool-icon.png',
  onActivate: function() {
    console.log('工具已激活');
  },
  onDeactivate: function() {
    console.log('工具已停用');
  },
  onMouseDown: function(event) {
    // 处理鼠标按下
  },
  onMouseMove: function(event) {
    // 处理鼠标移动
  },
  onMouseUp: function(event) {
    // 处理鼠标释放
  }
});
```

#### 画布 API

```javascript
// 获取画布信息
const canvasInfo = api.canvas.getInfo();
// { width, height, dpi }

// 获取当前图层像素数据
const imageData = api.canvas.getImageData();

// 设置像素数据
api.canvas.setImageData(imageData);

// 刷新画布显示
api.canvas.refresh();
```

#### 图层 API

```javascript
// 获取所有图层
const layers = api.layers.getAll();

// 获取活跃图层
const activeLayer = api.layers.getActive();

// 创建新图层
const newLayer = api.layers.create('新图层名称');

// 获取图层像素数据
const pixels = api.layers.getPixels(layerId);
```

#### UI API

```javascript
// 显示通知
api.ui.notify('操作成功', 'success');
api.ui.notify('警告信息', 'warning');
api.ui.notify('错误信息', 'error');

// 显示确认对话框
const result = await api.ui.confirm('确定要执行此操作吗？');

// 显示输入对话框
const value = await api.ui.prompt('请输入数值', '默认值');
```

#### 存储 API

```javascript
// 保存插件设置
api.storage.set('key', value);

// 读取插件设置
const value = api.storage.get('key', defaultValue);

// 删除设置
api.storage.remove('key');
```

---

## 示例插件

### 滤镜插件示例

参见: `/plugin-examples/filter-grayscale/`

### 笔刷插件示例

参见: `/plugin-examples/brush-scatter/`

### 工具插件示例

参见: `/plugin-examples/tool-ruler/`

---

## 调试与发布

### 调试模式

1. 在 DrawConnect 中按 `F12` 打开开发者工具
2. 在控制台查看插件日志
3. 使用 `console.log()` 输出调试信息

### 常见错误

| 错误 | 原因 | 解决方案 |
|------|------|----------|
| "No manifest.json found" | 清单文件缺失或路径错误 | 确保 manifest.json 在插件根目录 |
| "Invalid version format" | 版本号格式错误 | 使用 x.y.z 格式 |
| "Plugin ID cannot be empty" | ID 字段为空 | 填写唯一标识符 |
| "Permission denied" | 权限不足 | 在 manifest 中添加所需权限 |

### 打包发布

1. **创建压缩包**
   ```bash
   cd my-plugin
   zip -r ../my-plugin.dcplugin .
   ```

2. **命名规范**
   - 使用 `.dcplugin` 或 `.zip` 扩展名
   - 推荐命名: `插件名-版本号.dcplugin`

3. **发布渠道**
   - GitHub Releases
   - 个人网站
   - DrawConnect 插件商店（即将上线）

---

## 安全须知

1. **请求最小权限** - 只申请必需的权限
2. **沙箱限制** - 插件运行在沙箱中，有资源限制
3. **代码审查** - 安装第三方插件前请审查代码
4. **数据安全** - 不要在插件中存储敏感信息

---

## 获取帮助

- **文档**: 本目录下的其他文档
- **示例**: `/plugin-examples/` 目录
- **问题反馈**: GitHub Issues

祝您开发愉快！
