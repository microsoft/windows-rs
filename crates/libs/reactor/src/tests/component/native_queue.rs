use super::*;

struct QueueCountingAdapter {
    inner: RecordingAdapter,
    visits: Cell<usize>,
}

impl QueueCountingAdapter {
    fn new() -> Self {
        Self {
            inner: RecordingAdapter::new(),
            visits: Cell::new(0),
        }
    }
}

impl Adapter for QueueCountingAdapter {
    fn peek_native_event(&self) -> Option<NativeEvent> {
        self.visits.set(self.visits.get() + 1);
        self.inner.peek_native_event()
    }

    fn pop_native_event(&mut self) -> Option<NativeEvent> {
        self.visits.set(self.visits.get() + 1);
        self.inner.pop_native_event()
    }

    fn apply(&mut self, mutations: &[Mutation]) {
        self.inner.apply(mutations);
    }
}

type DeliveryLog = Rc<RefCell<Vec<(&'static str, u32)>>>;

struct NativeQueueBurst {
    callback: Callback<PointerEventInfo>,
    log: DeliveryLog,
    value: u32,
}

impl Component for NativeQueueBurst {
    type Input = DeliveryLog;
    type Message = u32;

    fn create(log: &Self::Input, context: &ComponentContext<Self>) -> Self {
        let sender = context.sender();
        let called = Rc::clone(log);
        Self {
            callback: Callback::new(move |info: PointerEventInfo| {
                called.borrow_mut().push(("callback", info.pointer_id));
                assert!(sender.send(info.pointer_id));
            }),
            log: Rc::clone(log),
            value: 0,
        }
    }

    fn update(&mut self, value: u32, _: &ComponentContext<Self>) {
        self.log.borrow_mut().push(("update", value));
        self.value = value;
    }

    fn view(&self, _: &Self::Input, _: &mut ViewContext<Self>) -> View {
        Border::new()
            .on_pointer_moved(self.callback.clone())
            .content(TextBlock::new().text(self.value.to_string()))
            .into()
    }
}

#[test]
fn native_queue_callback_bursts_are_linear() {
    for count in [1_000u32, 10_000] {
        let log: DeliveryLog = Rc::default();
        let mut host = ComponentHost::mount(
            QueueCountingAdapter::new(),
            [component::<NativeQueueBurst>("burst", Rc::clone(&log))],
        );
        let object = host.reference(&Key::from("burst")).unwrap().get().unwrap();
        let event = host.runtime().graph().events(object).unwrap()[0].clone();
        for pointer_id in 0..count {
            host.test_adapter_mut()
                .inner
                .queue_event(EventDispatch::new(
                    object,
                    event.id,
                    event.value.clone(),
                    EventPayload::PointerEventInfo(PointerEventInfo {
                        x: f64::from(pointer_id),
                        y: 1.0,
                        window_x: f64::from(pointer_id),
                        window_y: 1.0,
                        pointer_id,
                        modifiers: InputModifiers::NONE,
                        capture_succeeded: None,
                        is_captured: false,
                        is_left_button_pressed: false,
                        is_right_button_pressed: false,
                        is_middle_button_pressed: false,
                    }),
                ));
        }
        host.runtime().adapter().visits.set(0);
        let mut dispatched = 0;
        for _ in 0..count.div_ceil(64) {
            let report = host.drain(64);
            dispatched += report.dispatched;
            assert_eq!(report.dropped, 0);
        }
        assert_eq!(dispatched, count as usize);
        let expected: Vec<_> = (0..count)
            .flat_map(|value| [("callback", value), ("update", value)])
            .collect();
        assert_eq!(*log.borrow(), expected);
        assert_eq!(
            host.runtime().adapter().visits.get(),
            2 * count as usize + 2
        );
    }
}

#[test]
fn native_queue_observation_bursts_are_linear() {
    for count in [1_000, 10_000] {
        let mut runtime = Runtime::new(QueueCountingAdapter::new());
        runtime.update(TextBox::new().text("Before"));
        let object = runtime.graph().root().unwrap();
        for value in 0..count {
            runtime
                .adapter_mut()
                .inner
                .observe(Observation::SetProperty {
                    object,
                    property: Property {
                        id: PropertyId::Text,
                        value: PropertyValue::String(Rc::from(value.to_string())),
                    },
                });
        }
        runtime.adapter().visits.set(0);
        runtime.prepare_update();
        assert_eq!(runtime.adapter().visits.get(), 2 * count + 1);
        assert_eq!(
            runtime.graph().properties(object).unwrap()[0].value,
            PropertyValue::String(Rc::from((count - 1).to_string()))
        );
    }
}

#[test]
fn native_queue_realizations_are_linear() {
    for count in [1_000, 10_000] {
        let mut runtime = Runtime::new(QueueCountingAdapter::new());
        runtime.update(ItemsRepeater::new().virtual_source(VirtualSource::new(
            0,
            count,
            Key::from,
            |index| TextBlock::new().text(index.to_string()),
        )));
        let collection = runtime.graph().root().unwrap();
        for index in 0..count {
            runtime
                .adapter_mut()
                .inner
                .queue_realization(RealizationRequest::Realize {
                    collection,
                    container: RealizedContainer(index.try_into().unwrap()),
                    index,
                    source_revision: 0,
                });
        }
        runtime.adapter().visits.set(0);
        // Isolate queue scaling from the cost of constructing retained rows.
        for expected in 0..count {
            let Some(NativeWork::Virtual(VirtualWork::Realize { lease, index, .. })) =
                runtime.next_native_work()
            else {
                panic!("expected realization");
            };
            assert_eq!(index, expected);
            assert_eq!(lease.collection, collection);
            assert_eq!(
                lease.container,
                RealizedContainer(expected.try_into().unwrap())
            );
        }
        assert!(runtime.next_native_work().is_none());
        assert_eq!(runtime.adapter().visits.get(), count + 1);
    }
}

struct VirtualQueue;

impl Component for VirtualQueue {
    type Input = usize;
    type Message = ();

    fn create(_: &usize, _: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, count: &usize, _: &mut ViewContext<Self>) -> View {
        ItemsRepeater::new()
            .virtual_source(VirtualSource::new(0, *count, Key::from, |index| {
                TextBlock::new().text(index.to_string())
            }))
            .into()
    }
}

#[test]
fn native_queue_realization_and_recycling_are_linear() {
    for count in [64usize, 256] {
        let mut host = ComponentHost::mount(
            QueueCountingAdapter::new(),
            [component::<VirtualQueue>("rows", count)],
        );
        assert_eq!(host.runtime().graph().full_reference_scan_count(), 0);
        let collection = host.reference(&Key::from("rows")).unwrap().get().unwrap();
        for recycle in [false, true] {
            for index in 0..count {
                let container = RealizedContainer(index.try_into().unwrap());
                let request = if recycle {
                    RealizationRequest::Recycle {
                        collection,
                        container,
                        source_revision: 0,
                    }
                } else {
                    RealizationRequest::Realize {
                        collection,
                        container,
                        index,
                        source_revision: 0,
                    }
                };
                host.test_adapter_mut().inner.queue_realization(request);
            }
            host.runtime().adapter().visits.set(0);
            assert_eq!(host.drain(count + 1).dropped, 0);
            assert_eq!(host.runtime().adapter().visits.get(), 2 * count + 2);
            assert_eq!(host.runtime().graph().full_reference_scan_count(), 0);
            let retained = if recycle { 0 } else { count };
            assert_eq!(host.test_state().virtual_rows, retained);
            assert_eq!(
                host.runtime().adapter().inner.realized_count(collection),
                retained
            );
        }
    }
}
