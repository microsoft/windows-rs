use std::rc::Rc;

use crate::element::Framework;
use crate::element::props::*;
use crate::element::tree::*;
use crate::element::values::*;
use crate::element::{
    Element, KeyEventFn, KeysEventFn, OptionalKeyEventFn, RowFn, SelectionEventFn,
    enforce_display_only,
};
use crate::framework_properties::FrameworkProps;
struct VirtualCollection {
    kind: VirtualCollectionKind,
    items: VirtualCollectionItems,
    height: f64,
    empty: Option<Box<Element>>,
    automation_name: Option<String>,
    help_text: Option<String>,
    selection_mode: SelectionMode,
    selection: CollectionSelection,
    on_selection_changed: Option<SelectionEventFn>,
    on_item_invoked: Option<KeyEventFn>,
    selection_display_only: bool,
    can_reorder_items: bool,
    on_items_reordered: Option<KeysEventFn>,
    row: RowFn,
}

pub struct VirtualList(VirtualCollection);

pub struct VirtualGrid(VirtualCollection);

pub struct ListBox {
    props: ListBoxProps,
}

pub struct ComboBox {
    props: ComboBoxProps,
}

pub struct RadioButtons {
    props: RadioButtonsProps,
}

impl ListBox {
    pub fn new<T: Into<ListBoxItem>>(
        items: impl IntoIterator<Item = T>,
        on_selection_changed: impl Fn(CollectionSelection) + 'static,
    ) -> Framework<Self> {
        Self::from_items(ListBoxItems::new(items), on_selection_changed)
    }

    pub fn display<T: Into<ListBoxItem>>(items: impl IntoIterator<Item = T>) -> Framework<Self> {
        Self::display_items(ListBoxItems::new(items))
    }

    pub fn from_items(
        items: ListBoxItems,
        on_selection_changed: impl Fn(CollectionSelection) + 'static,
    ) -> Framework<Self> {
        Framework::new(Self::with_handler(
            items,
            Some(Rc::new(on_selection_changed)),
        ))
    }

    pub fn display_items(items: ListBoxItems) -> Framework<Self> {
        Framework::new(Self::with_handler(items, None))
    }

    fn with_handler(items: ListBoxItems, on_selection_changed: Option<SelectionEventFn>) -> Self {
        Self {
            props: ListBoxProps {
                items,
                selection_mode: SelectionMode::Single,
                selection: CollectionSelection::default(),
                on_selection_changed,
                framework: FrameworkProps::default(),
            },
        }
    }

    pub(crate) fn build_with_framework(mut self, framework: FrameworkProps) -> Element {
        assert!(
            self.props.selection_mode != SelectionMode::Single || self.props.selection.len() <= 1,
            "ListBox single-selection mode accepts at most one selected key"
        );
        let mut framework = framework;
        if self.props.on_selection_changed.is_none() {
            enforce_display_only(&mut framework);
        }
        self.props.framework = framework;
        Element::new(ElementKind::ListBox(self.props))
    }
}

impl ComboBox {
    pub fn new<T: Into<SelectorItem>>(
        items: impl IntoIterator<Item = T>,
        on_selection_changed: impl Fn(Option<u64>) + 'static,
    ) -> Framework<Self> {
        Self::from_items(SelectorItems::new(items), on_selection_changed)
    }

    pub fn display<T: Into<SelectorItem>>(items: impl IntoIterator<Item = T>) -> Framework<Self> {
        Self::display_items(SelectorItems::new(items))
    }

    pub fn from_items(
        items: SelectorItems,
        on_selection_changed: impl Fn(Option<u64>) + 'static,
    ) -> Framework<Self> {
        Framework::new(Self::with_handler(
            items,
            Some(Rc::new(on_selection_changed)),
        ))
    }

    pub fn display_items(items: SelectorItems) -> Framework<Self> {
        Framework::new(Self::with_handler(items, None))
    }

    fn with_handler(
        items: SelectorItems,
        on_selection_changed: Option<OptionalKeyEventFn>,
    ) -> Self {
        Self {
            props: ComboBoxProps {
                items,
                header: None,
                placeholder: None,
                editable: false,
                selected_key: None,
                on_selection_changed,
                framework: FrameworkProps::default(),
            },
        }
    }

