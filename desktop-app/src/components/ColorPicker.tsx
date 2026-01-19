import { useState, useRef, useCallback, useEffect } from 'react'
import { ChevronDown, ChevronUp, Download, Upload, Plus, Trash2 } from 'lucide-react'
import { open, save, message } from '@tauri-apps/api/dialog'
import { readTextFile, writeTextFile } from '@tauri-apps/api/fs'
import { useAppStore } from '../stores/appStore'
import { t } from '../i18n'
import './ColorPicker.css'

const defaultPresetColors = [
  '#000000', '#FFFFFF', '#FF0000', '#00FF00', '#0000FF',
  '#FFFF00', '#FF00FF', '#00FFFF', '#FF8000', '#8000FF',
  '#FF0080', '#0080FF', '#80FF00', '#00FF80', '#800000',
  '#008000', '#000080', '#808000', '#800080', '#008080',
  '#C0C0C0', '#808080', '#404040', '#FFC0CB', '#FFD700',
  '#FF6347', '#4169E1', '#32CD32', '#9370DB', '#20B2AA',
]

// Color conversion utilities
function hexToRgb(hex: string): { r: number; g: number; b: number } {
  const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex)
  return result
    ? {
        r: parseInt(result[1], 16),
        g: parseInt(result[2], 16),
        b: parseInt(result[3], 16),
      }
    : { r: 0, g: 0, b: 0 }
}

function rgbToHex(r: number, g: number, b: number): string {
  return '#' + [r, g, b].map(x => x.toString(16).padStart(2, '0')).join('').toUpperCase()
}

function rgbToHsv(r: number, g: number, b: number): { h: number; s: number; v: number } {
  r /= 255
  g /= 255
  b /= 255

  const max = Math.max(r, g, b)
  const min = Math.min(r, g, b)
  const d = max - min
  let h = 0
  const s = max === 0 ? 0 : d / max
  const v = max

  if (max !== min) {
    switch (max) {
      case r: h = (g - b) / d + (g < b ? 6 : 0); break
      case g: h = (b - r) / d + 2; break
      case b: h = (r - g) / d + 4; break
    }
    h /= 6
  }

  return { h: h * 360, s: s * 100, v: v * 100 }
}

function hsvToRgb(h: number, s: number, v: number): { r: number; g: number; b: number } {
  h /= 360
  s /= 100
  v /= 100

  let r = 0, g = 0, b = 0
  const i = Math.floor(h * 6)
  const f = h * 6 - i
  const p = v * (1 - s)
  const q = v * (1 - f * s)
  const t = v * (1 - (1 - f) * s)

  switch (i % 6) {
    case 0: r = v; g = t; b = p; break
    case 1: r = q; g = v; b = p; break
    case 2: r = p; g = v; b = t; break
    case 3: r = p; g = q; b = v; break
    case 4: r = t; g = p; b = v; break
    case 5: r = v; g = p; b = q; break
  }

  return {
    r: Math.round(r * 255),
    g: Math.round(g * 255),
    b: Math.round(b * 255),
  }
}

