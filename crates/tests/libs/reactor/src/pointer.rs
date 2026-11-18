use super::*;

struct Fixture {
    enabled: bool,
    moved: Option<PointerEventInfo>,
    focused: bool,
    outside_released: bool,
}

#[derive(Clone)]
enum Message {
    Enable,
    Moved(PointerEventInfo),
    Focused(FocusEventInfo),
    Blurred,
    Outside(PointerEventInfo),
}

impl Component for Fixture {
    type Input = (bool, bool);
    type Message = Message;

    fn create(input: &Self::Input, _: &ComponentContext<Self>) -> Self {
        Self {
            enabled: input.1,
            moved: None,
            focused: false,
            outside_released: false,
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Enable => self.enabled = true,
            Message::Moved(value) => {
                self.moved = Some(value);
                if value.is_left_button_pressed {
                    self.outside_released = false;
                }
            }
            Message::Focused(value) => {
                self.focused = value.is_direct && value.state == ElementFocusState::Pointer;
            }
            Message::Blurred => self.focused = false,
            Message::Outside(value) => self.outside_released = !value.is_left_button_pressed,
        }
    }

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let mut target = Border::new()
            .width(200.0)
            .height(120.0)
            .background(Color::rgb(240, 240, 240))
            .is_tab_stop(true)
            .on_pointer_moved(context.callback(Message::Moved))
            .on_got_focus(context.callback(Message::Focused))
            .on_lost_focus(context.callback(|_| Message::Blurred))
            .content(
                StackPanel::new()
                    .orientation(Orientation::Horizontal)
                    .children((
                        TextBlock::new()
                            .automation_id("left")
                            .width(80.0)
                            .height(20.0)
                            .vertical_alignment(VerticalAlignment::Top)
                            .text("Left"),
                        TextBlock::new()
                            .automation_id("right")
                            .width(80.0)
                            .height(20.0)
                            .vertical_alignment(VerticalAlignment::Top)
                            .text("Right"),
                    )),
            );
        // An absent policy must stay absent until Enable, not be set to false at mount.
        if self.enabled {
            target = if input.0 {
                target.capture_pointer_on_press(true)
            } else {
                target.focus_on_pointer_release(true)
            };
        }
        let state = self.moved.map_or_else(
            || "No movement".to_owned(),
            |value| {
                format!(
                    "Left {}; outside {}",
                    value.is_left_button_pressed,
                    value.x < 0.0 || value.x >= 200.0 || value.y < 0.0 || value.y >= 120.0,
                )
            },
        );
        context.window_visuals(WindowVisuals::new().client_size(640.0, 400.0));
        context.window_frame(
            format!(
                "Reactor UI fixture: pointer-{}-{}",
                if input.0 { "capture" } else { "focus" },
                if input.1 { "mounted" } else { "added" },
            ),
            StackPanel::new().margin(20.0).spacing(12.0).children((
                Button::new()
                    .automation_id("baseline")
                    .content("Focus baseline"),
                Button::new()
                    .automation_id("enable")
                    .content("Enable policy")
                    .on_click(context.message(Message::Enable)),
                StackPanel::new()
                    .orientation(Orientation::Horizontal)
                    .spacing(20.0)
                    .children((
                        target,
                        Border::new()
                            .width(180.0)
                            .height(120.0)
                            .background(Color::rgb(220, 220, 220))
                            .on_pointer_moved(context.callback(Message::Outside))
                            .content(TextBlock::new().automation_id("outside").text("Outside")),
                    )),
                TextBlock::new().automation_id("pointer").text(state),
                TextBlock::new()
                    .automation_id("outside-state")
                    .text(if self.outside_released {
                        "Released move outside"
                    } else {
                        "No released move outside"
                    }),
                TextBlock::new()
                    .automation_id("focus")
                    .text(if self.focused {
                        "Pointer focused"
                    } else {
                        "Unfocused"
                    }),
                TextBlock::new()
                    .automation_id("policy")
                    .text(if self.enabled { "Enabled" } else { "Absent" }),
            )),
        )
    }
}

