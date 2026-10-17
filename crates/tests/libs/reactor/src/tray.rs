use super::*;

const ICON: &str = concat!(env!("CARGO_MANIFEST_DIR"), "\\icon.ico");

#[derive(Clone, PartialEq)]
pub(super) struct TrayFixture {
    generation: u32,
    calls: u32,
    selected: u32,
    owner: u32,
    attached: bool,
    activation: Activation,
}

#[derive(Clone, Copy)]
pub(super) enum Message {
    Activate,
    SetActivation(Activation),
    Selected(u32),
    Exit,
}

#[derive(Clone, Copy, PartialEq)]
pub(super) enum Activation {
    ReplaceCallback,
    Remove,
    ReplaceOwner,
}

impl Application for TrayFixture {
    type Input = ();
    type Message = Message;

    fn create(_: &(), context: &ApplicationContext<Self>) -> Self {
        context.show_window("fixture");
        Self {
            generation: 0,
            calls: 0,
            selected: 0,
            owner: 0,
            attached: true,
            activation: Activation::ReplaceCallback,
        }
    }

    fn update(&mut self, message: Message, context: &ApplicationContext<Self>) {
        match message {
            Message::Activate => match self.activation {
                Activation::ReplaceCallback => self.generation += 1,
                Activation::Remove => self.attached = false,
                Activation::ReplaceOwner => {
                    self.owner += 1;
                    self.generation += 1;
                }
            },
            Message::SetActivation(activation) => self.activation = activation,
            Message::Selected(generation) => {
                self.selected = generation;
                self.calls += 1;
            }
            Message::Exit => context.exit(),
        }
    }

    fn view(&self, _: &(), context: &ApplicationViewContext<Self>) -> ApplicationView {
        let view = ApplicationView::new().window::<TrayWindow>(
            "fixture",
            WindowInput {
                state: self.clone(),
                send: context.forward(),
                closed: context.message(Message::Exit),
            },
        );
        if !self.attached {
            return view;
        }
        let generation = self.generation;
        view.notify_icon(
            self.owner,
            NotifyIcon::new(ICON)
                .on_activate(context.callback(|_| Message::Activate))
                .menu(Menu::new(
                    [
                        MenuItem::item("command", "Command"),
                        MenuItem::submenu("more", "More", [MenuItem::item("nested", "Nested")]),
                    ],
                    context.callback(move |key: Key| {
                        assert_eq!(key.as_str(), Some("command"));
                        Message::Selected(generation)
                    }),
                )),
        )
    }
}

#[derive(Clone, PartialEq)]
struct WindowInput {
    state: TrayFixture,
    send: Callback<Message>,
    closed: Callback<()>,
}

struct TrayWindow {
    closed: Callback<()>,
}

impl Drop for TrayWindow {
    fn drop(&mut self) {
        self.closed.call(());
    }
}

impl Component for TrayWindow {
    type Input = WindowInput;
    type Message = ();

    fn create(input: &WindowInput, _: &ComponentContext<Self>) -> Self {
        Self {
            closed: input.closed.clone(),
        }
    }

    fn update(&mut self, _: (), _: &ComponentContext<Self>) {}

    fn view(&self, input: &WindowInput, context: &mut ViewContext<Self>) -> View {
        let action = |activation| {
            let send = input.send.clone();
            move || send.call(Message::SetActivation(activation))
        };
        let state = &input.state;
        context.window_visuals(WindowVisuals::new().client_size(500.0, 350.0));
        context.window_frame(
            "Reactor UI fixture: tray",
            StackPanel::new().margin(20.0).spacing(12.0).children((
                Button::new()
                    .automation_id("replace")
                    .content("Replace callback on activation")
                    .on_click(action(Activation::ReplaceCallback)),
                Button::new()
                    .automation_id("remove")
                    .content("Remove icon on activation")
                    .on_click(action(Activation::Remove)),
                Button::new()
                    .automation_id("replace-owner")
                    .content("Replace icon on activation")
                    .on_click(action(Activation::ReplaceOwner)),
                TextBlock::new()
                    .automation_id("activation")
                    .text(match state.activation {
                        Activation::ReplaceCallback => "Replace callback",
                        Activation::Remove => "Remove icon",
                        Activation::ReplaceOwner => "Replace icon",
                    }),
                TextBlock::new().automation_id("state").text(format!(
                    "Generation {}; attached {}",
                    state.generation, state.attached
                )),
                TextBlock::new().automation_id("selection").text(format!(
                    "Selected {}; calls {}",
                    state.selected, state.calls
                )),
                TextBlock::new()
                    .automation_id("owner")
                    .text(format!("Owner {}", state.owner)),
            )),
        )
    }
}
