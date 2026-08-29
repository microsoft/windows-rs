//! Typed context propagation, shadowing, locality, and publication tests.

use super::super::*;
use super::support::*;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;

fn root_native(pump: &Pump<RecordingRuntime>) -> NodeId {
    Pump::<RecordingRuntime>::native_root(&pump.tree, pump.root().unwrap()).unwrap()
}

#[derive(Clone)]
struct ConsumerInput {
    context: Rc<Context<String>>,
    views: Rc<Cell<u32>>,
}

impl PartialEq for ConsumerInput {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.context, &other.context) && Rc::ptr_eq(&self.views, &other.views)
    }
}

struct Consumer(ConsumerInput);

impl Component for Consumer {
    type Message = ();
    type Input = ConsumerInput;

    fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self(input.clone())
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        self.0 = input.clone();
    }

    fn update(&mut self, (): (), _context: &ComponentContext<Self>) {}

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        self.0.views.set(self.0.views.get() + 1);
        View::native(TextBlock::new().text(context.use_context(&self.0.context)))
    }
}

#[derive(Clone)]
struct ProviderInput {
    context: Rc<Context<String>>,
    direct_views: Rc<Cell<u32>>,
    generation: u32,
    inner_views: Rc<Cell<u32>>,
    value: String,
}

impl PartialEq for ProviderInput {
    fn eq(&self, other: &Self) -> bool {
        self.generation == other.generation
            && self.value == other.value
            && Rc::ptr_eq(&self.context, &other.context)
            && Rc::ptr_eq(&self.direct_views, &other.direct_views)
            && Rc::ptr_eq(&self.inner_views, &other.inner_views)
    }
}

struct ProviderRoot(ProviderInput);

impl Component for ProviderRoot {
    type Message = ();
    type Input = ProviderInput;

    fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self(input.clone())
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        self.0 = input.clone();
    }

    fn update(&mut self, (): (), _context: &ComponentContext<Self>) {}

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        View::provide(
            &self.0.context,
            self.0.value.clone(),
            StackPanel::new().keyed_children([
                KeyedView::new(
                    "direct",
                    View::component::<Consumer>(ConsumerInput {
                        context: Rc::clone(&self.0.context),
                        views: Rc::clone(&self.0.direct_views),
                    }),
                ),
                KeyedView::new(
                    "inner",
                    View::provide(
                        &self.0.context,
                        "inner".to_string(),
                        View::component::<Consumer>(ConsumerInput {
                            context: Rc::clone(&self.0.context),
                            views: Rc::clone(&self.0.inner_views),
                        }),
                    ),
                ),
            ]),
        )
    }
}

#[test]
fn provider_changes_only_recompose_consumers_resolved_to_that_provider() {
    let context = Rc::new(Context::new("default".to_string()));
    let direct_views = Rc::new(Cell::new(0));
    let inner_views = Rc::new(Cell::new(0));
    let input = |value: &str, generation| ProviderInput {
        context: Rc::clone(&context),
        direct_views: Rc::clone(&direct_views),
        generation,
        inner_views: Rc::clone(&inner_views),
        value: value.to_string(),
    };
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<ProviderRoot>(input("outer", 0)))
        .unwrap();

    assert_eq!(direct_views.get(), 1);
    assert_eq!(inner_views.get(), 1);
    assert_eq!(
        recorded_text(pump.runtime(), root_native(&pump)),
        ["outer", "inner"]
    );

    pump.update_view(View::component::<ProviderRoot>(input("changed", 1)))
        .unwrap();
    assert_eq!(direct_views.get(), 2);
    assert_eq!(inner_views.get(), 1);
    assert_eq!(
        recorded_text(pump.runtime(), root_native(&pump)),
        ["changed", "inner"]
    );

    pump.update_view(View::component::<ProviderRoot>(input("changed", 2)))
        .unwrap();
    assert_eq!(direct_views.get(), 2);
    assert_eq!(inner_views.get(), 1);
}

