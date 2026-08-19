use super::controlled::inspectable_text;
use super::*;

struct RichEditCallbackState {
    expected: RefCell<String>,
    suppressing: Cell<bool>,
}

struct TreeCallbackState {
    expected_expansion: RefCell<BTreeMap<u64, bool>>,
    feedback_enabled: Cell<bool>,
    suppressing: Cell<bool>,
}

pub(super) struct RichEditBoxState {
    pub(super) value: bindings::RichEditBox,
    _revoker: windows_core::EventRevoker,
    callback: Rc<RichEditCallbackState>,
}

pub(super) struct TreeViewState {
    pub(super) value: bindings::TreeView,
    _revokers: Box<[windows_core::EventRevoker; 3]>,
    pub(super) nodes: Rc<RefCell<BTreeMap<u64, bindings::TreeViewNode>>>,
    callback: Rc<TreeCallbackState>,
}

impl WinUiRuntime {
    pub(super) fn create_rich_edit_box(&self, id: NodeId) -> WindowsResult<Handle> {
        let value = bindings::RichEditBox::new()?;
        let callback = Rc::new(RichEditCallbackState {
            expected: RefCell::new(String::new()),
            suppressing: Cell::new(false),
        });
        let event_value = value.clone();
        let event_state = Rc::clone(&callback);
        let events = Rc::clone(&self.events);
        let waker = Rc::clone(&self.waker);
        let revoker = value.TextChanged(move |_sender, _args| {
            if event_state.suppressing.get() {
                return;
            }
            let mut text = windows_core::HSTRING::new();
            event_value
                .Document()
                .unwrap()
                .GetText(bindings::TextGetOptions::None, &mut text)
                .unwrap();
            let mut text = text.to_string_lossy();
            if text.ends_with('\r') {
                text.pop();
            }
            if *event_state.expected.borrow() == text {
                return;
            }
            event_state.expected.replace(text.clone());
            events.borrow_mut().push_back(NativeEvent::TextChanged {
                target: id,
                value: text,
            });
            if let Some(wake) = waker.borrow().as_ref() {
                wake();
            }
        })?;
        Ok(Handle::RichEditBox(Box::new(RichEditBoxState {
            value,
            _revoker: revoker,
            callback,
        })))
    }

