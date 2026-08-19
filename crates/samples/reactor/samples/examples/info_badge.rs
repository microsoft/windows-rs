#![windows_subsystem = "windows"]

use windows_reactor::{Element, InfoBadge, RenderCx, StackPanel, TextBlock};

pub fn app(_cx: &mut RenderCx<'_>) -> Element {
    StackPanel::new([
        TextBlock::new("Dot (attention indicator)").build(),
        InfoBadge::dot()
            .automation_name("Attention indicator")
            .build(),
        TextBlock::new("Numeric (small / large counts)").build(),
        StackPanel::new([
            InfoBadge::numeric(1).automation_name("Count 1").build(),
            InfoBadge::numeric(9).automation_name("Count 9").build(),
            InfoBadge::numeric(42).automation_name("Count 42").build(),
            InfoBadge::numeric(999).automation_name("Count 999").build(),
        ])
        .spacing(12.0)
        .build(),
    ])
    .spacing(8.0)
    .build()
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("InfoBadge", app)
}
