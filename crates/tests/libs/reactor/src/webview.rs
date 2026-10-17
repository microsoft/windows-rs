use super::*;
use std::time::Instant;
use windows_webview::{EventRegistration, WebView, webview};

const PAGE: &str = r#"<!DOCTYPE html>
<html>
<body>
<p>Reactor WebView fixture</p>
<script>
let count = 0;
window.chrome.webview.addEventListener("message", event => {
    window.chrome.webview.postMessage(`echo:${++count}:${event.data}`);
});
window.chrome.webview.postMessage("ready");
</script>
</body>
</html>"#;

pub(super) struct WebViewFixture {
    attached: bool,
    generation: u32,
    messages: u32,
    received: String,
    released: Vec<u32>,
}

#[derive(Clone)]
pub(super) enum Message {
    Remove,
    Attach,
    Received(u32, String),
    Released(u32),
}

impl Component for WebViewFixture {
    type Input = ();
    type Message = Message;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            attached: true,
            generation: 0,
            messages: 0,
            received: String::new(),
            released: Vec::new(),
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Remove => self.attached = false,
            Message::Attach => {
                self.generation += 1;
                self.attached = true;
            }
            Message::Received(generation, text) => {
                self.messages += 1;
                self.received = format!("Browser {generation}: {text}; messages {}", self.messages);
            }
            Message::Released(generation) => self.released.push(generation),
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        let browser = self.attached.then(|| {
            keyed(
                self.generation,
                View::component::<Browser>((self.generation, context.callback(|message| message))),
            )
        });
        context.window_title("Reactor UI fixture: webview");
        context.window_visuals(WindowVisuals::new().client_size(600.0, 500.0));
        StackPanel::new()
            .margin(20.0)
            .spacing(8.0)
            .children((
                Button::new()
                    .automation_id("remove")
                    .content("Remove browser")
                    .on_click(context.message(Message::Remove)),
                Button::new()
                    .automation_id("attach")
                    .content("Attach browser")
                    .on_click(context.message(Message::Attach)),
                TextBlock::new()
                    .automation_id("received")
                    .text(&self.received),
                TextBlock::new()
                    .automation_id("released")
                    .text(format!("Released {:?}", self.released)),
                StackPanel::new().keyed_children(browser),
            ))
            .into()
    }
}

struct Browser {
    generation: u32,
    report: Callback<Message>,
    core: Option<WebView>,
    _subscription: Option<EventRegistration>,
    _navigation: Option<EventRegistration>,
    started: Instant,
}

#[derive(Clone)]
enum BrowserMessage {
    Ready(WebView),
    Send,
}

struct MessageLifetime {
    generation: u32,
    report: Callback<Message>,
}

impl Drop for MessageLifetime {
    fn drop(&mut self) {
        self.report.call(Message::Released(self.generation));
    }
}

impl Component for Browser {
    type Input = (u32, Callback<Message>);
    type Message = BrowserMessage;

    fn create(input: &Self::Input, _: &ComponentContext<Self>) -> Self {
        eprintln!("WebView {}: waiting for initialization", input.0);
        Self {
            generation: input.0,
            report: input.1.clone(),
            core: None,
            _subscription: None,
            _navigation: None,
            started: Instant::now(),
        }
    }

    fn update(&mut self, message: BrowserMessage, _: &ComponentContext<Self>) {
        match message {
            BrowserMessage::Ready(core) => {
                eprintln!(
                    "WebView {}: initialized after {:?}",
                    self.generation,
                    self.started.elapsed()
                );
                assert!(self.core.is_none(), "browser initialized more than once");
                let generation = self.generation;
                let started = self.started;
                self._navigation = Some(
                    core.on_navigation_completed(move |args| {
                        eprintln!(
                            "WebView {generation}: navigation completed after {:?}, success {}",
                            started.elapsed(),
                            args.is_success()
                        );
                    })
                    .unwrap(),
                );
                let lifetime = MessageLifetime {
                    generation: self.generation,
                    report: self.report.clone(),
                };
                self._subscription = Some(
                    core.on_web_message_received(move |args| {
                        let message = args.try_web_message_as_string().unwrap();
                        eprintln!(
                            "WebView {generation}: message {message:?} after {:?}",
                            started.elapsed(),
                        );
                        assert_eq!(args.source(), "about:blank");
                        lifetime
                            .report
                            .call(Message::Received(lifetime.generation, message));
                    })
                    .unwrap(),
                );
                core.navigate_to_string(PAGE).unwrap();
                self.core = Some(core);
            }
            BrowserMessage::Send => {
                self.core
                    .as_ref()
                    .unwrap()
                    .post_web_message_as_string("ping")
                    .unwrap();
            }
        }
    }

    fn view(&self, _: &Self::Input, context: &mut ViewContext<Self>) -> View {
        StackPanel::new()
            .spacing(8.0)
            .children((
                Button::new()
                    .automation_id("send")
                    .content("Send to page")
                    .is_enabled(self.core.is_some())
                    .on_click(context.message(BrowserMessage::Send)),
                Border::new()
                    .height(240.0)
                    .content(webview(context.callback(BrowserMessage::Ready))),
            ))
            .into()
    }
}
