use std::fmt;
use std::rc::Rc;

use super::*;
use crate::core::{ComponentView, ContextProvision};

pub(crate) mod sealed {
    pub trait Sealed {}

    pub trait StaticViews {
        fn into_views(self) -> Vec<super::View>;
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Key(KeyKind);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum KeyKind {
    Integer(u64),
    String(Rc<str>),
    Position(usize),
}

impl Key {
    pub(crate) fn position(value: usize) -> Self {
        Self(KeyKind::Position(value))
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
        Self(KeyKind::Integer(u64::try_from(value).unwrap()))
    }
}

impl From<String> for Key {
    fn from(value: String) -> Self {
        Self(KeyKind::String(value.into()))
    }
}

impl From<&str> for Key {
    fn from(value: &str) -> Self {
        Self(KeyKind::String(value.into()))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct KeyedElement {
    key: Key,
    element: Element,
}

impl KeyedElement {
    pub fn new(key: impl Into<Key>, element: impl Into<Element>) -> Self {
        Self {
            key: key.into(),
            element: element.into(),
        }
    }

    pub fn key(&self) -> &Key {
        &self.key
    }

    pub fn element(&self) -> &Element {
        &self.element
    }

    pub(crate) fn into_parts(self) -> (Key, Element) {
        (self.key, self.element)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct View(ViewKind);

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum ViewKind {
    Native(Element),
    Component(ComponentView),
    Fragment(Rc<Vec<KeyedView>>),
    Provider {
        provision: ContextProvision,
        child: Box<Self>,
    },
    Content {
        control: Element,
        content: Box<Self>,
    },
    Children {
        control: Element,
        children: Rc<Vec<KeyedView>>,
    },
    Slots {
        control: Element,
        slots: Rc<Vec<SlottedView>>,
    },
}

/// Converts a statically shaped expression into positional views.
///
/// This trait is sealed. `()` represents no views, fixed-size arrays represent homogeneous
/// shapes, and tuples represent heterogeneous shapes. Dynamic collections require
/// [`ChildrenControl::keyed_children`] or [`View::keyed_fragment`].
///
/// A `Vec` cannot supply positional children:
///
/// ```compile_fail
/// use windows_reactor_next::*;
///
/// let dynamic: Vec<View> = vec![TextBlock::new().into()];
/// let _ = StackPanel::new().children(dynamic);
/// ```
///
/// Iterator adapters cannot supply positional children:
///
/// ```compile_fail
/// use windows_reactor_next::*;
///
/// let dynamic = (0..3).map(|index| TextBlock::new().text(index.to_string()));
/// let _ = StackPanel::new().children(dynamic);
/// ```
pub trait IntoViews: sealed::StaticViews {}

impl View {
    pub fn empty() -> Self {
        Self::fragment(())
    }

    pub fn native(control: impl Into<Element>) -> Self {
        Self(ViewKind::Native(control.into()))
    }

    pub fn component<C: Component>(props: C::Props) -> Self {
        Self(ViewKind::Component(ComponentView::new::<C>(props)))
    }

    pub fn fragment(children: impl IntoViews) -> Self {
        Self(ViewKind::Fragment(positioned(children)))
    }

    pub fn keyed_fragment(children: impl IntoIterator<Item = KeyedView>) -> Self {
        Self(ViewKind::Fragment(Rc::new(children.into_iter().collect())))
    }

    pub fn provide<T>(context: &Context<T>, value: T, child: impl Into<Self>) -> Self
    where
        T: Clone + PartialEq + 'static,
    {
        Self(ViewKind::Provider {
            provision: ContextProvision::new(context, value),
            child: Box::new(child.into().into_kind()),
        })
    }

    pub(crate) fn from_kind(kind: ViewKind) -> Self {
        Self(kind)
    }

    pub(crate) fn into_kind(self) -> ViewKind {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SlotView<S> {
    slot: S,
    view: View,
}

impl<S> SlotView<S> {
    pub fn new(slot: S, view: impl Into<View>) -> Self {
        Self {
            slot,
            view: view.into(),
        }
    }

    fn into_parts(self) -> (S, View) {
        (self.slot, self.view)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SlottedView {
    pub(crate) slot: SlotId,
    pub(crate) view: View,
}

impl From<Element> for View {
    fn from(value: Element) -> Self {
        Self(ViewKind::Native(value))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct KeyedView {
    key: Key,
    view: View,
}

impl KeyedView {
    pub fn new(key: impl Into<Key>, view: impl Into<View>) -> Self {
        Self {
            key: key.into(),
            view: view.into(),
        }
    }

    pub fn key(&self) -> &Key {
        &self.key
    }

    pub fn view(&self) -> &View {
        &self.view
    }

    pub(crate) fn into_parts(self) -> (Key, View) {
        (self.key, self.view)
    }

    fn position(position: usize, view: View) -> Self {
        Self {
            key: Key::position(position),
            view,
        }
    }
}

fn positioned(children: impl IntoViews) -> Rc<Vec<KeyedView>> {
    let children = sealed::StaticViews::into_views(children);
    Rc::new(
        children
            .into_iter()
            .enumerate()
            .map(|(position, view)| KeyedView::position(position, view))
            .collect(),
    )
}

impl sealed::StaticViews for () {
    fn into_views(self) -> Vec<View> {
        Vec::new()
    }
}

impl IntoViews for () {}

impl<T, const N: usize> sealed::StaticViews for [T; N]
where
    T: Into<View>,
{
    fn into_views(self) -> Vec<View> {
        self.into_iter().map(Into::into).collect()
    }
}

impl<T, const N: usize> IntoViews for [T; N] where T: Into<View> {}

macro_rules! impl_into_views_tuple {
    ($($type:ident $index:tt),+ $(,)?) => {
        impl<$($type),+> sealed::StaticViews for ($($type,)+)
        where
            $($type: Into<View>,)+
        {
            fn into_views(self) -> Vec<View> {
                vec![$(self.$index.into()),+]
            }
        }

        impl<$($type),+> IntoViews for ($($type,)+)
        where
            $($type: Into<View>,)+
        {
        }
    };
}

impl_into_views_tuple!(A 0);
impl_into_views_tuple!(A 0, B 1);
impl_into_views_tuple!(A 0, B 1, C 2);
impl_into_views_tuple!(A 0, B 1, C 2, D 3);
impl_into_views_tuple!(A 0, B 1, C 2, D 3, E 4);
impl_into_views_tuple!(A 0, B 1, C 2, D 3, E 4, F 5);
impl_into_views_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6);
impl_into_views_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7);
impl_into_views_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8);
impl_into_views_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9);
impl_into_views_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10);
impl_into_views_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11);
impl_into_views_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12);
impl_into_views_tuple!(
    A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13
);
impl_into_views_tuple!(
    A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14
);
impl_into_views_tuple!(
    A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15
);

#[derive(Clone, Copy, Debug)]
pub enum GridLength {
    Auto,
    Pixel(f64),
    Star(f64),
}

impl GridLength {
    pub const STAR: Self = Self::Star(1.0);
}

impl PartialEq for GridLength {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Auto, Self::Auto) => true,
            (Self::Pixel(left), Self::Pixel(right)) | (Self::Star(left), Self::Star(right)) => {
                f64_eq(*left, *right)
            }
            _ => false,
        }
    }
}

#[doc(hidden)]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct GridPlacement {
    row: Option<i32>,
    column: Option<i32>,
    row_span: Option<i32>,
    column_span: Option<i32>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum Property<T> {
    #[default]
    Inherited,
    Set(T),
}

impl<T> Property<T> {
    pub fn as_set(&self) -> Option<&T> {
        match self {
            Self::Inherited => None,
            Self::Set(value) => Some(value),
        }
    }
}

pub(crate) fn f64_eq(left: f64, right: f64) -> bool {
    left == right || left.is_nan() && right.is_nan()
}

pub(crate) fn f64_property_eq(left: &Property<f64>, right: &Property<f64>) -> bool {
    match (left, right) {
        (Property::Inherited, Property::Inherited) => true,
        (Property::Set(left), Property::Set(right)) => f64_eq(*left, *right),
        _ => false,
    }
}

pub struct Callback<T>(Rc<dyn Fn(T) -> bool>);

impl<T> Callback<T> {
    pub fn new(callback: impl Fn(T) + 'static) -> Self {
        Self::new_with_acceptance(move |value| {
            callback(value);
            true
        })
    }

