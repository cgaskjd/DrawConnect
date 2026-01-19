/**
 * DrawConnect 散点笔刷插件示例
 *
 * 这个插件展示了如何创建自定义笔刷，包括：
 * - 散点圆点笔刷
 * - 散点星星笔刷
 * - 喷漆效果笔刷
 */

// 插件状态
let pluginApi = null;

/**
 * 插件初始化
 * @param {PluginAPI} api - DrawConnect API 对象
 */
function initialize(api) {
  pluginApi = api;
  console.log('散点笔刷插件已加载');

  // 注册散点圆点笔刷
  api.registerBrush({
    id: 'scatter-dots',
    name: '散点圆点',
    render: renderScatterDots,
    settings: {
      size: { type: 'number', min: 1, max: 200, default: 30, label: '大小' },
      opacity: { type: 'number', min: 0, max: 1, default: 0.8, label: '不透明度' },
      scatter: { type: 'number', min: 0, max: 100, default: 50, label: '散布' },
      count: { type: 'number', min: 1, max: 30, default: 5, label: '数量' }
    }
  });

  // 注册散点星星笔刷
  api.registerBrush({
    id: 'scatter-stars',
    name: '散点星星',
    render: renderScatterStars,
    settings: {
      size: { type: 'number', min: 5, max: 100, default: 20, label: '大小' },
      opacity: { type: 'number', min: 0, max: 1, default: 1, label: '不透明度' },
      scatter: { type: 'number', min: 0, max: 150, default: 80, label: '散布' },
      points: { type: 'number', min: 4, max: 12, default: 5, label: '星角数' }
    }
  });

  // 注册喷漆效果笔刷
  api.registerBrush({
    id: 'spray-paint',
    name: '喷漆效果',
    render: renderSprayPaint,
    settings: {
      size: { type: 'number', min: 10, max: 300, default: 50, label: '喷射范围' },
      density: { type: 'number', min: 1, max: 100, default: 30, label: '密度' },
      opacity: { type: 'number', min: 0, max: 1, default: 0.3, label: '不透明度' }
    }
  });
}

/**
 * 渲染散点圆点
 * @param {CanvasRenderingContext2D} ctx - 画布上下文
 * @param {StrokePoint} point - 笔触点
 * @param {Object} settings - 笔刷设置
 * @param {Object} color - 当前颜色 {r, g, b, a}
 */
function renderScatterDots(ctx, point, settings, color) {
  const { size, opacity, scatter, count } = settings;
  const scatterRange = (scatter / 100) * size;

  ctx.save();

  for (let i = 0; i < count; i++) {
    // 随机偏移位置
    const offsetX = (Math.random() - 0.5) * scatterRange * 2;
    const offsetY = (Math.random() - 0.5) * scatterRange * 2;

    // 随机大小变化
    const dotSize = size * (0.3 + Math.random() * 0.7) * (point.pressure || 1);

    // 设置颜色和透明度
    ctx.fillStyle = `rgba(${color.r}, ${color.g}, ${color.b}, ${opacity * (0.5 + Math.random() * 0.5)})`;

    // 绘制圆点
    ctx.beginPath();
    ctx.arc(
      point.x + offsetX,
      point.y + offsetY,
      dotSize / 2,
      0,
      Math.PI * 2
    );
    ctx.fill();
  }

  ctx.restore();
}

/**
 * 渲染散点星星
 * @param {CanvasRenderingContext2D} ctx - 画布上下文
 * @param {StrokePoint} point - 笔触点
 * @param {Object} settings - 笔刷设置
 * @param {Object} color - 当前颜色
 */
function renderScatterStars(ctx, point, settings, color) {
  const { size, opacity, scatter, points } = settings;
  const scatterRange = scatter;

  ctx.save();

  // 随机偏移位置
  const offsetX = (Math.random() - 0.5) * scatterRange * 2;
  const offsetY = (Math.random() - 0.5) * scatterRange * 2;

  // 随机大小和旋转
  const starSize = size * (0.5 + Math.random() * 0.5) * (point.pressure || 1);
  const rotation = Math.random() * Math.PI * 2;

  ctx.fillStyle = `rgba(${color.r}, ${color.g}, ${color.b}, ${opacity})`;

  // 绘制星星
  drawStar(
    ctx,
    point.x + offsetX,
    point.y + offsetY,
    points,
    starSize,
    starSize / 2,
    rotation
  );

  ctx.restore();
}

/**
 * 绘制星星形状
 */
function drawStar(ctx, cx, cy, spikes, outerRadius, innerRadius, rotation) {
  ctx.beginPath();
  const step = Math.PI / spikes;

  for (let i = 0; i < spikes * 2; i++) {
    const radius = i % 2 === 0 ? outerRadius : innerRadius;
    const angle = i * step + rotation - Math.PI / 2;
    const x = cx + Math.cos(angle) * radius;
    const y = cy + Math.sin(angle) * radius;

    if (i === 0) {
      ctx.moveTo(x, y);
    } else {
      ctx.lineTo(x, y);
    }
  }

  ctx.closePath();
  ctx.fill();
}

/**
 * 渲染喷漆效果
 * @param {CanvasRenderingContext2D} ctx - 画布上下文
 * @param {StrokePoint} point - 笔触点
 * @param {Object} settings - 笔刷设置
 * @param {Object} color - 当前颜色
 */
function renderSprayPaint(ctx, point, settings, color) {
  const { size, density, opacity } = settings;
  const radius = size / 2;

  ctx.save();

  // 根据密度计算点数
  const pointCount = Math.floor(density * (point.pressure || 1));

  for (let i = 0; i < pointCount; i++) {
    // 使用高斯分布让喷点集中在中心
    const angle = Math.random() * Math.PI * 2;
    const distance = Math.random() * radius * Math.sqrt(Math.random());

    const x = point.x + Math.cos(angle) * distance;
    const y = point.y + Math.sin(angle) * distance;

    // 距离中心越远，透明度越低
    const distanceRatio = distance / radius;
    const pointOpacity = opacity * (1 - distanceRatio * 0.5);

    // 随机点大小
    const pointSize = 1 + Math.random() * 2;

    ctx.fillStyle = `rgba(${color.r}, ${color.g}, ${color.b}, ${pointOpacity})`;
    ctx.fillRect(x, y, pointSize, pointSize);
  }

  ctx.restore();
}

/**
 * 插件清理
 */
function cleanup() {
  console.log('散点笔刷插件已卸载');
  pluginApi = null;
}

// 导出插件接口
module.exports = {
  initialize,
  cleanup
};
