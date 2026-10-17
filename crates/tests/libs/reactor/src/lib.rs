use std::rc::Rc;
use windows_reactor::*;

mod drag;
mod pointer;

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
    type Input = ();
    type Message = MenuMessage;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
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

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        let generation = self.generation;
        let button = Button::new().automation_id("open").content("Open menu");
        let button: View = if self.attached {
            button.menu(Menu::new(
                [
                    MenuItem::item(
                        "command",
                        if self.changed {
                            "Changed command"
                        } else {
                            "Command"
                        },
                    ),
                    MenuItem::submenu("more", "More", [MenuItem::item("nested", "Nested")]),
                ],
                context.callback(move |key: Key| {
                    assert_eq!(key.as_str(), Some("command"));
                    MenuMessage::Selected(generation)
                }),
            ))
        } else {
            button.into()
        };
        context.window_visuals(WindowVisuals::new().client_size(500.0, 500.0));
        context.window_frame(
            "Reactor UI fixture: menu",
            StackPanel::new().margin(20.0).spacing(12.0).children((
                StackPanel::new().keyed_children([keyed(self.owner, button)]),
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
        "menu" => App::run_component::<MenuFixture>(())?,
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
