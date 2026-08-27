#![windows_subsystem = "windows"]

use std::result::Result as StdResult;
use std::time::Duration;

use windows_reactor::*;

#[derive(Clone, Copy, PartialEq)]
enum Page {
    Home,
    Dashboard,
    Settings,
}

impl Page {
    fn tag(self) -> &'static str {
        match self {
            Self::Home => "home",
            Self::Dashboard => "dashboard",
            Self::Settings => "settings",
        }
    }

    fn from_tag(tag: &str) -> Self {
        match tag {
            "dashboard" => Self::Dashboard,
            "settings" => Self::Settings,
            _ => Self::Home,
        }
    }
}

fn fetch_stats() -> StdResult<Vec<String>, String> {
    std::thread::sleep(Duration::from_millis(500));
    Ok(vec![
        "Users online: 1,234".to_string(),
        "CPU usage: 42%".to_string(),
        "Memory: 8.2 / 16 GB".to_string(),
        "Disk: 120 GB free".to_string(),
    ])
}

enum DashboardState {
    Loading,
    Ready(Vec<String>),
    Error(String),
}

struct DashboardPage {
    state: DashboardState,
}

impl Component for DashboardPage {
    type Message = StdResult<Vec<String>, String>;
    type Input = ();

    fn create(_input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        _ = context.spawn_background(|_| fetch_stats());
        Self {
            state: DashboardState::Loading,
        }
    }

    fn update(&mut self, result: Self::Message, _context: &ComponentContext<Self>) {
        self.state = match result {
            Ok(stats) => DashboardState::Ready(stats),
            Err(error) => DashboardState::Error(error),
        };
    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        let content: View = match &self.state {
            DashboardState::Loading => ProgressRing::new().is_indeterminate(true).into(),
            DashboardState::Ready(stats) => StackPanel::new().spacing(4.0).keyed_children(
                stats
                    .iter()
                    .map(|stat| KeyedView::new(stat.clone(), stat.as_str())),
            ),
            DashboardState::Error(error) => format!("Error: {error}").into(),
        };
        StackPanel::new().spacing(8.0).children((
            TextBlock::new()
                .text("Dashboard")
                .font_size(28.0)
                .font_weight(700),
            "Live stats (loaded in a component task):",
            content,
        ))
    }
}

struct SettingsPage {
    dark_mode: bool,
    notifications: bool,
}

enum SettingsMessage {
    DarkMode(bool),
    Notifications(bool),
}

impl Component for SettingsPage {
    type Message = SettingsMessage;
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            dark_mode: false,
            notifications: true,
        }
    }

    fn update(&mut self, message: Self::Message, _context: &ComponentContext<Self>) {
        match message {
            SettingsMessage::DarkMode(value) => self.dark_mode = value,
            SettingsMessage::Notifications(value) => self.notifications = value,
        }
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        StackPanel::new().spacing(12.0).children((
            TextBlock::new()
                .text("Settings")
                .font_size(28.0)
                .font_weight(700),
            ToggleSwitch::new()
                .is_on(self.dark_mode)
                .on_toggled(context.callback(SettingsMessage::DarkMode))
                .slots([SlotView::new(ToggleSwitchSlot::Header, "Dark mode")]),
            ToggleSwitch::new()
                .is_on(self.notifications)
                .on_toggled(context.callback(SettingsMessage::Notifications))
                .slots([SlotView::new(ToggleSwitchSlot::Header, "Notifications")]),
            TextBlock::new()
                .text(format!(
                    "Dark: {} | Notifications: {}",
                    if self.dark_mode { "on" } else { "off" },
                    if self.notifications { "on" } else { "off" }
                ))
                .opacity(0.6),
        ))
    }
}

struct NavigationSample {
    page: Page,
}

impl Component for NavigationSample {
    type Message = Option<String>;
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self { page: Page::Home }
    }

    fn update(&mut self, tag: Self::Message, _context: &ComponentContext<Self>) {
        if let Some(tag) = tag {
            self.page = Page::from_tag(&tag);
        }
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("Navigation");
        let item = |page: Page, label, symbol| {
            KeyedView::new(
                page.tag(),
                NavigationViewItem::new()
                    .tag(page.tag())
                    .is_selected(self.page == page)
                    .slots([
                        SlotView::new(NavigationViewItemSlot::Content, label),
                        SlotView::new(
                            NavigationViewItemSlot::Icon,
                            SymbolIcon::new().symbol(symbol),
                        ),
                    ]),
            )
        };
        let body = match self.page {
            Page::Home => StackPanel::new().spacing(8.0).children((
                TextBlock::new()
                    .text("Welcome Home")
                    .font_size(28.0)
                    .font_weight(700),
                "This is the landing page of the app.",
                TextBlock::new()
                    .text("Use the navigation pane to switch between pages.")
                    .opacity(0.6),
            )),
            Page::Dashboard => View::component::<DashboardPage>(()),
            Page::Settings => View::component::<SettingsPage>(()),
        };

        NavigationView::new()
            .pane_display_mode(NavigationViewPaneDisplayMode::Left)
            .pane_title("My App")
            .is_settings_visible(false)
            .on_selected_tag_changed(context.callback(std::convert::identity))
            .slots([
                SlotView::collection(
                    NavigationViewSlot::MenuItems,
                    [
                        item(Page::Home, "Home", Symbol::Home),
                        item(Page::Dashboard, "Dashboard", Symbol::World),
                        item(Page::Settings, "Settings", Symbol::Setting),
                    ],
                ),
                SlotView::new(NavigationViewSlot::Content, body),
            ])
    }
}

fn main() {
    App::run_component::<NavigationSample>(()).unwrap();
}
