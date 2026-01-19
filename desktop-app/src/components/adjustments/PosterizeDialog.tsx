import { useMemo } from 'react'
import { useAppStore } from '../../stores/appStore'
import { t } from '../../i18n'
import { useAdjustmentDialog } from '../../hooks/useAdjustmentDialog'
import { BaseAdjustmentDialog, AdjustmentRow, SliderInput } from './BaseAdjustmentDialog'

interface PosterizeDialogProps {
  isOpen: boolean
  onClose: () => void
}

export function PosterizeDialog({ isOpen, onClose }: PosterizeDialogProps) {
  const { adjustPosterize } = useAppStore()

  const initialValues = useMemo(() => ({ levels: 4 }), [])

  const { values, handleCancel, handleReset, updateValue } = useAdjustmentDialog({
    isOpen,
    initialValues,
    isDefault: () => false,
    applyEffect: async (v) => { await adjustPosterize(v.levels) },
  })

  const onCancel = async () => {
    await handleCancel()
    onClose()
  }

  return (
    <BaseAdjustmentDialog
      isOpen={isOpen}
      title={t('menu.posterize')}
      onCancel={onCancel}
      onOk={onClose}
      onReset={handleReset}
    >
      <AdjustmentRow label={t('adjustment.levels')}>
        <SliderInput
          value={values.levels}
          onChange={(v) => updateValue('levels', v)}
          min={2}
          max={32}
          step={1}
        />
      </AdjustmentRow>
    </BaseAdjustmentDialog>
  )
}
