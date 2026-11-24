use super::*;

#[derive(Clone, Copy, PartialEq)]
pub(super) enum FocusMode {
    Controls,
    Empty,
    Hyperlink,
}

pub(super) struct FocusFixture {
    focus: String,
    activations: usize,
    first_focuses: usize,
}

impl Component for FocusFixture {
    type Input = FocusMode;
    type Message = (&'static str, FocusEventInfo);

    fn create(mode: &FocusMode, _: &ComponentContext<Self>) -> Self {
        Self {
            focus: if *mode == FocusMode::Empty {
                "Empty".into()
            } else {
                String::new()
            },
            activations: 0,
            first_focuses: 0,
        }
    }

    fn update(&mut self, (name, event): Self::Message, context: &ComponentContext<Self>) {
        self.focus = format!("{name} {:?}", event.state);
        if name == "First" {
            self.first_focuses += 1;
        }
        if name == "Second" && event.state == ElementFocusState::Keyboard && self.activations == 0 {
            assert!(context.activate_window());
            self.activations += 1;
        }
    }

    fn view(&self, mode: &FocusMode, context: &mut ViewContext<Self>) -> View {
        let mut children: Vec<View> = ["First", "Second"]
            .into_iter()
            .filter(|_| *mode != FocusMode::Empty)
            .map(|name| {
                Border::new()
                    .automation_id(name)
                    .automation_name(name)
                    .is_tab_stop(true)
                    .background(ThemeBrush::CardBackground)
                    .padding(16.0)
                    .on_got_focus(context.callback(move |event| (name, event)))
                    .content(name)
                    .into()
            })
            .collect();
        if *mode == FocusMode::Hyperlink {
            children.insert(
                0,
                RichTextBlock::new()
                    .paragraphs(RichText::new([RichTextParagraph::new([
                        RichTextInline::Hyperlink(RichTextHyperlink {
                            text: "Initial link".into(),
                            uri: "https://example.com".into(),
                        }),
                    ])]))
                    .into(),
            );
        }
        children.extend([
            TextBlock::new()
                .automation_id("focus")
                .text(&self.focus)
                .into(),
            TextBlock::new()
                .automation_id("activations")
                .text(self.activations.to_string())
                .into(),
            TextBlock::new()
                .automation_id("first-focuses")
                .text(self.first_focuses.to_string())
                .into(),
        ]);
        context.window_visuals(WindowVisuals::new().client_size(480.0, 320.0));
        context.window_frame(
            match mode {
                FocusMode::Controls => "Reactor UI fixture: window-focus",
                FocusMode::Empty => "Reactor UI fixture: window-focus-empty",
                FocusMode::Hyperlink => "Reactor UI fixture: window-focus-hyperlink",
            },
            StackPanel::new()
                .margin(20.0)
                .spacing(12.0)
                .children(children),
        )
    }
}

pub(super) struct WindowFixture {
    resized: Option<bool>,
    icon: bool,
    observed: WindowSize,
    restricted: bool,
}

pub(super) struct MaximizableFixture(Option<bool>);

impl Component for MaximizableFixture {
    type Input = bool;
    type Message = Option<bool>;

    fn create(_: &bool, _: &ComponentContext<Self>) -> Self {
        Self(Some(false))
    }

    fn update(&mut self, value: Option<bool>, _: &ComponentContext<Self>) {
        self.0 = value;
    }