    pub(super) fn create_tree_view(&self, id: NodeId) -> WindowsResult<Handle> {
        let value = bindings::TreeView::new()?;
        let nodes = Rc::new(RefCell::new(BTreeMap::new()));
        let callback = Rc::new(TreeCallbackState {
            expected_expansion: RefCell::new(BTreeMap::new()),
            feedback_enabled: Cell::new(false),
            suppressing: Cell::new(false),
        });
        let event_nodes = Rc::clone(&nodes);
        let events = Rc::clone(&self.events);
        let waker = Rc::clone(&self.waker);
        let invoked = value.ItemInvoked(move |_sender, args| {
            let invoked = args
                .as_ref()
                .unwrap()
                .InvokedItem()
                .unwrap()
                .cast::<bindings::TreeViewNode>()
                .unwrap();
            let key = event_nodes
                .borrow()
                .iter()
                .find_map(|(key, node)| (node == &invoked).then_some(*key))
                .unwrap();
            events
                .borrow_mut()
                .push_back(NativeEvent::ItemInvoked { target: id, key });
            if let Some(wake) = waker.borrow().as_ref() {
                wake();
            }
        })?;
        let events_interface: ITreeViewEvents = value.cast()?;
        let expanding_nodes = Rc::clone(&nodes);
        let expanding_state = Rc::clone(&callback);
        let expanding_dispatcher = self.dispatcher.clone();
        let events = Rc::clone(&self.events);
        let waker = Rc::clone(&self.waker);
        let expanding = events_interface.expanding(move |_sender, args| {
            if expanding_state.suppressing.get() {
                return;
            }
            let node = args.as_ref().unwrap().node().unwrap();
            let key = tree_node_key(&expanding_nodes.borrow(), &node);
            if expanding_state.expected_expansion.borrow().get(&key) == Some(&true) {
                return;
            }
            if !expanding_state.feedback_enabled.get() {
                defer_tree_expansion_event(
                    &expanding_dispatcher,
                    Rc::clone(&events),
                    Rc::clone(&waker),
                    id,
                    key,
                    true,
                );
                return;
            }
            expanding_state
                .expected_expansion
                .borrow_mut()
                .insert(key, true);
            queue_tree_expansion_event(&events, &waker, id, key, true);
        })?;
        let collapsed_nodes = Rc::clone(&nodes);
        let collapsed_state = Rc::clone(&callback);
        let collapsed_dispatcher = self.dispatcher.clone();
        let events = Rc::clone(&self.events);
        let waker = Rc::clone(&self.waker);
        let collapsed = events_interface.collapsed(move |_sender, args| {
            if collapsed_state.suppressing.get() {
                return;
            }
            let node = args.as_ref().unwrap().node().unwrap();
            let key = tree_node_key(&collapsed_nodes.borrow(), &node);
            if collapsed_state.expected_expansion.borrow().get(&key) == Some(&false) {
                return;
            }
            if !collapsed_state.feedback_enabled.get() {
                defer_tree_expansion_event(
                    &collapsed_dispatcher,
                    Rc::clone(&events),
                    Rc::clone(&waker),
                    id,
                    key,
                    false,
                );
                return;
            }
            collapsed_state
                .expected_expansion
                .borrow_mut()
                .insert(key, false);
            queue_tree_expansion_event(&events, &waker, id, key, false);
        })?;
        Ok(Handle::TreeView(Box::new(TreeViewState {
            value,
            _revokers: Box::new([invoked, expanding, collapsed]),
            nodes,
            callback,
        })))
    }

    pub(super) fn apply_rich_edit_box(
        &self,
        id: NodeId,
        update: &RichEditBoxUpdate,
    ) -> WindowsResult<()> {
        let Handle::RichEditBox(state) = &self.node(id)?.handle else {
            panic!("rich-edit update target is not a RichEditBox");
        };
        let header = update.header.as_deref().map(inspectable_text);
        state.value.SetHeader(header.as_ref())?;
        state
            .value
            .SetPlaceholderText(update.placeholder.as_deref().unwrap_or_default())?;

        if *state.callback.expected.borrow() != update.text {
            let previous = state.callback.expected.replace(update.text.clone());
            state.callback.suppressing.set(true);
            let result = (|| {
                if state.value.IsReadOnly()? {
                    state.value.SetIsReadOnly(false)?;
                }
                state
                    .value
                    .Document()?
                    .SetText(bindings::TextSetOptions::None, &update.text)
            })();
            state.callback.suppressing.set(false);
            if result.is_err() {
                state.callback.expected.replace(previous);
            }
            result?;
        }
        state.value.SetIsReadOnly(update.read_only)
    }

    pub(super) fn apply_rich_text_block(
        &self,
        id: NodeId,
        update: &RichTextBlockUpdate,
    ) -> WindowsResult<()> {
        let Handle::RichTextBlock(value) = &self.node(id)?.handle else {
            panic!("rich-text update target is not a RichTextBlock");
        };
        if let Some(font_size) = update.font_size {
            value.SetFontSize(font_size)?;
        } else {
            value
                .cast::<bindings::DependencyObject>()?
                .ClearValue(&bindings::RichTextBlock::FontSizeProperty()?)?;
        }
        value.SetIsTextSelectionEnabled(update.selectable)?;
        value.SetTextWrapping(if update.wrap {
            bindings::TextWrapping::Wrap
        } else {
            bindings::TextWrapping::NoWrap
        })?;
        let blocks = value.Blocks()?;
        blocks.Clear()?;
        for paragraph in update.paragraphs.iter() {
            let native = bindings::Paragraph::new()?;
            let inlines = native.Inlines()?;
            for inline in paragraph.inlines.iter() {
                match inline {
                    RichTextInline::Run(run) => append_run(&inlines, run)?,
                    RichTextInline::Hyperlink(link) => append_hyperlink(&inlines, link)?,
                    RichTextInline::LineBreak => {
                        append_run(&inlines, &RichTextRun::plain("\n"))?;
                    }
                }
            }
            blocks.Append(&native)?;
        }
        Ok(())
    }

