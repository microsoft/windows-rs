//! Generated transitions checked against an independent semantic model.

use super::super::*;
use crate::native::*;
use std::cell::RefCell;
use std::rc::Rc;

const ITEM_COUNT: usize = 12;
const SEEDS: u64 = 8;
const STEPS: usize = 200;

#[derive(Default)]
struct Lifecycle {
    cleanups: [u32; ITEM_COUNT],
    mounts: [u32; ITEM_COUNT],
}

#[derive(Clone)]
struct LeafInput {
    id: usize,
    lifecycle: Rc<RefCell<Lifecycle>>,
    reference: ElementRef<TextBox>,
    revision: u32,
}

impl PartialEq for LeafInput {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.revision == other.revision
            && Rc::ptr_eq(&self.lifecycle, &other.lifecycle)
    }
}

struct ModelLeaf {
    input: LeafInput,
}

impl Component for ModelLeaf {
    type Input = LeafInput;
    type Message = ();

    fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            input: input.clone(),
        }
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        self.input = input.clone();
    }

    fn update(&mut self, (): (), _context: &ComponentContext<Self>) {}

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let id = self.input.id;
        let lifecycle = Rc::clone(&self.input.lifecycle);
        context.use_effect("lifecycle", (), move || {
            lifecycle.borrow_mut().mounts[id] += 1;
            Some(Box::new(move || {
                lifecycle.borrow_mut().cleanups[id] += 1;
            }))
        });
        TextBox::new()
            .text(format!("{}:{}", self.input.id, self.input.revision))
            .element_ref(&self.input.reference)
            .into()
    }
}

#[derive(Clone)]
struct Item {
    component: bool,
    id: usize,
    revision: u32,
}

struct Model {
    cleanups: [u32; ITEM_COUNT],
    items: Vec<Item>,
    mounts: [u32; ITEM_COUNT],
}

impl Model {
    fn initial(seed: u64) -> Self {
        let items = (0..8)
            .map(|id| Item {
                component: (seed.rotate_left(id as u32) & 1) != 0,
                id,
                revision: 0,
            })
            .collect::<Vec<_>>();
        let mut mounts = [0; ITEM_COUNT];
        for item in &items {
            if item.component {
                mounts[item.id] += 1;
            }
        }
        Self {
            cleanups: [0; ITEM_COUNT],
            items,
            mounts,
        }
    }

    fn mutate(&mut self, random: &mut Random) {
        match random.index(6) {
            0 => {
                let absent = (0..ITEM_COUNT)
                    .filter(|id| self.items.iter().all(|item| item.id != *id))
                    .collect::<Vec<_>>();
                if let Some(&id) = absent.get(random.index(absent.len())) {
                    let component = random.boolean();
                    if component {
                        self.mounts[id] += 1;
                    }
                    let index = random.index(self.items.len() + 1);
                    self.items.insert(
                        index,
                        Item {
                            component,
                            id,
                            revision: random.next() % 4,
                        },
                    );
                }
            }
            1 if !self.items.is_empty() => {
                let index = random.index(self.items.len());
                let removed = self.items.remove(index);
                if removed.component {
                    self.cleanups[removed.id] += 1;
                }
            }
            2 if self.items.len() > 1 => {
                let from = random.index(self.items.len());
                let mut to = random.index(self.items.len());
                if from == to {
                    to = (to + 1) % self.items.len();
                }
                let item = self.items.remove(from);
                self.items.insert(to, item);
            }
            3 if !self.items.is_empty() => {
                let index = random.index(self.items.len());
                self.items[index].revision = self.items[index].revision.wrapping_add(1);
            }
            4 if !self.items.is_empty() => {
                let index = random.index(self.items.len());
                let item = &mut self.items[index];
                item.component = !item.component;
                if item.component {
                    self.mounts[item.id] += 1;
                } else {
                    self.cleanups[item.id] += 1;
                }
            }
            _ if self.items.len() > 1 => {
                let amount = random.index(self.items.len());
                self.items.rotate_left(amount);
            }
            _ => {}
        }
    }
}