export function ColorPicker() {
  const { brushColor, setBrushColor } = useAppStore()
  const [expanded, setExpanded] = useState(false)
  const [hexInput, setHexInput] = useState(brushColor)
  const [presetColors, setPresetColors] = useState<string[]>(defaultPresetColors)
  const [presetName, setPresetName] = useState('')

  const svCanvasRef = useRef<HTMLCanvasElement>(null)
  const hueCanvasRef = useRef<HTMLCanvasElement>(null)
  const isDraggingSV = useRef(false)
  const isDraggingHue = useRef(false)

  // Parse current color to HSV
  const rgb = hexToRgb(brushColor)
  const hsv = rgbToHsv(rgb.r, rgb.g, rgb.b)

  // Sync hex input when brushColor changes externally
  useEffect(() => {
    setHexInput(brushColor)
  }, [brushColor])

  // Draw saturation-value canvas
  useEffect(() => {
    const canvas = svCanvasRef.current
    if (!canvas || !expanded) return

    const ctx = canvas.getContext('2d')
    if (!ctx) return

    const width = canvas.width
    const height = canvas.height

    // Create saturation gradient (white to hue color)
    const hueRgb = hsvToRgb(hsv.h, 100, 100)
    const satGradient = ctx.createLinearGradient(0, 0, width, 0)
    satGradient.addColorStop(0, '#FFFFFF')
    satGradient.addColorStop(1, rgbToHex(hueRgb.r, hueRgb.g, hueRgb.b))

    ctx.fillStyle = satGradient
    ctx.fillRect(0, 0, width, height)

    // Create value gradient (transparent to black)
    const valGradient = ctx.createLinearGradient(0, 0, 0, height)
    valGradient.addColorStop(0, 'rgba(0,0,0,0)')
    valGradient.addColorStop(1, 'rgba(0,0,0,1)')

    ctx.fillStyle = valGradient
    ctx.fillRect(0, 0, width, height)
  }, [hsv.h, expanded])

  // Draw hue canvas
  useEffect(() => {
    const canvas = hueCanvasRef.current
    if (!canvas || !expanded) return

    const ctx = canvas.getContext('2d')
    if (!ctx) return

    const width = canvas.width
    const height = canvas.height

    const gradient = ctx.createLinearGradient(0, 0, width, 0)
    for (let i = 0; i <= 360; i += 60) {
      const rgb = hsvToRgb(i, 100, 100)
      gradient.addColorStop(i / 360, rgbToHex(rgb.r, rgb.g, rgb.b))
    }

    ctx.fillStyle = gradient
    ctx.fillRect(0, 0, width, height)
  }, [expanded])

  const handleSVMouseDown = useCallback((e: React.MouseEvent<HTMLCanvasElement>) => {
    isDraggingSV.current = true
    handleSVChange(e)
  }, [hsv.h])

  const handleSVChange = useCallback((e: React.MouseEvent<HTMLCanvasElement>) => {
    const canvas = svCanvasRef.current
    if (!canvas) return

    const rect = canvas.getBoundingClientRect()
    const x = Math.max(0, Math.min(1, (e.clientX - rect.left) / rect.width))
    const y = Math.max(0, Math.min(1, (e.clientY - rect.top) / rect.height))

    const newS = x * 100
    const newV = (1 - y) * 100
    const newRgb = hsvToRgb(hsv.h, newS, newV)
    setBrushColor(rgbToHex(newRgb.r, newRgb.g, newRgb.b))
  }, [hsv.h, setBrushColor])

  const handleHueMouseDown = useCallback((e: React.MouseEvent<HTMLCanvasElement>) => {
    isDraggingHue.current = true
    handleHueChange(e)
  }, [hsv.s, hsv.v])

  const handleHueChange = useCallback((e: React.MouseEvent<HTMLCanvasElement>) => {
    const canvas = hueCanvasRef.current
    if (!canvas) return

    const rect = canvas.getBoundingClientRect()
    const x = Math.max(0, Math.min(1, (e.clientX - rect.left) / rect.width))

    const newH = x * 360
    const newRgb = hsvToRgb(newH, hsv.s, hsv.v)
    setBrushColor(rgbToHex(newRgb.r, newRgb.g, newRgb.b))
  }, [hsv.s, hsv.v, setBrushColor])

  useEffect(() => {
    const handleMouseMove = (e: MouseEvent) => {
      if (isDraggingSV.current) {
        handleSVChange(e as unknown as React.MouseEvent<HTMLCanvasElement>)
      }
      if (isDraggingHue.current) {
        handleHueChange(e as unknown as React.MouseEvent<HTMLCanvasElement>)
      }
    }

    const handleMouseUp = () => {
      isDraggingSV.current = false
      isDraggingHue.current = false
    }

    window.addEventListener('mousemove', handleMouseMove)
    window.addEventListener('mouseup', handleMouseUp)

    return () => {
      window.removeEventListener('mousemove', handleMouseMove)
      window.removeEventListener('mouseup', handleMouseUp)
    }
  }, [handleSVChange, handleHueChange])

  const handleRgbChange = (channel: 'r' | 'g' | 'b', value: number) => {
    const newRgb = { ...rgb, [channel]: Math.max(0, Math.min(255, value)) }
    setBrushColor(rgbToHex(newRgb.r, newRgb.g, newRgb.b))
  }

  const handleHexInputChange = (value: string) => {
    setHexInput(value)
    if (/^#[0-9A-Fa-f]{6}$/.test(value)) {
      setBrushColor(value.toUpperCase())
    }
  }

  // Add current color to presets
  const addColorToPreset = () => {
    if (!presetColors.includes(brushColor)) {
      setPresetColors([...presetColors, brushColor])
    }
  }

  // Remove color from presets
  const removeColorFromPreset = (color: string) => {
    setPresetColors(presetColors.filter(c => c !== color))
  }

  // Import color palette from file
  const handleImportPalette = async () => {
    try {
      const path = await open({
        filters: [
          { name: '调色板文件', extensions: ['json', 'txt', 'hex', 'gpl'] }
        ],
        multiple: false,
      })

      if (path && typeof path === 'string') {
        const content = await readTextFile(path)
        let colors: string[] = []

        // Try to parse as JSON first
        try {
          const json = JSON.parse(content)
          if (Array.isArray(json)) {
            colors = json.filter(c => /^#[0-9A-Fa-f]{6}$/i.test(c)).map(c => c.toUpperCase())
          } else if (json.colors && Array.isArray(json.colors)) {
            colors = json.colors.filter((c: string) => /^#[0-9A-Fa-f]{6}$/i.test(c)).map((c: string) => c.toUpperCase())
          }
          if (json.name) {
            setPresetName(json.name)
          }
        } catch {
          // Try to parse as plain text (one color per line)
          const lines = content.split(/[\r\n]+/)
          for (const line of lines) {
            const trimmed = line.trim()
            // Match hex colors with or without #
            const match = trimmed.match(/^#?([0-9A-Fa-f]{6})$/i)
            if (match) {
              colors.push('#' + match[1].toUpperCase())
            }
            // Also try to match GIMP palette format: R G B
            const rgbMatch = trimmed.match(/^(\d{1,3})\s+(\d{1,3})\s+(\d{1,3})/)
            if (rgbMatch) {
              const r = parseInt(rgbMatch[1])
              const g = parseInt(rgbMatch[2])
              const b = parseInt(rgbMatch[3])
              if (r <= 255 && g <= 255 && b <= 255) {
                colors.push(rgbToHex(r, g, b))
              }
            }
          }
        }

        if (colors.length > 0) {
          setPresetColors(colors)
          await message(`成功导入 ${colors.length} 个颜色`, { title: '导入成功', type: 'info' })
        } else {
          await message('未找到有效的颜色数据', { title: '导入失败', type: 'warning' })
        }
      }
    } catch (error) {
      await message(String(error), { title: '导入失败', type: 'error' })
    }
  }

  // Export color palette to file
  const handleExportPalette = async () => {
    try {
      const path = await save({
        filters: [
          { name: '调色板文件', extensions: ['json'] }
        ],
        defaultPath: `${presetName || 'palette'}.json`,
      })

      if (path) {
        const data = {
          name: presetName || 'My Palette',
          colors: presetColors,
          version: '1.0',
        }
        await writeTextFile(path, JSON.stringify(data, null, 2))
        await message('调色板导出成功', { title: '成功', type: 'info' })
      }
    } catch (error) {
      await message(String(error), { title: '导出失败', type: 'error' })
    }
  }

  // Reset to default palette
  const resetPalette = () => {
    setPresetColors(defaultPresetColors)
    setPresetName('')
  }

  // Calculate picker positions
  const svX = (hsv.s / 100) * 100
  const svY = ((100 - hsv.v) / 100) * 100
  const hueX = (hsv.h / 360) * 100

  return (
    <div className={`color-picker ${expanded ? 'expanded' : ''}`}>
      <div className="color-picker-header">
        <div className="current-color-display">
          <div
            className="color-preview large"
            style={{ backgroundColor: brushColor }}
            title={t('color.picker')}
          />
          <input
            type="color"
            value={brushColor}
            onChange={(e) => setBrushColor(e.target.value.toUpperCase())}
            className="color-input"
          />
        </div>
        <div className="color-info">
          <span className="color-hex">{brushColor}</span>
        </div>
        <button
          className="expand-btn"
          onClick={() => setExpanded(!expanded)}
          title={expanded ? t('color.collapse') : t('color.expand')}
        >
          {expanded ? <ChevronUp size={18} /> : <ChevronDown size={18} />}
        </button>
      </div>

      {expanded && (
        <div className="color-picker-expanded">
          {/* Saturation-Value picker */}
          <div className="sv-picker">
            <canvas
              ref={svCanvasRef}
              width={280}
              height={180}
              onMouseDown={handleSVMouseDown}
            />
            <div
              className="sv-cursor"
              style={{ left: `${svX}%`, top: `${svY}%` }}
            />
          </div>

          {/* Hue slider */}
          <div className="hue-picker">
            <canvas
              ref={hueCanvasRef}
              width={280}
              height={20}
              onMouseDown={handleHueMouseDown}
            />
            <div
              className="hue-cursor"
              style={{ left: `${hueX}%` }}
            />
          </div>

          {/* RGB inputs */}
          <div className="rgb-inputs">
            <div className="rgb-input-group">
              <label>R</label>
              <input
                type="number"
                min="0"
                max="255"
                value={rgb.r}
                onChange={(e) => handleRgbChange('r', parseInt(e.target.value) || 0)}
              />
            </div>
            <div className="rgb-input-group">
              <label>G</label>
              <input
                type="number"
                min="0"
                max="255"
                value={rgb.g}
                onChange={(e) => handleRgbChange('g', parseInt(e.target.value) || 0)}
              />
            </div>
            <div className="rgb-input-group">
              <label>B</label>
              <input
                type="number"
                min="0"
                max="255"
                value={rgb.b}
                onChange={(e) => handleRgbChange('b', parseInt(e.target.value) || 0)}
              />
            </div>
          </div>

          {/* Hex input */}
          <div className="hex-input-group">
            <label>HEX</label>
            <input
              type="text"
              value={hexInput}
              onChange={(e) => handleHexInputChange(e.target.value)}
              placeholder="#000000"
              maxLength={7}
            />
          </div>
        </div>
      )}

      {/* Preset colors section */}
      <div className="preset-section">
        <div className="preset-header">
          <span className="preset-title">{t('color.presets')}</span>
          <div className="preset-actions">
            <button
              className="icon-btn tiny"
              onClick={addColorToPreset}
              title="添加当前颜色"
            >
              <Plus size={12} />
            </button>
            <button
              className="icon-btn tiny"
              onClick={handleImportPalette}
              title="导入调色板"
            >
              <Download size={12} />
            </button>
            <button
              className="icon-btn tiny"
              onClick={handleExportPalette}
              title="导出调色板"
            >
              <Upload size={12} />
            </button>
            <button
              className="icon-btn tiny"
              onClick={resetPalette}
              title="重置默认"
            >
              <Trash2 size={12} />
            </button>
          </div>
        </div>
        <div className="preset-colors">
          {presetColors.map((color, index) => (
            <button
              key={`${color}-${index}`}
              className={`preset-color ${color === brushColor ? 'active' : ''}`}
              style={{ backgroundColor: color }}
              onClick={() => setBrushColor(color)}
              onContextMenu={(e) => {
                e.preventDefault()
                removeColorFromPreset(color)
              }}
              title={`${color}\n右键删除`}
            />
          ))}
        </div>
      </div>
    </div>
  )
}
