use std::fmt;
use std::rc::Rc;

use super::*;
use crate::core::{ComponentView, ContextProvision};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Key {
    Integer(u64),
    String(Rc<str>),
}

impl From<u64> for Key {
    fn from(value: u64) -> Self {
        Self::Integer(value)
    }
}

impl From<u32> for Key {
    fn from(value: u32) -> Self {
        Self::Integer(value.into())
    }
}

impl From<usize> for Key {
    fn from(value: usize) -> Self {
        Self::Integer(u64::try_from(value).unwrap())
    }
}

impl From<String> for Key {
    fn from(value: String) -> Self {
        Self::String(value.into())
    }
}

impl From<&str> for Key {
    fn from(value: &str) -> Self {
        Self::String(value.into())
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
}

impl View {
    pub fn empty() -> Self {
        Self::fragment([])
    }

    pub fn native(control: impl Into<Element>) -> Self {
        Self(ViewKind::Native(control.into()))
    }

    pub fn component<C: Component>(props: C::Props) -> Self {
        Self(ViewKind::Component(ComponentView::new::<C>(props)))
    }

    pub fn fragment(children: impl IntoIterator<Item = KeyedView>) -> Self {
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

    pub fn content<C>(control: C, content: impl Into<Self>) -> Self
    where
        C: ContentControl + Into<Element>,
    {
        Self(ViewKind::Content {
            control: control.into(),
            content: Box::new(content.into().into_kind()),
        })
    }

    pub fn children<C>(control: C, children: impl IntoIterator<Item = KeyedView>) -> Self
    where
        C: ChildrenControl + Into<Element>,
    {
        Self(ViewKind::Children {
            control: control.into(),
            children: Rc::new(children.into_iter().collect()),
        })
    }

    pub(crate) fn from_kind(kind: ViewKind) -> Self {
        Self(kind)
    }

    pub(crate) fn into_kind(self) -> ViewKind {
        self.0
    }
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

pub struct Callback<T>(Rc<dyn Fn(T)>);

impl<T> Callback<T> {
    pub fn new(callback: impl Fn(T) + 'static) -> Self {
        Self(Rc::new(callback))
    }

    pub fn call(&self, value: T) {
        (self.0)(value);
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

pub(crate) mod sealed {
    pub trait Sealed {}
}

pub trait LayoutControl: sealed::Sealed {}
pub trait TextStyleControl: sealed::Sealed {}
/// Marks controls that support enabled state.
///
/// ```compile_fail
/// use windows_reactor_next::TextBlock;
///
/// let _ = TextBlock::new().is_enabled(false);
/// ```
pub trait EnabledControl: sealed::Sealed {}
pub trait ContentControl: sealed::Sealed {}
pub trait ChildrenControl: sealed::Sealed {}
pub trait ControlledTextControl: sealed::Sealed {}
pub trait ItemsControl: sealed::Sealed {}
