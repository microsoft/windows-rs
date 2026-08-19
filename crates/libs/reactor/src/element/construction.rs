use std::any::TypeId;
use std::rc::Rc;

use windows_time::DateTime;

use crate::element::tree::ElementKind;
use crate::element::{
    Border, Button, CalendarDatePicker, Canvas, CanvasChild, CheckBox, Color, ColorPicker, Context,
    DatePicker, Element, Expander, Grid, GridChild, HyperlinkButton, NumberBox, Orientation,
    PasswordBox, ProgressBar, ProgressRing, RadioButton, RatingControl, RelativePanel,
    RelativePanelChild, RenderCx, RepeatButton, ScrollView, ScrollViewer, Slider, SplitView,
    StackPanel, TextBlock, TextBox, ToggleButton, ToggleSwitch, Viewbox, VirtualGrid, VirtualList,
};
use crate::hooks::ComponentMemo;
use crate::resources::{ContextEntry, ContextKey, ContextProps};
pub fn component(render: impl for<'a> Fn(&mut RenderCx<'a>) -> Element + 'static) -> Element {
    component_from(render)
}

pub fn component_with_props<P>(
    props: P,
    render: impl for<'a> Fn(&mut RenderCx<'a>, &P) -> Element + 'static,
) -> Element
where
    P: 'static,
{
    component_with_props_from(props, render)
}

fn component_from<F>(render: F) -> Element
where
    F: for<'a> Fn(&mut RenderCx<'a>) -> Element + 'static,
{
    Element::new(ElementKind::Component {
        identity: TypeId::of::<F>(),
        render: Rc::new(render),
        memo: None,
    })
}

fn component_with_props_from<P, F>(props: P, render: F) -> Element
where
    P: 'static,
    F: for<'a> Fn(&mut RenderCx<'a>, &P) -> Element + 'static,
{
    let props = Rc::new(props);
    Element::new(ElementKind::Component {
        identity: TypeId::of::<(P, F)>(),
        render: Rc::new(move |cx| render(cx, props.as_ref())),
        memo: None,
    })
}

pub fn memo_component(
    deps: impl PartialEq + 'static,
    render: impl for<'a> Fn(&mut RenderCx<'a>) -> Element + 'static,
) -> Element {
    memo_component_from(deps, render)
}

fn memo_component_from<F>(deps: impl PartialEq + 'static, render: F) -> Element
where
    F: for<'a> Fn(&mut RenderCx<'a>) -> Element + 'static,
{
    Element::new(ElementKind::Component {
        identity: TypeId::of::<F>(),
        render: Rc::new(render),
        memo: Some(ComponentMemo::new(deps)),
    })
}

pub fn memo_component_with_props<P>(
    props: P,
    render: impl for<'a> Fn(&mut RenderCx<'a>, &P) -> Element + 'static,
) -> Element
where
    P: PartialEq + 'static,
{
    memo_component_with_props_from(props, render)
}

fn memo_component_with_props_from<P, F>(props: P, render: F) -> Element
where
    P: PartialEq + 'static,
    F: for<'a> Fn(&mut RenderCx<'a>, &P) -> Element + 'static,
{
    let props = Rc::new(props);
    Element::new(ElementKind::Component {
        identity: TypeId::of::<(P, F)>(),
        render: {
            let props = Rc::clone(&props);
            Rc::new(move |cx| render(cx, props.as_ref()))
        },
        memo: Some(ComponentMemo::from_rc(props)),
    })
}

pub fn fragment(children: impl IntoIterator<Item = Element>) -> Element {
    Element::new(ElementKind::Fragment {
        children: children.into_iter().collect(),
    })
}

pub fn fade_transition(
    child: Element,
    enter: Option<std::time::Duration>,
    exit: Option<std::time::Duration>,
) -> Element {
    Element::new(ElementKind::FadeTransition {
        child: Box::new(child),
        enter,
        exit,
    })
}

pub fn provide_context<T>(context: &Context<T>, value: T, child: Element) -> Element
where
    T: 'static,
{
    Element::new(ElementKind::Context {
        props: ContextProps {
            entry: ContextEntry {
                id: context.id,
                value: Rc::new(value),
            },
        },
        child: Box::new(child),
    })
}

