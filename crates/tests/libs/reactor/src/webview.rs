use super::*;
use std::{
    cell::Cell,
    thread::{self, ThreadId},
};
use windows_webview::{EventRegistration, WebView};

#[derive(PartialEq)]
struct Counts {
    owner: ThreadId,
    messages: Cell<u32>,
    scripts: Cell<u32>,
    pending_called: Cell<bool>,
    drops: Cell<u32>,
}

struct Capture(Rc<Counts>);

impl Capture {
    fn counts(&self) -> &Counts {
        assert_eq!(thread::current().id(), self.0.owner);
        &self.0
    }
}

impl Drop for Capture {
    fn drop(&mut self) {
        let counts = self.counts();
        counts.drops.set(counts.drops.get() + 1);
    }
}

struct Fixture {
    webview: Option<WebView>,
    registration: Option<EventRegistration>,
}

#[derive(Clone)]
enum Message {
    Ready(WebView),
    Page,
    Completed,
}

impl Component for Fixture {
    type Input = Rc<Counts>;
    type Message = Message;

    fn create(_: &Self::Input, _: &ComponentContext<Self>) -> Self {
        Self {
            webview: None,
            registration: None,
        }
    }

    fn update(&mut self, message: Message, context: &ComponentContext<Self>) {
        match message {
            Message::Ready(webview) => {
                assert!(self.webview.is_none());
                let capture = Capture(context.input().clone());
                let page = context.message(Message::Page);
                self.registration = Some(
                    webview
                        .on_web_message_received(move |args| {
                            assert_eq!(args.web_message_as_json(), "\"ready\"");
                            let counts = capture.counts();
                            counts.messages.set(counts.messages.get() + 1);
                            page.call(());
                        })
                        .unwrap(),
                );
                webview
                    .navigate_to_string(
                        "<!DOCTYPE html><script>chrome.webview.postMessage('ready')</script>",
                    )
                    .unwrap();
                self.webview = Some(webview);
            }
            Message::Page => {
                let capture = Capture(context.input().clone());
                let completed = context.message(Message::Completed);
                self.webview
                    .as_ref()
                    .unwrap()
                    .execute_script("6 * 7", move |result| {
                        assert_eq!(result.unwrap(), "42");
                        let counts = capture.counts();
                        counts.scripts.set(counts.scripts.get() + 1);
                        completed.call(());
                    })
                    .unwrap();
            }
            Message::Completed => {
                let capture = Capture(context.input().clone());
                self.webview
                    .as_ref()
                    .unwrap()
                    .execute_script("1 + 2", move |_| {
                        capture.counts().pending_called.set(true);
                    })
                    .unwrap();
                assert!(!context.input().pending_called.get());
                self.registration.take();
                self.webview.take();
                assert!(context.close_window());
            }
        }
    }

    fn view(&self, _: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("Reactor WebView callback ownership");
        context.window_visuals(WindowVisuals::new().client_size(400.0, 300.0));
        windows_webview::webview(context.callback(Message::Ready))
    }
}

pub(super) fn run() -> windows_core::Result<()> {
    let counts = Rc::new(Counts {
        owner: thread::current().id(),
        messages: Cell::new(0),
        scripts: Cell::new(0),
        pending_called: Cell::new(false),
        drops: Cell::new(0),
    });
    App::run_component::<Fixture>(counts.clone())?;
    assert_eq!(counts.messages.get(), 1);
    assert_eq!(counts.scripts.get(), 1);
    assert_eq!(counts.drops.get(), 3);
    Ok(())
}
