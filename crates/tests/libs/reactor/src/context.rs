use super::*;
use std::cell::Cell;
use std::sync::LazyLock;

static VALUE: LazyLock<Context<u32>> = LazyLock::new(|| Context::new(100));
static OTHER: LazyLock<Context<u32>> = LazyLock::new(|| Context::new(200));

pub(super) struct ContextFixture {
    outer: u32,
    other: u32,
    provided: bool,
    shadow: bool,
    renders: Cell<u32>,
}

pub(super) enum Message {
    Bump,
    Same,
    Provide,
    Shadow,
    Other,
}

impl Component for ContextFixture {
    type Input = bool;
    type Message = Message;

    fn create(_: &bool, _: &ComponentContext<Self>) -> Self {
        Self {
            outer: 1,
            other: 10,
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
            Message::Other => self.other += 1,
        }
    }

    fn view(&self, virtual_rows: &bool, context: &mut ViewContext<Self>) -> View {
        context.window_title(if *virtual_rows {
            "Reactor UI fixture: context-virtual"
        } else {
            "Reactor UI fixture: context"
        });
        context.window_visuals(WindowVisuals::new().client_size(500.0, 650.0));
        self.renders.set(self.renders.get() + 1);
        let shadowed = View::component::<Middle>("b");
        let consumers = if *virtual_rows {
            View::component::<VirtualConsumers>(self.shadow)
        } else {
            StackPanel::new()
                .spacing(12.0)
                .children((
                    View::component::<Middle>("a"),
                    if self.shadow {
                        provide(&VALUE, 50, shadowed)
                    } else {
                        shadowed
                    },
                ))
                .into()
        };
        let consumers = provide(&OTHER, self.other, consumers);
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
                Button::new()
                    .automation_id("other")
                    .content("Change other context")
                    .on_click(context.callback(|_| Message::Other)),
                TextBlock::new()
                    .automation_id("root")
                    .text(format!("Root renders {}", self.renders.get())),
                if self.provided {
                    provide(&VALUE, self.outer, consumers)
                } else {
                    consumers
                },
            ))
            .into()
    }
}

struct VirtualConsumers;

impl Component for VirtualConsumers {
    type Input = bool;
    type Message = ();

    fn create(_: &bool, _: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, shadow: &bool, context: &mut ViewContext<Self>) -> View {
        let value = context.use_context(&VALUE);
        let other = context.use_context(&OTHER);
        let shadow = *shadow;
        StackPanel::new()
            .children((
                TextBlock::new()
                    .automation_id("list-context")
                    .text(format!("Value {value}/{other}")),
                ScrollViewer::new()
                    .automation_id("rows")
                    .height(220.0)
                    .content(ItemsRepeater::new().virtual_source(VirtualSource::new(
                        0,
                        200,
                        |index| index,
                        move |index| {
                            let row = View::component::<ContextRow>(index);
                            if shadow && index == 0 {
                                provide(&VALUE, 50, row)
                            } else {
                                row
                            }
                        },
                    ))),
            ))
            .into()
    }
}

struct ContextRow {
    renders: Cell<u32>,
    clicks: u32,
}

impl Component for ContextRow {
    type Input = usize;
    type Message = ();

    fn create(_: &usize, _: &ComponentContext<Self>) -> Self {
        Self {
            renders: Cell::new(0),
            clicks: 0,
        }
    }

    fn update(&mut self, _: (), _: &ComponentContext<Self>) {
        self.clicks += 1;
    }

    fn view(&self, index: &usize, context: &mut ViewContext<Self>) -> View {
        self.renders.set(self.renders.get() + 1);
        let value = context.use_context(&VALUE);
        let other = context.use_context(&OTHER);
        Button::new()
            .automation_id(format!("context-row-{index}"))
            .height(40.0)
            .content(format!(
                "Value {value}/{other}; renders {}; clicks {}",
                self.renders.get(),
                self.clicks
            ))
            .on_click(context.forward())
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

struct WrappedContext<'a, C: Component> {
    inner: &'a mut ViewContext<C>,
}

impl<C: Component> WrappedContext<'_, C> {
    fn value(&mut self) -> u32 {
        self.inner.use_context(&VALUE)
    }
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
        let value = WrappedContext { inner: context }.value();
        TextBlock::new()
            .automation_id(format!("{name}-value"))
            .text(format!("Value {value}; renders {}", self.renders.get()))
            .into()
    }
}
