use super::*;
use std::any::Any;
use std::cell::Cell;
use std::collections::{HashSet, VecDeque};
use std::rc::Rc;
use std::sync::{Arc, Mutex};

const MESSAGE_CAPACITY: usize = 4_096;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ComponentId {
    index: u32,
    generation: u32,
}

#[derive(Clone, Default)]
pub struct ElementRef(Rc<Cell<Option<ObjectId>>>);

impl ElementRef {
    pub fn get(&self) -> Option<ObjectId> {
        self.0.get()
    }

    fn set(&self, object: Option<ObjectId>) {
        self.0.set(object);
    }
}

struct Message<M> {
    component: ComponentId,
    value: M,
}

pub struct ComponentSender<M> {
    component: ComponentId,
    queue: Arc<Mutex<VecDeque<Message<M>>>>,
}

impl<M> Clone for ComponentSender<M> {
    fn clone(&self) -> Self {
        Self {
            component: self.component,
            queue: Arc::clone(&self.queue),
        }
    }
}

impl<M> ComponentSender<M> {
    #[must_use]
    pub fn send(&self, value: M) -> bool {
        let mut queue = self.queue.lock().unwrap();
        if queue.len() >= MESSAGE_CAPACITY {
            return false;
        }
        queue.push_back(Message {
            component: self.component,
            value,
        });
        true
    }

    pub fn completion(&self) -> ComponentCompletion<M> {
        ComponentCompletion(self.clone())
    }
}

pub struct ComponentCompletion<M>(ComponentSender<M>);

impl<M> Clone for ComponentCompletion<M> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<M> ComponentCompletion<M> {
    #[must_use]
    pub fn complete(self, value: M) -> bool {
        self.0.send(value)
    }
}

pub struct ComponentContext<M> {
    reference: ElementRef,
    sender: ComponentSender<M>,
}

impl<M> ComponentContext<M> {
    pub fn sender(&self) -> ComponentSender<M> {
        self.sender.clone()
    }

    pub fn completion(&self) -> ComponentCompletion<M> {
        self.sender.completion()
    }

