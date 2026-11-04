use std::ffi::c_void;
use windows_reactor::*;
use windows_result::{HRESULT, Result};

const DWMWA_WINDOW_CORNER_PREFERENCE: u32 = 33;
const DWMWCP_DONOTROUND: i32 = 1;

#[link(name = "dwmapi")]
unsafe extern "system" {
    fn DwmSetWindowAttribute(
        hwnd: *mut c_void,
        attribute: u32,
        value: *const c_void,
        size: u32,
    ) -> HRESULT;
}

#[derive(Clone)]
enum Message {
    Create,
    Prepared(Result<()>),
    Show,
    Closed,
    Exit,
}

struct WindowCreation {
    ready: bool,
    status: String,
}

impl Application for WindowCreation {
    type Input = ();
    type Message = Message;

    fn create(_: &(), context: &ApplicationContext<Self>) -> Self {
        context.show_window("controls");
        Self {
            ready: false,
            status: "No secondary window has been created.".into(),
        }
    }

    fn update(&mut self, message: Message, context: &ApplicationContext<Self>) {
        match message {
            Message::Create => {
                self.status = "Creating a hidden window...".into();
                context.create_window("prepared");
            }
            Message::Prepared(result) => {
                self.ready = true;
                self.status = match result {
                    Ok(()) => "Created hidden; DWM configured square corners. Press Show.".into(),
                    Err(error) => format!("Created hidden, but DWM setup failed: {error}"),
                };
            }
            Message::Show => context.show_window("prepared"),
            Message::Closed => {
                self.ready = false;
                self.status = "The window was destroyed. Create another to repeat.".into();
            }
            Message::Exit => context.exit(),
        }
    }

    fn view(&self, _: &(), context: &ApplicationViewContext<Self>) -> ApplicationView {
        ApplicationView::new()
            .window::<Controls>(
                "controls",
                ControlsInput {
                    ready: self.ready,
                    status: self.status.clone(),
                    create: context.message(Message::Create),
                    show: context.message(Message::Show),
                    exit: context.message(Message::Exit),
                },
            )
            .window::<PreparedWindow>(
                "prepared",
                (
                    context.callback(Message::Prepared),
                    context.message(Message::Closed),
                ),
            )
    }
}

#[derive(Clone, PartialEq)]
struct ControlsInput {
    ready: bool,
    status: String,
    create: Callback<()>,
    show: Callback<()>,
    exit: Callback<()>,
}

struct Controls(Callback<()>);

impl Component for Controls {
    type Input = ControlsInput;
    type Message = ();

    fn create(input: &Self::Input, _: &ComponentContext<Self>) -> Self {
        Self(input.exit.clone())
    }

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("Window creation");
        context.window_visuals(WindowVisuals::new().client_size(560.0, 240.0));
        StackPanel::new()
            .margin(24.0)
            .spacing(12.0)
            .children((
                "Create a hidden window, configure its HWND, then show the same instance.",
                "Square corners use a Windows 11 DWM attribute; errors are shown below.",
                Button::new()
                    .automation_id("create")
                    .content("Create hidden")
                    .is_enabled(!input.ready)
                    .on_click(input.create.clone()),
                Button::new()
                    .automation_id("show")
                    .content("Show")
                    .is_enabled(input.ready)
                    .on_click(input.show.clone()),
                TextBlock::new()
                    .automation_id("status")
                    .text(&input.status)
                    .text_wrapping(TextWrapping::Wrap),
            ))
            .into()
    }
}

impl Drop for Controls {
    fn drop(&mut self) {
        self.0.call(());
    }
}

struct PreparedWindow {
    prepared: Callback<Result<()>>,
    closed: Callback<()>,
}

impl Component for PreparedWindow {
    type Input = (Callback<Result<()>>, Callback<()>);
    type Message = Result<()>;

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        assert!(context.run_window(|window| {
            let corners = DWMWCP_DONOTROUND;
            unsafe {
                DwmSetWindowAttribute(
                    window.as_raw(),
                    DWMWA_WINDOW_CORNER_PREFERENCE,
                    std::ptr::from_ref(&corners).cast(),
                    size_of_val(&corners) as u32,
                )
                .ok()
            }
        }));
        Self {
            prepared: input.0.clone(),
            closed: input.1.clone(),
        }
    }

    fn update(&mut self, result: Result<()>, _: &ComponentContext<Self>) {
        self.prepared.call(result);
    }

    fn view(&self, _: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("Prepared window");
        context.window_visuals(WindowVisuals::new().client_size(440.0, 160.0));
        TextBlock::new()
            .margin(24.0)
            .text("Native setup ran while hidden. Close this window, then create another.")
            .text_wrapping(TextWrapping::Wrap)
            .into()
    }
}

impl Drop for PreparedWindow {
    fn drop(&mut self) {
        self.closed.call(());
    }
}

fn main() {
    App::run_application::<WindowCreation>(()).unwrap();
}
