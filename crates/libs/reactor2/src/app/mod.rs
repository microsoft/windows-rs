mod auto_suggest_box;
mod breadcrumb_bar;
mod collection;
mod command;
mod container;
mod content;
mod events;
mod logical;
mod media;
mod menu;
mod mount;
mod native_host;
mod navigation;
mod overlay;
mod properties;
mod reconcile;
mod selector;
mod selector_bar;
mod shape;
mod status;
mod text;
mod value;
mod window;
mod work;

#[cfg(test)]
#[path = "../../testing/private/app_performance.rs"]
mod performance_support;

use std::any::Any;
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
#[cfg(test)]
use std::time::Duration;
#[cfg(test)]
use std::time::Instant;

use self::events::dispatch_native_event;
use self::logical::complete_fade_transition;
use self::mount::mount_element;
use self::reconcile::rerender_component;
use self::work::*;
use crate::arena::NodeKind;
use crate::element::Element;
use crate::element::props::*;
use crate::engine::{Engine, EngineError, RowFactory};
use crate::hooks::Cleanup;
use crate::id::NodeId;
use crate::mounted::{Mounted, MountedKind};
use crate::resources::{ContextDefaults, ContextEntry};
use crate::runtime::*;

pub(crate) fn match_virtual_rows(
    items: &VirtualCollectionItems,
    rows: impl IntoIterator<Item = (u64, NodeId)>,
) -> (Vec<(usize, u64, NodeId)>, Vec<NodeId>) {
    let mut rows = rows.into_iter().collect::<BTreeMap<_, _>>();
    let mut matched = Vec::new();
    match items {
        VirtualCollectionItems::Implicit(count) => {
            for index in 0..*count {
                let key = index as u64;
                if let Some(root) = rows.remove(&key) {
                    matched.push((index, key, root));
                }
            }
        }
        VirtualCollectionItems::Keyed(keys) => {
            for (index, key) in keys.as_slice().iter().copied().enumerate() {
                if let Some(root) = rows.remove(&key) {
                    matched.push((index, key, root));
                }
            }
        }
    }
    (matched, rows.into_values().collect())
}

pub struct Reactor<R: NativeRuntime> {
    pub(crate) engine: Engine<R>,
    root: Option<NodeId>,
    validated_arena_revision: Option<u64>,
    initial: Option<Element>,
    work: Rc<RefCell<WorkQueue>>,
    wake: Rc<RefCell<Option<Rc<dyn Fn()>>>>,
    resource_inbox: Arc<Mutex<VecDeque<ResourceCompletion>>>,
    resource_wake: Arc<Mutex<Option<Arc<dyn Fn() + Send + Sync>>>>,
    resource_active: Arc<AtomicBool>,
    context_defaults: Rc<ContextDefaults>,
    #[cfg(feature = "canvas")]
    committed_canvas_frames: Vec<NodeId>,
    #[cfg(test)]
    render_complete: Option<Rc<dyn Fn(&crate::performance::RenderMetrics)>>,
}

impl<R: NativeRuntime> Drop for Reactor<R> {
    fn drop(&mut self) {
        self.resource_active.store(false, Ordering::Release);
        self.engine.shutdown();
        if let Some(payload) = run_retired_cleanups(self.engine.take_retired())
            && !std::thread::panicking()
        {
            resume_unwind(payload);
        }
    }
}

impl<R: NativeRuntime> Reactor<R> {
    pub fn new(runtime: R, root: Element) -> Self {
        Self {
            engine: Engine::new(runtime),
            root: None,
            validated_arena_revision: None,
            initial: Some(root),
            work: Rc::new(RefCell::new(WorkQueue::default())),
            wake: Rc::new(RefCell::new(None)),
            resource_inbox: Arc::new(Mutex::new(VecDeque::new())),
            resource_wake: Arc::new(Mutex::new(None)),
            resource_active: Arc::new(AtomicBool::new(true)),
            context_defaults: Rc::new(ContextDefaults::default()),
            #[cfg(feature = "canvas")]
            committed_canvas_frames: Vec::new(),
            #[cfg(test)]
            render_complete: None,
        }
    }

    pub fn set_waker(&mut self, wake: Rc<dyn Fn()>) {
        *self.wake.borrow_mut() = Some(Rc::clone(&wake));
        self.engine.set_event_waker(Some(wake));
    }