pub fn run(capture: bool, enabled: bool) -> windows_core::Result<()> {
    App::run_component::<Fixture>((capture, enabled))
}

struct ClickFixture {
    clicks: usize,
}

impl Component for ClickFixture {
    type Input = ();
    type Message = usize;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self { clicks: 0 }
    }

    fn update(&mut self, _: usize, _: &ComponentContext<Self>) {
        // A slow update keeps the UI thread busy so a burst of clicks queues behind one render.
        std::thread::sleep(std::time::Duration::from_millis(50));
        self.clicks += 1;
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        // Capturing render state gives the callback a new identity on every render.
        let seen = self.clicks;
        context.window_frame(
            "Reactor UI fixture: pointer-clicks",
            StackPanel::new().margin(20.0).spacing(12.0).children((
                Button::new()
                    .automation_id("button")
                    .content("Click")
                    .on_click(context.callback(move |()| seen)),
                TextBlock::new()
                    .automation_id("clicks")
                    .text(format!("Clicks {}", self.clicks)),
            )),
        )
    }
}

pub fn run_clicks() -> windows_core::Result<()> {
    App::run_component::<ClickFixture>(())
}

// Stderr records each delivery stage so a failure log shows whether the injected input or the
// queued callback was lost.
struct RoutingFixture {
    mode: usize,
    entered: usize,
    calls: Rc<std::cell::RefCell<Vec<&'static str>>>,
}

#[derive(Clone)]
enum RoutingMessage {
    Mode,
    Call,
    Entered,
}

impl Component for RoutingFixture {
    type Input = ();
    type Message = RoutingMessage;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            mode: 0,
            entered: 0,
            calls: Rc::default(),
        }
    }

    fn update(&mut self, message: RoutingMessage, _: &ComponentContext<Self>) {
        match message {
            RoutingMessage::Mode => {
                eprintln!("routing: update mode");
                self.mode = (self.mode + 1) % 3;
                self.calls.borrow_mut().clear();
            }
            RoutingMessage::Call => eprintln!("routing: update call"),
            RoutingMessage::Entered => {
                self.entered += 1;
                eprintln!("routing: entered {}", self.entered);
            }
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        eprintln!(
            "routing: view mode={} calls={:?}",
            self.mode,
            self.calls.borrow()
        );
        let calls = Rc::clone(&self.calls);
        let notify = context.message(RoutingMessage::Call);
        let callback: PointerCallback = if self.mode == 0 {
            Callback::new(move |_: PointerEventInfo| {
                eprintln!("routing: queued");
                calls.borrow_mut().push("queued");
                notify.call(());
            })
            .into()
        } else {
            let handled = self.mode == 2;
            RoutedCallback::new(move |_: PointerEventInfo| {
                eprintln!("routing: routed");
                calls.borrow_mut().push("routed");
                notify.call(());
                handled
            })
            .into()
        };
        let calls = Rc::clone(&self.calls);
        context.window_frame(
            "Reactor UI fixture: pointer-routing",
            StackPanel::new().margin(20.0).spacing(12.0).children((
                Button::new()
                    .automation_id("mode")
                    .content("Change mode")
                    .on_click(context.message(RoutingMessage::Mode)),
                Border::new()
                    .on_pointer_pressed(RoutedCallback::new(move |_| {
                        eprintln!("routing: parent");
                        calls.borrow_mut().push("parent");
                        false
                    }))
                    .content(
                        Border::new()
                            .width(200.0)
                            .height(100.0)
                            .background(Color::rgb(240, 240, 240))
                            .on_pointer_entered(context.callback(|_| RoutingMessage::Entered))
                            .on_pointer_pressed(callback)
                            .content(
                                TextBlock::new()
                                    .automation_id("routing-target")
                                    .text("Pointer target"),
                            ),
                    ),
                TextBlock::new()
                    .automation_id("mode-state")
                    .text(format!("Mode {}", self.mode)),
                TextBlock::new()
                    .automation_id("calls")
                    .text(self.calls.borrow().join(", ")),
                TextBlock::new()
                    .automation_id("routing-entered")
                    .text(format!("Entered {}", self.entered)),
            )),
        )
    }
}

