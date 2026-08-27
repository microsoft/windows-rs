use windows_reactor::*;

fn main() {
    sample_reactor_controls::run("RelativePanel", || {
        RelativePanel::new().width(300.0).height(200.0).children((
            "Top Left",
            TextBlock::new()
                .text("Top Right")
                .relative_align_right()
                .relative_align_top(),
            TextBlock::new()
                .text("Bottom Left")
                .relative_align_left()
                .relative_align_bottom(),
            TextBlock::new()
                .text("Bottom Right")
                .relative_align_right()
                .relative_align_bottom(),
            TextBlock::new()
                .text("Center")
                .relative_align_horizontal_center()
                .relative_align_vertical_center(),
        ))
    })
    .unwrap();
}
