//! Deterministic model-based reconciliation tests.
//!
//! Each generated transition is checked against an independent model for native order, live
//! control count, typed-reference lifetime, and component cleanup counts.

use std::cell::RefCell;
use std::rc::Rc;

use rustc_hash::FxHashMap;
use test_reactor::{Op, RecordingBackend};
use windows_reactor::{
    Component, Context, Element, ElementRef, ElementRefExt, KeyExt, Prop, PropValue, ProvideExt,
    Reconciler, RenderCx, TextBoxHandle, border, component, error_boundary, memo, text_box, vstack,
};

const ITEM_COUNT: usize = 12;
const SEEDS: u64 = 8;
const STEPS: usize = 200;

#[derive(Clone, PartialEq)]
struct LeafProps {
    id: usize,
    revision: u32,
    bordered: bool,
}

#[derive(Default)]
struct Lifecycle {
    mounts: FxHashMap<usize, u32>,
    cleanups: FxHashMap<usize, u32>,
}

struct ModelLeaf {
    reference: ElementRef<TextBoxHandle>,
    lifecycle: Rc<RefCell<Lifecycle>>,
}

impl Component<LeafProps> for ModelLeaf {
    fn render(&self, props: &LeafProps, cx: &mut RenderCx) -> Element {
        let id = props.id;
        let lifecycle = Rc::clone(&self.lifecycle);
        cx.use_effect_with_cleanup((), move || {
            *lifecycle.borrow_mut().mounts.entry(id).or_default() += 1;
            let lifecycle = Rc::clone(&lifecycle);
            Some(move || *lifecycle.borrow_mut().cleanups.entry(id).or_default() += 1)
        });

        leaf(props, &self.reference)
    }
}

#[derive(Clone)]
struct Item {
    id: usize,
    revision: u32,
    bordered: bool,
    wrapped: bool,
}

impl Item {
    fn props(&self) -> LeafProps {
        LeafProps {
            id: self.id,
            revision: self.revision,
            bordered: self.bordered,
        }
    }

    fn text(&self) -> String {
        format!("{}:{}", self.id, self.revision)
    }
}

struct Model {
    items: Vec<Item>,
    expected_mounts: [u32; ITEM_COUNT],
    expected_cleanups: [u32; ITEM_COUNT],
}

impl Model {
    fn initial(seed: u64) -> Self {
        let items: Vec<Item> = (0..8)
            .map(|id| Item {
                id,
                revision: 0,
                bordered: (seed.rotate_left(id as u32) & 1) != 0,
                wrapped: id % 2 == 0,
            })
            .collect();
        let mut expected_mounts = [0; ITEM_COUNT];
        for item in &items {
            if item.wrapped {
                expected_mounts[item.id] = 1;
            }
        }
        Self {
            items,
            expected_mounts,
            expected_cleanups: [0; ITEM_COUNT],
        }
    }

    fn mutate(&mut self, rng: &mut Rng) {
        match rng.next_usize(7) {
            0 => {
                let absent: Vec<usize> = (0..ITEM_COUNT)
                    .filter(|id| self.items.iter().all(|item| item.id != *id))
                    .collect();
                if let Some(&id) = absent.get(rng.next_usize(absent.len())) {
                    let wrapped = rng.next_bool();
                    if wrapped {
                        self.expected_mounts[id] += 1;
                    }
                    let index = rng.next_usize(self.items.len() + 1);
                    self.items.insert(
                        index,
                        Item {
                            id,
                            revision: rng.next_u32() % 4,
                            bordered: rng.next_bool(),
                            wrapped,
                        },
                    );
                }
            }
            1 if !self.items.is_empty() => {
                let index = rng.next_usize(self.items.len());
                let removed = self.items.remove(index);
                if removed.wrapped {
                    self.expected_cleanups[removed.id] += 1;
                }
            }
            2 if self.items.len() > 1 => {
                let from = rng.next_usize(self.items.len());
                let mut to = rng.next_usize(self.items.len());
                if from == to {
                    to = (to + 1) % self.items.len();
                }
                let item = self.items.remove(from);
                self.items.insert(to, item);
            }
            3 if !self.items.is_empty() => {
                let index = rng.next_usize(self.items.len());
                self.items[index].revision = self.items[index].revision.wrapping_add(1);
            }
            4 if !self.items.is_empty() => {
                let index = rng.next_usize(self.items.len());
                self.items[index].bordered = !self.items[index].bordered;
            }
            5 if !self.items.is_empty() => {
                let index = rng.next_usize(self.items.len());
                let item = &mut self.items[index];
                item.wrapped = !item.wrapped;
                if item.wrapped {
                    self.expected_mounts[item.id] += 1;
                } else {
                    self.expected_cleanups[item.id] += 1;
                }
            }
            _ if self.items.len() > 1 => {
                let amount = rng.next_usize(self.items.len());
                self.items.rotate_left(amount);
            }
            _ => {}
        }
    }
}

struct Harness {
    references: Vec<ElementRef<TextBoxHandle>>,
    lifecycle: Rc<RefCell<Lifecycle>>,
    context: Context<u8>,
}

impl Harness {
    fn new() -> Self {
        Self {
            references: (0..ITEM_COUNT).map(|_| ElementRef::new()).collect(),
            lifecycle: Rc::new(RefCell::new(Lifecycle::default())),
            context: Context::new(0),
        }
    }