    pub(crate) fn build_with_framework(mut self, framework: FrameworkProps) -> Element {
        let mut framework = framework;
        if self.props.on_selection_changed.is_none() {
            enforce_display_only(&mut framework);
        }
        self.props.framework = framework;
        Element::new(ElementKind::ComboBox(self.props))
    }
}

impl RadioButtons {
    pub fn new<T: Into<SelectorItem>>(
        items: impl IntoIterator<Item = T>,
        on_selection_changed: impl Fn(Option<u64>) + 'static,
    ) -> Framework<Self> {
        Self::from_items(SelectorItems::new(items), on_selection_changed)
    }

    pub fn display<T: Into<SelectorItem>>(items: impl IntoIterator<Item = T>) -> Framework<Self> {
        Self::display_items(SelectorItems::new(items))
    }

    pub fn from_items(
        items: SelectorItems,
        on_selection_changed: impl Fn(Option<u64>) + 'static,
    ) -> Framework<Self> {
        Framework::new(Self::with_handler(
            items,
            Some(Rc::new(on_selection_changed)),
        ))
    }

    pub fn display_items(items: SelectorItems) -> Framework<Self> {
        Framework::new(Self::with_handler(items, None))
    }

    fn with_handler(
        items: SelectorItems,
        on_selection_changed: Option<OptionalKeyEventFn>,
    ) -> Self {
        Self {
            props: RadioButtonsProps {
                items,
                header: None,
                selected_key: None,
                max_columns: 1,
                on_selection_changed,
                framework: FrameworkProps::default(),
            },
        }
    }

    pub(crate) fn build_with_framework(mut self, framework: FrameworkProps) -> Element {
        let mut framework = framework;
        if self.props.on_selection_changed.is_none() {
            enforce_display_only(&mut framework);
        }
        self.props.framework = framework;
        Element::new(ElementKind::RadioButtons(self.props))
    }
}

impl VirtualCollection {
    fn new<F>(kind: VirtualCollectionKind, count: usize, height: f64, row: F) -> Self
    where
        F: Fn(usize) -> Element + 'static,
    {
        Self {
            kind,
            items: VirtualCollectionItems::Implicit(count),
            height,
            empty: None,
            automation_name: None,
            help_text: None,
            selection_mode: SelectionMode::None,
            selection: CollectionSelection::default(),
            on_selection_changed: None,
            on_item_invoked: None,
            selection_display_only: false,
            can_reorder_items: false,
            on_items_reordered: None,
            row: Rc::new(row),
        }
    }

    fn item_keys(mut self, keys: VirtualItemKeys) -> Self {
        self.items = VirtualCollectionItems::Keyed(keys);
        self
    }

    fn empty_state(mut self, value: Element) -> Self {
        self.empty = Some(Box::new(value));
        self
    }

    fn automation_name(mut self, value: impl Into<String>) -> Self {
        self.automation_name = Some(value.into());
        self
    }

    fn help_text(mut self, value: impl Into<String>) -> Self {
        self.help_text = Some(value.into());
        self
    }

    fn selection_mode(mut self, value: SelectionMode) -> Self {
        self.selection_mode = value;
        self
    }

    fn selection(
        mut self,
        value: CollectionSelection,
        handler: impl Fn(CollectionSelection) + 'static,
    ) -> Self {
        if self.selection_mode == SelectionMode::None {
            self.selection_mode = SelectionMode::Single;
        }
        self.selection = value;
        self.on_selection_changed = Some(Rc::new(handler));
        self
    }

    fn display_selection(mut self, value: CollectionSelection) -> Self {
        if self.selection_mode == SelectionMode::None {
            self.selection_mode = SelectionMode::Single;
        }
        self.selection = value;
        self.on_selection_changed = None;
        self.selection_display_only = true;
        self
    }

    fn on_item_invoked(mut self, handler: impl Fn(u64) + 'static) -> Self {
        self.on_item_invoked = Some(Rc::new(handler));
        self
    }

    fn reorderable(mut self, handler: impl Fn(Vec<u64>) + 'static) -> Self {
        self.can_reorder_items = true;
        self.on_items_reordered = Some(Rc::new(handler));
        self
    }

