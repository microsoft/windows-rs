use std::time::Duration;
use windows_reactor::{
    App, Border, Component, ComponentContext, ComponentTimer, IntegrationError, View, ViewContext,
};
use windows_webview::{WebView, webview_result};

struct Fixture {
    timeout: Option<ComponentTimer>,
}

enum Message {
    Ready(Result<WebView, IntegrationError>),
    Timeout,
}

impl Component for Fixture {
    type Input = ();
    type Message = Message;

    fn create(_input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        Self {
            timeout: Some(context.set_local_timeout(Duration::from_secs(30), || Message::Timeout)),
        }
    }

    fn update(&mut self, message: Self::Message, context: &ComponentContext<Self>) {
        match message {
            Message::Ready(result) => {
                result.unwrap();
                self.timeout = None;
                assert!(context.close_window());
            }
            Message::Timeout => {
                eprintln!("windows-webview Reactor integration did not initialize");
                std::process::exit(1);
            }
        }
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        Border::new()
            .content(webview_result(context.callback(Message::Ready)))
            .into()
    }
}

fn main() -> windows_core::Result<()> {
    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_secs(45));
        eprintln!("windows-webview Reactor integration timed out");
        std::process::exit(1);
    });
    App::run_component::<Fixture>(())
}
