#![windows_subsystem = "windows"]

use windows_reactor::*;

struct HyperlinkButtonSample {
    clicks: u32,
}

impl Component for HyperlinkButtonSample {
    type Message = ();
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self { clicks: 0 }
    }

    fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {
        self.clicks += 1;
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("HyperlinkButton");
        StackPanel::new().spacing(8.0).children((
            HyperlinkButton::new()
                .navigate_uri("https://learn.microsoft.com/windows/apps/")
                .unwrap()
                .content(TextBlock::new().text("Open Microsoft Docs")),
            HyperlinkButton::new()
                .on_click(context.message(()))
                .content(TextBlock::new().text(format!("Clicked {} times", self.clicks))),
            HyperlinkButton::new()
                .navigate_uri("https://example.com/")
                .unwrap()
                .is_enabled(false)
                .content(TextBlock::new().text("Disabled")),
        ))
    }
}

fn main() {
    App::run_component::<HyperlinkButtonSample>(()).unwrap();
}
