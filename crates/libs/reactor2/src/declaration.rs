use super::*;
use std::rc::Rc;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Key(KeyKind);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum KeyKind {
    Integer(u64),
    String(Rc<str>),
}

impl From<&str> for Key {
    fn from(value: &str) -> Self {
        Self(KeyKind::String(value.into()))
    }
}

impl From<String> for Key {
    fn from(value: String) -> Self {
        Self(KeyKind::String(value.into()))
    }
}

impl From<u64> for Key {
    fn from(value: u64) -> Self {
        Self(KeyKind::Integer(value))
    }
}

impl From<u32> for Key {
    fn from(value: u32) -> Self {
        Self(KeyKind::Integer(value.into()))
    }
}

impl From<usize> for Key {
    fn from(value: usize) -> Self {
        Self(KeyKind::Integer(value.try_into().unwrap()))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PropertyValue {
    String(Rc<str>),
    Bool(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Property {
    pub id: PropertyId,
    pub value: PropertyValue,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum RelationValue {
    One(Option<Rc<Declaration>>),
    Many(Rc<Vec<Declaration>>),
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct DeclaredRelation {
    pub id: RelationId,
    pub value: RelationValue,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Declaration {
    pub kind: ObjectType,
    pub key: Option<Key>,
    pub properties: SharedList<Property>,
    pub relations: SharedList<DeclaredRelation>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) enum SharedList<T> {
    #[default]
    Empty,
    One(T),
    Many(Rc<Vec<T>>),
}

impl<T> SharedList<T> {
    pub fn as_slice(&self) -> &[T] {
        match self {
            Self::Empty => &[],
            Self::One(value) => std::slice::from_ref(value),
            Self::Many(values) => values,
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.as_slice().iter()
    }
}

impl<T: Clone> SharedList<T> {
    fn upsert(&mut self, matches: impl Fn(&T) -> bool, value: T) {
        let current = std::mem::take(self);
        *self = match current {
            Self::Empty => Self::One(value),
            Self::One(current) if matches(&current) => Self::One(value),
            Self::One(current) => Self::Many(Rc::new(vec![current, value])),
            Self::Many(mut values) => {
                let entries = Rc::make_mut(&mut values);
                if let Some(current) = entries.iter_mut().find(|current| matches(current)) {
                    *current = value;
                } else {
                    entries.push(value);
                }
                Self::Many(values)
            }
        };
    }
}

impl<T> FromIterator<T> for SharedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(values: I) -> Self {
        let mut values = values.into_iter();
        let Some(first) = values.next() else {
            return Self::Empty;
        };
        let Some(second) = values.next() else {
            return Self::One(first);
        };
        Self::Many(Rc::new(
            std::iter::once(first)
                .chain(std::iter::once(second))
                .chain(values)
                .collect(),
        ))
    }
}

impl Drop for Declaration {
    fn drop(&mut self) {
        let mut pending = Vec::new();
        Self::take_owned_children(self, &mut pending);
        while let Some(mut child) = pending.pop() {
            Self::take_owned_children(&mut child, &mut pending);
        }
    }
}

impl Declaration {
    fn new(kind: ObjectType) -> Self {
        Self {
            kind,
            key: None,
            properties: SharedList::Empty,
            relations: SharedList::Empty,
        }
    }

    fn key(mut self, key: impl Into<Key>) -> Self {
        self.key = Some(key.into());
        self
    }

    fn property(mut self, id: PropertyId, value: PropertyValue) -> Self {
        self.properties
            .upsert(|property| property.id == id, Property { id, value });
        self
    }

    fn relation(mut self, id: RelationId, value: RelationValue) -> Self {
        self.relations
            .upsert(|relation| relation.id == id, DeclaredRelation { id, value });
        self
    }

    fn take_owned_children(declaration: &mut Self, pending: &mut Vec<Self>) {
        let relations = std::mem::take(&mut declaration.relations);
        match relations {
            SharedList::Empty => {}
            SharedList::One(relation) => Self::take_relation(relation, pending),
            SharedList::Many(relations) => {
                if let Ok(relations) = Rc::try_unwrap(relations) {
                    for relation in relations {
                        Self::take_relation(relation, pending);
                    }
                }
            }
        }
    }

    fn take_relation(relation: DeclaredRelation, pending: &mut Vec<Self>) {
        match relation.value {
            RelationValue::One(Some(child)) => {
                if let Ok(child) = Rc::try_unwrap(child) {
                    pending.push(child);
                }
            }
            RelationValue::Many(children) => {
                if let Ok(children) = Rc::try_unwrap(children) {
                    pending.extend(children);
                }
            }
            RelationValue::One(None) => {}
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Visual(pub(crate) Declaration);

#[derive(Clone, Debug, PartialEq)]
pub struct KeyedVisual(Visual);

pub fn keyed(key: impl Into<Key>, visual: impl Into<Visual>) -> KeyedVisual {
    let mut visual = visual.into();
    visual.0 = visual.0.key(key);
    KeyedVisual(visual)
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextBlock(Declaration);

impl TextBlock {
    pub fn new(text: impl Into<Rc<str>>) -> Self {
        Self(
            Declaration::new(ObjectType::TextBlock)
                .property(PropertyId::Text, PropertyValue::String(text.into())),
        )
    }
}

impl From<TextBlock> for Visual {
    fn from(value: TextBlock) -> Self {
        Self(value.0)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Border(Declaration);

impl Border {
    pub fn new() -> Self {
        Self(Declaration::new(ObjectType::Border))
    }

    pub fn content(mut self, content: impl Into<Visual>) -> Self {
        self.0 = self.0.relation(
            RelationId::Content,
            RelationValue::One(Some(Rc::new(content.into().0))),
        );
        self
    }
}

impl Default for Border {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Border> for Visual {
    fn from(value: Border) -> Self {
        Self(value.0)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Grid(Declaration);

impl Grid {
    pub fn new() -> Self {
        Self(Declaration::new(ObjectType::Grid))
    }

    /// Adds keyed visual children.
    ///
    /// Structural objects cannot be attached to a visual-child relation.
    ///
    /// ```compile_fail
    /// use windows_reactor2::*;
    ///
    /// let _ = Grid::new().children([TreeNode::new("node", "Node")]);
    /// ```
    pub fn children(mut self, children: impl IntoIterator<Item = KeyedVisual>) -> Self {
        self.0 = self.0.relation(
            RelationId::Children,
            RelationValue::Many(Rc::new(
                children.into_iter().map(|child| child.0.0).collect(),
            )),
        );
        self
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Grid> for Visual {
    fn from(value: Grid) -> Self {
        Self(value.0)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct StackPanel(Declaration);

impl StackPanel {
    pub fn new() -> Self {
        Self(Declaration::new(ObjectType::StackPanel))
    }

    pub fn children(mut self, children: impl IntoIterator<Item = Visual>) -> Self {
        self.0 = self.0.relation(
            RelationId::Children,
            RelationValue::Many(Rc::new(children.into_iter().map(|child| child.0).collect())),
        );
        self
    }
}

impl Default for StackPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl From<StackPanel> for Visual {
    fn from(value: StackPanel) -> Self {
        Self(value.0)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TreeNode(Declaration);

impl TreeNode {
    pub fn new(key: impl Into<Key>, text: impl Into<Rc<str>>) -> Self {
        Self(
            Declaration::new(ObjectType::TreeNode)
                .key(key)
                .property(PropertyId::Text, PropertyValue::String(text.into())),
        )
    }

    pub fn expanded(mut self, expanded: bool) -> Self {
        self.0 = self
            .0
            .property(PropertyId::Expanded, PropertyValue::Bool(expanded));
        self
    }

    pub fn content(mut self, content: impl Into<Visual>) -> Self {
        self.0 = self.0.relation(
            RelationId::Content,
            RelationValue::One(Some(Rc::new(content.into().0))),
        );
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = Self>) -> Self {
        self.0 = self.0.relation(
            RelationId::Children,
            RelationValue::Many(Rc::new(children.into_iter().map(|child| child.0).collect())),
        );
        self
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TreeView(Declaration);

impl TreeView {
    pub fn new() -> Self {
        Self(Declaration::new(ObjectType::TreeView))
    }

    pub fn nodes(mut self, nodes: impl IntoIterator<Item = TreeNode>) -> Self {
        self.0 = self.0.relation(
            RelationId::Roots,
            RelationValue::Many(Rc::new(nodes.into_iter().map(|node| node.0).collect())),
        );
        self
    }
}

impl Default for TreeView {
    fn default() -> Self {
        Self::new()
    }
}

impl From<TreeView> for Visual {
    fn from(value: TreeView) -> Self {
        Self(value.0)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DataItem(Declaration);

impl DataItem {
    pub fn new(key: impl Into<Key>, text: impl Into<Rc<str>>) -> Self {
        Self(
            Declaration::new(ObjectType::DataItem)
                .key(key)
                .property(PropertyId::Text, PropertyValue::String(text.into())),
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ListView(Declaration);

impl ListView {
    pub fn new() -> Self {
        Self(Declaration::new(ObjectType::ListView))
    }

    pub fn items(mut self, items: impl IntoIterator<Item = DataItem>) -> Self {
        self.0 = self.0.relation(
            RelationId::Items,
            RelationValue::Many(Rc::new(items.into_iter().map(|item| item.0).collect())),
        );
        self
    }
}

impl Default for ListView {
    fn default() -> Self {
        Self::new()
    }
}

impl From<ListView> for Visual {
    fn from(value: ListView) -> Self {
        Self(value.0)
    }
}
