use super::*;

pub(super) struct ApplicationFixture {
    value: u32,
    mounted: u32,
    closed: u32,
}

#[derive(Clone, Copy)]
pub(super) enum Message {
    Show,
    Increment,
    Mounted,
    Closed,
}

impl Application for ApplicationFixture {
    type Input = ();
    type Message = Message;

    fn create(_: &(), context: &ApplicationContext<Self>) -> Self {
        context.show_window("main");
        Self {
            value: 0,
            mounted: 0,
            closed: 0,
        }
    }

    fn update(&mut self, message: Message, context: &ApplicationContext<Self>) {
        match message {
            Message::Show => {
                context.show_window("secondary");
                context.show_window("secondary");
            }
            Message::Increment => self.value += 1,
            Message::Mounted => self.mounted += 1,
            Message::Closed => self.closed += 1,
        }
    }

    fn view(&self, _: &(), context: &ApplicationViewContext<Self>) -> ApplicationView {
        let input = |main| WindowInput {
            main,
            value: self.value,
            mounted: self.mounted,
            closed: self.closed,
            on_show: context.message(Message::Show),
            on_increment: context.message(Message::Increment),
            on_mounted: context.message(Message::Mounted),
            on_closed: context.message(Message::Closed),
        };
        ApplicationView::new()
            .window::<ApplicationWindow>("main", input(true))
            .window::<ApplicationWindow>("secondary", input(false))
    }
}

#[derive(Clone, PartialEq)]
struct WindowInput {
    main: bool,
    value: u32,
    mounted: u32,
    closed: u32,
    on_show: Callback<()>,
    on_increment: Callback<()>,
    on_mounted: Callback<()>,
    on_closed: Callback<()>,
}

struct ApplicationWindow(Option<Callback<()>>);

impl Drop for ApplicationWindow {
    fn drop(&mut self) {
        if let Some(closed) = &self.0 {
            closed.call(());
        }
    }
}

impl Component for ApplicationWindow {
    type Input = WindowInput;
    type Message = ();

    fn create(input: &WindowInput, _: &ComponentContext<Self>) -> Self {
        if !input.main {
            input.on_mounted.call(());
        }
        Self((!input.main).then(|| input.on_closed.clone()))
    }

    fn update(&mut self, _: (), context: &ComponentContext<Self>) {
        assert!(context.close_window());
    }

    fn view(&self, input: &WindowInput, context: &mut ViewContext<Self>) -> View {
        context.window_visuals(WindowVisuals::new().client_size(480.0, 320.0));
        if !input.main {
            context.window_title("Reactor secondary fixture");
            return StackPanel::new()
                .margin(20.0)
                .spacing(12.0)
                .children((
                    TextBlock::new()
                        .automation_id("secondary-value")
                        .text(format!("Value {}", input.value)),
                    Button::new()
                        .automation_id("close-secondary")
                        .content("Close")
                        .on_click(context.message(())),
                ))
                .into();
        }
        context.window_title("Reactor UI fixture: application");
        let show = input.on_show.clone();
        let increment = input.on_increment.clone();
        StackPanel::new()
            .margin(20.0)
            .spacing(12.0)
            .children((
                Button::new()
                    .automation_id("show")
                    .content("Show secondary twice")
                    .on_click(move || show.call(())),
                Button::new()
                    .automation_id("increment")
                    .content("Increment")
                    .on_click(move || increment.call(())),
                TextBlock::new().automation_id("lifetime").text(format!(
                    "Mounted {}; closed {}",
                    input.mounted, input.closed
                )),
            ))
            .into()
    }
}

pub(super) struct ReplacingWindow(u32);

#[derive(Clone, Copy)]
pub(super) enum ReplacementMessage {
    Replace,
    Ping,
}

impl Component for ReplacingWindow {
    type Input = bool;
    type Message = ReplacementMessage;

    fn create(_: &bool, _: &ComponentContext<Self>) -> Self {
        Self(0)
    }

    fn update(&mut self, message: ReplacementMessage, context: &ComponentContext<Self>) {
        match message {
            ReplacementMessage::Replace => {
                assert!(context.open_window::<Self>(true));
                assert!(context.close_window());
            }
            ReplacementMessage::Ping => self.0 += 1,
        }
    }

    fn view(&self, replacement: &bool, context: &mut ViewContext<Self>) -> View {
        context.window_title("Reactor UI fixture: window-replacement");
        context.window_visuals(WindowVisuals::new().client_size(480.0, 320.0));
        if !replacement {
            return Button::new()
                .automation_id("replace-window")
                .content("Replace last window")
                .on_click(context.message(ReplacementMessage::Replace))
                .into();
        }
        StackPanel::new()
            .margin(20.0)
            .spacing(12.0)
            .children((
                Button::new()
                    .automation_id("ping")
                    .content("Ping replacement")
                    .on_click(context.message(ReplacementMessage::Ping)),
                TextBlock::new()
                    .automation_id("replacement")
                    .text(format!("Updates {}", self.0)),
            ))
            .into()
    }
}

pub(super) fn startup_error() -> windows_core::Result<()> {
    let expected = windows_core::HRESULT(0x80004005u32 as i32);
    let called = Rc::new(std::cell::Cell::new(false));
    let startup_called = called.clone();
    let error = App::run_with(move |_| {
        startup_called.set(true);
        Err::<(), _>(windows_core::Error::new(expected, "startup failed"))
    })
    .unwrap_err();
    assert!(called.get(), "startup callback did not run");
    assert_eq!(error.code(), expected);
    Ok(())
}
