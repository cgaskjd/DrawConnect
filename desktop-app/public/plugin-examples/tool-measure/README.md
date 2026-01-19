# 测量工具插件

这是一个 DrawConnect 工具插件示例，展示如何创建自定义工具。

## 功能

本插件包含三种工具：

### 1. 标尺工具
- 测量两点之间的距离
- 支持像素、厘米、英寸单位切换
- 实时显示水平/垂直距离

### 2. 量角器
- 测量三点形成的角度
- 点击三个点：起点、顶点、终点
- 可视化显示角度弧

### 3. 网格叠加
- 在画布上叠加网格辅助线
- 可调节网格大小和颜色

## 安装

1. 将此文件夹复制到 DrawConnect 插件目录
2. 或通过插件管理器安装

## 使用方法

### 标尺工具
1. 选择标尺工具
2. 在画布上点击并拖动
3. 释放鼠标查看测量结果

### 量角器
1. 选择量角器工具
2. 依次点击三个点
3. 自动计算并显示角度

### 网格叠加
1. 选择网格叠加工具
2. 网格会自动显示在画布上
3. 在设置中调整网格大小

## 开发说明

### 工具 API

```javascript
api.registerTool({
  id: 'tool-id',
  name: '工具名称',
  cursor: 'crosshair',  // 鼠标样式

  // 生命周期回调
  onActivate: function() { },
  onDeactivate: function() { },

  // 鼠标事件
  onMouseDown: function(event) { },
  onMouseMove: function(event) { },
  onMouseUp: function(event) { },

  // 键盘事件
  onKeyDown: function(event) { },
  onKeyUp: function(event) { },

  // 渲染回调（用于绘制工具覆盖层）
  onRender: function(ctx) { },

  // 工具设置
  settings: {
    option: { type: 'number', min: 0, max: 100, default: 50 }
  }
});
```

### 事件对象

```javascript
event = {
  x: number,          // 画布坐标 X
  y: number,          // 画布坐标 Y
  clientX: number,    // 屏幕坐标 X
  clientY: number,    // 屏幕坐标 Y
  button: number,     // 鼠标按钮 (0=左, 1=中, 2=右)
  shiftKey: boolean,  // Shift 键状态
  ctrlKey: boolean,   // Ctrl 键状态
  altKey: boolean     // Alt 键状态
}
```

### 面板 API

```javascript
api.registerPanel({
  id: 'panel-id',
  name: '面板名称',
  render: function() {
    // 返回 HTML 字符串
    return '<div>面板内容</div>';
  }
});
```

## 许可证

MIT License
