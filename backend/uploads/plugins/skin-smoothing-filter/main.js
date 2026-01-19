/**
 * Skin Smoothing Filter Plugin for DrawConnect
 *
 * This plugin applies professional skin smoothing to portrait photos
 * while preserving natural texture and details.
 */

class SkinSmoothingFilter {
  constructor() {
    this.name = 'Skin Smoothing Filter';
    this.version = '1.0.0';
  }

  /**
   * Initialize the plugin
   * @param {Object} api - DrawConnect Plugin API
   */
  async init(api) {
    this.api = api;

    // Register the filter in the Filters menu
    api.registerFilter({
      id: 'skin-smoothing',
      name: 'Skin Smoothing',
      category: 'Portrait',
      icon: 'icon.svg',
      execute: this.applyFilter.bind(this)
    });

    // Register keyboard shortcut (optional)
    api.registerShortcut({
      key: 'Ctrl+Shift+S',
      action: () => this.showDialog()
    });

    console.log('Skin Smoothing Filter initialized');
  }

  /**
   * Show the filter settings dialog
   */
  async showDialog() {
    const settings = await this.api.showSettingsDialog({
      title: 'Skin Smoothing Settings',
      settings: this.api.getPluginSettings()
    });

    if (settings) {
      await this.applyFilter(settings);
    }
  }

  /**
   * Apply the skin smoothing filter
   * @param {Object} options - Filter options
   */
  async applyFilter(options = {}) {
    const {
      smoothingStrength = 50,
      preserveDetails = true,
      skinToneRange = 'auto'
    } = options;

    // Get the current canvas image data
    const imageData = await this.api.canvas.getImageData();
    const { data, width, height } = imageData;

    // Detect skin tone pixels
    const skinMask = this.detectSkinTones(data, width, height, skinToneRange);

    // Apply bilateral filter for smoothing (preserves edges)
    const smoothedData = this.bilateralFilter(
      data,
      width,
      height,
      smoothingStrength / 10,
      skinMask
    );

    // Blend with original if preserving details
    if (preserveDetails) {
      this.blendWithOriginal(data, smoothedData, skinMask, 0.7);
    }

    // Apply the result
    await this.api.canvas.putImageData(new ImageData(smoothedData, width, height));

    // Add to history for undo support
    await this.api.history.pushState('Skin Smoothing Applied');
  }

  /**
   * Detect skin tone pixels in the image
   */
  detectSkinTones(data, width, height, range) {
    const mask = new Uint8Array(width * height);

    for (let i = 0; i < data.length; i += 4) {
      const r = data[i];
      const g = data[i + 1];
      const b = data[i + 2];

      // Convert to YCbCr color space for better skin detection
      const y = 0.299 * r + 0.587 * g + 0.114 * b;
      const cb = 128 - 0.168736 * r - 0.331264 * g + 0.5 * b;
      const cr = 128 + 0.5 * r - 0.418688 * g - 0.081312 * b;

      // Skin tone detection thresholds
      let isSkin = false;

      if (range === 'auto' || range === 'medium') {
        isSkin = (cr > 133 && cr < 173) && (cb > 77 && cb < 127) && (y > 80);
      } else if (range === 'light') {
        isSkin = (cr > 130 && cr < 170) && (cb > 75 && cb < 125) && (y > 100);
      } else if (range === 'dark') {
        isSkin = (cr > 135 && cr < 180) && (cb > 80 && cb < 130) && (y > 50);
      }

      mask[i / 4] = isSkin ? 255 : 0;
    }

    return mask;
  }

  /**
   * Apply bilateral filter for edge-preserving smoothing
   */
  bilateralFilter(data, width, height, sigma, mask) {
    const result = new Uint8ClampedArray(data.length);
    const radius = Math.ceil(sigma * 2);

    for (let y = 0; y < height; y++) {
      for (let x = 0; x < width; x++) {
        const idx = (y * width + x) * 4;

        // Only apply to skin pixels
        if (mask[y * width + x] === 0) {
          result[idx] = data[idx];
          result[idx + 1] = data[idx + 1];
          result[idx + 2] = data[idx + 2];
          result[idx + 3] = data[idx + 3];
          continue;
        }

        let sumR = 0, sumG = 0, sumB = 0, sumWeight = 0;

        for (let dy = -radius; dy <= radius; dy++) {
          for (let dx = -radius; dx <= radius; dx++) {
            const nx = Math.min(Math.max(x + dx, 0), width - 1);
            const ny = Math.min(Math.max(y + dy, 0), height - 1);
            const nidx = (ny * width + nx) * 4;

            // Spatial weight
            const spatialDist = Math.sqrt(dx * dx + dy * dy);
            const spatialWeight = Math.exp(-spatialDist * spatialDist / (2 * sigma * sigma));

            // Range weight (color similarity)
            const colorDist = Math.sqrt(
              Math.pow(data[idx] - data[nidx], 2) +
              Math.pow(data[idx + 1] - data[nidx + 1], 2) +
              Math.pow(data[idx + 2] - data[nidx + 2], 2)
            );
            const rangeWeight = Math.exp(-colorDist * colorDist / (2 * 30 * 30));

            const weight = spatialWeight * rangeWeight;

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
   * Blend smoothed result with original for detail preservation
   */
  blendWithOriginal(original, smoothed, mask, blendFactor) {
    for (let i = 0; i < original.length; i += 4) {
      const maskValue = mask[i / 4] / 255;
      const blend = blendFactor * maskValue;

      smoothed[i] = original[i] * (1 - blend) + smoothed[i] * blend;
      smoothed[i + 1] = original[i + 1] * (1 - blend) + smoothed[i + 1] * blend;
      smoothed[i + 2] = original[i + 2] * (1 - blend) + smoothed[i + 2] * blend;
    }
  }

  /**
   * Cleanup when plugin is unloaded
   */
  async destroy() {
    console.log('Skin Smoothing Filter unloaded');
  }
}

// Export the plugin
export default SkinSmoothingFilter;
