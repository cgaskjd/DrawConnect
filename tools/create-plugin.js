#!/usr/bin/env node
/**
 * DrawConnect 插件脚手架工具
 *
 * 使用方法:
 *   node create-plugin.js <plugin-name> [--type=filter|brush|tool|mixed]
 *
 * 示例:
 *   node create-plugin.js my-awesome-filter --type=filter
 *   node create-plugin.js scatter-brush --type=brush
 *   node create-plugin.js ruler-tool --type=tool
 */

const fs = require('fs');
const path = require('path');
const readline = require('readline');

// 颜色输出
const colors = {
  reset: '\x1b[0m',
  bright: '\x1b[1m',
  green: '\x1b[32m',
  yellow: '\x1b[33m',
  blue: '\x1b[34m',
  red: '\x1b[31m',
  cyan: '\x1b[36m',
};

function log(message, color = 'reset') {
  console.log(`${colors[color]}${message}${colors.reset}`);
}

function logSuccess(message) {
  log(`✓ ${message}`, 'green');
}

function logInfo(message) {
  log(`ℹ ${message}`, 'blue');
}

function logWarn(message) {
  log(`⚠ ${message}`, 'yellow');
}

function logError(message) {
  log(`✗ ${message}`, 'red');
}

// 解析命令行参数
function parseArgs() {
  const args = process.argv.slice(2);
  const result = {
    name: null,
    type: 'filter',
  };

  for (const arg of args) {
    if (arg.startsWith('--type=')) {
      result.type = arg.split('=')[1];
    } else if (!arg.startsWith('--')) {
      result.name = arg;
    }
  }

  return result;
}

// 交互式输入
async function prompt(question, defaultValue = '') {
  const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
  });

  return new Promise((resolve) => {
    const defaultHint = defaultValue ? ` (${defaultValue})` : '';
    rl.question(`${question}${defaultHint}: `, (answer) => {
      rl.close();
      resolve(answer || defaultValue);
    });
  });
}

// 生成 manifest.json
function generateManifest(config) {
  const capabilities = {
    brushes: [],
    filters: [],
    tools: [],
    panels: [],
  };

  // 根据类型添加默认能力
  switch (config.type) {
    case 'filter':
      capabilities.filters.push({
        id: `${config.id}-default`,
        name: config.name,
        category: '自定义',
      });
      break;
    case 'brush':
      capabilities.brushes.push({
        id: `${config.id}-default`,
        name: config.name,
        category: '自定义',
      });
      break;
    case 'tool':
      capabilities.tools.push({
        id: `${config.id}-default`,
        name: config.name,
      });
      break;
    case 'mixed':
      capabilities.filters.push({
        id: `${config.id}-filter`,
        name: `${config.name} 滤镜`,
        category: '自定义',
      });
      capabilities.brushes.push({
        id: `${config.id}-brush`,
        name: `${config.name} 笔刷`,
        category: '自定义',
      });
      break;
  }

  const permissions = ['canvas:read', 'canvas:write'];
  if (config.type === 'filter' || config.type === 'mixed') {
    permissions.push('filter:register');
  }
  if (config.type === 'brush' || config.type === 'mixed') {
    permissions.push('brush:register', 'brush:render');
  }
  if (config.type === 'tool' || config.type === 'mixed') {
    permissions.push('tool:register');
  }

  return {
    id: config.id,
    name: config.name,
    version: '1.0.0',
    apiVersion: '1.0',
    description: config.description,
    author: {
      name: config.author,
      email: config.email,
    },
    license: config.license,
    type: config.type,
    runtime: 'javascript',
    main: 'main.js',
    permissions,
    capabilities,
    settings: {
      schema: {
        type: 'object',
        properties: {
          intensity: {
            type: 'number',
            minimum: 0,
            maximum: 100,
            default: 50,
          },
        },
      },
    },
    locales: ['zh', 'en'],
    keywords: [config.type, config.name.toLowerCase()],
    category: config.type.charAt(0).toUpperCase() + config.type.slice(1) + 's',
  };
}

// 生成 main.js
function generateMainJs(config) {
  const templates = {
    filter: generateFilterTemplate(config),
    brush: generateBrushTemplate(config),
    tool: generateToolTemplate(config),
    mixed: generateMixedTemplate(config),
  };

  return templates[config.type] || templates.filter;
}

