use super::runtime::WindowToken;
use super::scope::{ScopeArena, ScopeError, ScopeId, ScopeState};
use crate::element::View;
use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};
use std::fmt;
use std::marker::PhantomData;
use std::rc::Rc;

pub const LOCAL_MESSAGE_QUEUE_CAPACITY: usize = 4_096;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ComponentToken {
    window: WindowToken,
    scope: ScopeId,
}

impl ComponentToken {
    pub(crate) fn scope(self) -> ScopeId {
        self.scope
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComponentStoreError {
    ComponentTypeMismatch { expected: TypeId, actual: TypeId },
    MessageTypeMismatch { expected: TypeId, actual: TypeId },
    PropsTypeMismatch { expected: TypeId, actual: TypeId },
    Scope(ScopeError),
    WindowMismatch,
}

impl From<ScopeError> for ComponentStoreError {
    fn from(value: ScopeError) -> Self {
        Self::Scope(value)
    }
}

struct MessageEnvelope {
    token: ComponentToken,
    payload: Box<dyn Any>,
}

struct ComponentQueue {
    active: HashSet<ScopeId>,
    envelopes: VecDeque<MessageEnvelope>,
    open: bool,
    wake: Option<Rc<dyn Fn()>>,
}

pub struct LocalSender<M> {
    queue: Rc<RefCell<ComponentQueue>>,
    token: ComponentToken,
    marker: PhantomData<fn(M)>,
}

impl<M> Clone for LocalSender<M> {
    fn clone(&self) -> Self {
        Self {
            queue: Rc::clone(&self.queue),
            token: self.token,
            marker: PhantomData,
        }
    }
}

impl<M: 'static> LocalSender<M> {
    pub fn send(&self, message: M) -> bool {
        let wake = {
            let mut queue = self.queue.borrow_mut();
            if !queue.open
                || !queue.active.contains(&self.token.scope)
                || queue.envelopes.len() >= LOCAL_MESSAGE_QUEUE_CAPACITY
            {
                return false;
            }
            let wake = queue
                .envelopes
                .is_empty()
                .then(|| queue.wake.clone())
                .flatten();
            queue.envelopes.push_back(MessageEnvelope {
                token: self.token,
                payload: Box::new(message),
            });
            wake
        };
        if let Some(wake) = wake {
            wake();
        }
        true
    }

    pub fn token(&self) -> ComponentToken {
        self.token
    }
}

pub struct ComponentContext<C: Component> {
    sender: LocalSender<C::Message>,
}

impl<C: Component> ComponentContext<C> {
    pub fn sender(&self) -> LocalSender<C::Message> {
        self.sender.clone()
    }
}

pub struct ViewContext<C: Component> {
    effects: Rc<RefCell<ComponentEffects>>,
    sender: LocalSender<C::Message>,
}

impl<C: Component> ViewContext<C> {
    pub fn sender(&self) -> LocalSender<C::Message> {
        self.sender.clone()
    }

    pub fn use_effect<D>(
        &mut self,
        dependency: D,
        setup: impl FnOnce() -> Option<Box<dyn FnOnce()>> + 'static,
    ) where
        D: PartialEq + 'static,
    {
        self.effects.borrow_mut().use_effect(dependency, setup);
    }
}

pub trait Component: Sized + 'static {
    type Props: Clone + PartialEq + 'static;
    type Message: 'static;

    fn create(props: &Self::Props, context: &mut ComponentContext<Self>) -> Self;
    fn changed(&mut self, props: &Self::Props, context: &mut ComponentContext<Self>);
    fn update(&mut self, message: Self::Message, context: &mut ComponentContext<Self>);
    fn view(&self, context: &mut ViewContext<Self>) -> View;
}

trait ErasedComponentFactory {
    fn apply_props(
        &self,
        store: &mut ComponentStore,
        token: ComponentToken,
    ) -> Result<bool, ComponentStoreError>;
    fn as_any(&self) -> &dyn Any;
    fn component_type(&self) -> TypeId;
    fn equals(&self, other: &dyn ErasedComponentFactory) -> bool;
    fn reserve(&self, store: &mut ComponentStore) -> Result<ComponentToken, ComponentStoreError>;
    fn type_name(&self) -> &'static str;
}

