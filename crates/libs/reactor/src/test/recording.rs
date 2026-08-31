use std::collections::{BTreeMap, HashMap, HashSet};

use crate::core::*;
use crate::*;

#[derive(Debug)]
pub struct RecordedNode {
    kind: Option<MountedKind>,
    parent: Option<NodeId>,
    children: Vec<NodeId>,
    slots: BTreeMap<SlotId, NodeId>,
    slot_collections: BTreeMap<SlotId, Vec<NodeId>>,
    properties: BTreeMap<PropertyId, PropertyValue>,
    theme_style: ThemeStyle,
}

struct RecordedRetainedSubtree {
    nodes: Vec<NodeId>,
    parent: NodeId,
    slot: Option<SlotId>,
}

pub struct RecordingRuntime {
    application: Option<NodeId>,
    attachments: HashMap<(NodeId, RealizedContainer), NodeId>,
    nodes: HashMap<NodeId, RecordedNode>,
    opened_windows: Vec<View>,
    batches: usize,
    commands: Vec<Vec<Command>>,
    close_requests: Vec<NodeId>,
    content_dialogs: HashMap<NodeId, RecordedContentDialog>,
    content_dialog_request_order: u64,
    events: Vec<NativeWork<QueuedEvent>>,
    host_events: Vec<NativeWork<HostEvent>>,
    record_commands: bool,
    fail_at: HashSet<(usize, usize)>,
    fail_window_open: bool,
    identity: Option<WindowToken>,
    realizations: Vec<NativeWork<RealizationRequest>>,
    retained_subtrees: HashMap<NodeId, RecordedRetainedSubtree>,
    source_revisions: HashMap<NodeId, u64>,
    subscriptions: HashSet<(NodeId, EventId)>,
    tooltips: HashMap<NodeId, (NodeId, TooltipPlacement)>,
    flyouts: HashMap<NodeId, (NodeId, FlyoutPlacement)>,
    owned_menus: HashMap<NodeId, (NodeId, OwnedMenuKind, Vec<MenuItem>, u32)>,
    command_bar_flyouts:
        HashMap<NodeId, (NodeId, Vec<CommandBarCommand>, Vec<CommandBarCommand>, u32)>,
    tree_nodes: HashMap<NodeId, Vec<TreeNode>>,
    window_titles: HashMap<NodeId, String>,
    window_title_bars: HashMap<NodeId, (NodeId, WindowTitleBarHeight)>,
    window_observations: HashMap<NodeId, WindowObservationFlags>,
    window_visuals: HashMap<NodeId, WindowVisuals>,
    windows: HashSet<NodeId>,
}

impl Default for RecordingRuntime {
    fn default() -> Self {
        Self {
            application: None,
            attachments: HashMap::new(),
            nodes: HashMap::new(),
            opened_windows: Vec::new(),
            batches: 0,
            commands: Vec::new(),
            close_requests: Vec::new(),
            content_dialogs: HashMap::new(),
            content_dialog_request_order: 0,
            events: Vec::new(),
            host_events: Vec::new(),
            record_commands: true,
            fail_at: HashSet::new(),
            fail_window_open: false,
            identity: None,
            realizations: Vec::new(),
            retained_subtrees: HashMap::new(),
            source_revisions: HashMap::new(),
            subscriptions: HashSet::new(),
            tooltips: HashMap::new(),
            flyouts: HashMap::new(),
            owned_menus: HashMap::new(),
            command_bar_flyouts: HashMap::new(),
            tree_nodes: HashMap::new(),
            window_titles: HashMap::new(),
            window_title_bars: HashMap::new(),
            window_observations: HashMap::new(),
            window_visuals: HashMap::new(),
            windows: HashSet::new(),
        }
    }
}

impl RecordingRuntime {
    #[cfg(any(test, feature = "test"))]
    pub fn retained_subtrees(&self) -> usize {
        self.retained_subtrees.len()
    }

    #[cfg(any(test, feature = "test"))]
    pub fn complete_retirement(&mut self, root: NodeId) -> bool {
        let Some(retained) = self.retained_subtrees.remove(&root) else {
            return false;
        };
        let children = match retained.slot {
            Some(slot) => self
                .nodes
                .get_mut(&retained.parent)
                .and_then(|parent| parent.slot_collections.get_mut(&slot)),
            None => self
                .nodes
                .get_mut(&retained.parent)
                .map(|parent| &mut parent.children),
        };
        if let Some(children) = children {
            children.retain(|child| *child != root);
        }
        for node in retained.nodes {
            self.nodes.remove(&node);
            self.tree_nodes.remove(&node);
            self.source_revisions.remove(&node);
            self.subscriptions
                .retain(|(subscription_node, _)| *subscription_node != node);
        }
        true
    }

    fn is_tooltip_child(&self, node: NodeId) -> bool {
        self.tooltips.values().any(|(tooltip, _)| *tooltip == node)
    }

    fn is_flyout_child(&self, node: NodeId) -> bool {
        self.flyouts.values().any(|(content, _)| *content == node)
    }

    fn is_owned_attachment_child(&self, node: NodeId) -> bool {
        self.is_tooltip_child(node) || self.is_flyout_child(node)
    }

