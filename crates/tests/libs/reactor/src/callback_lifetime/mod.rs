use std::{
    cell::Cell,
    rc::Rc,
    sync::{Arc, Mutex, OnceLock},
    thread::{self, ThreadId},
};
use windows_core::*;
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
    retained_probe: Mutex<Option<ComObject<LocalProbe>>>,
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

interface_decl! {
    unsafe trait ILocalProbe(ILocalProbe_Vtbl, ILocalProbe_Impl): IUnknown
        = 0x96264ad8_741d_4dc3_a5d3_8cd9877c080a {
        unsafe fn invoke(&self) -> HRESULT;
    }
}

struct LocalProbe(local::Local<Capture>);

implement_decl! {
    impl LocalProbe as LocalProbe_Impl: [ILocalProbe]
}

impl ILocalProbe_Impl for LocalProbe_Impl {
    unsafe fn invoke(&self) -> HRESULT {
        self.0.try_with(Capture::invoke).into()
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
        let probe = ComObject::new(LocalProbe(local::Local::new(Capture {
            id: 3,
            value: Rc::new(Cell::new(0)),
            completion: Some(context.completion()),
        })));
        unsafe { probe.to_interface::<ILocalProbe>().invoke().ok().unwrap() };
        thread::spawn(move || {
            assert_eq!(
                unsafe { probe.to_interface::<ILocalProbe>().invoke() },
                HRESULT(0x8001010Eu32 as i32),
            );
            drop(probe);
        })
        .join()
        .unwrap();
        assert!(!state.drops.lock().unwrap().iter().any(|drop| drop.0 == 3));
        *state.retained_probe.lock().unwrap() =
            Some(ComObject::new(LocalProbe(local::Local::new(Capture {
                id: 4,
                value: Rc::new(Cell::new(0)),
                completion: None,
            }))));
        Self(0)
    }

    fn update(&mut self, _: (), context: &ComponentContext<Self>) {
        self.0 += 1;
        if self.0 == 3 {
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

pub(super) fn run() -> Result<()> {
    let state = Arc::new(State {
        owner: thread::current().id(),
        drops: Mutex::new(Vec::new()),
        retained: Mutex::new(None),
        dispatcher: Mutex::new(None),
        retained_probe: Mutex::new(None),
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
            (2, 0, state.owner),
            (3, 1, state.owner),
            (4, 0, state.owner),
        ]
    );
    let dispatcher = state.dispatcher.lock().unwrap().take().unwrap();
    assert!(!dispatcher.TryEnqueueWithPriority(
        bindings::DispatcherQueuePriority::Normal,
        &bindings::DispatcherQueueHandler::new(|| panic!("callback after shutdown")),
    )?);
    let event = state.retained.lock().unwrap().take().unwrap();
    thread::spawn(move || drop(event)).join().unwrap();
    let probe = state.retained_probe.lock().unwrap().take().unwrap();
    assert_eq!(
        unsafe { probe.to_interface::<ILocalProbe>().invoke() },
        HRESULT(0x80000013u32 as i32),
    );
    thread::spawn(move || drop(probe)).join().unwrap();
    assert_eq!(state.drops.lock().unwrap().len(), 5);
    Ok(())
}
