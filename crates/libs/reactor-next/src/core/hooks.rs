use std::any::Any;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

use super::*;

trait Slot {
    fn as_any(&self) -> &dyn Any;
    fn cleanup(&self) {}
}

struct StateSlot<T>(Rc<RefCell<T>>);

impl<T: 'static> Slot for StateSlot<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct EffectSlot<D> {
    dependency: RefCell<D>,
    cleanup: RefCell<Option<Box<dyn FnOnce()>>>,
    initialized: Cell<bool>,
}

impl<D: 'static> Slot for Rc<EffectSlot<D>> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn cleanup(&self) {
        if let Some(cleanup) = self.cleanup.borrow_mut().take() {
            cleanup();
        }
    }
}

#[derive(Clone)]
pub struct State<T> {
    value: Rc<RefCell<T>>,
    dirty: Rc<Cell<bool>>,
}

impl<T: Clone> State<T> {
    pub fn get(&self) -> T {
        self.value.borrow().clone()
    }
}

impl<T> State<T> {
    pub fn set(&self, value: T) {
        *self.value.borrow_mut() = value;
        self.dirty.set(true);
    }

    pub fn update(&self, update: impl FnOnce(&mut T)) {
        update(&mut self.value.borrow_mut());
        self.dirty.set(true);
    }
}

pub struct Hooks {
    slots: Vec<Box<dyn Slot>>,
    cursor: usize,
    dirty: Rc<Cell<bool>>,
    pending_effects: Vec<Box<dyn FnOnce()>>,
}

impl Hooks {
    pub(crate) fn new() -> Self {
        Self {
            slots: Vec::new(),
            cursor: 0,
            dirty: Rc::new(Cell::new(true)),
            pending_effects: Vec::new(),
        }
    }

    pub fn use_state<T: 'static>(&mut self, initialize: impl FnOnce() -> T) -> State<T> {
        let index = self.next_slot();
        let value = if let Some(slot) = self.slots.get(index) {
            slot.as_any()
                .downcast_ref::<StateSlot<T>>()
                .unwrap()
                .0
                .clone()
        } else {
            let value = Rc::new(RefCell::new(initialize()));
            self.slots.push(Box::new(StateSlot(Rc::clone(&value))));
            value
        };
        State {
            value,
            dirty: Rc::clone(&self.dirty),
        }
    }

    pub fn use_effect<D: Clone + PartialEq + 'static>(
        &mut self,
        dependency: D,
        setup: impl FnOnce() -> Option<Box<dyn FnOnce()>> + 'static,
    ) {
        let index = self.next_slot();
        let slot = if let Some(slot) = self.slots.get(index) {
            slot.as_any()
                .downcast_ref::<Rc<EffectSlot<D>>>()
                .unwrap()
                .clone()
        } else {
            let slot = Rc::new(EffectSlot {
                dependency: RefCell::new(dependency.clone()),
                cleanup: RefCell::new(None),
                initialized: Cell::new(false),
            });
            self.slots.push(Box::new(Rc::clone(&slot)));
            slot
        };

        let changed = *slot.dependency.borrow() != dependency || !slot.initialized.get();
        if changed {
            self.pending_effects.push(Box::new(move || {
                if let Some(cleanup) = slot.cleanup.borrow_mut().take() {
                    cleanup();
                }
                *slot.dependency.borrow_mut() = dependency;
                *slot.cleanup.borrow_mut() = setup();
                slot.initialized.set(true);
            }));
        }
    }

    pub(crate) fn begin(&mut self) {
        self.cursor = 0;
        self.pending_effects.clear();
        self.dirty.set(false);
    }

    pub(crate) fn finish(&mut self) -> Vec<Box<dyn FnOnce()>> {
        assert_eq!(self.cursor, self.slots.len(), "hook count changed");
        std::mem::take(&mut self.pending_effects)
    }

    pub(crate) fn dirty(&self) -> bool {
        self.dirty.get()
    }

    pub(crate) fn retry(&self) {
        self.dirty.set(true);
    }

    fn next_slot(&mut self) -> usize {
        let index = self.cursor;
        self.cursor += 1;
        index
    }
}

impl Drop for Hooks {
    fn drop(&mut self) {
        for slot in self.slots.iter().rev() {
            slot.cleanup();
        }
    }
}

pub struct RenderLoop<R, F> {
    pump: Pump<R>,
    hooks: Hooks,
    render: F,
    mounted: bool,
}

