use super::runtime::WindowToken;
use super::scope::{ScopeArena, ScopeError, ScopeId, ScopeState};
use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::rc::Rc;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ComponentToken {
    window: WindowToken,
    scope: ScopeId,
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

pub struct LocalSender<M> {
    queue: Rc<RefCell<VecDeque<MessageEnvelope>>>,
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
    pub fn send(&self, message: M) {
        self.queue.borrow_mut().push_back(MessageEnvelope {
            token: self.token,
            payload: Box::new(message),
        });
    }

    pub fn token(&self) -> ComponentToken {
        self.token
    }
}

trait ErasedScope {
    fn apply_props(&mut self, props: Box<dyn Any>) -> Result<bool, ComponentStoreError>;
    fn component(&self) -> &dyn Any;
    fn component_mut(&mut self) -> &mut dyn Any;
    fn dispatch(&mut self, message: Box<dyn Any>) -> Result<(), ComponentStoreError>;
    fn message_type(&self) -> TypeId;
    fn props_type(&self) -> TypeId;
}

struct TypedScope<C, P, M> {
    component: C,
    props: P,
    changed: fn(&mut C, &P),
    update: fn(&mut C, M),
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
        (self.changed)(&mut self.component, &self.props);
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
        (self.update)(&mut self.component, *message);
        Ok(())
    }

    fn message_type(&self) -> TypeId {
        TypeId::of::<M>()
    }

    fn props_type(&self) -> TypeId {
        TypeId::of::<P>()
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DrainReport {
    pub blocked: bool,
    pub dispatched: usize,
    pub dropped: usize,
}

pub struct ComponentStore {
    window: WindowToken,
    scopes: ScopeArena<Box<dyn ErasedScope>>,
    queue: Rc<RefCell<VecDeque<MessageEnvelope>>>,
}

impl ComponentStore {
    pub fn new(window: WindowToken) -> Self {
        Self {
            window,
            scopes: ScopeArena::new(),
            queue: Rc::new(RefCell::new(VecDeque::new())),
        }
    }

    pub fn reserve<C, P, M>(
        &mut self,
        component: C,
        props: P,
        changed: fn(&mut C, &P),
        update: fn(&mut C, M),
    ) -> Result<ComponentToken, ComponentStoreError>
    where
        C: 'static,
        P: Clone + PartialEq + 'static,
        M: 'static,
    {
        let scope = self.scopes.reserve(Box::new(TypedScope {
            component,
            props,
            changed,
            update,
        }))?;
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
        Ok(())
    }

    pub fn remove(&mut self, token: ComponentToken) -> Result<(), ComponentStoreError> {
        self.validate_window(token)?;
        self.scopes.remove(token.scope)?;
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
            let Some(envelope) = self.queue.borrow_mut().pop_front() else {
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
                    self.queue.borrow_mut().push_front(envelope);
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
                }
            }
        }
        Ok(report)
    }

    pub fn pending(&self) -> usize {
        self.queue.borrow().len()
    }

    fn validate_window(&self, token: ComponentToken) -> Result<(), ComponentStoreError> {
        if token.window == self.window {
            Ok(())
        } else {
            Err(ComponentStoreError::WindowMismatch)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{WindowId, WindowToken};

    struct State {
        changed: usize,
        sender: Option<LocalSender<u32>>,
        value: u32,
    }

    fn changed(state: &mut State, _props: &String) {
        state.changed += 1;
    }

    fn update(state: &mut State, message: u32) {
        state.value += message;
        if message == 1 {
            state.sender.as_ref().unwrap().send(2);
        }
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
            )
            .unwrap();
        store.publish(first).unwrap();
        let old_sender = store.sender::<u32>(first).unwrap();
        store.retire(first).unwrap();
        old_sender.send(1);
        assert_eq!(store.drain(10).unwrap().dropped, 1);
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
            )
            .unwrap();
        assert_ne!(first, second);
        store.publish(second).unwrap();
        old_sender.send(4);

        assert_eq!(store.drain(10).unwrap().dropped, 1);
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
            )
            .unwrap();

        assert!(matches!(
            store.sender::<String>(token),
            Err(ComponentStoreError::MessageTypeMismatch { .. })
        ));
    }
}
