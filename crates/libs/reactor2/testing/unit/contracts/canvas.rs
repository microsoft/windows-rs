#[cfg(feature = "canvas")]
use super::*;

#[cfg(feature = "canvas")]
struct CanvasLoopRuntime {
    inner: RecordingRuntime,
    events: Vec<NativeEvent>,
    on_frame: Rc<RefCell<Option<Box<dyn Fn()>>>>,
    frames: Rc<Cell<usize>>,
}

#[cfg(feature = "canvas")]
impl NativeRuntime for CanvasLoopRuntime {
    fn apply(&mut self, commands: &[Command]) {
        self.inner.apply(commands);
        for command in commands {
            match command {
                Command::Update {
                    id,
                    update: NativeUpdate::Control(ControlUpdate::SwapChainCanvas(update)),
                } if matches!(
                    update.as_ref(),
                    SwapChainCanvasUpdate::Canvas(CanvasUpdate::Invalidate(_))
                ) =>
                {
                    self.events.push(NativeEvent::CanvasFrame { target: *id });
                }
                Command::RunCanvasFrame { .. } => {
                    self.frames.set(self.frames.get() + 1);
                    if let Some(callback) = self.on_frame.borrow().as_ref() {
                        callback();
                    }
                }
                _ => {}
            }
        }
    }

    fn drain_events(&mut self) -> Vec<NativeEvent> {
        let mut events = self.inner.drain_events();
        events.append(&mut self.events);
        events
    }
}

#[cfg(feature = "canvas")]
#[test]
fn canvas_invalidation_queues_native_work_without_rerendering() {
    let renders = Rc::new(Cell::new(0));
    let invalidator = Rc::new(RefCell::new(None::<CanvasInvalidator>));
    let renders_for_render = Rc::clone(&renders);
    let invalidator_for_render = Rc::clone(&invalidator);
    let root = component(move |cx| {
        renders_for_render.set(renders_for_render.get() + 1);
        let current = cx.use_canvas_invalidator();
        *invalidator_for_render.borrow_mut() = Some(current.clone());
        swap_chain_canvas_invalidated(&current, |_| Ok(())).build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    assert_eq!(renders.get(), 1);

    let current = invalidator.borrow().as_ref().unwrap().clone();
    current.invalidate();
    current.invalidate();
    current.invalidate();
    reactor.pump();

    assert_eq!(renders.get(), 1);
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
                    update: NativeUpdate::Control(ControlUpdate::SwapChainCanvas(update)),
                    ..
                } if matches!(
                    update.as_ref(),
                    SwapChainCanvasUpdate::Canvas(CanvasUpdate::Invalidate(3))
                )
            ))
    );
}

#[cfg(feature = "canvas")]
#[test]
fn canvas_image_invalidation_uses_the_shared_scheduler_without_rerendering() {
    let renders = Rc::new(Cell::new(0));
    let invalidator = Rc::new(RefCell::new(None::<CanvasInvalidator>));
    let renders_for_render = Rc::clone(&renders);
    let invalidator_for_render = Rc::clone(&invalidator);
    let root = component(move |cx| {
        renders_for_render.set(renders_for_render.get() + 1);
        let current = cx.use_canvas_invalidator();
        *invalidator_for_render.borrow_mut() = Some(current.clone());
        canvas_image_invalidated(&current, |_| Ok(())).build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();

    let current = invalidator.borrow().as_ref().unwrap().clone();
    current.invalidate();
    current.invalidate();
    reactor.pump();

    assert_eq!(renders.get(), 1);
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
                    update: NativeUpdate::Control(ControlUpdate::CanvasImage(
                        CanvasUpdate::Invalidate(2)
                    )),
                    ..
                }
            ))
    );
}

#[cfg(feature = "canvas")]
#[test]
fn replacing_swap_chain_canvas_invalidator_resets_native_revision() {
    let use_second = Rc::new(RefCell::new(None::<State<bool>>));
    let first = Rc::new(RefCell::new(None::<CanvasInvalidator>));
    let second = Rc::new(RefCell::new(None::<CanvasInvalidator>));
    let use_second_for_render = Rc::clone(&use_second);
    let first_for_render = Rc::clone(&first);
    let second_for_render = Rc::clone(&second);
    let root = component(move |cx| {
        let current_use_second = cx.use_state(|| false);
        let current_first = cx.use_canvas_invalidator();
        let current_second = cx.use_canvas_invalidator();
        *use_second_for_render.borrow_mut() = Some(current_use_second.clone());
        *first_for_render.borrow_mut() = Some(current_first.clone());
        *second_for_render.borrow_mut() = Some(current_second.clone());
        let invalidator = if current_use_second.get().unwrap() {
            &current_second
        } else {
            &current_first
        };
        swap_chain_canvas_invalidated(invalidator, |_| Ok(())).build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();

    let first = first.borrow().as_ref().unwrap().clone();
    for _ in 0..10 {
        first.invalidate();
    }
    reactor.pump();
    assert!(use_second.borrow().as_ref().unwrap().try_set(true));
    reactor.pump();

    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .any(|command| matches!(
                command,
                Command::Update {
                    update: NativeUpdate::Control(ControlUpdate::SwapChainCanvas(update)),
                    ..
                } if matches!(
                    update.as_ref(),
                    SwapChainCanvasUpdate::Canvas(CanvasUpdate::Rebind {
                            invalidation_revision: 0,
                            ..
                        })
                )
            ))
    );

    second.borrow().as_ref().unwrap().invalidate();
    reactor.pump();
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .any(|command| matches!(
                command,
                Command::Update {
                    update: NativeUpdate::Control(ControlUpdate::SwapChainCanvas(update)),
                    ..
                } if matches!(
                    update.as_ref(),
                    SwapChainCanvasUpdate::Canvas(CanvasUpdate::Invalidate(1))
                )
            ))
    );
}