impl<R, F> RenderLoop<R, F>
where
    R: NativeRuntime,
    F: FnMut(&mut Hooks) -> Element,
{
    pub fn new(runtime: R, render: F) -> Self {
        Self {
            pump: Pump::new(runtime),
            hooks: Hooks::new(),
            render,
            mounted: false,
        }
    }

    pub fn run(&mut self) -> Result<(), PumpError> {
        const MAX_RENDERS: usize = 100;

        for _ in 0..MAX_RENDERS {
            if self.mounted && !self.hooks.dirty() {
                return Ok(());
            }

            self.hooks.begin();
            let element = (self.render)(&mut self.hooks);
            let effects = self.hooks.finish();
            let result = if self.mounted {
                self.pump.update(element)
            } else {
                let result = self.pump.mount(element);
                self.mounted = self.pump.root().is_some();
                result
            };
            if let Err(error) = result {
                self.hooks.retry();
                return Err(error);
            }
            for effect in effects {
                effect();
            }
        }
        Err(PumpError::RenderBudgetExceeded)
    }

    pub fn dispatch_events(&mut self) -> Result<usize, PumpError> {
        let dispatched = self.pump.dispatch_events();
        self.run()?;
        Ok(dispatched)
    }

    pub fn pump(&self) -> &Pump<R> {
        &self.pump
    }

    pub fn pump_mut(&mut self) -> &mut Pump<R> {
        &mut self.pump
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::native::*;

    #[test]
    fn state_callback_schedules_whole_root_render() {
        let state = Rc::new(RefCell::new(None));
        let state_capture = Rc::clone(&state);
        let mut app = RenderLoop::new(RecordingRuntime::default(), move |hooks| {
            let count = hooks.use_state(|| 0_u32);
            *state_capture.borrow_mut() = Some(count.clone());
            let callback = count.clone();
            StackPanel::new()
                .child("value", TextBlock::new().text(count.get().to_string()))
                .child(
                    "increment",
                    Button::new()
                        .on_click(move || callback.update(|value| *value += 1))
                        .content(TextBlock::new().text("+")),
                )
                .into()
        });
        app.run().unwrap();
        let root = app.pump().root().unwrap();
        let button = app.pump().runtime().node(root).unwrap().children()[1];
        let revision = app
            .pump()
            .event_revision(button, EventId::ButtonClick)
            .unwrap();

        app.pump_mut().queue_event(QueuedEvent {
            node: button,
            event: EventId::ButtonClick,
            revision,
            payload: EventPayload::Unit,
        });
        assert_eq!(app.dispatch_events().unwrap(), 1);

        assert_eq!(state.borrow().as_ref().unwrap().get(), 1);
        let text = app.pump().runtime().node(root).unwrap().children()[0];
        assert_eq!(
            app.pump()
                .runtime()
                .node(text)
                .unwrap()
                .property(PropertyId::TextBlockText),
            Some(&PropertyValue::Str("1".into()))
        );
    }

    #[test]
    fn effects_cleanup_after_commit_and_on_drop() {
        let state = Rc::new(RefCell::new(None));
        let state_capture = Rc::clone(&state);
        let log = Rc::new(RefCell::new(Vec::new()));
        let log_capture = Rc::clone(&log);
        let mut app = RenderLoop::new(RecordingRuntime::default(), move |hooks| {
            let value = hooks.use_state(|| 0_u32);
            *state_capture.borrow_mut() = Some(value.clone());
            let dependency = value.get();
            let setup_log = Rc::clone(&log_capture);
            hooks.use_effect(dependency, move || {
                setup_log.borrow_mut().push(format!("setup {dependency}"));
                let cleanup_log = Rc::clone(&setup_log);
                Some(Box::new(move || {
                    cleanup_log
                        .borrow_mut()
                        .push(format!("cleanup {dependency}"));
                }))
            });
            TextBlock::new().text(dependency.to_string()).into()
        });

        app.run().unwrap();
        state.borrow().as_ref().unwrap().set(1);
        app.run().unwrap();
        drop(app);

        assert_eq!(
            &*log.borrow(),
            &["setup 0", "cleanup 0", "setup 1", "cleanup 1"]
        );
    }

    #[test]
    fn failed_property_keeps_render_dirty_and_defers_effects() {
        let state = Rc::new(RefCell::new(None));
        let state_capture = Rc::clone(&state);
        let effects = Rc::new(Cell::new(0));
        let effects_capture = Rc::clone(&effects);
        let mut app = RenderLoop::new(RecordingRuntime::default(), move |hooks| {
            let value = hooks.use_state(|| 0_u32);
            *state_capture.borrow_mut() = Some(value.clone());
            let dependency = value.get();
            let effects = Rc::clone(&effects_capture);
            hooks.use_effect(dependency, move || {
                effects.set(effects.get() + 1);
                None
            });
            TextBlock::new().text(dependency.to_string()).into()
        });
        app.run().unwrap();
        assert_eq!(effects.get(), 1);
        let version = app.pump().version();

        state.borrow().as_ref().unwrap().set(1);
        app.pump_mut().runtime_mut().fail_at(0);
        assert!(matches!(app.run(), Err(PumpError::PropertyApplyFailed(_))));

        assert!(app.hooks.dirty());
        assert!(app.pump().retry_pending());
        assert_eq!(app.pump().version(), version);
        assert_eq!(effects.get(), 1);

        app.run().unwrap();
        assert!(!app.hooks.dirty());
        assert!(!app.pump().retry_pending());
        assert_eq!(app.pump().version(), version + 1);
        assert_eq!(effects.get(), 2);
    }

    #[test]
    fn render_budget_stops_self_scheduling_loop() {
        let mut app = RenderLoop::new(RecordingRuntime::default(), |hooks| {
            let state = hooks.use_state(|| 0_u32);
            state.update(|value| *value += 1);
            TextBlock::new().text(state.get().to_string()).into()
        });

        assert_eq!(app.run(), Err(PumpError::RenderBudgetExceeded));
    }
}
