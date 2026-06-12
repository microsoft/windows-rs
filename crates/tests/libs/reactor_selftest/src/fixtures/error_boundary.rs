use windows_reactor::Component;
use windows_reactor::Element;
use windows_reactor::error_boundary;
use windows_reactor::RenderCx;
use windows_reactor::{button, text_block};

use crate::fixtures::reconciler::{FixtureFuture, cc};
use crate::harness::Harness;
use windows_reactor::vstack;

struct ThrowingComponent;
impl Component for ThrowingComponent {
    fn render(&self, _: &(), _cx: &mut RenderCx) -> Element {
        panic!("Component crashed");
    }
}

#[derive(Clone, PartialEq)]
struct ShouldThrow(bool);

struct ConditionalThrow;
impl Component<ShouldThrow> for ConditionalThrow {
    fn render(&self, props: &ShouldThrow, _cx: &mut RenderCx) -> Element {
        assert!(!props.0, "Boom");
        text_block("Healthy").into()
    }
}

pub fn catches_render_error(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_cx| {
            error_boundary(
                windows_reactor::component(ThrowingComponent, ()),
                |msg| {
                    vstack((
                        text_block("Error caught!"),
                        text_block(format!("Message: {msg}")),
                    ))
                    .into()
                },
            )
        }));
        h.render().await;

        h.check(
            "ErrorBoundary_CatchesRenderError_FallbackShown",
            h.find_text("Error caught!").is_some(),
        );
        h.check(
            "ErrorBoundary_CatchesRenderError_MessageShown",
            h.find_text_containing("Component crashed").is_some(),
        );
    })
}

pub fn recovery(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|cx| {
            let (should_throw, set) = cx.use_state(true);
            vstack((
                error_boundary(
                    windows_reactor::component(
                        ConditionalThrow,
                        ShouldThrow(should_throw),
                    ),
                    |_msg| text_block("In error state").into(),
                ),
                button("Recover").on_click(move || set.call(false)),
            ))
            .into()
        }));
        h.render().await;

        h.check(
            "ErrorBoundary_Recovery_InitialError",
            h.find_text("In error state").is_some(),
        );

        let _ = h.click_button("Recover");
        h.render().await;

        h.check(
            "ErrorBoundary_Recovery_Recovered",
            h.find_text("Healthy").is_some(),
        );
        h.check(
            "ErrorBoundary_Recovery_ErrorCleared",
            h.find_text("In error state").is_none(),
        );
    })
}