    pub fn record_commands(&mut self, record: bool) {
        self.record_commands = record;
        if !record {
            self.commands.clear();
        }
    }

    pub fn fail_at(&mut self, command_index: usize) {
        self.fail_after(0, command_index);
    }

    pub fn fail_after(&mut self, batches: usize, command_index: usize) {
        self.fail_at
            .insert((self.batches + batches + 1, command_index));
    }

    #[cfg(any(test, feature = "test"))]
    pub fn queue_host_event(&mut self, event: HostEvent) {
        self.host_events.push(NativeWork {
            identity: self.identity.unwrap(),
            work: event,
        });
    }

    #[cfg(any(test, feature = "test"))]
    pub fn fail_window_open(&mut self) {
        self.fail_window_open = true;
    }

    pub fn node(&self, id: NodeId) -> Option<&RecordedNode> {
        self.nodes.get(&id)
    }

    #[cfg(any(test, feature = "test"))]
    pub fn record_property_observation(
        &mut self,
        node: NodeId,
        property: PropertyId,
        value: PropertyValue,
    ) -> Result<(), RuntimeError> {
        self.nodes
            .get_mut(&node)
            .ok_or(RuntimeError::MissingNode(node))?
            .properties
            .insert(property, value);
        Ok(())
    }

    pub fn batches(&self) -> usize {
        self.batches
    }

    pub fn commands(&self) -> &[Vec<Command>] {
        &self.commands
    }

    #[cfg(any(test, feature = "test"))]
    pub fn close_requests(&self) -> &[NodeId] {
        &self.close_requests
    }

    #[cfg(any(test, feature = "test"))]
    pub fn window_title(&self, node: NodeId) -> Option<&str> {
        self.window_titles.get(&node).map(String::as_str)
    }

    #[cfg(any(test, feature = "test"))]
    pub fn window_title_bar(&self, node: NodeId) -> Option<(NodeId, WindowTitleBarHeight)> {
        self.window_title_bars.get(&node).copied()
    }

    #[cfg(any(test, feature = "test"))]
    pub fn window_visuals(&self, node: NodeId) -> Option<WindowVisuals> {
        self.window_visuals.get(&node).copied()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    #[cfg(any(test, feature = "test"))]
    pub fn opened_windows(&self) -> &[View] {
        &self.opened_windows
    }

    pub fn tooltip(&self, target: NodeId) -> Option<(NodeId, TooltipPlacement)> {
        self.tooltips.get(&target).copied()
    }

    pub fn flyout(&self, target: NodeId) -> Option<(NodeId, FlyoutPlacement)> {
        self.flyouts.get(&target).copied()
    }

    pub fn owned_menu(
        &self,
        owner: NodeId,
    ) -> Option<&(NodeId, OwnedMenuKind, Vec<MenuItem>, u32)> {
        self.owned_menus.get(&owner)
    }

    pub fn command_bar_flyout(
        &self,
        owner: NodeId,
    ) -> Option<&(NodeId, Vec<CommandBarCommand>, Vec<CommandBarCommand>, u32)> {
        self.command_bar_flyouts.get(&owner)
    }

    pub fn tree_nodes(&self, target: NodeId) -> Option<&[TreeNode]> {
        self.tree_nodes.get(&target).map(Vec::as_slice)
    }

    pub fn queue_owned_click(&mut self, owner: NodeId, label: impl Into<String>) {
        let (event, revision) = if let Some((_, _, _, revision)) = self.owned_menus.get(&owner) {
            (EventId::OwnedMenuItemInvoked, *revision)
        } else {
            let (_, _, _, revision) = self.command_bar_flyouts[&owner];
            (EventId::OwnedCommandInvoked, revision)
        };
        self.events.push(NativeWork {
            identity: self.identity.unwrap(),
            work: QueuedEvent::new(owner, event, revision, EventPayload::String(label.into())),
        });
    }

    pub fn content_dialog(&self, node: NodeId) -> Option<RecordedContentDialog> {
        self.content_dialogs.get(&node).copied()
    }

    pub fn complete_content_dialog(
        &mut self,
        node: NodeId,
        revision: u32,
        result: ContentDialogResult,
    ) {
        let Some(state) = self.content_dialogs.get_mut(&node) else {
            return;
        };
        if !state.pending {
            return;
        }
        let invoke_callback = !state.suppress_callback;
        let retired = state.retired;
        state.pending = false;
        state.suppress_callback = false;

        let candidate = self
            .content_dialogs
            .iter()
            .filter(|(_, state)| state.queued && state.desired_open && !state.retired)
            .min_by_key(|(_, state)| state.request_order)
            .map(|(node, _)| *node);
        if let Some(candidate) = candidate {
            let state = self.content_dialogs.get_mut(&candidate).unwrap();
            state.queued = false;
            state.pending = true;
            state.show_count += 1;
        }
        if retired {
            self.content_dialogs.remove(&node);
        }

        let event = if invoke_callback {
            QueuedEvent::new(
                node,
                EventId::ContentDialogClosed,
                revision,
                EventPayload::ContentDialogResult(result),
            )
        } else {
            QueuedEvent::observation(
                node,
                EventId::ContentDialogClosed,
                revision,
                EventPayload::ContentDialogResult(result),
            )
        };
        self.events.push(NativeWork {
            identity: self.identity.unwrap(),
            work: event,
        });
    }

    pub fn queue_realization(&mut self, request: RealizationRequest) {
        self.realizations.push(NativeWork {
            identity: self.identity.unwrap(),
            work: request,
        });
    }

    pub fn queue_realize(
        &mut self,
        collection: NodeId,
        container: RealizedContainer,
        index: usize,
    ) {
        self.queue_realization(RealizationRequest::Realize {
            collection,
            container,
            index,
            source_revision: self.source_revisions[&collection],
        });
    }

    pub fn queue_recycle(&mut self, collection: NodeId, container: RealizedContainer) {
        self.queue_realization(RealizationRequest::Recycle {
            collection,
            container,
            source_revision: self.source_revisions[&collection],
        });
    }

    pub fn source_revision(&self, collection: NodeId) -> Option<u64> {
        self.source_revisions.get(&collection).copied()
    }

    pub fn queue_realization_with_identity(
        &mut self,
        identity: WindowToken,
        request: RealizationRequest,
    ) {
        self.realizations.push(NativeWork {
            identity,
            work: request,
        });
    }
}

impl RecordedNode {
    fn child_list(&self, slot: Option<SlotId>) -> &[NodeId] {
        match slot {
            Some(slot) => self.slot_collections.get(&slot).map_or(&[], Vec::as_slice),
            None => &self.children,
        }
    }