    pub fn root(&self) -> ElementRef {
        self.reference.clone()
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct EffectKey(Key);

impl<T: Into<Key>> From<T> for EffectKey {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

type EffectCleanup = Box<dyn FnOnce()>;
type EffectSetup = Box<dyn FnOnce() -> Option<EffectCleanup>>;

struct EffectRegistration {
    dependency: Box<dyn Any>,
    equals: fn(&dyn Any, &dyn Any) -> bool,
    key: EffectKey,
    setup: EffectSetup,
}

struct EffectSlot {
    cleanup: Option<EffectCleanup>,
    dependency: Box<dyn Any>,
    equals: fn(&dyn Any, &dyn Any) -> bool,
    key: EffectKey,
}

#[derive(Default)]
struct EffectDraft {
    duplicate: Option<EffectKey>,
    registrations: Vec<EffectRegistration>,
}

impl EffectDraft {
    fn register<D>(
        &mut self,
        key: EffectKey,
        dependency: D,
        setup: impl FnOnce() -> Option<EffectCleanup> + 'static,
    ) where
        D: PartialEq + 'static,
    {
        if self
            .registrations
            .iter()
            .any(|registration| registration.key == key)
        {
            self.duplicate.get_or_insert(key);
            return;
        }
        self.registrations.push(EffectRegistration {
            dependency: Box::new(dependency),
            equals: |left, right| {
                left.downcast_ref::<D>()
                    .zip(right.downcast_ref::<D>())
                    .is_some_and(|(left, right)| left == right)
            },
            key,
            setup: Box::new(setup),
        });
    }

    fn commit(self, slots: &mut Vec<EffectSlot>) {
        let mut previous = std::mem::take(slots);
        for registration in self.registrations {
            let retained = previous
                .iter()
                .position(|slot| slot.key == registration.key)
                .and_then(|index| {
                    let same = {
                        let slot = &previous[index];
                        (slot.equals)(slot.dependency.as_ref(), registration.dependency.as_ref())
                    };
                    same.then(|| previous.remove(index))
                });
            if let Some(slot) = retained {
                slots.push(slot);
            } else {
                if let Some(index) = previous
                    .iter()
                    .position(|slot| slot.key == registration.key)
                    && let Some(cleanup) = previous.remove(index).cleanup
                {
                    cleanup();
                }
                slots.push(EffectSlot {
                    cleanup: (registration.setup)(),
                    dependency: registration.dependency,
                    equals: registration.equals,
                    key: registration.key,
                });
            }
        }
        for slot in previous.iter_mut().rev() {
            if let Some(cleanup) = slot.cleanup.take() {
                cleanup();
            }
        }
    }
}

pub struct ComponentViewContext<M> {
    context: ComponentContext<M>,
    effects: EffectDraft,
}

impl<M> ComponentViewContext<M> {
    pub fn sender(&self) -> ComponentSender<M> {
        self.context.sender()
    }

    pub fn completion(&self) -> ComponentCompletion<M> {
        self.context.completion()
    }

    pub fn root(&self) -> ElementRef {
        self.context.root()
    }

    pub fn use_effect<D>(
        &mut self,
        key: impl Into<EffectKey>,
        dependency: D,
        setup: impl FnOnce() -> Option<EffectCleanup> + 'static,
    ) where
        D: PartialEq + 'static,
    {
        self.effects.register(key.into(), dependency, setup);
    }

    pub fn use_effect_guard<D, G>(
        &mut self,
        key: impl Into<EffectKey>,
        dependency: D,
        setup: impl FnOnce() -> G + 'static,
    ) where
        D: PartialEq + 'static,
        G: 'static,
    {
        self.use_effect(key, dependency, move || {
            let guard = setup();
            Some(Box::new(move || drop(guard)))
        });
    }
}

pub trait Component: 'static {
    type Message: Send + 'static;

    fn update(&mut self, message: Self::Message, context: &ComponentContext<Self::Message>);
    fn view(&self, context: &mut ComponentViewContext<Self::Message>) -> Visual;
}

struct Scope<C: Component> {
    component: C,
    effects: Vec<EffectSlot>,
    key: Key,
    reference: ElementRef,
    root: Option<ObjectId>,
    sender: ComponentSender<C::Message>,
}

struct ScopeSlot<C: Component> {
    generation: u32,
    scope: Option<Scope<C>>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ComponentDrain {
    pub dispatched: usize,
    pub dropped: usize,
    pub mutations: usize,
}

#[derive(Debug)]
pub enum ComponentError<E> {
    DuplicateEffect(EffectKey),
    DuplicateKey(Key),
    MissingComponent(Key),
    Runtime(UpdateError<E>),
}

impl<E> From<UpdateError<E>> for ComponentError<E> {
    fn from(value: UpdateError<E>) -> Self {
        Self::Runtime(value)
    }
}

pub struct ComponentSet<C: Component, A: Adapter> {
    order: Vec<ComponentId>,
    queue: Arc<Mutex<VecDeque<Message<C::Message>>>>,
    runtime: Runtime<A>,
    scopes: Vec<ScopeSlot<C>>,
}

impl<C: Component, A: Adapter> Drop for ComponentSet<C, A> {
    fn drop(&mut self) {
        for slot in &mut self.scopes {
            let Some(scope) = slot.scope.as_mut() else {
                continue;
            };
            scope.reference.set(None);
            for effect in scope.effects.iter_mut().rev() {
                if let Some(cleanup) = effect.cleanup.take() {
                    cleanup();
                }
            }
        }
    }
}

impl<C: Component, A: Adapter> ComponentSet<C, A> {
    pub fn mount(
        adapter: A,
        components: impl IntoIterator<Item = (Key, C)>,
    ) -> Result<Self, ComponentError<A::Error>> {
        let queue = Arc::new(Mutex::new(VecDeque::new()));
        let mut set = Self {
            order: Vec::new(),
            queue,
            runtime: Runtime::new(adapter),
            scopes: Vec::new(),
        };
        let mut pending_effects = Vec::new();
        let mut declarations = Vec::new();
        let mut keys = HashSet::new();
        for (key, component) in components {
            if !keys.insert(key.clone()) {
                return Err(ComponentError::DuplicateKey(key));
            }
            let id = ComponentId {
                index: u32::try_from(set.scopes.len()).unwrap(),
                generation: 0,
            };
            let reference = ElementRef::default();
            let sender = ComponentSender {
                component: id,
                queue: Arc::clone(&set.queue),
            };
            let (view, effects) = Self::render(&component, &sender, &reference)?;
            declarations.push(keyed(key.clone(), view));
            set.scopes.push(ScopeSlot {
                generation: 0,
                scope: Some(Scope {
                    component,
                    effects: Vec::new(),
                    key,
                    reference,
                    root: None,
                    sender,
                }),
            });
            set.order.push(id);
            pending_effects.push((id, effects));
        }
        let root = Grid::new().children(declarations);
        set.runtime.update(root)?;
        set.refresh_roots();
        for (id, effects) in pending_effects {
            effects.commit(&mut set.scope_mut(id).unwrap().effects);
        }
        Ok(set)
    }

    pub fn runtime(&self) -> &Runtime<A> {
        &self.runtime
    }

    pub fn runtime_mut(&mut self) -> &mut Runtime<A> {
        &mut self.runtime
    }

    pub fn sender(&self, key: &Key) -> Option<ComponentSender<C::Message>> {
        self.find(key)
            .map(|id| self.scope(id).unwrap().sender.clone())
    }

    pub fn reference(&self, key: &Key) -> Option<ElementRef> {
        self.find(key)
            .map(|id| self.scope(id).unwrap().reference.clone())
    }

    pub fn drain(&mut self, limit: usize) -> Result<ComponentDrain, ComponentError<A::Error>> {
        let mut report = ComponentDrain::default();
        let mut events = Vec::new();
        if let Err(error) = self.runtime.drain_events(&mut events) {
            self.invalidate();
            return Err(ComponentError::Runtime(error));
        }
        for event in events {
            event.invoke();
        }
        while report.dispatched < limit {
            let Some(message) = self.queue.lock().unwrap().pop_front() else {
                break;
            };
            let Some(scope) = self.scope_mut(message.component) else {
                report.dropped += 1;
                continue;
            };
            let context = ComponentContext {
                reference: scope.reference.clone(),
                sender: scope.sender.clone(),
            };
            scope.component.update(message.value, &context);
            let (view, effects) = Self::render(&scope.component, &scope.sender, &scope.reference)?;
            let root = scope.root.unwrap();
            let mutations = match self.runtime.update_subtree(root, view) {
                Ok(mutations) => mutations,
                Err(error) => {
                    self.invalidate();
                    return Err(ComponentError::Runtime(error));
                }
            };
            let scope = self.scope_mut(message.component).unwrap();
            effects.commit(&mut scope.effects);
            report.dispatched += 1;
            report.mutations += mutations.len();
        }
        Ok(report)
    }

    pub fn remove(&mut self, key: &Key) -> Result<(), ComponentError<A::Error>> {
        let id = self
            .find(key)
            .ok_or_else(|| ComponentError::MissingComponent(key.clone()))?;
        let parent = self.runtime.graph().root().unwrap();
        let child = self.scope(id).unwrap().root.unwrap();
        if let Err(error) = self
            .runtime
            .remove_child(parent, RelationId::Children, child)
        {
            self.invalidate();
            return Err(ComponentError::Runtime(error));
        }
        self.order.retain(|current| *current != id);
        let slot = &mut self.scopes[id.index as usize];
        let mut scope = slot.scope.take().unwrap();
        scope.reference.set(None);
        for effect in scope.effects.iter_mut().rev() {
            if let Some(cleanup) = effect.cleanup.take() {
                cleanup();
            }
        }
        slot.generation = slot.generation.wrapping_add(1);
        Ok(())
    }

    fn render(
        component: &C,
        sender: &ComponentSender<C::Message>,
        reference: &ElementRef,
    ) -> Result<(Visual, EffectDraft), ComponentError<A::Error>> {
        let mut context = ComponentViewContext {
            context: ComponentContext {
                reference: reference.clone(),
                sender: sender.clone(),
            },
            effects: EffectDraft::default(),
        };
        let view = component.view(&mut context);
        if let Some(key) = context.effects.duplicate {
            return Err(ComponentError::DuplicateEffect(key));
        }
        Ok((view, context.effects))
    }

    fn refresh_roots(&mut self) {
        let root = self.runtime.graph().root().unwrap();
        let children = self
            .runtime
            .graph()
            .children(root, RelationId::Children)
            .unwrap()
            .to_vec();
        for (id, object) in self.order.clone().into_iter().zip(children) {
            let scope = self.scope_mut(id).unwrap();
            scope.root = Some(object);
            scope.reference.set(Some(object));
        }
    }

    fn find(&self, key: &Key) -> Option<ComponentId> {
        self.order
            .iter()
            .copied()
            .find(|id| self.scope(*id).is_some_and(|scope| &scope.key == key))
    }

    fn scope(&self, id: ComponentId) -> Option<&Scope<C>> {
        let slot = self.scopes.get(id.index as usize)?;
        (slot.generation == id.generation)
            .then_some(slot.scope.as_ref())
            .flatten()
    }

    fn scope_mut(&mut self, id: ComponentId) -> Option<&mut Scope<C>> {
        let slot = self.scopes.get_mut(id.index as usize)?;
        (slot.generation == id.generation)
            .then_some(slot.scope.as_mut())
            .flatten()
    }

    fn invalidate(&mut self) {
        for slot in &mut self.scopes {
            if let Some(scope) = slot.scope.as_mut() {
                scope.root = None;
                scope.reference.set(None);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct Counter {
        cleanup: Arc<AtomicUsize>,
        sender: Arc<Mutex<Option<ComponentSender<usize>>>>,
        value: usize,
    }

    impl Component for Counter {
        type Message = usize;

        fn update(&mut self, message: usize, _context: &ComponentContext<usize>) {
            self.value += message;
        }

        fn view(&self, context: &mut ComponentViewContext<usize>) -> Visual {
            *self.sender.lock().unwrap() = Some(context.sender());
            let cleanup = Arc::clone(&self.cleanup);
            context.use_effect_guard("value", self.value, move || Cleanup(cleanup));
            TextBlock::new(self.value.to_string()).into()
        }
    }

    struct Cleanup(Arc<AtomicUsize>);

    impl Drop for Cleanup {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }

    #[test]
    fn component_updates_only_its_retained_subtree() {
        let cleanup = Arc::new(AtomicUsize::new(0));
        let senders = (0..1_024)
            .map(|_| Arc::new(Mutex::new(None)))
            .collect::<Vec<_>>();
        let mut set = ComponentSet::mount(
            RecordingAdapter::default(),
            senders.iter().enumerate().map(|(index, sender)| {
                (
                    Key::from(index),
                    Counter {
                        cleanup: Arc::clone(&cleanup),
                        sender: Arc::clone(sender),
                        value: 0,
                    },
                )
            }),
        )
        .unwrap();
        let reference = set.reference(&Key::from(512usize)).unwrap();
        let root = reference.get().unwrap();
        let sender = senders[512].lock().unwrap().as_ref().unwrap().clone();
        assert!(sender.send(1));

        let report = set.drain(1).unwrap();

        assert_eq!(
            report,
            ComponentDrain {
                dispatched: 1,
                dropped: 0,
                mutations: 1,
            }
        );
        assert_eq!(reference.get(), Some(root));
        assert_eq!(cleanup.load(Ordering::Relaxed), 1);

        assert!(sender.send(0));
        assert_eq!(
            set.drain(1).unwrap(),
            ComponentDrain {
                dispatched: 1,
                dropped: 0,
                mutations: 0,
            }
        );
        assert_eq!(cleanup.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn retired_component_drops_effects_and_async_completion() {
        let cleanup = Arc::new(AtomicUsize::new(0));
        let sender = Arc::new(Mutex::new(None));
        let mut set = ComponentSet::mount(
            RecordingAdapter::default(),
            [(
                Key::from("counter"),
                Counter {
                    cleanup: Arc::clone(&cleanup),
                    sender: Arc::clone(&sender),
                    value: 0,
                },
            )],
        )
        .unwrap();
        let reference = set.reference(&Key::from("counter")).unwrap();
        let completion = sender.lock().unwrap().as_ref().unwrap().completion();

        set.remove(&Key::from("counter")).unwrap();
        assert!(
            std::thread::spawn(move || completion.complete(1))
                .join()
                .unwrap()
        );
        let report = set.drain(1).unwrap();

        assert_eq!(reference.get(), None);
        assert_eq!(cleanup.load(Ordering::Relaxed), 1);
        assert_eq!(report.dropped, 1);
        assert_eq!(report.dispatched, 0);
    }
}
