use windows_reactor::*;

fn app(_cx: &mut RenderCx) -> Element {
    relative_panel([
        text_block("Top Left"),
        text_block("Top Right")
            .relative_align_right()
            .relative_align_top(),
        text_block("Bottom Left")
            .relative_align_left()
            .relative_align_bottom(),
        text_block("Bottom Right")
            .relative_align_right()
            .relative_align_bottom(),
        text_block("Center")
            .relative_align_h_center()
            .relative_align_v_center(),
    ])
    .width(300.0)
    .height(200.0)
    .into()
}

fn main() -> Result<()> {
    reactor_samples::run("RelativePanel", app)
}
