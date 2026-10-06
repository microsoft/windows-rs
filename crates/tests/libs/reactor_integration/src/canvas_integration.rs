use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;
use windows_canvas::{Canvas, ColorF};
use windows_reactor::{
    App, Border, Component, ComponentContext, ComponentTimer, View, ViewContext,
};

struct Fixture {
    drawn: Rc<Cell<bool>>,
    timeout: Option<ComponentTimer>,
}

enum Message {
    Drawn,
    Timeout,
}

impl Component for Fixture {
    type Input = ();
    type Message = Message;

    fn create(_input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        Self {
            drawn: Rc::new(Cell::new(false)),
            timeout: Some(context.set_timeout(Duration::from_secs(10), Message::Timeout)),
        }
    }

    fn update(&mut self, message: Self::Message, context: &ComponentContext<Self>) {
        match message {
            Message::Drawn => {
                self.timeout = None;
                assert!(context.close_window());
            }
            Message::Timeout => {
                eprintln!("windows-canvas Reactor integration did not draw");
                std::process::exit(1);
            }
        }
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let drawn = Rc::clone(&self.drawn);
        let sender = context.sender();
        Border::new()
            .content(Canvas::animated(move |drawing| {
                drawing.clear(ColorF::TRANSPARENT);
                if !drawn.replace(true) {
                    _ = sender.send(Message::Drawn);
                }
                Ok(())
            }))
            .into()
    }
}

fn main() -> windows_core::Result<()> {
    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_secs(15));
        eprintln!("windows-canvas Reactor integration timed out");
        std::process::exit(1);
    });
    App::run_component::<Fixture>(())
}