#[test]
fn provider_key_changes_do_not_recompose_shadowed_consumers() {
    #[derive(Clone)]
    struct Input {
        context_a: Rc<Context<String>>,
        context_b: Rc<Context<String>>,
        direct_a: Rc<Cell<u32>>,
        direct_b: Rc<Cell<u32>>,
        selected: bool,
        shadowed_a: Rc<Cell<u32>>,
        shadowed_b: Rc<Cell<u32>>,
    }

    impl PartialEq for Input {
        fn eq(&self, other: &Self) -> bool {
            self.selected == other.selected
                && Rc::ptr_eq(&self.context_a, &other.context_a)
                && Rc::ptr_eq(&self.context_b, &other.context_b)
                && Rc::ptr_eq(&self.direct_a, &other.direct_a)
                && Rc::ptr_eq(&self.direct_b, &other.direct_b)
                && Rc::ptr_eq(&self.shadowed_a, &other.shadowed_a)
                && Rc::ptr_eq(&self.shadowed_b, &other.shadowed_b)
        }
    }

    struct Root(Input);

    impl Component for Root {
        type Message = ();
        type Input = Input;

        fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
            Self(input.clone())
        }

        fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
            self.0 = input.clone();
        }

        fn update(&mut self, (): (), _context: &ComponentContext<Self>) {}

        fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
            let child = StackPanel::new().keyed_children([
                KeyedView::new(
                    "direct-a",
                    View::component::<Consumer>(ConsumerInput {
                        context: Rc::clone(&self.0.context_a),
                        views: Rc::clone(&self.0.direct_a),
                    }),
                ),
                KeyedView::new(
                    "direct-b",
                    View::component::<Consumer>(ConsumerInput {
                        context: Rc::clone(&self.0.context_b),
                        views: Rc::clone(&self.0.direct_b),
                    }),
                ),
                KeyedView::new(
                    "shadowed-a",
                    View::provide(
                        &self.0.context_a,
                        "inner-a".to_string(),
                        View::component::<Consumer>(ConsumerInput {
                            context: Rc::clone(&self.0.context_a),
                            views: Rc::clone(&self.0.shadowed_a),
                        }),
                    ),
                ),
                KeyedView::new(
                    "shadowed-b",
                    View::provide(
                        &self.0.context_b,
                        "inner-b".to_string(),
                        View::component::<Consumer>(ConsumerInput {
                            context: Rc::clone(&self.0.context_b),
                            views: Rc::clone(&self.0.shadowed_b),
                        }),
                    ),
                ),
            ]);
            if self.0.selected {
                View::provide(&self.0.context_b, "outer-b".to_string(), child)
            } else {
                View::provide(&self.0.context_a, "outer-a".to_string(), child)
            }
        }
    }

    let context_a = Rc::new(Context::new("default-a".to_string()));
    let context_b = Rc::new(Context::new("default-b".to_string()));
    let direct_a = Rc::new(Cell::new(0));
    let direct_b = Rc::new(Cell::new(0));
    let shadowed_a = Rc::new(Cell::new(0));
    let shadowed_b = Rc::new(Cell::new(0));
    let input = |selected| Input {
        context_a: Rc::clone(&context_a),
        context_b: Rc::clone(&context_b),
        direct_a: Rc::clone(&direct_a),
        direct_b: Rc::clone(&direct_b),
        selected,
        shadowed_a: Rc::clone(&shadowed_a),
        shadowed_b: Rc::clone(&shadowed_b),
    };
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<Root>(input(false)))
        .unwrap();
    pump.update_view(View::component::<Root>(input(true)))
        .unwrap();

    assert_eq!(direct_a.get(), 2);
    assert_eq!(direct_b.get(), 2);
    assert_eq!(shadowed_a.get(), 1);
    assert_eq!(shadowed_b.get(), 1);
    assert_eq!(
        recorded_text(pump.runtime(), root_native(&pump)),
        ["default-a", "outer-b", "inner-a", "inner-b"]
    );
}