    pub(crate) fn set_resource_waker(&mut self, wake: Arc<dyn Fn() + Send + Sync>) {
        *self.resource_wake.lock().unwrap() = Some(wake);
    }

    pub(crate) fn wake_pending_canvas_work(&self) {
        #[cfg(feature = "canvas")]
        {
            if !self.work.borrow().canvas.is_empty()
                && let Some(wake) = self.wake.borrow().as_ref()
            {
                wake();
            }
        }
    }

    pub fn pump(&mut self) {
        self.pump_inner().unwrap();
    }

    fn pump_inner(&mut self) -> Result<(), EngineError> {
        let scheduler = Rc::new(AppScheduler::new(
            Rc::clone(&self.work),
            Rc::clone(&self.wake),
            Arc::clone(&self.resource_inbox),
            Arc::clone(&self.resource_wake),
            Arc::clone(&self.resource_active),
        ));
        let services =
            RenderServices::new(Rc::clone(&scheduler), Rc::clone(&self.context_defaults));
        #[cfg(feature = "canvas")]
        self.committed_canvas_frames.clear();
        loop {
            let completions = {
                let mut inbox = self.resource_inbox.lock().unwrap();
                inbox.drain(..).collect::<Vec<_>>()
            };
            for completion in completions {
                let accepted = self
                    .engine
                    .arena
                    .get(completion.owner)
                    .and_then(|node| node.mounted.as_ref())
                    .and_then(|mounted| match &mounted.kind {
                        MountedKind::Component { hooks, .. } => hooks.get(completion.slot as usize),
                        _ => None,
                    })
                    .is_some_and(|hook| match completion.revision {
                        Some(revision) => hook.accept_resource(revision, completion.result),
                        None => hook.accept_async_state(completion.result),
                    });
                if accepted {
                    self.work.borrow_mut().dirty.insert(completion.owner);
                }
            }
            let timers = std::mem::take(&mut self.work.borrow_mut().timers);
            for timer in timers {
                match timer {
                    TimerAction::Start(spec) => self.engine.start_timer(spec)?,
                    TimerAction::Stop {
                        owner,
                        slot,
                        revision,
                    } => self.engine.stop_timer(owner, slot, revision),
                }
            }
            let activations = std::mem::take(&mut self.work.borrow_mut().window_activations);
            for id in activations {
                if self.engine.contains(id)
                    && matches!(self.engine.arena.get(id).unwrap().kind, NodeKind::Window)
                {
                    self.engine.activate_window(id)?;
                }
            }
            let focus_requests = std::mem::take(&mut self.work.borrow_mut().focus_requests);
            for id in focus_requests {
                if self.engine.contains(id) {
                    self.engine.focus_element(id)?;
                }
            }
            let composition = std::mem::take(&mut self.work.borrow_mut().composition);
            for (id, actions) in composition {
                if self.engine.contains(id) {
                    for action in actions {
                        self.engine.run_composition_action(id, action)?;
                    }
                }
            }
            #[cfg(feature = "canvas")]
            {
                let pending = std::mem::take(&mut self.work.borrow_mut().canvas);
                let mut deferred = BTreeMap::new();
                for (id, revision) in pending {
                    if self.committed_canvas_frames.contains(&id) {
                        deferred.insert(id, revision);
                    } else if self.engine.contains(id) {
                        self.engine.invalidate_canvas(id, revision)?;
                    }
                }
                let mut work = self.work.borrow_mut();
                for (id, revision) in deferred {
                    work.canvas
                        .entry(id)
                        .and_modify(|current| *current = (*current).max(revision))
                        .or_insert(revision);
                }
                drop(work);
                let hosts = std::mem::take(&mut self.work.borrow_mut().swap_chain_hosts);
                for (id, actions) in hosts {
                    if self.engine.contains(id) {
                        for action in actions {
                            self.engine.run_swap_chain_host_action(id, action)?;
                        }
                    }
                }
            }
            #[cfg(feature = "webview")]
            {
                let webviews = std::mem::take(&mut self.work.borrow_mut().webviews);
                for (id, actions) in webviews {
                    if self.engine.contains(id) {
                        for action in actions {
                            self.engine.run_webview_action(id, action)?;
                        }
                    }
                }
            }
            #[cfg(test)]
            self.engine.begin_performance_pass();
            #[cfg(test)]
            let pass_started = Instant::now();
            let rendered = if let Some(element) = self.initial.take() {
                let root = mount_element(&mut self.engine, element, &services)?;
                self.root = Some(root);
                self.validate_root()?;
                self.engine.commit()?;
                #[cfg(feature = "canvas")]
                self.engine
                    .drain_committed_canvas_frames(&mut self.committed_canvas_frames);
                true
            } else {
                let pending = std::mem::take(&mut self.work.borrow_mut().dirty);
                let rendered = !pending.is_empty();
                let dirty = dirty_roots(&self.engine, &pending);
                let render_services = services.with_dirty(Rc::new(pending));
                for id in dirty {
                    if self.engine.contains(id) {
                        rerender_component(&mut self.engine, id, &render_services)?;
                    }
                }
                self.validate_root()?;
                self.engine.commit()?;
                #[cfg(feature = "canvas")]
                self.engine
                    .drain_committed_canvas_frames(&mut self.committed_canvas_frames);
                rendered
            };
            #[cfg(test)]
            let before_effects = pass_started.elapsed();
            #[cfg(test)]
            let effects_started = Instant::now();
            self.finish_commit(&scheduler);
            #[cfg(not(test))]
            let _ = rendered;
            #[cfg(test)]
            if rendered {
                self.finish_performance_pass(before_effects, effects_started.elapsed());
            }

            let mut rows = AppRowFactory {
                services: services.clone(),
            };
            let (events, had_events) = self.engine.process_events(&mut rows)?;
            #[cfg(feature = "canvas")]
            self.engine
                .drain_committed_canvas_frames(&mut self.committed_canvas_frames);
            self.finish_commit(&scheduler);
            for event in events {
                let completed = match &event {
                    NativeEvent::TimerFired {
                        owner,
                        slot: FADE_TRANSITION_TIMER_SLOT,
                        revision,
                    } => complete_fade_transition(&mut self.engine, *owner, *revision)?,
                    _ => false,
                };
                if completed {
                    self.engine.commit()?;
                    self.finish_commit(&scheduler);
                } else {
                    dispatch_native_event(&self.engine, event);
                }
            }
            if self.work.borrow().dirty.is_empty()
                && self.work.borrow().timers.is_empty()
                && {
                    #[cfg(feature = "canvas")]
                    {
                        self.work
                            .borrow()
                            .canvas
                            .keys()
                            .all(|id| self.committed_canvas_frames.contains(id))
                    }
                    #[cfg(not(feature = "canvas"))]
                    {
                        true
                    }
                }
                && !had_events
            {
                break;
            }
        }
        Ok(())
    }

