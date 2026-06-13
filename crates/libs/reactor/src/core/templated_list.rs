use std::any::TypeId;
use std::rc::Rc;

use super::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum TemplatedKind {
    ListView,
    GridView,
    FlipView,
}

/// Selection behaviour for templated lists (ListView, GridView).
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub enum SelectionMode {
    /// No selection allowed.
    None,
    /// Exactly one item may be selected at a time (default).
    #[default]
    Single,
    /// Multiple items may be selected via Ctrl+Click.
    Multiple,
    /// Range selection via Shift+Click and Ctrl+Click.
    Extended,
}

/// Erased adapter exposing the items of a [`TemplatedListElement`] to the
/// reconciler without leaking the concrete item type.
pub trait TemplatedListImpl: 'static {
    fn item_count(&self) -> usize;

    fn build_item_view(&self, index: usize) -> Element;

    fn item_key(&self, index: usize) -> Option<String>;

    fn selected_index(&self) -> i32;

    fn invoke_selection_changed(&self, index: i32);

    fn item_type_id(&self) -> TypeId;

    fn items_ptr(&self) -> *const ();

    fn has_selection_handler(&self) -> bool;

    fn raw_selection_callback(&self) -> Option<Callback<i32>>;
}

struct TemplatedListCell<T: 'static> {
    items: Rc<Vec<T>>,
    view_builder: Rc<dyn Fn(&T, usize) -> Element>,
    key_selector: Option<Rc<dyn Fn(&T) -> String>>,
    on_selection_changed: Option<Callback<i32>>,
    selected_index: i32,
}

impl<T: 'static> TemplatedListImpl for TemplatedListCell<T> {
    fn item_count(&self) -> usize {
        self.items.len()
    }

    fn build_item_view(&self, index: usize) -> Element {
        match self.items.get(index) {
            Some(item) => (self.view_builder)(item, index),
            None => Element::Empty,
        }
    }

    fn item_key(&self, index: usize) -> Option<String> {
        let item = self.items.get(index)?;
        self.key_selector.as_ref().map(|k| k(item))
    }

    fn selected_index(&self) -> i32 {
        self.selected_index
    }

    fn invoke_selection_changed(&self, index: i32) {
        if let Some(cb) = &self.on_selection_changed {
            cb.invoke(index);
        }
    }

    fn item_type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }

    fn items_ptr(&self) -> *const () {
        Rc::as_ptr(&self.items) as *const ()
    }

    fn has_selection_handler(&self) -> bool {
        self.on_selection_changed.is_some()
    }

    fn raw_selection_callback(&self) -> Option<Callback<i32>> {
        self.on_selection_changed.clone()
    }
}

/// Virtualised list/grid/flip element backed by a [`TemplatedListImpl`].
pub struct TemplatedListElement {
    pub key: Option<String>,
    pub kind: TemplatedKind,
    pub selection_mode: SelectionMode,
    pub can_drag_items: bool,
    pub can_reorder_items: bool,
    pub allow_drop: bool,
    pub modifiers: Modifiers,
    pub items_impl: Rc<dyn TemplatedListImpl>,
}

impl Clone for TemplatedListElement {
    fn clone(&self) -> Self {
        Self {
            key: self.key.clone(),
            kind: self.kind,
            selection_mode: self.selection_mode,
            can_drag_items: self.can_drag_items,
            can_reorder_items: self.can_reorder_items,
            allow_drop: self.allow_drop,
            modifiers: self.modifiers.clone(),
            items_impl: Rc::clone(&self.items_impl),
        }
    }
}

impl std::fmt::Debug for TemplatedListElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TemplatedListElement")
            .field("key", &self.key)
            .field("kind", &self.kind)
            .field("item_count", &self.items_impl.item_count())
            .finish()
    }
}

impl PartialEq for TemplatedListElement {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
            && self.kind == other.kind
            && self.selection_mode == other.selection_mode
            && self.can_drag_items == other.can_drag_items
            && self.can_reorder_items == other.can_reorder_items
            && self.allow_drop == other.allow_drop
            && self.modifiers == other.modifiers
            && Rc::ptr_eq(&self.items_impl, &other.items_impl)
    }
}

impl TemplatedListElement {
    pub fn same_items_as(&self, other: &TemplatedListElement) -> bool {
        self.items_impl.item_type_id() == other.items_impl.item_type_id()
            && self.items_impl.items_ptr() == other.items_impl.items_ptr()
    }

    pub fn item_count(&self) -> usize {
        self.items_impl.item_count()
    }

    pub fn build_item_view(&self, index: usize) -> Element {
        self.items_impl.build_item_view(index)
    }

    pub fn item_key(&self, index: usize) -> Option<String> {
        self.items_impl.item_key(index)
    }

    pub fn selected_index(&self) -> i32 {
        self.items_impl.selected_index()
    }

    pub fn invoke_selection_changed(&self, index: i32) {
        self.items_impl.invoke_selection_changed(index);
    }

    pub fn has_selection_handler(&self) -> bool {
        self.items_impl.has_selection_handler()
    }

    pub fn raw_selection_callback(&self) -> Option<Callback<i32>> {
        self.items_impl.raw_selection_callback()
    }