#[test]
fn broad_provider_update_skips_non_consuming_component_boundary() {
    #[derive(Clone)]
    struct SubtreeInput {
        context: Rc<Context<String>>,
        consumer_views: Rc<Cell<u32>>,
        subtree_views: Rc<Cell<u32>>,
    }

    impl PartialEq for SubtreeInput {
        fn eq(&self, other: &Self) -> bool {
            Rc::ptr_eq(&self.context, &other.context)
                && Rc::ptr_eq(&self.consumer_views, &other.consumer_views)
                && Rc::ptr_eq(&self.subtree_views, &other.subtree_views)
        }
    }

    struct Subtree(SubtreeInput);

    impl Component for Subtree {
        type Message = ();
        type Input = SubtreeInput;

        fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
            Self(input.clone())
        }

        fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
            self.0 = input.clone();
        }

        fn update(&mut self, (): (), _context: &ComponentContext<Self>) {}

        fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
            self.0.subtree_views.set(self.0.subtree_views.get() + 1);
            let mut children = (0..1_024_usize)
                .map(|index| {
                    KeyedView::new(
                        index,
                        View::native(TextBlock::new().text(index.to_string())),
                    )
                })
                .collect::<Vec<_>>();
            children.push(KeyedView::new(
                1_024_usize,
                View::component::<Consumer>(ConsumerInput {
                    context: Rc::clone(&self.0.context),
                    views: Rc::clone(&self.0.consumer_views),
                }),
            ));
            StackPanel::new().keyed_children(children)
        }
    }

    #[derive(Clone)]
    struct RootInput {
        context: Rc<Context<String>>,
        subtree: SubtreeInput,
        value: String,
    }

    impl PartialEq for RootInput {
        fn eq(&self, other: &Self) -> bool {
            self.value == other.value
                && Rc::ptr_eq(&self.context, &other.context)
                && self.subtree == other.subtree
        }
    }

    struct Root(RootInput);

    impl Component for Root {
        type Message = ();
        type Input = RootInput;

        fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
            Self(input.clone())
        }

        fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
            self.0 = input.clone();
        }

        fn update(&mut self, (): (), _context: &ComponentContext<Self>) {}

        fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
            View::provide(
                &self.0.context,
                self.0.value.clone(),
                View::component::<Subtree>(self.0.subtree.clone()),
            )
        }
    }

    let context = Rc::new(Context::new("default".to_string()));
    let consumer_views = Rc::new(Cell::new(0));
    let subtree_views = Rc::new(Cell::new(0));
    let subtree = SubtreeInput {
        context: Rc::clone(&context),
        consumer_views: Rc::clone(&consumer_views),
        subtree_views: Rc::clone(&subtree_views),
    };
    let input = |value: &str| RootInput {
        context: Rc::clone(&context),
        subtree: subtree.clone(),
        value: value.to_string(),
    };
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<Root>(input("first")))
        .unwrap();
    pump.update_view(View::component::<Root>(input("second")))
        .unwrap();

    assert_eq!(subtree_views.get(), 1);
    assert_eq!(consumer_views.get(), 2);
    assert_eq!(
        recorded_text(pump.runtime(), root_native(&pump))
            .last()
            .map(String::as_str),
        Some("second")
    );
}

#[test]
fn component_message_provider_update_recomposes_consumer() {
    #[derive(Clone)]
    struct Input {
        context: Rc<Context<String>>,
        sender: Rc<RefCell<Option<LocalSender<String>>>>,
        views: Rc<Cell<u32>>,
    }

    impl PartialEq for Input {
        fn eq(&self, other: &Self) -> bool {
            Rc::ptr_eq(&self.context, &other.context)
                && Rc::ptr_eq(&self.sender, &other.sender)
                && Rc::ptr_eq(&self.views, &other.views)
        }
    }

    struct Root {
        input: Input,
        value: String,
    }

    impl Component for Root {
        type Message = String;
        type Input = Input;

        fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
            *input.sender.borrow_mut() = Some(context.sender());
            Self {
                input: input.clone(),
                value: "first".to_string(),
            }
        }

        fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
            self.input = input.clone();
        }

        fn update(&mut self, message: String, _context: &ComponentContext<Self>) {
            self.value = message;
        }

        fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
            View::provide(
                &self.input.context,
                self.value.clone(),
                View::component::<Consumer>(ConsumerInput {
                    context: Rc::clone(&self.input.context),
                    views: Rc::clone(&self.input.views),
                }),
            )
        }
    }

    let context = Rc::new(Context::new("default".to_string()));
    let sender = Rc::new(RefCell::new(None));
    let views = Rc::new(Cell::new(0));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<Root>(Input {
        context,
        sender: Rc::clone(&sender),
        views: Rc::clone(&views),
    }))
    .unwrap();

    assert!(sender.borrow().as_ref().unwrap().send("second".to_string()));
    assert_eq!(pump.dispatch_components(1), Ok(1));
    assert_eq!(views.get(), 2);
    assert_eq!(
        pump.runtime()
            .node(root_native(&pump))
            .unwrap()
            .property(PropertyId::TextBlockText),
        Some(&PropertyValue::Str("second".to_string()))
    );
}

#[test]
fn consumer_uses_default_without_a_provider() {
    let context = Rc::new(Context::new("default".to_string()));
    let views = Rc::new(Cell::new(0));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<Consumer>(ConsumerInput {
        context,
        views: Rc::clone(&views),
    }))
    .unwrap();

    assert_eq!(views.get(), 1);
    assert_eq!(
        pump.runtime()
            .node(root_native(&pump))
            .unwrap()
            .property(PropertyId::TextBlockText),
        Some(&PropertyValue::Str("default".to_string()))
    );
}

