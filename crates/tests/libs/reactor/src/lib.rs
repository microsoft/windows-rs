use std::rc::Rc;
use windows_reactor::*;

mod application;
mod background;
mod canvas;
mod dialog;
mod drag;
mod flyout;
mod image;
mod keyboard;
mod pointer;
mod reference;
mod repeater;
mod retirement;
mod selection;
mod timer;
mod tray;
mod window;

#[derive(Clone, Copy, PartialEq)]
enum MenuSurface {
    Button,
    DropDown,
    MenuBar,
    CommandBar,
}

struct MenuFixture {
    generation: u32,
    calls: u32,
    selected: u32,
    attached: bool,
    changed: bool,
    owner: u32,
}

#[derive(Clone)]
enum MenuMessage {
    Replace,
    Change,
    Remove,
    Attach,
    ReplaceOwner,
    Selected(u32),
}

impl Component for MenuFixture {
    type Input = MenuSurface;
    type Message = MenuMessage;

    fn create(_: &MenuSurface, _: &ComponentContext<Self>) -> Self {
        Self {
            generation: 0,
            calls: 0,
            selected: 0,
            attached: true,
            changed: false,
            owner: 0,
        }
    }

    fn update(&mut self, message: MenuMessage, _: &ComponentContext<Self>) {
        match message {
            MenuMessage::Replace => self.generation += 1,
            MenuMessage::Change => self.changed = true,
            MenuMessage::Remove => self.attached = false,
            MenuMessage::Attach => self.attached = true,
            MenuMessage::ReplaceOwner => {
                self.owner += 1;
                self.generation += 1;
            }
            MenuMessage::Selected(generation) => {
                self.selected = generation;
                self.calls += 1;
            }
        }
    }

    fn view(&self, input: &MenuSurface, context: &mut ViewContext<Self>) -> View {
        let generation = self.generation;
        let (title, mut target): (&str, View) = match input {
            MenuSurface::Button => (
                "Reactor UI fixture: menu",
                Button::new()
                    .automation_id("open")
                    .content("Open menu")
                    .into(),
            ),
            MenuSurface::DropDown => (
                "Reactor UI fixture: menu-dropdown",
                DropDownButton::new()
                    .automation_id("open")
                    .content("Open menu")
                    .into(),
            ),
            MenuSurface::MenuBar => (
                "Reactor UI fixture: menu-bar",
                MenuBarItem::new()
                    .automation_id("open")
                    .title("Open menu")
                    .into(),
            ),
            MenuSurface::CommandBar => (
                "Reactor UI fixture: command-bar",
                Button::new()
                    .automation_id("open")
                    .content("Open commands")
                    .into(),
            ),
        };
        if self.attached {
            let label = if self.changed {
                "Changed command"
            } else {
                "Command"
            };
            let callback = context.callback(move |key: Key| {
                assert_eq!(key.as_str(), Some("command"));
                MenuMessage::Selected(generation)
            });
            target = if *input == MenuSurface::CommandBar {
                target.command_bar_flyout(CommandBarFlyout::new(
                    [CommandBarCommand::button("command", label)],
                    [],
                    callback,
                ))
            } else {
                target.menu(Menu::new(
                    [
                        MenuItem::item("command", label),
                        MenuItem::submenu("more", "More", [MenuItem::item("nested", "Nested")]),
                    ],
                    callback,
                ))
            };
        }
        let target = if *input == MenuSurface::MenuBar {
            MenuBar::new().items([target]).into()
        } else {
            target
        };
        context.window_visuals(WindowVisuals::new().client_size(500.0, 500.0));
        context.window_frame(
            title,
            StackPanel::new().margin(20.0).spacing(12.0).children((
                StackPanel::new().keyed_children([keyed(self.owner, target)]),
                Button::new()
                    .automation_id("replace")
                    .content("Replace callback")
                    .on_click(context.message(MenuMessage::Replace)),
                Button::new()
                    .automation_id("change")
                    .content("Change content")
                    .on_click(context.message(MenuMessage::Change)),
                Button::new()
                    .automation_id("remove")
                    .content("Remove menu")
                    .on_click(context.message(MenuMessage::Remove)),
                Button::new()
                    .automation_id("attach")
                    .content("Attach menu")
                    .on_click(context.message(MenuMessage::Attach)),
                Button::new()
                    .automation_id("replace-owner")
                    .content("Replace owner")
                    .on_click(context.message(MenuMessage::ReplaceOwner)),
                TextBlock::new().automation_id("state").text(format!(
                    "Generation {}; attached {}",
                    generation, self.attached
                )),
                TextBlock::new()
                    .automation_id("selection")
                    .text(format!("Selected {}; calls {}", self.selected, self.calls)),
                TextBlock::new()
                    .automation_id("content")
                    .text(if self.changed { "Changed" } else { "Original" }),
                TextBlock::new()
                    .automation_id("owner")
                    .text(format!("Owner {}", self.owner)),
            )),
        )
    }
}