    fn tree(&self, model: &Model) -> Element {
        let children: Vec<Element> = model
            .items
            .iter()
            .map(|item| {
                let props = item.props();
                let element = if item.wrapped {
                    let leaf_component = ModelLeaf {
                        reference: self.references[item.id].clone(),
                        lifecycle: Rc::clone(&self.lifecycle),
                    };
                    match item.id % 5 {
                        0 => component(leaf_component, props),
                        1 => memo(leaf_component, props),
                        2 => component(leaf_component, props).provide(&self.context, item.id as u8),
                        3 => error_boundary(component(leaf_component, props), |_| {
                            text_box("fallback").into()
                        }),
                        _ => error_boundary(
                            memo(leaf_component, props).provide(&self.context, item.id as u8),
                            |_| text_box("fallback").into(),
                        ),
                    }
                } else {
                    leaf(&props, &self.references[item.id])
                };
                element.with_key(item.id.to_string())
            })
            .collect();
        vstack(children).into()
    }

    fn assert_state(
        &self,
        model: &Model,
        reconciler: &Reconciler<RecordingBackend>,
        root: windows_reactor::ControlId,
        seed: u64,
        step: usize,
    ) {
        reconciler.assert_consistent();
        reconciler.backend.assert_consistent();

        let actual: Vec<String> = reconciler
            .backend
            .children_of(root)
            .iter()
            .map(|id| descendant_text(&reconciler.backend, *id))
            .collect();
        let expected: Vec<String> = model.items.iter().map(Item::text).collect();
        assert_eq!(
            actual, expected,
            "native order differs at seed {seed}, step {step}"
        );

        let expected_controls = 1 + model
            .items
            .iter()
            .map(|item| if item.bordered { 2 } else { 1 })
            .sum::<usize>();
        assert_eq!(
            reconciler.backend.live_control_count(),
            expected_controls,
            "live control count differs at seed {seed}, step {step}"
        );

        for (id, reference) in self.references.iter().enumerate() {
            let expected_mounted = model.items.iter().any(|item| item.id == id);
            assert_eq!(
                reference.is_mounted(),
                expected_mounted,
                "reference state differs for item {id} at seed {seed}, step {step}"
            );
        }

        let lifecycle = self.lifecycle.borrow();
        for id in 0..ITEM_COUNT {
            assert_eq!(
                lifecycle.mounts.get(&id).copied().unwrap_or_default(),
                model.expected_mounts[id],
                "mount count differs for item {id} at seed {seed}, step {step}"
            );
            assert_eq!(
                lifecycle.cleanups.get(&id).copied().unwrap_or_default(),
                model.expected_cleanups[id],
                "cleanup count differs for item {id} at seed {seed}, step {step}"
            );
        }
    }
}

fn leaf(props: &LeafProps, reference: &ElementRef<TextBoxHandle>) -> Element {
    let leaf: Element = text_box(format!("{}:{}", props.id, props.revision))
        .element_ref(reference)
        .into();
    if props.bordered {
        border(leaf).into()
    } else {
        leaf
    }
}

fn descendant_text(backend: &RecordingBackend, id: windows_reactor::ControlId) -> String {
    if let Some(text) = backend.ops.iter().rev().find_map(|op| match op {
        Op::SetProp {
            id: prop_id,
            prop: Prop::Value,
            value: PropValue::Str(text),
        } if *prop_id == id => Some(text.clone()),
        _ => None,
    }) {
        return text;
    }

    let children = backend.children_of(id);
    assert_eq!(
        children.len(),
        1,
        "modeled wrapper must have one native child"
    );
    descendant_text(backend, children[0])
}

struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Self(seed)
    }

    fn next_u32(&mut self) -> u32 {
        let mut value = self.0;
        value ^= value << 13;
        value ^= value >> 7;
        value ^= value << 17;
        self.0 = value;
        value as u32
    }

    fn next_usize(&mut self, upper: usize) -> usize {
        if upper == 0 {
            0
        } else {
            self.next_u32() as usize % upper
        }
    }

    fn next_bool(&mut self) -> bool {
        self.next_u32() & 1 != 0
    }
}

#[test]
fn generated_transitions_match_model() {
    for seed in 1..=SEEDS {
        let harness = Harness::new();
        let mut model = Model::initial(seed);
        let mut rng = Rng::new(seed * 0x9e37_79b9);
        let mut reconciler = Reconciler::new(RecordingBackend::new());
        let mut tree = harness.tree(&model);
        let root = reconciler
            .reconcile(None, &tree, None, Rc::new(|| {}))
            .unwrap();
        harness.assert_state(&model, &reconciler, root, seed, 0);

        for step in 1..=STEPS {
            model.mutate(&mut rng);
            let next = harness.tree(&model);
            let next_root = reconciler
                .reconcile(Some(&tree), &next, Some(root), Rc::new(|| {}))
                .unwrap();
            assert_eq!(
                next_root, root,
                "root identity changed at seed {seed}, step {step}"
            );
            tree = next;
            harness.assert_state(&model, &reconciler, root, seed, step);
        }

        for item in &model.items {
            if item.wrapped {
                model.expected_cleanups[item.id] += 1;
            }
        }
        reconciler.unmount(root);
        reconciler.backend.assert_consistent();
        assert_eq!(reconciler.backend.live_control_count(), 0);
        assert!(
            harness
                .references
                .iter()
                .all(|reference| !reference.is_mounted())
        );
        let lifecycle = harness.lifecycle.borrow();
        for id in 0..ITEM_COUNT {
            assert_eq!(
                lifecycle.mounts.get(&id).copied().unwrap_or_default(),
                model.expected_mounts[id],
                "final mount count differs for item {id}, seed {seed}"
            );
            assert_eq!(
                lifecycle.cleanups.get(&id).copied().unwrap_or_default(),
                model.expected_cleanups[id],
                "final cleanup count differs for item {id}, seed {seed}"
            );
        }
    }
}