#[cfg(feature = "canvas")]
#[test]
fn replacing_canvas_image_invalidator_resets_native_revision() {
    let use_second = Rc::new(RefCell::new(None::<State<bool>>));
    let first = Rc::new(RefCell::new(None::<CanvasInvalidator>));
    let second = Rc::new(RefCell::new(None::<CanvasInvalidator>));
    let use_second_for_render = Rc::clone(&use_second);
    let first_for_render = Rc::clone(&first);
    let second_for_render = Rc::clone(&second);
    let root = component(move |cx| {
        let current_use_second = cx.use_state(|| false);
        let current_first = cx.use_canvas_invalidator();
        let current_second = cx.use_canvas_invalidator();
        *use_second_for_render.borrow_mut() = Some(current_use_second.clone());
        *first_for_render.borrow_mut() = Some(current_first.clone());
        *second_for_render.borrow_mut() = Some(current_second.clone());
        let invalidator = if current_use_second.get().unwrap() {
            &current_second
        } else {
            &current_first
        };
        canvas_image_invalidated(invalidator, |_| Ok(())).build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();

    let first = first.borrow().as_ref().unwrap().clone();
    for _ in 0..10 {
        first.invalidate();
    }
    reactor.pump();
    assert!(use_second.borrow().as_ref().unwrap().try_set(true));
    reactor.pump();

    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .any(|command| matches!(
                command,
                Command::Update {
                    update: NativeUpdate::Control(ControlUpdate::CanvasImage(
                        CanvasUpdate::Rebind {
                            invalidation_revision: 0,
                            ..
                        }
                    )),
                    ..
                }
            ))
    );

    second.borrow().as_ref().unwrap().invalidate();
    reactor.pump();
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .any(|command| matches!(
                command,
                Command::Update {
                    update: NativeUpdate::Control(ControlUpdate::CanvasImage(
                        CanvasUpdate::Invalidate(1)
                    )),
                    ..
                }
            ))
    );
}

