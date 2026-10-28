use super::*;

pub(super) const MESSAGE_CAPACITY: usize = 4_096;

pub(super) struct Message {
    pub(super) component: ComponentId,
    pub(super) control: Option<Arc<TaskControl>>,
    pub(super) sequence: u64,
    pub(super) value: MessageValue,
}

pub(super) enum MessageValue {
    Send(Box<dyn Any + Send>),
    Local(Box<dyn FnOnce() -> Box<dyn Any> + Send>),
}

pub(super) struct LocalMessage {
    pub(super) component: ComponentId,
    pub(super) sequence: u64,
    pub(super) value: LocalMessageValue,
}

pub(super) enum LocalMessageValue {
    Message(Box<dyn Any>),
    Input(ComponentNode),
}

#[derive(Default)]
pub(super) struct MessageQueue {
    pub(super) closed: bool,
    pub(super) messages: VecDeque<Message>,
    pub(super) next_sequence: u64,
    pub(super) wake_pending: bool,
    pub(super) waker: Option<Arc<dyn Fn() + Send + Sync>>,
}

pub(super) type SharedQueue = Arc<Mutex<MessageQueue>>;

pub(super) type LocalQueue = Rc<RefCell<VecDeque<LocalMessage>>>;

pub(crate) struct ApplicationMessages<M> {
    sender: LocalSender<M>,
}

impl<M: 'static> ApplicationMessages<M> {
    pub(crate) fn new(waker: impl Fn() + Send + Sync + 'static) -> Self {
        Self {
            sender: LocalSender {
                component: ComponentId {
                    index: 0,
                    generation: 0,
                },
                queue: Rc::new(RefCell::new(VecDeque::new())),
                wake: Arc::new(Mutex::new(MessageQueue {
                    waker: Some(Arc::new(waker)),
                    ..Default::default()
                })),
                marker: PhantomData,
            },
        }
    }

    pub(crate) fn sender(&self) -> LocalSender<M> {
        self.sender.clone()
    }

    pub(crate) fn pop(&self) -> Option<M> {
        self.sender.wake.lock().unwrap().wake_pending = false;
        self.sender.queue.borrow_mut().pop_front().map(|message| {
            let LocalMessageValue::Message(value) = message.value else {
                unreachable!()
            };
            *value.downcast::<M>().unwrap()
        })
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.sender.queue.borrow().is_empty()
    }
}

impl<M> Drop for ApplicationMessages<M> {
    fn drop(&mut self) {
        self.sender.wake.lock().unwrap().closed = true;
        self.sender.queue.borrow_mut().clear();
    }
}

#[derive(Clone)]
pub(super) struct ComponentQueues {
    pub(super) shared: SharedQueue,
    pub(super) local: LocalQueue,
}

pub struct LocalSender<M> {
    pub(super) component: ComponentId,
    pub(super) queue: LocalQueue,
    pub(super) wake: SharedQueue,
    pub(super) marker: PhantomData<fn(M)>,
}

impl<M> Clone for LocalSender<M> {
    fn clone(&self) -> Self {
        Self {
            component: self.component,
            queue: Rc::clone(&self.queue),
            wake: Arc::clone(&self.wake),
            marker: PhantomData,
        }
    }
}

impl<M: 'static> LocalSender<M> {
    #[must_use]
    pub fn send(&self, value: M) -> bool {
        let mut wake_queue = self.wake.lock().unwrap();
        if wake_queue.closed {
            return false;
        }
        let mut queue = self.queue.borrow_mut();
        if queue.len() >= MESSAGE_CAPACITY {
            return false;
        }
        let sequence = wake_queue.next_sequence;
        wake_queue.next_sequence = wake_queue.next_sequence.wrapping_add(1);
        queue.push_back(LocalMessage {
            component: self.component,
            sequence,
            value: LocalMessageValue::Message(Box::new(value)),
        });
        drop(queue);
        let wake = (!wake_queue.wake_pending)
            .then(|| wake_queue.waker.clone())
            .flatten();
        wake_queue.wake_pending |= wake.is_some();
        drop(wake_queue);
        if let Some(wake) = wake {
            wake();
        }
        true
    }

    pub fn callback<T, F>(&self, map: F) -> Callback<T>
    where
        F: Fn(T) -> M + 'static,
    {
        let sender = self.clone();
        let callback = move |value| {
            _ = sender.send(map(value));
        };
        if size_of::<F>() == 0 {
            Callback::new_identified(
                CallbackIdentity {
                    queue: Rc::as_ptr(&self.queue) as usize,
                    component: self.component,
                    mapping: TypeId::of::<F>(),
                },
                callback,
            )
        } else {
            Callback::new(callback)
        }
    }

    pub fn forward(&self) -> Callback<M> {
        self.callback(std::convert::identity)
    }

    pub fn message(&self, message: M) -> Callback<()>
    where
        M: Clone,
    {
        self.callback(move |()| message.clone())
    }
}

pub struct ComponentSender<M> {
    pub(super) component: ComponentId,
    pub(super) queue: SharedQueue,
    pub(super) marker: PhantomData<fn(M)>,
}

impl<M> Clone for ComponentSender<M> {
    fn clone(&self) -> Self {
        Self {
            component: self.component,
            queue: Arc::clone(&self.queue),
            marker: PhantomData,
        }
    }
}

impl<M: Send + 'static> ComponentSender<M> {
    #[must_use]
    pub fn send(&self, value: M) -> bool {
        self.enqueue(value, None)
    }

    pub fn completion(&self) -> ComponentCompletion<M> {
        ComponentCompletion(self.clone())
    }

    pub fn callback<T, F>(&self, map: F) -> Callback<T>
    where
        F: Fn(T) -> M + 'static,
    {
        let sender = self.clone();
        let callback = move |value| {
            _ = sender.send(map(value));
        };
        if size_of::<F>() == 0 {
            Callback::new_identified(
                CallbackIdentity {
                    queue: Arc::as_ptr(&self.queue) as usize,
                    component: self.component,
                    mapping: TypeId::of::<F>(),
                },
                callback,
            )
        } else {
            Callback::new(callback)
        }
    }

    pub fn forward(&self) -> Callback<M> {
        self.callback(std::convert::identity)
    }

    pub fn message(&self, message: M) -> Callback<()>
    where
        M: Clone,
    {
        self.callback(move |()| message.clone())
    }

    pub(super) fn send_controlled(&self, value: M, control: Arc<TaskControl>) -> bool {
        self.enqueue(value, Some(control))
    }

    fn enqueue(&self, value: M, control: Option<Arc<TaskControl>>) -> bool {
        let mut queue = self.queue.lock().unwrap();
        if queue.closed || queue.messages.len() >= MESSAGE_CAPACITY {
            if let Some(control) = control {
                control.reject();
            }
            return false;
        }
        let sequence = queue.next_sequence;
        queue.next_sequence = queue.next_sequence.wrapping_add(1);
        queue.messages.push_back(Message {
            component: self.component,
            control,
            sequence,
            value: MessageValue::Send(Box::new(value)),
        });
        let wake = (!queue.wake_pending).then(|| queue.waker.clone()).flatten();
        queue.wake_pending |= wake.is_some();
        drop(queue);
        if let Some(wake) = wake {
            wake();
        }
        true
    }
}

pub struct ComponentCompletion<M>(ComponentSender<M>);

impl<M> Clone for ComponentCompletion<M> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<M: Send + 'static> ComponentCompletion<M> {
    #[must_use]
    pub fn complete(self, value: M) -> bool {
        self.0.send(value)
    }
}