    pub(crate) fn new_with_acceptance(callback: impl Fn(T) -> bool + 'static) -> Self {
        Self(Rc::new(callback))
    }

    #[must_use = "false means the adapted message was rejected"]
    pub fn call(&self, value: T) -> bool {
        (self.0)(value)
    }
}

impl<T> Clone for Callback<T> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl<T> fmt::Debug for Callback<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("Callback")
            .field(&Rc::as_ptr(&self.0))
            .finish()
    }
}

impl<T> PartialEq for Callback<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

/// Converts a payload handler or typed message callback into an event callback.
pub trait IntoPayloadCallback<T> {
    fn into_payload_callback(self) -> Callback<T>;
}

impl<T, F> IntoPayloadCallback<T> for F
where
    F: Fn(T) + 'static,
{
    fn into_payload_callback(self) -> Callback<T> {
        Callback::new(self)
    }
}

impl<T> IntoPayloadCallback<T> for Callback<T> {
    fn into_payload_callback(self) -> Self {
        self
    }
}

/// Converts a zero-argument handler or typed message callback into an event callback.
pub trait IntoUnitCallback {
    fn into_unit_callback(self) -> Callback<()>;
}

impl<F> IntoUnitCallback for F
where
    F: Fn() + 'static,
{
    fn into_unit_callback(self) -> Callback<()> {
        Callback::new(move |()| self())
    }
}

impl IntoUnitCallback for Callback<()> {
    fn into_unit_callback(self) -> Self {
        self
    }
}

