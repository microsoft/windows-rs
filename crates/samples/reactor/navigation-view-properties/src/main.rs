use windows_reactor::*;

struct NavigationProperties {
    header: usize,
    back_enabled: bool,
    pane_visible: bool,
}

#[derive(Clone)]
enum Message {
    CycleHeader,
    ToggleBack,
    TogglePane,
}

impl Component for NavigationProperties {
    type Input = ();
    type Message = Message;

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            header: 0,
            back_enabled: false,
            pane_visible: true,
        }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::CycleHeader => self.header = (self.header + 1) % 3,
            Message::ToggleBack => self.back_enabled = !self.back_enabled,
            Message::TogglePane => self.pane_visible = !self.pane_visible,
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("NavigationView properties");
        let navigation = NavigationView::new()
            .pane_display_mode(NavigationViewPaneDisplayMode::Left)
            .open_pane_length(240.0)
            .is_pane_toggle_button_visible(false)
            .is_settings_visible(false)
            .is_back_button_visible(NavigationViewBackButtonVisible::Visible)
            .is_back_enabled(self.back_enabled)
            .is_pane_visible(self.pane_visible)
            .is_pane_open(self.pane_visible)
            .menu_items((NavigationViewItem::new().content("Home").icon(Symbol::Home),))
            .pane_footer("Pane footer stays unchanged")
            .content(
                Border::new().padding(24.0).content(
                    StackPanel::new().spacing(12.0).children((
                        "Change these settings and watch the pane on the left.",
                        format!("PaneHeader: {}", ["Text", "Border", "None"][self.header]),
                        Button::new()
                            .content("Cycle header: text -> border -> none")
                            .on_click(context.message(Message::CycleHeader)),
                        format!("IsBackEnabled: {}", self.back_enabled),
                        Button::new()
                            .content("Toggle back arrow enabled")
                            .on_click(context.message(Message::ToggleBack)),
                        "The back arrow demonstrates enabled state only; it does not navigate.",
                        format!("IsPaneVisible: {}", self.pane_visible),
                        Button::new()
                            .content("Show / hide pane")
                            .on_click(context.message(Message::TogglePane)),
                        "These controls remain available while the pane is hidden.",
                    )),
                ),
            );

        match self.header {
            0 => navigation.pane_header("Workspace A"),
            1 => navigation.pane_header(
                Border::new()
                    .padding(12.0)
                    .border_thickness(2.0)
                    .border_brush(Color::rgb(60, 120, 220))
                    .content("Workspace B"),
            ),
            _ => navigation,
        }
        .into()
    }
}

fn main() {
    App::run_component::<NavigationProperties>(()).unwrap();
}
