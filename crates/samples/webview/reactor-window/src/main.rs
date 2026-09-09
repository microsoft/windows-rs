#![windows_subsystem = "windows"]

use std::rc::Rc;
use windows_reactor::*;
use windows_webview::{Result, WebView, WebViewWindow};

const HOME: &str = "https://learn.microsoft.com/windows/apps/";

struct BrowserControls {
    address: String,
    generation: u64,
    active_generation: Option<u64>,
    window: Option<Rc<WebViewWindow>>,
    status: String,
}

#[derive(Clone)]
enum Message {
    Address(String),
    Back,
    Close,
    Forward,
    Go,
    Open,
    Reload,
    WindowClosed(u64),
    WindowInitialized(u64, std::result::Result<Rc<WebViewWindow>, String>),
    Zoom(f64),
}

impl Component for BrowserControls {
    type Input = ();
    type Message = Message;

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            address: HOME.to_string(),
            generation: 0,
            active_generation: None,
            window: None,
            status: "waiting to open browser".to_string(),
        }
    }

    fn update(&mut self, message: Message, context: &ComponentContext<Self>) {
        match message {
            Message::Address(address) => self.address = address,
            Message::Back => self.command(WebView::go_back),
            Message::Close => {
                if let Some(window) = &self.window {
                    window.window().close();
                }
            }
            Message::Forward => self.command(WebView::go_forward),
            Message::Go => {
                let address = normalize(&self.address);
                self.command(|webview| webview.navigate(&address));
            }
            Message::Open => {
                if self.active_generation.is_none() {
                    self.generation += 1;
                    self.active_generation = Some(self.generation);
                    match create_window(self.generation, context.sender()) {
                        Ok(()) => self.status = "creating".to_string(),
                        Err(error) => {
                            self.active_generation = None;
                            self.status = format!("creation failed: {error}");
                        }
                    }
                }
            }
            Message::Reload => self.command(WebView::reload),
            Message::WindowClosed(generation)
                if self.active_generation == Some(generation) && self.window.is_some() =>
            {
                self.active_generation = None;
                self.window = None;
                self.status = "closed".to_string();
            }
            Message::WindowInitialized(generation, result)
                if self.active_generation == Some(generation) =>
            {
                match result {
                    Ok(window) => {
                        self.status = match window.webview().navigate(HOME) {
                            Ok(()) => "ready".to_string(),
                            Err(error) => format!("initial navigation failed: {error}"),
                        };
                        self.window = Some(window);
                    }
                    Err(error) => {
                        self.active_generation = None;
                        self.status = format!("creation failed: {error}");
                    }
                }
            }
            Message::Zoom(delta) => {
                if let Some(window) = &self.window {
                    let controller = window.controller();
                    _ = controller
                        .set_zoom_factor((controller.zoom_factor() + delta).clamp(0.25, 5.0));
                    self.status = format!("zoom: {:.0}%", controller.zoom_factor() * 100.0);
                }
            }
            Message::WindowClosed(_) | Message::WindowInitialized(_, _) => {}
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Reactor controls for a windows-window WebView");
        context.window_visuals(
            WindowVisuals::new()
                .backdrop(WindowBackdrop::Mica)
                .client_size(720.0, 210.0),
        );

        let sender = context.sender();
        context.use_effect("open-browser", (), move || {
            _ = sender.send(Message::Open);
            None
        });

        let navigation = Grid::new()
            .columns([
                GridLength::Auto,
                GridLength::Auto,
                GridLength::Auto,
                GridLength::STAR,
                GridLength::Auto,
            ])
            .column_spacing(8.0)
            .children((
                grid_button("Back", Message::Back, 0, context),
                grid_button("Forward", Message::Forward, 1, context),
                grid_button("Reload", Message::Reload, 2, context),
                TextBox::new()
                    .text(&self.address)
                    .placeholder_text("Enter a URL")
                    .on_text_changed(context.callback(Message::Address))
                    .grid_column(3),
                grid_button("Go", Message::Go, 4, context),
            ));
        let host_controls = StackPanel::new()
            .orientation(Orientation::Horizontal)
            .spacing(8.0)
            .children((
                button("Zoom -", Message::Zoom(-0.25), context),
                button("Zoom +", Message::Zoom(0.25), context),
                Button::new()
                    .is_enabled(self.active_generation.is_none())
                    .on_click(context.message(Message::Open))
                    .content("Open browser"),
                Button::new()
                    .is_enabled(self.window.is_some())
                    .on_click(context.message(Message::Close))
                    .content("Close browser"),
            ));

        StackPanel::new()
            .spacing(12.0)
            .margin(Thickness::uniform(12.0))
            .children((
                TextBlock::new().text(
                    "This Reactor window controls a WebView hosted by a separate windows-window.",
                ),
                navigation,
                host_controls,
                TextBlock::new().text(&self.status),
            ))
    }
}

impl BrowserControls {
    fn command(&mut self, action: impl FnOnce(&WebView) -> Result<()>) {
        if let Some(window) = &self.window
            && let Err(error) = action(window.webview())
        {
            self.status = format!("command failed: {error}");
        }
    }
}

fn button(
    label: impl Into<View>,
    message: Message,
    context: &ViewContext<BrowserControls>,
) -> View {
    Button::new()
        .on_click(context.message(message))
        .content(label)
}

fn grid_button(
    label: impl Into<View>,
    message: Message,
    column: i32,
    context: &ViewContext<BrowserControls>,
) -> View {
    Button::new()
        .on_click(context.message(message))
        .grid_column(column)
        .content(label)
}

fn create_window(generation: u64, sender: LocalSender<Message>) -> Result<()> {
    let closed = sender.clone();
    WebViewWindow::new("WebView2 - windows-window host")
        .client_size(900, 700)
        .quit_on_close(false)
        .on_close(move || {
            _ = closed.send(Message::WindowClosed(generation));
        })
        .create(move |result| {
            _ = sender.send(Message::WindowInitialized(
                generation,
                result.map(Rc::new).map_err(|error| error.to_string()),
            ));
        })
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
    App::run_component::<BrowserControls>(())
}
