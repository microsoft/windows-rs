#![windows_subsystem = "windows"]

use std::thread;
use std::time::Duration;

use windows_reactor::{
    CancellationToken, Element, FontWeight, Icon, IconSymbol, NavigationItem,
    NavigationPaneDisplayMode, NavigationView, RenderCx, Resource, TextBlock, ToggleSwitch,
    component, vstack,
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Page {
    Home,
    Dashboard,
    Settings,
}

impl Page {
    fn key(self) -> u64 {
        match self {
            Self::Home => 0,
            Self::Dashboard => 1,
            Self::Settings => 2,
        }
    }

    fn from_key(key: u64) -> Self {
        match key {
            1 => Self::Dashboard,
            2 => Self::Settings,
            _ => Self::Home,
        }
    }
}

fn heading(value: impl Into<String>) -> Element {
    TextBlock::new(value)
        .font_size(28.0)
        .font_weight(FontWeight::BOLD)
        .build()
}

fn home_page(_cx: &mut RenderCx<'_>) -> Element {
    vstack(
        8.0,
        [
            heading("Welcome Home"),
            TextBlock::new("This is the landing page of the app.").build(),
            TextBlock::new("Use the navigation pane to switch between pages.")
                .opacity(0.6)
                .build(),
        ],
    )
}

fn fetch_stats(cancel: CancellationToken, _key: ()) -> windows_core::Result<Vec<&'static str>> {
    for _ in 0..20 {
        if cancel.is_cancelled() {
            return Err(windows_core::Error::new(
                windows_core::HRESULT(0x80004004_u32 as i32),
                "dashboard load cancelled",
            ));
        }
        thread::sleep(Duration::from_millis(25));
    }
    Ok(vec![
        "Users online: 1,234",
        "CPU usage: 42%",
        "Memory: 8.2 / 16 GB",
        "Disk: 120 GB free",
    ])
}

fn dashboard_page(cx: &mut RenderCx<'_>) -> Element {
    let stats = cx.use_resource((), fetch_stats);
    let content = match stats {
        Resource::Loading => TextBlock::new("Loading dashboard...").build(),
        Resource::Ready(stats) => vstack(
            4.0,
            stats
                .iter()
                .map(|stat| TextBlock::new(*stat).build())
                .collect::<Vec<_>>(),
        ),
        Resource::Failed(error) => TextBlock::new(format!("Error: {error}")).build(),
    };

    vstack(
        8.0,
        [
            heading("Dashboard"),
            TextBlock::new("Live stats (loaded via use_resource):").build(),
            content,
        ],
    )
}

fn settings_page(cx: &mut RenderCx<'_>) -> Element {
    let dark_mode = cx.use_state(|| false);
    let notifications = cx.use_state(|| true);
    let dark = dark_mode.value();
    let notifying = notifications.value();

    vstack(
        12.0,
        [
            heading("Settings"),
            ToggleSwitch::new(dark, move |value| {
                dark_mode.set(value);
            })
            .header("Dark mode")
            .build(),
            ToggleSwitch::new(notifying, move |value| {
                notifications.set(value);
            })
            .header("Notifications")
            .build(),
            TextBlock::new(format!(
                "Dark: {} | Notifications: {}",
                if dark { "on" } else { "off" },
                if notifying { "on" } else { "off" }
            ))
            .opacity(0.6)
            .build(),
        ],
    )
}

fn app(cx: &mut RenderCx<'_>) -> Element {
    let page = cx.use_state(|| Page::Home);
    let current = page.value();
    let content = match current {
        Page::Home => component(home_page),
        Page::Dashboard => component(dashboard_page),
        Page::Settings => component(settings_page),
    };

    NavigationView::new(
        [
            NavigationItem::new(Page::Home.key(), "Home").icon(Icon::symbol(IconSymbol::HOME)),
            NavigationItem::new(Page::Dashboard.key(), "Dashboard")
                .icon(Icon::symbol(IconSymbol::WORLD)),
            NavigationItem::new(Page::Settings.key(), "Settings")
                .icon(Icon::symbol(IconSymbol::SETTINGS)),
        ],
        content,
        move |key| {
            if let Some(key) = key {
                page.set(Page::from_key(key));
            }
        },
    )
    .selected_key(Some(current.key()))
    .pane_display_mode(NavigationPaneDisplayMode::Left)
    .pane_title("My App")
    .settings_visible(false)
    .build()
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("Navigation", app)
}