function generateFilterTemplate(config) {
  return `/**
 * ${config.name} - DrawConnect 滤镜插件
 *
 * ${config.description}
 */

let pluginApi = null;

/**
 * 插件初始化
 */
function initialize(api) {
  pluginApi = api;
  console.log('${config.name} 插件已加载');

  // 注册滤镜
  api.registerFilter({
    id: '${config.id}-default',
    name: '${config.name}',
    apply: applyFilter,
    settings: {
      intensity: { type: 'number', min: 0, max: 100, default: 50, label: '强度' }
    }
  });
}

/**
 * 滤镜处理函数
 * @param {ImageData} imageData - 图像数据
 * @param {Object} settings - 用户设置
 * @returns {ImageData} 处理后的图像数据
 */
function applyFilter(imageData, settings) {
  const data = imageData.data;
  const intensity = (settings.intensity || 50) / 100;

  // 遍历每个像素 (RGBA)
  for (let i = 0; i < data.length; i += 4) {
    const r = data[i];
    const g = data[i + 1];
    const b = data[i + 2];
    // const a = data[i + 3];  // Alpha 通道

    // TODO: 在这里实现您的滤镜效果
    // 示例：灰度化
    const gray = 0.299 * r + 0.587 * g + 0.114 * b;
    data[i] = r + (gray - r) * intensity;
    data[i + 1] = g + (gray - g) * intensity;
    data[i + 2] = b + (gray - b) * intensity;
  }

  return imageData;
}

/**
 * 插件清理
 */
function cleanup() {
  console.log('${config.name} 插件已卸载');
  pluginApi = null;
}

module.exports = { initialize, cleanup };
`;
}

function generateBrushTemplate(config) {
  return `/**
 * ${config.name} - DrawConnect 笔刷插件
 *
 * ${config.description}
 */

let pluginApi = null;

/**
 * 插件初始化
 */
function initialize(api) {
  pluginApi = api;
  console.log('${config.name} 插件已加载');

  // 注册笔刷
  api.registerBrush({
    id: '${config.id}-default',
    name: '${config.name}',
    render: renderBrush,
    settings: {
      size: { type: 'number', min: 1, max: 500, default: 20, label: '大小' },
      opacity: { type: 'number', min: 0, max: 1, default: 1, label: '不透明度' },
      hardness: { type: 'number', min: 0, max: 1, default: 0.5, label: '硬度' }
    }
  });
}

/**
 * 笔刷渲染函数
 * @param {CanvasRenderingContext2D} ctx - 画布上下文
 * @param {StrokePoint} point - 笔触点
 * @param {Object} settings - 笔刷设置
 * @param {Object} color - 当前颜色 {r, g, b, a}
 */
function renderBrush(ctx, point, settings, color) {
  const { size, opacity, hardness } = settings;
  const radius = (size / 2) * (point.pressure || 1);

  ctx.save();

  // TODO: 在这里实现您的笔刷渲染逻辑
  // 示例：简单圆形笔刷
  ctx.fillStyle = \`rgba(\${color.r}, \${color.g}, \${color.b}, \${opacity})\`;

  // 使用径向渐变实现硬度效果
  const gradient = ctx.createRadialGradient(
    point.x, point.y, radius * hardness,
    point.x, point.y, radius
  );
  gradient.addColorStop(0, \`rgba(\${color.r}, \${color.g}, \${color.b}, \${opacity})\`);
  gradient.addColorStop(1, \`rgba(\${color.r}, \${color.g}, \${color.b}, 0)\`);

  ctx.fillStyle = gradient;
  ctx.beginPath();
  ctx.arc(point.x, point.y, radius, 0, Math.PI * 2);
  ctx.fill();

  ctx.restore();
}

/**
 * 插件清理
 */
function cleanup() {
  console.log('${config.name} 插件已卸载');
  pluginApi = null;
}

module.exports = { initialize, cleanup };
`;
}

