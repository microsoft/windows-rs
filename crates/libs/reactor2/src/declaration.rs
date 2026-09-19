use super::*;
use std::fmt;
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

#[derive(Clone)]
pub struct Callback<T>(Rc<dyn Fn(T)>);

impl<T> Callback<T> {
    pub fn new(callback: impl Fn(T) + 'static) -> Self {
        Self(Rc::new(callback))
    }

    pub fn call(&self, value: T) {
        (self.0)(value);
    }
}

impl<T> fmt::Debug for Callback<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_tuple("Callback").finish()
    }
}

impl<T> PartialEq for Callback<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum EventValue {
    String(Callback<Rc<str>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Event {
    pub id: EventId,
    pub value: EventValue,
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
    pub events: SharedList<Event>,
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
        Self::queue_relations(std::mem::take(&mut self.relations), &mut pending);
        while let Some(frame) = pending.pop() {
            match frame {
                DropFrame::One(Some(mut child)) => {
                    Self::queue_relations(std::mem::take(&mut child.relations), &mut pending);
                }
                DropFrame::One(None) => {}
                DropFrame::Declarations(mut children) => {
                    if let Some(mut child) = children.next() {
                        pending.push(DropFrame::Declarations(children));
                        Self::queue_relations(std::mem::take(&mut child.relations), &mut pending);
                    }
                }
                DropFrame::Relations(mut relations) => {
                    if let Some(relation) = relations.next() {
                        pending.push(DropFrame::Relations(relations));
                        Self::queue_relation(relation, &mut pending);
                    }
                }
            }
        }
    }
}

enum DropFrame {
    One(Option<Declaration>),
    Declarations(std::vec::IntoIter<Declaration>),
    Relations(std::vec::IntoIter<DeclaredRelation>),
}

impl Declaration {
    fn new(kind: ObjectType) -> Self {
        Self {
            kind,
            key: None,
            properties: SharedList::Empty,
            events: SharedList::Empty,
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

    fn event(mut self, id: EventId, value: EventValue) -> Self {
        self.events
            .upsert(|event| event.id == id, Event { id, value });
        self
    }

    fn queue_relations(relations: SharedList<DeclaredRelation>, pending: &mut Vec<DropFrame>) {
        match relations {
            SharedList::Empty => {}
            SharedList::One(relation) => Self::queue_relation(relation, pending),
            SharedList::Many(relations) => {
                if let Ok(relations) = Rc::try_unwrap(relations) {
                    pending.push(DropFrame::Relations(relations.into_iter()));
                }
            }
        }
    }

    fn queue_relation(relation: DeclaredRelation, pending: &mut Vec<DropFrame>) {
        match relation.value {
            RelationValue::One(Some(child)) => {
                if let Ok(child) = Rc::try_unwrap(child) {
                    pending.push(DropFrame::One(Some(child)));
                }
            }
            RelationValue::Many(children) => {
                if let Ok(children) = Rc::try_unwrap(children) {
                    pending.push(DropFrame::Declarations(children.into_iter()));
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
pub struct TextBox(Declaration);

impl TextBox {
    pub fn new(text: impl Into<Rc<str>>) -> Self {
        Self(
            Declaration::new(ObjectType::TextBox)
                .property(PropertyId::Text, PropertyValue::String(text.into())),
        )
    }

    pub fn on_text_changed(mut self, callback: impl Fn(Rc<str>) + 'static) -> Self {
        self = self.on_text_changed_callback(Callback::new(callback));
        self
    }

    pub fn on_text_changed_callback(mut self, callback: Callback<Rc<str>>) -> Self {
        self.0 = self
            .0
            .event(EventId::TextChanged, EventValue::String(callback));
        self
    }
}

impl From<TextBox> for Visual {
    fn from(value: TextBox) -> Self {
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
