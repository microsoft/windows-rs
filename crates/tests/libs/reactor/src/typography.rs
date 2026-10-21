use super::*;

// The NavigationView header template gives its content a larger font size than the TextBlock
// default, so clearing a local size must restore the inherited value rather than 14.
pub(super) struct InheritedTypographyFixture(bool);

impl Component for InheritedTypographyFixture {
    type Input = ();
    type Message = bool;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self(false)
    }

    fn update(&mut self, message: bool, _: &ComponentContext<Self>) {
        self.0 = message;
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        let mut header = TextBlock::new()
            .automation_id("header")
            .text("Header")
            .horizontal_alignment(HorizontalAlignment::Left);
        if self.0 {
            header = header.font_size(14.0);
        }
        context.window_visuals(WindowVisuals::new().client_size(500.0, 400.0));
        context.window_frame(
            "Reactor UI fixture: typography-inherited",
            NavigationView::new()
                .is_settings_visible(false)
                .header(header)
                .content(
                    StackPanel::new().margin(20.0).spacing(8.0).children((
                        Button::new()
                            .automation_id("apply")
                            .content("Apply font size")
                            .on_click(context.message(true)),
                        Button::new()
                            .automation_id("reset")
                            .content("Reset font size")
                            .on_click(context.message(false)),
                    )),
                ),
        )
    }
}

pub(super) struct TypographyFixture(u8);

impl Component for TypographyFixture {
    type Input = ();
    type Message = u8;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self(0)
    }

    fn update(&mut self, message: u8, _: &ComponentContext<Self>) {
        self.0 = message;
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        let text = |id, value| {
            TextBlock::new()
                .automation_id(id)
                .text(value)
                .font_size(20.0)
                .horizontal_alignment(HorizontalAlignment::Left)
        };
        let mut family = text("family", "iiiiiiii");
        let mut spacing = text("spacing", "Spacing");
        let mut height = text("height", "Line one\nLine two\nLine three");
        match self.0 {
            0 => {}
            1 => {
                family = family.font_family("Consolas");
                spacing = spacing.character_spacing(250);
                height = height.line_height(48.0);
            }
            2 | 3 => {
                family = family.font_family("Arial");
                spacing = spacing.character_spacing(-100);
                height = height.line_height(if self.0 == 2 { 32.0 } else { 0.0 });
            }
            _ => unreachable!(),
        }
        context.window_visuals(WindowVisuals::new().client_size(500.0, 600.0));
        context.window_frame(
            "Reactor UI fixture: typography",
            StackPanel::new().margin(20.0).spacing(8.0).children((
                family,
                spacing,
                height,
                Button::new()
                    .automation_id("apply")
                    .content("Apply typography")
                    .on_click(context.message(1)),
                Button::new()
                    .automation_id("change")
                    .content("Change typography")
                    .on_click(context.message(2)),
                Button::new()
                    .automation_id("automatic")
                    .content("Automatic line height")
                    .on_click(context.message(3)),
                Button::new()
                    .automation_id("reset")
                    .content("Reset typography")
                    .on_click(context.message(0)),
            )),
        )
    }
}
