use std::any::Any;
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::rc::{Rc, Weak};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, Weak as SyncWeak};
use std::time::Duration;

use crate::hooks::{AsyncSetFn, HookCell, RenderScheduler, ResourceTask, SchedulerRef};
use crate::id::NodeId;
use crate::resources::{ContextDefaults, ContextEntry};
use crate::runtime::TimerSpec;

pub(crate) struct AppScheduler {
    work: Rc<RefCell<WorkQueue>>,
    wake: Rc<RefCell<Option<Rc<dyn Fn()>>>>,
    resource_inbox: Arc<Mutex<VecDeque<ResourceCompletion>>>,
    resource_wake: Arc<Mutex<Option<Arc<dyn Fn() + Send + Sync>>>>,
    resource_active: Arc<AtomicBool>,
}

impl AppScheduler {
    pub(crate) fn new(
        work: Rc<RefCell<WorkQueue>>,
        wake: Rc<RefCell<Option<Rc<dyn Fn()>>>>,
        resource_inbox: Arc<Mutex<VecDeque<ResourceCompletion>>>,
        resource_wake: Arc<Mutex<Option<Arc<dyn Fn() + Send + Sync>>>>,
        resource_active: Arc<AtomicBool>,
    ) -> Self {
        Self {
            work,
            wake,
            resource_inbox,
            resource_wake,
            resource_active,
        }
    }

    pub(crate) fn has_effects(&self) -> bool {
        !self.work.borrow().effects.is_empty()
    }

    pub(crate) fn take_effects(&self) -> Vec<Weak<HookCell>> {
        std::mem::take(&mut self.work.borrow_mut().effects)
    }

    pub(crate) fn has_resources(&self) -> bool {
        !self.work.borrow().resources.is_empty()
    }

    pub(crate) fn take_resources(&self) -> Vec<Weak<HookCell>> {
        std::mem::take(&mut self.work.borrow_mut().resources)
    }
}

impl RenderScheduler for AppScheduler {
    fn invalidate(&self, node: NodeId) {
        self.work.borrow_mut().dirty.insert(node);
        if let Some(wake) = self.wake.borrow().as_ref() {
            wake();
        }
    }

    fn activate_window(&self, node: NodeId) {
        self.work.borrow_mut().window_activations.insert(node);
        if let Some(wake) = self.wake.borrow().as_ref() {
            wake();
        }
    }

    fn focus_element(&self, node: NodeId) {
        self.work.borrow_mut().focus_requests.insert(node);
        if let Some(wake) = self.wake.borrow().as_ref() {
            wake();
        }
    }

    fn run_composition_action(&self, node: NodeId, action: crate::composition::CompositionAction) {
        self.work
            .borrow_mut()
            .composition
            .entry(node)
            .or_default()
            .push(action);
        if let Some(wake) = self.wake.borrow().as_ref() {
            wake();
        }
    }

    #[cfg(feature = "canvas")]
    fn invalidate_canvas(&self, node: NodeId, revision: u64) {
        self.work
            .borrow_mut()
            .canvas
            .entry(node)
            .and_modify(|current| *current = (*current).max(revision))
            .or_insert(revision);
        if let Some(wake) = self.wake.borrow().as_ref() {
            wake();
        }
    }

    #[cfg(feature = "canvas")]
    fn run_swap_chain_host_action(&self, node: NodeId, action: crate::canvas::SwapChainHostAction) {
        self.work
            .borrow_mut()
            .swap_chain_hosts
            .entry(node)
            .or_default()
            .push(action);
        if let Some(wake) = self.wake.borrow().as_ref() {
            wake();
        }
    }

    #[cfg(feature = "webview")]
    fn run_webview_action(&self, node: NodeId, action: crate::webview::WebViewAction) {
        self.work
            .borrow_mut()
            .webviews
            .entry(node)
            .or_default()
            .push(action);
        if let Some(wake) = self.wake.borrow().as_ref() {
            wake();
        }
    }

    fn queue_effect(&self, effect: Weak<HookCell>) {
        self.work.borrow_mut().effects.push(effect);
    }

    fn start_timer(
        &self,
        owner: NodeId,
        slot: u32,
        revision: u64,
        interval: Duration,
        repeating: bool,
    ) {
        self.work
            .borrow_mut()
            .timers
            .push(TimerAction::Start(TimerSpec {
                owner,
                slot,
                revision,
                interval,
                repeating,
            }));
    }

    fn stop_timer(&self, owner: NodeId, slot: u32, revision: u64) {
        self.work.borrow_mut().timers.push(TimerAction::Stop {
            owner,
            slot,
            revision,
        });
    }

    fn queue_resource(&self, resource: Weak<HookCell>) {
        self.work.borrow_mut().resources.push(resource);
    }

