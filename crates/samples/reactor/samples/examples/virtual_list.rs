#![windows_subsystem = "windows"]

use windows_reactor::{
    Border, Color, Element, RenderCx, TextBlock, Thickness, VirtualList, vstack,
};

fn app(_cx: &mut RenderCx<'_>) -> Element {
    vstack(
        8.0,
        [
            TextBlock::new("Only the visible rows are realized from 5,000 logical items.").build(),
            VirtualList::new(5_000, 420.0, |index| {
                let padding = if index % 5 == 0 { 14.0 } else { 6.0 };
                Border::new(TextBlock::new(format!("Item {index}")).build())
                    .background(if index % 2 == 0 {
                        Color::rgb(245, 247, 250)
                    } else {
                        Color::rgb(232, 238, 246)
                    })
                    .padding(Thickness::uniform(padding))
                    .build()
            })
            .automation_name("5,000 virtual items")
            .build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("VirtualList", app)
}