struct TypedComponentFactory<C: Component> {
    props: C::Props,
}

impl<C: Component> ErasedComponentFactory for TypedComponentFactory<C> {
    fn apply_props(
        &self,
        store: &mut ComponentStore,
        token: ComponentToken,
    ) -> Result<bool, ComponentStoreError> {
        store.apply_props(token, self.props.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn component_type(&self) -> TypeId {
        TypeId::of::<C>()
    }

    fn equals(&self, other: &dyn ErasedComponentFactory) -> bool {
        other.component_type() == TypeId::of::<C>()
            && other
                .as_any()
                .downcast_ref::<Self>()
                .is_some_and(|other| self.props == other.props)
    }

    fn reserve(&self, store: &mut ComponentStore) -> Result<ComponentToken, ComponentStoreError> {
        store.reserve_component::<C>(self.props.clone())
    }

    fn type_name(&self) -> &'static str {
        std::any::type_name::<C>()
    }
}

#[derive(Clone)]
pub struct ComponentView {
    factory: Rc<dyn ErasedComponentFactory>,
}

impl ComponentView {
    pub fn new<C: Component>(props: C::Props) -> Self {
        Self {
            factory: Rc::new(TypedComponentFactory::<C> { props }),
        }
    }

    pub(crate) fn component_type(&self) -> TypeId {
        self.factory.component_type()
    }

    pub(crate) fn apply_props(
        &self,
        store: &mut ComponentStore,
        token: ComponentToken,
    ) -> Result<bool, ComponentStoreError> {
        self.factory.apply_props(store, token)
    }

    pub(crate) fn reserve(
        &self,
        store: &mut ComponentStore,
    ) -> Result<ComponentToken, ComponentStoreError> {
        self.factory.reserve(store)
    }
}

impl fmt::Debug for ComponentView {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("Component")
            .field(&self.factory.type_name())
            .finish()
    }
}

impl PartialEq for ComponentView {
    fn eq(&self, other: &Self) -> bool {
        self.factory.equals(&*other.factory)
    }
}

trait ErasedScope {
    fn apply_props(&mut self, props: Box<dyn Any>) -> Result<bool, ComponentStoreError>;
    fn component(&self) -> &dyn Any;
    fn component_mut(&mut self) -> &mut dyn Any;
    fn dispatch(&mut self, message: Box<dyn Any>) -> Result<(), ComponentStoreError>;
    fn message_type(&self) -> TypeId;
    fn props_type(&self) -> TypeId;
    fn view(&self) -> Result<View, ComponentStoreError>;
    fn cleanup_effects(&self);
    fn commit_effects(&self);
    fn prepare_effects(&self);
}

type EffectCleanup = Box<dyn FnOnce()>;
type EffectSetup = Box<dyn FnOnce() -> Option<EffectCleanup>>;

struct EffectSlot {
    cleanup: Option<EffectCleanup>,
    dependency: Box<dyn Any>,
}

struct PendingEffect {
    dependency: Box<dyn Any>,
    setup: EffectSetup,
    slot: usize,
}

#[derive(Default)]
pub(crate) struct ComponentEffects {
    cursor: usize,
    pending: Vec<PendingEffect>,
    slots: Vec<EffectSlot>,
}

impl ComponentEffects {
    fn begin_view(&mut self) {
        self.cursor = 0;
        self.pending.clear();
    }

    fn use_effect<D>(
        &mut self,
        dependency: D,
        setup: impl FnOnce() -> Option<EffectCleanup> + 'static,
    ) where
        D: PartialEq + 'static,
    {
        let slot = self.cursor;
        self.cursor += 1;
        let changed = self
            .slots
            .get(slot)
            .and_then(|slot| slot.dependency.downcast_ref::<D>())
            != Some(&dependency);
        if changed {
            self.pending.push(PendingEffect {
                dependency: Box::new(dependency),
                setup: Box::new(setup),
                slot,
            });
        }
    }

