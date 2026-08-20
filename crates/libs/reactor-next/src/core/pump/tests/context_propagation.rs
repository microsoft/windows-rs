//! Typed context propagation, shadowing, locality, and publication tests.

use super::super::*;
use super::support::*;
use crate::native::*;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;

fn root_native(pump: &Pump<RecordingRuntime>) -> NodeId {
    Pump::<RecordingRuntime>::native_root(&pump.tree, pump.root().unwrap()).unwrap()
}

#[derive(Clone)]
struct ConsumerProps {
    context: Rc<Context<String>>,
    views: Rc<Cell<u32>>,
}

impl PartialEq for ConsumerProps {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.context, &other.context) && Rc::ptr_eq(&self.views, &other.views)
    }
}

struct Consumer(ConsumerProps);

impl Component for Consumer {
    type Message = ();
    type Props = ConsumerProps;

    fn create(props: &Self::Props, _context: &mut ComponentContext<Self>) -> Self {
        Self(props.clone())
    }

    fn changed(&mut self, props: &Self::Props, _context: &mut ComponentContext<Self>) {
        self.0 = props.clone();
    }

    fn update(&mut self, (): (), _context: &mut ComponentContext<Self>) {}

    fn view(&self, context: &mut ViewContext<Self>) -> View {
        self.0.views.set(self.0.views.get() + 1);
        View::native(TextBlock::new().text(context.use_context(&self.0.context)))
    }
}

#[derive(Clone)]
struct ProviderProps {
    context: Rc<Context<String>>,
    direct_views: Rc<Cell<u32>>,
    generation: u32,
    inner_views: Rc<Cell<u32>>,
    value: String,
}

impl PartialEq for ProviderProps {
    fn eq(&self, other: &Self) -> bool {
        self.generation == other.generation
            && self.value == other.value
            && Rc::ptr_eq(&self.context, &other.context)
            && Rc::ptr_eq(&self.direct_views, &other.direct_views)
            && Rc::ptr_eq(&self.inner_views, &other.inner_views)
    }
}

struct ProviderRoot(ProviderProps);

impl Component for ProviderRoot {
    type Message = ();
    type Props = ProviderProps;

    fn create(props: &Self::Props, _context: &mut ComponentContext<Self>) -> Self {
        Self(props.clone())
    }

    fn changed(&mut self, props: &Self::Props, _context: &mut ComponentContext<Self>) {
        self.0 = props.clone();
    }

    fn update(&mut self, (): (), _context: &mut ComponentContext<Self>) {}

    fn view(&self, _context: &mut ViewContext<Self>) -> View {
        View::provide(
            &self.0.context,
            self.0.value.clone(),
            View::children(
                StackPanel::new(),
                [
                    KeyedView::new(
                        "direct",
                        View::component::<Consumer>(ConsumerProps {
                            context: Rc::clone(&self.0.context),
                            views: Rc::clone(&self.0.direct_views),
                        }),
                    ),
                    KeyedView::new(
                        "inner",
                        View::provide(
                            &self.0.context,
                            "inner".to_string(),
                            View::component::<Consumer>(ConsumerProps {
                                context: Rc::clone(&self.0.context),
                                views: Rc::clone(&self.0.inner_views),
                            }),
                        ),
                    ),
                ],
            ),
        )
    }
}

#[test]
fn provider_changes_only_recompose_consumers_resolved_to_that_provider() {
    let context = Rc::new(Context::new("default".to_string()));
    let direct_views = Rc::new(Cell::new(0));
    let inner_views = Rc::new(Cell::new(0));
    let props = |value: &str, generation| ProviderProps {
        context: Rc::clone(&context),
        direct_views: Rc::clone(&direct_views),
        generation,
        inner_views: Rc::clone(&inner_views),
        value: value.to_string(),
    };
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<ProviderRoot>(props("outer", 0)))
        .unwrap();

    assert_eq!(direct_views.get(), 1);
    assert_eq!(inner_views.get(), 1);
    assert_eq!(
        recorded_text(pump.runtime(), root_native(&pump)),
        ["outer", "inner"]
    );

    pump.update_view(View::component::<ProviderRoot>(props("changed", 1)))
        .unwrap();
    assert_eq!(direct_views.get(), 2);
    assert_eq!(inner_views.get(), 1);
    assert_eq!(
        recorded_text(pump.runtime(), root_native(&pump)),
        ["changed", "inner"]
    );

    pump.update_view(View::component::<ProviderRoot>(props("changed", 2)))
        .unwrap();
    assert_eq!(direct_views.get(), 2);
    assert_eq!(inner_views.get(), 1);
}

