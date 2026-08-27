//! Component lifecycle and effect commit/cleanup contract tests for [`Pump`].

use super::super::*;
use super::support::*;
use crate::native::*;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum EffectPlan {
    Conditional { show_a: bool },
    Duplicate,
    Ordered { reverse: bool },
    Single { dependency: u32 },
}

#[derive(Clone)]
struct KeyedEffectInput {
    log: Rc<RefCell<Vec<String>>>,
    plan: EffectPlan,
}

impl PartialEq for KeyedEffectInput {
    fn eq(&self, other: &Self) -> bool {
        self.plan == other.plan && Rc::ptr_eq(&self.log, &other.log)
    }
}

struct KeyedEffects;

impl KeyedEffects {
    fn unit_effect(
        context: &mut ViewContext<Self>,
        key: &'static str,
        label: &'static str,
        log: &Rc<RefCell<Vec<String>>>,
    ) {
        let log = Rc::clone(log);
        context.use_effect(key, (), move || {
            log.borrow_mut().push(format!("setup {label}"));
            Some(Box::new(move || {
                log.borrow_mut().push(format!("cleanup {label}"));
            }))
        });
    }

    fn value_effect(
        context: &mut ViewContext<Self>,
        key: &'static str,
        dependency: u32,
        log: &Rc<RefCell<Vec<String>>>,
    ) {
        let log = Rc::clone(log);
        context.use_effect(key, dependency, move || {
            log.borrow_mut().push(format!("setup D {dependency}"));
            Some(Box::new(move || {
                log.borrow_mut().push(format!("cleanup D {dependency}"));
            }))
        });
    }
}

impl Component for KeyedEffects {
    type Message = ();
    type Input = KeyedEffectInput;

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn update(&mut self, (): (), _context: &ComponentContext<Self>) {}

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        match input.plan {
            EffectPlan::Conditional { show_a } => {
                if show_a {
                    Self::unit_effect(context, "a", "A", &input.log);
                }
                Self::unit_effect(context, "b", "B", &input.log);
            }
            EffectPlan::Duplicate => {
                Self::value_effect(context, "duplicate", 1, &input.log);
                Self::value_effect(context, "duplicate", 2, &input.log);
            }
            EffectPlan::Ordered { reverse: false } => {
                Self::unit_effect(context, "a", "A", &input.log);
                Self::unit_effect(context, "b", "B", &input.log);
            }
            EffectPlan::Ordered { reverse: true } => {
                Self::unit_effect(context, "b", "B", &input.log);
                Self::unit_effect(context, "a", "A", &input.log);
            }
            EffectPlan::Single { dependency } => {
                Self::value_effect(context, "duplicate", dependency, &input.log);
            }
        }
        TextBlock::new().text(format!("{:?}", input.plan)).into()
    }
}

#[test]
fn conditional_effect_omission_cleans_a_and_leaves_b_active() {
    let log = Rc::new(RefCell::new(Vec::new()));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<KeyedEffects>(KeyedEffectInput {
        log: Rc::clone(&log),
        plan: EffectPlan::Conditional { show_a: true },
    }))
    .unwrap();
    assert_eq!(&*log.borrow(), &["setup A", "setup B"]);

    pump.update_view(View::component::<KeyedEffects>(KeyedEffectInput {
        log: Rc::clone(&log),
        plan: EffectPlan::Conditional { show_a: false },
    }))
    .unwrap();
    assert_eq!(&*log.borrow(), &["setup A", "setup B", "cleanup A"]);

    pump.shutdown();
    assert_eq!(
        &*log.borrow(),
        &["setup A", "setup B", "cleanup A", "cleanup B"]
    );
}

