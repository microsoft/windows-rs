#![windows_subsystem = "console"]

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Duration;
use windows_reactor::{
    App, AppContext, AppProxy, Component, ComponentContext, TextBlock, View, ViewContext,
};
use windows_reactor2 as reactor2;

#[derive(Clone)]
struct Input {
    app: AppProxy,
}

impl PartialEq for Input {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

struct Fixture {
    app: AppProxy,
    runtime: reactor2::Runtime<reactor2::native::WinUiAdapter>,
    window: reactor2::native::NativeWindow,
    iteration: usize,
    input_phase: u8,
    text: Rc<RefCell<Rc<str>>>,
    text_changed: reactor2::Callback<Rc<str>>,
    text_changed_count: Rc<Cell<usize>>,
    replacement_text_changed: reactor2::Callback<Rc<str>>,
    replacement_text_changed_count: Rc<Cell<usize>>,
}

impl Fixture {
    fn declaration(
        iteration: usize,
        text: Rc<str>,
        text_changed: Option<reactor2::Callback<Rc<str>>>,
    ) -> reactor2::Visual {
        let children = [
            reactor2::TreeNode::new("first-child", format!("First child {iteration}"))
                .content(reactor2::TextBlock::new("Nested content")),
            reactor2::TreeNode::new("second-child", "Second child"),
        ];
        let children = if iteration.is_multiple_of(2) {
            children
        } else {
            [children[1].clone(), children[0].clone()]
        };
        let roots = [
            reactor2::TreeNode::new("first", format!("First {iteration}"))
                .expanded(true)
                .content(reactor2::StackPanel::new().children(vec![
                    reactor2::TextBlock::new("Folder").into(),
                    reactor2::TextBlock::new(format!("{} changes", iteration % 7)).into(),
                ]))
                .children(children),
            reactor2::TreeNode::new("second", format!("Second {iteration}"))
                .content(reactor2::TextBlock::new("Leaf")),
        ];
        let roots = if iteration.is_multiple_of(2) {
            reactor2::TreeView::new().nodes(roots)
        } else {
            reactor2::TreeView::new().nodes([roots[1].clone(), roots[0].clone()])
        };
        let items = if iteration.is_multiple_of(2) {
            [
                reactor2::DataItem::new("first", format!("First item {iteration}")),
                reactor2::DataItem::new("second", "Second item"),
            ]
        } else {
            [
                reactor2::DataItem::new("second", "Second item"),
                reactor2::DataItem::new("first", format!("First item {iteration}")),
            ]
        };
        let text_box = reactor2::TextBox::new(text);
        let text_box = if let Some(text_changed) = text_changed {
            text_box.on_text_changed_callback(text_changed)
        } else {
            text_box
        };
        reactor2::StackPanel::new()
            .children(vec![
                text_box.into(),
                reactor2::Border::new()
                    .content(reactor2::TextBlock::new(format!("Iteration {iteration}")))
                    .into(),
                roots.into(),
                reactor2::ListView::new().items(items).into(),
            ])
            .into()
    }

    fn schedule(context: &ComponentContext<Self>) {
        context.spawn_background(|_| {
            std::thread::sleep(Duration::from_millis(20));
        });
    }
}

impl Component for Fixture {
    type Input = Input;
    type Message = ();

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        let text = Rc::new(RefCell::new(Rc::<str>::from("Initial text")));
        let text_changed_count = Rc::new(Cell::new(0));
        let text_for_callback = Rc::clone(&text);
        let count_for_callback = Rc::clone(&text_changed_count);
        let text_changed = reactor2::Callback::new(move |value| {
            *text_for_callback.borrow_mut() = value;
            count_for_callback.set(count_for_callback.get() + 1);
        });
        let replacement_text_changed_count = Rc::new(Cell::new(0));
        let replacement_text_for_callback = Rc::clone(&text);
        let replacement_count_for_callback = Rc::clone(&replacement_text_changed_count);
        let replacement_text_changed = reactor2::Callback::new(move |value| {
            *replacement_text_for_callback.borrow_mut() = value;
            replacement_count_for_callback.set(replacement_count_for_callback.get() + 1);
        });
        let mut runtime = reactor2::Runtime::new(reactor2::native::WinUiAdapter::default());
        let sender = context.sender();
        runtime.adapter_mut().set_event_waker(move || {
            sender.send(());
        });
        runtime
            .update(Self::declaration(
                0,
                Rc::clone(&text.borrow()),
                Some(text_changed.clone()),
            ))
            .unwrap();
        let root = runtime.graph().root().unwrap();
        let window = runtime.adapter().open_window(root).unwrap();
        Self::schedule(context);
        Self {
            app: input.app.clone(),
            runtime,
            window,
            iteration: 0,
            input_phase: 0,
            text,
            text_changed,
            text_changed_count,
            replacement_text_changed,
            replacement_text_changed_count,
        }
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        self.app = input.app.clone();
    }

