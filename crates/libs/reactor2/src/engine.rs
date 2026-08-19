use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::rc::Rc;

use crate::arena::{Arena, Node, NodeKind, RealizedRow};
use crate::element::props::{
    AttachedPlacement, CanvasPlacement, GridPlacement, RelativePanelPlacement,
};
use crate::element::tree::StructuralSlot;
use crate::element::{GridLength, Thickness};
use crate::hooks::Cleanup;
use crate::id::NodeId;
use crate::mounted::{Mounted, MountedKind, MountedWindow};
use crate::runtime::*;

mod events;
mod tree;
mod updates;
mod validation;
mod virtualization;

#[cfg(test)]
use performance_support::PerformanceStats;
#[cfg(test)]
use tree::apply_minimal_reorder;

pub trait RowFactory<R: NativeRuntime> {
    fn key(
        &mut self,
        _engine: &Engine<R>,
        _host: NodeId,
        index: usize,
    ) -> Result<u64, EngineError> {
        Ok(index as u64)
    }

    fn mount(
        &mut self,
        engine: &mut Engine<R>,
        host: NodeId,
        index: usize,
    ) -> Result<NodeId, EngineError>;
}

impl<R, F> RowFactory<R> for F
where
    R: NativeRuntime,
    F: FnMut(&mut Engine<R>, NodeId, usize) -> Result<NodeId, EngineError>,
{
    fn mount(
        &mut self,
        engine: &mut Engine<R>,
        host: NodeId,
        index: usize,
    ) -> Result<NodeId, EngineError> {
        self(engine, host, index)
    }
}

fn same_optional_f64(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}

#[derive(Debug)]
pub enum EngineError {
    InvalidNode(NodeId),
    ParentConflict {
        child: NodeId,
        parent: NodeId,
    },
    RowRootAlreadyParented(NodeId),
    VirtualRowNativeRootCount {
        host: NodeId,
        index: usize,
        count: usize,
    },
    AttachedChildNativeRootCount {
        edge: NodeId,
        count: usize,
    },
    DuplicateSiblingKey {
        key: u64,
    },
    InvalidApplicationTree(&'static str),
    WindowContentNativeRootCount {
        window: NodeId,
        count: usize,
    },
    InvalidWindowContent {
        window: NodeId,
        content: NodeId,
    },
    InvalidWindowOwner {
        owner: NodeId,
        child: NodeId,
    },
    IncompatibleEvent {
        target: NodeId,
        event: &'static str,
    },
    TimerOwnerNotComponent(NodeId),
    VirtualHostManaged(NodeId),
    VirtualRowMissing(NodeId),
    NativeParentRejectsChildren(NodeId),
    UnsupportedCommand {
        id: NodeId,
        kind: NativeKind,
        command: &'static str,
    },
}

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNode(id) => write!(f, "unknown or stale node {id:?}"),
            Self::ParentConflict { child, parent } => {
                write!(f, "node {child:?} already has parent {parent:?}")
            }
            Self::RowRootAlreadyParented(id) => {
                write!(f, "realized row root {id:?} already has a parent")
            }
            Self::VirtualRowNativeRootCount { host, index, count } => write!(
                f,
                "virtual row {index} for {host:?} projected {count} native roots instead of one"
            ),
            Self::AttachedChildNativeRootCount { edge, count } => write!(
                f,
                "attached-layout child {edge:?} projected {count} native roots instead of one"
            ),
            Self::DuplicateSiblingKey { key } => write!(f, "duplicate sibling key {key}"),
            Self::InvalidApplicationTree(message) => {
                write!(f, "invalid application tree: {message}")
            }
            Self::WindowContentNativeRootCount { window, count } => write!(
                f,
                "window {window:?} content projected {count} native roots instead of one"
            ),
            Self::InvalidWindowContent { window, content } => write!(
                f,
                "native content {content:?} is not the projected root owned by window {window:?}"
            ),
            Self::InvalidWindowOwner { owner, child } => write!(
                f,
                "window {child:?} is not structurally owned by window {owner:?}"
            ),
            Self::IncompatibleEvent { target, event } => {
                write!(f, "{event} event is incompatible with live node {target:?}")
            }
            Self::TimerOwnerNotComponent(id) => {
                write!(f, "timer owner {id:?} is not a mounted component")
            }
            Self::VirtualHostManaged(id) => {
                write!(
                    f,
                    "virtual host {id:?} accepts children only through realization"
                )
            }
            Self::VirtualRowMissing(id) => {
                write!(f, "virtual row {id:?} is not registered with its host")
            }
            Self::NativeParentRejectsChildren(id) => {
                write!(f, "native node {id:?} does not accept projected children")
            }
            Self::UnsupportedCommand { id, kind, command } => {
                write!(f, "{command} does not support {kind:?} node {id:?}")
            }
        }
    }
}