#[test]
fn reordering_effect_keys_does_not_cleanup_or_setup() {
    let log = Rc::new(RefCell::new(Vec::new()));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<KeyedEffects>(KeyedEffectInput {
        log: Rc::clone(&log),
        plan: EffectPlan::Ordered { reverse: false },
    }))
    .unwrap();
    assert_eq!(&*log.borrow(), &["setup A", "setup B"]);

    pump.update_view(View::component::<KeyedEffects>(KeyedEffectInput {
        log: Rc::clone(&log),
        plan: EffectPlan::Ordered { reverse: true },
    }))
    .unwrap();
    assert_eq!(&*log.borrow(), &["setup A", "setup B"]);

    pump.shutdown();
    assert_eq!(
        &*log.borrow(),
        &["setup A", "setup B", "cleanup A", "cleanup B"]
    );
}

#[test]
fn duplicate_effect_key_rejects_before_native_mutation_or_pending_setup() {
    let log = Rc::new(RefCell::new(Vec::new()));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<KeyedEffects>(KeyedEffectInput {
        log: Rc::clone(&log),
        plan: EffectPlan::Single { dependency: 0 },
    }))
    .unwrap();
    let native_batches = pump.runtime().batches();
    assert_eq!(&*log.borrow(), &["setup D 0"]);

    assert_eq!(
        pump.update_view(View::component::<KeyedEffects>(KeyedEffectInput {
            log: Rc::clone(&log),
            plan: EffectPlan::Duplicate,
        })),
        Err(PumpError::DuplicateEffectKey(EffectKey::from("duplicate")))
    );
    assert_eq!(pump.runtime().batches(), native_batches);
    assert_eq!(&*log.borrow(), &["setup D 0"]);

    pump.update_view(View::component::<KeyedEffects>(KeyedEffectInput {
        log: Rc::clone(&log),
        plan: EffectPlan::Single { dependency: 0 },
    }))
    .unwrap();
    assert_eq!(&*log.borrow(), &["setup D 0"]);

    pump.shutdown();
    assert_eq!(&*log.borrow(), &["setup D 0", "cleanup D 0"]);
}

#[test]
fn dropping_component_pump_cleans_effects_before_native_reset() {
    struct DropRuntime {
        inner: RecordingRuntime,
        log: Rc<RefCell<Vec<&'static str>>>,
    }

    impl NativeRuntime for DropRuntime {
        fn apply(&mut self, commands: &[Command]) -> Result<(), NativeApplyError> {
            self.inner.apply(commands)
        }

        fn reset(&mut self) {
            self.log.borrow_mut().push("reset");
            self.inner.reset();
        }
    }

    #[derive(Clone)]
    struct Input(Rc<RefCell<Vec<&'static str>>>);

    impl PartialEq for Input {
        fn eq(&self, other: &Self) -> bool {
            Rc::ptr_eq(&self.0, &other.0)
        }
    }

    struct DropEffect(Input);

    impl Component for DropEffect {
        type Message = ();
        type Input = Input;

        fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
            Self(input.clone())
        }

        fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
            self.0 = input.clone();
        }

        fn update(&mut self, _message: Self::Message, _context: &ComponentContext<Self>) {}

        fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
            let log = Rc::clone(&self.0.0);
            context.use_effect("drop", (), move || {
                Some(Box::new(move || log.borrow_mut().push("cleanup")))
            });
            View::native(TextBlock::new())
        }
    }

    let log = Rc::new(RefCell::new(Vec::new()));
    let mut pump = Pump::new(DropRuntime {
        inner: RecordingRuntime::default(),
        log: Rc::clone(&log),
    });
    pump.mount_view(View::component::<DropEffect>(Input(Rc::clone(&log))))
        .unwrap();
    drop(pump);

    assert_eq!(&*log.borrow(), &["cleanup", "reset"]);
}

