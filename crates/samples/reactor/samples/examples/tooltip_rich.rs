#![windows_subsystem = "windows"]

use windows_reactor::*;

struct RichTooltipSample;

impl Component for RichTooltipSample {
    type Message = ();
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("TooltipRich");
        let rich_panel = StackPanel::new().spacing(4.0).children((
            TextBlock::new().text("Action: Save").font_weight(700),
            "Writes the current document to disk.",
        ));

        StackPanel::new().spacing(8.0).children((
            Button::new()
                .content("Save")
                .tooltip_with(Tooltip::rich(rich_panel)),
            Button::new().content("Open").tooltip("Opens a document"),
        ))
    }
}

fn main() {
    App::run_component::<RichTooltipSample>(()).unwrap();
}
