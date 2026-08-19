use std::any::Any;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::element::{Color, CornerRadius, Thickness};

static NEXT_CONTEXT_ID: AtomicUsize = AtomicUsize::new(1);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct ContextId(usize);

impl ContextId {
    fn dynamic() -> Self {
        Self(
            NEXT_CONTEXT_ID
                .try_update(Ordering::Relaxed, Ordering::Relaxed, |id| id.checked_add(2))
                .unwrap_or_else(|_| panic!("context identity space exhausted")),
        )
    }

    fn static_key<T>(key: &'static ContextKey<T>) -> Self {
        let id = std::ptr::from_ref(&key.marker).addr();
        debug_assert_eq!(id & 1, 0);
        Self(id)
    }
}

#[derive(Clone)]
pub(crate) struct ContextEntry {
    pub id: ContextId,
    pub value: Rc<dyn Any>,
}

pub struct Context<T> {
    pub(crate) id: ContextId,
    pub(crate) default: Rc<T>,
}

impl<T> Context<T> {
    pub fn new(default: T) -> Self {
        Self {
            id: ContextId::dynamic(),
            default: Rc::new(default),
        }
    }
}

impl<T> Clone for Context<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            default: Rc::clone(&self.default),
        }
    }
}

pub struct ContextKey<T> {
    marker: AtomicUsize,
    default: fn() -> T,
    value: PhantomData<fn() -> T>,
}

impl<T> ContextKey<T> {
    pub const fn new(default: fn() -> T) -> Self {
        Self {
            marker: AtomicUsize::new(0),
            default,
            value: PhantomData,
        }
    }

    pub(crate) fn id(&'static self) -> ContextId {
        ContextId::static_key(self)
    }
}

#[derive(Default)]
pub(crate) struct ContextDefaults {
    values: RefCell<BTreeMap<ContextId, Rc<dyn Any>>>,
}

impl ContextDefaults {
    pub(crate) fn get<T>(&self, key: &'static ContextKey<T>) -> T
    where
        T: Clone + 'static,
    {
        let id = key.id();
        let mut values = self.values.borrow_mut();
        let value = values.entry(id).or_insert_with(|| Rc::new((key.default)()));
        value.downcast_ref::<T>().unwrap().clone()
    }
}

pub(crate) struct ContextProps {
    pub entry: ContextEntry,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ApplicationResource {
    String(String),
    Number(f64),
    Thickness(Thickness),
    CornerRadius(CornerRadius),
    SolidColorBrush(Color),
}

impl From<&str> for ApplicationResource {
    fn from(value: &str) -> Self {
        Self::String(value.into())
    }
}

impl From<String> for ApplicationResource {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<f64> for ApplicationResource {
    fn from(value: f64) -> Self {
        assert!(
            value.is_finite(),
            "application resource number must be finite"
        );
        Self::Number(value)
    }
}

impl From<Thickness> for ApplicationResource {
    fn from(value: Thickness) -> Self {
        Self::Thickness(value)
    }
}

impl From<CornerRadius> for ApplicationResource {
    fn from(value: CornerRadius) -> Self {
        Self::CornerRadius(value)
    }
}

impl From<Color> for ApplicationResource {
    fn from(value: Color) -> Self {
        Self::SolidColorBrush(value)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ApplicationResources(BTreeMap<String, ApplicationResource>);

impl ApplicationResources {
    pub fn new<K, V>(entries: impl IntoIterator<Item = (K, V)>) -> Self
    where
        K: Into<String>,
        V: Into<ApplicationResource>,
    {
        let mut resources = BTreeMap::new();
        for (key, value) in entries {
            let key = key.into();
            assert!(
                !key.is_empty(),
                "application resource key must not be empty"
            );
            assert!(
                resources.insert(key, value.into()).is_none(),
                "application resource keys must be unique"
            );
        }

        Self(resources)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn entries(&self) -> impl ExactSizeIterator<Item = (&str, &ApplicationResource)> {
        self.0.iter().map(|(key, value)| (key.as_str(), value))
    }

    pub(crate) fn get(&self, key: &str) -> Option<&ApplicationResource> {
        self.0.get(key)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ElementResources(ApplicationResources);

impl ElementResources {
    pub fn new<K, V>(entries: impl IntoIterator<Item = (K, V)>) -> Self
    where
        K: Into<String>,
        V: Into<ApplicationResource>,
    {
        Self(ApplicationResources::new(entries))
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn entries(&self) -> impl ExactSizeIterator<Item = (&str, &ApplicationResource)> {
        self.0.entries()
    }

    pub(crate) fn get(&self, key: &str) -> Option<&ApplicationResource> {
        self.0.get(key)
    }
}

impl IntoIterator for ApplicationResources {
    type Item = (String, ApplicationResource);
    type IntoIter = std::collections::btree_map::IntoIter<String, ApplicationResource>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
