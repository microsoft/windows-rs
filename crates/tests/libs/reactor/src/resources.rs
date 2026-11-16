use super::*;

// Writes every `ResourceValue` kind into an element's resource dictionary. WinUI asks the boxed
// struct values for their runtime class names, so this covers the names of generated structs.
pub(super) struct ResourcesFixture {
    styled: bool,
    toggles: u32,
}

impl Component for ResourcesFixture {
    type Input = ();
    type Message = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            styled: true,
            toggles: 0,
        }
    }

    fn update(&mut self, _: (), _: &ComponentContext<Self>) {
        self.styled = !self.styled;
        self.toggles += 1;
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Reactor UI fixture: resources");
        context.window_visuals(WindowVisuals::new().client_size(500.0, 300.0));
        let mut target = Button::new().automation_id("target").content("Target");
        if self.styled {
            target = target.resource_overrides(
                ResourceOverrides::new()
                    .set("ButtonBackground", Color::rgb(178, 34, 34))
                    .set("ButtonBorderThemeThickness", Thickness::uniform(2.0))
                    .set("ControlCornerRadius", CornerRadius::uniform(8.0)),
            );
        }
        StackPanel::new()
            .margin(20.0)
            .spacing(12.0)
            .children((
                TextBlock::new()
                    .automation_id("state")
                    .text(format!("Styled {}; toggles {}", self.styled, self.toggles)),
                target,
                Button::new()
                    .automation_id("toggle")
                    .content("Toggle resources")
                    .on_click(context.forward()),
            ))
            .into()
    }
}