#[test]
fn stopped_context_reads_are_not_invalidated() {
    #[derive(Clone)]
    struct ConditionalInput {
        context: Rc<Context<String>>,
        enabled: bool,
        views: Rc<Cell<u32>>,
    }

    impl PartialEq for ConditionalInput {
        fn eq(&self, other: &Self) -> bool {
            self.enabled == other.enabled
                && Rc::ptr_eq(&self.context, &other.context)
                && Rc::ptr_eq(&self.views, &other.views)
        }
    }

    struct ConditionalConsumer(ConditionalInput);

    impl Component for ConditionalConsumer {
        type Message = ();
        type Input = ConditionalInput;

        fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
            Self(input.clone())
        }

        fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
            self.0 = input.clone();
        }

        fn update(&mut self, (): (), _context: &ComponentContext<Self>) {}

        fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
            self.0.views.set(self.0.views.get() + 1);
            let value = if self.0.enabled {
                context.use_context(&self.0.context)
            } else {
                "static".to_string()
            };
            View::native(TextBlock::new().text(value))
        }
    }

    #[derive(Clone)]
    struct RootInput {
        context: Rc<Context<String>>,
        enabled: bool,
        value: String,
        views: Rc<Cell<u32>>,
    }

    impl PartialEq for RootInput {
        fn eq(&self, other: &Self) -> bool {
            self.enabled == other.enabled
                && self.value == other.value
                && Rc::ptr_eq(&self.context, &other.context)
                && Rc::ptr_eq(&self.views, &other.views)
        }
    }

    struct Root(RootInput);

    impl Component for Root {
        type Message = ();
        type Input = RootInput;

        fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
            Self(input.clone())
        }

        fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
            self.0 = input.clone();
        }

        fn update(&mut self, (): (), _context: &ComponentContext<Self>) {}

        fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
            View::provide(
                &self.0.context,
                self.0.value.clone(),
                View::component::<ConditionalConsumer>(ConditionalInput {
                    context: Rc::clone(&self.0.context),
                    enabled: self.0.enabled,
                    views: Rc::clone(&self.0.views),
                }),
            )
        }
    }

    let context = Rc::new(Context::new("default".to_string()));
    let views = Rc::new(Cell::new(0));
    let input = |enabled, value: &str| RootInput {
        context: Rc::clone(&context),
        enabled,
        value: value.to_string(),
        views: Rc::clone(&views),
    };
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<Root>(input(true, "first")))
        .unwrap();
    pump.update_view(View::component::<Root>(input(false, "first")))
        .unwrap();
    pump.update_view(View::component::<Root>(input(false, "second")))
        .unwrap();

    assert_eq!(views.get(), 2);
}

#[derive(Clone)]
struct ListInput {
    context: Rc<Context<String>>,
    entries: Vec<(u64, String)>,
    views: Rc<Cell<u32>>,
}

impl PartialEq for ListInput {
    fn eq(&self, other: &Self) -> bool {
        self.entries == other.entries
            && Rc::ptr_eq(&self.context, &other.context)
            && Rc::ptr_eq(&self.views, &other.views)
    }
}

struct ProviderList(ListInput);

impl Component for ProviderList {
    type Message = ();
    type Input = ListInput;

    fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self(input.clone())
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        self.0 = input.clone();
    }

    fn update(&mut self, (): (), _context: &ComponentContext<Self>) {}

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        StackPanel::new().keyed_children(self.0.entries.iter().map(|(key, value)| {
            KeyedView::new(
                *key,
                View::provide(
                    &self.0.context,
                    value.clone(),
                    View::component::<Consumer>(ConsumerInput {
                        context: Rc::clone(&self.0.context),
                        views: Rc::clone(&self.0.views),
                    }),
                ),
            )
        }))
    }
}

#[test]
fn keyed_provider_moves_preserve_identity_and_retirement_removes_consumers() {
    let context = Rc::new(Context::new("default".to_string()));
    let views = Rc::new(Cell::new(0));
    let input = |entries| ListInput {
        context: Rc::clone(&context),
        entries,
        views: Rc::clone(&views),
    };
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<ProviderList>(input(vec![
        (1, "one".to_string()),
        (2, "two".to_string()),
    ])))
    .unwrap();
    assert_eq!(views.get(), 2);

    pump.update_view(View::component::<ProviderList>(input(vec![
        (2, "two".to_string()),
        (1, "one".to_string()),
    ])))
    .unwrap();
    assert_eq!(views.get(), 2);
    assert_eq!(
        recorded_text(pump.runtime(), root_native(&pump)),
        ["two", "one"]
    );

    pump.update_view(View::component::<ProviderList>(input(vec![(
        2,
        "changed".to_string(),
    )])))
    .unwrap();
    assert_eq!(views.get(), 3);
    assert_eq!(
        recorded_text(pump.runtime(), root_native(&pump)),
        ["changed"]
    );
}

