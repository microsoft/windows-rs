use super::*;

impl WinUiRuntime {
    pub(super) fn apply_forced_canvas_scale(
        &mut self,
        mut layout: NativeCanvasLayout,
    ) -> NativeCanvasLayout {
        if layout.width > 0.0
            && layout.height > 0.0
            && let Some((scale_x, scale_y)) = self.canvas_test_scale.take()
        {
            layout.scale_x = scale_x;
            layout.scale_y = scale_y;
        }
        layout
    }

    pub(super) fn take_forced_canvas_present_loss(&mut self) -> bool {
        std::mem::take(&mut self.canvas_test_present_loss)
    }
}