#[test]
fn component_effects_commit_after_mount_and_cleanup_once() {
    #[derive(Clone)]
    struct Input {
        log: Rc<RefCell<Vec<String>>>,
        sender: Rc<RefCell<Option<LocalSender<u32>>>>,
    }

    impl PartialEq for Input {
        fn eq(&self, other: &Self) -> bool {
            Rc::ptr_eq(&self.log, &other.log) && Rc::ptr_eq(&self.sender, &other.sender)
        }
    }

    struct EffectComponent {
        log: Rc<RefCell<Vec<String>>>,
        value: u32,
    }

    impl Component for EffectComponent {
        type Message = u32;
        type Input = Input;

        fn create(input: &Input, cx: &ComponentContext<Self>) -> Self {
            *input.sender.borrow_mut() = Some(cx.sender());
            Self {
                log: Rc::clone(&input.log),
                value: 0,
            }
        }

        fn update(&mut self, message: u32, _cx: &ComponentContext<Self>) {
            self.value = message;
        }

        fn input_changed(&mut self, _input: &Input, _cx: &ComponentContext<Self>) {}

        fn view(&self, _input: &Self::Input, cx: &mut ViewContext<Self>) -> View {
            let log = Rc::clone(&self.log);
            let value = self.value;
            cx.use_effect("value", value, move || {
                log.borrow_mut().push(format!("setup {value}"));
                Some(Box::new(move || {
                    log.borrow_mut().push(format!("cleanup {value}"));
                }))
            });
            Element::from(TextBlock::new().text(value.to_string())).into()
        }
    }

    let log = Rc::new(RefCell::new(Vec::new()));
    let sender = Rc::new(RefCell::new(None));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<EffectComponent>(Input {
        log: Rc::clone(&log),
        sender: Rc::clone(&sender),
    }))
    .unwrap();
    assert_eq!(&*log.borrow(), &["setup 0"]);

    sender.borrow().as_ref().unwrap().send(1);
    pump.dispatch_components(1).unwrap();
    assert_eq!(&*log.borrow(), &["setup 0", "cleanup 0", "setup 1"]);

    pump.shutdown();
    assert_eq!(
        &*log.borrow(),
        &["setup 0", "cleanup 0", "setup 1", "cleanup 1"]
    );
    drop(pump);
    assert_eq!(
        &*log.borrow(),
        &["setup 0", "cleanup 0", "setup 1", "cleanup 1"]
    );
}

#[test]
fn full_tree_recomposition_preserves_unchanged_effects_and_replaces_changed_effects() {
    #[derive(Clone)]
    struct Input {
        log: Rc<RefCell<Vec<String>>>,
        sender: Rc<RefCell<Option<LocalSender<u32>>>>,
    }

    impl PartialEq for Input {
        fn eq(&self, other: &Self) -> bool {
            Rc::ptr_eq(&self.log, &other.log) && Rc::ptr_eq(&self.sender, &other.sender)
        }
    }

    struct EffectComponent {
        input: Input,
        value: u32,
    }

    impl Component for EffectComponent {
        type Message = u32;
        type Input = Input;

        fn create(input: &Input, context: &ComponentContext<Self>) -> Self {
            *input.sender.borrow_mut() = Some(context.sender());
            Self {
                input: input.clone(),
                value: 0,
            }
        }

        fn input_changed(&mut self, input: &Input, _context: &ComponentContext<Self>) {
            self.input = input.clone();
        }

        fn update(&mut self, value: u32, _context: &ComponentContext<Self>) {
            self.value = value;
        }

        fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
            let stable_log = Rc::clone(&self.input.log);
            context.use_effect("stable", (), move || {
                stable_log.borrow_mut().push("stable setup".to_string());
                Some(Box::new(move || {
                    stable_log.borrow_mut().push("stable cleanup".to_string());
                }))
            });
            let changed_log = Rc::clone(&self.input.log);
            let value = self.value;
            context.use_effect("changed", value, move || {
                changed_log
                    .borrow_mut()
                    .push(format!("changed setup {value}"));
                Some(Box::new(move || {
                    changed_log
                        .borrow_mut()
                        .push(format!("changed cleanup {value}"));
                }))
            });
            Button::new().content(TextBlock::new().text(value.to_string()))
        }
    }

    let log = Rc::new(RefCell::new(Vec::new()));
    let sender = Rc::new(RefCell::new(None));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<EffectComponent>(Input {
        log: Rc::clone(&log),
        sender: Rc::clone(&sender),
    }))
    .unwrap();
    assert_eq!(&*log.borrow(), &["stable setup", "changed setup 0"]);

    assert!(sender.borrow().as_ref().unwrap().send(1));
    pump.dispatch_components(1).unwrap();
    assert_eq!(
        &*log.borrow(),
        &[
            "stable setup",
            "changed setup 0",
            "changed cleanup 0",
            "changed setup 1",
        ]
    );

    pump.shutdown();
    assert_eq!(
        &*log.borrow(),
        &[
            "stable setup",
            "changed setup 0",
            "changed cleanup 0",
            "changed setup 1",
            "changed cleanup 1",
            "stable cleanup",
        ]
    );
}

