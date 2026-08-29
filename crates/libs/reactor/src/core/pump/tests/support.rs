//! Shared fixtures and helper functions used across the [`super::super`] test
//! categories.

use super::super::*;
use std::cell::Cell;
use std::rc::Rc;

pub(super) fn keyed_text(values: &[&str]) -> Element {
    StackPanel::new()
        .native_children(
            values
                .iter()
                .map(|value| KeyedElement::new(*value, TextBlock::new().text(*value))),
        )
        .into()
}

pub(super) fn keyed_numbers(values: &[u64]) -> Element {
    StackPanel::new()
        .native_children(
            values
                .iter()
                .map(|value| KeyedElement::new(*value, TextBlock::new().text(value.to_string()))),
        )
        .into()
}

pub(super) fn representative_tree() -> Element {
    StackPanel::new()
        .spacing(8.0)
        .native_child(
            "button",
            Button::new()
                .is_enabled(true)
                .on_click(|| {})
                .native_content(TextBlock::new().text("increment")),
        )
        .into()
}

pub(super) fn recorded_text(runtime: &RecordingRuntime, root: NodeId) -> Vec<String> {
    runtime
        .node(root)
        .unwrap()
        .children()
        .iter()
        .map(|child| {
            let PropertyValue::Str(value) = runtime
                .node(*child)
                .unwrap()
                .property(PropertyId::TextBlockText)
                .unwrap()
            else {
                panic!("expected text");
            };
            value.clone()
        })
        .collect()
}

#[derive(Default)]
pub(super) struct EventErrorRuntime {
    pub(super) error: Option<NativeWork<QueuedEventError>>,
    identity: Option<WindowToken>,
}

impl NativeRuntime for EventErrorRuntime {
    fn apply(&mut self, _commands: &[Command]) -> Result<(), NativeApplyError> {
        Ok(())
    }

    fn reset(&mut self) {}

    fn set_identity(&mut self, identity: WindowToken) {
        self.identity = Some(identity);
    }

    fn drain_event_errors(&mut self) -> Vec<NativeWork<QueuedEventError>> {
        self.error.take().into_iter().collect()
    }
}

pub(super) struct Leaf {
    text: String,
}

impl Component for Leaf {
    type Input = String;
    type Message = ();

    fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            text: input.clone(),
        }
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        self.text.clone_from(input);
    }

    fn update(&mut self, _message: Self::Message, _context: &ComponentContext<Self>) {}

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        View::native(TextBlock::new().text(self.text.clone()))
    }
}

pub(super) struct Root {
    text: String,
}

impl Component for Root {
    type Input = String;
    type Message = String;

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        context.sender().send("message".to_string());
        Self {
            text: input.clone(),
        }
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        self.text.clone_from(input);
    }

    fn update(&mut self, message: Self::Message, _context: &ComponentContext<Self>) {
        self.text = message;
    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        View::component::<Leaf>(self.text.clone())
    }
}

pub(super) struct List {
    items: Vec<(u64, String)>,
}

impl Component for List {
    type Input = Vec<(u64, String)>;
    type Message = ();

    fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            items: input.clone(),
        }
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        self.items.clone_from(input);
    }

    fn update(&mut self, _message: Self::Message, _context: &ComponentContext<Self>) {}

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        StackPanel::new().keyed_children(
            self.items
                .iter()
                .map(|(key, text)| KeyedView::new(*key, View::component::<Leaf>(text.clone()))),
        )
    }
}

pub(super) struct AltLeaf {
    text: String,
}

impl Component for AltLeaf {
    type Input = String;
    type Message = ();

    fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            text: input.clone(),
        }
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        self.text.clone_from(input);
    }

    fn update(&mut self, _message: Self::Message, _context: &ComponentContext<Self>) {}

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        View::native(TextBlock::new().text(format!("alt:{}", self.text)))
    }
}

pub(super) struct MixedList {
    alt: bool,
}

impl Component for MixedList {
    type Input = bool;
    type Message = bool;

    fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self { alt: *input }
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        self.alt = *input;
    }

    fn update(&mut self, message: Self::Message, _context: &ComponentContext<Self>) {
        self.alt = message;
    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        let child = if self.alt {
            View::component::<AltLeaf>("value".to_string())
        } else {
            View::component::<Leaf>("value".to_string())
        };
        StackPanel::new().keyed_children([KeyedView::new(1u64, child)])
    }
}

#[derive(Clone)]
pub(super) struct ViewCounts {
    pub(super) child: Rc<Cell<u32>>,
    pub(super) parent: Rc<Cell<u32>>,
}

impl PartialEq for ViewCounts {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.child, &other.child) && Rc::ptr_eq(&self.parent, &other.parent)
    }
}

pub(super) struct CountingChild {
    views: Rc<Cell<u32>>,
}

impl Component for CountingChild {
    type Input = Rc<Cell<u32>>;
    type Message = ();

    fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            views: Rc::clone(input),
        }
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        self.views = Rc::clone(input);
    }

    fn update(&mut self, _message: Self::Message, _context: &ComponentContext<Self>) {}

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        self.views.set(self.views.get() + 1);
        View::native(TextBlock::new())
    }
}

pub(super) struct CountingParent {
    counts: ViewCounts,
}

impl Component for CountingParent {
    type Input = ViewCounts;
    type Message = ();

    fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            counts: input.clone(),
        }
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        self.counts = input.clone();
    }

    fn update(&mut self, _message: Self::Message, _context: &ComponentContext<Self>) {}

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        self.counts.parent.set(self.counts.parent.get() + 1);
        View::component::<CountingChild>(Rc::clone(&self.counts.child))
    }
}
