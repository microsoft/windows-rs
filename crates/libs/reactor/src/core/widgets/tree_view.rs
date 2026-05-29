use super::*;

/// Definition of a single node in a [`TreeViewWidget`].
#[derive(Clone, Debug, PartialEq)]
pub struct TreeNodeDef {
    /// Display text of this node.
    pub text: String,
    /// Whether this node is expanded by default.
    pub is_expanded: bool,
    /// Child nodes.
    pub children: Vec<TreeNodeDef>,
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

    pub fn child(mut self, node: TreeNodeDef) -> Self {
        self.children.push(node);
        self
    }

    pub fn children(mut self, nodes: Vec<TreeNodeDef>) -> Self {
        self.children = nodes;
        self
    }
}

/// Builder for a [`TreeNodeDef`].
pub fn tree_node(text: impl Into<String>) -> TreeNodeDef {
    TreeNodeDef::new(text)
}

/// Selection mode for [`TreeViewWidget`].
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum TreeSelectionMode {
    /// No selection.
    None,
    /// Single item selection.
    #[default]
    Single,
    /// Multiple items can be selected.
    Multiple,
}

/// `Microsoft.UI.Xaml.Controls.TreeView`. A hierarchical list view.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct TreeViewWidget {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub nodes: Vec<TreeNodeDef>,
    pub selection_mode: TreeSelectionMode,
    pub on_item_invoked: Option<Callback<String>>,
}

impl TreeViewWidget {
    pub fn new(nodes: Vec<TreeNodeDef>) -> Self {
        Self {
            nodes,
            ..Default::default()
        }
    }

    pub fn selection_mode(mut self, mode: TreeSelectionMode) -> Self {
        self.selection_mode = mode;
        self
    }

    pub fn on_item_invoked<F: Fn(String) + 'static>(mut self, f: F) -> Self {
        self.on_item_invoked = Some(Callback::new(f));
        self
    }
}

impl Widget for TreeViewWidget {
    widget_header!(ControlKind::TreeView);
    fn bindings(&self) -> PropBindings {
        vec![
            Binding::Prop(
                Prop::TreeViewNodes,
                PropValue::TreeViewNodes(self.nodes.clone()),
            ),
            Binding::Prop(
                Prop::TreeViewSelectionMode,
                PropValue::TreeViewSelectionMode(self.selection_mode),
            ),
            Binding::Event(
                Event::TreeViewItemInvoked,
                self.on_item_invoked
                    .as_ref()
                    .map(|cb| EventHandler::TextChanged(cb.clone())),
            ),
        ]
    }
}

pub fn tree_view(nodes: Vec<TreeNodeDef>) -> TreeViewWidget {
    TreeViewWidget::new(nodes)
}