    fn view(&self, custom: &bool, context: &mut ViewContext<Self>) -> View {
        let mut visuals = WindowVisuals::new().client_size(480.0, 320.0);
        if let Some(value) = self.0 {
            visuals = visuals.maximizable(value);
        }
        context.window_visuals(visuals);
        let content = StackPanel::new().margin(20.0).spacing(12.0).children((
            Button::new()
                .automation_id("enable-maximize")
                .content("Enable maximization")
                .on_click(context.message(Some(true))),
            Button::new()
                .automation_id("disable-maximize")
                .content("Disable maximization")
                .on_click(context.message(Some(false))),
            Button::new()
                .automation_id("clear-maximize")
                .content("Restore native default")
                .on_click(context.message(None)),
            TextBlock::new()
                .automation_id("maximizable")
                .text(format!("Maximizable {:?}", self.0)),
        ));
        if *custom {
            context.window_frame("Reactor UI fixture: window-maximizable-frame", content)
        } else {
            context.window_title("Reactor UI fixture: window-maximizable");
            content.into()
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub(super) enum WindowMode {
    Native,
    Custom,
    CustomPolicy,
}

#[derive(Clone, Copy)]
pub(super) enum Message {
    Resize(bool),
    Icon(bool),
    Size(WindowSize),
    Presentation,
}

impl Component for WindowFixture {
    type Input = WindowMode;
    type Message = Message;

    fn create(_: &WindowMode, _: &ComponentContext<Self>) -> Self {
        Self {
            resized: None,
            icon: false,
            observed: WindowSize::default(),
            restricted: false,
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Resize(resized) => self.resized = Some(resized),
            Message::Icon(icon) => self.icon = icon,
            Message::Size(size) => self.observed = size,
            Message::Presentation => self.restricted = !self.restricted,
        }
    }

    fn view(&self, mode: &WindowMode, context: &mut ViewContext<Self>) -> View {
        context.on_window_size(context.callback(Message::Size));
        let resized = self.resized.unwrap_or(false);
        let (width, height) = if resized {
            (640.0, 400.0)
        } else {
            (480.0, 320.0)
        };
        let fixture = match mode {
            WindowMode::Native => "window",
            WindowMode::Custom => "window-frame",
            WindowMode::CustomPolicy => "window-frame-policy",
        };
        let suffix = if resized { " updated" } else { "" };
        let title = format!("Reactor UI fixture: {fixture}{suffix}");
        let mut visuals = WindowVisuals::new()
            .always_on_top(self.restricted)
            .minimizable(!self.restricted);
        if *mode != WindowMode::CustomPolicy || self.resized.is_some() {
            visuals = visuals.client_size(width, height);
        }
        if self.icon {
            visuals = visuals.icon(concat!(env!("CARGO_MANIFEST_DIR"), "\\icon.ico"));
        }
        context.window_visuals(visuals);
        let content = StackPanel::new().margin(20.0).spacing(12.0).children((
            Button::new()
                .automation_id("resize")
                .content("Update title and size")
                .on_click(context.message(Message::Resize(true))),
            Button::new()
                .automation_id("reset")
                .content("Restore title and size")
                .on_click(context.message(Message::Resize(false))),
            Button::new()
                .automation_id("set-icon")
                .content("Set icon")
                .on_click(context.message(Message::Icon(true))),
            Button::new()
                .automation_id("clear-icon")
                .content("Clear icon")
                .on_click(context.message(Message::Icon(false))),
            Button::new()
                .automation_id("presentation")
                .content("Toggle topmost and minimization")
                .on_click(context.message(Message::Presentation)),
            TextBlock::new().automation_id("size").text(format!(
                "Size {:.3} x {:.3}",
                self.observed.width, self.observed.height
            )),
        ));
        if *mode != WindowMode::Native {
            context.window_frame(title, content)
        } else {
            context.window_title(title);
            content.into()
        }
    }
}

pub(super) struct TitleBarFixture {
    shown: bool,
    shifted: bool,
    value: f64,
}

#[derive(Clone, Copy)]
pub(super) enum TitleBarMessage {
    Show(bool),
    Move,
    Value(f64),
}

impl Component for TitleBarFixture {
    type Input = bool;
    type Message = TitleBarMessage;

    fn create(initial: &bool, _: &ComponentContext<Self>) -> Self {
        Self {
            shown: *initial,
            shifted: false,
            value: 50.0,
        }
    }

    fn update(&mut self, message: TitleBarMessage, _: &ComponentContext<Self>) {
        match message {
            TitleBarMessage::Show(shown) => {
                self.shown = shown;
                self.value = 50.0;
            }
            TitleBarMessage::Move => {
                self.shifted = !self.shifted;
                self.value = 50.0;
            }
            TitleBarMessage::Value(value) => self.value = value,
        }
    }

    fn view(&self, initial: &bool, context: &mut ViewContext<Self>) -> View {
        let fixture = if *initial {
            "window-titlebar-initial"
        } else {
            "window-titlebar-dynamic"
        };
        context.window_visuals(WindowVisuals::new().client_size(800.0, 320.0));
        let alignment = if self.shifted {
            HorizontalAlignment::Right
        } else {
            HorizontalAlignment::Left
        };
        let content: View = if self.shown {
            Slider::new()
                .automation_id("slider")
                .width(120.0)
                .horizontal_alignment(alignment)
                .value(self.value)
                .maximum(100.0)
                .on_value_changed(context.callback(TitleBarMessage::Value))
                .into()
        } else {
            TextBlock::new()
                .automation_id("drag-region")
                .width(120.0)
                .horizontal_alignment(alignment)
                .text("Drag here")
                .into()
        };
        Grid::new()
            .rows([GridLength::Auto, GridLength::Star(1.0)])
            .children((
                TitleBar::new()
                    .title(format!("Reactor UI fixture: {fixture}"))
                    .content(Grid::new().width(300.0).height(32.0).children([content])),
                StackPanel::new()
                    .grid_row(1)
                    .margin(20.0)
                    .spacing(12.0)
                    .children((
                        Button::new()
                            .automation_id("show")
                            .content("Add slider")
                            .on_click(context.message(TitleBarMessage::Show(true))),
                        Button::new()
                            .automation_id("move")
                            .content("Move slider")
                            .on_click(context.message(TitleBarMessage::Move)),
                        Button::new()
                            .automation_id("hide")
                            .content("Remove slider")
                            .on_click(context.message(TitleBarMessage::Show(false))),
                        TextBlock::new()
                            .automation_id("slider-value")
                            .text(format!("{:.0}", self.value)),
                    )),
            ))
            .into()
    }
}

pub(super) struct PlacementFixture {
    changed: bool,
    observed: Option<WindowPlacement>,
}

#[derive(Clone, Copy)]
pub(super) enum PlacementMessage {
    ChangeInitial,
    Observed(WindowPlacement),
}

impl Component for PlacementFixture {
    type Input = ();
    type Message = PlacementMessage;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            changed: false,
            observed: None,
        }
    }

    fn update(&mut self, message: PlacementMessage, _: &ComponentContext<Self>) {
        match message {
            PlacementMessage::ChangeInitial => self.changed = true,
            PlacementMessage::Observed(placement) => self.observed = Some(placement),
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        let placement = if self.changed {
            WindowPlacement {
                x: 240,
                y: 220,
                width: 760,
                height: 580,
                maximized: false,
            }
        } else {
            WindowPlacement {
                x: 80,
                y: 100,
                width: 640,
                height: 480,
                maximized: false,
            }
        };
        let observed = match self.observed {
            Some(value) => format!(
                "Placement {} {}; {} x {}; maximized {}",
                value.x, value.y, value.width, value.height, value.maximized
            ),
            None => "Waiting for placement".to_owned(),
        };
        context.window_title("Reactor UI fixture: window-placement");
        context.window_visuals(
            WindowVisuals::new()
                .client_size(320.0, 240.0)
                .initial_placement(placement),
        );
        context.on_window_placement(context.callback(PlacementMessage::Observed));
        StackPanel::new()
            .margin(20.0)
            .spacing(12.0)
            .children((
                TextBlock::new().automation_id("placement").text(observed),
                TextBlock::new()
                    .automation_id("state")
                    .text(format!("Changed initial placement: {}", self.changed)),
                Button::new()
                    .automation_id("change-initial")
                    .content("Change initial placement")
                    .on_click(context.message(PlacementMessage::ChangeInitial)),
            ))
            .into()
    }
}

pub(super) struct VisualsFixture {
    constrained: bool,
    dark: bool,
    backdrop: SystemBackdrop,
    moved: bool,
    observed: WindowSize,
}

#[derive(Clone, Copy)]
pub(super) enum VisualsMessage {
    Constraints,
    Theme,
    Backdrop(SystemBackdrop),
    Move,
    Size(WindowSize),
}

impl Component for VisualsFixture {
    type Input = ();
    type Message = VisualsMessage;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            constrained: true,
            dark: true,
            backdrop: SystemBackdrop::None,
            moved: false,
            observed: WindowSize::default(),
        }
    }

