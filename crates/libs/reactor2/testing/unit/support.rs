use std::cell::RefCell;
use std::collections::{BTreeMap, VecDeque};

use crate::app::Reactor;
use crate::arena::NodeKind;
use crate::element::{
    ElementRef, State, SystemTitleBar, WindowBackdrop, WindowConstraints, WindowIcon,
    WindowOverlappedPolicy, WindowPresenter, WindowRef, WindowSize, WindowTheme,
};
use crate::engine::Engine;
use crate::id::NodeId;
use crate::runtime::{
    ApplicationUpdate, Attachment, Command, CommandSection, ControlUpdate, NativeEvent, NativeKind,
    NativeRuntime, NativeUpdate, OwnerRelation, TabViewItemUpdate, TimerSpec, WindowCreate,
    WindowUpdate,
};

pub trait ReactorTestExt<R: NativeRuntime> {
    fn engine(&self) -> &Engine<R>;
}

impl<R: NativeRuntime> ReactorTestExt<R> for Reactor<R> {
    fn engine(&self) -> &Engine<R> {
        &self.engine
    }
}

pub trait EngineTestExt<R> {
    fn runtime(&self) -> &R;
    fn is_valid(&self) -> bool;
    fn node_count(&self) -> usize;
    fn node_kind(&self, id: NodeId) -> Option<&NodeKind>;
}

impl<R> EngineTestExt<R> for Engine<R> {
    fn runtime(&self) -> &R {
        &self.runtime
    }

    fn is_valid(&self) -> bool {
        true
    }

    fn node_count(&self) -> usize {
        self.arena.ids().count()
    }

    fn node_kind(&self, id: NodeId) -> Option<&NodeKind> {
        self.arena.get(id).map(|node| &node.kind)
    }
}

pub trait ElementRefTestExt {
    fn node(&self) -> Option<NodeId>;
}

impl<T> ElementRefTestExt for ElementRef<T> {
    fn node(&self) -> Option<NodeId> {
        self.target.current.get()
    }
}

pub trait WindowRefTestExt {
    fn node(&self) -> Option<NodeId>;
}

impl WindowRefTestExt for WindowRef {
    fn node(&self) -> Option<NodeId> {
        self.target.current.get()
    }
}

pub trait StateTestExt {
    fn node(&self) -> NodeId;
}

impl<T> StateTestExt for State<T> {
    fn node(&self) -> NodeId {
        self.node
    }
}

#[derive(Debug, Default)]
pub struct RecordingRuntime {
    application_resources: BTreeMap<NodeId, crate::ApplicationResources>,
    windows: BTreeMap<NodeId, WindowCreate>,
    window_owners: BTreeMap<NodeId, NodeId>,
    window_contents: BTreeMap<NodeId, NodeId>,
    window_backdrops: BTreeMap<NodeId, WindowBackdrop>,
    window_icons: BTreeMap<NodeId, WindowIcon>,
    window_themes: BTreeMap<NodeId, WindowTheme>,
    window_title_bars: BTreeMap<NodeId, SystemTitleBar>,
    window_custom_title_bars: BTreeMap<NodeId, NodeId>,
    window_overlapped: BTreeMap<NodeId, WindowOverlappedPolicy>,
    window_sizes: BTreeMap<NodeId, WindowSize>,
    window_constraints: BTreeMap<NodeId, WindowConstraints>,
    window_presenters: BTreeMap<NodeId, WindowPresenter>,
    nodes: BTreeMap<NodeId, NativeKind>,
    timers: BTreeMap<(NodeId, u32), TimerSpec>,
    parents: BTreeMap<NodeId, NodeId>,
    attachments: BTreeMap<NodeId, Attachment>,
    owner_relations: BTreeMap<NodeId, (NodeId, OwnerRelation)>,
    children: BTreeMap<NodeId, Vec<NodeId>>,
    tab_item_keys: BTreeMap<NodeId, u64>,
    events: RefCell<VecDeque<NativeEvent>>,
    batches: Vec<Vec<Command>>,
    window_activations: Vec<NodeId>,
    focused_elements: Vec<NodeId>,
    fail_next: RefCell<Option<String>>,
}