    fn finish_commit(&mut self, scheduler: &AppScheduler) {
        if !self.engine.has_retired()
            && !self.engine.has_references()
            && !scheduler.has_effects()
            && !scheduler.has_resources()
        {
            return;
        }

        if let Some(payload) = run_retired_cleanups(self.engine.take_retired()) {
            resume_unwind(payload);
        }
        commit_element_references(&self.engine, self.root.unwrap());
        let effects = scheduler.take_effects();
        for effect in effects {
            if let Some(effect) = effect.upgrade() {
                effect.commit_effect();
            }
        }
        let resources = scheduler.take_resources();
        for resource in resources {
            if let Some(resource) = resource.upgrade() {
                resource.commit_resource();
            }
        }
    }

    fn validate_root(&mut self) -> Result<(), EngineError> {
        let revision = self.engine.arena.revision();
        if self.validated_arena_revision == Some(revision) {
            return Ok(());
        }

        let root = self
            .root
            .expect("the application root must project one native node");
        if let Some(windows) = self.engine.validate_application_root(root)? {
            self.engine.sync_window_content_roots(&windows)?;
            self.validated_arena_revision = Some(revision);
            return Ok(());
        }
        self.engine
            .single_projected_native_root(root)
            .expect("the application root must project one native node");
        self.validated_arena_revision = Some(revision);
        Ok(())
    }
}

pub(crate) fn set_virtual_collection_items<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    items: &VirtualCollectionItems,
) -> Result<(), EngineError> {
    match items {
        VirtualCollectionItems::Implicit(count) => engine.set_virtual_item_count(id, *count),
        VirtualCollectionItems::Keyed(keys) => engine.set_virtual_item_keys(id, keys.values()),
    }
}

