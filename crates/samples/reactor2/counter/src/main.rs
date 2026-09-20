#![windows_subsystem = "windows"]

use std::cell::RefCell;
use std::rc::Rc;
use windows_reactor::{App, AppContext};
use windows_reactor2 as reactor2;

struct Count;

impl reactor2::Component for Count {
    type Input = u32;
    type Message = ();

    fn create(_input: &Self::Input, _context: &reactor2::ComponentContext<Self::Message>) -> Self {
        Self
    }

    fn view(
        &self,
        input: &Self::Input,
        _context: &mut reactor2::ComponentViewContext<'_, Self::Message>,
    ) -> reactor2::Visual {
        reactor2::TextBlock::new(input.to_string()).into()
    }
}

struct Counter {
    count: u32,
}

impl reactor2::Component for Counter {
    type Input = ();
    type Message = ();

    fn create(_input: &Self::Input, _context: &reactor2::ComponentContext<Self::Message>) -> Self {
        Self { count: 0 }
    }

    fn update(
        &mut self,
        _message: Self::Message,
        _context: &reactor2::ComponentContext<Self::Message>,
    ) {
        self.count += 1;
    }

    fn view(
        &self,
        _input: &Self::Input,
        context: &mut reactor2::ComponentViewContext<'_, Self::Message>,
    ) -> reactor2::Visual {
        let increment = context.sender();
        reactor2::StackPanel::new()
            .spacing(8.0)
            .children([
                reactor2::TextBlock::new("Reactor2 counter").into(),
                reactor2::Border::new()
                    .content(reactor2::component::<Count>("count", self.count))
                    .into(),
                reactor2::Button::new()
                    .content(reactor2::TextBlock::new("Increment"))
                    .on_click(move || {
                        _ = increment.send(());
                    })
                    .into(),
            ])
            .into()
    }
}

struct Host {
    host: reactor2::ComponentHost<reactor2::native::WinUiAdapter>,
    _window: reactor2::native::NativeWindow,
}

impl Host {
    fn new(context: &AppContext) -> windows_core::Result<Rc<RefCell<Option<Self>>>> {
        let state = Rc::new(RefCell::new(None::<Self>));
        let drain_state = Rc::clone(&state);
        let drain = context.callback(move || {
            drain_state
                .borrow_mut()
                .as_mut()
                .unwrap()
                .host
                .drain(usize::MAX)
                .unwrap();
        });
        let mut host = reactor2::ComponentHost::mount(
            reactor2::native::WinUiAdapter::default(),
            [reactor2::component::<Counter>("counter", ())],
        )
        .unwrap();
        let wake = drain.clone();
        host.set_waker(move || {
            _ = wake.invoke();
        });
        let wake = drain;
        host.runtime_mut().adapter_mut().set_event_waker(move || {
            _ = wake.invoke();
        });
        let root = host.runtime().graph().root().unwrap();
        let mut window = host.runtime().adapter().open_window(root).unwrap();
        let application = context.proxy();
        window
            .set_closed(move || {
                _ = application.exit();
            })
            .unwrap();
        *state.borrow_mut() = Some(Self {
            host,
            _window: window,
        });
        Ok(state)
    }
}

fn main() -> windows_core::Result<()> {
    App::run_with(Host::new)
}
