use std::collections::BTreeSet;
use std::rc::Rc;

use crate::element::Framework;
use crate::element::props::TreeViewProps;
use crate::element::tree::ElementKind;
use crate::element::{Element, KeyBoolEventFn, KeyEventFn};
use crate::framework_properties::FrameworkProps;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub key: u64,
    pub text: String,
    pub expanded: bool,
    pub children: Rc<[Self]>,
}

impl TreeNode {
    pub fn new(key: u64, text: impl Into<String>) -> Self {
        Self {
            key,
            text: text.into(),
            expanded: false,
            children: Rc::from([]),
        }
    }

    pub fn expanded(mut self, value: bool) -> Self {
        self.expanded = value;
        self
    }

    pub fn child(mut self, value: Self) -> Self {
        self.children = Rc::from([value]);
        self
    }

    pub fn children(mut self, values: impl IntoIterator<Item = Self>) -> Self {
        self.children = values.into_iter().collect();
        self
    }
}

pub struct TreeView {
    props: TreeViewProps,
}

impl TreeView {
    pub fn new(
        nodes: impl IntoIterator<Item = TreeNode>,
        on_expanded_changed: impl Fn(u64, bool) + 'static,
    ) -> Framework<Self> {
        Framework::new(Self::with_handler(
            nodes,
            Some(Rc::new(on_expanded_changed)),
        ))
    }

    pub fn display(nodes: impl IntoIterator<Item = TreeNode>) -> Framework<Self> {
        Framework::new(Self::with_handler(nodes, None))
    }

    fn with_handler(
        nodes: impl IntoIterator<Item = TreeNode>,
        on_expanded_changed: Option<KeyBoolEventFn>,
    ) -> Self {
        let nodes = nodes.into_iter().collect::<Rc<[_]>>();
        validate_tree_keys(&nodes, &mut BTreeSet::new());
        Self {
            props: TreeViewProps {
                nodes,
                on_expanded_changed,
                on_item_invoked: None,
                framework: FrameworkProps::default(),
            },
        }
    }

    pub(crate) fn build_with_framework(mut self, framework: FrameworkProps) -> Element {
        self.props.framework = framework;
        Element::new(ElementKind::TreeView(self.props))
    }
}

fn validate_tree_keys(nodes: &[TreeNode], keys: &mut BTreeSet<u64>) {
    for node in nodes {
        assert!(keys.insert(node.key), "TreeView node keys must be unique");
        validate_tree_keys(&node.children, keys);
    }
}

impl Framework<TreeView> {
    pub fn on_item_invoked(mut self, handler: impl Fn(u64) + 'static) -> Self {
        self.control.props.on_item_invoked = Some(Rc::new(handler) as KeyEventFn);
        self
    }
}