    fn update(&mut self, _message: Self::Message, context: &ComponentContext<Self>) {
        let mut events = Vec::new();
        self.runtime.drain_events(&mut events).unwrap();
        for event in events {
            event.invoke();
        }
        self.iteration += 1;
        let root = self.runtime.graph().root().unwrap();
        let text_box = self
            .runtime
            .graph()
            .children(root, reactor2::RelationId::Children)
            .unwrap()[0];
        match self.input_phase {
            0 => {
                self.runtime
                    .adapter()
                    .simulate_text_input(text_box, "Native input", 3, 2)
                    .unwrap();
                self.input_phase = 1;
                Self::schedule(context);
                return;
            }
            1 if self.text_changed_count.get() == 0 => {
                Self::schedule(context);
                return;
            }
            1 => {
                assert_eq!(self.text_changed_count.get(), 1);
                assert_eq!(self.text.borrow().as_ref(), "Native input");
                self.input_phase = 2;
            }
            2 => {
                *self.text.borrow_mut() = Rc::from("X");
                self.input_phase = 3;
            }
            3 => {
                assert_eq!(self.text_changed_count.get(), 1);
                assert_eq!(self.text.borrow().as_ref(), "X");
                self.runtime
                    .adapter()
                    .simulate_text_input(text_box, "Stale input", 2, 0)
                    .unwrap();
                self.input_phase = 4;
            }
            5 => {
                assert_eq!(self.text_changed_count.get(), 1);
                assert_eq!(self.replacement_text_changed_count.get(), 1);
                assert_eq!(self.text.borrow().as_ref(), "Replacement input");
                self.input_phase = 6;
            }
            _ => {}
        }
        let text_changed = if self.input_phase >= 4 {
            self.replacement_text_changed.clone()
        } else {
            self.text_changed.clone()
        };
        self.runtime
            .update(Self::declaration(
                self.iteration,
                Rc::clone(&self.text.borrow()),
                Some(text_changed),
            ))
            .unwrap();
        if self.input_phase == 2 {
            assert_eq!(
                self.runtime.adapter().text_box_state(text_box).unwrap(),
                ("Native input".to_string(), 3, 2)
            );
            assert_eq!(self.text_changed_count.get(), 1);
        } else if self.input_phase == 3 {
            assert_eq!(
                self.runtime.adapter().text_box_state(text_box).unwrap(),
                ("X".to_string(), 1, 0)
            );
            assert_eq!(self.text_changed_count.get(), 1);
        } else if self.input_phase == 4 {
            self.runtime
                .adapter()
                .simulate_text_input(text_box, "Replacement input", 5, 0)
                .unwrap();
            self.input_phase = 5;
            Self::schedule(context);
            return;
        }
        self.runtime
            .adapter()
            .validate_graph(self.runtime.graph())
            .unwrap();
        if self.iteration == 100 {
            self.window.close().unwrap();
            self.app.exit().unwrap();
        } else {
            Self::schedule(context);
        }
    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        TextBlock::new()
            .text(format!("Reactor2 TreeView stress: {}", self.iteration))
            .into()
    }
}

fn main() -> windows_core::Result<()> {
    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_secs(15));
        eprintln!("Reactor2 native self-test timed out");
        std::process::exit(1);
    });
    App::run_with(|app: &AppContext| {
        app.open_window(View::component::<Fixture>(Input { app: app.proxy() }))
    })
}