struct TextFixture {
    text: Rc<str>,
}

#[derive(Clone)]
enum TextMessage {
    Changed(Rc<str>),
    Reset,
}

impl Component for TextFixture {
    type Input = ();
    type Message = TextMessage;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self { text: Rc::from("") }
    }

    fn update(&mut self, message: TextMessage, _: &ComponentContext<Self>) {
        self.text = match message {
            TextMessage::Changed(text) => text,
            TextMessage::Reset => Rc::from(""),
        };
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        context.window_frame(
            "Reactor UI fixture: text",
            StackPanel::new().margin(20.0).spacing(12.0).children((
                TextBox::new()
                    .automation_id("input")
                    .text(self.text.clone())
                    .on_text_changed(context.callback(TextMessage::Changed)),
                TextBlock::new()
                    .automation_id("echo")
                    .text(format!("Text: {}", self.text)),
                Button::new()
                    .automation_id("reset")
                    .content("Reset")
                    .on_click(context.message(TextMessage::Reset)),
            )),
        )
    }
}

struct ReorderFixture {
    order: Vec<&'static str>,
}

impl Component for ReorderFixture {
    type Input = ();
    type Message = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            order: vec!["a", "b", "c", "d"],
        }
    }

    fn update(&mut self, _: (), _: &ComponentContext<Self>) {
        self.order = vec!["d", "a", "b", "c"];
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        let rows = self.order.iter().map(|&key| {
            let mut row = TextBox::new().automation_id(key);
            if key == "b" {
                row = row.on_text_changed(context.callback(|_| ()));
            }
            KeyedView::new(key, row)
        });
        context.window_frame(
            "Reactor UI fixture: keyed-reorder",
            StackPanel::new().margin(20.0).spacing(12.0).children((
                TextBlock::new()
                    .automation_id("order")
                    .text(format!("Order {}", self.order.join(" "))),
                StackPanel::new().spacing(8.0).keyed_children(rows),
            )),
        )
    }
}

pub fn run(fixture: &str) -> windows_core::Result<()> {
    match fixture {
        "application" => App::run_application::<application::ApplicationFixture>(())?,
        "background" => App::run_component::<background::BackgroundFixture>(())?,
        "window-replacement" => App::run_component::<application::ReplacingWindow>(false)?,
        "startup-error" => application::startup_error()?,
        "canvas" => App::run_component::<canvas::CanvasFixture>(())?,
        "flyout" => App::run_component::<flyout::FlyoutFixture>(())?,
        "dialog" => App::run_component::<dialog::DialogFixture>(())?,
        "image" => App::run_component::<image::ImageFixture>(())?,
        "keyboard" => App::run_component::<keyboard::KeyboardFixture>(())?,
        "reference" => App::run_component::<reference::ReferenceFixture>(())?,
        "repeater" => App::run_component::<repeater::RepeaterFixture>(())?,
        "retirement" => App::run_component::<retirement::RetirementFixture>(())?,
        "selection" => App::run_component::<selection::SelectionFixture>(())?,
        "timer" => App::run_component::<timer::TimerFixture>(())?,
        "menu" => App::run_component::<MenuFixture>(MenuSurface::Button)?,
        "menu-dropdown" => App::run_component::<MenuFixture>(MenuSurface::DropDown)?,
        "menu-bar" => App::run_component::<MenuFixture>(MenuSurface::MenuBar)?,
        "command-bar" => App::run_component::<MenuFixture>(MenuSurface::CommandBar)?,
        "tray" => App::run_application::<tray::TrayFixture>(())?,
        "window" => App::run_component::<window::WindowFixture>(window::WindowMode::Native)?,
        "window-placement" => App::run_component::<window::PlacementFixture>(())?,
        "window-frame" => App::run_component::<window::WindowFixture>(window::WindowMode::Custom)?,
        "window-frame-policy" => App::run_component_with_policy::<window::WindowFixture>(
            window::WindowMode::CustomPolicy,
            WindowPolicy::new().client_size(480.0, 320.0),
        )?,
        "text" => App::run_component::<TextFixture>(())?,
        "keyed-reorder" => App::run_component::<ReorderFixture>(())?,
        "pointer-capture-mounted" => pointer::run(true, true)?,
        "pointer-capture-added" => pointer::run(true, false)?,
        "pointer-focus-mounted" => pointer::run(false, true)?,
        "pointer-focus-added" => pointer::run(false, false)?,
        "drag-mounted" => drag::run(true)?,
        "drag-added" => drag::run(false)?,
        "exit" => {}
        "panic" => {
            println!("UI fixture panic probe stdout");
            panic!("UI fixture panic probe stderr");
        }
        "hang" => {
            eprintln!("UI fixture hang probe");
            loop {
                std::thread::park();
            }
        }
        _ => panic!("unknown fixture: {fixture}"),
    }
    Ok(())
}
