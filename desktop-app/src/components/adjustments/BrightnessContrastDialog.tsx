import { useMemo } from 'react'
import { useAppStore } from '../../stores/appStore'
import { t } from '../../i18n'
import { useAdjustmentDialog } from '../../hooks/useAdjustmentDialog'
import { BaseAdjustmentDialog, AdjustmentRow, SliderInput } from './BaseAdjustmentDialog'

interface BrightnessContrastDialogProps {
  isOpen: boolean
  onClose: () => void
}

interface BrightnessContrastValues {
  brightness: number
  contrast: number
}

export function BrightnessContrastDialog({ isOpen, onClose }: BrightnessContrastDialogProps) {
  const { adjustBrightnessContrast } = useAppStore()

  const initialValues = useMemo(() => ({ brightness: 0, contrast: 0 }), [])

  const { values, handleCancel, handleReset, updateValue } = useAdjustmentDialog<BrightnessContrastValues>({
    isOpen,
    initialValues,
    isDefault: (v) => v.brightness === 0 && v.contrast === 0,
    applyEffect: async (v) => { await adjustBrightnessContrast(v.brightness, v.contrast) },
  })

  const onCancel = async () => {
    await handleCancel()
    onClose()
  }

  return (
    <BaseAdjustmentDialog
      isOpen={isOpen}
      title={t('menu.brightnessContrast')}
      onCancel={onCancel}
      onOk={onClose}
      onReset={handleReset}
    >
      <AdjustmentRow label={t('adjustment.brightness')}>
        <SliderInput
          value={values.brightness}
          onChange={(v) => updateValue('brightness', v)}
          min={-1}
          max={1}
          step={0.01}
          displayMultiplier={100}
        />
      </AdjustmentRow>
      <AdjustmentRow label={t('adjustment.contrast')}>
        <SliderInput
          value={values.contrast}
          onChange={(v) => updateValue('contrast', v)}
          min={-1}
          max={1}
          step={0.01}
          displayMultiplier={100}
        />
      </AdjustmentRow>
    </BaseAdjustmentDialog>
  )
}
