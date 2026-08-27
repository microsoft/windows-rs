#![windows_subsystem = "windows"]

use windows_reactor::*;

fn display_mode_name(mode: NavigationViewDisplayMode) -> &'static str {
    match mode {
        NavigationViewDisplayMode::Minimal => "minimal",
        NavigationViewDisplayMode::Compact => "compact",
        NavigationViewDisplayMode::Expanded => "expanded",
    }
}

enum NavigationMessage {
    PaneOpen(bool),
    DisplayMode(NavigationViewDisplayMode),
    TogglePane,
}

struct ResponsiveNavigation {
    pane_open: bool,
    display_mode: NavigationViewDisplayMode,
}

impl Component for ResponsiveNavigation {
    type Message = NavigationMessage;
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            pane_open: true,
            display_mode: NavigationViewDisplayMode::Expanded,
        }
    }

    fn update(&mut self, message: NavigationMessage, _context: &ComponentContext<Self>) {
        match message {
            NavigationMessage::PaneOpen(value) => self.pane_open = value,
            NavigationMessage::DisplayMode(value) => self.display_mode = value,
            NavigationMessage::TogglePane => self.pane_open = !self.pane_open,
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Responsive Navigation");
        let footer = if self.display_mode == NavigationViewDisplayMode::Expanded {
            "Signed in: Ada"
        } else {
            "AD"
        };
        let items = [
            ("home", "Home", Symbol::Home),
            ("documents", "Documents", Symbol::Document),
        ]
        .into_iter()
        .map(|(tag, label, symbol)| {
            KeyedView::new(
                tag,
                NavigationViewItem::new().tag(tag).slots([
                    SlotView::new(
                        NavigationViewItemSlot::Content,
                        TextBlock::new().text(label),
                    ),
                    SlotView::new(
                        NavigationViewItemSlot::Icon,
                        SymbolIcon::new().symbol(symbol),
                    ),
                ]),
            )
        });
        NavigationView::new()
            .is_pane_open(self.pane_open)
            .on_is_pane_open_changed(context.callback(NavigationMessage::PaneOpen))
            .pane_display_mode(NavigationViewPaneDisplayMode::Auto)
            .on_display_mode_changed(context.callback(NavigationMessage::DisplayMode))
            .pane_title("Responsive navigation")
            .is_settings_visible(false)
            .slots([
                SlotView::collection(NavigationViewSlot::MenuItems, items),
                SlotView::new(
                    NavigationViewSlot::Content,
                    StackPanel::new().spacing(12.0).children((
                        TextBlock::new().text(format!(
                            "Actual display mode: {}",
                            display_mode_name(self.display_mode)
                        )),
                        TextBlock::new().text(if self.pane_open {
                            "Pane is open"
                        } else {
                            "Pane is closed"
                        }),
                        Button::new()
                            .on_click(context.callback(|_| NavigationMessage::TogglePane))
                            .content(TextBlock::new().text("Toggle pane")),
                        TextBlock::new()
                            .text("Resize the window to cross compact and minimal thresholds."),
                    )),
                ),
                SlotView::new(
                    NavigationViewSlot::PaneFooter,
                    TextBlock::new().text(footer),
                ),
            ])
    }
}

fn main() {
    App::run_component::<ResponsiveNavigation>(()).unwrap();
}
