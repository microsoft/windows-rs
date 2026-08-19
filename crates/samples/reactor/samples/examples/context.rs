#![windows_subsystem = "windows"]

use windows_reactor::{
    Context, Element, FontWeight, RenderCx, TextBlock, button, component, hstack, provide_context,
    vstack,
};

fn app(cx: &mut RenderCx<'_>) -> Element {
    let theme = cx.use_state(|| "light".to_string());
    let selected = theme.value();
    let context = cx.use_memo((), || Context::new("light".to_string()));

    let light = theme.clone();
    let dark = theme.clone();
    let neon = theme;
    let leaf_context = context.clone();
    let leaf = component(move |cx| {
        let theme = cx.use_context(&leaf_context);
        TextBlock::new(format!("Leaf sees theme = {theme}"))
            .font_size(16.0)
            .font_weight(FontWeight::BOLD)
            .build()
    });

    provide_context(
        &context,
        selected,
        vstack(
            12.0,
            [
                TextBlock::new("Pick a theme; the leaf reads it through typed context.").build(),
                hstack(
                    8.0,
                    [
                        button("light", move || {
                            light.set("light".to_string());
                        }),
                        button("dark", move || {
                            dark.set("dark".to_string());
                        }),
                        button("neon", move || {
                            neon.set("neon".to_string());
                        }),
                    ],
                ),
                leaf,
            ],
        ),
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("Context", app)
}