    fn prepare(&mut self) {
        for pending in &self.pending {
            if let Some(slot) = self.slots.get_mut(pending.slot)
                && let Some(cleanup) = slot.cleanup.take()
            {
                cleanup();
            }
        }
        for slot in self.slots.iter_mut().skip(self.cursor) {
            if let Some(cleanup) = slot.cleanup.take() {
                cleanup();
            }
        }
    }

    fn commit(&mut self) {
        self.slots.truncate(self.cursor);
        for pending in self.pending.drain(..) {
            let slot = EffectSlot {
                cleanup: (pending.setup)(),
                dependency: pending.dependency,
            };
            if pending.slot == self.slots.len() {
                self.slots.push(slot);
            } else {
                self.slots[pending.slot] = slot;
            }
        }
    }

    fn cleanup(&mut self) {
        for slot in self.slots.iter_mut().rev() {
            if let Some(cleanup) = slot.cleanup.take() {
                cleanup();
            }
        }
        self.slots.clear();
        self.pending.clear();
        self.cursor = 0;
    }
}

struct TypedScope<C, P, M> {
    component: C,
    effects: Rc<RefCell<ComponentEffects>>,
    props: P,
    changed: fn(&mut C, &P, LocalSender<M>),
    sender: LocalSender<M>,
    update: fn(&mut C, M, LocalSender<M>),
    view: Box<dyn Fn(&C, LocalSender<M>, Rc<RefCell<ComponentEffects>>) -> View>,
}

impl<C, P, M> Drop for TypedScope<C, P, M> {
    fn drop(&mut self) {
        self.effects.borrow_mut().cleanup();
    }
}

