import { useMemo } from 'react'
import { useAppStore } from '../../stores/appStore'
import { t } from '../../i18n'
import { useAdjustmentDialog } from '../../hooks/useAdjustmentDialog'
import { BaseAdjustmentDialog, AdjustmentRow, SliderInput } from './BaseAdjustmentDialog'

interface EmbossDialogProps {
  isOpen: boolean
  onClose: () => void
}

export function EmbossDialog({ isOpen, onClose }: EmbossDialogProps) {
  const { filterEmboss } = useAppStore()

  const initialValues = useMemo(() => ({ angle: 135, height: 1, amount: 100 }), [])

  const { values, handleCancel, handleReset, updateValue } = useAdjustmentDialog({
    isOpen,
    initialValues,
    isDefault: () => false,
    applyEffect: async (v) => { await filterEmboss(v.angle, v.height, v.amount) },
  })

  const onCancel = async () => {
    await handleCancel()
    onClose()
  }

  return (
    <BaseAdjustmentDialog
      isOpen={isOpen}
      title={t('menu.emboss')}
      onCancel={onCancel}
      onOk={onClose}
      onReset={handleReset}
    >
      <AdjustmentRow label={t('adjustment.angle')}>
        <SliderInput
          value={values.angle}
          onChange={(v) => updateValue('angle', v)}
          min={0}
          max={360}
          step={1}
        />
      </AdjustmentRow>
      <AdjustmentRow label={t('adjustment.height')}>
        <SliderInput
          value={values.height}
          onChange={(v) => updateValue('height', v)}
          min={1}
          max={10}
          step={0.1}
        />
      </AdjustmentRow>
      <AdjustmentRow label={t('adjustment.amount')}>
        <SliderInput
          value={values.amount}
          onChange={(v) => updateValue('amount', v)}
          min={0}
          max={500}
          step={1}
        />
      </AdjustmentRow>
    </BaseAdjustmentDialog>
  )
}
