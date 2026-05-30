use std::cell::Cell;
use std::rc::Rc;
use std::sync::LazyLock;

use windows_reactor::core::backend::RecordingBackend;
use windows_reactor::core::component::Component;
use windows_reactor::core::component_element::component;
use windows_reactor::core::context::Context;
use windows_reactor::core::element::Element;
use windows_reactor::core::reconciler::Reconciler;
use windows_reactor::core::render_context::RenderCx;
use windows_reactor::dsl::{ElementExt, border, text_block};

use windows_reactor::vstack;
static THEME: LazyLock<Context<String>> =
    LazyLock::new(|| Context::new("default-theme".to_string()));
static LEVEL: LazyLock<Context<i32>> = LazyLock::new(|| Context::new(0));

struct Leaf {
    seen: Rc<Cell<String>>,
}
impl Component for Leaf {
    fn render(&self, _props: &(), cx: &mut RenderCx) -> Element {
        let t = cx.use_context(&THEME);
        self.seen.set(t.clone());
        text_block(t).into()
    }
}

fn reconcile(
    r: &mut Reconciler<RecordingBackend>,
    old: Option<&Element>,
    new: &Element,
    existing: Option<windows_reactor::core::backend::ControlId>,
) -> Option<windows_reactor::core::backend::ControlId> {
    r.reconcile(old, new, existing, Rc::new(|| {}))
}

#[test]
fn consumer_with_no_provider_sees_default() {
    let seen = Rc::new(Cell::new(String::new()));
    let root = component(
        Leaf {
            seen: Rc::clone(&seen),
        },
        (),
    );
    let mut r = Reconciler::new(RecordingBackend::new());
    let _ = reconcile(&mut r, None, &root, None);
    assert_eq!(seen.take(), "default-theme");
}

#[test]
fn provide_at_root_reaches_grandchild() {
    let seen = Rc::new(Cell::new(String::new()));

    let leaf = component(
        Leaf {
            seen: Rc::clone(&seen),
        },
        (),
    );
    let tree: Element = vstack((border(leaf),)).provide(&THEME, "dark".to_string());

    let mut r = Reconciler::new(RecordingBackend::new());
    let _ = reconcile(&mut r, None, &tree, None);
    assert_eq!(seen.take(), "dark");
}

#[test]
fn nested_providers_shadow_outer() {
    let seen = Rc::new(Cell::new(String::new()));
    let leaf = component(
        Leaf {
            seen: Rc::clone(&seen),
        },
        (),
    );

    let inner_wrapper: Element = border(leaf).provide(&THEME, "inner".to_string());
    let tree: Element = vstack((inner_wrapper,)).provide(&THEME, "outer".to_string());

    let mut r = Reconciler::new(RecordingBackend::new());
    let _ = reconcile(&mut r, None, &tree, None);
    assert_eq!(seen.take(), "inner");
}

#[test]
fn distinct_contexts_are_resolved_independently() {
    let seen_theme = Rc::new(Cell::new(String::new()));
    let seen_level = Rc::new(Cell::new(0));

    struct Observer {
        theme: Rc<Cell<String>>,
        level: Rc<Cell<i32>>,
    }
    impl Component for Observer {
        fn render(&self, _props: &(), cx: &mut RenderCx) -> Element {
            self.theme.set(cx.use_context(&THEME));
            self.level.set(cx.use_context(&LEVEL));
            text_block("obs").into()
        }
    }

    let obs = component(
        Observer {
            theme: Rc::clone(&seen_theme),
            level: Rc::clone(&seen_level),
        },
        (),
    );
    let tree: Element = obs.provide(&THEME, "neon".to_string()).provide(&LEVEL, 3);

    let mut r = Reconciler::new(RecordingBackend::new());
    let _ = reconcile(&mut r, None, &tree, None);
    assert_eq!(seen_theme.take(), "neon");
    assert_eq!(seen_level.get(), 3);
}

#[test]
fn changing_provided_value_triggers_subscriber_rerender() {
    let renders = Rc::new(Cell::new(0));
    let seen = Rc::new(Cell::new(String::new()));

    struct Observer {
        renders: Rc<Cell<u32>>,
        seen: Rc<Cell<String>>,
    }
    impl Component for Observer {
        fn render(&self, _props: &(), cx: &mut RenderCx) -> Element {
            self.renders.set(self.renders.get() + 1);
            let t = cx.use_context(&THEME);
            self.seen.set(t.clone());
            text_block(t).into()
        }
    }

    let tree_a: Element = component(
        Observer {
            renders: Rc::clone(&renders),
            seen: Rc::clone(&seen),
        },
        (),
    )
    .provide(&THEME, "v1".to_string());
    let tree_b: Element = component(
        Observer {
            renders: Rc::clone(&renders),
            seen: Rc::clone(&seen),
        },
        (),
    )
    .provide(&THEME, "v2".to_string());

    let mut r = Reconciler::new(RecordingBackend::new());
    let id = reconcile(&mut r, None, &tree_a, None).unwrap();
    assert_eq!(renders.get(), 1);
    assert_eq!(seen.take(), "v1");

    let _ = reconcile(&mut r, Some(&tree_a), &tree_b, Some(id));
    assert!(
        renders.get() >= 2,
        "observer must re-render on provision change"
    );
    assert_eq!(seen.take(), "v2");
}

