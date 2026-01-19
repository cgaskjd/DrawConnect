import { useState, useEffect, useCallback, useRef } from 'react'
import { useAppStore, AdjustmentType } from '../stores/appStore'
import { t } from '../i18n'
import './AdjustmentPanel.css'

export function AdjustmentPanel() {
  const {
    activeAdjustment,
    setActiveAdjustment,
    undo,
    adjustBrightnessContrast,
    adjustHueSaturation,
    adjustLevels,
    adjustPosterize,
    adjustThreshold,
    filterGaussianBlur,
    filterPixelate,
    filterEmboss,
    filterOilPaint,
  } = useAppStore()

  const [hasPreview, setHasPreview] = useState(false)
  const isCancelledRef = useRef(false)

  // Brightness/Contrast state
  const [brightness, setBrightness] = useState(0)
  const [contrast, setContrast] = useState(0)

  // Hue/Saturation state
  const [hue, setHue] = useState(0)
  const [saturation, setSaturation] = useState(0)
  const [lightness, setLightness] = useState(0)

  // Levels state
  const [inputBlack, setInputBlack] = useState(0)
  const [inputWhite, setInputWhite] = useState(255)
  const [gamma, setGamma] = useState(1.0)
  const [outputBlack, setOutputBlack] = useState(0)
  const [outputWhite, setOutputWhite] = useState(255)

  // Posterize state
  const [posterizeLevels, setPosterizeLevels] = useState(4)

  // Threshold state
  const [thresholdLevel, setThresholdLevel] = useState(128)

  // Gaussian blur state
  const [blurRadius, setBlurRadius] = useState(5)

  // Pixelate state
  const [cellSize, setCellSize] = useState(10)

  // Emboss state
  const [embossAngle, setEmbossAngle] = useState(135)
  const [embossHeight, setEmbossHeight] = useState(1)
  const [embossAmount, setEmbossAmount] = useState(100)

  // Oil paint state
  const [oilRadius, setOilRadius] = useState(4)
  const [oilLevels, setOilLevels] = useState(20)

  // Reset all values when adjustment type changes
  useEffect(() => {
    if (activeAdjustment) {
      isCancelledRef.current = false
      setHasPreview(false)
      setBrightness(0)
      setContrast(0)
      setHue(0)
      setSaturation(0)
      setLightness(0)
      setInputBlack(0)
      setInputWhite(255)
      setGamma(1.0)
      setOutputBlack(0)
      setOutputWhite(255)
      setPosterizeLevels(4)
      setThresholdLevel(128)
      setBlurRadius(5)
      setCellSize(10)
      setEmbossAngle(135)
      setEmbossHeight(1)
      setEmbossAmount(100)
      setOilRadius(4)
      setOilLevels(20)
    }
  }, [activeAdjustment])

  const applyPreview = useCallback(async () => {
    // Guard: Don't apply if cancelled or no active adjustment
    if (isCancelledRef.current || !activeAdjustment) return

    if (hasPreview) {
      await undo()
    }

    // Double-check after async operation
    if (isCancelledRef.current) return

    switch (activeAdjustment) {
      case 'brightness_contrast':
        if (brightness !== 0 || contrast !== 0) {
          await adjustBrightnessContrast(brightness, contrast)
          setHasPreview(true)
        } else {
          setHasPreview(false)
        }
        break
      case 'hue_saturation':
        if (hue !== 0 || saturation !== 0 || lightness !== 0) {
          await adjustHueSaturation(hue, saturation, lightness)
          setHasPreview(true)
        } else {
          setHasPreview(false)
        }
        break
      case 'levels':
        if (inputBlack !== 0 || inputWhite !== 255 || gamma !== 1.0 || outputBlack !== 0 || outputWhite !== 255) {
          await adjustLevels(inputBlack, inputWhite, gamma, outputBlack, outputWhite)
          setHasPreview(true)
        } else {
          setHasPreview(false)
        }
        break
      case 'posterize':
        await adjustPosterize(posterizeLevels)
        setHasPreview(true)
        break
      case 'threshold':
        await adjustThreshold(thresholdLevel)
        setHasPreview(true)
        break
      case 'gaussian_blur':
        if (blurRadius > 0) {
          await filterGaussianBlur(blurRadius)
          setHasPreview(true)
        }
        break
      case 'pixelate':
        if (cellSize > 1) {
          await filterPixelate(cellSize)
          setHasPreview(true)
        }
        break
      case 'emboss':
        await filterEmboss(embossAngle, embossHeight, embossAmount)
        setHasPreview(true)
        break
      case 'oil_paint':
        await filterOilPaint(oilRadius, oilLevels)
        setHasPreview(true)
        break
    }
  }, [
    hasPreview, activeAdjustment, undo,
    brightness, contrast, adjustBrightnessContrast,
    hue, saturation, lightness, adjustHueSaturation,
    inputBlack, inputWhite, gamma, outputBlack, outputWhite, adjustLevels,
    posterizeLevels, adjustPosterize,
    thresholdLevel, adjustThreshold,
    blurRadius, filterGaussianBlur,
    cellSize, filterPixelate,
    embossAngle, embossHeight, embossAmount, filterEmboss,
    oilRadius, oilLevels, filterOilPaint,
  ])

  const handleCancel = async () => {
    // Set cancelled flag immediately to prevent any pending previews
    isCancelledRef.current = true
    if (hasPreview) await undo()
    setActiveAdjustment(null)
  }

  const handleOk = () => {
    setActiveAdjustment(null)
  }

  const handleReset = async () => {
    if (hasPreview) await undo()
    setHasPreview(false)
    // Reset to default values based on type
    switch (activeAdjustment) {
      case 'brightness_contrast':
        setBrightness(0)
        setContrast(0)
        break
      case 'hue_saturation':
        setHue(0)
        setSaturation(0)
        setLightness(0)
        break
      case 'levels':
        setInputBlack(0)
        setInputWhite(255)
        setGamma(1.0)
        setOutputBlack(0)
        setOutputWhite(255)
        break
      case 'posterize':
        setPosterizeLevels(4)
        break
      case 'threshold':
        setThresholdLevel(128)
        break
      case 'gaussian_blur':
        setBlurRadius(5)
        break
      case 'pixelate':
        setCellSize(10)
        break
      case 'emboss':
        setEmbossAngle(135)
        setEmbossHeight(1)
        setEmbossAmount(100)
        break
      case 'oil_paint':
        setOilRadius(4)
        setOilLevels(20)
        break
    }
  }

  // Apply preview when values change
  useEffect(() => {
    if (activeAdjustment && !isCancelledRef.current) {
      const timer = setTimeout(() => {
        applyPreview()
      }, 50)
      return () => clearTimeout(timer)
    }
  }, [
    activeAdjustment,
    brightness, contrast, hue, saturation, lightness,
    inputBlack, inputWhite, gamma, outputBlack, outputWhite,
    posterizeLevels, thresholdLevel, blurRadius, cellSize,
    embossAngle, embossHeight, embossAmount, oilRadius, oilLevels,
    applyPreview,
  ])

  if (!activeAdjustment) return null

  const getTitle = () => {
    switch (activeAdjustment) {
      case 'brightness_contrast': return t('menu.brightnessContrast')
      case 'hue_saturation': return t('menu.hueSaturation')
      case 'levels': return t('menu.levels')
      case 'posterize': return t('menu.posterize')
      case 'threshold': return t('menu.threshold')
      case 'gaussian_blur': return t('menu.gaussianBlur')
      case 'pixelate': return t('menu.pixelate')
      case 'emboss': return t('menu.emboss')
      case 'oil_paint': return t('menu.oilPaint')
      default: return ''
    }
  }

  const renderControls = () => {
    switch (activeAdjustment) {
      case 'brightness_contrast':
        return (
          <>
            <div className="adjustment-row">
              <label>{t('adjustment.brightness')}</label>
              <input
                type="range" min="-100" max="100" step="1"
                value={brightness}
                onChange={e => setBrightness(parseInt(e.target.value))}
              />
              <input
                type="number" min="-100" max="100"
                value={brightness}
                onChange={e => setBrightness(parseInt(e.target.value || '0'))}
                className="value-input"
              />
            </div>
            <div className="adjustment-row">
              <label>{t('adjustment.contrast')}</label>
              <input
                type="range" min="-100" max="100" step="1"
                value={contrast}
                onChange={e => setContrast(parseInt(e.target.value))}
              />
              <input
                type="number" min="-100" max="100"
                value={contrast}
                onChange={e => setContrast(parseInt(e.target.value || '0'))}
                className="value-input"
              />
            </div>
          </>
        )

      case 'hue_saturation':
        return (
          <>
            <div className="adjustment-row">
              <label>{t('adjustment.hue')}</label>
              <input
                type="range" min="-180" max="180" step="1"
                value={hue}
                onChange={e => setHue(parseInt(e.target.value))}
              />
              <input
                type="number" min="-180" max="180"
                value={hue}
                onChange={e => setHue(parseInt(e.target.value || '0'))}
                className="value-input"
              />
            </div>
            <div className="adjustment-row">
              <label>{t('adjustment.saturation')}</label>
              <input
                type="range" min="-100" max="100" step="1"
                value={saturation}
                onChange={e => setSaturation(parseInt(e.target.value))}
              />
              <input
                type="number" min="-100" max="100"
                value={saturation}
                onChange={e => setSaturation(parseInt(e.target.value || '0'))}
                className="value-input"
              />
            </div>
            <div className="adjustment-row">
              <label>{t('adjustment.lightness')}</label>
              <input
                type="range" min="-100" max="100" step="1"
                value={lightness}
                onChange={e => setLightness(parseInt(e.target.value))}
              />
              <input
                type="number" min="-100" max="100"
                value={lightness}
                onChange={e => setLightness(parseInt(e.target.value || '0'))}
                className="value-input"
              />
            </div>
          </>
        )

      case 'levels':
        return (
          <>
            <div className="section-label">{t('adjustment.inputLevels')}</div>
            <div className="adjustment-row">
              <label>{t('adjustment.black')}</label>
              <input
                type="range" min="0" max="255" step="1"
                value={inputBlack}
                onChange={e => setInputBlack(parseInt(e.target.value))}
              />
              <input
                type="number" min="0" max="255"
                value={inputBlack}
                onChange={e => setInputBlack(parseInt(e.target.value || '0'))}
                className="value-input"
              />
            </div>
            <div className="adjustment-row">
              <label>{t('adjustment.gamma')}</label>
              <input
                type="range" min="0.1" max="10" step="0.1"
                value={gamma}
                onChange={e => setGamma(parseFloat(e.target.value))}
              />
              <input
                type="number" min="0.1" max="10" step="0.1"
                value={gamma.toFixed(1)}
                onChange={e => setGamma(parseFloat(e.target.value || '1'))}
                className="value-input"
              />
            </div>
            <div className="adjustment-row">
              <label>{t('adjustment.white')}</label>
              <input
                type="range" min="0" max="255" step="1"
                value={inputWhite}
                onChange={e => setInputWhite(parseInt(e.target.value))}
              />
              <input
                type="number" min="0" max="255"
                value={inputWhite}
                onChange={e => setInputWhite(parseInt(e.target.value || '255'))}
                className="value-input"
              />
            </div>
            <div className="section-label">{t('adjustment.outputLevels')}</div>
            <div className="adjustment-row">
              <label>{t('adjustment.black')}</label>
              <input
                type="range" min="0" max="255" step="1"
                value={outputBlack}
                onChange={e => setOutputBlack(parseInt(e.target.value))}
              />
              <input
                type="number" min="0" max="255"
                value={outputBlack}
                onChange={e => setOutputBlack(parseInt(e.target.value || '0'))}
                className="value-input"
              />
            </div>
            <div className="adjustment-row">
              <label>{t('adjustment.white')}</label>
              <input
                type="range" min="0" max="255" step="1"
                value={outputWhite}
                onChange={e => setOutputWhite(parseInt(e.target.value))}
              />
              <input
                type="number" min="0" max="255"
                value={outputWhite}
                onChange={e => setOutputWhite(parseInt(e.target.value || '255'))}
                className="value-input"
              />
            </div>
          </>
        )

      case 'posterize':
        return (
          <div className="adjustment-row">
            <label>{t('adjustment.levels')}</label>
            <input
              type="range" min="2" max="32" step="1"
              value={posterizeLevels}
              onChange={e => setPosterizeLevels(parseInt(e.target.value))}
            />
            <input
              type="number" min="2" max="255"
              value={posterizeLevels}
              onChange={e => setPosterizeLevels(parseInt(e.target.value || '4'))}
              className="value-input"
            />
          </div>
        )

      case 'threshold':
        return (
          <div className="adjustment-row">
            <label>{t('adjustment.threshold')}</label>
            <input
              type="range" min="0" max="255" step="1"
              value={thresholdLevel}
              onChange={e => setThresholdLevel(parseInt(e.target.value))}
            />
            <input
              type="number" min="0" max="255"
              value={thresholdLevel}
              onChange={e => setThresholdLevel(parseInt(e.target.value || '128'))}
              className="value-input"
            />
          </div>
        )

      case 'gaussian_blur':
        return (
          <div className="adjustment-row">
            <label>{t('adjustment.radius')}</label>
            <input
              type="range" min="1" max="100" step="1"
              value={blurRadius}
              onChange={e => setBlurRadius(parseInt(e.target.value))}
            />
            <input
              type="number" min="1" max="200"
              value={blurRadius}
              onChange={e => setBlurRadius(parseInt(e.target.value || '5'))}
              className="value-input"
            />
          </div>
        )

      case 'pixelate':
        return (
          <div className="adjustment-row">
            <label>{t('adjustment.cellSize')}</label>
            <input
              type="range" min="2" max="100" step="1"
              value={cellSize}
              onChange={e => setCellSize(parseInt(e.target.value))}
            />
            <input
              type="number" min="2" max="200"
              value={cellSize}
              onChange={e => setCellSize(parseInt(e.target.value || '10'))}
              className="value-input"
            />
          </div>
        )

      case 'emboss':
        return (
          <>
            <div className="adjustment-row">
              <label>{t('adjustment.angle')}</label>
              <input
                type="range" min="0" max="360" step="1"
                value={embossAngle}
                onChange={e => setEmbossAngle(parseInt(e.target.value))}
              />
              <input
                type="number" min="0" max="360"
                value={embossAngle}
                onChange={e => setEmbossAngle(parseInt(e.target.value || '135'))}
                className="value-input"
              />
            </div>
            <div className="adjustment-row">
              <label>{t('adjustment.height')}</label>
              <input
                type="range" min="1" max="10" step="0.1"
                value={embossHeight}
                onChange={e => setEmbossHeight(parseFloat(e.target.value))}
              />
              <input
                type="number" min="1" max="10" step="0.1"
                value={embossHeight.toFixed(1)}
                onChange={e => setEmbossHeight(parseFloat(e.target.value || '1'))}
                className="value-input"
              />
            </div>
            <div className="adjustment-row">
              <label>{t('adjustment.amount')}</label>
              <input
                type="range" min="0" max="500" step="1"
                value={embossAmount}
                onChange={e => setEmbossAmount(parseInt(e.target.value))}
              />
              <input
                type="number" min="0" max="500"
                value={embossAmount}
                onChange={e => setEmbossAmount(parseInt(e.target.value || '100'))}
                className="value-input"
              />
            </div>
          </>
        )

      case 'oil_paint':
        return (
          <>
            <div className="adjustment-row">
              <label>{t('adjustment.radius')}</label>
              <input
                type="range" min="1" max="10" step="1"
                value={oilRadius}
                onChange={e => setOilRadius(parseInt(e.target.value))}
              />
              <input
                type="number" min="1" max="10"
                value={oilRadius}
                onChange={e => setOilRadius(parseInt(e.target.value || '4'))}
                className="value-input"
              />
            </div>
            <div className="adjustment-row">
              <label>{t('adjustment.levels')}</label>
              <input
                type="range" min="2" max="256" step="1"
                value={oilLevels}
                onChange={e => setOilLevels(parseInt(e.target.value))}
              />
              <input
                type="number" min="2" max="256"
                value={oilLevels}
                onChange={e => setOilLevels(parseInt(e.target.value || '20'))}
                className="value-input"
              />
            </div>
          </>
        )

      default:
        return null
    }
  }

  return (
    <div className="adjustment-panel">
      <div className="adjustment-panel-header">
        <h3>{getTitle()}</h3>
        <button className="close-btn" onClick={handleCancel}>×</button>
      </div>
      <div className="adjustment-panel-content">
        {renderControls()}
      </div>
      <div className="adjustment-panel-footer">
        <button className="btn-secondary" onClick={handleReset}>{t('adjustment.reset')}</button>
        <div className="footer-right">
          <button className="btn-secondary" onClick={handleCancel}>{t('dialog.cancel')}</button>
          <button className="btn-primary" onClick={handleOk}>{t('dialog.ok')}</button>
        </div>
      </div>
    </div>
  )
}