struct Harness {
    lifecycle: Rc<RefCell<Lifecycle>>,
    references: Vec<ElementRef<TextBox>>,
}

impl Harness {
    fn new() -> Self {
        Self {
            lifecycle: Rc::new(RefCell::new(Lifecycle::default())),
            references: (0..ITEM_COUNT).map(|_| ElementRef::new()).collect(),
        }
    }

    fn view(&self, model: &Model) -> View {
        StackPanel::new().keyed_children(model.items.iter().map(|item| {
            let input = LeafInput {
                id: item.id,
                lifecycle: Rc::clone(&self.lifecycle),
                reference: self.references[item.id].clone(),
                revision: item.revision,
            };
            let child = if item.component {
                View::component::<ModelLeaf>(input)
            } else {
                TextBox::new()
                    .text(format!("{}:{}", item.id, item.revision))
                    .element_ref(&self.references[item.id])
                    .into()
            };
            KeyedView::new(item.id, child)
        }))
    }

    fn assert_state(
        &self,
        model: &Model,
        pump: &mut Pump<RecordingRuntime>,
        seed: u64,
        step: usize,
    ) {
        let root = pump.root().unwrap();
        let actual = pump
            .runtime()
            .node(root)
            .unwrap()
            .children()
            .iter()
            .map(|child| {
                let PropertyValue::Str(text) = pump
                    .runtime()
                    .node(*child)
                    .unwrap()
                    .property(PropertyId::TextBoxText)
                    .unwrap()
                else {
                    panic!("expected TextBox text");
                };
                text.clone()
            })
            .collect::<Vec<_>>();
        let expected = model
            .items
            .iter()
            .map(|item| format!("{}:{}", item.id, item.revision))
            .collect::<Vec<_>>();
        assert_eq!(actual, expected, "native order at seed {seed}, step {step}");

        for (id, reference) in self.references.iter().enumerate() {
            let expected = model.items.iter().any(|item| item.id == id);
            assert_eq!(
                reference.request_focus(),
                expected,
                "reference state for item {id} at seed {seed}, step {step}"
            );
        }
        assert_eq!(pump.process_imperatives(), Ok(model.items.len()));

        let lifecycle = self.lifecycle.borrow();
        assert_eq!(
            lifecycle.mounts, model.mounts,
            "mount counts at seed {seed}, step {step}"
        );
        assert_eq!(
            lifecycle.cleanups, model.cleanups,
            "cleanup counts at seed {seed}, step {step}"
        );
    }
}

struct Random(u64);

impl Random {
    fn new(seed: u64) -> Self {
        Self(seed)
    }

    fn next(&mut self) -> u32 {
        let mut value = self.0;
        value ^= value << 13;
        value ^= value >> 7;
        value ^= value << 17;
        self.0 = value;
        value as u32
    }

    fn index(&mut self, upper: usize) -> usize {
        if upper == 0 {
            0
        } else {
            self.next() as usize % upper
        }
    }

    fn boolean(&mut self) -> bool {
        self.next() & 1 != 0
    }
}

#[test]
fn generated_transitions_match_semantic_model() {
    for seed in 1..=SEEDS {
        let harness = Harness::new();
        let mut model = Model::initial(seed);
        let mut random = Random::new(seed * 0x9e37_79b9);
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount_view(harness.view(&model)).unwrap();
        let root = pump.root();
        harness.assert_state(&model, &mut pump, seed, 0);

        for step in 1..=STEPS {
            model.mutate(&mut random);
            pump.update_view(harness.view(&model)).unwrap();
            assert_eq!(
                pump.root(),
                root,
                "root identity at seed {seed}, step {step}"
            );
            harness.assert_state(&model, &mut pump, seed, step);
        }

        for item in &model.items {
            if item.component {
                model.cleanups[item.id] += 1;
            }
        }
        pump.shutdown();
        assert!(pump.runtime().is_empty());
        assert!(
            harness
                .references
                .iter()
                .all(|reference| !reference.request_focus())
        );
        assert_eq!(harness.lifecycle.borrow().mounts, model.mounts);
        assert_eq!(harness.lifecycle.borrow().cleanups, model.cleanups);
    }
}
