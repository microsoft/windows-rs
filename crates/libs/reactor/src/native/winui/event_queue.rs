use super::*;

pub(super) struct QueuedEvent {
    pub(super) object: ObjectId,
    pub(super) event: EventId,
    pub(super) handler: QueuedHandler,
}

pub(super) enum QueuedHandler {
    /// The callback subscribed when the native event fired. Delivering to it keeps an occurrence
    /// queued behind a render from being lost when that render replaces the callback. `live`
    /// is cleared when the subscription ends, so removing or re-adding the event, or replacing
    /// the control, drops the occurrence.
    Callback {
        call: EventCall,
        live: Rc<Cell<bool>>,
    },
    /// The menu or command bar flyout presentation revision that raised a click on the item with
    /// this key. Callback-only updates keep the presentation, so the click resolves to the current
    /// `on_click`.
    Presentation { revision: u64, key: Key },
}

pub(super) type QueuedNativeEvent = NativeEvent<QueuedEvent>;

/// A child of a selection or reorder relation, keyed by native COM identity so event handlers can
/// map native items back to objects.
pub(super) struct NativeItem {
    pub(super) owner: ObjectId,
    pub(super) identity: usize,
}

#[derive(Default)]
pub(super) struct NativeEventQueue {
    pub(super) events: RefCell<VecDeque<QueuedNativeEvent>>,
    pub(super) feedback: RefCell<FeedbackState>,
    pub(super) items: RefCell<HashMap<ObjectId, NativeItem>>,
    pub(super) waker: RefCell<Option<Rc<dyn Fn()>>>,
    realizer: RefCell<Option<Rc<dyn Fn()>>>,
    pub(super) wake_pending: Cell<bool>,
    pub(super) content_dialogs: RefCell<NativeContentDialogScheduler>,
}

impl NativeEventQueue {
    pub(super) fn reports(&self, object: ObjectId, event: EventId) -> bool {
        self.feedback.borrow().reports(object, event)
    }

    /// Drops queued events and observations for an object whose control was replaced.
    pub(super) fn discard_object(&self, object: ObjectId) {
        self.events.borrow_mut().retain(|queued| {
            queued.event().is_none_or(|event| event.object != object)
                && queued
                    .observation()
                    .is_none_or(|observation| observation.object() != object)
        });
    }

    /// Drops queued occurrences of an object's events whose source has been replaced.
    pub(super) fn discard_events(&self, object: ObjectId, events: &[EventId]) {
        self.events.borrow_mut().retain(|queued| {
            queued
                .event()
                .is_none_or(|event| event.object != object || !events.contains(&event.event))
        });
    }

    pub(super) fn item_object(&self, owner: ObjectId, value: &IInspectable) -> Option<ObjectId> {
        let identity = com_identity(value).ok()?;
        self.items.borrow().iter().find_map(|(object, item)| {
            (item.owner == owner && item.identity == identity).then_some(*object)
        })
    }

    /// Queues an event occurrence with the state it reports, or the state alone. Returns false
    /// when there is neither.
    pub(super) fn queue(
        &self,
        observation: Option<Observation>,
        event: Option<QueuedEvent>,
    ) -> bool {
        let Some(queued) = NativeEvent::report(observation, event) else {
            return false;
        };
        self.events.borrow_mut().push_back(queued);
        true
    }

    pub(super) fn queue_realization(self: &Rc<Self>, request: RealizationRequest) {
        let mut events = self.events.borrow_mut();
        let request = coalesce_queued_realization(&mut events, request);
        events.push_back(NativeEvent::Realization(request));
        drop(events);
        WinUiAdapter::schedule_event_wake(self);
    }

    /// Applies the realizations at the front of the native queue before returning a realized
    /// element so that layout measures its content. Queued native events, messages, and the
    /// realizations behind them wait for the queued wake, as does all work when a window or
    /// application driver is already active.
    pub(super) fn realize_now(&self) {
        let realizer = self.realizer.borrow().clone();
        if let Some(realizer) = realizer {
            realizer();
        }
    }
}

fn coalesce_queued_realization(
    events: &mut VecDeque<QueuedNativeEvent>,
    request: RealizationRequest,
) -> RealizationRequest {
    let RealizationRequest::Recycle {
        collection,
        container,
        source_revision,
    } = request
    else {
        return request;
    };
    let pending = events.iter().rposition(|event| {
        matches!(
            event,
            NativeEvent::Realization(RealizationRequest::Realize {
                collection: pending_collection,
                container: pending_container,
                ..
            }) if *pending_collection == collection && *pending_container == container
        )
    });
    if let Some(pending) = pending {
        events.remove(pending);
        RealizationRequest::Cancel {
            collection,
            container,
            source_revision,
        }
    } else {
        request
    }
}

impl WinUiAdapter {
    pub fn set_event_waker(&mut self, waker: impl Fn() + 'static) {
        *self.event_queue.waker.borrow_mut() = Some(Rc::new(waker));
        if !self.event_queue.events.borrow().is_empty() {
            Self::schedule_event_wake(&self.event_queue);
        }
    }

    pub fn set_realizer(&mut self, realizer: impl Fn() + 'static) {
        *self.event_queue.realizer.borrow_mut() = Some(Rc::new(realizer));
    }
}