    pub(super) fn apply_tree_view_update(
        &self,
        id: NodeId,
        update: &TreeViewUpdate,
    ) -> WindowsResult<()> {
        match update {
            TreeViewUpdate::Nodes(definitions) => self.apply_tree_view_nodes(id, definitions),
            TreeViewUpdate::ExpandedChanged(enabled) => {
                let Handle::TreeView(state) = &self.node(id)?.handle else {
                    panic!("tree update target is not a TreeView");
                };
                state.callback.feedback_enabled.set(*enabled);
                Ok(())
            }
        }
    }

    fn apply_tree_view_nodes(&self, id: NodeId, definitions: &[TreeNode]) -> WindowsResult<()> {
        let Handle::TreeView(state) = &self.node(id)?.handle else {
            panic!("tree update target is not a TreeView");
        };
        let mut expected = BTreeMap::new();
        collect_tree_expansion(definitions, &mut expected);
        *state.callback.expected_expansion.borrow_mut() = expected;
        state.callback.suppressing.set(true);
        let result = (|| {
            let previous = state.nodes.borrow();
            let mut next = BTreeMap::new();
            let roots = state.value.RootNodes()?;
            roots.Clear()?;
            for node in previous.values() {
                node.Children()?.Clear()?;
            }
            for definition in definitions {
                let node = reconcile_tree_node(definition, &previous, &mut next)?;
                roots.Append(&node)?;
            }
            drop(previous);
            state.nodes.replace(next);
            Ok(())
        })();
        state.callback.suppressing.set(false);
        result
    }
}

fn defer_tree_expansion_event(
    dispatcher: &bindings::DispatcherQueue,
    events: Rc<RefCell<VecDeque<NativeEvent>>>,
    waker: Rc<RefCell<Option<Rc<dyn Fn()>>>>,
    target: NodeId,
    key: u64,
    expanded: bool,
) {
    let handler = bindings::DispatcherQueueHandler::new(move || {
        queue_tree_expansion_event(&events, &waker, target, key, expanded);
    });
    assert!(
        dispatcher
            .TryEnqueueWithPriority(bindings::DispatcherQueuePriority::Low, &handler)
            .unwrap(),
        "dispatcher rejected TreeView expansion restoration"
    );
}

fn collect_tree_expansion(nodes: &[TreeNode], expected: &mut BTreeMap<u64, bool>) {
    for node in nodes {
        expected.insert(node.key, node.expanded);
        collect_tree_expansion(&node.children, expected);
    }
}

fn tree_node_key(
    nodes: &BTreeMap<u64, bindings::TreeViewNode>,
    node: &bindings::TreeViewNode,
) -> u64 {
    nodes
        .iter()
        .find_map(|(key, candidate)| (candidate == node).then_some(*key))
        .unwrap()
}

fn queue_tree_expansion_event(
    events: &RefCell<VecDeque<NativeEvent>>,
    waker: &RefCell<Option<Rc<dyn Fn()>>>,
    target: NodeId,
    key: u64,
    expanded: bool,
) {
    events
        .borrow_mut()
        .push_back(NativeEvent::TreeNodeExpandedChanged {
            target,
            key,
            expanded,
        });
    if let Some(wake) = waker.borrow().as_ref() {
        wake();
    }
}

fn append_run(inlines: &bindings::InlineCollection, descriptor: &RichTextRun) -> WindowsResult<()> {
    let run = bindings::Run::new()?;
    run.SetText(&descriptor.text)?;
    let element: bindings::ITextElement = run.cast()?;
    if descriptor.bold {
        element.SetFontWeight(bindings::FontWeight { weight: 700 })?;
    }
    if descriptor.italic {
        element.SetFontStyle(bindings::FontStyle::Italic)?;
    }
    inlines.Append(&run)
}