pub fn run_routing() -> windows_core::Result<()> {
    App::run_component::<RoutingFixture>(())
}

#[derive(Default)]
struct CursorFixture {
    inner: usize,
    outer: bool,
    entered: usize,
}

#[derive(Clone)]
enum CursorMessage {
    Inner,
    Outer,
    Entered,
}

impl Component for CursorFixture {
    type Input = ();
    type Message = CursorMessage;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            inner: 0,
            outer: true,
            entered: 0,
        }
    }

    fn update(&mut self, message: CursorMessage, _: &ComponentContext<Self>) {
        match message {
            CursorMessage::Inner => self.inner = (self.inner + 1) % 3,
            CursorMessage::Outer => self.outer = !self.outer,
            CursorMessage::Entered => {
                self.entered += 1;
                eprintln!("cursor: inner entered message {}", self.entered);
            }
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        let inner_shape = [
            Some(InputSystemCursorShape::SizeWestEast),
            None,
            Some(InputSystemCursorShape::IBeam),
        ][self.inner];
        let mut inner = Border::new()
            .width(160.0)
            .height(80.0)
            .background(Color::rgb(200, 200, 200))
            .horizontal_alignment(HorizontalAlignment::Left)
            .vertical_alignment(VerticalAlignment::Top)
            .on_pointer_entered(context.callback(|_| CursorMessage::Entered))
            .content(TextBlock::new().automation_id("cursor-inner").text("Inner"));
        if let Some(shape) = inner_shape {
            inner = inner.pointer_cursor(shape);
        }
        let mut outer = Border::new()
            .width(320.0)
            .height(160.0)
            .background(Color::rgb(240, 240, 240))
            .content(
                Grid::new()
                    .rows([GridLength::STAR, GridLength::Auto])
                    .children((
                        inner,
                        TextBlock::new()
                            .automation_id("cursor-outer")
                            .grid_row(1)
                            .text("Outer"),
                    )),
            );
        if self.outer {
            outer = outer.pointer_cursor(InputSystemCursorShape::Hand);
        }
        context.window_visuals(WindowVisuals::new().client_size(640.0, 400.0));
        // Logs whether pointer input reaches the content at all, separately from the inner target.
        let log = |event: &'static str| {
            RoutedCallback::new(move |info: PointerEventInfo| {
                eprintln!(
                    "cursor: root {event} at ({:.1}, {:.1})",
                    info.window_x, info.window_y
                );
                false
            })
        };
        context.window_frame(
            "Reactor UI fixture: pointer-cursor",
            Border::new()
                .on_pointer_entered(log("entered"))
                .on_pointer_moved(log("moved"))
                .content(
                    StackPanel::new().margin(20.0).spacing(12.0).children((
                        Button::new()
                            .automation_id("inner-cursor")
                            .content("Change inner cursor")
                            .on_click(context.message(CursorMessage::Inner)),
                        Button::new()
                            .automation_id("outer-cursor")
                            .content("Toggle outer cursor")
                            .on_click(context.message(CursorMessage::Outer)),
                        outer,
                        TextBlock::new().automation_id("cursor-state").text(format!(
                            "Inner {inner_shape:?}; outer {}",
                            if self.outer { "Hand" } else { "None" }
                        )),
                        TextBlock::new()
                            .automation_id("cursor-entered")
                            .text(format!("Entered {}", self.entered)),
                    )),
                ),
        )
    }
}

pub fn run_cursor() -> windows_core::Result<()> {
    App::run_component::<CursorFixture>(())
}
