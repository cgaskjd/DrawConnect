/**
 * DrawConnect 示例滤镜插件
 *
 * 这是插件的主入口文件，展示了如何创建滤镜插件
 */

// 插件初始化
function initialize(api) {
  console.log('示例滤镜插件已加载');

  // 注册滤镜
  api.registerFilter({
    id: 'sample-grayscale',
    name: '灰度化',
    apply: applyGrayscale
  });

  api.registerFilter({
    id: 'sample-sepia',
    name: '复古色调',
    apply: applySepia
  });
}

// 灰度化滤镜
function applyGrayscale(imageData, settings) {
  const data = imageData.data;
  const intensity = (settings.intensity || 100) / 100;

  for (let i = 0; i < data.length; i += 4) {
    const r = data[i];
    const g = data[i + 1];
    const b = data[i + 2];

    // 使用加权平均计算灰度值
    const gray = 0.299 * r + 0.587 * g + 0.114 * b;

    // 根据强度混合原色和灰度
    data[i] = r + (gray - r) * intensity;
    data[i + 1] = g + (gray - g) * intensity;
    data[i + 2] = b + (gray - b) * intensity;
  }

  return imageData;
}

// 复古色调滤镜
function applySepia(imageData, settings) {
  const data = imageData.data;
  const intensity = (settings.intensity || 100) / 100;

  for (let i = 0; i < data.length; i += 4) {
    const r = data[i];
    const g = data[i + 1];
    const b = data[i + 2];

    // 计算复古色调
    const newR = Math.min(255, 0.393 * r + 0.769 * g + 0.189 * b);
    const newG = Math.min(255, 0.349 * r + 0.686 * g + 0.168 * b);
    const newB = Math.min(255, 0.272 * r + 0.534 * g + 0.131 * b);

    // 根据强度混合
    data[i] = r + (newR - r) * intensity;
    data[i + 1] = g + (newG - g) * intensity;
    data[i + 2] = b + (newB - b) * intensity;
  }

  return imageData;
}

// 插件卸载时的清理
function cleanup() {
  console.log('示例滤镜插件已卸载');
}

// 导出插件接口
module.exports = {
  initialize,
  cleanup
};