impl<C, P, M> ErasedScope for TypedScope<C, P, M>
where
    C: 'static,
    P: Clone + PartialEq + 'static,
    M: 'static,
{
    fn apply_props(&mut self, props: Box<dyn Any>) -> Result<bool, ComponentStoreError> {
        let actual = props.as_ref().type_id();
        let props = props
            .downcast::<P>()
            .map_err(|_| ComponentStoreError::PropsTypeMismatch {
                expected: TypeId::of::<P>(),
                actual,
            })?;
        if self.props == *props {
            return Ok(false);
        }
        self.props = *props;
        (self.changed)(&mut self.component, &self.props, self.sender.clone());
        Ok(true)
    }

    fn component(&self) -> &dyn Any {
        &self.component
    }

    fn component_mut(&mut self) -> &mut dyn Any {
        &mut self.component
    }

    fn dispatch(&mut self, message: Box<dyn Any>) -> Result<(), ComponentStoreError> {
        let actual = message.as_ref().type_id();
        let message =
            message
                .downcast::<M>()
                .map_err(|_| ComponentStoreError::MessageTypeMismatch {
                    expected: TypeId::of::<M>(),
                    actual,
                })?;
        (self.update)(&mut self.component, *message, self.sender.clone());
        Ok(())
    }

    fn message_type(&self) -> TypeId {
        TypeId::of::<M>()
    }

    fn props_type(&self) -> TypeId {
        TypeId::of::<P>()
    }

    fn view(&self) -> Result<View, ComponentStoreError> {
        self.effects.borrow_mut().begin_view();
        Ok((self.view)(
            &self.component,
            self.sender.clone(),
            Rc::clone(&self.effects),
        ))
    }

    fn cleanup_effects(&self) {
        self.effects.borrow_mut().cleanup();
    }

    fn commit_effects(&self) {
        self.effects.borrow_mut().commit();
    }

    fn prepare_effects(&self) {
        self.effects.borrow_mut().prepare();
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DrainReport {
    pub blocked: bool,
    pub dispatched: usize,
    pub dropped: usize,
    pub(crate) dirty: Vec<ComponentToken>,
}

pub struct ComponentStore {
    window: WindowToken,
    scopes: ScopeArena<Box<dyn ErasedScope>>,
    queue: Rc<RefCell<ComponentQueue>>,
}

impl ComponentStore {
    pub fn new(window: WindowToken) -> Self {
        Self {
            window,
            scopes: ScopeArena::new(),
            queue: Rc::new(RefCell::new(ComponentQueue {
                active: HashSet::new(),
                envelopes: VecDeque::new(),
                open: true,
                wake: None,
            })),
        }
    }

    pub fn reserve<C, P, M>(
        &mut self,
        component: C,
        props: P,
        changed: fn(&mut C, &P, LocalSender<M>),
        update: fn(&mut C, M, LocalSender<M>),
        view: fn(&C, LocalSender<M>) -> View,
    ) -> Result<ComponentToken, ComponentStoreError>
    where
        C: 'static,
        P: Clone + PartialEq + 'static,
        M: 'static,
    {
        let queue = Rc::clone(&self.queue);
        let window = self.window;
        let view = Box::new(move |component: &C, sender, _effects| view(component, sender));
        let scope = self.scopes.reserve_with(move |scope| {
            queue.borrow_mut().active.insert(scope);
            let sender = LocalSender {
                queue: Rc::clone(&queue),
                token: ComponentToken { window, scope },
                marker: PhantomData,
            };
            Box::new(TypedScope {
                component,
                effects: Rc::new(RefCell::new(ComponentEffects::default())),
                props,
                changed,
                sender,
                update,
                view,
            }) as Box<dyn ErasedScope>
        })?;
        Ok(ComponentToken {
            window: self.window,
            scope,
        })
    }

    pub fn reserve_component<C: Component>(
        &mut self,
        props: C::Props,
    ) -> Result<ComponentToken, ComponentStoreError> {
        fn changed<C: Component>(
            component: &mut C,
            props: &C::Props,
            sender: LocalSender<C::Message>,
        ) {
            component.changed(props, &mut ComponentContext { sender });
        }

        fn update<C: Component>(
            component: &mut C,
            message: C::Message,
            sender: LocalSender<C::Message>,
        ) {
            component.update(message, &mut ComponentContext { sender });
        }

        fn view<C: Component>(
            component: &C,
            sender: LocalSender<C::Message>,
            effects: Rc<RefCell<ComponentEffects>>,
        ) -> View {
            component.view(&mut ViewContext { effects, sender })
        }

        let queue = Rc::clone(&self.queue);
        let window = self.window;
        let scope = self.scopes.reserve_with(move |scope| {
            queue.borrow_mut().active.insert(scope);
            let sender = LocalSender {
                queue: Rc::clone(&queue),
                token: ComponentToken { window, scope },
                marker: PhantomData,
            };
            let component = C::create(
                &props,
                &mut ComponentContext {
                    sender: sender.clone(),
                },
            );
            Box::new(TypedScope {
                component,
                effects: Rc::new(RefCell::new(ComponentEffects::default())),
                props,
                changed: changed::<C>,
                sender,
                update: update::<C>,
                view: Box::new(view::<C>),
            }) as Box<dyn ErasedScope>
        })?;
        Ok(ComponentToken {
            window: self.window,
            scope,
        })
    }

    pub fn publish(&mut self, token: ComponentToken) -> Result<(), ComponentStoreError> {
        self.validate_window(token)?;
        self.scopes.publish(token.scope)?;
        Ok(())
    }

    pub fn retire(&mut self, token: ComponentToken) -> Result<(), ComponentStoreError> {
        self.validate_window(token)?;
        self.scopes.retire(token.scope)?;
        let mut queue = self.queue.borrow_mut();
        queue.active.remove(&token.scope);
        queue
            .envelopes
            .retain(|envelope| envelope.token.scope != token.scope);
        Ok(())
    }

    pub fn remove(&mut self, token: ComponentToken) -> Result<(), ComponentStoreError> {
        self.validate_window(token)?;
        self.scopes.remove(token.scope)?;
        let mut queue = self.queue.borrow_mut();
        queue.active.remove(&token.scope);
        queue
            .envelopes
            .retain(|envelope| envelope.token.scope != token.scope);
        Ok(())
    }

    pub fn sender<M: 'static>(
        &self,
        token: ComponentToken,
    ) -> Result<LocalSender<M>, ComponentStoreError> {
        self.validate_window(token)?;
        let scope = self.scopes.get(token.scope)?;
        let actual = TypeId::of::<M>();
        let expected = scope.message_type();
        if actual != expected {
            return Err(ComponentStoreError::MessageTypeMismatch { expected, actual });
        }
        Ok(LocalSender {
            queue: Rc::clone(&self.queue),
            token,
            marker: PhantomData,
        })
    }

    pub fn apply_props<P: 'static>(
        &mut self,
        token: ComponentToken,
        props: P,
    ) -> Result<bool, ComponentStoreError> {
        self.validate_window(token)?;
        let scope = self.scopes.get_mut(token.scope)?;
        let actual = TypeId::of::<P>();
        let expected = scope.props_type();
        if actual != expected {
            return Err(ComponentStoreError::PropsTypeMismatch { expected, actual });
        }
        scope.apply_props(Box::new(props))
    }

    pub fn component<C: 'static>(&self, token: ComponentToken) -> Result<&C, ComponentStoreError> {
        self.validate_window(token)?;
        let component = self.scopes.get(token.scope)?.component();
        let actual = component.type_id();
        component
            .downcast_ref()
            .ok_or(ComponentStoreError::ComponentTypeMismatch {
                expected: TypeId::of::<C>(),
                actual,
            })
    }

    pub fn component_mut<C: 'static>(
        &mut self,
        token: ComponentToken,
    ) -> Result<&mut C, ComponentStoreError> {
        self.validate_window(token)?;
        let component = self.scopes.get_mut(token.scope)?.component_mut();
        let actual = (*component).type_id();
        component
            .downcast_mut()
            .ok_or(ComponentStoreError::ComponentTypeMismatch {
                expected: TypeId::of::<C>(),
                actual,
            })
    }

    pub fn drain(&mut self, budget: usize) -> Result<DrainReport, ComponentStoreError> {
        let mut report = DrainReport::default();
        for _ in 0..budget {
            let Some(envelope) = self.queue.borrow_mut().envelopes.pop_front() else {
                break;
            };
            if envelope.token.window != self.window {
                report.dropped += 1;
                continue;
            }
            let state = match self.scopes.state(envelope.token.scope) {
                Ok(state) => state,
                Err(ScopeError::Stale(_)) => {
                    report.dropped += 1;
                    continue;
                }
                Err(error) => return Err(error.into()),
            };
            match state {
                ScopeState::Reserved => {
                    self.queue.borrow_mut().envelopes.push_front(envelope);
                    report.blocked = true;
                    break;
                }
                ScopeState::Retiring => {
                    report.dropped += 1;
                }
                ScopeState::Published => {
                    self.scopes
                        .get_mut(envelope.token.scope)?
                        .dispatch(envelope.payload)?;
                    report.dispatched += 1;
                    report.dirty.push(envelope.token);
                }
            }
        }
        Ok(report)
    }

    pub fn pending(&self) -> usize {
        self.queue.borrow().envelopes.len()
    }

    pub(crate) fn pending_tokens(&self) -> Vec<ComponentToken> {
        self.queue
            .borrow()
            .envelopes
            .iter()
            .map(|envelope| envelope.token)
            .collect()
    }

    pub fn view(&self, token: ComponentToken) -> Result<View, ComponentStoreError> {
        self.validate_window(token)?;
        self.scopes.get(token.scope)?.view()
    }

    pub fn cleanup_effects(&self, token: ComponentToken) -> Result<(), ComponentStoreError> {
        self.validate_window(token)?;
        self.scopes.get(token.scope)?.cleanup_effects();
        Ok(())
    }

    pub fn commit_effects(&self, token: ComponentToken) -> Result<(), ComponentStoreError> {
        self.validate_window(token)?;
        self.scopes.get(token.scope)?.commit_effects();
        Ok(())
    }

    pub fn prepare_effects(&self, token: ComponentToken) -> Result<(), ComponentStoreError> {
        self.validate_window(token)?;
        self.scopes.get(token.scope)?.prepare_effects();
        Ok(())
    }

    pub(crate) fn token(&self, scope: ScopeId) -> Result<ComponentToken, ComponentStoreError> {
        self.scopes.state(scope)?;
        Ok(ComponentToken {
            window: self.window,
            scope,
        })
    }

    pub(crate) fn set_waker(&mut self, wake: Rc<dyn Fn()>) {
        self.queue.borrow_mut().wake = Some(wake);
    }

    pub(crate) fn close(&mut self) {
        let mut queue = self.queue.borrow_mut();
        queue.open = false;
        queue.active.clear();
        queue.envelopes.clear();
        queue.wake = None;
    }

    fn validate_window(&self, token: ComponentToken) -> Result<(), ComponentStoreError> {
        if token.window == self.window {
            Ok(())
        } else {
            Err(ComponentStoreError::WindowMismatch)
        }
    }
}