pub(crate) fn contexts_for_node<R: NativeRuntime>(
    engine: &Engine<R>,
    id: NodeId,
) -> Vec<ContextEntry> {
    let mut contexts = Vec::new();
    let mut parent = engine.parent(id);
    while let Some(id) = parent {
        let node = engine.arena.get(id).unwrap();
        if let Some(Mounted {
            kind: MountedKind::Context(props),
            ..
        }) = &node.mounted
        {
            contexts.push(props.entry.clone());
        }
        parent = node.parent;
    }
    contexts.reverse();
    contexts
}

pub(crate) fn window_owner_for_node<R: NativeRuntime>(
    engine: &Engine<R>,
    id: NodeId,
) -> Option<NodeId> {
    let mut child = id;
    let mut parent = engine.parent(id);
    while let Some(id) = parent {
        let node = engine.arena.get(id).unwrap();
        if matches!(node.kind, NodeKind::Window) {
            return node
                .children
                .iter()
                .position(|current| *current == child)
                .is_some_and(|index| index != 0)
                .then_some(id);
        }
        child = id;
        parent = node.parent;
    }
    None
}

fn commit_element_references<R: NativeRuntime>(engine: &Engine<R>, id: NodeId) {
    let node = engine.arena.get(id).unwrap();
    if let Some(Mounted {
        kind: MountedKind::Reference { reference, target },
        ..
    }) = &node.mounted
    {
        reference.commit(*target);
    }
    if let Some(Mounted {
        kind: MountedKind::Window(window),
        ..
    }) = &node.mounted
        && let Some(reference) = &window.props.reference
    {
        reference.commit(id);
    }
    for child in &node.children {
        commit_element_references(engine, *child);
    }
}

fn run_retired_cleanups(
    (mut cleanups, retired): (Vec<Cleanup>, Vec<Mounted>),
) -> Option<Box<dyn Any + Send>> {
    for mounted in retired {
        if let MountedKind::Component { hooks, .. } = mounted.kind {
            for hook in hooks {
                if let Some(cleanup) = hook.take_effect_cleanup() {
                    cleanups.push(cleanup);
                }
                if let Some(cleanup) = hook.take_resource_cleanup() {
                    cleanups.push(cleanup);
                }
            }
        }
    }
    run_cleanups(cleanups)
}

fn run_cleanups(cleanups: Vec<Cleanup>) -> Option<Box<dyn Any + Send>> {
    let mut first = None;
    for cleanup in cleanups {
        if let Err(payload) = catch_unwind(AssertUnwindSafe(cleanup))
            && first.is_none()
        {
            first = Some(payload);
        }
    }
    first
}

fn dirty_roots<R: NativeRuntime>(engine: &Engine<R>, pending: &BTreeSet<NodeId>) -> Vec<NodeId> {
    pending
        .iter()
        .copied()
        .filter(|id| {
            let mut parent = engine.parent(*id);
            while let Some(id) = parent {
                if pending.contains(&id) {
                    return false;
                }
                parent = engine.parent(id);
            }
            true
        })
        .collect()
}

struct AppRowFactory {
    services: RenderServices,
}

impl<R: NativeRuntime> RowFactory<R> for AppRowFactory {
    fn key(&mut self, engine: &Engine<R>, host: NodeId, index: usize) -> Result<u64, EngineError> {
        let node = engine
            .arena
            .get(host)
            .ok_or(EngineError::InvalidNode(host))?;
        let Some(Mounted {
            kind: MountedKind::VirtualCollection(props),
            ..
        }) = &node.mounted
        else {
            return Err(EngineError::InvalidNode(host));
        };
        props.items.key(index).ok_or(EngineError::InvalidNode(host))
    }

    fn mount(
        &mut self,
        engine: &mut Engine<R>,
        host: NodeId,
        index: usize,
    ) -> Result<NodeId, EngineError> {
        let row = {
            let node = engine
                .arena
                .get(host)
                .ok_or(EngineError::InvalidNode(host))?;
            let Some(Mounted {
                kind: MountedKind::VirtualCollection(props),
                ..
            }) = &node.mounted
            else {
                return Err(EngineError::InvalidNode(host));
            };
            Rc::clone(&props.row)
        };
        let services = self.services.with_contexts(contexts_for_node(engine, host));
        mount_element(engine, row(index), &services)
    }
}
