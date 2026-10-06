use super::*;

mod context;
mod lifecycle;
mod messaging;
mod reconcile;
mod virtualization;
mod window;

type PendingTimer = (Arc<TestTimerRegistration>, Box<dyn FnOnce() + Send>);

#[derive(Default)]
struct TestServices {
    background: Mutex<VecDeque<Box<dyn FnOnce() + Send>>>,
    timers: Mutex<VecDeque<PendingTimer>>,
}

impl TestServices {
    fn run_background(&self) {
        self.background.lock().unwrap().pop_front().unwrap()();
    }

    fn fire_timer(&self) {
        let (timer, callback) = self.timers.lock().unwrap().pop_front().unwrap();
        if !timer.cancelled.load(Ordering::Acquire) {
            callback();
        }
    }
}

impl ComponentServices for TestServices {
    fn spawn_background(&self, work: Box<dyn FnOnce() + Send>) {
        self.background.lock().unwrap().push_back(work);
    }

    fn set_timeout(
        &self,
        _delay: Duration,
        callback: Box<dyn FnOnce() + Send>,
    ) -> Arc<dyn ComponentTimerRegistration> {
        let timer = Arc::new(TestTimerRegistration::default());
        self.timers
            .lock()
            .unwrap()
            .push_back((Arc::clone(&timer), callback));
        timer
    }
}

#[derive(Default)]
struct TestTimerRegistration {
    cancelled: AtomicBool,
}

impl ComponentTimerRegistration for TestTimerRegistration {
    fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
    }
}

#[derive(Clone)]
struct CounterInput {
    cleanup: Arc<AtomicUsize>,
    value: usize,
}

impl PartialEq for CounterInput {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && Arc::ptr_eq(&self.cleanup, &other.cleanup)
    }
}

struct Counter {
    value: usize,
}

impl Component for Counter {
    type Input = CounterInput;
    type Message = usize;

    fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self { value: input.value }
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        self.value = input.value;
    }

    fn update(&mut self, message: usize, _context: &ComponentContext<Self>) {
        self.value += message;
    }

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let cleanup = Arc::clone(&input.cleanup);
        context.use_effect_guard("value", self.value, move || Cleanup(cleanup));
        TextBlock::new().text(self.value.to_string()).into()
    }
}

struct Label;

impl Component for Label {
    type Input = Rc<str>;
    type Message = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        TextBlock::new().text(input.clone()).into()
    }
}

struct Cleanup(Arc<AtomicUsize>);

impl Drop for Cleanup {
    fn drop(&mut self) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

#[derive(Clone)]
struct ContextInput {
    context: Rc<Context<usize>>,
    renders: Arc<AtomicUsize>,
    subscribe: bool,
}

impl PartialEq for ContextInput {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.context, &other.context)
            && Arc::ptr_eq(&self.renders, &other.renders)
            && self.subscribe == other.subscribe
    }
}

struct ContextReader;

impl Component for ContextReader {
    type Input = ContextInput;
    type Message = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        input.renders.fetch_add(1, Ordering::Relaxed);
        let value = if input.subscribe {
            context.use_context(&input.context)
        } else {
            0
        };
        TextBlock::new().text(value.to_string()).into()
    }
}

#[derive(Clone)]
struct StatefulInput {
    changes: Rc<Cell<usize>>,
    value: usize,
}

impl PartialEq for StatefulInput {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && Rc::ptr_eq(&self.changes, &other.changes)
    }
}

struct StatefulInputComponent;

impl Component for StatefulInputComponent {
    type Input = StatefulInput;
    type Message = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        input.changes.set(input.changes.get() + 1);
    }

    fn view(&self, input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        TextBlock::new().text(input.value.to_string()).into()
    }
}

struct ControlledApplyAdapter {
    fail_after: Rc<Cell<Option<usize>>>,
    inner: RecordingAdapter,
    successful: Rc<Cell<usize>>,
}

impl Adapter for ControlledApplyAdapter {
    type Error = ();

    fn preview_native_events(&self, events: &mut Vec<NativeEvent>) {
        self.inner.preview_native_events(events);
    }

    fn pop_native_event(&mut self) -> Option<NativeEvent> {
        self.inner.pop_native_event()
    }

    fn validate(&self, mutations: &[Mutation]) -> Result<(), Self::Error> {
        self.inner.validate(mutations).map_err(|_| ())
    }

    fn apply(&mut self, mutations: &[Mutation]) -> Result<(), Self::Error> {
        if let Some(remaining) = self.fail_after.get() {
            if remaining == 0 {
                return Err(());
            }
            self.fail_after.set(Some(remaining - 1));
        }
        self.inner.apply(mutations).map_err(|_| ())?;
        self.successful.set(self.successful.get() + 1);
        Ok(())
    }

    fn focus(&mut self, object: ObjectId) -> Result<bool, Self::Error> {
        self.inner.focus(object).map_err(|_| ())
    }
}