fn append_hyperlink(
    inlines: &bindings::InlineCollection,
    descriptor: &RichTextHyperlink,
) -> WindowsResult<()> {
    let hyperlink = bindings::Hyperlink::new()?;
    hyperlink.SetNavigateUri(&bindings::Uri::CreateUri(&descriptor.uri)?)?;
    let span: bindings::ISpan = hyperlink.cast()?;
    append_run(&span.Inlines()?, &RichTextRun::plain(&descriptor.text))?;
    inlines.Append(&hyperlink)
}

#[cfg(test)]
mod callback_state_tests {
    use super::*;

    #[test]
    fn text_state_drops_field_borrows_before_synchronous_reentry() {
        let state = Rc::new(RichEditCallbackState {
            expected: RefCell::new("before".to_string()),
            suppressing: Cell::new(false),
        });
        let callback_state = Rc::clone(&state);
        let published = RefCell::new(Vec::new());
        let callback = |value: &str| {
            if callback_state.suppressing.get() {
                assert_eq!(&*callback_state.expected.borrow(), value);
                return;
            }
            callback_state.expected.replace(value.to_string());
            published.borrow_mut().push(value.to_string());
        };

        let previous = state.expected.replace("controlled".to_string());
        state.suppressing.set(true);
        callback("controlled");
        state.suppressing.set(false);
        assert_eq!(previous, "before");
        assert!(published.borrow().is_empty());

        callback("native");
        assert_eq!(&*state.expected.borrow(), "native");
        assert_eq!(&*published.borrow(), &["native"]);
    }
}

fn reconcile_tree_node(
    definition: &TreeNode,
    previous: &BTreeMap<u64, bindings::TreeViewNode>,
    next: &mut BTreeMap<u64, bindings::TreeViewNode>,
) -> WindowsResult<bindings::TreeViewNode> {
    assert!(
        !next.contains_key(&definition.key),
        "TreeView keys must be globally unique"
    );
    let node = previous
        .get(&definition.key)
        .cloned()
        .map_or_else(bindings::TreeViewNode::new, Ok)?;
    node.SetContent(&inspectable_text(&definition.text))?;
    node.SetIsExpanded(definition.expanded)?;
    let children = node.Children()?;
    for child in definition.children.iter() {
        children.Append(&reconcile_tree_node(child, previous, next)?)?;
    }
    next.insert(definition.key, node.clone());
    Ok(node)
}

windows_core::imp::define_interface!(
    ITreeViewEvents,
    ITreeViewEvents_Vtbl,
    0x1bef9af4_712c_50ef_9bb4_881b975232ab
);

impl windows_core::RuntimeType for ITreeViewEvents {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}

impl ITreeViewEvents {
    fn expanding<F>(&self, handler: F) -> WindowsResult<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<bindings::TreeView>, windows_core::Ref<TreeViewExpandingEventArgs>)
            + 'static,
    {
        let handler =
            bindings::TypedEventHandler::<bindings::TreeView, TreeViewExpandingEventArgs>::new(
                handler,
            );
        unsafe {
            let mut token = core::mem::zeroed();
            (Interface::vtable(self).expanding)(
                Interface::as_raw(self),
                Interface::as_raw(&handler),
                &mut token,
            )
            .map(|| token)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token,
                Interface::vtable(self).remove_expanding,
            ))
        }
    }

    fn collapsed<F>(&self, handler: F) -> WindowsResult<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<bindings::TreeView>, windows_core::Ref<TreeViewCollapsedEventArgs>)
            + 'static,
    {
        let handler =
            bindings::TypedEventHandler::<bindings::TreeView, TreeViewCollapsedEventArgs>::new(
                handler,
            );
        unsafe {
            let mut token = core::mem::zeroed();
            (Interface::vtable(self).collapsed)(
                Interface::as_raw(self),
                Interface::as_raw(&handler),
                &mut token,
            )
            .map(|| token)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token,
                Interface::vtable(self).remove_collapsed,
            ))
        }
    }
}

