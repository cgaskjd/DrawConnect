# 散点笔刷插件

这是一个 DrawConnect 笔刷插件示例，展示如何创建自定义笔刷。

## 功能

本插件包含三种笔刷：

### 1. 散点圆点
- 在笔触周围随机散布圆点
- 可调节散布范围和数量

### 2. 散点星星
- 绘制随机位置和旋转的星星形状
- 可调节星角数量

### 3. 喷漆效果
- 模拟喷漆罐的效果
- 使用高斯分布让喷点集中在中心

## 安装

1. 将此文件夹复制到 DrawConnect 插件目录
2. 或通过插件管理器安装

## 设置选项

| 设置 | 类型 | 范围 | 说明 |
|------|------|------|------|
| size | 数值 | 1-300 | 笔刷大小 |
| opacity | 数值 | 0-1 | 不透明度 |
| scatter | 数值 | 0-150 | 散布程度 |
| count/density | 数值 | 1-100 | 点数/密度 |

## 开发说明

### 笔刷 API

```javascript
api.registerBrush({
  id: 'brush-id',
  name: '笔刷名称',
  render: function(ctx, point, settings, color) {
    // ctx: CanvasRenderingContext2D
    // point: { x, y, pressure, tiltX, tiltY }
    // settings: 用户设置值
    // color: { r, g, b, a }
  },
  settings: {
    // 定义可调节的设置项
  }
});
```

### 笔触点数据

```javascript
point = {
  x: number,        // X 坐标
  y: number,        // Y 坐标
  pressure: number, // 压力值 0-1
  tiltX: number,    // X 倾斜角度
  tiltY: number,    // Y 倾斜角度
  timestamp: number // 时间戳
}
```

## 许可证

MIT License