impl RecordingRuntime {
    pub fn application_resources(&self, id: NodeId) -> Option<&crate::ApplicationResources> {
        self.application_resources.get(&id)
    }

    pub fn queue_event(&self, event: NativeEvent) {
        self.events.borrow_mut().push_back(event);
    }

    pub fn fail_next(&self, message: impl Into<String>) {
        *self.fail_next.borrow_mut() = Some(message.into());
    }

    pub fn contains(&self, id: NodeId) -> bool {
        self.nodes.contains_key(&id)
    }

    pub fn kind(&self, id: NodeId) -> Option<NativeKind> {
        self.nodes.get(&id).copied()
    }

    pub fn native_node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn window_ids(&self) -> Vec<NodeId> {
        self.windows.keys().copied().collect()
    }

    pub fn focused_elements(&self) -> &[NodeId] {
        &self.focused_elements
    }

    pub fn window_title(&self, id: NodeId) -> Option<&str> {
        self.windows.get(&id).map(|window| window.title.as_str())
    }

    pub fn window_size(&self, id: NodeId) -> Option<WindowSize> {
        self.window_sizes.get(&id).copied()
    }

    pub fn window_backdrop(&self, id: NodeId) -> Option<WindowBackdrop> {
        self.window_backdrops.get(&id).copied()
    }

    pub fn window_icon(&self, id: NodeId) -> Option<&WindowIcon> {
        self.window_icons.get(&id)
    }

    pub fn window_theme(&self, id: NodeId) -> Option<WindowTheme> {
        self.windows
            .contains_key(&id)
            .then(|| self.window_themes.get(&id).copied().unwrap_or_default())
    }

    pub fn window_title_bar(&self, id: NodeId) -> Option<SystemTitleBar> {
        self.windows
            .contains_key(&id)
            .then(|| self.window_title_bars.get(&id).copied().unwrap_or_default())
    }

    pub fn window_custom_title_bar(&self, id: NodeId) -> Option<NodeId> {
        self.window_custom_title_bars.get(&id).copied()
    }

    pub fn window_overlapped(&self, id: NodeId) -> Option<WindowOverlappedPolicy> {
        self.windows
            .contains_key(&id)
            .then(|| self.window_overlapped.get(&id).copied().unwrap_or_default())
    }

    pub fn window_presenter(&self, id: NodeId) -> Option<WindowPresenter> {
        self.windows
            .contains_key(&id)
            .then(|| self.window_presenters.get(&id).copied().unwrap_or_default())
    }

    pub fn window_constraints(&self, id: NodeId) -> Option<WindowConstraints> {
        self.windows.contains_key(&id).then(|| {
            self.window_constraints
                .get(&id)
                .copied()
                .unwrap_or_default()
        })
    }

    pub fn window_content(&self, id: NodeId) -> Option<NodeId> {
        self.window_contents.get(&id).copied()
    }

    pub fn window_owner(&self, id: NodeId) -> Option<NodeId> {
        self.window_owners.get(&id).copied()
    }

    pub fn window_activations(&self) -> &[NodeId] {
        &self.window_activations
    }

    pub fn parent(&self, id: NodeId) -> Option<NodeId> {
        self.parents.get(&id).copied()
    }

    pub fn children(&self, id: NodeId) -> &[NodeId] {
        self.children.get(&id).map_or(&[], Vec::as_slice)
    }

    pub fn attachment(&self, id: NodeId) -> Option<Attachment> {
        self.attachments.get(&id).copied()
    }

    pub fn relation_owner(&self, id: NodeId) -> Option<NodeId> {
        self.owner_relations.get(&id).map(|(owner, _)| *owner)
    }

    pub fn batches(&self) -> &[Vec<Command>] {
        &self.batches
    }

