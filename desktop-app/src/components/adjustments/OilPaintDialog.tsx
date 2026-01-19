import { useMemo } from 'react'
import { useAppStore } from '../../stores/appStore'
import { t } from '../../i18n'
import { useAdjustmentDialog } from '../../hooks/useAdjustmentDialog'
import { BaseAdjustmentDialog, AdjustmentRow, SliderInput } from './BaseAdjustmentDialog'

interface OilPaintDialogProps {
  isOpen: boolean
  onClose: () => void
}

export function OilPaintDialog({ isOpen, onClose }: OilPaintDialogProps) {
  const { filterOilPaint } = useAppStore()

  const initialValues = useMemo(() => ({ radius: 4, levels: 20 }), [])

  const { values, handleCancel, handleReset, updateValue } = useAdjustmentDialog({
    isOpen,
    initialValues,
    isDefault: () => false,
    applyEffect: async (v) => { await filterOilPaint(v.radius, v.levels) },
  })

  const onCancel = async () => {
    await handleCancel()
    onClose()
  }

  return (
    <BaseAdjustmentDialog
      isOpen={isOpen}
      title={t('menu.oilPaint')}
      onCancel={onCancel}
      onOk={onClose}
      onReset={handleReset}
    >
      <AdjustmentRow label={t('adjustment.radius')}>
        <SliderInput
          value={values.radius}
          onChange={(v) => updateValue('radius', v)}
          min={1}
          max={10}
          step={1}
        />
      </AdjustmentRow>
      <AdjustmentRow label={t('adjustment.levels')}>
        <SliderInput
          value={values.levels}
          onChange={(v) => updateValue('levels', v)}
          min={2}
          max={256}
          step={1}
        />
      </AdjustmentRow>
    </BaseAdjustmentDialog>
  )
}