struct AObserver {
    renders: Rc<Cell<u32>>,
}
impl Component for AObserver {
    fn render(&self, _props: &(), cx: &mut RenderCx) -> Element {
        self.renders.set(self.renders.get() + 1);
        let _t = cx.use_context(&THEME);
        text_block("a").into()
    }
}

struct BObserver {
    renders: Rc<Cell<u32>>,
}
impl Component for BObserver {
    fn render(&self, _props: &(), cx: &mut RenderCx) -> Element {
        self.renders.set(self.renders.get() + 1);
        let _l = cx.use_context(&LEVEL);
        text_block("b").into()
    }
}

#[test]
fn changing_a_provision_only_rerenders_consumers_that_read_it() {
    let a_renders = Rc::new(Cell::new(0));
    let b_renders = Rc::new(Cell::new(0));

    let build = |theme: &'static str, level: i32| -> Element {
        let a = component(
            AObserver {
                renders: Rc::clone(&a_renders),
            },
            (),
        );
        let b = component(
            BObserver {
                renders: Rc::clone(&b_renders),
            },
            (),
        );
        vstack((a, b))
            .provide(&THEME, theme.to_string())
            .provide(&LEVEL, level)
    };

    let tree_v1 = build("light", 0);
    let mut r = Reconciler::new(RecordingBackend::new());
    let id = reconcile(&mut r, None, &tree_v1, None).unwrap();
    assert_eq!(a_renders.get(), 1, "a renders once on mount");
    assert_eq!(b_renders.get(), 1, "b renders once on mount");

    let tree_v2 = build("dark", 0);
    let _ = reconcile(&mut r, Some(&tree_v1), &tree_v2, Some(id));
    assert_eq!(
        a_renders.get(),
        2,
        "AObserver (reads THEME) must re-render when THEME changes"
    );
    assert_eq!(
        b_renders.get(),
        1,
        "BObserver (reads only LEVEL) must NOT re-render when only THEME changes"
    );

    let tree_v3 = build("dark", 7);
    let _ = reconcile(&mut r, Some(&tree_v2), &tree_v3, Some(id));
    assert_eq!(
        a_renders.get(),
        2,
        "AObserver (reads only THEME) must NOT re-render when only LEVEL changes"
    );
    assert_eq!(
        b_renders.get(),
        2,
        "BObserver (reads LEVEL) must re-render when LEVEL changes"
    );
}

#[test]
fn unchanged_provisions_dont_rerender_any_descendant_components() {
    let a_renders = Rc::new(Cell::new(0));
    let b_renders = Rc::new(Cell::new(0));

    let mk = || -> Element {
        let a = component(
            AObserver {
                renders: Rc::clone(&a_renders),
            },
            (),
        );
        let b = component(
            BObserver {
                renders: Rc::clone(&b_renders),
            },
            (),
        );
        vstack((a, b))
            .provide(&THEME, "fixed".to_string())
            .provide(&LEVEL, 1)
    };

    let tree_v1 = mk();
    let tree_v2 = mk();

    let mut r = Reconciler::new(RecordingBackend::new());
    let id = reconcile(&mut r, None, &tree_v1, None).unwrap();
    assert_eq!(a_renders.get(), 1);
    assert_eq!(b_renders.get(), 1);

    let _ = reconcile(&mut r, Some(&tree_v1), &tree_v2, Some(id));

    assert!(
        a_renders.get() <= 2,
        "AObserver re-rendered too many times: {}",
        a_renders.get()
    );
    assert!(
        b_renders.get() <= 2,
        "BObserver re-rendered too many times: {}",
        b_renders.get()
    );
}

#[test]
fn nested_providers_leave_forced_components_empty_after_update() {
    let renders = Rc::new(Cell::new(0u32));
    let seen_theme = Rc::new(Cell::new(String::new()));
    let seen_level = Rc::new(Cell::new(0));

    struct Observer {
        renders: Rc<Cell<u32>>,
        theme: Rc<Cell<String>>,
        level: Rc<Cell<i32>>,
    }
    impl Component for Observer {
        fn render(&self, _props: &(), cx: &mut RenderCx) -> Element {
            self.renders.set(self.renders.get() + 1);
            self.theme.set(cx.use_context(&THEME));
            self.level.set(cx.use_context(&LEVEL));
            text_block("obs").into()
        }
    }

    let build = |theme: &'static str, level: i32| -> Element {
        component(
            Observer {
                renders: Rc::clone(&renders),
                theme: Rc::clone(&seen_theme),
                level: Rc::clone(&seen_level),
            },
            (),
        )
        .provide(&LEVEL, level)
        .provide(&THEME, theme.to_string())
    };

    let tree_v1 = build("light", 1);
    let mut r = Reconciler::new(RecordingBackend::new());
    let id = reconcile(&mut r, None, &tree_v1, None).unwrap();
    assert_eq!(renders.get(), 1);
    assert_eq!(seen_theme.take(), "light");
    assert_eq!(seen_level.get(), 1);
    assert_eq!(
        r.debug_forced_components_len(),
        0,
        "forced_components must be drained after mount"
    );

    let tree_v2 = build("dark", 2);
    let _ = reconcile(&mut r, Some(&tree_v1), &tree_v2, Some(id));
    assert_eq!(seen_theme.take(), "dark");
    assert_eq!(seen_level.get(), 2);
    assert_eq!(
        r.debug_forced_components_len(),
        0,
        "forced_components must be drained after a nested-provider update; \
         a non-zero value indicates ids leaked across the inner→outer \
         provider scopes"
    );

    let renders_before = renders.get();
    let tree_v3 = build("dark", 2);
    let _ = reconcile(&mut r, Some(&tree_v2), &tree_v3, Some(id));
    assert_eq!(
        renders.get(),
        renders_before,
        "no provisions changed, so observer must not re-render; \
         extra renders indicate a `forced_components` leak"
    );
    assert_eq!(r.debug_forced_components_len(), 0);
}