    fn build(self) -> Element {
        let name = match self.kind {
            VirtualCollectionKind::ListView => "VirtualList",
            VirtualCollectionKind::GridView => "VirtualGrid",
        };
        assert!(
            self.selection_mode != SelectionMode::None || self.selection.is_empty(),
            "{name} selection must be empty when selection mode is None"
        );
        assert!(
            self.selection_mode != SelectionMode::Single || self.selection.len() <= 1,
            "{name} single-selection mode accepts at most one selected key"
        );
        assert!(
            self.on_selection_changed.is_some()
                || self.selection_display_only
                || self.selection_mode == SelectionMode::None,
            "{name} selection requires a callback or display_selection"
        );
        Element::new(ElementKind::VirtualCollection(Box::new(
            VirtualCollectionProps {
                kind: self.kind,
                items: self.items,
                height: self.height,
                empty: self.empty,
                automation_name: self.automation_name,
                help_text: self.help_text,
                selection_mode: self.selection_mode,
                selection: self.selection,
                on_selection_changed: self.on_selection_changed,
                on_item_invoked: self.on_item_invoked,
                selection_display_only: self.selection_display_only,
                can_reorder_items: self.can_reorder_items,
                on_items_reordered: self.on_items_reordered,
                row: self.row,
            },
        )))
    }
}

macro_rules! impl_virtual_collection {
    ($builder:ident, $kind:ident) => {
        impl $builder {
            pub fn new<F>(count: usize, height: f64, row: F) -> Self
            where
                F: Fn(usize) -> Element + 'static,
            {
                Self(VirtualCollection::new(
                    VirtualCollectionKind::$kind,
                    count,
                    height,
                    row,
                ))
            }

            pub fn item_keys(mut self, keys: VirtualItemKeys) -> Self {
                self.0 = self.0.item_keys(keys);
                self
            }

            pub fn empty_state(mut self, value: Element) -> Self {
                self.0 = self.0.empty_state(value);
                self
            }

            pub fn automation_name(mut self, value: impl Into<String>) -> Self {
                self.0 = self.0.automation_name(value);
                self
            }

            pub fn help_text(mut self, value: impl Into<String>) -> Self {
                self.0 = self.0.help_text(value);
                self
            }

            pub fn selection_mode(mut self, value: SelectionMode) -> Self {
                self.0 = self.0.selection_mode(value);
                self
            }

            pub fn selection(
                mut self,
                value: CollectionSelection,
                handler: impl Fn(CollectionSelection) + 'static,
            ) -> Self {
                self.0 = self.0.selection(value, handler);
                self
            }

            pub fn display_selection(mut self, value: CollectionSelection) -> Self {
                self.0 = self.0.display_selection(value);
                self
            }

            pub fn on_item_invoked(mut self, handler: impl Fn(u64) + 'static) -> Self {
                self.0 = self.0.on_item_invoked(handler);
                self
            }

            pub fn reorderable(mut self, handler: impl Fn(Vec<u64>) + 'static) -> Self {
                self.0 = self.0.reorderable(handler);
                self
            }

            pub fn build(self) -> Element {
                self.0.build()
            }
        }
    };
}

impl_virtual_collection!(VirtualList, ListView);
impl_virtual_collection!(VirtualGrid, GridView);

impl Framework<ListBox> {
    pub fn selection_mode(mut self, value: SelectionMode) -> Self {
        assert!(
            value != SelectionMode::None,
            "ListBox does not support SelectionMode::None"
        );
        self.control.props.selection_mode = value;
        self
    }

    pub fn selection(mut self, value: CollectionSelection) -> Self {
        self.control.props.selection = value;
        self
    }
}

impl Framework<ComboBox> {
    pub fn header(mut self, value: impl Into<String>) -> Self {
        self.control.props.header = Some(value.into());
        self
    }

    pub fn placeholder_text(mut self, value: impl Into<String>) -> Self {
        self.control.props.placeholder = Some(value.into());
        self
    }

    pub fn editable(mut self, value: bool) -> Self {
        self.control.props.editable = value;
        self
    }

    pub fn selected_key(mut self, value: Option<u64>) -> Self {
        self.control.props.selected_key = value;
        self
    }
}

impl Framework<RadioButtons> {
    pub fn header(mut self, value: impl Into<String>) -> Self {
        self.control.props.header = Some(value.into());
        self
    }

    pub fn selected_key(mut self, value: Option<u64>) -> Self {
        self.control.props.selected_key = value;
        self
    }

    pub fn max_columns(mut self, value: i32) -> Self {
        assert!(value > 0, "RadioButtons max columns must be positive");
        self.control.props.max_columns = value;
        self
    }
}
