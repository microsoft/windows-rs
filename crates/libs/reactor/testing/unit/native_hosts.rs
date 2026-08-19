use std::cell::RefCell;
use std::rc::Rc;

use super::*;

fn native_node(reactor: &Reactor<RecordingRuntime>, native_kind: NativeKind) -> NodeId {
    reactor
        .engine()
        .runtime()
        .batches()
        .iter()
        .flatten()
        .find_map(|command| match command {
            Command::Create { id, kind } if *kind == native_kind => Some(*id),
            _ => None,
        })
        .unwrap()
}

#[test]
fn composition_host_reference_is_owner_bound() {
    let mounted = Rc::new(RefCell::new(None::<State<bool>>));
    let reference = Rc::new(RefCell::new(None::<CompositionHostRef<()>>));
    let mounted_render = Rc::clone(&mounted);
    let reference_render = Rc::clone(&reference);
    let root = component(move |cx| {
        let show = cx.use_state(|| true);
        *mounted_render.borrow_mut() = Some(show.clone());
        let host = cx.use_composition_host_ref::<()>();
        *reference_render.borrow_mut() = Some(host.clone());
        if show.value() {
            CompositionHost::new(
                &host,
                |_| -> windows_core::Result<CompositionContent<()>> {
                    panic!("model runtime does not invoke native factories")
                },
                |_, _| Ok(()),
            )
            .build()
        } else {
            TextBlock::new("removed").build()
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let target = native_node(&reactor, NativeKind::CompositionHost);
    let host = reference.borrow().as_ref().unwrap().clone();
    assert!(host.is_mounted());
    assert!(host.update(|_| Ok(())));
    reactor.pump();
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .any(|command| matches!(
                command,
                Command::Update {
                    update: NativeUpdate::Control(ControlUpdate::CompositionHost(update)),
                    ..
                } if matches!(update.as_ref(), CompositionHostUpdate::Action(_))
            ))
    );

    mounted.borrow().as_ref().unwrap().set(false);
    reactor.pump();
    assert!(!host.is_mounted());
    assert!(!host.update(|_| Ok(())));
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::CompositionLayout {
            target,
            width: 1.0,
            height: 1.0,
            rasterization_scale: 1.0,
        });
    reactor.pump();
}

#[cfg(feature = "canvas")]
#[test]
fn swap_chain_host_reference_is_owner_bound() {
    let mounted = Rc::new(RefCell::new(None::<State<bool>>));
    let reference = Rc::new(RefCell::new(None::<SwapChainHostRef<()>>));
    let mounted_render = Rc::clone(&mounted);
    let reference_render = Rc::clone(&reference);
    let root = component(move |cx| {
        let show = cx.use_state(|| true);
        *mounted_render.borrow_mut() = Some(show.clone());
        let host = cx.use_swap_chain_host_ref::<()>();
        *reference_render.borrow_mut() = Some(host.clone());
        if show.value() {
            SwapChainHost::new(
                &host,
                |_| -> windows_core::Result<SwapChainHostContent<()>> {
                    panic!("model runtime does not invoke native factories")
                },
                |_, _, _| Ok(()),
                |_, _, _| Ok(()),
            )
            .build()
        } else {
            TextBlock::new("removed").build()
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let target = native_node(&reactor, NativeKind::SwapChainHost);
    let host = reference.borrow().as_ref().unwrap().clone();
    assert!(host.is_mounted());
    assert!(host.update(|_| Ok(())));
    assert!(host.invalidate());
    reactor.pump();
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .any(|command| matches!(
                command,
                Command::Update {
                    update: NativeUpdate::Control(ControlUpdate::SwapChainHost(update)),
                    ..
                } if matches!(
                    update.as_ref(),
                    SwapChainHostUpdate::Action(canvas::SwapChainHostAction::Invalidate)
                )
            ))
    );

    mounted.borrow().as_ref().unwrap().set(false);
    reactor.pump();
    assert!(!host.is_mounted());
    assert!(!host.update(|_| Ok(())));
    assert!(!host.invalidate());
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::SwapChainHostFrame { target });
    reactor.pump();
}

#[cfg(feature = "webview")]
#[test]
fn webview_host_reference_is_owner_bound() {
    let mounted = Rc::new(RefCell::new(None::<State<bool>>));
    let reference = Rc::new(RefCell::new(None::<WebViewRef>));
    let created = Rc::new(RefCell::new(Vec::new()));
    let navigation = Rc::new(RefCell::new(Vec::new()));
    let mounted_render = Rc::clone(&mounted);
    let reference_render = Rc::clone(&reference);
    let created_render = Rc::clone(&created);
    let navigation_render = Rc::clone(&navigation);
    let root = component(move |cx| {
        let show = cx.use_state(|| true);
        *mounted_render.borrow_mut() = Some(show.clone());
        let host = cx.use_webview_ref();
        *reference_render.borrow_mut() = Some(host.clone());
        if show.value() {
            WebViewHost::new(&host)
                .source("https://example.com")
                .on_created({
                    let created = Rc::clone(&created_render);
                    move |result| created.borrow_mut().push(result.is_ok())
                })
                .on_navigation_completed({
                    let navigation = Rc::clone(&navigation_render);
                    move |event| navigation.borrow_mut().push(event)
                })
                .build()
        } else {
            TextBlock::new("removed").build()
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let target = native_node(&reactor, NativeKind::WebViewHost);
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::WebViewCreated {
            target,
            result: Ok(()),
        });
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::WebViewNavigationCompleted {
            target,
            navigation_id: 7,
            is_success: true,
            source: "https://example.com/next".into(),
        });
    reactor.pump();
    assert_eq!(*created.borrow(), [true]);
    assert_eq!(
        *navigation.borrow(),
        [WebViewNavigationCompleted {
            navigation_id: 7,
            is_success: true,
            source: "https://example.com/next".to_string(),
        }]
    );
    let host = reference.borrow().as_ref().unwrap().clone();
    assert!(host.is_mounted());
    assert!(host.reload());
    reactor.pump();
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .any(|command| matches!(
                command,
                Command::Update {
                    update: NativeUpdate::Control(ControlUpdate::WebViewHost(
                        WebViewHostUpdate::Action(webview::WebViewAction::Reload)
                    )),
                    ..
                }
            ))
    );

    mounted.borrow().as_ref().unwrap().set(false);
    reactor.pump();
    assert!(!host.is_mounted());
    assert!(!host.reload());
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::WebViewCreated {
            target,
            result: Err(windows_core::Error::new(
                windows_core::HRESULT(0x80004005_u32 as i32),
                "stale",
            )),
        });
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::WebViewNavigationCompleted {
            target,
            navigation_id: 8,
            is_success: true,
            source: "https://stale.example".into(),
        });
    reactor.pump();
    assert_eq!(created.borrow().len(), 1);
    assert_eq!(navigation.borrow().len(), 1);
}
