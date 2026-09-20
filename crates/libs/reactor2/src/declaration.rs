use super::*;
use std::fmt;
use std::rc::Rc;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Color {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const fn argb(a: u8, r: u8, g: u8, b: u8) -> Self {
        Self { a, r, g, b }
    }

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::argb(255, r, g, b)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CornerRadius {
    pub top_left: f64,
    pub top_right: f64,
    pub bottom_right: f64,
    pub bottom_left: f64,
}

impl CornerRadius {
    pub const fn new(top_left: f64, top_right: f64, bottom_right: f64, bottom_left: f64) -> Self {
        Self {
            top_left,
            top_right,
            bottom_right,
            bottom_left,
        }
    }

    pub const fn uniform(value: f64) -> Self {
        Self::new(value, value, value, value)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Thickness {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}

impl Thickness {
    pub const fn new(left: f64, top: f64, right: f64, bottom: f64) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }

    pub const fn uniform(value: f64) -> Self {
        Self::new(value, value, value, value)
    }
}

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
    Color(Color),
    CornerRadius(CornerRadius),
    F64(f64),
    OptionalBool(Option<bool>),
    Thickness(Thickness),
    Enum {
        kind: &'static str,
        variant: &'static str,
    },
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
    F64(Callback<f64>),
    Unit(Callback<()>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum EventPayload {
    String(Rc<str>),
    F64(f64),
    Unit,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Event {
    pub id: EventId,
    pub value: EventValue,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum RelationValue {
    One(Option<Rc<DeclaredNode>>),
    Many(Rc<Vec<DeclaredNode>>),
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
    pub component: Option<ComponentId>,
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
    pub(crate) fn upsert(&mut self, matches: impl Fn(&T) -> bool, value: T) {
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

    fn sort_by_key<K: Ord>(&mut self, key: impl FnMut(&T) -> K) {
        if let Self::Many(values) = self {
            Rc::make_mut(values).sort_by_key(key);
        }
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
                    if let DeclaredNode::Object(ref mut child) = child {
                        Self::queue_relations(std::mem::take(&mut child.relations), &mut pending);
                    }
                }
                DropFrame::One(None) => {}
                DropFrame::Declarations(mut children) => {
                    if let Some(mut child) = children.next() {
                        pending.push(DropFrame::Declarations(children));
                        if let DeclaredNode::Object(ref mut child) = child {
                            Self::queue_relations(
                                std::mem::take(&mut child.relations),
                                &mut pending,
                            );
                        }
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
    One(Option<DeclaredNode>),
    Declarations(std::vec::IntoIter<DeclaredNode>),
    Relations(std::vec::IntoIter<DeclaredRelation>),
}

#[derive(Clone)]
pub(crate) enum DeclaredNode {
    Object(Declaration),
    Component {
        node: ComponentNode,
        relation_key: Option<Key>,
    },
}

impl fmt::Debug for DeclaredNode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Object(value) => value.fmt(formatter),
            Self::Component { node, relation_key } => formatter
                .debug_struct("Component")
                .field("key", &node.key)
                .field("relation_key", relation_key)
                .finish(),
        }
    }
}

impl PartialEq for DeclaredNode {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Object(left), Self::Object(right)) => left == right,
            (
                Self::Component {
                    node: left,
                    relation_key: left_key,
                },
                Self::Component {
                    node: right,
                    relation_key: right_key,
                },
            ) => left == right && left_key == right_key,
            _ => false,
        }
    }
}

impl DeclaredNode {
    fn key(self, key: impl Into<Key>) -> Self {
        let key = key.into();
        match self {
            Self::Object(value) => Self::Object(value.key(key)),
            Self::Component { node, .. } => Self::Component {
                node,
                relation_key: Some(key),
            },
        }
    }

    pub(crate) fn object(self) -> Result<Declaration, GraphError> {
        match self {
            Self::Object(value) => Ok(value),
            Self::Component { .. } => Err(GraphError::UnresolvedComponent),
        }
    }

    pub(crate) fn as_object(&self) -> Result<&Declaration, GraphError> {
        match self {
            Self::Object(value) => Ok(value),
            Self::Component { .. } => Err(GraphError::UnresolvedComponent),
        }
    }
}

impl Declaration {
    fn new(kind: ObjectType) -> Self {
        Self {
            kind,
            key: None,
            component: None,
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
        let contracts = property_contracts(self.kind);
        self.properties.sort_by_key(|property| {
            contracts
                .iter()
                .position(|contract| contract.id == property.id)
                .unwrap()
        });
        self
    }

    pub(crate) fn relation(mut self, id: RelationId, value: RelationValue) -> Self {
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
pub struct Visual(pub(crate) DeclaredNode);

#[derive(Clone, Debug, PartialEq)]
pub struct KeyedVisual(Visual);

pub fn keyed(key: impl Into<Key>, visual: impl Into<Visual>) -> KeyedVisual {
    let mut visual = visual.into();
    visual.0 = visual.0.key(key);
    KeyedVisual(visual)
}

mod generated_declarations {
    use super::*;
    include!("generated_declarations.rs");
}

pub use generated_declarations::*;