pub fn provide_context_key<T>(context: &'static ContextKey<T>, value: T, child: Element) -> Element
where
    T: 'static,
{
    Element::new(ElementKind::Context {
        props: ContextProps {
            entry: ContextEntry {
                id: context.id(),
                value: Rc::new(value),
            },
        },
        child: Box::new(child),
    })
}

pub fn stack_panel(children: impl IntoIterator<Item = Element>) -> Element {
    StackPanel::new(children).build()
}

pub fn vstack(spacing: f64, children: impl IntoIterator<Item = Element>) -> Element {
    StackPanel::new(children).spacing(spacing).build()
}

pub fn hstack(spacing: f64, children: impl IntoIterator<Item = Element>) -> Element {
    StackPanel::new(children)
        .orientation(Orientation::Horizontal)
        .spacing(spacing)
        .build()
}

pub fn grid<T>(children: impl IntoIterator<Item = T>) -> Element
where
    T: Into<GridChild>,
{
    Grid::new(children).build()
}

pub fn grid_child(child: Element) -> GridChild {
    GridChild::new(child)
}

pub fn canvas<T>(children: impl IntoIterator<Item = T>) -> Element
where
    T: Into<CanvasChild>,
{
    Canvas::new(children).build()
}

pub fn canvas_child(child: Element) -> CanvasChild {
    CanvasChild::new(child)
}

pub fn relative_panel<T>(children: impl IntoIterator<Item = T>) -> Element
where
    T: Into<RelativePanelChild>,
{
    RelativePanel::new(children).build()
}

pub fn relative_panel_child(child: Element) -> RelativePanelChild {
    RelativePanelChild::new(child)
}

pub fn viewbox(child: Element) -> Element {
    Viewbox::new(child).build()
}

pub fn scroll_viewer(child: Element) -> Element {
    ScrollViewer::new(child).build()
}

pub fn scroll_view(child: Element) -> Element {
    ScrollView::new(child).build()
}

pub fn split_view(content: Element, pane: Element) -> Element {
    SplitView::display(content, pane).build()
}

pub fn expander(header: Element, content: Element) -> Element {
    Expander::display(header, content).build()
}

pub fn border(child: Element) -> Element {
    Border::new(child).build()
}

pub fn button(label: impl Into<String>, on_click: impl Fn() + 'static) -> Element {
    Button::new(label).on_click(on_click).build()
}

pub fn hyperlink_button(label: impl Into<String>, on_click: impl Fn() + 'static) -> Element {
    HyperlinkButton::new(label).on_click(on_click).build()
}

pub fn repeat_button(label: impl Into<String>, on_click: impl Fn() + 'static) -> Element {
    RepeatButton::new(label).on_click(on_click).build()
}

pub fn toggle_button(
    label: impl Into<String>,
    checked: bool,
    on_toggle: impl Fn(bool) + 'static,
) -> Element {
    ToggleButton::new(label, checked, on_toggle).build()
}

pub fn toggle_switch(on: bool, on_toggle: impl Fn(bool) + 'static) -> Element {
    ToggleSwitch::new(on, on_toggle).build()
}

pub fn progress_bar(value: f64) -> Element {
    ProgressBar::new(value).build()
}

pub fn progress_ring(value: f64) -> Element {
    ProgressRing::new(value).build()
}

pub fn slider(value: f64, on_change: impl Fn(f64) + 'static) -> Element {
    Slider::new(value, on_change).build()
}

pub fn number_box(
    value: impl Into<Option<f64>>,
    on_change: impl Fn(Option<f64>) + 'static,
) -> Element {
    NumberBox::new(value, on_change).build()
}

pub fn rating_control(
    value: impl Into<Option<f64>>,
    on_change: impl Fn(Option<f64>) + 'static,
) -> Element {
    RatingControl::new(value, on_change).build()
}

pub fn color_picker(color: Color, on_change: impl Fn(Color) + 'static) -> Element {
    ColorPicker::new(color, on_change).build()
}

pub fn date_picker(
    date: impl Into<Option<DateTime>>,
    on_change: impl Fn(Option<DateTime>) + 'static,
) -> Element {
    DatePicker::new(date, on_change).build()
}

pub fn calendar_date_picker(
    date: impl Into<Option<DateTime>>,
    on_change: impl Fn(Option<DateTime>) + 'static,
) -> Element {
    CalendarDatePicker::new(date, on_change).build()
}

pub fn button_enabled(
    label: impl Into<String>,
    enabled: bool,
    on_click: impl Fn() + 'static,
) -> Element {
    Button::new(label)
        .on_click(on_click)
        .enabled(enabled)
        .build()
}

pub fn check_box(
    label: impl Into<String>,
    checked: bool,
    on_toggle: impl Fn(bool) + 'static,
) -> Element {
    CheckBox::new(label, checked, on_toggle).build()
}

pub fn radio_button(
    label: impl Into<String>,
    checked: bool,
    on_toggle: impl Fn(bool) + 'static,
) -> Element {
    RadioButton::new(label, checked, on_toggle).build()
}

pub fn text_block(text: impl Into<String>) -> Element {
    TextBlock::new(text).build()
}

pub fn text_box(text: impl Into<String>, on_change: impl Fn(String) + 'static) -> Element {
    TextBox::new(text, on_change).build()
}

pub fn password_box(password: impl Into<String>, on_change: impl Fn(String) + 'static) -> Element {
    PasswordBox::new(password, on_change).build()
}

pub fn virtual_list<F>(count: usize, height: f64, row: F) -> Element
where
    F: Fn(usize) -> Element + 'static,
{
    VirtualList::new(count, height, row).build()
}

pub fn virtual_grid<F>(count: usize, height: f64, row: F) -> Element
where
    F: Fn(usize) -> Element + 'static,
{
    VirtualGrid::new(count, height, row).build()
}