#[cfg(feature = "canvas")]
#[test]
fn removed_canvas_image_invalidators_stop_scheduling_work() {
    let visible = Rc::new(RefCell::new(None::<State<bool>>));
    let invalidator = Rc::new(RefCell::new(None::<CanvasInvalidator>));
    let visible_for_render = Rc::clone(&visible);
    let invalidator_for_render = Rc::clone(&invalidator);
    let root = component(move |cx| {
        let show = cx.use_state(|| true);
        let current = cx.use_canvas_invalidator();
        *visible_for_render.borrow_mut() = Some(show.clone());
        *invalidator_for_render.borrow_mut() = Some(current.clone());
        if show.get().unwrap() {
            canvas_image_invalidated(&current, |_| Ok(())).build()
        } else {
            text_block("removed")
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let current = invalidator.borrow().as_ref().unwrap().clone();

    assert!(visible.borrow().as_ref().unwrap().try_set(false));
    reactor.pump();
    let batches = reactor.engine().runtime().batches().len();
    current.invalidate();
    reactor.pump();

    assert_eq!(reactor.engine().runtime().batches().len(), batches);
}

#[cfg(feature = "canvas")]
#[test]
fn canvas_image_native_events_route_through_typed_commands() {
    let mut reactor = Reactor::new(
        RecordingRuntime::default(),
        canvas_image(|_| Ok(())).build(),
    );
    reactor.pump();
    let target = native_node(&reactor, NativeKind::CanvasImage);

    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::CanvasImageLayout {
            target,
            width: 120.0,
            height: 80.0,
            scale: 1.25,
        });
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::CanvasImageFrame { target });
    reactor.pump();

    let commands = reactor.engine().runtime().batches().iter().flatten();
    assert!(commands.clone().any(|command| matches!(
        command,
        Command::ApplyCanvasImageLayout {
            target: current,
            width,
            height,
            scale,
        } if *current == target
            && width.to_bits() == 120.0f32.to_bits()
            && height.to_bits() == 80.0f32.to_bits()
            && scale.to_bits() == 1.25f32.to_bits()
    )));
    assert!(commands.clone().any(
        |command| matches!(command, Command::RunCanvasImageFrame { target: current } if *current == target)
    ));
}

#[cfg(feature = "canvas")]
#[test]
fn stale_canvas_image_events_are_ignored_after_removal() {
    let visible = Rc::new(RefCell::new(None::<State<bool>>));
    let visible_for_render = Rc::clone(&visible);
    let root = component(move |cx| {
        let show = cx.use_state(|| true);
        *visible_for_render.borrow_mut() = Some(show.clone());
        if show.get().unwrap() {
            canvas_image(|_| Ok(())).build()
        } else {
            text_block("removed")
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let target = native_node(&reactor, NativeKind::CanvasImage);

    assert!(visible.borrow().as_ref().unwrap().try_set(false));
    reactor.pump();
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::CanvasImageFrame { target });
    reactor.pump();

    assert!(reactor.engine().is_valid());
}

#[cfg(feature = "canvas")]
#[test]
fn removed_canvas_invalidators_stop_scheduling_work() {
    let visible = Rc::new(RefCell::new(None::<State<bool>>));
    let invalidator = Rc::new(RefCell::new(None::<CanvasInvalidator>));
    let visible_for_render = Rc::clone(&visible);
    let invalidator_for_render = Rc::clone(&invalidator);
    let root = component(move |cx| {
        let show = cx.use_state(|| true);
        let current = cx.use_canvas_invalidator();
        *visible_for_render.borrow_mut() = Some(show.clone());
        *invalidator_for_render.borrow_mut() = Some(current.clone());
        if show.get().unwrap() {
            swap_chain_canvas_invalidated(&current, |_| Ok(())).build()
        } else {
            text_block("removed")
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let current = invalidator.borrow().as_ref().unwrap().clone();

    assert!(visible.borrow().as_ref().unwrap().try_set(false));
    reactor.pump();
    let batches = reactor.engine().runtime().batches().len();
    current.invalidate();
    reactor.pump();

    assert_eq!(reactor.engine().runtime().batches().len(), batches);
}

#[cfg(feature = "canvas")]
#[test]
fn canvas_invalidator_survives_keyed_replacement() {
    let key = Rc::new(RefCell::new(None::<State<u64>>));
    let invalidator = Rc::new(RefCell::new(None::<CanvasInvalidator>));
    let key_for_render = Rc::clone(&key);
    let invalidator_for_render = Rc::clone(&invalidator);
    let root = component(move |cx| {
        let current_key = cx.use_state(|| 1u64);
        let current = cx.use_canvas_invalidator();
        *key_for_render.borrow_mut() = Some(current_key.clone());
        *invalidator_for_render.borrow_mut() = Some(current.clone());
        stack_panel([
            swap_chain_canvas_invalidated(&current, |_| Ok(()))
                .build()
                .key(current_key.get().unwrap()),
            text_block("anchor").key(99),
        ])
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();

    assert!(key.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    invalidator.borrow().as_ref().unwrap().invalidate();
    reactor.pump();

    let canvases = created_nodes(&reactor, NativeKind::SwapChainCanvas);
    assert_eq!(canvases.len(), 2);
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
                    id,
                    update: NativeUpdate::Control(ControlUpdate::SwapChainCanvas(update)),
                } if *id == canvases[1]
                    && matches!(
                        update.as_ref(),
                        SwapChainCanvasUpdate::Canvas(CanvasUpdate::Invalidate(1))
                    )
            ))
    );
}

#[cfg(feature = "canvas")]
#[test]
fn self_invalidating_canvas_yields_after_one_frame_per_pump() {
    let invalidator = Rc::new(RefCell::new(None::<CanvasInvalidator>));
    let invalidator_for_render = Rc::clone(&invalidator);
    let on_frame = Rc::new(RefCell::new(None::<Box<dyn Fn()>>));
    let frames = Rc::new(Cell::new(0));
    let runtime = CanvasLoopRuntime {
        inner: RecordingRuntime::default(),
        events: Vec::new(),
        on_frame: Rc::clone(&on_frame),
        frames: Rc::clone(&frames),
    };
    let root = component(move |cx| {
        let current = cx.use_canvas_invalidator();
        *invalidator_for_render.borrow_mut() = Some(current.clone());
        swap_chain_canvas_invalidated(&current, |_| Ok(())).build()
    });
    let mut reactor = Reactor::new(runtime, root);
    reactor.pump();

    let current = invalidator.borrow().as_ref().unwrap().clone();
    *on_frame.borrow_mut() = Some(Box::new({
        let current = current.clone();
        move || current.invalidate()
    }));
    current.invalidate();

    reactor.pump();
    assert_eq!(frames.get(), 1);
    reactor.pump();
    assert_eq!(frames.get(), 2);
}
