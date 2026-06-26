use super::*;

/// Definition of a single node in a [`TreeView`].
#[derive(Clone, Debug, PartialEq)]
pub struct TreeNodeDef {
    /// Display text of this node.
    pub text: String,
    /// Whether this node is expanded by default.
    pub is_expanded: bool,
    /// Child nodes.
    pub children: Vec<Self>,
}

impl TreeNodeDef {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            is_expanded: false,
            children: Vec::new(),
        }
    }

    pub fn expanded(mut self) -> Self {
        self.is_expanded = true;
        self
    }

    pub fn child(mut self, node: Self) -> Self {
        self.children.push(node);
        self
    }

    pub fn children(mut self, nodes: Vec<Self>) -> Self {
        self.children = nodes;
        self
    }
}

/// Builder for a [`TreeNodeDef`].
pub fn tree_node(text: impl Into<String>) -> TreeNodeDef {
    TreeNodeDef::new(text)
}

/// `Microsoft.UI.Xaml.Controls.TreeView`. A hierarchical list view.
#[derive(Clone, Debug, PartialEq)]
pub struct TreeView {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub nodes: Vec<TreeNodeDef>,
    pub selection_mode: TreeViewSelectionMode,
    pub on_item_invoked: Option<Callback<String>>,
}
impl Default for TreeView {
    fn default() -> Self {
        Self {
            key: None,
            modifiers: Modifiers::default(),
            nodes: Vec::new(),
            selection_mode: TreeViewSelectionMode::Single,
            on_item_invoked: None,
        }
    }
}

impl TreeView {
    pub fn new(nodes: Vec<TreeNodeDef>) -> Self {
        Self {
            nodes,
            ..Default::default()
        }
    }

    pub fn selection_mode(mut self, mode: TreeViewSelectionMode) -> Self {
        self.selection_mode = mode;
        self
    }

    pub fn on_item_invoked(mut self, f: impl IntoCallback<String>) -> Self {
        self.on_item_invoked = Some(f.into_callback());
        self
    }
}

impl Widget for TreeView {
    widget_header!(ControlKind::TreeView);
    fn bindings(&self) -> PropBindings {
        let mut out = generated::tree_view_bindings(self);
        out.push(Binding::Prop(
            Prop::Nodes,
            PropValue::TreeViewNodes(self.nodes.clone()),
        ));
        out
    }
}

pub fn tree_view(nodes: Vec<TreeNodeDef>) -> TreeView {
    TreeView::new(nodes)
}
