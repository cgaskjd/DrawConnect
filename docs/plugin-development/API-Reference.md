# DrawConnect 插件 API 参考

本文档提供 DrawConnect 插件 API 的完整参考。

## 目录

1. [PluginAPI 对象](#pluginapi-对象)
2. [Canvas API](#canvas-api)
3. [Layer API](#layer-api)
4. [Brush API](#brush-api)
5. [Filter API](#filter-api)
6. [Tool API](#tool-api)
7. [UI API](#ui-api)
8. [Storage API](#storage-api)
9. [类型定义](#类型定义)

---

## PluginAPI 对象

插件通过 `initialize(api)` 函数接收 PluginAPI 对象。

```typescript
interface PluginAPI {
  // 子系统
  canvas: CanvasAPI;
  layers: LayerAPI;
  ui: UIApi;
  storage: StorageAPI;

  // 注册函数
  registerBrush(brush: BrushDefinition): void;
  registerFilter(filter: FilterDefinition): void;
  registerTool(tool: ToolDefinition): void;
  registerPanel(panel: PanelDefinition): void;

  // 工具函数
  log(message: string): void;
  getSettings(): PluginSettings;
}
```

---

## Canvas API

### `canvas.getInfo()`

获取画布信息。

```javascript
const info = api.canvas.getInfo();
// 返回: { width: number, height: number, dpi: number }
```

### `canvas.getImageData(x?, y?, width?, height?)`

获取画布像素数据。

```javascript
// 获取整个画布
const imageData = api.canvas.getImageData();

// 获取指定区域
const regionData = api.canvas.getImageData(100, 100, 200, 200);

// 返回: ImageData { width, height, data: Uint8ClampedArray }
```

### `canvas.setImageData(imageData, x?, y?)`

设置画布像素数据。

```javascript
// 设置到原点
api.canvas.setImageData(imageData);

// 设置到指定位置
api.canvas.setImageData(imageData, 100, 100);
```

### `canvas.refresh()`

刷新画布显示。

```javascript
api.canvas.refresh();
```

### `canvas.getPixel(x, y)`

获取指定位置的像素颜色。

```javascript
const color = api.canvas.getPixel(100, 100);
// 返回: { r: number, g: number, b: number, a: number }
```

### `canvas.setPixel(x, y, color)`

设置指定位置的像素颜色。

```javascript
api.canvas.setPixel(100, 100, { r: 255, g: 0, b: 0, a: 255 });
```

---

## Layer API

### `layers.getAll()`

获取所有图层。

```javascript
const allLayers = api.layers.getAll();
// 返回: LayerInfo[]
```

### `layers.getActive()`

获取当前活跃图层。

```javascript
const activeLayer = api.layers.getActive();
// 返回: LayerInfo | null
```

### `layers.getById(id)`

根据 ID 获取图层。

```javascript
const layer = api.layers.getById('layer-uuid');
// 返回: LayerInfo | null
```

### `layers.create(name)`

创建新图层。

```javascript
const newLayer = api.layers.create('新图层');
// 返回: LayerInfo
```

### `layers.delete(id)`

删除图层。

```javascript
api.layers.delete('layer-uuid');
```

### `layers.setActive(id)`

设置活跃图层。

```javascript
api.layers.setActive('layer-uuid');
```

### `layers.getPixels(id)`

获取图层像素数据。

```javascript
const pixels = api.layers.getPixels('layer-uuid');
// 返回: Uint8ClampedArray
```

### `layers.setPixels(id, pixels)`

设置图层像素数据。

```javascript
api.layers.setPixels('layer-uuid', pixelArray);
```

---

## Brush API

### `registerBrush(definition)`

注册自定义笔刷。

```typescript
interface BrushDefinition {
  id: string;           // 唯一标识符
  name: string;         // 显示名称
  category?: string;    // 分类
  icon?: string;        // 图标路径

  // 渲染函数
  render: (
    ctx: CanvasRenderingContext2D,
    point: StrokePoint,
    settings: BrushSettings,
    color: RGBA
  ) => void;

  // 可选: 初始化函数
  init?: () => void;

  // 可选: 清理函数
  cleanup?: () => void;

  // 设置定义
  settings?: {
    [key: string]: SettingDefinition;
  };
}
```

### 笔刷渲染函数参数

```typescript
interface StrokePoint {
  x: number;            // X 坐标
  y: number;            // Y 坐标
  pressure: number;     // 压力值 (0-1)
  tiltX: number;        // X 倾斜角度
  tiltY: number;        // Y 倾斜角度
  timestamp: number;    // 时间戳
}

interface BrushSettings {
  size: number;         // 笔刷大小
  opacity: number;      // 不透明度
  hardness: number;     // 硬度
  spacing: number;      // 间距
  [key: string]: any;   // 自定义设置
}

interface RGBA {
  r: number;  // 0-255
  g: number;  // 0-255
  b: number;  // 0-255
  a: number;  // 0-255
}
```

### 示例

```javascript
api.registerBrush({
  id: 'my-brush',
  name: '我的笔刷',
  category: '自定义',

  render(ctx, point, settings, color) {
    ctx.fillStyle = `rgba(${color.r}, ${color.g}, ${color.b}, ${settings.opacity})`;
    ctx.beginPath();
    ctx.arc(point.x, point.y, settings.size / 2 * point.pressure, 0, Math.PI * 2);
    ctx.fill();
  },

  settings: {
    size: { type: 'number', min: 1, max: 500, default: 20 },
    opacity: { type: 'number', min: 0, max: 1, default: 1 }
  }
});
```

---

## Filter API

### `registerFilter(definition)`

注册自定义滤镜。

```typescript
interface FilterDefinition {
  id: string;           // 唯一标识符
  name: string;         // 显示名称
  category?: string;    // 分类

  // 应用函数
  apply: (
    imageData: ImageData,
    settings: FilterSettings
  ) => ImageData;

  // 设置定义
  settings?: {
    [key: string]: SettingDefinition;
  };
}
```

### 滤镜应用函数

```javascript
function apply(imageData, settings) {
  const data = imageData.data;

  for (let i = 0; i < data.length; i += 4) {
    // data[i]     - Red
    // data[i + 1] - Green
    // data[i + 2] - Blue
    // data[i + 3] - Alpha

    // 处理像素...
  }

  return imageData;
}
```

### 示例

```javascript
api.registerFilter({
  id: 'my-filter',
  name: '我的滤镜',
  category: '颜色',

  apply(imageData, settings) {
    const data = imageData.data;
    const intensity = settings.intensity / 100;

    for (let i = 0; i < data.length; i += 4) {
      const gray = (data[i] + data[i + 1] + data[i + 2]) / 3;
      data[i] = data[i] + (gray - data[i]) * intensity;
      data[i + 1] = data[i + 1] + (gray - data[i + 1]) * intensity;
      data[i + 2] = data[i + 2] + (gray - data[i + 2]) * intensity;
    }

    return imageData;
  },

  settings: {
    intensity: { type: 'number', min: 0, max: 100, default: 50, label: '强度' }
  }
});
```

---

## Tool API

### `registerTool(definition)`

注册自定义工具。

```typescript
interface ToolDefinition {
  id: string;           // 唯一标识符
  name: string;         // 显示名称
  icon?: string;        // 图标路径
  cursor?: string;      // 鼠标样式

  // 生命周期回调
  onActivate?: () => void;
  onDeactivate?: () => void;

  // 鼠标事件
  onMouseDown?: (event: ToolEvent) => void;
  onMouseMove?: (event: ToolEvent) => void;
  onMouseUp?: (event: ToolEvent) => void;
  onMouseEnter?: (event: ToolEvent) => void;
  onMouseLeave?: (event: ToolEvent) => void;
  onClick?: (event: ToolEvent) => void;
  onDoubleClick?: (event: ToolEvent) => void;
  onWheel?: (event: WheelEvent) => void;

  // 键盘事件
  onKeyDown?: (event: KeyboardEvent) => void;
  onKeyUp?: (event: KeyboardEvent) => void;

  // 渲染回调
  onRender?: (ctx: CanvasRenderingContext2D, settings: ToolSettings) => void;

  // 设置定义
  settings?: {
    [key: string]: SettingDefinition;
  };
}

interface ToolEvent {
  x: number;            // 画布坐标 X
  y: number;            // 画布坐标 Y
  clientX: number;      // 屏幕坐标 X
  clientY: number;      // 屏幕坐标 Y
  button: number;       // 鼠标按钮
  buttons: number;      // 按下的按钮位掩码
  shiftKey: boolean;    // Shift 键
  ctrlKey: boolean;     // Ctrl 键
  altKey: boolean;      // Alt 键
  metaKey: boolean;     // Meta 键 (Cmd/Win)
}
```

### 示例

```javascript
api.registerTool({
  id: 'my-tool',
  name: '我的工具',
  cursor: 'crosshair',

  onActivate() {
    console.log('工具已激活');
  },

  onDeactivate() {
    console.log('工具已停用');
  },

  onMouseDown(event) {
    console.log(`点击位置: (${event.x}, ${event.y})`);
  },

  onRender(ctx, settings) {
    // 绘制工具覆盖层
    ctx.strokeStyle = '#FF0000';
    ctx.strokeRect(0, 0, 100, 100);
  }
});
```

---

## UI API

### `ui.notify(message, type?)`

显示通知消息。

```javascript
api.ui.notify('操作成功');
api.ui.notify('操作成功', 'success');
api.ui.notify('警告信息', 'warning');
api.ui.notify('错误信息', 'error');
api.ui.notify('提示信息', 'info');
```

### `ui.confirm(message)`

显示确认对话框。

```javascript
const confirmed = await api.ui.confirm('确定要执行此操作吗？');
if (confirmed) {
  // 用户点击了确定
}
```

### `ui.prompt(message, defaultValue?)`

显示输入对话框。

```javascript
const value = await api.ui.prompt('请输入名称', '默认值');
if (value !== null) {
  // 用户输入了值
}
```

### `ui.alert(message)`

显示警告对话框。

```javascript
await api.ui.alert('这是一条重要消息');
```

### `registerPanel(definition)`

注册自定义面板。

```typescript
interface PanelDefinition {
  id: string;           // 唯一标识符
  name: string;         // 显示名称
  position?: 'left' | 'right' | 'bottom';  // 位置

  // 渲染函数（返回 HTML 字符串）
  render: () => string;

  // 可选: 事件处理
  onMount?: (element: HTMLElement) => void;
  onUnmount?: () => void;
}
```

---

## Storage API

### `storage.set(key, value)`

保存数据。

```javascript
api.storage.set('myKey', { foo: 'bar' });
api.storage.set('count', 42);
api.storage.set('enabled', true);
```

### `storage.get(key, defaultValue?)`

读取数据。

```javascript
const value = api.storage.get('myKey');
const count = api.storage.get('count', 0);
```

### `storage.remove(key)`

删除数据。

```javascript
api.storage.remove('myKey');
```

### `storage.clear()`

清除所有数据。

```javascript
api.storage.clear();
```

### `storage.keys()`

获取所有键。

```javascript
const keys = api.storage.keys();
// 返回: string[]
```

---

## 类型定义

### SettingDefinition

设置项定义。

```typescript
interface SettingDefinition {
  type: 'number' | 'string' | 'boolean' | 'color' | 'select';
  label?: string;       // 显示标签
  description?: string; // 描述

  // number 类型
  min?: number;
  max?: number;
  step?: number;
  default?: number;

  // string 类型
  default?: string;
  maxLength?: number;

  // boolean 类型
  default?: boolean;

  // color 类型
  default?: string;     // 十六进制颜色

  // select 类型
  options?: Array<{ value: string; label: string }>;
  default?: string;
}
```

### LayerInfo

图层信息。

```typescript
interface LayerInfo {
  id: string;
  name: string;
  visible: boolean;
  locked: boolean;
  opacity: number;      // 0-1
  blendMode: string;
  width: number;
  height: number;
}
```

### ImageData

标准 Web ImageData 对象。

```typescript
interface ImageData {
  width: number;
  height: number;
  data: Uint8ClampedArray;  // RGBA 像素数据
}
```

---

## 最佳实践

### 1. 错误处理

```javascript
function applyFilter(imageData, settings) {
  try {
    // 处理逻辑
    return imageData;
  } catch (error) {
    api.log(`滤镜错误: ${error.message}`);
    api.ui.notify('滤镜应用失败', 'error');
    return imageData;  // 返回原始数据
  }
}
```

### 2. 性能优化

```javascript
// 使用 TypedArray 方法
function processPixels(imageData) {
  const data = imageData.data;
  const len = data.length;

  // 避免在循环中创建对象
  for (let i = 0; i < len; i += 4) {
    // 直接操作数组
  }
}
```

### 3. 内存管理

```javascript
let cachedData = null;

function initialize(api) {
  // 初始化时创建
  cachedData = new Uint8Array(1024);
}

function cleanup() {
  // 清理时释放
  cachedData = null;
}
```

### 4. 状态管理

```javascript
// 使用闭包保存状态
const createTool = () => {
  let state = { isDrawing: false };

  return {
    onMouseDown() { state.isDrawing = true; },
    onMouseUp() { state.isDrawing = false; }
  };
};
```

---

## 调试

### 控制台日志

```javascript
api.log('调试信息');
console.log('标准日志');
console.warn('警告');
console.error('错误');
```

### 开发者工具

在 DrawConnect 中按 `F12` 打开开发者工具。

---

## 版本兼容性

| API 版本 | DrawConnect 版本 | 说明 |
|----------|------------------|------|
| 1.0 | 1.0.0+ | 初始版本 |

---

## 参考资源

- [插件开发指南](./README.md)
- [示例插件](../public/plugin-examples/)
- [类型定义文件](./types.d.ts)
