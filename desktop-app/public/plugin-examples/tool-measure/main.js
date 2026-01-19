/**
 * DrawConnect 测量工具插件示例
 *
 * 这个插件展示了如何创建自定义工具，包括：
 * - 标尺工具（测量距离）
 * - 量角器工具（测量角度）
 * - 网格叠加工具
 */

// 插件状态
let pluginApi = null;
let currentTool = null;
let measureState = {
  startPoint: null,
  endPoint: null,
  anglePoints: [],  // 用于量角器的三个点
  isDrawing: false
};

// 设置
let settings = {
  unit: 'px',
  gridSize: 50,
  showLabel: true,
  dpi: 96  // 默认 DPI
};

/**
 * 插件初始化
 */
function initialize(api) {
  pluginApi = api;
  console.log('测量工具插件已加载');

  // 获取画布 DPI
  const canvasInfo = api.canvas.getInfo();
  if (canvasInfo && canvasInfo.dpi) {
    settings.dpi = canvasInfo.dpi;
  }

  // 注册标尺工具
  api.registerTool({
    id: 'ruler',
    name: '标尺工具',
    cursor: 'crosshair',
    onActivate: () => onToolActivate('ruler'),
    onDeactivate: onToolDeactivate,
    onMouseDown: onRulerMouseDown,
    onMouseMove: onRulerMouseMove,
    onMouseUp: onRulerMouseUp,
    onRender: renderRuler
  });

  // 注册量角器工具
  api.registerTool({
    id: 'protractor',
    name: '量角器',
    cursor: 'crosshair',
    onActivate: () => onToolActivate('protractor'),
    onDeactivate: onToolDeactivate,
    onMouseDown: onProtractorMouseDown,
    onMouseMove: onProtractorMouseMove,
    onMouseUp: onProtractorMouseUp,
    onRender: renderProtractor
  });

  // 注册网格叠加工具
  api.registerTool({
    id: 'grid-overlay',
    name: '网格叠加',
    cursor: 'default',
    onActivate: () => onToolActivate('grid-overlay'),
    onDeactivate: onToolDeactivate,
    onRender: renderGrid,
    settings: {
      gridSize: { type: 'number', min: 10, max: 200, default: 50, label: '网格大小' },
      gridColor: { type: 'color', default: '#00000033', label: '网格颜色' }
    }
  });

  // 注册测量信息面板
  api.registerPanel({
    id: 'measure-info',
    name: '测量信息',
    render: renderMeasurePanel
  });
}

/**
 * 工具激活
 */
function onToolActivate(toolId) {
  currentTool = toolId;
  resetMeasureState();
  console.log(`工具已激活: ${toolId}`);
}

/**
 * 工具停用
 */
function onToolDeactivate() {
  currentTool = null;
  resetMeasureState();
}

/**
 * 重置测量状态
 */
function resetMeasureState() {
  measureState = {
    startPoint: null,
    endPoint: null,
    anglePoints: [],
    isDrawing: false
  };
}

// ============================================================================
// 标尺工具
// ============================================================================

function onRulerMouseDown(event) {
  measureState.startPoint = { x: event.x, y: event.y };
  measureState.endPoint = null;
  measureState.isDrawing = true;
}

function onRulerMouseMove(event) {
  if (measureState.isDrawing && measureState.startPoint) {
    measureState.endPoint = { x: event.x, y: event.y };
    pluginApi.canvas.refresh();
  }
}

function onRulerMouseUp(event) {
  measureState.endPoint = { x: event.x, y: event.y };
  measureState.isDrawing = false;

  // 计算并显示距离
  const distance = calculateDistance(measureState.startPoint, measureState.endPoint);
  const formattedDistance = formatDistance(distance);

  pluginApi.ui.notify(`距离: ${formattedDistance}`, 'info');
  pluginApi.canvas.refresh();
}

/**
 * 渲染标尺
 */
