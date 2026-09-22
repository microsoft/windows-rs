use super::*;
use std::fmt;
use std::rc::Rc;
use std::time::Duration;

fn canonical_rich_edit_text(value: Rc<str>) -> Rc<str> {
    if value.contains('\r') {
        Rc::from(value.replace("\r\n", "\n").replace('\r', "\n"))
    } else {
        value
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExitTransition {
    duration: Duration,
}

impl ExitTransition {
    pub fn fade(duration: Duration) -> Option<Self> {
        (!duration.is_zero()).then_some(Self { duration })
    }

    pub const fn duration(self) -> Duration {
        self.duration
    }
}

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

    pub fn is_finite_non_negative(self) -> bool {
        [
            self.top_left,
            self.top_right,
            self.bottom_right,
            self.bottom_left,
        ]
        .into_iter()
        .all(|value| value.is_finite() && value >= 0.0)
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

    pub fn is_finite(self) -> bool {
        [self.left, self.top, self.right, self.bottom]
            .into_iter()
            .all(f64::is_finite)
    }

    pub fn is_finite_non_negative(self) -> bool {
        [self.left, self.top, self.right, self.bottom]
            .into_iter()
            .all(|value| value.is_finite() && value >= 0.0)
    }
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ThemeTransition {
    Reposition,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum TooltipPlacement {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
    Mouse,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Tooltip {
    content: Box<Visual>,
    placement: TooltipPlacement,
}

impl Tooltip {
    pub fn text(value: impl Into<Rc<str>>) -> Self {
        Self::rich(TextBlock::new().text(value))
    }

    pub fn rich(content: impl Into<Visual>) -> Self {
        Self {
            content: Box::new(content.into()),
            placement: TooltipPlacement::Top,
        }
    }

    pub fn placement(mut self, placement: TooltipPlacement) -> Self {
        self.placement = placement;
        self
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FontWeight(u16);

impl FontWeight {
    pub const BLACK: Self = Self(900);
    pub const BOLD: Self = Self(700);
    pub const EXTRA_BLACK: Self = Self(950);
    pub const EXTRA_BOLD: Self = Self(800);
    pub const EXTRA_LIGHT: Self = Self(200);
    pub const LIGHT: Self = Self(300);
    pub const MEDIUM: Self = Self(500);
    pub const NORMAL: Self = Self(400);
    pub const SEMI_BOLD: Self = Self(600);
    pub const SEMI_LIGHT: Self = Self(350);
    pub const THIN: Self = Self(100);

    pub const fn new(weight: u16) -> Option<Self> {
        if weight >= 1 && weight <= 999 {
            Some(Self(weight))
        } else {
            None
        }
    }

    pub(crate) const fn value(self) -> u16 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GridLength {
    pub(crate) size: GridLengthSize,
    pub(crate) min: Option<f64>,
    pub(crate) max: Option<f64>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum GridLengthSize {
    Auto,
    Pixel(f64),
    Star(f64),
}

#[expect(non_upper_case_globals, non_snake_case)]
impl GridLength {
    pub const Auto: Self = Self::new(GridLengthSize::Auto);
    pub const STAR: Self = Self::new(GridLengthSize::Star(1.0));

    const fn new(size: GridLengthSize) -> Self {
        Self {
            size,
            min: None,
            max: None,
        }
    }

    pub const fn Pixel(value: f64) -> Self {
        assert_grid_length(value);
        Self::new(GridLengthSize::Pixel(value))
    }

    pub const fn Star(value: f64) -> Self {
        assert_grid_length(value);
        Self::new(GridLengthSize::Star(value))
    }

    pub fn min(mut self, value: f64) -> Self {
        assert_grid_length(value);
        assert!(self.max.is_none_or(|max| value <= max));
        self.min = Some(value);
        self
    }

    pub fn max(mut self, value: f64) -> Self {
        assert_grid_length(value);
        assert!(self.min.is_none_or(|min| min <= value));
        self.max = Some(value);
        self
    }
}

const fn assert_grid_length(value: f64) {
    assert!(value.is_finite() && value >= 0.0);
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
        Self(KeyKind::Integer(value as u64))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PropertyValue {
    String(Rc<str>),
    Bool(bool),
    Color(Color),
    CornerRadius(CornerRadius),
    F64(f64),
    FontWeight(FontWeight),
    GridLengths(Rc<[GridLength]>),
    I32(i32),
    OptionalF64(Option<f64>),
    OptionalBool(Option<bool>),
    SelectionIndex(Option<usize>),
    StringList(Rc<[Rc<str>]>),
    ThemeTransitions(Rc<[ThemeTransition]>),
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

/// Pointer state in element-local and window-relative device-independent pixels.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PointerEventInfo {
    pub x: f64,
    pub y: f64,
    pub window_x: f64,
    pub window_y: f64,
    pub pointer_id: u32,
    pub capture_succeeded: Option<bool>,
    pub is_captured: bool,
    pub is_left_button_pressed: bool,
    pub is_right_button_pressed: bool,
    pub is_middle_button_pressed: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum EventValue {
    Bool(Callback<bool>),
    String(Callback<Rc<str>>),
    F64(Callback<f64>),
    OptionalBool(Callback<Option<bool>>),
    OptionalF64(Callback<Option<f64>>),
    PointerEventInfo(Callback<PointerEventInfo>),
    Selection(Callback<Option<Rc<str>>>),
    SelectionIndex(Callback<Option<usize>>),
    Unit(Callback<()>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum EventPayload {
    Bool(bool),
    String(Rc<str>),
    F64(f64),
    OptionalBool(Option<bool>),
    OptionalF64(Option<f64>),
    PointerEventInfo(PointerEventInfo),
    Selection(SelectionChange),
    SelectionIndex(Option<usize>),
    Unit,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SelectionChange {
    pub item: Option<ObjectId>,
    pub value: Option<Rc<str>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Event {
    pub id: EventId,
    pub value: EventValue,
}

/// A lazily materialized keyed source for a virtualizing declaration.
///
/// ```
/// use windows_reactor2::{ItemsRepeater, Key, TextBlock, VirtualSource, Visual};
///
/// let source = VirtualSource::new(
///     7,
///     10_000,
///     Key::from,
///     |index| -> Visual { TextBlock::new().text(index.to_string()).into() },
/// );
/// let view: Visual = ItemsRepeater::new().virtual_source(source).into();
/// ```
#[derive(Clone)]
pub struct VirtualSource {
    pub(crate) key_revision: u64,
    pub(crate) len: usize,
    pub(crate) key: Rc<dyn Fn(usize) -> Key>,
    pub(crate) view: Rc<dyn Fn(usize) -> Visual>,
}

impl VirtualSource {
    pub fn new<K, V, KI, VI>(key_revision: u64, len: usize, key: K, view: V) -> Self
    where
        K: Fn(usize) -> KI + 'static,
        V: Fn(usize) -> VI + 'static,
        KI: Into<Key>,
        VI: Into<Visual>,
    {
        Self {
            key_revision,
            len,
            key: Rc::new(move |index| key(index).into()),
            view: Rc::new(move |index| view(index).into()),
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn key_revision(&self) -> u64 {
        self.key_revision
    }
}

impl fmt::Debug for VirtualSource {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("VirtualSource")
            .field("key_revision", &self.key_revision)
            .field("len", &self.len)
            .finish_non_exhaustive()
    }
}

impl PartialEq for VirtualSource {
    fn eq(&self, other: &Self) -> bool {
        self.key_revision == other.key_revision
            && self.len == other.len
            && Rc::ptr_eq(&self.key, &other.key)
            && Rc::ptr_eq(&self.view, &other.view)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum VirtualItems {
    Eager(Rc<Vec<(Key, Visual)>>),
    Lazy(VirtualSource),
}

impl VirtualItems {
    pub(crate) fn eager(items: impl IntoIterator<Item = KeyedVisual>) -> Self {
        Self::Eager(Rc::new(
            items
                .into_iter()
                .map(|item| {
                    let visual = item.0;
                    let key = visual.0.key_ref().clone();
                    (key, visual)
                })
                .collect(),
        ))
    }

    pub(crate) fn len(&self) -> usize {
        match self {
            Self::Eager(items) => items.len(),
            Self::Lazy(source) => source.len,
        }
    }

    pub(crate) fn key(&self, index: usize) -> Option<Key> {
        match self {
            Self::Eager(items) => items.get(index).map(|(key, _)| key.clone()),
            Self::Lazy(source) => (index < source.len).then(|| (source.key)(index)),
        }
    }

    pub(crate) fn view(&self, index: usize) -> Option<Visual> {
        match self {
            Self::Eager(items) => items.get(index).map(|(_, view)| view.clone()),
            Self::Lazy(source) => (index < source.len).then(|| (source.view)(index)),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct DeclaredVirtualItems {
    pub(crate) relation: RelationId,
    pub(crate) owner: Option<ComponentId>,
    pub(crate) items: VirtualItems,
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
pub(crate) struct DeclaredTooltip {
    pub content: Box<DeclaredNode>,
    pub placement: TooltipPlacement,
}

impl DeclaredTooltip {
    pub(crate) fn declaration(&self) -> Declaration {
        Declaration::new(ObjectType::ToolTip).relation(
            RelationId::Content,
            RelationValue::One(Some(Rc::new(self.content.as_ref().clone()))),
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Declaration {
    pub kind: ObjectType,
    pub key: Option<Key>,
    pub component: Option<ComponentId>,
    pub reference: Option<ElementRef>,
    pub exit_transition: Option<ExitTransition>,
    pub window_title_bar: Option<WindowTitleBarHeight>,
    pub tooltip: Option<Box<DeclaredTooltip>>,
    pub properties: SharedList<Property>,
    pub events: SharedList<Event>,
    pub relations: SharedList<DeclaredRelation>,
    pub virtual_items: Option<Box<DeclaredVirtualItems>>,
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
        if let Some(tooltip) = self.tooltip.take() {
            pending.push(DropFrame::One(Some(*tooltip.content)));
        }
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
        tooltip: Option<Box<DeclaredTooltip>>,
    },
}

impl fmt::Debug for DeclaredNode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Object(value) => value.fmt(formatter),
            Self::Component {
                node,
                relation_key,
                tooltip,
            } => formatter
                .debug_struct("Component")
                .field("key", &node.key)
                .field("relation_key", relation_key)
                .field("tooltip", tooltip)
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
                    tooltip: left_tooltip,
                },
                Self::Component {
                    node: right,
                    relation_key: right_key,
                    tooltip: right_tooltip,
                },
            ) => left == right && left_key == right_key && left_tooltip == right_tooltip,
            _ => false,
        }
    }
}

impl DeclaredNode {
    fn key(self, key: impl Into<Key>) -> Self {
        let key = key.into();
        match self {
            Self::Object(value) => Self::Object(value.key(key)),
            Self::Component { node, tooltip, .. } => Self::Component {
                node,
                relation_key: Some(key),
                tooltip,
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

    fn key_ref(&self) -> &Key {
        match self {
            Self::Object(value) => {
                let Some(key) = value.key.as_ref() else {
                    unreachable!("KeyedVisual object without a key");
                };
                key
            }
            Self::Component {
                node, relation_key, ..
            } => relation_key.as_ref().unwrap_or(&node.key),
        }
    }
}

impl Declaration {
    fn new(kind: ObjectType) -> Self {
        Self {
            kind,
            key: None,
            component: None,
            reference: None,
            exit_transition: None,
            window_title_bar: None,
            tooltip: None,
            properties: SharedList::Empty,
            events: SharedList::Empty,
            relations: SharedList::Empty,
            virtual_items: None,
        }
    }

    fn key(mut self, key: impl Into<Key>) -> Self {
        self.key = Some(key.into());
        self
    }

    fn property(mut self, id: PropertyId, value: PropertyValue) -> Self {
        self.properties
            .upsert(|property| property.id == id, Property { id, value });
        self.properties
            .sort_by_key(|property| property_order(self.kind, property.id));
        self
    }

    fn window_title_bar(mut self, height: WindowTitleBarHeight) -> Self {
        self.window_title_bar = Some(height);
        self
    }

    pub(crate) fn relation(mut self, id: RelationId, value: RelationValue) -> Self {
        self.relations
            .upsert(|relation| relation.id == id, DeclaredRelation { id, value });
        self
    }

    pub(crate) fn virtual_items(mut self, relation: RelationId, items: VirtualItems) -> Self {
        self.virtual_items = Some(Box::new(DeclaredVirtualItems {
            relation,
            owner: None,
            items,
        }));
        self
    }

    pub(crate) fn virtual_item(mut self, relation: RelationId, item: KeyedVisual) -> Self {
        let visual = item.0;
        let item = (visual.0.key_ref().clone(), visual);
        let items = match self.virtual_items.take() {
            Some(items)
                if matches!(
                    items.as_ref(),
                    DeclaredVirtualItems {
                        relation: current,
                        items: VirtualItems::Eager(_),
                        ..
                    } if *current == relation
                ) =>
            {
                let DeclaredVirtualItems {
                    owner,
                    items: VirtualItems::Eager(items),
                    ..
                } = *items
                else {
                    unreachable!()
                };
                let mut items = items.as_ref().clone();
                items.push(item);
                self.virtual_items = Some(Box::new(DeclaredVirtualItems {
                    relation,
                    owner,
                    items: VirtualItems::Eager(Rc::new(items)),
                }));
                return self;
            }
            _ => Rc::new(vec![item]),
        };
        self.virtual_items = Some(Box::new(DeclaredVirtualItems {
            relation,
            owner: None,
            items: VirtualItems::Eager(items),
        }));
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

pub trait TooltipExt: Into<Visual> + Sized {
    fn tooltip(self, value: impl Into<Rc<str>>) -> Visual {
        self.tooltip_with(Tooltip::text(value))
    }

    fn tooltip_with(self, tooltip: Tooltip) -> Visual {
        let mut visual = self.into();
        let tooltip = DeclaredTooltip {
            content: Box::new(tooltip.content.0),
            placement: tooltip.placement,
        };
        match &mut visual.0 {
            DeclaredNode::Object(declaration) => declaration.tooltip = Some(Box::new(tooltip)),
            DeclaredNode::Component {
                tooltip: current, ..
            } => *current = Some(Box::new(tooltip)),
        }
        visual
    }
}

impl<T> TooltipExt for T where T: Into<Visual> {}

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
