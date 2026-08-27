#![windows_subsystem = "windows"]

use windows_reactor::*;

fn main() {
    sample_reactor_controls::run("Expander", || {
        StackPanel::new().spacing(8.0).max_width(400.0).children((
            Expander::new().is_expanded(true).slots([
                SlotView::new(ExpanderSlot::Header, "Details"),
                SlotView::new(
                    ExpanderSlot::Content,
                    StackPanel::new().spacing(4.0).children((
                        "Hidden details live inside the expander.",
                        "Click the chevron to collapse this panel.",
                    )),
                ),
            ]),
            Expander::new().is_expanded(false).slots([
                SlotView::new(ExpanderSlot::Header, "More"),
                SlotView::new(ExpanderSlot::Content, "Collapsed by default."),
            ]),
            Expander::new().is_expanded(true).slots([
                SlotView::new(
                    ExpanderSlot::Header,
                    StackPanel::new()
                        .orientation(Orientation::Horizontal)
                        .spacing(8.0)
                        .children((
                            TextBlock::new().text("[*]").font_size(18.0),
                            TextBlock::new().text("Settings").font_weight(700),
                        )),
                ),
                SlotView::new(
                    ExpanderSlot::Content,
                    "Body content for the rich header expander.",
                ),
            ]),
        ))
    })
    .unwrap();
}
