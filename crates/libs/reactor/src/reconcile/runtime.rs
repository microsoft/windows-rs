use super::*;

impl<A: Adapter> Runtime<A> {
    pub fn new(adapter: A) -> Self {
        Self {
            graph: RetainedGraph::default(),
            adapter,
            mutations: Vec::new(),
            virtual_refreshes: VecDeque::new(),
            validator: DeclarationValidator::default(),
            native_event_active: Rc::new(Cell::new(false)),
            references: ReferenceEndpoint::new(),
        }
    }

    pub fn graph(&self) -> &RetainedGraph {
        &self.graph
    }

    pub fn adapter(&self) -> &A {
        &self.adapter
    }

    /// Takes the planned mutations as a batch. The retained graph already reflects the batch, so
    /// later planning, including a realization nested inside the batch's native application,
    /// builds on it.
    pub(super) fn take_batch(&mut self, references: Vec<ReferenceChange>) -> NativeBatch {
        let mut mutations = std::mem::take(&mut self.mutations);
        defer_window_title_bar_sets(&mut mutations);
        NativeBatch {
            mutations,
            references,
        }
    }

    pub(crate) fn apply_batch(&mut self, batch: NativeBatch) -> UpdateStats {
        self.adapter.apply(&batch.mutations);
        self.publish_batch(batch)
    }

    /// Publishes the reference changes of a batch whose mutations were applied, and keeps its
    /// mutation buffer for the next batch unless a large batch grew it.
    pub(crate) fn publish_batch(&mut self, batch: NativeBatch) -> UpdateStats {
        let mut mutations = batch.mutations;
        apply_reference_changes(&self.references, batch.references);
        let stats = UpdateStats::capture(&mutations);
        if mutations.capacity() > 256 {
            mutations = Vec::with_capacity(256);
        }
        if mutations.capacity() > self.mutations.capacity() {
            mutations.clear();
            self.mutations = mutations;
        }
        stats
    }

    pub(crate) fn set_imperative_waker(&self, waker: impl Fn() + 'static) {
        self.references.set_waker(waker);
    }

    fn dispatch_imperative(&mut self) -> bool {
        let Some(queued) = self.references.pop() else {
            return false;
        };
        // Revocations must also reach retiring objects, whose native subscriptions stay live until
        // the exit transition completes.
        let available = if matches!(queued.request, ImperativeRequest::RevokeObservation { .. }) {
            self.graph.is_allocated(queued.object)
        } else {
            self.graph.kind(queued.object).is_some() && queued.is_current()
        };
        if available {
            self.adapter.imperative(queued.request);
        } else {
            queued.request.complete_unavailable();
        }
        true
    }

    pub(super) fn pop_native_work(&mut self) -> Option<NativeWork> {
        let event = self.adapter.pop_native_event()?;
        // Only a property's controlled event reports its state; an action that also observes the
        // property, such as `Click`, dispatches even when the state is settled.
        let report = match (&event.observation, &event.event) {
            (Some(Observation::SetProperty { object, property }), Some(dispatch)) => {
                Some(self.graph.kind(*object).is_some_and(|kind| {
                    controlled_event(kind, property.id) == Some(dispatch.event)
                }))
            }
            _ => None,
        };
        // A report followed by another report of the same state is skipped, so a control that
        // passes through intermediate states within one interaction reports only where it settled.
        if report == Some(true) {
            let current = event.event.as_ref().unwrap();
            if self.adapter.peek_native_dispatch().is_some_and(|next| {
                next.observation.is_some()
                    && next.event.is_some_and(|next| {
                        next.object == current.object && next.event == current.event
                    })
            }) {
                return Some(NativeWork::Maintenance);
            }
        }
        let action = report == Some(false);
        let dispatch = event
            .observation
            .is_none_or(|observation| self.graph.apply_observation(observation, event.record))
            || action;
        if let Some(completion) = event.retirement {
            self.apply_retirement_completion(completion);
        }
        if let Some(request) = event.realization {
            if let RealizationRequest::Recycle {
                collection,
                container,
                ..
            }
            | RealizationRequest::Cancel {
                collection,
                container,
                ..
            } = request
            {
                self.virtual_refreshes.retain(|refresh| {
                    !matches!(
                        refresh,
                        RealizationRequest::Realize {
                            collection: refresh_collection,
                            container: refresh_container,
                            ..
                        } if *refresh_collection == collection && *refresh_container == container
                    )
                });
            }
            return Some(
                self.graph
                    .virtual_work(request)
                    .map_or(NativeWork::Maintenance, NativeWork::Virtual),
            );
        }
        let Some(mut dispatch) = event.event.filter(|_| dispatch) else {
            return Some(NativeWork::Maintenance);
        };
        if !self.graph.validate_event_dispatch(&mut dispatch) {
            return Some(NativeWork::Maintenance);
        }
        self.native_event_active.set(true);
        Some(NativeWork::Event(NativeEventDispatch::new(
            dispatch,
            Rc::clone(&self.native_event_active),
        )))
    }

