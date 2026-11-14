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

struct RoutingFixture {
    mode: usize,
    calls: Rc<std::cell::RefCell<Vec<String>>>,
}

impl Component for RoutingFixture {
    type Input = bool;
    type Message = bool;

    fn create(_: &bool, _: &ComponentContext<Self>) -> Self {
        Self {
            mode: 0,
            calls: Rc::default(),
        }
    }

    fn update(&mut self, change_mode: bool, _: &ComponentContext<Self>) {
        if change_mode {
            self.mode += 1;
            self.calls.borrow_mut().clear();
        }
    }

    fn view(&self, wheel: &bool, context: &mut ViewContext<Self>) -> View {
        let wheel = *wheel;
        let mode = self.mode % if wheel { 4 } else { 3 };
        if wheel {
            eprintln!("wheel view: mode {mode}, calls {:?}", self.calls.borrow());
        }
        let record = move |name: &str, info: PointerEventInfo| {
            if wheel {
                eprintln!("wheel callback: {name}, mode {mode}, {info:?}");
                format!(
                    "{name} delta {} horizontal {} shift {} control {}",
                    info.wheel_delta,
                    info.is_horizontal_wheel,
                    info.modifiers.contains(InputModifiers::SHIFT),
                    info.modifiers.contains(InputModifiers::CONTROL),
                )
            } else {
                name.to_owned()
            }
        };
        let calls = Rc::clone(&self.calls);
        let notify = context.message(false);
        let callback: PointerCallback = if mode == 0 {
            Callback::new(move |info: PointerEventInfo| {
                calls.borrow_mut().push(record("queued", info));
                notify.call(());
            })
            .into()
        } else {
            let handled = mode == 2;
            RoutedCallback::new(move |info: PointerEventInfo| {
                calls.borrow_mut().push(record("routed", info));
                notify.call(());
                handled
            })
            .into()
        };
        let calls = Rc::clone(&self.calls);
        let notify = context.message(false);
        let parent_callback = RoutedCallback::new(move |info| {
            calls.borrow_mut().push(record("parent", info));
            notify.call(());
            false
        });
        let child = Border::new()
            .width(200.0)
            .height(100.0)
            .background(Color::rgb(240, 240, 240))
            .content(
                TextBlock::new()
                    .automation_id("routing-target")
                    .text("Pointer target"),
            );
        let parent = Border::new();
        let (parent, child) = if wheel {
            let trace = |event| {
                RoutedCallback::new(move |info: PointerEventInfo| {
                    eprintln!("wheel target {event}: {info:?}");
                    false
                })
            };
            let child = child
                .on_pointer_entered(trace("entered"))
                .on_pointer_moved(trace("moved"))
                .on_pointer_exited(trace("exited"));
            (
                parent.on_pointer_wheel_changed(parent_callback),
                if mode == 3 {
                    child
                } else {
                    child.on_pointer_wheel_changed(callback)
                },
            )
        } else {
            (
                parent.on_pointer_pressed(parent_callback),
                child.on_pointer_pressed(callback),
            )
        };
        context.window_frame(
            if wheel {
                "Reactor UI fixture: pointer-wheel"
            } else {
                "Reactor UI fixture: pointer-routing"
            },
            StackPanel::new().margin(20.0).spacing(12.0).children((
                Button::new()
                    .automation_id("mode")
                    .content("Change mode")
                    .on_click(context.message(true)),
                parent.content(child),
                TextBlock::new()
                    .automation_id("mode-state")
                    .text(format!("Mode {mode}")),
                TextBlock::new()
                    .automation_id("calls")
                    .text(self.calls.borrow().join(", ")),
            )),
        )
    }
}

pub fn run_routing(wheel: bool) -> windows_core::Result<()> {
    App::run_component::<RoutingFixture>(wheel)
}