function renderRuler(ctx) {
  if (!measureState.startPoint || !measureState.endPoint) return;

  const start = measureState.startPoint;
  const end = measureState.endPoint;

  ctx.save();

  // 绘制测量线
  ctx.strokeStyle = '#FF5722';
  ctx.lineWidth = 2;
  ctx.setLineDash([5, 5]);

  ctx.beginPath();
  ctx.moveTo(start.x, start.y);
  ctx.lineTo(end.x, end.y);
  ctx.stroke();

  // 绘制端点
  ctx.fillStyle = '#FF5722';
  ctx.setLineDash([]);

  ctx.beginPath();
  ctx.arc(start.x, start.y, 5, 0, Math.PI * 2);
  ctx.fill();

  ctx.beginPath();
  ctx.arc(end.x, end.y, 5, 0, Math.PI * 2);
  ctx.fill();

  // 显示距离标签
  if (settings.showLabel) {
    const distance = calculateDistance(start, end);
    const formattedDistance = formatDistance(distance);
    const midX = (start.x + end.x) / 2;
    const midY = (start.y + end.y) / 2;

    ctx.font = '14px Arial';
    ctx.fillStyle = '#FF5722';
    ctx.textAlign = 'center';
    ctx.textBaseline = 'bottom';

    // 绘制背景
    const textWidth = ctx.measureText(formattedDistance).width;
    ctx.fillStyle = 'rgba(255, 255, 255, 0.8)';
    ctx.fillRect(midX - textWidth / 2 - 5, midY - 20, textWidth + 10, 20);

    // 绘制文字
    ctx.fillStyle = '#FF5722';
    ctx.fillText(formattedDistance, midX, midY - 5);
  }

  ctx.restore();
}

// ============================================================================
// 量角器工具
// ============================================================================

function onProtractorMouseDown(event) {
  if (measureState.anglePoints.length >= 3) {
    measureState.anglePoints = [];
  }
  measureState.anglePoints.push({ x: event.x, y: event.y });
  measureState.isDrawing = measureState.anglePoints.length < 3;
  pluginApi.canvas.refresh();
}

function onProtractorMouseMove(event) {
  // 量角器不需要 mousemove 处理
}

function onProtractorMouseUp(event) {
  if (measureState.anglePoints.length === 3) {
    const angle = calculateAngle(
      measureState.anglePoints[0],
      measureState.anglePoints[1],
      measureState.anglePoints[2]
    );
    pluginApi.ui.notify(`角度: ${angle.toFixed(1)}°`, 'info');
  }
}

/**
 * 渲染量角器
 */
function renderProtractor(ctx) {
  const points = measureState.anglePoints;
  if (points.length === 0) return;

  ctx.save();

  // 绘制点
  ctx.fillStyle = '#2196F3';
  points.forEach((point, index) => {
    ctx.beginPath();
    ctx.arc(point.x, point.y, 6, 0, Math.PI * 2);
    ctx.fill();

    // 标记点序号
    ctx.font = '12px Arial';
    ctx.fillStyle = '#FFFFFF';
    ctx.textAlign = 'center';
    ctx.textBaseline = 'middle';
    ctx.fillText((index + 1).toString(), point.x, point.y);
  });

  // 绘制线段
  if (points.length >= 2) {
    ctx.strokeStyle = '#2196F3';
    ctx.lineWidth = 2;

    ctx.beginPath();
    ctx.moveTo(points[0].x, points[0].y);
    ctx.lineTo(points[1].x, points[1].y);
    ctx.stroke();
  }

  if (points.length === 3) {
    ctx.beginPath();
    ctx.moveTo(points[1].x, points[1].y);
    ctx.lineTo(points[2].x, points[2].y);
    ctx.stroke();

    // 绘制角度弧
    const angle = calculateAngle(points[0], points[1], points[2]);
    const startAngle = Math.atan2(points[0].y - points[1].y, points[0].x - points[1].x);
    const endAngle = Math.atan2(points[2].y - points[1].y, points[2].x - points[1].x);

    ctx.strokeStyle = 'rgba(33, 150, 243, 0.5)';
    ctx.lineWidth = 2;
    ctx.beginPath();
    ctx.arc(points[1].x, points[1].y, 30, startAngle, endAngle);
    ctx.stroke();

    // 显示角度
    if (settings.showLabel) {
      ctx.font = '14px Arial';
      ctx.fillStyle = '#2196F3';
      ctx.textAlign = 'center';
      ctx.fillText(`${angle.toFixed(1)}°`, points[1].x, points[1].y - 40);
    }
  }

  ctx.restore();
}

// ============================================================================
// 网格叠加工具
// ============================================================================

/**
 * 渲染网格
 */
