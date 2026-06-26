//! A minimal browser: a `windows-reactor` toolbar driving a WebView2.
//!
//! The reactor owns the WinUI XAML `WebView2` control; `windows-webview`'s
//! `reactor` feature bridges it to the same COM `ICoreWebView2` surface used by
//! the standalone crate, so the [`WebView`] handed to `on_ready` drives
//! navigation just like any other host. Here an address bar plus Back / Forward
//! / Reload / Go buttons — ordinary reactor controls — interact with it, and the
//! address bar reflects link clicks back via the navigation-completed event.

#![windows_subsystem = "windows"]

use windows_reactor::*;
use windows_webview::{EventRegistration, WebView, webview};

const HOME: &str = "https://learn.microsoft.com/windows/apps/";

fn app(cx: &mut RenderCx) -> Element {
    let (address, set_address) = cx.use_state(String::from(HOME));
    let web = cx.use_ref::<Option<WebView>>(None);
    let registration = cx.use_ref::<Option<EventRegistration>>(None);

    let on_ready = {
        let web = web.clone();
        let set_address = set_address.clone();
        move |ready: WebView| {
            let reflect = {
                let ready = ready.clone();
                let set_address = set_address.clone();
                move |_args| {
                    let source = ready.source();
                    if !source.is_empty() {
                        set_address.call(source);
                    }
                }
            };
            *registration.borrow_mut() = ready.on_navigation_completed(reflect).ok();
            let _ = ready.navigate(HOME);
            *web.borrow_mut() = Some(ready);
        }
    };

    let go = {
        let web = web.clone();
        let address = address.clone();
        move || {
            if let Some(web) = web.borrow().as_ref() {
                let _ = web.navigate(&normalize(&address));
            }
        }
    };
    let back = with_web(&web, WebView::go_back);
    let forward = with_web(&web, WebView::go_forward);
    let reload = with_web(&web, WebView::reload);

    let toolbar = grid((
        Element::from(button("\u{2190}").on_click(back)).grid_column(0),
        Element::from(button("\u{2192}").on_click(forward)).grid_column(1),
        Element::from(button("\u{21BB}").on_click(reload)).grid_column(2),
        Element::from(
            text_box(address)
                .placeholder_text("Enter a URL")
                .on_text_changed(set_address),
        )
        .grid_column(3),
        Element::from(button("Go").on_click(go)).grid_column(4),
    ))
    .columns([
        GridLength::Auto,
        GridLength::Auto,
        GridLength::Auto,
        GridLength::STAR,
        GridLength::Auto,
    ])
    .column_spacing(8.0)
    .margin(Thickness::uniform(8.0));

    grid((
        Element::from(toolbar).grid_row(0),
        Element::from(webview(on_ready)).grid_row(1),
    ))
    .rows([GridLength::Auto, GridLength::STAR])
    .into()
}

/// Turns a user-typed address into a navigable absolute URI. WebView2's
/// `Navigate` rejects a bare host such as `example.com`, so a missing scheme is
/// filled in with `https://`.
fn normalize(address: &str) -> String {
    let address = address.trim();
    if address.contains("://") {
        address.to_string()
    } else {
        format!("https://{address}")
    }
}

/// Builds a button click handler that runs `action` against the ready WebView.
fn with_web(
    web: &HookRef<Option<WebView>>,
    action: fn(&WebView) -> Result<()>,
) -> impl Fn() + 'static {
    let web = web.clone();
    move || {
        if let Some(web) = web.borrow().as_ref() {
            let _ = action(web);
        }
    }
}

fn main() -> Result<()> {
    App::new()
        .title("WebView2")
        .backdrop(Backdrop::Mica)
        .render(app)
}
