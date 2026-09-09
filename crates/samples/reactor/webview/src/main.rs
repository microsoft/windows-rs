#![windows_subsystem = "windows"]

use std::rc::Rc;
use windows_reactor::*;
use windows_webview::{
    EnvironmentOptions, EventRegistration, Result, WebView, WebViewWindow, webview_result,
};

const HOME: &str = "https://learn.microsoft.com/windows/apps/";

#[derive(Clone, Copy)]
enum Host {
    Reactor,
    Window,
}

struct Browser {
    address: String,
    reactor: Option<WebView>,
    reactor_registrations: Vec<EventRegistration>,
    reactor_status: String,
    window: Option<Rc<WebViewWindow>>,
    window_status: String,
}

#[derive(Clone)]
enum Message {
    Address(String),
    Back,
    DevTools,
    Forward,
    Go,
    HostNavigated(Host, String, bool),
    HostStatus(Host, String),
    OpenWindow,
    PagePing,
    ReactorInitialized(std::result::Result<WebView, IntegrationError>),
    Reload,
    Script,
    CloseWindow,
    WindowClosed,
    WindowInitialized(std::result::Result<Rc<WebViewWindow>, String>),
    Zoom(f64),
}

impl Component for Browser {
    type Input = ();
    type Message = Message;

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            address: HOME.to_string(),
            reactor: None,
            reactor_registrations: Vec::new(),
            reactor_status: "creating".to_string(),
            window: None,
            window_status: "waiting for Reactor host".to_string(),
        }
    }

    fn update(&mut self, message: Message, context: &ComponentContext<Self>) {
        match message {
            Message::Address(address) => self.address = address,
            Message::Back => self.each_webview(|_, webview| webview.go_back()),
            Message::DevTools => self.each_webview(|_, webview| webview.open_dev_tools_window()),
            Message::Forward => self.each_webview(|_, webview| webview.go_forward()),
            Message::Go => {
                let address = normalize(&self.address);
                self.each_webview(|_, webview| webview.navigate(&address));
            }
            Message::HostNavigated(host, source, succeeded) => {
                if matches!(host, Host::Reactor) && !source.is_empty() {
                    self.address.clone_from(&source);
                }
                let status = if succeeded {
                    format!("loaded: {source}")
                } else {
                    format!("navigation failed: {source}")
                };
                match host {
                    Host::Reactor => self.reactor_status = status,
                    Host::Window => self.window_status = status,
                }
            }
            Message::HostStatus(host, status) => match host {
                Host::Reactor => self.reactor_status = status,
                Host::Window => self.window_status = status,
            },
            Message::PagePing => self
                .each_webview(|_, webview| webview.post_web_message_as_string("Hello from Rust")),
            Message::OpenWindow => {
                if self.window.is_none() && self.window_status != "creating" {
                    self.window_status = create_window(context.sender());
                }
            }
            Message::ReactorInitialized(result) => match result {
                Ok(webview) => match subscribe(&webview, Host::Reactor, context.sender()) {
                    Ok(registrations) => {
                        self.reactor_registrations = registrations;
                        self.reactor_status = "installing page bridge".to_string();
                        if let Err(error) = initialize(&webview, Host::Reactor, context.sender()) {
                            self.reactor_status = format!("initialization failed: {error}");
                        }
                        self.reactor = Some(webview);
                        self.window_status = create_window(context.sender());
                    }
                    Err(error) => {
                        self.reactor_status = format!("event setup failed: {error}");
                        self.window_status = create_window(context.sender());
                    }
                },
                Err(error) => {
                    self.reactor_status = format!("creation failed: {error:?}");
                    self.window_status = create_window(context.sender());
                }
            },
            Message::Reload => self.each_webview(|_, webview| webview.reload()),
            Message::Script => {
                let sender = context.sender();
                self.each_webview(|host, webview| {
                    let sender = sender.clone();
                    webview.execute_script(
                        r#"document.title = "windows-rs script";
                           window.chrome.webview.postMessage({
                               source: "executeScript",
                               href: location.href
                           });
                           document.title"#,
                        move |result| {
                            _ = sender.send(Message::HostStatus(
                                host,
                                format!("script result: {result:?}"),
                            ));
                        },
                    )
                });
            }
            Message::CloseWindow => {
                if let Some(window) = &self.window {
                    window.window().close();
                }
            }
            Message::WindowClosed => {
                self.window_status = "closed".to_string();
                self.window = None;
            }
            Message::WindowInitialized(result) => match result {
                Ok(window) => match subscribe(window.webview(), Host::Window, context.sender()) {
                    Ok(registrations) => {
                        window.retain_all(registrations);
                        self.window_status = "installing page bridge".to_string();
                        if let Err(error) =
                            initialize(window.webview(), Host::Window, context.sender())
                        {
                            self.window_status = format!("initialization failed: {error}");
                        }
                        self.window = Some(window);
                    }
                    Err(error) => self.window_status = format!("event setup failed: {error}"),
                },
                Err(error) => self.window_status = format!("creation failed: {error}"),
            },
            Message::Zoom(delta) => {
                if let Some(window) = &self.window {
                    let controller = window.controller();
                    _ = controller
                        .set_zoom_factor((controller.zoom_factor() + delta).clamp(0.25, 5.0));
                    self.window_status = format!("zoom: {:.0}%", controller.zoom_factor() * 100.0);
                }
            }
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("WebView2 - Reactor host and shared controls");
        context.window_visuals(
            WindowVisuals::new()
                .backdrop(WindowBackdrop::Mica)
                .client_size(900.0, 760.0),
        );

        let button = |label, message, column| {
            Button::new()
                .on_click(context.message(message))
                .grid_column(column)
                .content(label)
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
                button("Back", Message::Back, 0),
                button("Forward", Message::Forward, 1),
                button("Reload", Message::Reload, 2),
                TextBox::new()
                    .text(&self.address)
                    .placeholder_text("Enter a URL")
                    .on_text_changed(context.callback(Message::Address))
                    .grid_column(3),
                button("Go", Message::Go, 4),
            ));

        let shared_actions = StackPanel::new()
            .orientation(Orientation::Horizontal)
            .spacing(8.0)
            .children((
                Button::new()
                    .on_click(context.message(Message::Script))
                    .content("Script + page message"),
                Button::new()
                    .on_click(context.message(Message::PagePing))
                    .content("Host message"),
                Button::new()
                    .on_click(context.message(Message::DevTools))
                    .content("DevTools"),
            ));

        let raw_actions = StackPanel::new()
            .orientation(Orientation::Horizontal)
            .spacing(8.0)
            .children((
                Button::new()
                    .on_click(context.message(Message::Zoom(-0.25)))
                    .content("Controller zoom -"),
                Button::new()
                    .on_click(context.message(Message::Zoom(0.25)))
                    .content("Controller zoom +"),
                Button::new()
                    .is_enabled(self.window.is_none() && self.window_status != "creating")
                    .on_click(context.message(Message::OpenWindow))
                    .content("Open raw"),
                Button::new()
                    .is_enabled(self.window.is_some())
                    .on_click(context.message(Message::CloseWindow))
                    .content("Close raw"),
            ));

        let actions = StackPanel::new()
            .spacing(4.0)
            .margin(Thickness::uniform(8.0))
            .grid_row(2)
            .children((
                TextBlock::new().text("Shared WebView API - affects both hosts"),
                shared_actions,
                TextBlock::new()
                    .text("windows-window Controller and lifecycle - affects the raw host only"),
                raw_actions,
            ));

        let status = StackPanel::new()
            .spacing(2.0)
            .margin(Thickness::uniform(8.0))
            .grid_row(3)
            .children((
                TextBlock::new().text(format!("Reactor/WinUI: {}", self.reactor_status)),
                TextBlock::new().text(format!("windows-window: {}", self.window_status)),
            ));

        let content = webview_result(context.callback(Message::ReactorInitialized));
        Grid::new()
            .rows([
                GridLength::Auto,
                GridLength::STAR,
                GridLength::Auto,
                GridLength::Auto,
            ])
            .children((
                toolbar,
                Border::new().grid_row(1).content(content),
                actions,
                status,
            ))
    }
}

