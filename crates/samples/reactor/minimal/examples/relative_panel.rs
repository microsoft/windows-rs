//! Sample for the `RelativePanel` layout.

use windows_reactor::*;

fn app(_cx: &mut RenderCx) -> Element {
    relative_panel([
        // Top-left (default position).
        text_block("Top Left"),
        // Top-right corner.
        text_block("Top Right")
            .relative_align_right()
            .relative_align_top(),
        // Bottom-left corner.
        text_block("Bottom Left")
            .relative_align_left()
            .relative_align_bottom(),
        // Bottom-right corner.
        text_block("Bottom Right")
            .relative_align_right()
            .relative_align_bottom(),
        // Centered.
        text_block("Center")
            .relative_align_h_center()
            .relative_align_v_center(),
    ])
    .width(300.0)
    .height(200.0)
    .into()
}

fn main() -> Result<()> {
    reactor_minimal::run("RelativePanel", app)
}
