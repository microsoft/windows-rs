use std::rc::Rc;
use std::sync::LazyLock;

use windows_reactor::core::backend::{ControlKind, Op, RecordingBackend};
use windows_reactor::core::component::Component;
use windows_reactor::core::component_element::component;
use windows_reactor::core::context::Context;
use windows_reactor::core::element::Element;
use windows_reactor::core::element::{
    Expander, HyperlinkButton, InfoBar, ProgressBar, ProgressRing, RadioButton, Shape, Slider,
    TabItem, TabView, ToggleSwitch,
};
use windows_reactor::core::reconciler::Reconciler;
use windows_reactor::core::render_context::RenderCx;
use windows_reactor::dsl::{ElementExt, text_block};
use windows_reactor::vstack;

static THEME: LazyLock<Context<String>> = LazyLock::new(|| Context::new("light".to_string()));

struct GallerySnapshot;
impl Component for GallerySnapshot {
    fn render(&self, _props: &(), _cx: &mut RenderCx) -> Element {
        let tabs = TabView::new([
            TabItem::new("Input", input_page()).with_key("input"),
            TabItem::new("Layout", layout_page()).with_key("layout"),
            TabItem::new("Shell", shell_page()).with_key("shell"),
            TabItem::new("Shapes", shapes_page()).with_key("shapes"),
            TabItem::new(
                "Context",
                component(Level3, ()).provide(&THEME, "dark".to_string()),
            )
            .with_key("context"),
        ]);
        vstack((tabs,)).into()
    }
}

fn input_page() -> Element {
    vstack((
        ToggleSwitch::new(true).on_content("On").off_content("Off"),
        Slider::new(35.0).range(0.0, 100.0),
        RadioButton::new("Small").group("size"),
        RadioButton::new("Medium").group("size").checked(true),
        RadioButton::new("Large").group("size"),
    ))
    .into()
}

fn layout_page() -> Element {
    Expander::new(text_block("inner")).header("Click me").into()
}

fn shell_page() -> Element {
    vstack((InfoBar::new("hi").message("hello"),)).into()
}

fn shapes_page() -> Element {
    vstack((
        Shape::rectangle().fill_rgb(10, 20, 30),
        Shape::ellipse().fill_rgb(200, 50, 80),
        Shape::line(0.0, 0.0, 50.0, 0.0),
        ProgressBar::new(50.0),
        ProgressRing::indeterminate(),
        HyperlinkButton::new("Docs"),
    ))
    .into()
}

struct Level3;
impl Component for Level3 {
    fn render(&self, _props: &(), cx: &mut RenderCx) -> Element {
        let theme = cx.use_context(&THEME);
        text_block(format!("theme={theme}")).into()
    }
}

fn mount_gallery() -> Reconciler<RecordingBackend> {
    let mut r = Reconciler::new(RecordingBackend::new());
    let root = component(GallerySnapshot, ());
    r.reconcile(None, &root, None, Rc::new(|| {}));
    r
}

fn kinds_created(r: &Reconciler<RecordingBackend>) -> Vec<ControlKind> {
    r.backend
        .ops
        .iter()
        .filter_map(|op| match op {
            Op::Create { kind, .. } => Some(*kind),
            _ => None,
        })
        .collect()
}

#[test]
fn gallery_mounts_all_control_kinds_from_each_page() {
    let r = mount_gallery();
    let kinds = kinds_created(&r);

    for expected in [
        ControlKind::TabView,
        ControlKind::TabViewItem,
        ControlKind::ToggleSwitch,
        ControlKind::Slider,
        ControlKind::RadioButton,
        ControlKind::Expander,
        ControlKind::InfoBar,
        ControlKind::Rectangle,
        ControlKind::Ellipse,
        ControlKind::Line,
        ControlKind::ProgressBar,
        ControlKind::ProgressRing,
        ControlKind::HyperlinkButton,
    ] {
        assert!(
            kinds.contains(&expected),
            "gallery did not create a {expected:?}; kinds={kinds:?}"
        );
    }
}

#[test]
fn context_demo_page_resolves_provided_theme() {
    let r = mount_gallery();

    let saw_dark = r.backend.ops.iter().any(|op| {
        matches!(
            op,
            Op::SetProp {
                prop: windows_reactor::core::backend::Prop::Text,
                value: windows_reactor::core::backend::PropValue::Str(s),
                ..
            } if s == "theme=dark"
        )
    });
    assert!(saw_dark, "context demo must resolve provided theme");
}