#[repr(C)]
pub struct ITreeViewEvents_Vtbl {
    base__: windows_core::IInspectable_Vtbl,
    root_nodes: usize,
    selection_mode: usize,
    set_selection_mode: usize,
    selected_nodes: usize,
    expand: usize,
    collapse: usize,
    select_all: usize,
    item_invoked: usize,
    remove_item_invoked: usize,
    expanding: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    remove_expanding:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    collapsed: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    remove_collapsed:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}

windows_core::imp::define_interface!(
    ITreeViewExpandingEventArgs,
    ITreeViewExpandingEventArgs_Vtbl,
    0xbd769ef7_cadc_5334_93ad_c9bbe820643d
);

impl windows_core::RuntimeType for ITreeViewExpandingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}

impl ITreeViewExpandingEventArgs {
    fn node(&self) -> WindowsResult<bindings::TreeViewNode> {
        unsafe {
            let mut result = core::mem::zeroed();
            (Interface::vtable(self).node)(Interface::as_raw(self), &mut result)
                .and_then(|| windows_core::Type::from_abi(result))
        }
    }
}

#[repr(C)]
pub struct ITreeViewExpandingEventArgs_Vtbl {
    base__: windows_core::IInspectable_Vtbl,
    node: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}

#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
struct TreeViewExpandingEventArgs(windows_core::IUnknown);

windows_core::imp::interface_hierarchy!(
    TreeViewExpandingEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);

impl windows_core::RuntimeType for TreeViewExpandingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ITreeViewExpandingEventArgs>();
}

unsafe impl Interface for TreeViewExpandingEventArgs {
    type Vtable = <ITreeViewExpandingEventArgs as Interface>::Vtable;
    const IID: windows_core::GUID = <ITreeViewExpandingEventArgs as Interface>::IID;
}

impl core::ops::Deref for TreeViewExpandingEventArgs {
    type Target = ITreeViewExpandingEventArgs;

    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}

impl windows_core::RuntimeName for TreeViewExpandingEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.TreeViewExpandingEventArgs";
}

windows_core::imp::define_interface!(
    ITreeViewCollapsedEventArgs,
    ITreeViewCollapsedEventArgs_Vtbl,
    0x8ee00b59_42c6_5d73_809f_68710088e5a5
);

impl windows_core::RuntimeType for ITreeViewCollapsedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}

impl ITreeViewCollapsedEventArgs {
    fn node(&self) -> WindowsResult<bindings::TreeViewNode> {
        unsafe {
            let mut result = core::mem::zeroed();
            (Interface::vtable(self).node)(Interface::as_raw(self), &mut result)
                .and_then(|| windows_core::Type::from_abi(result))
        }
    }
}

#[repr(C)]
pub struct ITreeViewCollapsedEventArgs_Vtbl {
    base__: windows_core::IInspectable_Vtbl,
    node: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}

#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
struct TreeViewCollapsedEventArgs(windows_core::IUnknown);

windows_core::imp::interface_hierarchy!(
    TreeViewCollapsedEventArgs,
    windows_core::IUnknown,
    windows_core::IInspectable
);

impl windows_core::RuntimeType for TreeViewCollapsedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ITreeViewCollapsedEventArgs>();
}

unsafe impl Interface for TreeViewCollapsedEventArgs {
    type Vtable = <ITreeViewCollapsedEventArgs as Interface>::Vtable;
    const IID: windows_core::GUID = <ITreeViewCollapsedEventArgs as Interface>::IID;
}

impl core::ops::Deref for TreeViewCollapsedEventArgs {
    type Target = ITreeViewCollapsedEventArgs;

    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}

impl windows_core::RuntimeName for TreeViewCollapsedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.TreeViewCollapsedEventArgs";
}
