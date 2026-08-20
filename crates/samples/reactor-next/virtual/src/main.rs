use windows_reactor_next::*;

fn main() {
    bootstrap().unwrap();
    App::run(|_| {
        ScrollViewer::new()
            .content(ItemsRepeater::new().items((0_u32..10_000).map(|index| {
                KeyedElement::new(index, TextBlock::new().text(format!("Virtual row {index}")))
            })))
            .into()
    })
    .unwrap();
}
