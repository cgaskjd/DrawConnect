import { useMemo } from 'react'
import { useAppStore } from '../../stores/appStore'
import { t } from '../../i18n'
import { useAdjustmentDialog } from '../../hooks/useAdjustmentDialog'
import { BaseAdjustmentDialog, AdjustmentRow, SliderInput } from './BaseAdjustmentDialog'

interface HueSaturationDialogProps {
  isOpen: boolean
  onClose: () => void
}

export function HueSaturationDialog({ isOpen, onClose }: HueSaturationDialogProps) {
  const { adjustHueSaturation } = useAppStore()

  const initialValues = useMemo(() => ({ hue: 0, saturation: 0, lightness: 0 }), [])

  const { values, handleCancel, handleReset, updateValue } = useAdjustmentDialog({
    isOpen,
    initialValues,
    isDefault: (v) => v.hue === 0 && v.saturation === 0 && v.lightness === 0,
    applyEffect: async (v) => { await adjustHueSaturation(v.hue, v.saturation, v.lightness) },
  })

  const onCancel = async () => {
    await handleCancel()
    onClose()
  }

  return (
    <BaseAdjustmentDialog
      isOpen={isOpen}
      title={t('menu.hueSaturation')}
      onCancel={onCancel}
      onOk={onClose}
      onReset={handleReset}
    >
      <AdjustmentRow label={t('adjustment.hue')}>
        <SliderInput
          value={values.hue}
          onChange={(v) => updateValue('hue', v)}
          min={-180}
          max={180}
          step={1}
        />
      </AdjustmentRow>
      <AdjustmentRow label={t('adjustment.saturation')}>
        <SliderInput
          value={values.saturation}
          onChange={(v) => updateValue('saturation', v)}
          min={-1}
          max={1}
          step={0.01}
          displayMultiplier={100}
        />
      </AdjustmentRow>
      <AdjustmentRow label={t('adjustment.lightness')}>
        <SliderInput
          value={values.lightness}
          onChange={(v) => updateValue('lightness', v)}
          min={-1}
          max={1}
          step={0.01}
          displayMultiplier={100}
        />
      </AdjustmentRow>
    </BaseAdjustmentDialog>
  )
}
