use std::{
    cell::Cell,
    rc::Rc,
    sync::{Arc, Mutex, OnceLock},
    thread::{self, ThreadId},
};
use windows_core::Event;
use windows_reactor::*;

#[allow(dead_code, nonstandard_style, clippy::all)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "\\callback_bindings.rs"));
}

// Exercise the private adapter with a real dispatcher without exposing test APIs in the library.
#[allow(dead_code)]
#[path = "../../../../../libs/reactor/src/native/local.rs"]
mod local;

struct State {
    owner: ThreadId,
    drops: Mutex<Vec<(usize, usize, ThreadId)>>,
    retained: Mutex<Option<Event<bindings::DispatcherQueueHandler>>>,
    dispatcher: Mutex<Option<bindings::DispatcherQueue>>,
}

static STATE: OnceLock<Arc<State>> = OnceLock::new();

struct Capture {
    id: usize,
    value: Rc<Cell<usize>>,
    completion: Option<ComponentCompletion<()>>,
}

impl Capture {
    fn invoke(&self) {
        assert_eq!(thread::current().id(), STATE.get().unwrap().owner);
        self.value.set(self.value.get() + 1);
    }
}

impl Drop for Capture {
    fn drop(&mut self) {
        let state = STATE.get().unwrap();
        let owner = thread::current().id();
        assert_eq!(owner, state.owner);
        state
            .drops
            .lock()
            .unwrap()
            .push((self.id, self.value.get(), owner));
        if let Some(completion) = self.completion.take() {
            assert!(completion.complete(()));
        }
    }
}

struct Fixture(usize);

impl Component for Fixture {
    type Input = ();
    type Message = ();

    fn create(_: &(), context: &ComponentContext<Self>) -> Self {
        let state = STATE.get().unwrap();
        let dispatcher = bindings::DispatcherQueue::GetForCurrentThread().unwrap();
        *state.dispatcher.lock().unwrap() = Some(dispatcher.clone());
        for id in 0..3 {
            let capture = Capture {
                id,
                value: Rc::new(Cell::new(0)),
                completion: (id != 2).then(|| context.completion()),
            };
            let handler =
                bindings::DispatcherQueueHandler::new(local::task(move || capture.invoke()));
            if id == 1 {
                assert!(
                    dispatcher
                        .TryEnqueueWithPriority(bindings::DispatcherQueuePriority::Normal, &handler)
                        .unwrap()
                );
            } else {
                let event = Event::new();
                event.add(&handler).unwrap();
                drop(handler);
                if id == 0 {
                    thread::spawn(move || drop(event)).join().unwrap();
                    assert!(state.drops.lock().unwrap().is_empty());
                } else {
                    *state.retained.lock().unwrap() = Some(event);
                }
            }
        }
        Self(0)
    }

    fn update(&mut self, _: (), context: &ComponentContext<Self>) {
        self.0 += 1;
        if self.0 == 2 {
            assert!(context.close_window());
        }
    }

    fn view(&self, _: &(), _: &mut ViewContext<Self>) -> View {
        TextBlock::new().text("Native callback lifetime").into()
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        local::clear();
    }
}

pub(super) fn run() -> windows_core::Result<()> {
    let state = Arc::new(State {
        owner: thread::current().id(),
        drops: Mutex::new(Vec::new()),
        retained: Mutex::new(None),
        dispatcher: Mutex::new(None),
    });
    assert!(STATE.set(state.clone()).is_ok());
    App::run_component::<Fixture>(())?;
    let mut drops = state.drops.lock().unwrap().clone();
    drops.sort_by_key(|drop| drop.0);
    assert_eq!(
        drops,
        [
            (0, 0, state.owner),
            (1, 1, state.owner),
            (2, 0, state.owner)
        ]
    );
    let dispatcher = state.dispatcher.lock().unwrap().take().unwrap();
    assert!(!dispatcher.TryEnqueueWithPriority(
        bindings::DispatcherQueuePriority::Normal,
        &bindings::DispatcherQueueHandler::new(|| panic!("callback after shutdown")),
    )?);
    let event = state.retained.lock().unwrap().take().unwrap();
    thread::spawn(move || drop(event)).join().unwrap();
    assert_eq!(state.drops.lock().unwrap().len(), 3);
    Ok(())
}