impl std::error::Error for EngineError {}

pub struct Engine<R> {
    pub(crate) arena: Arena,
    pub(crate) runtime: R,
    pending: Vec<Command>,
    retired: Vec<Mounted>,
    retired_reference_cleanups: Vec<Cleanup>,
    parked_virtual_rows: BTreeMap<NodeId, BTreeMap<u64, NodeId>>,
    virtual_empty: BTreeMap<NodeId, NodeId>,
    references: usize,
    #[cfg(feature = "canvas")]
    committed_canvas_frames: Vec<NodeId>,
    #[cfg(test)]
    performance: PerformanceStats,
}

impl<R: NativeRuntime> Engine<R> {
    pub fn new(runtime: R) -> Self {
        Self {
            arena: Arena::default(),
            runtime,
            pending: Vec::new(),
            retired: Vec::new(),
            retired_reference_cleanups: Vec::new(),
            parked_virtual_rows: BTreeMap::new(),
            virtual_empty: BTreeMap::new(),
            references: 0,
            #[cfg(feature = "canvas")]
            committed_canvas_frames: Vec::new(),
            #[cfg(test)]
            performance: PerformanceStats::default(),
        }
    }

    pub fn contains(&self, id: NodeId) -> bool {
        self.arena.contains(id)
    }

    pub fn parent(&self, id: NodeId) -> Option<NodeId> {
        self.arena.get(id).and_then(|node| node.parent)
    }

    pub fn create_logical(&mut self) -> Result<NodeId, EngineError> {
        Ok(self.arena.insert(Node {
            parent: None,
            children: Vec::new(),
            kind: NodeKind::Logical,
            native_kind: None,
            mounted: None,
        }))
    }

    pub(crate) fn create_structural_slot(
        &mut self,
        slot: StructuralSlot,
    ) -> Result<NodeId, EngineError> {
        Ok(self.arena.insert(Node {
            parent: None,
            children: Vec::new(),
            kind: NodeKind::StructuralSlot(slot),
            native_kind: None,
            mounted: None,
        }))
    }

    pub(crate) fn create_navigation_section(
        &mut self,
        section: NavigationSection,
    ) -> Result<NodeId, EngineError> {
        Ok(self.arena.insert(Node {
            parent: None,
            children: Vec::new(),
            kind: NodeKind::NavigationSection(section),
            native_kind: None,
            mounted: None,
        }))
    }

    pub(crate) fn create_application(&mut self) -> Result<NodeId, EngineError> {
        Ok(self.arena.insert(Node {
            parent: None,
            children: Vec::new(),
            kind: NodeKind::Application,
            native_kind: None,
            mounted: None,
        }))
    }

    pub(crate) fn update_application(
        &mut self,
        id: NodeId,
        update: ApplicationUpdate,
    ) -> Result<(), EngineError> {
        let node = self.arena.get(id).ok_or(EngineError::InvalidNode(id))?;
        if !matches!(node.kind, NodeKind::Application) {
            return Err(EngineError::InvalidNode(id));
        }
        self.pending.push(Command::UpdateApplication { id, update });
        Ok(())
    }

    pub(crate) fn create_window(&mut self, create: WindowCreate) -> Result<NodeId, EngineError> {
        let id = self.arena.insert(Node {
            parent: None,
            children: Vec::new(),
            kind: NodeKind::Window,
            native_kind: None,
            mounted: None,
        });
        self.pending.push(Command::CreateWindow { id, create });
        Ok(id)
    }

