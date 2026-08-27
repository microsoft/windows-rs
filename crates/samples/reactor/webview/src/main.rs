#![windows_subsystem = "windows"]

use windows_reactor::*;
use windows_webview::{EventRegistration, Result, WebView, webview};

const HOME: &str = "https://learn.microsoft.com/windows/apps/";

struct Browser {
    address: String,
    webview: Option<WebView>,
    registration: Option<EventRegistration>,
}

#[derive(Clone)]
enum Message {
    Address(String),
    Back,
    Forward,
    Go,
    Initialized(WebView),
    Reload,
}

impl Component for Browser {
    type Input = ();
    type Message = Message;

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            address: HOME.to_string(),
            webview: None,
            registration: None,
        }
    }

    fn update(&mut self, message: Message, context: &ComponentContext<Self>) {
        match message {
            Message::Address(address) => self.address = address,
            Message::Back => self.with_web(WebView::go_back),
            Message::Forward => self.with_web(WebView::go_forward),
            Message::Go => {
                let address = normalize(&self.address);
                self.with_web(|webview| webview.navigate(&address));
            }
            Message::Initialized(webview) => {
                let sender = context.sender();
                let ready = webview.clone();
                self.registration = webview
                    .on_navigation_completed(move |_| {
                        let source = ready.source();
                        if !source.is_empty() {
                            _ = sender.send(Message::Address(source));
                        }
                    })
                    .ok();
                _ = webview.navigate(HOME);
                self.webview = Some(webview);
            }
            Message::Reload => self.with_web(WebView::reload),
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("WebView2");
        context.window_visuals(WindowVisuals::new().backdrop(WindowBackdrop::Mica));

        let button = |label, message, column| {
            Button::new()
                .on_click(context.message(message))
                .grid_column(column)
                .content(TextBlock::new().text(label))
        };
        let toolbar = Grid::new()
            .columns([
                GridLength::Auto,
                GridLength::Auto,
                GridLength::Auto,
                GridLength::STAR,
                GridLength::Auto,
            ])
            .column_spacing(8.0)
            .margin(Thickness::uniform(8.0))
            .grid_row(0)
            .children((
                button("\u{2190}", Message::Back, 0),
                button("\u{2192}", Message::Forward, 1),
                button("\u{21BB}", Message::Reload, 2),
                TextBox::new()
                    .text(&self.address)
                    .placeholder_text("Enter a URL")
                    .on_text_changed(context.callback(Message::Address))
                    .grid_column(3),
                button("Go", Message::Go, 4),
            ));

        let content = webview(context.callback(Message::Initialized));
        Grid::new()
            .rows([GridLength::Auto, GridLength::STAR])
            .children((toolbar, Border::new().grid_row(1).content(content)))
    }
}

impl Browser {
    fn with_web(&self, action: impl FnOnce(&WebView) -> Result<()>) {
        if let Some(webview) = &self.webview {
            _ = action(webview);
        }
    }
}

fn normalize(address: &str) -> String {
    let address = address.trim();
    if address.contains("://") {
        address.to_string()
    } else {
        format!("https://{address}")
    }
}

fn main() -> Result<()> {
    App::run_component::<Browser>(())
}