#[test]
fn provider_key_changes_do_not_recompose_shadowed_consumers() {
    #[derive(Clone)]
    struct Props {
        context_a: Rc<Context<String>>,
        context_b: Rc<Context<String>>,
        direct_a: Rc<Cell<u32>>,
        direct_b: Rc<Cell<u32>>,
        selected: bool,
        shadowed_a: Rc<Cell<u32>>,
        shadowed_b: Rc<Cell<u32>>,
    }

    impl PartialEq for Props {
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

    struct Root(Props);

    impl Component for Root {
        type Message = ();
        type Props = Props;

        fn create(props: &Self::Props, _context: &mut ComponentContext<Self>) -> Self {
            Self(props.clone())
        }

        fn changed(&mut self, props: &Self::Props, _context: &mut ComponentContext<Self>) {
            self.0 = props.clone();
        }

        fn update(&mut self, (): (), _context: &mut ComponentContext<Self>) {}

        fn view(&self, _context: &mut ViewContext<Self>) -> View {
            let child = View::children(
                StackPanel::new(),
                [
                    KeyedView::new(
                        "direct-a",
                        View::component::<Consumer>(ConsumerProps {
                            context: Rc::clone(&self.0.context_a),
                            views: Rc::clone(&self.0.direct_a),
                        }),
                    ),
                    KeyedView::new(
                        "direct-b",
                        View::component::<Consumer>(ConsumerProps {
                            context: Rc::clone(&self.0.context_b),
                            views: Rc::clone(&self.0.direct_b),
                        }),
                    ),
                    KeyedView::new(
                        "shadowed-a",
                        View::provide(
                            &self.0.context_a,
                            "inner-a".to_string(),
                            View::component::<Consumer>(ConsumerProps {
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
                            View::component::<Consumer>(ConsumerProps {
                                context: Rc::clone(&self.0.context_b),
                                views: Rc::clone(&self.0.shadowed_b),
                            }),
                        ),
                    ),
                ],
            );
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
    let props = |selected| Props {
        context_a: Rc::clone(&context_a),
        context_b: Rc::clone(&context_b),
        direct_a: Rc::clone(&direct_a),
        direct_b: Rc::clone(&direct_b),
        selected,
        shadowed_a: Rc::clone(&shadowed_a),
        shadowed_b: Rc::clone(&shadowed_b),
    };
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<Root>(props(false)))
        .unwrap();
    pump.update_view(View::component::<Root>(props(true)))
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
    struct SubtreeProps {
        context: Rc<Context<String>>,
        consumer_views: Rc<Cell<u32>>,
        subtree_views: Rc<Cell<u32>>,
    }

    impl PartialEq for SubtreeProps {
        fn eq(&self, other: &Self) -> bool {
            Rc::ptr_eq(&self.context, &other.context)
                && Rc::ptr_eq(&self.consumer_views, &other.consumer_views)
                && Rc::ptr_eq(&self.subtree_views, &other.subtree_views)
        }
    }

    struct Subtree(SubtreeProps);

    impl Component for Subtree {
        type Message = ();
        type Props = SubtreeProps;

        fn create(props: &Self::Props, _context: &mut ComponentContext<Self>) -> Self {
            Self(props.clone())
        }

        fn changed(&mut self, props: &Self::Props, _context: &mut ComponentContext<Self>) {
            self.0 = props.clone();
        }

        fn update(&mut self, (): (), _context: &mut ComponentContext<Self>) {}

        fn view(&self, _context: &mut ViewContext<Self>) -> View {
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
                View::component::<Consumer>(ConsumerProps {
                    context: Rc::clone(&self.0.context),
                    views: Rc::clone(&self.0.consumer_views),
                }),
            ));
            View::children(StackPanel::new(), children)
        }
    }

    #[derive(Clone)]
    struct RootProps {
        context: Rc<Context<String>>,
        subtree: SubtreeProps,
        value: String,
    }

    impl PartialEq for RootProps {
        fn eq(&self, other: &Self) -> bool {
            self.value == other.value
                && Rc::ptr_eq(&self.context, &other.context)
                && self.subtree == other.subtree
        }
    }

    struct Root(RootProps);

    impl Component for Root {
        type Message = ();
        type Props = RootProps;

        fn create(props: &Self::Props, _context: &mut ComponentContext<Self>) -> Self {
            Self(props.clone())
        }

        fn changed(&mut self, props: &Self::Props, _context: &mut ComponentContext<Self>) {
            self.0 = props.clone();
        }

        fn update(&mut self, (): (), _context: &mut ComponentContext<Self>) {}

        fn view(&self, _context: &mut ViewContext<Self>) -> View {
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
    let subtree = SubtreeProps {
        context: Rc::clone(&context),
        consumer_views: Rc::clone(&consumer_views),
        subtree_views: Rc::clone(&subtree_views),
    };
    let props = |value: &str| RootProps {
        context: Rc::clone(&context),
        subtree: subtree.clone(),
        value: value.to_string(),
    };
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<Root>(props("first")))
        .unwrap();
    pump.update_view(View::component::<Root>(props("second")))
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
    struct Props {
        context: Rc<Context<String>>,
        sender: Rc<RefCell<Option<LocalSender<String>>>>,
        views: Rc<Cell<u32>>,
    }

    impl PartialEq for Props {
        fn eq(&self, other: &Self) -> bool {
            Rc::ptr_eq(&self.context, &other.context)
                && Rc::ptr_eq(&self.sender, &other.sender)
                && Rc::ptr_eq(&self.views, &other.views)
        }
    }

    struct Root {
        props: Props,
        value: String,
    }

    impl Component for Root {
        type Message = String;
        type Props = Props;

        fn create(props: &Self::Props, context: &mut ComponentContext<Self>) -> Self {
            *props.sender.borrow_mut() = Some(context.sender());
            Self {
                props: props.clone(),
                value: "first".to_string(),
            }
        }

        fn changed(&mut self, props: &Self::Props, _context: &mut ComponentContext<Self>) {
            self.props = props.clone();
        }

        fn update(&mut self, message: String, _context: &mut ComponentContext<Self>) {
            self.value = message;
        }

        fn view(&self, _context: &mut ViewContext<Self>) -> View {
            View::provide(
                &self.props.context,
                self.value.clone(),
                View::component::<Consumer>(ConsumerProps {
                    context: Rc::clone(&self.props.context),
                    views: Rc::clone(&self.props.views),
                }),
            )
        }
    }

    let context = Rc::new(Context::new("default".to_string()));
    let sender = Rc::new(RefCell::new(None));
    let views = Rc::new(Cell::new(0));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<Root>(Props {
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
    pump.mount_view(View::component::<Consumer>(ConsumerProps {
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
    struct ConditionalProps {
        context: Rc<Context<String>>,
        enabled: bool,
        views: Rc<Cell<u32>>,
    }

    impl PartialEq for ConditionalProps {
        fn eq(&self, other: &Self) -> bool {
            self.enabled == other.enabled
                && Rc::ptr_eq(&self.context, &other.context)
                && Rc::ptr_eq(&self.views, &other.views)
        }
    }

    struct ConditionalConsumer(ConditionalProps);

    impl Component for ConditionalConsumer {
        type Message = ();
        type Props = ConditionalProps;

        fn create(props: &Self::Props, _context: &mut ComponentContext<Self>) -> Self {
            Self(props.clone())
        }

        fn changed(&mut self, props: &Self::Props, _context: &mut ComponentContext<Self>) {
            self.0 = props.clone();
        }

        fn update(&mut self, (): (), _context: &mut ComponentContext<Self>) {}

        fn view(&self, context: &mut ViewContext<Self>) -> View {
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
    struct RootProps {
        context: Rc<Context<String>>,
        enabled: bool,
        value: String,
        views: Rc<Cell<u32>>,
    }

    impl PartialEq for RootProps {
        fn eq(&self, other: &Self) -> bool {
            self.enabled == other.enabled
                && self.value == other.value
                && Rc::ptr_eq(&self.context, &other.context)
                && Rc::ptr_eq(&self.views, &other.views)
        }
    }

    struct Root(RootProps);

    impl Component for Root {
        type Message = ();
        type Props = RootProps;

        fn create(props: &Self::Props, _context: &mut ComponentContext<Self>) -> Self {
            Self(props.clone())
        }

        fn changed(&mut self, props: &Self::Props, _context: &mut ComponentContext<Self>) {
            self.0 = props.clone();
        }

        fn update(&mut self, (): (), _context: &mut ComponentContext<Self>) {}

        fn view(&self, _context: &mut ViewContext<Self>) -> View {
            View::provide(
                &self.0.context,
                self.0.value.clone(),
                View::component::<ConditionalConsumer>(ConditionalProps {
                    context: Rc::clone(&self.0.context),
                    enabled: self.0.enabled,
                    views: Rc::clone(&self.0.views),
                }),
            )
        }
    }

    let context = Rc::new(Context::new("default".to_string()));
    let views = Rc::new(Cell::new(0));
    let props = |enabled, value: &str| RootProps {
        context: Rc::clone(&context),
        enabled,
        value: value.to_string(),
        views: Rc::clone(&views),
    };
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<Root>(props(true, "first")))
        .unwrap();
    pump.update_view(View::component::<Root>(props(false, "first")))
        .unwrap();
    pump.update_view(View::component::<Root>(props(false, "second")))
        .unwrap();

    assert_eq!(views.get(), 2);
}

#[derive(Clone)]
struct ListProps {
    context: Rc<Context<String>>,
    entries: Vec<(u64, String)>,
    views: Rc<Cell<u32>>,
}

impl PartialEq for ListProps {
    fn eq(&self, other: &Self) -> bool {
        self.entries == other.entries
            && Rc::ptr_eq(&self.context, &other.context)
            && Rc::ptr_eq(&self.views, &other.views)
    }
}

struct ProviderList(ListProps);

impl Component for ProviderList {
    type Message = ();
    type Props = ListProps;

    fn create(props: &Self::Props, _context: &mut ComponentContext<Self>) -> Self {
        Self(props.clone())
    }

    fn changed(&mut self, props: &Self::Props, _context: &mut ComponentContext<Self>) {
        self.0 = props.clone();
    }

    fn update(&mut self, (): (), _context: &mut ComponentContext<Self>) {}

    fn view(&self, _context: &mut ViewContext<Self>) -> View {
        View::children(
            StackPanel::new(),
            self.0.entries.iter().map(|(key, value)| {
                KeyedView::new(
                    *key,
                    View::provide(
                        &self.0.context,
                        value.clone(),
                        View::component::<Consumer>(ConsumerProps {
                            context: Rc::clone(&self.0.context),
                            views: Rc::clone(&self.0.views),
                        }),
                    ),
                )
            }),
        )
    }
}

#[test]
fn keyed_provider_moves_preserve_identity_and_retirement_removes_consumers() {
    let context = Rc::new(Context::new("default".to_string()));
    let views = Rc::new(Cell::new(0));
    let props = |entries| ListProps {
        context: Rc::clone(&context),
        entries,
        views: Rc::clone(&views),
    };
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<ProviderList>(props(vec![
        (1, "one".to_string()),
        (2, "two".to_string()),
    ])))
    .unwrap();
    assert_eq!(views.get(), 2);

    pump.update_view(View::component::<ProviderList>(props(vec![
        (2, "two".to_string()),
        (1, "one".to_string()),
    ])))
    .unwrap();
    assert_eq!(views.get(), 2);
    assert_eq!(
        recorded_text(pump.runtime(), root_native(&pump)),
        ["two", "one"]
    );

    pump.update_view(View::component::<ProviderList>(props(vec![(
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

struct InvalidConsumer(ConsumerProps);

impl Component for InvalidConsumer {
    type Message = ();
    type Props = ConsumerProps;

    fn create(props: &Self::Props, _context: &mut ComponentContext<Self>) -> Self {
        Self(props.clone())
    }

    fn changed(&mut self, props: &Self::Props, _context: &mut ComponentContext<Self>) {
        self.0 = props.clone();
    }

    fn update(&mut self, (): (), _context: &mut ComponentContext<Self>) {}

    fn view(&self, context: &mut ViewContext<Self>) -> View {
        self.0.views.set(self.0.views.get() + 1);
        if context.use_context(&self.0.context) == "bad" {
            View::fragment([
                KeyedView::new("a", View::native(TextBlock::new())),
                KeyedView::new("b", View::native(TextBlock::new())),
            ])
        } else {
            View::native(TextBlock::new())
        }
    }
}

#[derive(Clone)]
struct InvalidRootProps {
    context: Rc<Context<String>>,
    value: String,
    views: Rc<Cell<u32>>,
}

impl PartialEq for InvalidRootProps {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
            && Rc::ptr_eq(&self.context, &other.context)
            && Rc::ptr_eq(&self.views, &other.views)
    }
}

struct InvalidRoot(InvalidRootProps);

impl Component for InvalidRoot {
    type Message = ();
    type Props = InvalidRootProps;

    fn create(props: &Self::Props, _context: &mut ComponentContext<Self>) -> Self {
        Self(props.clone())
    }

    fn changed(&mut self, props: &Self::Props, _context: &mut ComponentContext<Self>) {
        self.0 = props.clone();
    }

    fn update(&mut self, (): (), _context: &mut ComponentContext<Self>) {}

    fn view(&self, _context: &mut ViewContext<Self>) -> View {
        View::provide(
            &self.0.context,
            self.0.value.clone(),
            View::component::<InvalidConsumer>(ConsumerProps {
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
    let props = |value: &str| InvalidRootProps {
        context: Rc::clone(&context),
        value: value.to_string(),
        views: Rc::clone(&views),
    };
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<InvalidRoot>(props("good")))
        .unwrap();
    let version = pump.version();

    assert_eq!(
        pump.update_view(View::component::<InvalidRoot>(props("bad"))),
        Err(PumpError::StructureUnsupported)
    );
    assert_eq!(
        pump.update_view(View::component::<InvalidRoot>(props("bad"))),
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
    let props = |context: &Rc<Context<String>>| ConsumerProps {
        context: Rc::clone(context),
        views: Rc::clone(&views),
    };
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<Consumer>(props(&first)))
        .unwrap();
    let root = pump.root().unwrap();
    let scope = pump.tree.component_scope(root).unwrap();
    let token = pump.components.token(scope).unwrap();
    pump.runtime_mut().fail_at(0);

    assert!(matches!(
        pump.update_view(View::component::<Consumer>(props(&second))),
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
    let props = |value: &str, views: &Rc<Cell<u32>>| ProviderProps {
        context: Rc::clone(&context),
        direct_views: Rc::clone(views),
        generation: 0,
        inner_views: Rc::new(Cell::new(0)),
        value: value.to_string(),
    };
    let mut first = Pump::new(RecordingRuntime::default());
    let mut second = Pump::new(RecordingRuntime::default());
    first
        .mount_view(View::component::<ProviderRoot>(props(
            "first",
            &first_views,
        )))
        .unwrap();
    second
        .mount_view(View::component::<ProviderRoot>(props(
            "second",
            &second_views,
        )))
        .unwrap();

    first
        .update_view(View::component::<ProviderRoot>(props(
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