struct InvalidConsumer(ConsumerInput);

impl Component for InvalidConsumer {
    type Message = ();
    type Input = ConsumerInput;

    fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self(input.clone())
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        self.0 = input.clone();
    }

    fn update(&mut self, (): (), _context: &ComponentContext<Self>) {}

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        self.0.views.set(self.0.views.get() + 1);
        if context.use_context(&self.0.context) == "bad" {
            View::fragment([TextBlock::new(), TextBlock::new()])
        } else {
            View::native(TextBlock::new())
        }
    }
}

#[derive(Clone)]
struct InvalidRootInput {
    context: Rc<Context<String>>,
    value: String,
    views: Rc<Cell<u32>>,
}

impl PartialEq for InvalidRootInput {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
            && Rc::ptr_eq(&self.context, &other.context)
            && Rc::ptr_eq(&self.views, &other.views)
    }
}

struct InvalidRoot(InvalidRootInput);

impl Component for InvalidRoot {
    type Message = ();
    type Input = InvalidRootInput;

    fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self(input.clone())
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        self.0 = input.clone();
    }

    fn update(&mut self, (): (), _context: &ComponentContext<Self>) {}

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        View::provide(
            &self.0.context,
            self.0.value.clone(),
            View::component::<InvalidConsumer>(ConsumerInput {
                context: Rc::clone(&self.0.context),
                views: Rc::clone(&self.0.views),
            }),
        )
    }
}

#[test]
fn failed_context_planning_retries_without_publishing_dependencies() {
    let context = Rc::new(Context::new("default".to_string()));
    let views = Rc::new(Cell::new(0));
    let input = |value: &str| InvalidRootInput {
        context: Rc::clone(&context),
        value: value.to_string(),
        views: Rc::clone(&views),
    };
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<InvalidRoot>(input("good")))
        .unwrap();
    let version = pump.version();

    assert_eq!(
        pump.update_view(View::component::<InvalidRoot>(input("bad"))),
        Err(PumpError::StructureUnsupported)
    );
    assert_eq!(
        pump.update_view(View::component::<InvalidRoot>(input("bad"))),
        Err(PumpError::StructureUnsupported)
    );
    assert_eq!(pump.version(), version);
    assert_eq!(views.get(), 3);
}

#[test]
fn failed_native_apply_does_not_publish_changed_context_dependency() {
    let first = Rc::new(Context::new("first".to_string()));
    let second = Rc::new(Context::new("second".to_string()));
    let views = Rc::new(Cell::new(0));
    let input = |context: &Rc<Context<String>>| ConsumerInput {
        context: Rc::clone(context),
        views: Rc::clone(&views),
    };
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<Consumer>(input(&first)))
        .unwrap();
    let root = pump.root().unwrap();
    let scope = pump.tree.component_scope(root).unwrap();
    let token = pump.components.token(scope).unwrap();
    pump.runtime_mut().fail_at(0);

    assert!(matches!(
        pump.update_view(View::component::<Consumer>(input(&second))),
        Err(PumpError::NativeApplyFailed(_))
    ));
    assert!(
        pump.components
            .context_dependencies(token)
            .unwrap()
            .unwrap()
            .contains(&ContextDependency {
                id: first.id(),
                provider: None,
            })
    );
    assert_eq!(
        pump.components
            .context_consumers(ContextDependency {
                id: second.id(),
                provider: None,
            })
            .count(),
        0
    );
}

#[test]
fn context_state_is_isolated_per_pump() {
    let context = Rc::new(Context::new("default".to_string()));
    let first_views = Rc::new(Cell::new(0));
    let second_views = Rc::new(Cell::new(0));
    let input = |value: &str, views: &Rc<Cell<u32>>| ProviderInput {
        context: Rc::clone(&context),
        direct_views: Rc::clone(views),
        generation: 0,
        inner_views: Rc::new(Cell::new(0)),
        value: value.to_string(),
    };
    let mut first = Pump::new(RecordingRuntime::default());
    let mut second = Pump::new(RecordingRuntime::default());
    first
        .mount_view(View::component::<ProviderRoot>(input(
            "first",
            &first_views,
        )))
        .unwrap();
    second
        .mount_view(View::component::<ProviderRoot>(input(
            "second",
            &second_views,
        )))
        .unwrap();

    first
        .update_view(View::component::<ProviderRoot>(input(
            "changed",
            &first_views,
        )))
        .unwrap();
    assert_eq!(first_views.get(), 2);
    assert_eq!(second_views.get(), 1);
    assert_eq!(
        recorded_text(second.runtime(), root_native(&second))[0],
        "second"
    );
}
