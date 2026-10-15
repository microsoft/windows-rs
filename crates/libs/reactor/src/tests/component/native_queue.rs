use super::*;

struct PreviewCountingAdapter {
    inner: RecordingAdapter,
    visits: Cell<usize>,
}

impl PreviewCountingAdapter {
    fn new() -> Self {
        let mut inner = RecordingAdapter::new();
        // Isolate preview work from the recording backend's validation clones.
        inner.validate_batches(false);
        Self {
            inner,
            visits: Cell::new(0),
        }
    }
}

impl Adapter for PreviewCountingAdapter {
    type Error = AdapterError;

    fn preview_native_events(&self, events: &mut Vec<NativeEvent>) {
        let before = events.len();
        self.inner.preview_native_events(events);
        self.visits.set(self.visits.get() + events.len() - before);
    }

    fn pop_native_event(&mut self) -> Option<NativeEvent> {
        self.inner.pop_native_event()
    }

    fn validate(&self, mutations: &[Mutation]) -> Result<(), Self::Error> {
        self.inner.validate(mutations)
    }

    fn apply(&mut self, mutations: &[Mutation]) -> Result<(), Self::Error> {
        self.inner.apply(mutations)
    }

    fn focus(&mut self, object: ObjectId) -> Result<bool, Self::Error> {
        self.inner.focus(object)
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
            PreviewCountingAdapter::new(),
            [component::<NativeQueueBurst>("burst", Rc::clone(&log))],
        )
        .unwrap();
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
            let report = host.drain(64).unwrap();
            dispatched += report.dispatched;
            assert_eq!(report.dropped, 0);
        }
        assert_eq!(dispatched, count as usize);
        let expected: Vec<_> = (0..count)
            .flat_map(|value| [("callback", value), ("update", value)])
            .collect();
        assert_eq!(*log.borrow(), expected);
        assert_eq!(host.runtime().adapter().visits.get(), 2 * count as usize);
    }
}

#[test]
fn native_queue_observation_bursts_are_linear() {
    for count in [1_000, 10_000] {
        let mut runtime = Runtime::new(PreviewCountingAdapter::new());
        runtime.update(TextBox::new().text("Before")).unwrap();
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
        runtime.prepare_update().unwrap();
        assert_eq!(runtime.adapter().visits.get(), count);
        assert_eq!(
            runtime.graph().properties(object).unwrap()[0].value,
            PropertyValue::String(Rc::from((count - 1).to_string()))
        );
    }
}