    pub(crate) fn set_window_content(
        &mut self,
        window: NodeId,
        content: NodeId,
    ) -> Result<(), EngineError> {
        let node = self
            .arena
            .get(window)
            .ok_or(EngineError::InvalidNode(window))?;
        if !matches!(node.kind, NodeKind::Window) || !self.arena.contains(content) {
            return Err(EngineError::InvalidNode(window));
        }
        if node
            .children
            .first()
            .and_then(|branch| self.single_projected_native_root(*branch))
            != Some(content)
        {
            return Err(EngineError::InvalidWindowContent { window, content });
        }
        self.pending
            .push(Command::SetWindowContent { window, content });
        Ok(())
    }

    pub(crate) fn set_window_owner(
        &mut self,
        owner: NodeId,
        child: NodeId,
    ) -> Result<(), EngineError> {
        if !self
            .arena
            .get(owner)
            .is_some_and(|node| matches!(node.kind, NodeKind::Window))
            || !self
                .arena
                .get(child)
                .is_some_and(|node| matches!(node.kind, NodeKind::Window))
        {
            return Err(EngineError::InvalidNode(child));
        }
        self.pending.push(Command::SetWindowOwner { owner, child });
        Ok(())
    }

    pub(crate) fn update_window(
        &mut self,
        id: NodeId,
        update: WindowUpdate,
    ) -> Result<(), EngineError> {
        let node = self.arena.get(id).ok_or(EngineError::InvalidNode(id))?;
        if !matches!(node.kind, NodeKind::Window) {
            return Err(EngineError::InvalidNode(id));
        }
        self.pending.push(Command::UpdateWindow { id, update });
        Ok(())
    }

    pub(crate) fn activate_window(&mut self, id: NodeId) -> Result<(), EngineError> {
        let node = self.arena.get(id).ok_or(EngineError::InvalidNode(id))?;
        if !matches!(node.kind, NodeKind::Window) {
            return Err(EngineError::InvalidNode(id));
        }
        self.pending.push(Command::ActivateWindow { id });
        Ok(())
    }

    pub(crate) fn focus_element(&mut self, id: NodeId) -> Result<(), EngineError> {
        let node = self.arena.get(id).ok_or(EngineError::InvalidNode(id))?;
        if !matches!(node.kind, NodeKind::Native) {
            return Err(EngineError::InvalidNode(id));
        }
        self.pending.push(Command::FocusElement { id });
        Ok(())
    }

    pub(crate) fn create_command_section(
        &mut self,
        section: CommandSection,
    ) -> Result<NodeId, EngineError> {
        Ok(self.arena.insert(Node {
            parent: None,
            children: Vec::new(),
            kind: NodeKind::CommandSection(section),
            native_kind: None,
            mounted: None,
        }))
    }

    pub fn create_native(&mut self, kind: NativeKind) -> Result<NodeId, EngineError> {
        self.create_native_node(kind, NodeKind::Native)
    }

    pub(crate) fn create_owned_native(&mut self, kind: NativeKind) -> Result<NodeId, EngineError> {
        self.create_native_node(kind, NodeKind::OwnedNative)
    }

    pub(crate) fn create_owner_bound(
        &mut self,
        owner: NodeId,
        accessory: NodeId,
        relation: OwnerRelation,
        project_accessory: bool,
    ) -> Result<NodeId, EngineError> {
        if self.arena.get(owner).is_none() || self.arena.get(accessory).is_none() {
            return Err(EngineError::InvalidNode(owner));
        }
        if self.arena.get(owner).unwrap().parent.is_some()
            || self.arena.get(accessory).unwrap().parent.is_some()
        {
            return Err(EngineError::ParentConflict {
                child: accessory,
                parent: owner,
            });
        }
        let owner_root = self
            .single_projected_native_root(owner)
            .ok_or(EngineError::NativeParentRejectsChildren(owner))?;
        let accessory_root = self
            .single_projected_native_root(accessory)
            .ok_or(EngineError::NativeParentRejectsChildren(accessory))?;
        let id = self.arena.insert(Node {
            parent: None,
            children: vec![owner, accessory],
            kind: NodeKind::OwnerBound {
                relation,
                project_accessory,
            },
            native_kind: None,
            mounted: None,
        });
        self.arena.get_mut(owner).unwrap().parent = Some(id);
        self.arena.get_mut(accessory).unwrap().parent = Some(id);
        self.pending.push(Command::BindOwner {
            owner: owner_root,
            accessory: accessory_root,
            relation,
        });
        Ok(id)
    }