    fn child_list_mut(&mut self, slot: Option<SlotId>) -> &mut Vec<NodeId> {
        match slot {
            Some(slot) => self.slot_collections.entry(slot).or_default(),
            None => &mut self.children,
        }
    }

    pub fn kind(&self) -> Option<MountedKind> {
        self.kind
    }

    pub fn property(&self, property: PropertyId) -> Option<&PropertyValue> {
        self.properties.get(&property)
    }

    pub fn theme_style(&self) -> ThemeStyle {
        self.theme_style
    }

    pub fn children(&self) -> &[NodeId] {
        &self.children
    }

    pub fn slot(&self, slot: SlotId) -> Option<NodeId> {
        self.slots.get(&slot).copied()
    }

    pub fn slot_children(&self, slot: SlotId) -> &[NodeId] {
        self.slot_collections.get(&slot).map_or(&[], Vec::as_slice)
    }
}

impl RecordingRuntime {
    fn apply_one(&mut self, command: &Command) -> Result<(), RuntimeError> {
        match command {
            Command::CreateApplication { node } => {
                if self.nodes.contains_key(node) {
                    return Err(RuntimeError::DuplicateNode(*node));
                }
                if self.application.is_some() {
                    return Err(RuntimeError::DuplicateNode(*node));
                }
                self.nodes.insert(
                    *node,
                    RecordedNode {
                        kind: None,
                        parent: None,
                        children: Vec::new(),
                        slots: BTreeMap::new(),
                        slot_collections: BTreeMap::new(),
                        properties: BTreeMap::new(),
                        theme_style: ThemeStyle::default(),
                    },
                );
                self.application = Some(*node);
            }
            Command::CreateWindow { node } => {
                if self.nodes.contains_key(node) {
                    return Err(RuntimeError::DuplicateNode(*node));
                }
                self.nodes.insert(
                    *node,
                    RecordedNode {
                        kind: None,
                        parent: None,
                        children: Vec::new(),
                        slots: BTreeMap::new(),
                        slot_collections: BTreeMap::new(),
                        properties: BTreeMap::new(),
                        theme_style: ThemeStyle::default(),
                    },
                );
                self.windows.insert(*node);
            }
            Command::ActivateWindow { node } => {
                self.nodes
                    .get(node)
                    .ok_or(RuntimeError::MissingNode(*node))?;
            }
            Command::CloseWindow { node } => {
                if !self.windows.contains(node) {
                    return Err(RuntimeError::MissingNode(*node));
                }
                self.close_requests.push(*node);
            }
            Command::SetWindowTitle { node, title } => {
                if !self.windows.contains(node) {
                    return Err(RuntimeError::MissingNode(*node));
                }
                self.window_titles.insert(*node, title.clone());
            }
            Command::ClearWindowTitleBar { node } => {
                if !self.windows.contains(node) {
                    return Err(RuntimeError::MissingNode(*node));
                }
                self.window_title_bars.remove(node);
            }
            Command::SetWindowTitleBar {
                node,
                title_bar,
                height,
            } => {
                if !self.windows.contains(node) {
                    return Err(RuntimeError::MissingNode(*node));
                }
                if self.nodes.get(title_bar).and_then(|node| node.kind)
                    != Some(MountedKind::TitleBar)
                {
                    return Err(RuntimeError::MissingNode(*title_bar));
                }
                self.window_title_bars.insert(*node, (*title_bar, *height));
            }
            Command::SetWindowVisuals { node, visuals } => {
                if !self.windows.contains(node) {
                    return Err(RuntimeError::MissingNode(*node));
                }
                self.window_visuals.insert(*node, *visuals);
            }
            Command::SetWindowObservations { node, observations } => {
                if !self.windows.contains(node) {
                    return Err(RuntimeError::MissingNode(*node));
                }
                self.window_observations.insert(*node, *observations);
            }
            Command::Create { node, kind } => {
                if self.nodes.contains_key(node) {
                    return Err(RuntimeError::DuplicateNode(*node));
                }
                self.nodes.insert(
                    *node,
                    RecordedNode {
                        kind: Some(*kind),
                        parent: None,
                        children: Vec::new(),
                        slots: BTreeMap::new(),
                        slot_collections: BTreeMap::new(),
                        properties: BTreeMap::new(),
                        theme_style: ThemeStyle::default(),
                    },
                );
                if *kind == MountedKind::ContentDialog {
                    self.content_dialogs
                        .insert(*node, RecordedContentDialog::default());
                }
            }
            Command::CreateVirtualCollection {
                node,
                source_revision,
                ..
            } => {
                if self.nodes.contains_key(node) {
                    return Err(RuntimeError::DuplicateNode(*node));
                }
                self.nodes.insert(
                    *node,
                    RecordedNode {
                        kind: None,
                        parent: None,
                        children: Vec::new(),
                        slots: BTreeMap::new(),
                        slot_collections: BTreeMap::new(),
                        properties: BTreeMap::new(),
                        theme_style: ThemeStyle::default(),
                    },
                );
                self.source_revisions.insert(*node, *source_revision);
            }
            Command::ResetVirtualCollection {
                node,
                source_revision,
                ..
            } => {
                self.nodes
                    .get(node)
                    .ok_or(RuntimeError::MissingNode(*node))?;
                self.source_revisions.insert(*node, *source_revision);
            }
            Command::AttachRealized {
                collection,
                container,
                child,
            } => {
                if self
                    .nodes
                    .get(child)
                    .ok_or(RuntimeError::MissingNode(*child))?
                    .parent
                    .is_some()
                    || self.is_owned_attachment_child(*child)
                {
                    return Err(RuntimeError::AlreadyParented(*child));
                }
                self.nodes
                    .get(collection)
                    .ok_or(RuntimeError::MissingNode(*collection))?;
                if let Some(previous) = self.attachments.get(&(*collection, *container)).copied() {
                    self.nodes
                        .get(&previous)
                        .ok_or(RuntimeError::MissingNode(previous))?;
                    let collection = self.nodes.get_mut(collection).unwrap();
                    let position = collection
                        .children
                        .iter()
                        .position(|current| *current == previous)
                        .ok_or(RuntimeError::ChildNotFound(previous))?;
                    collection.children.remove(position);
                    self.nodes.get_mut(&previous).unwrap().parent = None;
                }
                self.attachments.insert((*collection, *container), *child);
                self.nodes
                    .get_mut(collection)
                    .ok_or(RuntimeError::MissingNode(*collection))?
                    .children
                    .push(*child);
                self.nodes.get_mut(child).unwrap().parent = Some(*collection);
            }
            Command::DetachRealized {
                collection,
                container,
                child,
            } => {
                let attachment = (*collection, *container);
                self.nodes
                    .get(child)
                    .ok_or(RuntimeError::MissingNode(*child))?;
                if self.attachments.get(&attachment) != Some(child) {
                    return Err(RuntimeError::ChildNotFound(*child));
                }
                let collection = self
                    .nodes
                    .get_mut(collection)
                    .ok_or(RuntimeError::MissingNode(*collection))?;
                let position = collection
                    .children
                    .iter()
                    .position(|current| current == child)
                    .ok_or(RuntimeError::ChildNotFound(*child))?;
                collection.children.remove(position);
                self.attachments.remove(&attachment);
                self.nodes.get_mut(child).unwrap().parent = None;
            }
            Command::AcknowledgeRecycle { collection, .. } => {
                self.nodes
                    .get(collection)
                    .ok_or(RuntimeError::MissingNode(*collection))?;
            }
            Command::Destroy { node } => {
                if self
                    .window_title_bars
                    .values()
                    .any(|(title_bar, _)| title_bar == node)
                {
                    return Err(RuntimeError::StillParented(*node));
                }
                if self.tooltips.contains_key(node)
                    || self.flyouts.contains_key(node)
                    || self.is_owned_attachment_child(*node)
                    || self
                        .owned_menus
                        .values()
                        .any(|(target, _, _, _)| target == node)
                    || self
                        .command_bar_flyouts
                        .values()
                        .any(|(target, _, _, _)| target == node)
                {
                    return Err(RuntimeError::StillParented(*node));
                }
                let recorded = self
                    .nodes
                    .get(node)
                    .ok_or(RuntimeError::MissingNode(*node))?;
                if recorded.parent.is_some() {
                    return Err(RuntimeError::StillParented(*node));
                }
                if !recorded.children.is_empty()
                    || !recorded.slots.is_empty()
                    || recorded
                        .slot_collections
                        .values()
                        .any(|children| !children.is_empty())
                {
                    return Err(RuntimeError::HasChildren(*node));
                }
                self.nodes.remove(node);
                self.tree_nodes.remove(node);
                if let Some(dialog) = self.content_dialogs.get_mut(node) {
                    if dialog.pending {
                        dialog.retired = true;
                    } else {
                        self.content_dialogs.remove(node);
                    }
                }
                self.source_revisions.remove(node);
                self.subscriptions
                    .retain(|(subscription_node, _)| subscription_node != node);
                self.window_titles.remove(node);
                self.window_title_bars.remove(node);
                self.window_title_bars
                    .retain(|_, (title_bar, _)| title_bar != node);
                self.window_visuals.remove(node);
                self.windows.remove(node);
                if self.application == Some(*node) {
                    self.application = None;
                }
            }
            Command::RetireSubtree {
                root,
                nodes,
                parent,
                slot,
                ..
            } => {
                if self.retained_subtrees.contains_key(root) {
                    return Err(RuntimeError::StillParented(*root));
                }
                let recorded = self
                    .nodes
                    .get(root)
                    .ok_or(RuntimeError::MissingNode(*root))?;
                if recorded.parent != Some(*parent) {
                    return Err(RuntimeError::ChildNotFound(*root));
                }
                if nodes.iter().any(|node| !self.nodes.contains_key(node)) {
                    return Err(RuntimeError::MissingNode(*root));
                }
                self.retained_subtrees.insert(
                    *root,
                    RecordedRetainedSubtree {
                        nodes: nodes.clone(),
                        parent: *parent,
                        slot: *slot,
                    },
                );
            }
            Command::Focus { node, completion } => {
                self.nodes
                    .get(node)
                    .ok_or(RuntimeError::MissingNode(*node))?;
                _ = completion.call(Ok(true));
            }
            Command::InitializeWebView2 { node, completion } => {
                let recorded = self
                    .nodes
                    .get(node)
                    .ok_or(RuntimeError::MissingNode(*node))?;
                if recorded.kind != Some(MountedKind::WebView2) {
                    _ = completion.call(Err(RuntimeError::UnsupportedKind));
                    return Ok(());
                }
                _ = completion.call(Err(RuntimeError::UnsupportedKind));
            }
            Command::ObserveSwapChainPanel { node, .. } => {
                let recorded = self
                    .nodes
                    .get(node)
                    .ok_or(RuntimeError::MissingNode(*node))?;
                if recorded.kind != Some(MountedKind::SwapChainPanel) {
                    return Err(RuntimeError::UnsupportedKind);
                }
            }
            Command::SetSwapChain {
                node, completion, ..
            } => {
                let recorded = self
                    .nodes
                    .get(node)
                    .ok_or(RuntimeError::MissingNode(*node))?;
                let result = if recorded.kind == Some(MountedKind::SwapChainPanel) {
                    Ok(())
                } else {
                    Err(RuntimeError::UnsupportedKind)
                };
                _ = completion.call(result);
            }
            Command::SetNativeImageSource {
                node, completion, ..
            } => {
                let recorded = self
                    .nodes
                    .get(node)
                    .ok_or(RuntimeError::MissingNode(*node))?;
                let result = if recorded.kind == Some(MountedKind::Image) {
                    Ok(())
                } else {
                    Err(RuntimeError::UnsupportedKind)
                };
                _ = completion.call(result);
            }
            Command::ObserveImageScale { node, .. } => {
                let recorded = self
                    .nodes
                    .get(node)
                    .ok_or(RuntimeError::MissingNode(*node))?;
                if recorded.kind != Some(MountedKind::Image) {
                    return Err(RuntimeError::UnsupportedKind);
                }
            }
            Command::ObserveCompositionHost { node, .. } => {
                let recorded = self
                    .nodes
                    .get(node)
                    .ok_or(RuntimeError::MissingNode(*node))?;
                if recorded.kind != Some(MountedKind::Grid) {
                    return Err(RuntimeError::UnsupportedKind);
                }
            }
            Command::RevokeObservation { .. } => {}
            Command::SetCompositionChildVisual {
                node, completion, ..
            } => {
                let recorded = self
                    .nodes
                    .get(node)
                    .ok_or(RuntimeError::MissingNode(*node))?;
                let result = if recorded.kind == Some(MountedKind::Grid) {
                    Ok(())
                } else {
                    Err(RuntimeError::UnsupportedKind)
                };
                _ = completion.call(result);
            }
            Command::SetProperty {
                node,
                property,
                value,
            } => {
                self.nodes
                    .get_mut(node)
                    .ok_or(RuntimeError::MissingNode(*node))?
                    .properties
                    .insert(*property, value.clone());
            }
            Command::ClearProperty { node, property } => {
                self.nodes
                    .get_mut(node)
                    .ok_or(RuntimeError::MissingNode(*node))?
                    .properties
                    .remove(property);
            }
            Command::SetThemeStyle { node, style } => {
                self.nodes
                    .get_mut(node)
                    .ok_or(RuntimeError::MissingNode(*node))?
                    .theme_style = *style;
            }
            Command::SubscribeEvent { node, event, .. } => {
                self.nodes
                    .get(node)
                    .ok_or(RuntimeError::MissingNode(*node))?;
                if !self.subscriptions.insert((*node, *event)) {
                    return Err(RuntimeError::DuplicateEvent(*node, *event));
                }
            }
            Command::UnsubscribeEvent { node, event } => {
                self.nodes
                    .get(node)
                    .ok_or(RuntimeError::MissingNode(*node))?;
                if !self.subscriptions.remove(&(*node, *event)) {
                    return Err(RuntimeError::MissingSubscription(*node, *event));
                }
            }
            Command::SetSlot {
                parent,
                slot,
                child,
            } => {
                if child == &Some(*parent) {
                    return Err(RuntimeError::SelfParent(*parent));
                }
                self.nodes
                    .get(parent)
                    .ok_or(RuntimeError::MissingNode(*parent))?;
                if let Some(child) = child {
                    let child_node = self
                        .nodes
                        .get(child)
                        .ok_or(RuntimeError::MissingNode(*child))?;
                    let current = self.nodes[parent].slots.get(slot).copied();
                    if self.is_owned_attachment_child(*child)
                        || child_node.parent.is_some() && current != Some(*child)
                    {
                        return Err(RuntimeError::AlreadyParented(*child));
                    }
                }
                let previous = if let Some(child) = child {
                    self.nodes
                        .get_mut(parent)
                        .unwrap()
                        .slots
                        .insert(*slot, *child)
                } else {
                    self.nodes.get_mut(parent).unwrap().slots.remove(slot)
                };
                if let Some(previous) = previous {
                    self.nodes.get_mut(&previous).unwrap().parent = None;
                }
                if let Some(child) = child {
                    self.nodes.get_mut(child).unwrap().parent = Some(*parent);
                }
            }
            Command::SetTooltip {
                target,
                tooltip,
                placement,
            } => {
                self.nodes
                    .get(target)
                    .ok_or(RuntimeError::MissingNode(*target))?;
                if let Some(tooltip) = tooltip {
                    if target == tooltip {
                        return Err(RuntimeError::SelfParent(*target));
                    }
                    let tooltip_node = self
                        .nodes
                        .get(tooltip)
                        .ok_or(RuntimeError::MissingNode(*tooltip))?;
                    let current = self.tooltips.get(target).map(|(tooltip, _)| *tooltip);
                    if self.is_owned_attachment_child(*target) {
                        return Err(RuntimeError::AlreadyParented(*target));
                    }
                    if tooltip_node.parent.is_some()
                        || self.tooltips.contains_key(tooltip)
                        || self.flyouts.contains_key(tooltip)
                        || self.is_owned_attachment_child(*tooltip) && current != Some(*tooltip)
                    {
                        return Err(RuntimeError::AlreadyParented(*tooltip));
                    }
                    if self
                        .tooltips
                        .iter()
                        .any(|(owner, (current, _))| owner != target && current == tooltip)
                    {
                        return Err(RuntimeError::AlreadyParented(*tooltip));
                    }
                    self.tooltips.insert(*target, (*tooltip, *placement));
                } else {
                    self.tooltips.remove(target);
                }
            }
            Command::SetFlyout {
                target,
                content,
                placement,
            } => {
                let target_node = self
                    .nodes
                    .get(target)
                    .ok_or(RuntimeError::MissingNode(*target))?;
                if !matches!(
                    target_node.kind,
                    Some(MountedKind::Button | MountedKind::SplitButton)
                ) {
                    return Err(RuntimeError::UnsupportedKind);
                }
                if let Some(content) = content {
                    if target == content {
                        return Err(RuntimeError::SelfParent(*target));
                    }
                    let content_node = self
                        .nodes
                        .get(content)
                        .ok_or(RuntimeError::MissingNode(*content))?;
                    let current = self.flyouts.get(target).map(|(content, _)| *content);
                    if self.is_owned_attachment_child(*target) {
                        return Err(RuntimeError::AlreadyParented(*target));
                    }
                    if content_node.parent.is_some()
                        || self.tooltips.contains_key(content)
                        || self.flyouts.contains_key(content)
                        || self.is_owned_attachment_child(*content) && current != Some(*content)
                    {
                        return Err(RuntimeError::AlreadyParented(*content));
                    }
                    if self
                        .flyouts
                        .iter()
                        .any(|(owner, (current, _))| owner != target && current == content)
                    {
                        return Err(RuntimeError::AlreadyParented(*content));
                    }
                    self.flyouts.insert(*target, (*content, *placement));
                } else {
                    self.flyouts.remove(target);
                }
            }
            Command::SetOwnedMenu {
                owner,
                target,
                kind,
                items,
                revision,
            } => {
                let target_kind = self
                    .nodes
                    .get(target)
                    .ok_or(RuntimeError::MissingNode(*target))?
                    .kind;
                let expected = match kind {
                    OwnedMenuKind::ButtonFlyout => MountedKind::Button,
                    OwnedMenuKind::DropDownButtonFlyout => MountedKind::DropDownButton,
                    OwnedMenuKind::MenuBarItem => MountedKind::MenuBarItem,
                };
                if target_kind != Some(expected) {
                    return Err(RuntimeError::UnsupportedKind);
                }
                if let Some(items) = items {
                    self.owned_menus
                        .insert(*owner, (*target, *kind, items.clone(), *revision));
                } else {
                    self.owned_menus.remove(owner);
                }
            }
            Command::SetCommandBarFlyout {
                owner,
                target,
                primary,
                secondary,
                revision,
            } => {
                if self
                    .nodes
                    .get(target)
                    .ok_or(RuntimeError::MissingNode(*target))?
                    .kind
                    != Some(MountedKind::Button)
                {
                    return Err(RuntimeError::UnsupportedKind);
                }
                if let Some(primary) = primary {
                    self.command_bar_flyouts.insert(
                        *owner,
                        (*target, primary.clone(), secondary.clone(), *revision),
                    );
                } else {
                    self.command_bar_flyouts.remove(owner);
                }
            }
            Command::SetTreeViewNodes { target, nodes } => {
                if self
                    .nodes
                    .get(target)
                    .ok_or(RuntimeError::MissingNode(*target))?
                    .kind
                    != Some(MountedKind::TreeView)
                {
                    return Err(RuntimeError::UnsupportedKind);
                }
                self.tree_nodes.insert(*target, nodes.clone());
            }
            Command::SetContentDialogOpen { node, owner, open } => {
                if self.nodes.get(node).and_then(|node| node.kind)
                    != Some(MountedKind::ContentDialog)
                {
                    return Err(RuntimeError::UnsupportedKind);
                }
                if !self.nodes.contains_key(owner) {
                    return Err(RuntimeError::MissingNode(*owner));
                }
                let occupied = self
                    .content_dialogs
                    .iter()
                    .any(|(other, state)| other != node && state.pending);
                let state = self.content_dialogs.get_mut(node).unwrap();
                if *open == state.desired_open {
                    return Ok(());
                }
                state.desired_open = *open;
                if *open {
                    if state.pending || occupied {
                        state.queued = true;
                        self.content_dialog_request_order += 1;
                        state.request_order = self.content_dialog_request_order;
                    } else {
                        state.pending = true;
                        state.show_count += 1;
                    }
                } else {
                    state.queued = false;
                    if state.pending {
                        state.suppress_callback = true;
                        state.hide_count += 1;
                    }
                }
            }
            Command::InsertChild {
                parent,
                slot,
                child,
                index,
            } => {
                if parent == child {
                    return Err(RuntimeError::SelfParent(*child));
                }
                let retained = self
                    .retained_subtrees
                    .iter()
                    .filter(|(_, retained)| retained.parent == *parent && retained.slot == *slot)
                    .map(|(root, _)| *root)
                    .collect::<HashSet<_>>();
                let parent_node = self
                    .nodes
                    .get(parent)
                    .ok_or(RuntimeError::MissingNode(*parent))?;
                let physical =
                    physical_child_index(parent_node.child_list(*slot), &retained, *index)?;
                let child_node = self
                    .nodes
                    .get(child)
                    .ok_or(RuntimeError::MissingNode(*child))?;
                if child_node.parent.is_some() || self.is_owned_attachment_child(*child) {
                    return Err(RuntimeError::AlreadyParented(*child));
                }

                self.nodes
                    .get_mut(parent)
                    .unwrap()
                    .child_list_mut(*slot)
                    .insert(physical, *child);
                self.nodes.get_mut(child).unwrap().parent = Some(*parent);
            }
            Command::RemoveChild {
                parent,
                slot,
                child,
            } => {
                let child_node = self
                    .nodes
                    .get(child)
                    .ok_or(RuntimeError::MissingNode(*child))?;
                if child_node.parent != Some(*parent) {
                    return Err(RuntimeError::ChildNotFound(*child));
                }
                let children = self
                    .nodes
                    .get_mut(parent)
                    .ok_or(RuntimeError::MissingNode(*parent))?
                    .child_list_mut(*slot);
                let position = children
                    .iter()
                    .position(|current| current == child)
                    .ok_or(RuntimeError::ChildNotFound(*child))?;
                children.remove(position);
                self.nodes.get_mut(child).unwrap().parent = None;
            }
            Command::SynchronizeChildren {
                parent,
                slot,
                children,
            } => {
                if children.contains(parent) {
                    return Err(RuntimeError::SelfParent(*parent));
                }
                if children.iter().collect::<HashSet<_>>().len() != children.len() {
                    return Err(RuntimeError::DuplicateNode(*parent));
                }
                for child in children {
                    let node = self
                        .nodes
                        .get(child)
                        .ok_or(RuntimeError::MissingNode(*child))?;
                    if self.is_owned_attachment_child(*child)
                        || node.parent.is_some() && node.parent != Some(*parent)
                    {
                        return Err(RuntimeError::AlreadyParented(*child));
                    }
                }
                let retained = self
                    .retained_subtrees
                    .iter()
                    .filter(|(_, retained)| retained.parent == *parent && retained.slot == *slot)
                    .map(|(root, _)| *root)
                    .collect::<HashSet<_>>();
                let desired = merge_retained_children(
                    self.nodes
                        .get(parent)
                        .ok_or(RuntimeError::MissingNode(*parent))?
                        .child_list(*slot),
                    children,
                    &retained,
                );
                let previous = {
                    let parent = self
                        .nodes
                        .get_mut(parent)
                        .ok_or(RuntimeError::MissingNode(*parent))?;
                    std::mem::replace(parent.child_list_mut(*slot), desired.clone())
                };
                for child in previous {
                    self.nodes.get_mut(&child).unwrap().parent = None;
                }
                for child in &desired {
                    let node = self.nodes.get_mut(child).unwrap();
                    if node.parent.is_some() {
                        return Err(RuntimeError::AlreadyParented(*child));
                    }
                    node.parent = Some(*parent);
                }
            }
            Command::MoveChild {
                parent,
                slot,
                child,
                index,
            } => {
                let retained = self
                    .retained_subtrees
                    .iter()
                    .filter(|(_, retained)| retained.parent == *parent && retained.slot == *slot)
                    .map(|(root, _)| *root)
                    .collect::<HashSet<_>>();
                let parent_node = self
                    .nodes
                    .get_mut(parent)
                    .ok_or(RuntimeError::MissingNode(*parent))?;
                let children = parent_node.child_list_mut(*slot);
                let position = children
                    .iter()
                    .position(|current| current == child)
                    .ok_or(RuntimeError::ChildNotFound(*child))?;
                let child = children.remove(position);
                let physical = physical_child_index(children, &retained, *index)?;
                children.insert(physical, child);
            }
        }
        Ok(())
    }
}

fn physical_child_index(
    children: &[NodeId],
    retained: &HashSet<NodeId>,
    semantic_index: usize,
) -> Result<usize, RuntimeError> {
    let mut live = 0;
    for (physical, child) in children.iter().enumerate() {
        if retained.contains(child) {
            continue;
        }
        if live == semantic_index {
            return Ok(physical);
        }
        live += 1;
    }
    if live == semantic_index {
        Ok(children.len())
    } else {
        Err(RuntimeError::IndexOutOfBounds)
    }
}

fn merge_retained_children(
    current: &[NodeId],
    desired: &[NodeId],
    retained: &HashSet<NodeId>,
) -> Vec<NodeId> {
    let desired_set = desired.iter().copied().collect::<HashSet<_>>();
    let mut before = HashMap::<NodeId, Vec<NodeId>>::new();
    let mut trailing = Vec::new();
    for (index, child) in current.iter().copied().enumerate() {
        if !retained.contains(&child) {
            continue;
        }
        if let Some(successor) = current[index + 1..]
            .iter()
            .copied()
            .find(|candidate| desired_set.contains(candidate))
        {
            before.entry(successor).or_default().push(child);
        } else {
            trailing.push(child);
        }
    }
    let mut merged = Vec::with_capacity(desired.len() + retained.len());
    for child in desired.iter().copied() {
        if let Some(retained) = before.remove(&child) {
            merged.extend(retained);
        }
        merged.push(child);
    }
    merged.extend(trailing);
    merged
}

impl NativeRuntime for RecordingRuntime {
    fn apply(&mut self, commands: &[Command]) -> Result<(), NativeApplyError> {
        self.batches += 1;
        if self.record_commands {
            self.commands.push(commands.to_vec());
        }
        for (index, command) in commands.iter().enumerate() {
            let result = if self.fail_at.remove(&(self.batches, index)) {
                Err(RuntimeError::Injected)
            } else {
                self.apply_one(command)
            };
            result.map_err(|error| NativeApplyError {
                command: index,
                error,
            })?;
        }
        Ok(())
    }