    fn update(&mut self, message: VisualsMessage, _: &ComponentContext<Self>) {
        match message {
            VisualsMessage::Constraints => self.constrained = !self.constrained,
            VisualsMessage::Theme => self.dark = !self.dark,
            VisualsMessage::Backdrop(backdrop) => self.backdrop = backdrop,
            VisualsMessage::Move => self.moved = true,
            VisualsMessage::Size(size) => self.observed = size,
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Reactor UI fixture: window-visuals");
        context.on_window_size(context.callback(VisualsMessage::Size));
        let position = if self.moved {
            ScreenPoint::new(300, 260)
        } else {
            ScreenPoint::new(120, 90)
        };
        let mut visuals = WindowVisuals::new()
            .client_size(480.0, 320.0)
            .initial_position(position)
            .backdrop(self.backdrop)
            .theme(if self.dark {
                WindowTheme::Dark
            } else {
                WindowTheme::Light
            });
        if self.constrained {
            visuals = visuals.constraints(WindowConstraints {
                min_width: Some(400.0),
                min_height: Some(300.0),
                max_width: Some(640.0),
                max_height: Some(480.0),
            });
        }
        context.window_visuals(visuals);
        let backdrop = |id, name, backdrop| {
            Button::new()
                .automation_id(id)
                .content(name)
                .on_click(context.message(VisualsMessage::Backdrop(backdrop)))
        };
        Grid::new()
            .rows([
                GridLength::Auto,
                GridLength::Auto,
                GridLength::Auto,
                GridLength::Auto,
                GridLength::STAR,
            ])
            .margin(12.0)
            .row_spacing(8.0)
            .children((
                TextBlock::new().automation_id("size").text(format!(
                    "Size {:.3} x {:.3}",
                    self.observed.width, self.observed.height
                )),
                TextBlock::new()
                    .grid_row(1)
                    .automation_id("state")
                    .text(format!(
                        "Constrained {}; dark {}; {:?}; moved {}",
                        self.constrained, self.dark, self.backdrop, self.moved
                    )),
                StackPanel::new()
                    .grid_row(2)
                    .orientation(Orientation::Horizontal)
                    .spacing(8.0)
                    .children((
                        Button::new()
                            .automation_id("constraints")
                            .content("Constraints")
                            .on_click(context.message(VisualsMessage::Constraints)),
                        Button::new()
                            .automation_id("theme")
                            .content("Theme")
                            .on_click(context.message(VisualsMessage::Theme)),
                        Button::new()
                            .automation_id("move")
                            .content("Position")
                            .on_click(context.message(VisualsMessage::Move)),
                    )),
                StackPanel::new()
                    .grid_row(3)
                    .orientation(Orientation::Horizontal)
                    .spacing(8.0)
                    .children((
                        backdrop("mica", "Mica", SystemBackdrop::Mica),
                        backdrop("mica-alt", "Mica alt", SystemBackdrop::MicaAlt),
                        backdrop("acrylic", "Acrylic", SystemBackdrop::Acrylic),
                        backdrop("no-backdrop", "None", SystemBackdrop::None),
                    )),
                // Leaves the window background visible for pixel checks.
                Border::new()
                    .grid_row(4)
                    .automation_id("swatch")
                    .automation_name("Background"),
            ))
            .into()
    }
}
