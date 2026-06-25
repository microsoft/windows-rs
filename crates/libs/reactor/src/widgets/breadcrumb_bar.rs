use super::*;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct BreadcrumbBar {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub items: Vec<String>,
    pub on_item_clicked: Option<Callback<i32>>,
}
impl BreadcrumbBar {
    pub fn new<I, S>(items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            items: items.into_iter().map(Into::into).collect(),
            ..Default::default()
        }
    }
    pub fn on_item_clicked(mut self, f: impl IntoCallback<i32>) -> Self {
        self.on_item_clicked = Some(f.into_callback());
        self
    }
}

impl Widget for BreadcrumbBar {
    widget_header!(ControlKind::BreadcrumbBar);
    fn bindings(&self) -> PropBindings {
        let mut out = generated::breadcrumb_bar_bindings(self);
        out.push(Binding::Prop(
            Prop::Items,
            PropValue::StrList(self.items.clone()),
        ));
        out
    }
}