    pub(crate) fn next_native_work(&mut self) -> Option<NativeWork> {
        assert!(
            !self.native_event_active.get(),
            "native event dispatch is in progress"
        );
        if let Some(work) = self.pop_native_work() {
            return Some(work);
        }
        if let Some(request) = self.virtual_refreshes.pop_front() {
            return Some(
                self.graph
                    .virtual_work(request)
                    .map_or(NativeWork::Maintenance, NativeWork::Virtual),
            );
        }
        for _ in 0..64 {
            if !self.dispatch_imperative() {
                return None;
            }
        }
        self.references.wake();
        None
    }

    /// Applies queued native observations and retirement completions. Returns `false` while a
    /// native callback or virtual realization must run before the next update.
    pub(crate) fn prepare_update(&mut self) -> bool {
        if self.native_event_active.get() {
            return true;
        }
        if !self.virtual_refreshes.is_empty() {
            return false;
        }
        while let Some(next) = self.adapter.peek_native_event() {
            if next.event.is_some() || next.realization.is_some() {
                return false;
            }
            let event = self.adapter.pop_native_event().unwrap();
            if let Some(observation) = event.observation {
                self.graph.apply_observation(observation, event.record);
            }
            if let Some(completion) = event.retirement {
                self.apply_retirement_completion(completion);
            }
        }
        true
    }

    pub(crate) fn assert_ready(&mut self) {
        assert!(self.prepare_update(), "native events are pending");
    }

    pub fn update(&mut self, root: impl Into<View>) -> UpdateStats {
        self.assert_ready();
        self.mutations.clear();
        let declaration = root.into().0.object();
        if let Some(root) = self.graph.root {
            let mut remaining = MAX_OBJECTS;
            if self
                .graph
                .matches_declaration(root, &declaration, &mut remaining)
            {
                return UpdateStats::default();
            }
        }
        let object_count = self.validator.validate(&declaration);
        if self.graph.objects.is_empty() {
            self.graph.objects.reserve_exact(object_count);
        }
        if let Some(current) = self.graph.root {
            let previous = self.graph.get(current).unwrap().kind;
            let next = declaration.kind;
            assert!(
                previous == next,
                "root type changed from {previous:?} to {next:?}"
            );
        }
        let mut references = Vec::new();
        let mut virtual_refreshes = Vec::new();
        let mut planner = Planner {
            retained: &mut self.graph,
            mutations: &mut self.mutations,
            references: &mut references,
            virtual_refreshes: Some(&mut virtual_refreshes),
        };
        let root = match planner.retained.root {
            Some(current) => planner.reconcile_object(current, &declaration),
            None => planner.mount(&declaration),
        };
        planner.retained.root = Some(root);
        let batch = self.take_batch(references);
        self.virtual_refreshes.extend(virtual_refreshes);
        self.apply_batch(batch)
    }

    pub(crate) fn plan_subtree_update(
        &mut self,
        object: ObjectId,
        root: impl Into<View>,
    ) -> NativeBatch {
        self.plan_subtree(object, root.into(), None)
    }

    fn plan_subtree(
        &mut self,
        object: ObjectId,
        declaration: View,
        owner: Option<(ObjectId, RelationId)>,
    ) -> NativeBatch {
        self.assert_ready();
        self.mutations.clear();
        let declaration = declaration.0.object();
        self.validator.validate(&declaration);
        let Some(previous) = self.graph.kind(object) else {
            panic!("stale object {object:?}");
        };
        if previous != declaration.kind {
            let owner = owner.or_else(|| self.graph.owner(object));
            let Some((parent, relation)) = owner else {
                panic!(
                    "root type changed from {previous:?} to {:?}",
                    declaration.kind
                );
            };
            let owns_object =
                self.graph
                    .relation(parent, relation)
                    .is_some_and(|owned| match &owned.value {
                        RetainedRelationValue::One(child) => *child == Some(object),
                        RetainedRelationValue::Many(children) => children.contains(&object),
                    });
            assert!(
                owns_object,
                "{object:?} is not a child of relation {relation:?}"
            );
            let contract = relation_contracts(self.graph.kind(parent).unwrap())
                .iter()
                .find(|contract| contract.id == relation)
                .unwrap();
            assert!(
                contract.realization == Realization::Owned
                    && relation_accepts(contract, declaration.kind),
                "{:?} is not a valid child of relation {relation:?}",
                declaration.kind
            );
        }
        let mut remaining = MAX_OBJECTS;
        if self
            .graph
            .matches_subtree_declaration(object, &declaration, &mut remaining)
        {
            return NativeBatch::default();
        }
        let mut references = Vec::new();
        let mut virtual_refreshes = Vec::new();
        let mut planner = Planner {
            retained: &mut self.graph,
            mutations: &mut self.mutations,
            references: &mut references,
            virtual_refreshes: Some(&mut virtual_refreshes),
        };
        if previous == declaration.kind {
            planner.reconcile_object(object, &declaration);
        } else {
            planner.replace_object(object, &declaration);
        }
        let batch = self.take_batch(references);
        self.virtual_refreshes.extend(virtual_refreshes);
        batch
    }
}