impl Drop for ComponentStore {
    fn drop(&mut self) {
        self.close();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TextBlock;
    use crate::core::{WindowId, WindowToken};

    struct State {
        changed: usize,
        sender: Option<LocalSender<u32>>,
        value: u32,
    }

    fn changed(state: &mut State, _props: &String, _sender: LocalSender<u32>) {
        state.changed += 1;
    }

    fn update(state: &mut State, message: u32, _sender: LocalSender<u32>) {
        state.value += message;
        if message == 1 {
            state.sender.as_ref().unwrap().send(2);
        }
    }

    fn view(_state: &State, _sender: LocalSender<u32>) -> View {
        View::Empty
    }

    fn store() -> ComponentStore {
        ComponentStore::new(WindowToken::new(WindowId::allocate()))
    }

    #[test]
    fn reserved_messages_wait_for_publication_and_reentrant_messages_queue() {
        let mut store = store();
        let token = store
            .reserve(
                State {
                    changed: 0,
                    sender: None,
                    value: 0,
                },
                "first".to_string(),
                changed,
                update,
                view,
            )
            .unwrap();
        let sender = store.sender::<u32>(token).unwrap();
        store.component_mut::<State>(token).unwrap().sender = Some(sender.clone());
        sender.send(1);

        assert_eq!(
            store.drain(10),
            Ok(DrainReport {
                blocked: true,
                dispatched: 0,
                dropped: 0,
                dirty: Vec::new(),
            })
        );
        store.publish(token).unwrap();
        assert_eq!(store.drain(10).unwrap().dispatched, 2);
        assert_eq!(store.component::<State>(token).unwrap().value, 3);
    }

    #[test]
    fn props_are_typed_coalesced_and_applied_before_messages() {
        let mut store = store();
        let token = store
            .reserve(
                State {
                    changed: 0,
                    sender: None,
                    value: 0,
                },
                "first".to_string(),
                changed,
                update,
                view,
            )
            .unwrap();
        store.publish(token).unwrap();

        assert_eq!(store.apply_props(token, "first".to_string()), Ok(false));
        assert_eq!(store.apply_props(token, "second".to_string()), Ok(true));
        assert_eq!(store.component::<State>(token).unwrap().changed, 1);
        assert!(matches!(
            store.apply_props(token, 1u32),
            Err(ComponentStoreError::PropsTypeMismatch { .. })
        ));
    }

    #[test]
    fn retiring_and_stale_tokens_cannot_reach_a_reused_slot() {
        let mut store = store();
        let first = store
            .reserve(
                State {
                    changed: 0,
                    sender: None,
                    value: 0,
                },
                String::new(),
                changed,
                update,
                view,
            )
            .unwrap();
        store.publish(first).unwrap();
        let old_sender = store.sender::<u32>(first).unwrap();
        store.retire(first).unwrap();
        old_sender.send(1);
        assert_eq!(store.drain(10).unwrap().dropped, 0);
        store.remove(first).unwrap();

        let second = store
            .reserve(
                State {
                    changed: 0,
                    sender: None,
                    value: 0,
                },
                String::new(),
                changed,
                update,
                view,
            )
            .unwrap();
        assert_ne!(first, second);
        store.publish(second).unwrap();
        old_sender.send(4);

        assert_eq!(store.drain(10).unwrap().dropped, 0);
        assert_eq!(store.component::<State>(second).unwrap().value, 0);
    }

    #[test]
    fn sender_creation_checks_the_message_type() {
        let mut store = store();
        let token = store
            .reserve(
                State {
                    changed: 0,
                    sender: None,
                    value: 0,
                },
                String::new(),
                changed,
                update,
                view,
            )
            .unwrap();

        assert!(matches!(
            store.sender::<String>(token),
            Err(ComponentStoreError::MessageTypeMismatch { .. })
        ));
    }

    #[test]
    fn valid_sender_wakes_once_and_retirement_blocks_new_traffic() {
        let wakes = Rc::new(std::cell::Cell::new(0));
        let wake_capture = Rc::clone(&wakes);
        let mut store = store();
        store.set_waker(Rc::new(move || {
            wake_capture.set(wake_capture.get() + 1);
        }));
        let token = store
            .reserve(
                State {
                    changed: 0,
                    sender: None,
                    value: 0,
                },
                String::new(),
                changed,
                update,
                view,
            )
            .unwrap();
        store.publish(token).unwrap();
        let sender = store.sender::<u32>(token).unwrap();

        sender.send(1);
        sender.send(2);
        assert_eq!(wakes.get(), 1);
        assert_eq!(store.pending(), 2);
        store.retire(token).unwrap();
        assert_eq!(store.pending(), 0);
        sender.send(3);
        assert_eq!(store.pending(), 0);
        assert_eq!(wakes.get(), 1);
    }

    #[test]
    fn local_message_queue_reports_backpressure_at_its_fixed_capacity() {
        let mut store = store();
        let token = store
            .reserve(
                State {
                    changed: 0,
                    sender: None,
                    value: 0,
                },
                String::new(),
                changed,
                update,
                view,
            )
            .unwrap();
        store.publish(token).unwrap();
        let sender = store.sender::<u32>(token).unwrap();

        for value in 0..LOCAL_MESSAGE_QUEUE_CAPACITY {
            assert!(sender.send(value as u32));
        }
        assert!(!sender.send(u32::MAX));
        assert_eq!(store.pending(), LOCAL_MESSAGE_QUEUE_CAPACITY);
    }

    struct Counter {
        value: u32,
    }

    impl Component for Counter {
        type Props = u32;
        type Message = u32;

        fn create(props: &Self::Props, context: &mut ComponentContext<Self>) -> Self {
            context.sender().send(1);
            Self { value: *props }
        }

        fn changed(&mut self, props: &Self::Props, _context: &mut ComponentContext<Self>) {
            self.value = *props;
        }

        fn update(&mut self, message: Self::Message, _context: &mut ComponentContext<Self>) {
            self.value += message;
        }

        fn view(&self, _context: &mut ViewContext<Self>) -> View {
            View::native(TextBlock::new().text(self.value.to_string()))
        }
    }

    #[test]
    fn component_factory_creates_with_a_reserved_sender_and_composes() {
        let first = ComponentView::new::<Counter>(2);
        let same = ComponentView::new::<Counter>(2);
        let changed = ComponentView::new::<Counter>(3);
        assert_eq!(first, same);
        assert_ne!(first, changed);

        let mut store = store();
        let token = first.reserve(&mut store).unwrap();
        assert_eq!(store.pending(), 1);
        assert!(store.drain(10).unwrap().blocked);
        store.publish(token).unwrap();
        assert_eq!(store.drain(10).unwrap().dispatched, 1);
        assert_eq!(store.component::<Counter>(token).unwrap().value, 3);
        assert_eq!(
            store.view(token),
            Ok(View::native(TextBlock::new().text("3")))
        );
    }
}
