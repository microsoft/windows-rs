use super::*;
use std::cell::Cell;
use std::sync::LazyLock;

static VALUE: LazyLock<Context<u32>> = LazyLock::new(|| Context::new(100));

pub(super) struct ContextFixture {
    outer: u32,
    provided: bool,
    shadow: bool,
    renders: Cell<u32>,
}

pub(super) enum Message {
    Bump,
    Same,
    Provide,
    Shadow,
}

impl Component for ContextFixture {
    type Input = ();
    type Message = Message;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            outer: 1,
            provided: true,
            shadow: false,
            renders: Cell::new(0),
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Bump => self.outer += 1,
            Message::Same => {}
            Message::Provide => self.provided = !self.provided,
            Message::Shadow => self.shadow = !self.shadow,
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Reactor UI fixture: context");
        context.window_visuals(WindowVisuals::new().client_size(500.0, 400.0));
        self.renders.set(self.renders.get() + 1);
        let shadowed = View::component::<Middle>("b");
        let consumers = StackPanel::new().spacing(12.0).children((
            View::component::<Middle>("a"),
            if self.shadow {
                provide(&VALUE, 50, shadowed)
            } else {
                shadowed
            },
        ));
        StackPanel::new()
            .margin(20.0)
            .spacing(12.0)
            .children((
                Button::new()
                    .automation_id("bump")
                    .content("Bump")
                    .on_click(context.callback(|_| Message::Bump)),
                Button::new()
                    .automation_id("same")
                    .content("Render with the same value")
                    .on_click(context.callback(|_| Message::Same)),
                Button::new()
                    .automation_id("provide")
                    .content("Toggle provider")
                    .on_click(context.callback(|_| Message::Provide)),
                Button::new()
                    .automation_id("shadow")
                    .content("Toggle shadow")
                    .on_click(context.callback(|_| Message::Shadow)),
                TextBlock::new()
                    .automation_id("root")
                    .text(format!("Root renders {}", self.renders.get())),
                if self.provided {
                    provide(&VALUE, self.outer, consumers)
                } else {
                    consumers.into()
                },
            ))
            .into()
    }
}

// Its input never changes, so value changes must reach `Consumer` without re-rendering it.
struct Middle {
    renders: Cell<u32>,
}

impl Component for Middle {
    type Input = &'static str;
    type Message = ();

    fn create(_: &&'static str, _: &ComponentContext<Self>) -> Self {
        Self {
            renders: Cell::new(0),
        }
    }

    fn view(&self, name: &&'static str, _: &mut ViewContext<Self>) -> View {
        self.renders.set(self.renders.get() + 1);
        StackPanel::new()
            .children((
                TextBlock::new()
                    .automation_id(format!("{name}-middle"))
                    .text(format!("Middle renders {}", self.renders.get())),
                View::component::<Consumer>(*name),
            ))
            .into()
    }
}

struct Consumer {
    renders: Cell<u32>,
}

impl Component for Consumer {
    type Input = &'static str;
    type Message = ();

    fn create(_: &&'static str, _: &ComponentContext<Self>) -> Self {
        Self {
            renders: Cell::new(0),
        }
    }

    fn view(&self, name: &&'static str, context: &mut ViewContext<Self>) -> View {
        self.renders.set(self.renders.get() + 1);
        let value = context.use_context(&VALUE);
        TextBlock::new()
            .automation_id(format!("{name}-value"))
            .text(format!("Value {value}; renders {}", self.renders.get()))
            .into()
    }
}