function generateToolTemplate(config) {
  return `/**
 * ${config.name} - DrawConnect 工具插件
 *
 * ${config.description}
 */

let pluginApi = null;
let toolState = {
  isActive: false,
  startPoint: null,
  currentPoint: null
};

/**
 * 插件初始化
 */
function initialize(api) {
  pluginApi = api;
  console.log('${config.name} 插件已加载');

  // 注册工具
  api.registerTool({
    id: '${config.id}-default',
    name: '${config.name}',
    cursor: 'crosshair',
    onActivate: onToolActivate,
    onDeactivate: onToolDeactivate,
    onMouseDown: onMouseDown,
    onMouseMove: onMouseMove,
    onMouseUp: onMouseUp,
    onRender: renderTool,
    settings: {
      showGuides: { type: 'boolean', default: true, label: '显示辅助线' }
    }
  });
}

function onToolActivate() {
  toolState.isActive = true;
  console.log('${config.name} 工具已激活');
}

function onToolDeactivate() {
  toolState.isActive = false;
  toolState.startPoint = null;
  toolState.currentPoint = null;
  console.log('${config.name} 工具已停用');
}

function onMouseDown(event) {
  toolState.startPoint = { x: event.x, y: event.y };
  toolState.currentPoint = { x: event.x, y: event.y };
}

function onMouseMove(event) {
  if (toolState.startPoint) {
    toolState.currentPoint = { x: event.x, y: event.y };
    pluginApi.canvas.refresh();
  }
}

function onMouseUp(event) {
  // TODO: 在这里处理工具操作完成的逻辑
  const start = toolState.startPoint;
  const end = { x: event.x, y: event.y };

  if (start) {
    console.log(\`操作完成: (\${start.x}, \${start.y}) -> (\${end.x}, \${end.y})\`);
  }

  toolState.startPoint = null;
  toolState.currentPoint = null;
  pluginApi.canvas.refresh();
}

/**
 * 工具渲染（绘制覆盖层）
 */
function renderTool(ctx, settings) {
  if (!toolState.startPoint || !toolState.currentPoint) return;

  ctx.save();

  // TODO: 在这里实现您的工具覆盖层渲染
  // 示例：绘制选择框
  const start = toolState.startPoint;
  const current = toolState.currentPoint;

  ctx.strokeStyle = '#2196F3';
  ctx.lineWidth = 2;
  ctx.setLineDash([5, 5]);

  ctx.strokeRect(
    Math.min(start.x, current.x),
    Math.min(start.y, current.y),
    Math.abs(current.x - start.x),
    Math.abs(current.y - start.y)
  );

  ctx.restore();
}

/**
 * 插件清理
 */
function cleanup() {
  console.log('${config.name} 插件已卸载');
  pluginApi = null;
  toolState = { isActive: false, startPoint: null, currentPoint: null };
}

module.exports = { initialize, cleanup };
`;
}

function generateMixedTemplate(config) {
  return `/**
 * ${config.name} - DrawConnect 混合插件
 *
 * ${config.description}
 */

let pluginApi = null;

/**
 * 插件初始化
 */
function initialize(api) {
  pluginApi = api;
  console.log('${config.name} 插件已加载');

  // 注册滤镜
  api.registerFilter({
    id: '${config.id}-filter',
    name: '${config.name} 滤镜',
    apply: applyFilter,
    settings: {
      intensity: { type: 'number', min: 0, max: 100, default: 50, label: '强度' }
    }
  });

  // 注册笔刷
  api.registerBrush({
    id: '${config.id}-brush',
    name: '${config.name} 笔刷',
    render: renderBrush,
    settings: {
      size: { type: 'number', min: 1, max: 500, default: 20, label: '大小' },
      opacity: { type: 'number', min: 0, max: 1, default: 1, label: '不透明度' }
    }
  });
}

// 滤镜实现
function applyFilter(imageData, settings) {
  const data = imageData.data;
  const intensity = (settings.intensity || 50) / 100;

  for (let i = 0; i < data.length; i += 4) {
    // TODO: 实现滤镜效果
  }

  return imageData;
}

// 笔刷实现
function renderBrush(ctx, point, settings, color) {
  // TODO: 实现笔刷渲染
  ctx.fillStyle = \`rgba(\${color.r}, \${color.g}, \${color.b}, \${settings.opacity})\`;
  ctx.beginPath();
  ctx.arc(point.x, point.y, settings.size / 2, 0, Math.PI * 2);
  ctx.fill();
}

/**
 * 插件清理
 */
function cleanup() {
  console.log('${config.name} 插件已卸载');
  pluginApi = null;
}

module.exports = { initialize, cleanup };
`;
}