#[test]
fn component_outside_provider_subtree_never_force_rerenders() {
    let inside_renders = Rc::new(Cell::new(0));
    let outside_renders = Rc::new(Cell::new(0));

    let build = |theme: &'static str| -> Element {
        let inside = component(
            AObserver {
                renders: Rc::clone(&inside_renders),
            },
            (),
        );
        let outside = component(
            AObserver {
                renders: Rc::clone(&outside_renders),
            },
            (),
        );

        let provider_scope: Element = inside.provide(&THEME, theme.to_string());
        vstack((provider_scope, outside)).into()
    };

    let v1 = build("a");
    let mut r = Reconciler::new(RecordingBackend::new());
    let id = reconcile(&mut r, None, &v1, None).unwrap();
    assert_eq!(inside_renders.get(), 1);
    assert_eq!(outside_renders.get(), 1);

    let v2 = build("b");
    let _ = reconcile(&mut r, Some(&v1), &v2, Some(id));
    assert_eq!(
        inside_renders.get(),
        2,
        "the inside-scope consumer must re-render on theme change"
    );
    assert_eq!(
        outside_renders.get(),
        1,
        "the outside-scope consumer must NOT re-render — it doesn't \
         see the changed provision"
    );
}

#[test]
fn reproviding_same_value_does_not_force_consumer_rerender() {
    let renders = Rc::new(Cell::new(0));
    let seen = Rc::new(Cell::new(String::new()));

    struct Observer {
        renders: Rc<Cell<u32>>,
        seen: Rc<Cell<String>>,
    }
    impl Component for Observer {
        fn render(&self, _props: &(), cx: &mut RenderCx) -> Element {
            self.renders.set(self.renders.get() + 1);
            let t = cx.use_context(&THEME);
            self.seen.set(t.clone());
            text_block(t).into()
        }
    }

    let build = || -> Element {
        component(
            Observer {
                renders: Rc::clone(&renders),
                seen: Rc::clone(&seen),
            },
            (),
        )
        .provide(&THEME, "fixed".to_string())
    };

    let tree_v1 = build();
    let tree_v2 = build();

    let mut r = Reconciler::new(RecordingBackend::new());
    let id = reconcile(&mut r, None, &tree_v1, None).unwrap();
    assert_eq!(renders.get(), 1, "first render");
    seen.take();

    let _ = reconcile(&mut r, Some(&tree_v1), &tree_v2, Some(id));
    assert_eq!(
        renders.get(),
        1,
        "audit §7.1.15: a re-`provide(..)` with the same value must \
         compare equal via the type-monomorphized PartialEq path; the \
         observer must NOT re-render. Got {} renders.",
        renders.get()
    );
}

struct PanicOnRender;

impl Component for PanicOnRender {
    fn render(&self, _props: &(), _cx: &mut RenderCx) -> Element {
        panic!("§7.1.12: deliberate panic from PanicOnRender");
    }
}

#[test]
fn provider_push_pop_recovers_after_panic_in_child() {
    use windows_reactor::core::error_boundary::error_boundary;

    let post_panic_seen = Rc::new(Cell::new(String::new()));

    let panicking_provider: Element =
        component(PanicOnRender, ()).provide(&THEME, "poison".to_string());
    let bounded = error_boundary(panicking_provider, |_msg: &str| -> Element {
        text_block("recovered").into()
    });

    let post_leaf = component(
        Leaf {
            seen: Rc::clone(&post_panic_seen),
        },
        (),
    );

    let tree: Element = vstack((bounded, post_leaf)).into();

    let mut r = Reconciler::new(RecordingBackend::new());
    let _ = reconcile(&mut r, None, &tree, None);

    assert_eq!(
        post_panic_seen.take(),
        "default-theme",
        "audit §7.1.12: a panic inside a Provider's child must not \
         leave provisions on the context stack — the post-boundary \
         sibling must see the default value, not the panicking \
         provider's poisoned value."
    );
}
