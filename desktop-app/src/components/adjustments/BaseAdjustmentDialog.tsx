import { ReactNode } from 'react'
import { t } from '../../i18n'
import './AdjustmentDialog.css'

interface BaseAdjustmentDialogProps {
  isOpen: boolean
  title: string
  onCancel: () => void
  onOk: () => void
  onReset: () => void
  children: ReactNode
}

export function BaseAdjustmentDialog({
  isOpen,
  title,
  onCancel,
  onOk,
  onReset,
  children,
}: BaseAdjustmentDialogProps) {
  if (!isOpen) return null

  return (
    <div className="dialog-overlay" onClick={onCancel}>
      <div className="adjustment-dialog" onClick={e => e.stopPropagation()}>
        <div className="dialog-header">
          <h3>{title}</h3>
          <button className="close-btn" onClick={onCancel}>×</button>
        </div>

        <div className="dialog-content">
          {children}
        </div>

        <div className="dialog-footer">
          <button className="btn-secondary" onClick={onReset}>
            {t('adjustment.reset')}
          </button>
          <div className="footer-right">
            <button className="btn-secondary" onClick={onCancel}>
              {t('dialog.cancel')}
            </button>
            <button className="btn-primary" onClick={onOk}>
              {t('dialog.ok')}
            </button>
          </div>
        </div>
      </div>
    </div>
  )
}

interface AdjustmentRowProps {
  label: string
  children: ReactNode
}

export function AdjustmentRow({ label, children }: AdjustmentRowProps) {
  return (
    <div className="adjustment-row">
      <label>{label}</label>
      {children}
    </div>
  )
}

interface SliderInputProps {
  value: number
  onChange: (value: number) => void
  min: number
  max: number
  step?: number
  displayMultiplier?: number
}

export function SliderInput({
  value,
  onChange,
  min,
  max,
  step = 1,
  displayMultiplier = 1,
}: SliderInputProps) {
  const displayValue = displayMultiplier !== 1
    ? Math.round(value * displayMultiplier)
    : (step < 1 ? value.toFixed(1) : value)

  return (
    <>
      <input
        type="range"
        min={min}
        max={max}
        step={step}
        value={value}
        onChange={e => onChange(parseFloat(e.target.value))}
      />
      <input
        type="number"
        min={displayMultiplier !== 1 ? min * displayMultiplier : min}
        max={displayMultiplier !== 1 ? max * displayMultiplier : max}
        value={displayValue}
        onChange={e => {
          const numVal = parseFloat(e.target.value) || 0
          onChange(displayMultiplier !== 1 ? numVal / displayMultiplier : numVal)
        }}
        className="value-input"
      />
    </>
  )
}
