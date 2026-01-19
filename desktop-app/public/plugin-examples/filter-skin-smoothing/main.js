/**
 * DrawConnect 皮肤磨皮滤镜插件
 *
 * 专业的人像皮肤磨皮滤镜，使用双边滤波算法
 * 自动检测肤色并应用自然平滑效果，同时保留皮肤纹理细节
 */

// 插件状态
let pluginApi = null;

/**
 * 插件初始化
 * @param {PluginAPI} api - DrawConnect API 对象
 */
function initialize(api) {
  pluginApi = api;
  console.log('皮肤磨皮滤镜插件已加载');

  // 注册主磨皮滤镜
  api.registerFilter({
    id: 'skin-smoothing',
    name: '皮肤磨皮',
    category: '人像美化',
    apply: applySkinSmoothing,
    settings: {
      smoothingStrength: { type: 'number', min: 0, max: 100, default: 50, label: '磨皮强度' },
      preserveDetails: { type: 'boolean', default: true, label: '保留纹理' },
      skinToneRange: { type: 'select', options: ['auto', 'light', 'medium', 'dark'], default: 'auto', label: '肤色范围' },
      edgePreservation: { type: 'number', min: 0, max: 100, default: 70, label: '边缘保持' }
    }
  });

  // 注册轻度磨皮预设
  api.registerFilter({
    id: 'skin-smoothing-light',
    name: '轻度磨皮',
    category: '人像美化',
    apply: (ctx, imageData, settings) => applySkinSmoothing(ctx, imageData, {
      ...settings,
      smoothingStrength: 30,
      preserveDetails: true
    }),
    settings: {
      skinToneRange: { type: 'select', options: ['auto', 'light', 'medium', 'dark'], default: 'auto', label: '肤色范围' }
    }
  });

  // 注册强力磨皮预设
  api.registerFilter({
    id: 'skin-smoothing-strong',
    name: '强力磨皮',
    category: '人像美化',
    apply: (ctx, imageData, settings) => applySkinSmoothing(ctx, imageData, {
      ...settings,
      smoothingStrength: 80,
      preserveDetails: false
    }),
    settings: {
      skinToneRange: { type: 'select', options: ['auto', 'light', 'medium', 'dark'], default: 'auto', label: '肤色范围' }
    }
  });
}

/**
 * 应用皮肤磨皮滤镜
 * @param {CanvasRenderingContext2D} ctx - 画布上下文
 * @param {ImageData} imageData - 图像数据
 * @param {Object} settings - 滤镜设置
 * @returns {ImageData} - 处理后的图像数据
 */
function applySkinSmoothing(ctx, imageData, settings) {
  const {
    smoothingStrength = 50,
    preserveDetails = true,
    skinToneRange = 'auto',
    edgePreservation = 70
  } = settings;

  const { data, width, height } = imageData;

  // 检测皮肤区域
  const skinMask = detectSkinTones(data, width, height, skinToneRange);

  // 计算双边滤波参数
  const sigma = smoothingStrength / 10;
  const colorSigma = 30 + (100 - edgePreservation) * 0.5;

  // 应用双边滤波
  const smoothedData = bilateralFilter(data, width, height, sigma, colorSigma, skinMask);

  // 混合原图以保留细节
  if (preserveDetails) {
    blendWithOriginal(data, smoothedData, skinMask, 0.7);
  } else {
    // 直接复制平滑后的数据
    for (let i = 0; i < data.length; i++) {
      data[i] = smoothedData[i];
    }
  }

  return new ImageData(data, width, height);
}

/**
 * 检测皮肤色调像素
 * 使用 YCbCr 色彩空间进行肤色检测
 */
function detectSkinTones(data, width, height, range) {
  const mask = new Uint8Array(width * height);

  for (let i = 0; i < data.length; i += 4) {
    const r = data[i];
    const g = data[i + 1];
    const b = data[i + 2];

    // 转换到 YCbCr 色彩空间
    const y = 0.299 * r + 0.587 * g + 0.114 * b;
    const cb = 128 - 0.168736 * r - 0.331264 * g + 0.5 * b;
    const cr = 128 + 0.5 * r - 0.418688 * g - 0.081312 * b;

    // 肤色检测阈值
    let isSkin = false;

    if (range === 'auto' || range === 'medium') {
      // 通用肤色范围
      isSkin = (cr > 133 && cr < 173) && (cb > 77 && cb < 127) && (y > 80);
    } else if (range === 'light') {
      // 浅色肤色
      isSkin = (cr > 130 && cr < 170) && (cb > 75 && cb < 125) && (y > 100);
    } else if (range === 'dark') {
      // 深色肤色
      isSkin = (cr > 135 && cr < 180) && (cb > 80 && cb < 130) && (y > 50);
    }

    mask[i / 4] = isSkin ? 255 : 0;
  }

  // 应用形态学操作去除噪点
  return morphologicalClean(mask, width, height);
}