#[test]
fn component_effect_setup_follows_parent_first_tree_order() {
    #[derive(Clone)]
    struct Input {
        label: &'static str,
        log: Rc<RefCell<Vec<&'static str>>>,
    }

    impl PartialEq for Input {
        fn eq(&self, other: &Self) -> bool {
            self.label == other.label && Rc::ptr_eq(&self.log, &other.log)
        }
    }

    struct Child(Input);

    impl Component for Child {
        type Message = ();
        type Input = Input;

        fn create(input: &Input, _cx: &ComponentContext<Self>) -> Self {
            Self(input.clone())
        }

        fn input_changed(&mut self, input: &Input, _cx: &ComponentContext<Self>) {
            self.0 = input.clone();
        }

        fn update(&mut self, (): (), _cx: &ComponentContext<Self>) {}

        fn view(&self, _input: &Self::Input, cx: &mut ViewContext<Self>) -> View {
            let label = self.0.label;
            let log = Rc::clone(&self.0.log);
            cx.use_effect("child", (), move || {
                log.borrow_mut().push(label);
                None
            });
            View::native(TextBlock::new())
        }
    }

    struct Parent(Rc<RefCell<Vec<&'static str>>>);

    impl Component for Parent {
        type Message = ();
        type Input = Rc<RefCell<Vec<&'static str>>>;

        fn create(input: &Self::Input, _cx: &ComponentContext<Self>) -> Self {
            Self(Rc::clone(input))
        }

        fn input_changed(&mut self, input: &Self::Input, _cx: &ComponentContext<Self>) {
            self.0 = Rc::clone(input);
        }

        fn update(&mut self, (): (), _cx: &ComponentContext<Self>) {}

        fn view(&self, _input: &Self::Input, cx: &mut ViewContext<Self>) -> View {
            let log = Rc::clone(&self.0);
            cx.use_effect("parent", (), move || {
                log.borrow_mut().push("parent");
                None
            });
            StackPanel::new().children((
                View::component::<Child>(Input {
                    label: "a",
                    log: Rc::clone(&self.0),
                }),
                View::component::<Child>(Input {
                    label: "b",
                    log: Rc::clone(&self.0),
                }),
            ))
        }
    }

    let log = Rc::new(RefCell::new(Vec::new()));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<Parent>(Rc::clone(&log)))
        .unwrap();

    assert_eq!(&*log.borrow(), &["parent", "a", "b"]);
}

#[test]
fn component_host_treats_initial_property_failure_as_fatal() {
    let mut probe = Pump::new(RecordingRuntime::default());
    probe
        .mount_view(View::component::<Leaf>("value".to_string()))
        .unwrap();
    let failed = probe.runtime().commands()[0]
        .iter()
        .position(|command| matches!(command, Command::SetProperty { .. }))
        .unwrap();

    let mut runtime = RecordingRuntime::default();
    runtime.fail_at(failed);
    let mut pump = Pump::new(runtime);
    assert!(matches!(
        pump.mount_view(View::component::<Leaf>("value".to_string())),
        Err(PumpError::NativeApplyFailed(_))
    ));
    assert!(pump.poisoned());
    assert!(!pump.native_work_pending());
}