    pub(crate) fn reattach_owner_bound(&mut self, id: NodeId) -> Result<(), EngineError> {
        let node = self.arena.get(id).ok_or(EngineError::InvalidNode(id))?;
        let NodeKind::OwnerBound { relation, .. } = node.kind else {
            return Err(EngineError::InvalidNode(id));
        };
        if node.children.len() != 2 {
            return Err(EngineError::InvalidNode(id));
        }
        let owner = self
            .single_projected_native_root(node.children[0])
            .ok_or(EngineError::NativeParentRejectsChildren(id))?;
        let accessory = self
            .single_projected_native_root(node.children[1])
            .ok_or(EngineError::NativeParentRejectsChildren(id))?;
        self.pending.push(Command::BindOwner {
            owner,
            accessory,
            relation,
        });
        Ok(())
    }

    pub fn create_virtual_host(&mut self, kind: NativeKind) -> Result<NodeId, EngineError> {
        self.create_native_node(
            kind,
            NodeKind::VirtualHost {
                realized: Default::default(),
            },
        )
    }

    pub fn commit(&mut self) -> Result<(), EngineError> {
        if self.pending.is_empty() {
            return Ok(());
        }
        for command in &self.pending {
            if let Command::SetWindowOwner { owner, child } = *command
                && !self.valid_window_owner(owner, child)
            {
                return Err(EngineError::InvalidWindowOwner { owner, child });
            }
        }
        let commands = std::mem::take(&mut self.pending);
        #[cfg(feature = "canvas")]
        let canvas_frames = commands.iter().filter_map(|command| match command {
            Command::RunCanvasImageFrame { target } | Command::RunCanvasFrame { target } => {
                Some(*target)
            }
            _ => None,
        });
        self.runtime.apply(&commands);
        #[cfg(feature = "canvas")]
        for target in canvas_frames {
            if !self.committed_canvas_frames.contains(&target) {
                self.committed_canvas_frames.push(target);
            }
        }
        Ok(())
    }

    pub(crate) fn shutdown(&mut self) {
        let needs_retirement = self
            .arena
            .nodes()
            .any(|node| node.mounted.as_ref().is_some_and(Mounted::needs_retirement));
        if needs_retirement {
            loop {
                let root = {
                    let mut ids = self.arena.ids();
                    ids.find(|id| self.arena.get(*id).unwrap().parent.is_none())
                };
                let Some(root) = root else {
                    break;
                };
                self.retire_subtree(root);
            }
        }
        self.arena = Arena::default();
        self.parked_virtual_rows.clear();
        self.virtual_empty.clear();
        self.pending.clear();
        #[cfg(feature = "canvas")]
        self.committed_canvas_frames.clear();
    }

    pub(crate) fn take_retired(&mut self) -> (Vec<Cleanup>, Vec<Mounted>) {
        (
            std::mem::take(&mut self.retired_reference_cleanups),
            std::mem::take(&mut self.retired),
        )
    }

    pub(crate) fn has_retired(&self) -> bool {
        !self.retired_reference_cleanups.is_empty() || !self.retired.is_empty()
    }

    pub(crate) fn retire_cleanup(&mut self, cleanup: Cleanup) {
        self.retired_reference_cleanups.push(cleanup);
    }

    pub(crate) fn add_reference(&mut self) {
        self.references += 1;
    }

    pub(crate) fn remove_reference(&mut self) {
        self.references -= 1;
    }

    pub(crate) fn has_references(&self) -> bool {
        self.references != 0
    }

    pub(crate) fn set_event_waker(&mut self, waker: Option<Rc<dyn Fn()>>) {
        self.runtime.set_event_waker(waker);
    }
}

#[cfg(test)]
#[path = "../testing/private/engine_performance.rs"]
mod performance_support;

#[cfg(test)]
#[path = "../testing/private/engine.rs"]
mod tests;