pub trait LayoutControl: sealed::Sealed {
    #[doc(hidden)]
    fn grid_placement_mut(&mut self) -> &mut Option<Rc<GridPlacement>>;
}
pub trait TextStyleControl: sealed::Sealed {}
/// Marks controls that support enabled state.
///
/// ```compile_fail
/// use windows_reactor_next::TextBlock;
///
/// let _ = TextBlock::new().is_enabled(false);
/// ```
pub trait EnabledControl: sealed::Sealed {}
pub trait ContentControl: sealed::Sealed + Into<Element> + Sized {
    fn content(self, content: impl Into<View>) -> View {
        View(ViewKind::Content {
            control: self.into(),
            content: Box::new(content.into().into_kind()),
        })
    }
}
pub trait ChildrenControl: sealed::Sealed + Into<Element> + Sized {
    fn children(self, children: impl IntoViews) -> View {
        View(ViewKind::Children {
            control: self.into(),
            children: positioned(children),
        })
    }

    fn keyed_children(self, children: impl IntoIterator<Item = KeyedView>) -> View {
        View(ViewKind::Children {
            control: self.into(),
            children: Rc::new(children.into_iter().collect()),
        })
    }
}
pub trait SlotsControl: sealed::Sealed + Into<Element> + Sized {
    type Slot: Copy;

    fn slots(self, slots: impl IntoIterator<Item = SlotView<Self::Slot>>) -> View {
        let control = self.into();
        let kind = control.kind();
        let slots = slots
            .into_iter()
            .map(|slot| {
                let (slot, view) = slot.into_parts();
                SlottedView {
                    slot: slot_id(kind, Self::slot_index(slot)).unwrap(),
                    view,
                }
            })
            .collect();
        View(ViewKind::Slots {
            control,
            slots: Rc::new(slots),
        })
    }

    #[doc(hidden)]
    fn slot_index(slot: Self::Slot) -> u8;
}
pub trait ControlledTextControl: sealed::Sealed {}
pub trait ItemsControl: sealed::Sealed {}
pub trait GridDefinitionsControl: sealed::Sealed {}

/// Places a concrete native control in its parent Grid.
///
/// Components and fragments can produce more than one native root, so place a native wrapper when
/// a composed view needs Grid placement.
///
/// ```compile_fail
/// use windows_reactor_next::*;
///
/// struct Child;
/// # impl Component for Child {
/// #     type Message = ();
/// #     type Props = ();
/// #     fn create(_: &(), _: &mut ComponentContext<Self>) -> Self { Self }
/// #     fn update(&mut self, _: (), _: &mut ComponentContext<Self>) {}
/// #     fn view(&self, _: &(), _: &mut ViewContext<Self>) -> View { View::empty() }
/// # }
/// let _ = View::component::<Child>(()).grid_row(0);
/// ```
///
/// ```compile_fail
/// use windows_reactor_next::*;
///
/// let _ = View::fragment((TextBlock::new(), TextBlock::new())).grid_column(0);
/// ```
pub trait GridChildExt: LayoutControl + Sized {
    fn grid_row(mut self, row: i32) -> Self {
        assert!(row >= 0, "Grid row must be non-negative");
        Rc::make_mut(
            self.grid_placement_mut()
                .get_or_insert_with(|| Rc::new(GridPlacement::default())),
        )
        .row = Some(row);
        self
    }

    fn grid_column(mut self, column: i32) -> Self {
        assert!(column >= 0, "Grid column must be non-negative");
        Rc::make_mut(
            self.grid_placement_mut()
                .get_or_insert_with(|| Rc::new(GridPlacement::default())),
        )
        .column = Some(column);
        self
    }

    fn grid_row_span(mut self, span: i32) -> Self {
        assert!(span > 0, "Grid row span must be positive");
        Rc::make_mut(
            self.grid_placement_mut()
                .get_or_insert_with(|| Rc::new(GridPlacement::default())),
        )
        .row_span = Some(span);
        self
    }

    fn grid_column_span(mut self, span: i32) -> Self {
        assert!(span > 0, "Grid column span must be positive");
        Rc::make_mut(
            self.grid_placement_mut()
                .get_or_insert_with(|| Rc::new(GridPlacement::default())),
        )
        .column_span = Some(span);
        self
    }
}

impl<T: LayoutControl> GridChildExt for T {}

pub(crate) fn visit_grid_placement(
    placement: Option<&GridPlacement>,
    visit: &mut dyn FnMut(PropertyId, Option<PropertyValue>),
) {
    let value = |value: Option<i32>| value.map(PropertyValue::I32);
    visit(
        PropertyId::GridRow,
        value(placement.and_then(|value| value.row)),
    );
    visit(
        PropertyId::GridColumn,
        value(placement.and_then(|value| value.column)),
    );
    visit(
        PropertyId::GridRowSpan,
        value(placement.and_then(|value| value.row_span)),
    );
    visit(
        PropertyId::GridColumnSpan,
        value(placement.and_then(|value| value.column_span)),
    );
}