    pub fn timers(&self) -> &BTreeMap<(NodeId, u32), TimerSpec> {
        &self.timers
    }
}

impl NativeRuntime for RecordingRuntime {
    fn apply(&mut self, commands: &[Command]) {
        (|| -> Result<(), String> {
            if let Some(error) = self.fail_next.borrow_mut().take() {
                return Err(error);
            }

            let mut windows = self.windows.clone();
            let mut application_resources = self.application_resources.clone();
            let mut window_owners = self.window_owners.clone();
            let mut window_contents = self.window_contents.clone();
            let mut window_backdrops = self.window_backdrops.clone();
            let mut window_icons = self.window_icons.clone();
            let mut window_themes = self.window_themes.clone();
            let mut window_title_bars = self.window_title_bars.clone();
            let mut window_custom_title_bars = self.window_custom_title_bars.clone();
            let mut window_overlapped = self.window_overlapped.clone();
            let mut window_sizes = self.window_sizes.clone();
            let mut window_constraints = self.window_constraints.clone();
            let mut window_presenters = self.window_presenters.clone();
            let mut nodes = self.nodes.clone();
            let mut timers = self.timers.clone();
            let mut parents = self.parents.clone();
            let mut attachments = self.attachments.clone();
            let mut owner_relations = self.owner_relations.clone();
            let mut children = self.children.clone();
            let mut tab_item_keys = self.tab_item_keys.clone();
            let mut window_activations = self.window_activations.clone();
            let mut focused_elements = self.focused_elements.clone();
            for command in commands {
                match *command {
                    Command::StartTimer(spec) => {
                        timers.insert((spec.owner, spec.slot), spec);
                    }
                    Command::StopTimer {
                        owner,
                        slot,
                        revision,
                    } => {
                        let key = (owner, slot);
                        if timers
                            .get(&key)
                            .is_some_and(|timer| timer.revision == revision)
                        {
                            timers.remove(&key);
                        }
                    }
                    Command::UpdateApplication { id, ref update } => match update {
                        ApplicationUpdate::Resources(resources) => {
                            application_resources.insert(id, (**resources).clone());
                        }
                    },
                    Command::CreateWindow { id, ref create } => {
                        if windows.insert(id, create.clone()).is_some() {
                            return Err(failure("window already exists"));
                        }
                    }
                    Command::SetWindowContent { window, content } => {
                        if !windows.contains_key(&window) || !nodes.contains_key(&content) {
                            return Err(failure("window content references an unknown node"));
                        }
                        if window_contents
                            .iter()
                            .any(|(other, current)| *other != window && *current == content)
                        {
                            return Err(failure("window content is already assigned"));
                        }
                        window_contents.insert(window, content);
                    }
                    Command::SetWindowOwner { owner, child } => {
                        if !windows.contains_key(&owner) || !windows.contains_key(&child) {
                            return Err(failure("window owner references an unknown window"));
                        }
                        if owner == child {
                            return Err(failure("window cannot own itself"));
                        }
                        if window_owners.insert(child, owner).is_some() {
                            return Err(failure("window owner is already assigned"));
                        }
                    }
                    Command::UpdateWindow { id, ref update } => {
                        let Some(window) = windows.get_mut(&id) else {
                            return Err(failure("window update target is unknown"));
                        };
                        match update {
                            WindowUpdate::Title(title) => window.title.clone_from(title),
                            WindowUpdate::Backdrop(backdrop) => {
                                if let Some(backdrop) = backdrop {
                                    window_backdrops.insert(id, *backdrop);
                                } else {
                                    window_backdrops.remove(&id);
                                }
                            }
                            WindowUpdate::Icon(icon) => {
                                window_icons.insert(id, icon.clone());
                            }
                            WindowUpdate::Theme(theme) => {
                                window_themes.insert(id, *theme);
                            }
                            WindowUpdate::TitleBar(title_bar) => {
                                window_title_bars.insert(id, **title_bar);
                            }
                            WindowUpdate::BindTitleBar(title_bar) => {
                                if !nodes.contains_key(title_bar) {
                                    return Err(failure(
                                        "window title bar references an unknown node",
                                    ));
                                }
                                window_custom_title_bars.insert(id, *title_bar);
                            }
                            WindowUpdate::UnbindTitleBar => {
                                window_custom_title_bars.remove(&id);
                            }
                            WindowUpdate::Overlapped(policy) => {
                                window_overlapped.insert(id, *policy);
                            }
                            WindowUpdate::ClientSize(size) => {
                                window_sizes.insert(id, *size);
                            }
                            WindowUpdate::Constraints(constraints) => {
                                window_constraints.insert(id, constraints.value());
                            }
                            WindowUpdate::Presenter(presenter) => {
                                window_presenters.insert(id, *presenter);
                            }
                        }
                    }
                    Command::ActivateWindow { id } => {
                        if !windows.contains_key(&id) {
                            return Err(failure("activation target is unknown"));
                        }
                        window_activations.push(id);
                    }
                    Command::FocusElement { id } => {
                        if !nodes.contains_key(&id) {
                            return Err(failure("focus target is unknown"));
                        }
                        focused_elements.push(id);
                    }
                    Command::CloseWindow { id } => {
                        if window_owners.values().any(|owner| *owner == id) {
                            return Err(failure("owned windows must close before their parent"));
                        }
                        if window_custom_title_bars.contains_key(&id) {
                            return Err(failure("custom title bar must be unbound before close"));
                        }
                        if windows.remove(&id).is_none() {
                            return Err(failure("close references an unknown window"));
                        }
                        window_contents.remove(&id);
                        window_backdrops.remove(&id);
                        window_icons.remove(&id);
                        window_themes.remove(&id);
                        window_title_bars.remove(&id);
                        window_custom_title_bars.remove(&id);
                        window_overlapped.remove(&id);
                        window_sizes.remove(&id);
                        window_constraints.remove(&id);
                        window_presenters.remove(&id);
                        window_owners.remove(&id);
                    }
                    Command::Create { id, kind } => {
                        if nodes.insert(id, kind).is_some() {
                            return Err(failure("native node already exists"));
                        }
                        children.insert(id, Vec::new());
                    }
                    Command::Attach {
                        parent,
                        child,
                        attachment,
                    } => {
                        if !nodes.contains_key(&parent) || !nodes.contains_key(&child) {
                            return Err(failure("attach references an unknown node"));
                        }
                        if parents.contains_key(&child) {
                            return Err(failure("native node already has a parent"));
                        }
                        let siblings = children.get_mut(&parent).unwrap();
                        let position = match attachment {
                            Attachment::Child { index } => {
                                if index > siblings.len() {
                                    return Err(failure("attach index is out of bounds"));
                                }
                                index
                            }
                            Attachment::Command { section, index } => {
                                let primary_count = siblings
                                    .iter()
                                    .filter(|sibling| {
                                        matches!(
                                            attachments.get(sibling),
                                            Some(Attachment::Command {
                                                section: CommandSection::Primary,
                                                ..
                                            })
                                        )
                                    })
                                    .count();
                                let section_count = siblings
                                    .iter()
                                    .filter(|sibling| {
                                        matches!(
                                            attachments.get(sibling),
                                            Some(Attachment::Command {
                                                section: sibling_section,
                                                ..
                                            }) if *sibling_section == section
                                        )
                                    })
                                    .count();
                                if index > section_count {
                                    return Err(failure("command attach index is out of bounds"));
                                }
                                match section {
                                    CommandSection::Primary => index,
                                    CommandSection::Secondary => primary_count + index,
                                }
                            }
                            Attachment::Content => {
                                if siblings.iter().any(|sibling| {
                                    attachments.get(sibling) == Some(&Attachment::Content)
                                }) {
                                    return Err(failure("content parent already has a child"));
                                }
                                if siblings.iter().any(|sibling| {
                                    attachments.get(sibling) == Some(&Attachment::Header)
                                }) {
                                    siblings.len()
                                } else {
                                    0
                                }
                            }
                            Attachment::Pane => {
                                if siblings.iter().any(|sibling| {
                                    attachments.get(sibling) == Some(&Attachment::Pane)
                                }) {
                                    return Err(failure("pane parent already has a child"));
                                }
                                siblings.len()
                            }
                            Attachment::PaneFooter => {
                                if siblings.iter().any(|sibling| {
                                    attachments.get(sibling) == Some(&Attachment::PaneFooter)
                                }) {
                                    return Err(failure("pane-footer parent already has a child"));
                                }
                                siblings.len()
                            }
                            Attachment::Header => {
                                if siblings.iter().any(|sibling| {
                                    attachments.get(sibling) == Some(&Attachment::Header)
                                }) {
                                    return Err(failure("header parent already has a child"));
                                }
                                0
                            }
                            Attachment::Item { index } => {
                                if index > siblings.len() {
                                    return Err(failure("item attach index is out of bounds"));
                                }
                                index
                            }
                            Attachment::VirtualItem { index, .. } => siblings
                                .iter()
                                .position(|sibling| {
                                    matches!(
                                        attachments.get(sibling),
                                        Some(Attachment::VirtualItem {
                                            index: sibling_index,
                                            ..
                                        }) if *sibling_index > index
                                    )
                                })
                                .unwrap_or(siblings.len()),
                        };
                        siblings.insert(position, child);
                        parents.insert(child, parent);
                        attachments.insert(child, attachment);
                    }
                    Command::Detach { parent, child } => {
                        if parents.get(&child) != Some(&parent) {
                            return Err(failure("detach does not match native parent"));
                        }
                        parents.remove(&child);
                        attachments.remove(&child);
                        let siblings = children.get_mut(&parent).unwrap();
                        let index = siblings
                            .iter()
                            .position(|candidate| *candidate == child)
                            .ok_or_else(|| failure("native child is missing"))?;
                        siblings.remove(index);
                    }
                    Command::BindOwner {
                        owner,
                        accessory,
                        relation,
                    } => {
                        if !nodes.contains_key(&owner) || !nodes.contains_key(&accessory) {
                            return Err(failure("owner relation references an unknown node"));
                        }
                        if owner_relations
                            .insert(accessory, (owner, relation))
                            .is_some()
                        {
                            return Err(failure("accessory already has an owner relation"));
                        }
                    }
                    Command::UnbindOwner {
                        owner,
                        accessory,
                        relation,
                    } => {
                        if owner_relations.remove(&accessory) != Some((owner, relation)) {
                            return Err(failure("owner relation does not match"));
                        }
                    }
                    Command::Move {
                        parent,
                        child,
                        index,
                    } => {
                        if parents.get(&child) != Some(&parent)
                            || !matches!(
                                attachments.get(&child),
                                Some(
                                    Attachment::Child { .. }
                                        | Attachment::Command { .. }
                                        | Attachment::Item { .. }
                                )
                            )
                        {
                            return Err(failure("move does not match a movable child"));
                        }
                        let siblings = children.get_mut(&parent).unwrap();
                        if index >= siblings.len() {
                            return Err(failure("move index is out of bounds"));
                        }
                        let current = siblings
                            .iter()
                            .position(|candidate| *candidate == child)
                            .ok_or_else(|| failure("native child is missing"))?;
                        let child = siblings.remove(current);
                        siblings.insert(index, child);
                    }
                    Command::RunDeferred { target, window, .. } => {
                        if !nodes.contains_key(&target) {
                            return Err(failure("deferred command references an unknown node"));
                        }
                        if window.is_some_and(|window| !windows.contains_key(&window)) {
                            return Err(failure("deferred command references an unknown window"));
                        }
                    }
                    Command::ApplyCompositionLayout { target, .. } => {
                        if !nodes.contains_key(&target) {
                            return Err(failure("composition command references an unknown node"));
                        }
                    }
                    #[cfg(feature = "canvas")]
                    Command::ApplyCanvasImageLayout { target, .. }
                    | Command::RunCanvasImageFrame { target }
                    | Command::ApplyCanvasLayout { target, .. }
                    | Command::RunCanvasFrame { target }
                    | Command::ApplySwapChainHostLayout { target, .. }
                    | Command::RunSwapChainHostFrame { target } => {
                        if !nodes.contains_key(&target) {
                            return Err(failure("canvas command references an unknown node"));
                        }
                    }
                    #[cfg(feature = "webview")]
                    Command::FinishWebViewInitialization { target, .. } => {
                        if !nodes.contains_key(&target) {
                            return Err(failure("WebView command references an unknown node"));
                        }
                    }
                    Command::Destroy { id } => {
                        if window_custom_title_bars
                            .values()
                            .any(|title_bar| *title_bar == id)
                        {
                            return Err(failure("bound title bar cannot be destroyed"));
                        }
                        if parents.contains_key(&id) || parents.values().any(|parent| *parent == id)
                        {
                            return Err(failure("destroyed native node is still attached"));
                        }
                        if nodes.remove(&id).is_none() {
                            return Err(failure("destroy references an unknown node"));
                        }
                        tab_item_keys.remove(&id);
                        children.remove(&id);
                    }
                    Command::Update { id, ref update } => {
                        let Some(kind) = nodes.get(&id).copied() else {
                            return Err(failure("update target is unknown"));
                        };
                        if !update.supports(kind) {
                            return Err(failure(format!(
                                "{} does not support {kind:?}",
                                update.name()
                            )));
                        }
                        if let NativeUpdate::Control(ControlUpdate::TabViewItem(
                            TabViewItemUpdate::Key(key),
                        )) = update
                        {
                            tab_item_keys.insert(id, *key);
                        }
                    }
                }

                fn failure(message: impl Into<String>) -> String {
                    message.into()
                }
            }

            self.windows = windows;
            self.application_resources = application_resources;
            self.window_owners = window_owners;
            self.window_contents = window_contents;
            self.window_backdrops = window_backdrops;
            self.window_icons = window_icons;
            self.window_themes = window_themes;
            self.window_title_bars = window_title_bars;
            self.window_custom_title_bars = window_custom_title_bars;
            self.window_overlapped = window_overlapped;
            self.window_sizes = window_sizes;
            self.window_constraints = window_constraints;
            self.window_presenters = window_presenters;
            self.nodes = nodes;
            self.timers = timers;
            self.parents = parents;
            self.attachments = attachments;
            self.owner_relations = owner_relations;
            self.children = children;
            self.tab_item_keys = tab_item_keys;
            self.window_activations = window_activations;
            self.focused_elements = focused_elements;
            self.batches.push(commands.to_vec());
            Ok(())
        })()
        .unwrap();
    }

    fn drain_events(&mut self) -> Vec<NativeEvent> {
        let events = self.events.borrow_mut().drain(..).collect::<Vec<_>>();
        for event in &events {
            if let NativeEvent::TabsReordered { target, keys } = event {
                let Some(children) = self.children.get(target).cloned() else {
                    continue;
                };
                let reordered = keys
                    .iter()
                    .map(|key| {
                        children
                            .iter()
                            .copied()
                            .find(|child| self.tab_item_keys.get(child) == Some(key))
                            .unwrap()
                    })
                    .collect::<Vec<_>>();
                assert_eq!(
                    children.len(),
                    reordered.len(),
                    "recorded TabView reorder changed the child count"
                );
                self.children.insert(*target, reordered.clone());
                for (index, child) in reordered.into_iter().enumerate() {
                    self.attachments.insert(child, Attachment::Item { index });
                }
            }
        }
        events
    }
}
