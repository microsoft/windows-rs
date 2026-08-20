//! Shared fixtures and helper functions used across the [`super::super`] test
//! categories.

use super::super::*;
use crate::native::*;
use std::cell::Cell;
use std::rc::Rc;

pub(super) fn keyed_text(values: &[&str]) -> Element {
    StackPanel::new()
        .children(
            values
                .iter()
                .map(|value| KeyedElement::new(*value, TextBlock::new().text(*value))),
        )
        .into()
}

pub(super) fn keyed_numbers(values: &[u64]) -> Element {
    StackPanel::new()
        .children(
            values
                .iter()
                .map(|value| KeyedElement::new(*value, TextBlock::new().text(value.to_string()))),
        )
        .into()
}

pub(super) fn representative_tree() -> Element {
    StackPanel::new()
        .spacing(8.0)
        .child(
            "button",
            Button::new()
                .is_enabled(true)
                .on_click(|| {})
                .content(TextBlock::new().text("increment")),
        )
        .into()
}

pub(super) fn arena_keys(pump: &Pump<RecordingRuntime>) -> Vec<Key> {
    pump.tree
        .children(pump.root().unwrap())
        .unwrap()
        .iter()
        .map(|node| pump.tree.key(*node).unwrap().unwrap().clone())
        .collect()
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

pub(super) fn structural_receipt(error: PumpError) -> CommitReceipt {
    let PumpError::StructuralApplyFailed(receipt) = error else {
        panic!("expected structural apply failure");
    };
    receipt
}

pub(super) fn recovered_structure(error: PumpError) -> StructuralRecovery {
    let PumpError::RecoveredStructure(recovery) = error else {
        panic!("expected recovered structure");
    };
    *recovery
}

#[derive(Default)]
pub(super) struct ShortReceiptRuntime {
    inner: RecordingRuntime,
    pub(super) short_next: bool,
}

impl NativeRuntime for ShortReceiptRuntime {
    fn apply(&mut self, commands: &[Command]) -> CommitReceipt {
        let mut receipt = self.inner.apply(commands);
        if self.short_next {
            self.short_next = false;
            receipt.outcomes.pop();
        }
        receipt
    }

    fn reset(&mut self) {
        self.inner.reset();
    }
}

#[derive(Default)]
pub(super) struct EventErrorRuntime {
    pub(super) error: Option<NativeWork<QueuedEventError>>,
    identity: Option<NativeIdentity>,
}

impl NativeRuntime for EventErrorRuntime {
    fn apply(&mut self, commands: &[Command]) -> CommitReceipt {
        CommitReceipt {
            outcomes: vec![CommandOutcome::Applied; commands.len()],
        }
    }

    fn reset(&mut self) {}

    fn set_identity(&mut self, identity: NativeIdentity) {
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
    type Props = String;
    type Message = ();

    fn create(props: &Self::Props, _context: &mut ComponentContext<Self>) -> Self {
        Self {
            text: props.clone(),
        }
    }

    fn changed(&mut self, props: &Self::Props, _context: &mut ComponentContext<Self>) {
        self.text.clone_from(props);
    }

    fn update(&mut self, _message: Self::Message, _context: &mut ComponentContext<Self>) {}

    fn view(&self, _context: &mut ViewContext<Self>) -> View {
        View::native(TextBlock::new().text(self.text.clone()))
    }
}

pub(super) struct Root {
    text: String,
}

impl Component for Root {
    type Props = String;
    type Message = String;

    fn create(props: &Self::Props, context: &mut ComponentContext<Self>) -> Self {
        context.sender().send("message".to_string());
        Self {
            text: props.clone(),
        }
    }

    fn changed(&mut self, props: &Self::Props, _context: &mut ComponentContext<Self>) {
        self.text.clone_from(props);
    }

    fn update(&mut self, message: Self::Message, _context: &mut ComponentContext<Self>) {
        self.text = message;
    }

    fn view(&self, _context: &mut ViewContext<Self>) -> View {
        View::component::<Leaf>(self.text.clone())
    }
}

pub(super) struct List {
    items: Vec<(u64, String)>,
}

impl Component for List {
    type Props = Vec<(u64, String)>;
    type Message = ();

    fn create(props: &Self::Props, _context: &mut ComponentContext<Self>) -> Self {
        Self {
            items: props.clone(),
        }
    }

    fn changed(&mut self, props: &Self::Props, _context: &mut ComponentContext<Self>) {
        self.items.clone_from(props);
    }

    fn update(&mut self, _message: Self::Message, _context: &mut ComponentContext<Self>) {}

    fn view(&self, _context: &mut ViewContext<Self>) -> View {
        View::children(
            StackPanel::new(),
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
    type Props = String;
    type Message = ();

    fn create(props: &Self::Props, _context: &mut ComponentContext<Self>) -> Self {
        Self {
            text: props.clone(),
        }
    }

    fn changed(&mut self, props: &Self::Props, _context: &mut ComponentContext<Self>) {
        self.text.clone_from(props);
    }

    fn update(&mut self, _message: Self::Message, _context: &mut ComponentContext<Self>) {}

    fn view(&self, _context: &mut ViewContext<Self>) -> View {
        View::native(TextBlock::new().text(format!("alt:{}", self.text)))
    }
}

pub(super) struct MixedList {
    alt: bool,
}

impl Component for MixedList {
    type Props = bool;
    type Message = bool;

    fn create(props: &Self::Props, _context: &mut ComponentContext<Self>) -> Self {
        Self { alt: *props }
    }

    fn changed(&mut self, props: &Self::Props, _context: &mut ComponentContext<Self>) {
        self.alt = *props;
    }

    fn update(&mut self, message: Self::Message, _context: &mut ComponentContext<Self>) {
        self.alt = message;
    }

    fn view(&self, _context: &mut ViewContext<Self>) -> View {
        let child = if self.alt {
            View::component::<AltLeaf>("value".to_string())
        } else {
            View::component::<Leaf>("value".to_string())
        };
        View::children(StackPanel::new(), [KeyedView::new(1u64, child)])
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
    type Props = Rc<Cell<u32>>;
    type Message = ();

    fn create(props: &Self::Props, _context: &mut ComponentContext<Self>) -> Self {
        Self {
            views: Rc::clone(props),
        }
    }

    fn changed(&mut self, props: &Self::Props, _context: &mut ComponentContext<Self>) {
        self.views = Rc::clone(props);
    }

    fn update(&mut self, _message: Self::Message, _context: &mut ComponentContext<Self>) {}

    fn view(&self, _context: &mut ViewContext<Self>) -> View {
        self.views.set(self.views.get() + 1);
        View::native(TextBlock::new())
    }
}

pub(super) struct CountingParent {
    counts: ViewCounts,
}

impl Component for CountingParent {
    type Props = ViewCounts;
    type Message = ();

    fn create(props: &Self::Props, _context: &mut ComponentContext<Self>) -> Self {
        Self {
            counts: props.clone(),
        }
    }

    fn changed(&mut self, props: &Self::Props, _context: &mut ComponentContext<Self>) {
        self.counts = props.clone();
    }

    fn update(&mut self, _message: Self::Message, _context: &mut ComponentContext<Self>) {}

    fn view(&self, _context: &mut ViewContext<Self>) -> View {
        self.counts.parent.set(self.counts.parent.get() + 1);
        View::component::<CountingChild>(Rc::clone(&self.counts.child))
    }
}
