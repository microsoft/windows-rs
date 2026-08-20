use windows_reactor_next::*;

fn main() {
    bootstrap().unwrap();
    App::run(|hooks| {
        let count = hooks.use_state(|| 0_u32);
        let increment = count.clone();
        StackPanel::new()
            .spacing(8.0)
            .child("value", TextBlock::new().text(count.get().to_string()))
            .child(
                "increment",
                Button::new()
                    .on_click(move || increment.update(|value| *value += 1))
                    .content(TextBlock::new().text("+")),
            )
            .into()
    })
    .unwrap();
}
