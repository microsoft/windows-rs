#![windows_subsystem = "windows"]

use windows_reactor::*;
use windows_webview::{EventRegistration, Result, WebView, webview_result};

const HOME: &str = "https://learn.microsoft.com/windows/apps/";

struct Browser {
    address: String,
    webview: Option<WebView>,
    registrations: Vec<EventRegistration>,
    status: String,
}

#[derive(Clone)]
enum Message {
    Address(String),
    Back,
    DevTools,
    Forward,
    Go,
    SendToPage,
    Initialized(std::result::Result<WebView, IntegrationError>),
    Navigated(String, bool),
    ReceivedFromPage(String),
    Reload,
    Script,
    Status(String),
}

impl Component for Browser {
    type Input = ();
    type Message = Message;

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            address: HOME.to_string(),
            webview: None,
            registrations: Vec::new(),
            status: "creating".to_string(),
        }
    }

    fn update(&mut self, message: Message, context: &ComponentContext<Self>) {
        match message {
            Message::Address(address) => self.address = address,
            Message::Back => self.command(WebView::go_back),
            Message::DevTools => self.command(WebView::open_dev_tools_window),
            Message::Forward => self.command(WebView::go_forward),
            Message::Go => {
                let address = normalize(&self.address);
                self.command(|webview| webview.navigate(&address));
            }
            Message::SendToPage => {
                self.command(|webview| webview.post_web_message_as_string("Hello from Rust"));
            }
            Message::Initialized(result) => match result {
                Ok(webview) => match subscribe(&webview, context.sender()) {
                    Ok(registrations) => {
                        self.registrations = registrations;
                        self.status = "installing page bridge".to_string();
                        if let Err(error) = initialize(&webview, context.sender()) {
                            self.status = format!("initialization failed: {error}");
                        }
                        self.webview = Some(webview);
                    }
                    Err(error) => self.status = format!("event setup failed: {error}"),
                },
                Err(error) => self.status = format!("creation failed: {error:?}"),
            },
            Message::Navigated(source, succeeded) => {
                if !source.is_empty() {
                    self.address.clone_from(&source);
                }
                self.status = if succeeded {
                    format!("loaded: {source}")
                } else {
                    format!("navigation failed: {source}")
                };
            }
            Message::ReceivedFromPage(message) => {
                self.status = format!("page message: {message}");
            }
            Message::Reload => self.command(WebView::reload),
            Message::Script => {
                if let Some(webview) = &self.webview {
                    let sender = context.sender();
                    if let Err(error) = webview.execute_script(
                        r#"document.title = "windows-rs script";
                           window.chrome.webview.postMessage(location.href);
                           document.title"#,
                        move |result| {
                            _ = sender.send(Message::Status(format!("script result: {result:?}")));
                        },
                    ) {
                        self.status = format!("command failed: {error}");
                    }
                }
            }
            Message::Status(status) => self.status = status,
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("WebView2 - Reactor host");
        context.window_visuals(
            WindowVisuals::new()
                .backdrop(WindowBackdrop::Mica)
                .client_size(900.0, 700.0),
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
        let actions = StackPanel::new()
            .orientation(Orientation::Horizontal)
            .spacing(8.0)
            .margin(Thickness::uniform(8.0))
            .grid_row(2)
            .children((
                Button::new()
                    .on_click(context.message(Message::Script))
                    .content("Run script"),
                Button::new()
                    .on_click(context.message(Message::SendToPage))
                    .content("Post message"),
                Button::new()
                    .on_click(context.message(Message::DevTools))
                    .content("DevTools"),
            ));
        let status = TextBlock::new()
            .margin(Thickness::uniform(8.0))
            .grid_row(3)
            .text(&self.status);

        Grid::new()
            .rows([
                GridLength::Auto,
                GridLength::STAR,
                GridLength::Auto,
                GridLength::Auto,
            ])
            .children((
                toolbar,
                Border::new()
                    .grid_row(1)
                    .content(webview_result(context.callback(Message::Initialized))),
                actions,
                status,
            ))
    }
}

impl Browser {
    fn command(&mut self, action: impl FnOnce(&WebView) -> Result<()>) {
        if let Some(webview) = &self.webview
            && let Err(error) = action(webview)
        {
            self.status = format!("command failed: {error}");
        }
    }
}

fn initialize(webview: &WebView, sender: LocalSender<Message>) -> Result<()> {
    let ready = webview.clone();
    webview.add_script_to_execute_on_document_created(
        r#"window.chrome.webview.addEventListener("message", event => {
               document.title = "Host message: " + String(event.data);
               window.chrome.webview.postMessage(event.data);
           });"#,
        move |result| {
            let status = match result {
                Ok(_) => match ready.navigate(HOME) {
                    Ok(()) => "ready".to_string(),
                    Err(error) => format!("initial navigation failed: {error}"),
                },
                Err(error) => format!("page bridge failed: {error}"),
            };
            _ = sender.send(Message::Status(status));
        },
    )
}

fn subscribe(webview: &WebView, sender: LocalSender<Message>) -> Result<Vec<EventRegistration>> {
    let navigated = webview.clone();
    let navigation_sender = sender.clone();
    Ok(vec![
        webview.on_navigation_completed(move |args| {
            _ = navigation_sender.send(Message::Navigated(navigated.source(), args.is_success()));
        })?,
        webview.on_web_message_received(move |args| {
            _ = sender.send(Message::ReceivedFromPage(args.web_message_as_json()));
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