    fn launch_resource(&self, task: ResourceTask) {
        let inbox = Arc::clone(&self.resource_inbox);
        let wake = self.resource_wake.lock().unwrap().clone();
        let active = Arc::clone(&self.resource_active);
        let worker = move || {
            let result = (task.work)();
            inbox.lock().unwrap().push_back(ResourceCompletion {
                owner: task.owner,
                slot: task.slot,
                revision: Some(task.revision),
                result,
            });
            if active.load(Ordering::Acquire)
                && let Some(wake) = wake
            {
                wake();
            }
        };
        windows_threading::submit(worker);
    }

    fn async_setter(&self, owner: NodeId, slot: u32, live: SyncWeak<()>) -> AsyncSetFn {
        let inbox = Arc::clone(&self.resource_inbox);
        let wake = Arc::clone(&self.resource_wake);
        let active = Arc::clone(&self.resource_active);
        Arc::new(move |result| {
            let Some(_live) = live.upgrade() else {
                return false;
            };
            if !active.load(Ordering::Acquire) {
                return false;
            }
            inbox.lock().unwrap().push_back(ResourceCompletion {
                owner,
                slot,
                revision: None,
                result,
            });
            if let Some(wake) = wake.lock().unwrap().as_ref() {
                wake();
            }
            true
        })
    }
}

pub(crate) enum TimerAction {
    Start(TimerSpec),
    Stop {
        owner: NodeId,
        slot: u32,
        revision: u64,
    },
}

#[derive(Default)]
pub(crate) struct WorkQueue {
    pub dirty: BTreeSet<NodeId>,
    pub window_activations: BTreeSet<NodeId>,
    pub focus_requests: BTreeSet<NodeId>,
    pub composition: BTreeMap<NodeId, Vec<crate::composition::CompositionAction>>,
    #[cfg(feature = "canvas")]
    pub canvas: BTreeMap<NodeId, u64>,
    #[cfg(feature = "canvas")]
    pub swap_chain_hosts: BTreeMap<NodeId, Vec<crate::canvas::SwapChainHostAction>>,
    #[cfg(feature = "webview")]
    pub webviews: BTreeMap<NodeId, Vec<crate::webview::WebViewAction>>,
    effects: Vec<Weak<HookCell>>,
    pub timers: Vec<TimerAction>,
    resources: Vec<Weak<HookCell>>,
}

pub(crate) struct ResourceCompletion {
    pub owner: NodeId,
    pub slot: u32,
    pub revision: Option<u64>,
    pub result: Box<dyn Any + Send>,
}

#[derive(Clone)]
pub(crate) struct RenderServices {
    scheduler: Rc<AppScheduler>,
    pub contexts: Vec<ContextEntry>,
    context_defaults: Rc<ContextDefaults>,
    pub dirty: Rc<BTreeSet<NodeId>>,
    pub window_owner: Option<NodeId>,
}

impl RenderServices {
    pub(crate) fn new(scheduler: Rc<AppScheduler>, context_defaults: Rc<ContextDefaults>) -> Self {
        Self {
            scheduler,
            contexts: Vec::new(),
            context_defaults,
            dirty: Rc::new(BTreeSet::new()),
            window_owner: None,
        }
    }

    pub(crate) fn scheduler(&self) -> SchedulerRef {
        self.scheduler.clone()
    }

    pub(crate) fn context_defaults(&self) -> &ContextDefaults {
        &self.context_defaults
    }

    pub(crate) fn with_context(&self, entry: ContextEntry) -> Self {
        let mut contexts = self.contexts.clone();
        contexts.push(entry);
        Self {
            scheduler: Rc::clone(&self.scheduler),
            contexts,
            context_defaults: Rc::clone(&self.context_defaults),
            dirty: Rc::clone(&self.dirty),
            window_owner: self.window_owner,
        }
    }

    pub(crate) fn with_contexts(&self, contexts: Vec<ContextEntry>) -> Self {
        Self {
            scheduler: Rc::clone(&self.scheduler),
            contexts,
            context_defaults: Rc::clone(&self.context_defaults),
            dirty: Rc::clone(&self.dirty),
            window_owner: self.window_owner,
        }
    }

    pub(crate) fn with_dirty(&self, dirty: Rc<BTreeSet<NodeId>>) -> Self {
        Self {
            scheduler: Rc::clone(&self.scheduler),
            contexts: self.contexts.clone(),
            context_defaults: Rc::clone(&self.context_defaults),
            dirty,
            window_owner: self.window_owner,
        }
    }

    pub(crate) fn with_window_owner(&self, window_owner: Option<NodeId>) -> Self {
        Self {
            scheduler: Rc::clone(&self.scheduler),
            contexts: self.contexts.clone(),
            context_defaults: Rc::clone(&self.context_defaults),
            dirty: Rc::clone(&self.dirty),
            window_owner,
        }
    }
}

pub(crate) fn same_contexts(left: &[ContextEntry], right: &[ContextEntry]) -> bool {
    left.len() == right.len()
        && left
            .iter()
            .zip(right)
            .all(|(left, right)| left.id == right.id && Rc::ptr_eq(&left.value, &right.value))
}
