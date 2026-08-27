use crate::controls::*;
use windows_reactor::*;

pub struct RelativePanelPage {
    bottom: bool,
}

impl Component for RelativePanelPage {
    type Message = bool;
    type Input = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self { bottom: false }
    }

    fn update(&mut self, bottom: bool, _: &ComponentContext<Self>) {
        self.bottom = bottom;
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        let left = if self.bottom {
            TextBlock::new()
                .text("Bottom left")
                .relative_align_left()
                .relative_align_bottom()
        } else {
            TextBlock::new()
                .text("Top left")
                .relative_align_left()
                .relative_align_top()
        };
        let right = if self.bottom {
            TextBlock::new()
                .text("Bottom right")
                .relative_align_right()
                .relative_align_bottom()
        } else {
            TextBlock::new()
                .text("Top right")
                .relative_align_right()
                .relative_align_top()
        };
        page_content(
            "RelativePanel",
            "Positions children relative to the panel edges and center.",
            [KeyedView::new(
                "layout",
                sample_card(
                    "Switch Layouts",
                    StackPanel::new().spacing(12.0).children((
                        ToggleSwitch::new()
                            .is_on(self.bottom)
                            .on_toggled(context.forward())
                            .slot(ToggleSwitchSlot::Header, "Show bottom corners"),
                        RelativePanel::new().height(200.0).children((
                            left,
                            right,
                            TextBlock::new()
                                .text("Center")
                                .relative_align_horizontal_center()
                                .relative_align_vertical_center(),
                        )),
                    )),
                    "RelativePanel::new().children((child.relative_align_right(), ...))",
                ),
            )],
        )
    }
}