function renderGrid(ctx, toolSettings) {
  const canvasInfo = pluginApi.canvas.getInfo();
  const gridSize = toolSettings?.gridSize || settings.gridSize;
  const gridColor = toolSettings?.gridColor || '#00000033';

  ctx.save();

  ctx.strokeStyle = gridColor;
  ctx.lineWidth = 1;

  // 绘制垂直线
  for (let x = gridSize; x < canvasInfo.width; x += gridSize) {
    ctx.beginPath();
    ctx.moveTo(x, 0);
    ctx.lineTo(x, canvasInfo.height);
    ctx.stroke();
  }

  // 绘制水平线
  for (let y = gridSize; y < canvasInfo.height; y += gridSize) {
    ctx.beginPath();
    ctx.moveTo(0, y);
    ctx.lineTo(canvasInfo.width, y);
    ctx.stroke();
  }

  // 绘制网格尺寸标签
  ctx.font = '10px Arial';
  ctx.fillStyle = '#666666';
  ctx.textAlign = 'left';
  ctx.fillText(`${gridSize}px`, 5, gridSize - 5);

  ctx.restore();
}

// ============================================================================
// 测量信息面板
// ============================================================================

/**
 * 渲染测量信息面板
 */
function renderMeasurePanel() {
  let html = '<div class="measure-panel">';

  if (measureState.startPoint && measureState.endPoint) {
    const distance = calculateDistance(measureState.startPoint, measureState.endPoint);
    html += `
      <div class="measure-item">
        <label>距离:</label>
        <span>${formatDistance(distance)}</span>
      </div>
      <div class="measure-item">
        <label>起点:</label>
        <span>(${measureState.startPoint.x.toFixed(0)}, ${measureState.startPoint.y.toFixed(0)})</span>
      </div>
      <div class="measure-item">
        <label>终点:</label>
        <span>(${measureState.endPoint.x.toFixed(0)}, ${measureState.endPoint.y.toFixed(0)})</span>
      </div>
    `;

    // 计算水平/垂直距离
    const dx = Math.abs(measureState.endPoint.x - measureState.startPoint.x);
    const dy = Math.abs(measureState.endPoint.y - measureState.startPoint.y);
    html += `
      <div class="measure-item">
        <label>水平:</label>
        <span>${formatDistance(dx)}</span>
      </div>
      <div class="measure-item">
        <label>垂直:</label>
        <span>${formatDistance(dy)}</span>
      </div>
    `;
  }

  if (measureState.anglePoints.length === 3) {
    const angle = calculateAngle(
      measureState.anglePoints[0],
      measureState.anglePoints[1],
      measureState.anglePoints[2]
    );
    html += `
      <div class="measure-item">
        <label>角度:</label>
        <span>${angle.toFixed(2)}°</span>
      </div>
    `;
  }

  html += '</div>';
  return html;
}

// ============================================================================
// 工具函数
// ============================================================================

/**
 * 计算两点之间的距离
 */
function calculateDistance(p1, p2) {
  const dx = p2.x - p1.x;
  const dy = p2.y - p1.y;
  return Math.sqrt(dx * dx + dy * dy);
}

/**
 * 计算三点形成的角度
 */
function calculateAngle(p1, vertex, p2) {
  const v1 = { x: p1.x - vertex.x, y: p1.y - vertex.y };
  const v2 = { x: p2.x - vertex.x, y: p2.y - vertex.y };

  const dot = v1.x * v2.x + v1.y * v2.y;
  const mag1 = Math.sqrt(v1.x * v1.x + v1.y * v1.y);
  const mag2 = Math.sqrt(v2.x * v2.x + v2.y * v2.y);

  const cosAngle = dot / (mag1 * mag2);
  const angle = Math.acos(Math.max(-1, Math.min(1, cosAngle)));

  return angle * (180 / Math.PI);
}

/**
 * 格式化距离显示
 */
function formatDistance(pixelDistance) {
  switch (settings.unit) {
    case 'cm':
      const cm = pixelDistance / settings.dpi * 2.54;
      return `${cm.toFixed(2)} cm`;
    case 'in':
      const inches = pixelDistance / settings.dpi;
      return `${inches.toFixed(2)} in`;
    default:
      return `${pixelDistance.toFixed(1)} px`;
  }
}

/**
 * 插件清理
 */
function cleanup() {
  console.log('测量工具插件已卸载');
  pluginApi = null;
  currentTool = null;
  resetMeasureState();
}

// 导出插件接口
module.exports = {
  initialize,
  cleanup
};