/**
 * 形态学清理 - 去除孤立点和填充小孔
 */
function morphologicalClean(mask, width, height) {
  const result = new Uint8Array(mask.length);

  // 简单的 3x3 中值滤波清理
  for (let y = 1; y < height - 1; y++) {
    for (let x = 1; x < width - 1; x++) {
      const idx = y * width + x;

      // 统计 3x3 邻域内的皮肤像素数量
      let count = 0;
      for (let dy = -1; dy <= 1; dy++) {
        for (let dx = -1; dx <= 1; dx++) {
          if (mask[(y + dy) * width + (x + dx)] > 0) {
            count++;
          }
        }
      }

      // 多数投票
      result[idx] = count >= 5 ? 255 : 0;
    }
  }

  return result;
}

/**
 * 双边滤波 - 边缘保持的平滑滤波
 * @param {Uint8ClampedArray} data - 原始图像数据
 * @param {number} width - 图像宽度
 * @param {number} height - 图像高度
 * @param {number} spatialSigma - 空间高斯参数
 * @param {number} colorSigma - 颜色高斯参数
 * @param {Uint8Array} mask - 皮肤掩码
 * @returns {Uint8ClampedArray} - 平滑后的数据
 */
function bilateralFilter(data, width, height, spatialSigma, colorSigma, mask) {
  const result = new Uint8ClampedArray(data.length);
  const radius = Math.ceil(spatialSigma * 2);

  // 预计算空间权重
  const spatialWeights = [];
  for (let dy = -radius; dy <= radius; dy++) {
    for (let dx = -radius; dx <= radius; dx++) {
      const dist = Math.sqrt(dx * dx + dy * dy);
      spatialWeights.push(Math.exp(-dist * dist / (2 * spatialSigma * spatialSigma)));
    }
  }

  const colorSigma2 = 2 * colorSigma * colorSigma;

  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      const idx = (y * width + x) * 4;
      const maskIdx = y * width + x;

      // 非皮肤区域直接复制
      if (mask[maskIdx] === 0) {
        result[idx] = data[idx];
        result[idx + 1] = data[idx + 1];
        result[idx + 2] = data[idx + 2];
        result[idx + 3] = data[idx + 3];
        continue;
      }

      let sumR = 0, sumG = 0, sumB = 0, sumWeight = 0;
      let weightIdx = 0;

      for (let dy = -radius; dy <= radius; dy++) {
        for (let dx = -radius; dx <= radius; dx++) {
          const nx = Math.min(Math.max(x + dx, 0), width - 1);
          const ny = Math.min(Math.max(y + dy, 0), height - 1);
          const nidx = (ny * width + nx) * 4;

          // 空间权重
          const spatialWeight = spatialWeights[weightIdx++];

          // 颜色权重
          const dr = data[idx] - data[nidx];
          const dg = data[idx + 1] - data[nidx + 1];
          const db = data[idx + 2] - data[nidx + 2];
          const colorDist = dr * dr + dg * dg + db * db;
          const colorWeight = Math.exp(-colorDist / colorSigma2);

          const weight = spatialWeight * colorWeight;

          sumR += data[nidx] * weight;
          sumG += data[nidx + 1] * weight;
          sumB += data[nidx + 2] * weight;
          sumWeight += weight;
        }
      }

      result[idx] = sumR / sumWeight;
      result[idx + 1] = sumG / sumWeight;
      result[idx + 2] = sumB / sumWeight;
      result[idx + 3] = data[idx + 3];
    }
  }

  return result;
}

/**
 * 与原图混合 - 保留皮肤纹理细节
 */
function blendWithOriginal(original, smoothed, mask, blendFactor) {
  for (let i = 0; i < original.length; i += 4) {
    const maskIdx = i / 4;
    const maskValue = mask[maskIdx] / 255;
    const blend = blendFactor * maskValue;

    original[i] = Math.round(original[i] * (1 - blend) + smoothed[i] * blend);
    original[i + 1] = Math.round(original[i + 1] * (1 - blend) + smoothed[i + 1] * blend);
    original[i + 2] = Math.round(original[i + 2] * (1 - blend) + smoothed[i + 2] * blend);
  }
}

/**
 * 插件清理
 */
function cleanup() {
  console.log('皮肤磨皮滤镜插件已卸载');
  pluginApi = null;
}

// 导出插件接口
module.exports = {
  initialize,
  cleanup
};