#[test]
fn fatal_component_apply_does_not_commit_pending_effects() {
    #[derive(Clone)]
    struct Input {
        alternate: bool,
        log: Rc<RefCell<Vec<String>>>,
    }

    impl PartialEq for Input {
        fn eq(&self, other: &Self) -> bool {
            self.alternate == other.alternate && Rc::ptr_eq(&self.log, &other.log)
        }
    }

    struct EffectComponent(Input);

    impl Component for EffectComponent {
        type Message = ();
        type Input = Input;

        fn create(input: &Input, _cx: &ComponentContext<Self>) -> Self {
            Self(input.clone())
        }

        fn update(&mut self, _message: (), _cx: &ComponentContext<Self>) {}

        fn input_changed(&mut self, input: &Input, _cx: &ComponentContext<Self>) {
            self.0 = input.clone();
        }

        fn view(&self, _input: &Self::Input, cx: &mut ViewContext<Self>) -> View {
            let alternate = self.0.alternate;
            let log = Rc::clone(&self.0.log);
            cx.use_effect("replace", alternate, move || {
                log.borrow_mut().push(format!("setup {alternate}"));
                Some(Box::new(move || {
                    log.borrow_mut().push(format!("cleanup {alternate}"));
                }))
            });
            if alternate {
                Element::from(Button::new()).into()
            } else {
                Element::from(TextBlock::new()).into()
            }
        }
    }

    let log = Rc::new(RefCell::new(Vec::new()));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<EffectComponent>(Input {
        alternate: false,
        log: Rc::clone(&log),
    }))
    .unwrap();
    pump.runtime_mut().fail_after(0, 0);
    pump.runtime_mut().fail_after(1, 0);

    assert!(matches!(
        pump.update_view(View::component::<EffectComponent>(Input {
            alternate: true,
            log: Rc::clone(&log),
        })),
        Err(PumpError::NativeApplyFailed(_))
    ));
    assert_eq!(&*log.borrow(), &["setup false", "cleanup false"]);
}

#[test]
fn retired_component_effects_cleanup_child_first() {
    #[derive(Clone)]
    struct Input {
        child: bool,
        log: Rc<RefCell<Vec<&'static str>>>,
        name: &'static str,
    }

    impl PartialEq for Input {
        fn eq(&self, other: &Self) -> bool {
            self.child == other.child
                && self.name == other.name
                && Rc::ptr_eq(&self.log, &other.log)
        }
    }

    struct EffectTree(Input);

    impl Component for EffectTree {
        type Message = ();
        type Input = Input;

        fn create(input: &Input, _cx: &ComponentContext<Self>) -> Self {
            Self(input.clone())
        }

        fn update(&mut self, _message: (), _cx: &ComponentContext<Self>) {}

        fn input_changed(&mut self, input: &Input, _cx: &ComponentContext<Self>) {
            self.0 = input.clone();
        }

        fn view(&self, _input: &Self::Input, cx: &mut ViewContext<Self>) -> View {
            let cleanup = self.0.name;
            let log = Rc::clone(&self.0.log);
            cx.use_effect("lifecycle", (), move || {
                Some(Box::new(move || {
                    log.borrow_mut().push(cleanup);
                }))
            });
            if self.0.child {
                View::component::<Self>(Input {
                    child: false,
                    log: Rc::clone(&self.0.log),
                    name: "child",
                })
            } else {
                View::native(TextBlock::new())
            }
        }
    }

    let log = Rc::new(RefCell::new(Vec::new()));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<EffectTree>(Input {
        child: true,
        log: Rc::clone(&log),
        name: "parent",
    }))
    .unwrap();

    pump.update_view(View::native(TextBlock::new())).unwrap();
    assert_eq!(&*log.borrow(), &["child", "parent"]);
}
