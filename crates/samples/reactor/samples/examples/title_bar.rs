use windows_core::Result;
use windows_reactor::*;

struct Sample {
    back_clicks: i32,
    pane_clicks: i32,
}

#[derive(Clone)]
enum Message {
    Back,
    Pane,
}

impl Component for Sample {
    type Input = ();
    type Message = Message;

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            back_clicks: 0,
            pane_clicks: 0,
        }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::Back => self.back_clicks += 1,
            Message::Pane => self.pane_clicks += 1,
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("TitleBar");

        StackPanel::new().spacing(8.0).children((
            TitleBar::new()
                .preferred_height(WindowTitleBarHeight::Tall)
                .title("windows_reactor - title_bar sample")
                .subtitle("Minimal demo")
                .is_back_button_visible(true)
                .is_back_button_enabled(true)
                .is_pane_toggle_button_visible(true)
                .on_back_requested(context.message(Message::Back))
                .on_pane_toggle_requested(context.message(Message::Pane)),
            TextBlock::new().text(format!(
                "back_clicks = {}, pane_toggle_clicks = {}",
                self.back_clicks, self.pane_clicks
            )),
        ))
    }
}

fn main() -> Result<()> {
    App::run_component::<Sample>(())
}
