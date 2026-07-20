#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DragOperation {
    None,
    Copy,
    Move,
    Link,
}

#[derive(Clone)]
pub struct DroppedItem {
    pub path: String,
    pub name: String,
    pub is_folder: bool,
}

pub struct DragContext {
    pub has_text: bool,
    pub has_html: bool,
    pub has_rtf: bool,
    pub has_bitmap: bool,
    pub has_storage_items: bool,
    pub has_uri: bool,
    pub has_web_link: bool,
    pub has_application_link: bool,
    pub caption: Option<String>,
    pub glyph_visible: Option<bool>,
    pub content_visible: Option<bool>,
    pub(crate) get_text_fn: Option<Box<dyn Fn() -> Option<String>>>,
    pub(crate) get_storage_items_fn: Option<Box<dyn Fn() -> Vec<DroppedItem>>>,
}

impl DragContext {
    pub fn text(&self) -> Option<String> {
        self.get_text_fn.as_ref().and_then(|f| f())
    }

    pub fn storage_items(&self) -> Vec<DroppedItem> {
        self.get_storage_items_fn
            .as_ref()
            .map_or_else(Vec::new, |f| f())
    }
}

crate::impl_rc_fn_wrapper! {
    pub struct DragCallback(dyn Fn(&mut DragContext) -> DragOperation);
}

impl DragCallback {
    pub fn call(&self, ctx: &mut DragContext) -> DragOperation {
        (self.inner)(ctx)
    }
}

crate::impl_rc_fn_wrapper! {
    pub struct DragNotifyCallback(dyn Fn(&DragContext));
}

impl DragNotifyCallback {
    pub fn call(&self, ctx: &DragContext) {
        (self.inner)(ctx);
    }
}

crate::impl_arc_fn_wrapper! {
    pub struct DragAsyncCallback(dyn Fn(&mut DragContext) -> DragOperation);
}

impl DragAsyncCallback {
    pub fn call(&self, ctx: &mut DragContext) -> DragOperation {
        (self.inner)(ctx)
    }
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct DragHandlers {
    pub on_drag_enter: Option<DragAsyncCallback>,
    pub on_drag_leave: Option<DragNotifyCallback>,
    pub on_drag_over: Option<DragCallback>,
    pub on_drag_drop: Option<DragAsyncCallback>,
}

impl DragHandlers {
    pub fn is_empty(&self) -> bool {
        self.on_drag_enter.is_none()
            && self.on_drag_leave.is_none()
            && self.on_drag_over.is_none()
            && self.on_drag_drop.is_none()
    }
}