    pub fn fully_keyed(&self) -> bool {
        (0..self.item_count()).all(|i| self.items_impl.item_key(i).is_some())
    }

    pub fn collect_keys(&self) -> Vec<String> {
        (0..self.item_count())
            .map(|i| {
                self.items_impl
                    .item_key(i)
                    .unwrap_or_else(|| format!("__pos_{i}"))
            })
            .collect()
    }
}

/// Fluent builder for [`TemplatedListElement`]; pick a kind via
/// [`list_view`] / [`grid_view`] / [`flip_view`].
pub struct TemplatedListBuilder<T: 'static> {
    kind: TemplatedKind,
    items: Rc<Vec<T>>,
    view_builder: Rc<dyn Fn(&T, usize) -> Element>,
    key_selector: Option<Rc<dyn Fn(&T) -> String>>,
    on_selection_changed: Option<Callback<i32>>,
    selected_index: i32,
    selection_mode: SelectionMode,
    can_drag_items: bool,
    can_reorder_items: bool,
    allow_drop: bool,
    modifiers: Modifiers,
    element_key: Option<String>,
}

impl<T: 'static> TemplatedListBuilder<T> {
    fn new<R: Into<Element>>(
        kind: TemplatedKind,
        items: Vec<T>,
        view: impl Fn(&T, usize) -> R + 'static,
    ) -> Self {
        Self {
            kind,
            items: Rc::new(items),
            view_builder: Rc::new(move |item, idx| view(item, idx).into()),
            key_selector: None,
            on_selection_changed: None,
            selected_index: -1,
            selection_mode: SelectionMode::Single,
            can_drag_items: false,
            can_reorder_items: false,
            allow_drop: false,
            modifiers: Modifiers::default(),
            element_key: None,
        }
    }

    pub fn with_key_selector(mut self, k: impl Fn(&T) -> String + 'static) -> Self {
        self.key_selector = Some(Rc::new(k));
        self
    }

    pub fn selected_index(mut self, i: i32) -> Self {
        self.selected_index = i;
        self
    }

    pub fn on_selection_changed(mut self, cb: impl Fn(i32) + 'static) -> Self {
        self.on_selection_changed = Some(Callback::new(cb));
        self
    }

    pub fn selection_mode(mut self, mode: SelectionMode) -> Self {
        self.selection_mode = mode;
        self
    }

    pub fn can_drag_items(mut self, v: bool) -> Self {
        self.can_drag_items = v;
        self
    }

    pub fn can_reorder_items(mut self, v: bool) -> Self {
        self.can_reorder_items = v;
        self
    }

    pub fn allow_drop(mut self, v: bool) -> Self {
        self.allow_drop = v;
        self
    }

    pub fn with_key(mut self, k: impl Into<String>) -> Self {
        self.element_key = Some(k.into());
        self
    }

    pub fn width(mut self, v: f64) -> Self {
        self.modifiers.width = Some(v);
        self
    }

    pub fn height(mut self, v: f64) -> Self {
        self.modifiers.height = Some(v);
        self
    }

    pub fn build(self) -> Element {
        let cell = TemplatedListCell::<T> {
            items: self.items,
            view_builder: self.view_builder,
            key_selector: self.key_selector,
            on_selection_changed: self.on_selection_changed,
            selected_index: self.selected_index,
        };
        Element::TemplatedList(TemplatedListElement {
            key: self.element_key,
            kind: self.kind,
            selection_mode: self.selection_mode,
            can_drag_items: self.can_drag_items,
            can_reorder_items: self.can_reorder_items,
            allow_drop: self.allow_drop,
            modifiers: self.modifiers,
            items_impl: Rc::new(cell),
        })
    }
}

impl<T: 'static> From<TemplatedListBuilder<T>> for Element {
    fn from(b: TemplatedListBuilder<T>) -> Self {
        b.build()
    }
}

pub fn list_view<T: 'static, R: Into<Element>>(
    items: Vec<T>,
    view: impl Fn(&T, usize) -> R + 'static,
) -> TemplatedListBuilder<T> {
    TemplatedListBuilder::new(TemplatedKind::ListView, items, view)
}

pub fn grid_view<T: 'static, R: Into<Element>>(
    items: Vec<T>,
    view: impl Fn(&T, usize) -> R + 'static,
) -> TemplatedListBuilder<T> {
    TemplatedListBuilder::new(TemplatedKind::GridView, items, view)
}

pub fn flip_view<T: 'static, R: Into<Element>>(
    items: Vec<T>,
    view: impl Fn(&T, usize) -> R + 'static,
) -> TemplatedListBuilder<T> {
    TemplatedListBuilder::new(TemplatedKind::FlipView, items, view)
}

/// Alias for [`list_view`] — exposes the same virtualised
/// `Microsoft.UI.Xaml.Controls.ListView` adapter under the name used in
/// the public roadmap (W1). Convert / route the result the same way as
/// any other [`Element`].
pub fn virtual_list<T: 'static, R: Into<Element>>(
    items: Vec<T>,
    view: impl Fn(&T, usize) -> R + 'static,
) -> TemplatedListBuilder<T> {
    list_view(items, view)
}