impl Browser {
    fn each_webview(&mut self, action: impl Fn(Host, &WebView) -> Result<()>) {
        let reactor_error = self
            .reactor
            .as_ref()
            .and_then(|webview| action(Host::Reactor, webview).err());
        let window_error = self
            .window
            .as_ref()
            .and_then(|window| action(Host::Window, window.webview()).err());

        if let Some(error) = reactor_error {
            self.reactor_status = format!("command failed: {error}");
        }
        if let Some(error) = window_error {
            self.window_status = format!("command failed: {error}");
        }
    }
}

fn create_window(sender: LocalSender<Message>) -> String {
    let closed = sender.clone();
    let user_data = std::env::temp_dir().join("windows-rs-reactor-webview-raw");
    let result = WebViewWindow::new("WebView2 - windows-window host")
        .position(980, 80)
        .client_size(900, 760)
        .environment_options(
            EnvironmentOptions::new().user_data_folder(user_data.to_string_lossy()),
        )
        .quit_on_close(false)
        .on_close(move || {
            _ = closed.send(Message::WindowClosed);
        })
        .create(move |result| {
            _ = sender.send(Message::WindowInitialized(
                result.map(Rc::new).map_err(|error| error.to_string()),
            ));
        });

    match result {
        Ok(()) => "creating".to_string(),
        Err(error) => format!("creation failed: {error}"),
    }
}

fn initialize(webview: &WebView, host: Host, sender: LocalSender<Message>) -> Result<()> {
    let ready = webview.clone();
    webview.add_script_to_execute_on_document_created(
        r#"window.chrome.webview.addEventListener("message", event => {
               document.title = "Host message: " + String(event.data);
               window.chrome.webview.postMessage({
                   source: "hostMessage",
                   value: event.data
               });
           });"#,
        move |result| {
            let status = match result {
                Ok(_) => match ready.navigate(HOME) {
                    Ok(()) => "ready".to_string(),
                    Err(error) => format!("initial navigation failed: {error}"),
                },
                Err(error) => format!("page bridge failed: {error}"),
            };
            _ = sender.send(Message::HostStatus(host, status));
        },
    )
}

fn subscribe(
    webview: &WebView,
    host: Host,
    sender: LocalSender<Message>,
) -> Result<Vec<EventRegistration>> {
    let navigated = webview.clone();
    let navigation_sender = sender.clone();
    let title_sender = sender.clone();
    let message_sender = sender.clone();
    let process_sender = sender;

    Ok(vec![
        webview.on_navigation_completed(move |args| {
            _ = navigation_sender.send(Message::HostNavigated(
                host,
                navigated.source(),
                args.is_success(),
            ));
        })?,
        webview.on_document_title_changed(move |title| {
            _ = title_sender.send(Message::HostStatus(host, format!("title: {title}")));
        })?,
        webview.on_web_message_received(move |args| {
            _ = message_sender.send(Message::HostStatus(
                host,
                format!("page message: {}", args.web_message_as_json()),
            ));
        })?,
        webview.on_process_failed(move |args| {
            _ = process_sender.send(Message::HostStatus(
                host,
                format!("process failed: {:?}", args.kind()),
            ));
        })?,
    ])
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
