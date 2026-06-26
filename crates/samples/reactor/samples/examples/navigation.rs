//! Multi-page navigation using enum-based routing.

use std::thread;
use std::time::Duration;

use windows_reactor::*;

#[derive(Clone, PartialEq)]
enum Page {
    Home,
    Dashboard,
    Settings,
}

impl Page {
    fn tag(&self) -> &'static str {
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

fn home_page(_: &(), _cx: &mut RenderCx) -> Element {
    vstack((
        text_block("Welcome Home").font_size(28.0).bold(),
        text_block("This is the landing page of the app."),
        text_block("Use the navigation pane to switch between pages.").opacity(0.6),
    ))
    .spacing(8.0)
    .into()
}

fn fetch_stats(_: ()) -> std::result::Result<Vec<String>, String> {
    thread::sleep(Duration::from_millis(500));
    Ok(vec![
        "Users online: 1,234".to_string(),
        "CPU usage: 42%".to_string(),
        "Memory: 8.2 / 16 GB".to_string(),
        "Disk: 120 GB free".to_string(),
    ])
}

fn dashboard_page(_: &(), cx: &mut RenderCx) -> Element {
    let stats = cx.use_resource(fetch_stats, ());

    let content: Element = stats
        .view(|data| {
            vstack(
                data.iter()
                    .map(|s| text_block(s).into())
                    .collect::<Vec<Element>>(),
            )
            .spacing(4.0)
        })
        .into();

    vstack((
        text_block("Dashboard").font_size(28.0).bold(),
        text_block("Live stats (loaded via use_resource):"),
        content,
    ))
    .spacing(8.0)
    .into()
}

fn settings_page(_: &(), cx: &mut RenderCx) -> Element {
    let (dark_mode, set_dark) = cx.use_state(false);
    let (notifications, set_notif) = cx.use_state(true);

    vstack((
        text_block("Settings").font_size(28.0).bold(),
        ToggleSwitch::new(dark_mode)
            .header("Dark mode")
            .on_toggled(set_dark),
        ToggleSwitch::new(notifications)
            .header("Notifications")
            .on_toggled(set_notif),
        text_block(format!(
            "Dark: {} | Notifications: {}",
            if dark_mode { "on" } else { "off" },
            if notifications { "on" } else { "off" }
        ))
        .opacity(0.6),
    ))
    .spacing(12.0)
    .into()
}

fn app(cx: &mut RenderCx) -> Element {
    let (page, set_page) = cx.use_state(Page::Home);

    let menu_items = [
        NavViewItem::new("Home").tag("home").icon(Symbol::Home),
        NavViewItem::new("Dashboard")
            .tag("dashboard")
            .icon(Symbol::World),
        NavViewItem::new("Settings")
            .tag("settings")
            .icon(Symbol::Setting),
    ];

    let body: Element = match &page {
        Page::Home => component(home_page, ()),
        Page::Dashboard => component(dashboard_page, ()),
        Page::Settings => component(settings_page, ()),
    };

    NavigationView::new(menu_items, body)
        .selected_tag(page.tag())
        .on_selection_changed(move |tag: String| set_page.call(Page::from_tag(&tag)))
        .pane_display_mode(NavigationViewPaneDisplayMode::Left)
        .pane_title("My App")
        .into()
}

fn main() -> Result<()> {
    reactor_samples::run("Navigation", app)
}