// 生成 README.md
function generateReadme(config) {
  return `# ${config.name}

${config.description}

## 安装

1. 将此文件夹复制到 DrawConnect 插件目录
2. 或通过插件管理器安装
3. 或打包为 \`.dcplugin\` 文件后安装

## 功能

- [ ] 功能 1
- [ ] 功能 2

## 使用方法

1. 打开 DrawConnect
2. 选择此插件提供的功能

## 设置选项

| 设置 | 类型 | 范围 | 说明 |
|------|------|------|------|
| intensity | 数值 | 0-100 | 效果强度 |

## 开发

\`\`\`bash
# 修改代码后重新加载插件
# 在 DrawConnect 中：工具 → 插件管理 → 刷新
\`\`\`

## 许可证

${config.license}

## 作者

${config.author} <${config.email}>
`;
}

// 创建插件
async function createPlugin() {
  log('\n🎨 DrawConnect 插件脚手架\n', 'cyan');

  const args = parseArgs();

  // 交互式获取配置
  const name = args.name || await prompt('插件名称', 'my-plugin');
  const displayName = await prompt('显示名称', name);
  const type = args.type || await prompt('插件类型 (filter/brush/tool/mixed)', 'filter');
  const description = await prompt('插件描述', `一个 DrawConnect ${type} 插件`);
  const author = await prompt('作者名称', 'Developer');
  const email = await prompt('作者邮箱', 'dev@example.com');
  const license = await prompt('许可证', 'MIT');

  // 生成 ID
  const id = `com.${author.toLowerCase().replace(/\s+/g, '')}.${name.toLowerCase().replace(/\s+/g, '-')}`;

  const config = {
    name: displayName,
    id,
    type,
    description,
    author,
    email,
    license,
  };

  // 创建目录
  const pluginDir = path.join(process.cwd(), name);

  if (fs.existsSync(pluginDir)) {
    logError(`目录 "${name}" 已存在`);
    process.exit(1);
  }

  fs.mkdirSync(pluginDir, { recursive: true });
  logSuccess(`创建目录: ${name}/`);

  // 创建文件
  const manifest = generateManifest(config);
  fs.writeFileSync(
    path.join(pluginDir, 'manifest.json'),
    JSON.stringify(manifest, null, 2)
  );
  logSuccess('创建文件: manifest.json');

  const mainJs = generateMainJs(config);
  fs.writeFileSync(path.join(pluginDir, 'main.js'), mainJs);
  logSuccess('创建文件: main.js');

  const readme = generateReadme(config);
  fs.writeFileSync(path.join(pluginDir, 'README.md'), readme);
  logSuccess('创建文件: README.md');

  // 完成
  log('\n✨ 插件创建成功！\n', 'green');
  logInfo(`插件目录: ${pluginDir}`);
  logInfo(`插件 ID: ${id}`);

  log('\n下一步:', 'yellow');
  log(`  1. cd ${name}`);
  log(`  2. 编辑 main.js 实现您的功能`);
  log(`  3. 在 DrawConnect 中安装测试`);

  log('\n安装方法:', 'yellow');
  log(`  - 打开 DrawConnect → 工具 → 插件管理 → 从文件夹安装`);
  log(`  - 选择 "${pluginDir}" 目录\n`);
}

// 显示帮助
function showHelp() {
  log('\nDrawConnect 插件脚手架工具\n', 'cyan');
  log('使用方法:');
  log('  node create-plugin.js <plugin-name> [options]\n');
  log('选项:');
  log('  --type=<type>    插件类型 (filter|brush|tool|mixed)');
  log('  --help           显示帮助信息\n');
  log('示例:');
  log('  node create-plugin.js my-filter --type=filter');
  log('  node create-plugin.js my-brush --type=brush');
  log('  node create-plugin.js my-tool --type=tool\n');
}

// 入口
if (process.argv.includes('--help') || process.argv.includes('-h')) {
  showHelp();
} else {
  createPlugin().catch(console.error);
}