    fn open_windows(&mut self, roots: Vec<View>) -> Result<(), RuntimeError> {
        if std::mem::take(&mut self.fail_window_open) {
            return Err(RuntimeError::Injected);
        }
        self.opened_windows.extend(roots);
        Ok(())
    }

    fn reset(&mut self) {
        self.application = None;
        self.attachments.clear();
        self.close_requests.clear();
        self.content_dialogs.clear();
        self.content_dialog_request_order = 0;
        self.events.clear();
        self.host_events.clear();
        self.nodes.clear();
        self.realizations.clear();
        self.retained_subtrees.clear();
        self.source_revisions.clear();
        self.subscriptions.clear();
        self.tooltips.clear();
        self.flyouts.clear();
        self.window_titles.clear();
        self.window_title_bars.clear();
        self.window_observations.clear();
        self.window_visuals.clear();
        self.windows.clear();
    }

    fn set_identity(&mut self, identity: WindowToken) {
        self.identity = Some(identity);
    }

    fn drain_realizations(&mut self) -> Vec<NativeWork<RealizationRequest>> {
        std::mem::take(&mut self.realizations)
    }

    fn drain_events(&mut self) -> Vec<NativeWork<QueuedEvent>> {
        std::mem::take(&mut self.events)
    }

    fn drain_host_events(&mut self) -> Vec<NativeWork<HostEvent>> {
        std::mem::take(&mut self.host_events)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RecordedContentDialog {
    pub desired_open: bool,
    pub pending: bool,
    pub queued: bool,
    pub show_count: usize,
    pub hide_count: usize,
    request_order: u64,
    suppress_callback: bool,
    retired: bool,
}

#[cfg(test)]
#[path = "recording_tests.rs"]
mod tests;
