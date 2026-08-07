use crate::fixtures::reconciler::{FixtureFuture, cc};
use crate::harness::Harness;
use windows_reactor::{ElementRef, ElementRefExt, TextBoxHandle, text_block, text_box};

pub fn text_box_focus_and_clear(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        let reference = ElementRef::<TextBoxHandle>::new();
        let mounted_reference = reference.clone();
        h.mount(cc(move |_| {
            text_box("Focus target")
                .element_ref(&mounted_reference)
                .into()
        }));
        h.render().await;

        h.check("ElementRef_TextBox_Populated", reference.is_mounted());
        h.check(
            "ElementRef_TextBox_Focus",
            reference.focus().unwrap_or(false),
        );

        h.mount(cc(|_| text_block("TextBox removed").into()));
        h.render().await;
        h.check("ElementRef_TextBox_Cleared", !reference.is_mounted());
    })
}
