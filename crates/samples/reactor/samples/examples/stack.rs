#![windows_subsystem = "windows"]

use windows_reactor::{Element, RenderCx, hstack, text_block, vstack};

pub fn app(_cx: &mut RenderCx<'_>) -> Element {
    vstack(
        12.0,
        [
            text_block("vstack - vertical orientation"),
            hstack(
                8.0,
                [
                    text_block("left"),
                    text_block("middle"),
                    text_block("right"),
                ],
            ),
            text_block("...back to the vstack"),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("Stack", app)
}
