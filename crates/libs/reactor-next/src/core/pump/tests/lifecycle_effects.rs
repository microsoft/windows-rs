//! Component lifecycle and effect commit/cleanup contract tests for [`Pump`].

use super::super::*;
use super::support::*;
use crate::native::*;
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn component_effects_commit_after_mount_and_cleanup_once() {
    #[derive(Clone)]
    struct Props {
        log: Rc<RefCell<Vec<String>>>,
        sender: Rc<RefCell<Option<LocalSender<u32>>>>,
    }

    impl PartialEq for Props {
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
        type Props = Props;

        fn create(props: &Props, cx: &mut ComponentContext<Self>) -> Self {
            *props.sender.borrow_mut() = Some(cx.sender());
            Self {
                log: Rc::clone(&props.log),
                value: 0,
            }
        }

        fn update(&mut self, message: u32, _cx: &mut ComponentContext<Self>) {
            self.value = message;
        }

        fn changed(&mut self, _props: &Props, _cx: &mut ComponentContext<Self>) {}

        fn view(&self, cx: &mut ViewContext<Self>) -> View {
            let log = Rc::clone(&self.log);
            let value = self.value;
            cx.use_effect(value, move || {
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
    pump.mount_view(View::component::<EffectComponent>(Props {
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
fn component_host_retries_initial_property_failure_without_a_message() {
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
        Err(PumpError::PropertyApplyFailed(_))
    ));
    assert!(pump.native_work_pending());

    assert_eq!(pump.dispatch_components(64), Ok(0));
    assert!(!pump.retry_pending());
    assert!(!pump.native_work_pending());
}

#[test]
fn failed_component_recovery_does_not_commit_pending_effects() {
    #[derive(Clone)]
    struct Props {
        alternate: bool,
        log: Rc<RefCell<Vec<String>>>,
    }

    impl PartialEq for Props {
        fn eq(&self, other: &Self) -> bool {
            self.alternate == other.alternate && Rc::ptr_eq(&self.log, &other.log)
        }
    }

    struct EffectComponent(Props);

    impl Component for EffectComponent {
        type Message = ();
        type Props = Props;

        fn create(props: &Props, _cx: &mut ComponentContext<Self>) -> Self {
            Self(props.clone())
        }

        fn update(&mut self, _message: (), _cx: &mut ComponentContext<Self>) {}

        fn changed(&mut self, props: &Props, _cx: &mut ComponentContext<Self>) {
            self.0 = props.clone();
        }

        fn view(&self, cx: &mut ViewContext<Self>) -> View {
            let alternate = self.0.alternate;
            let log = Rc::clone(&self.0.log);
            cx.use_effect(alternate, move || {
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
    pump.mount_view(View::component::<EffectComponent>(Props {
        alternate: false,
        log: Rc::clone(&log),
    }))
    .unwrap();
    pump.runtime_mut().fail_after(0, 0);
    pump.runtime_mut().fail_after(1, 0);

    assert!(matches!(
        pump.update_view(View::component::<EffectComponent>(Props {
            alternate: true,
            log: Rc::clone(&log),
        })),
        Err(PumpError::RecoveryFailed(_))
    ));
    assert_eq!(&*log.borrow(), &["setup false", "cleanup false"]);
}

#[test]
fn retired_component_effects_cleanup_child_first() {
    #[derive(Clone)]
    struct Props {
        child: bool,
        log: Rc<RefCell<Vec<&'static str>>>,
        name: &'static str,
    }

    impl PartialEq for Props {
        fn eq(&self, other: &Self) -> bool {
            self.child == other.child
                && self.name == other.name
                && Rc::ptr_eq(&self.log, &other.log)
        }
    }

    struct EffectTree(Props);

    impl Component for EffectTree {
        type Message = ();
        type Props = Props;

        fn create(props: &Props, _cx: &mut ComponentContext<Self>) -> Self {
            Self(props.clone())
        }

        fn update(&mut self, _message: (), _cx: &mut ComponentContext<Self>) {}

        fn changed(&mut self, props: &Props, _cx: &mut ComponentContext<Self>) {
            self.0 = props.clone();
        }

        fn view(&self, cx: &mut ViewContext<Self>) -> View {
            let cleanup = self.0.name;
            let log = Rc::clone(&self.0.log);
            cx.use_effect((), move || {
                Some(Box::new(move || {
                    log.borrow_mut().push(cleanup);
                }))
            });
            if self.0.child {
                View::component::<Self>(Props {
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
    pump.mount_view(View::component::<EffectTree>(Props {
        child: true,
        log: Rc::clone(&log),
        name: "parent",
    }))
    .unwrap();

    pump.update_view(View::native(TextBlock::new())).unwrap();
    assert_eq!(&*log.borrow(), &["child", "parent"]);
}
