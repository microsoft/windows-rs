use crate::controls::*;
use windows_reactor::*;

pub fn materials_page(_: &(), cx: &mut RenderCx) -> impl Into<Element> {
    let (selected, set_selected) = cx.use_state(0_i32);

    let options: Vec<String> = ["Mica", "Mica Alt", "Acrylic", "None (solid)"]
        .into_iter()
        .map(String::from)
        .collect();

    let description = match selected {
        0 => {
            "Mica samples the desktop wallpaper and applies a subtle tint. Default for app windows."
        }
        1 => "Mica Alt uses a stronger tint - ideal for tab-based UIs and title bar regions.",
        2 => "Acrylic provides translucent blur with noise. Falls back to solid when inactive.",
        _ => "No system backdrop - solid color background.",
    };

    page_content(
        "Materials",
        "Window backdrop materials that provide depth and visual hierarchy using translucent surfaces.",
        vec![
            sample_card(
                "Live Backdrop Switcher",
                vstack((
                    text_block("Select a backdrop material to apply it to this window:"),
                    list_view(options, |item, _idx| {
                        text_block(item.clone())
                            .padding(Thickness::uniform(10.0))
                    })
                        .with_key_selector(|item| item.clone())
                        .selected_index(selected)
                        .on_selection_changed({
                            let set_selected = set_selected;
                            move |index: i32| {
                                let backdrop = match index {
                                    0 => Some(Backdrop::Mica),
                                    1 => Some(Backdrop::MicaAlt),
                                    2 => Some(Backdrop::Acrylic),
                                    _ => None,
                                };
                                set_backdrop(backdrop);
                                set_selected.call(index);
                            }
                        })
                        .height(170.0),
                    text_block(description).opacity(0.7),
                ))
                .spacing(8.0)
                ,
                r#"set_backdrop(Some(Backdrop::Mica));
set_backdrop(Some(Backdrop::MicaAlt));
set_backdrop(Some(Backdrop::Acrylic));
set_backdrop(None); // remove backdrop"#,
            ),
            sample_card(
                "Usage Guidance",
                vstack((
                    text_block("� Use Mica for primary app windows (default material)")
                        .font_size(13.0),
                    text_block("� Use Mica Alt for windows with prominent tabs or sections")
                        .font_size(13.0),
                    text_block("� Use Acrylic for transient surfaces (flyouts, dialogs)")
                        .font_size(13.0),
                    text_block("� All materials degrade gracefully on unsupported hardware")
                        .font_size(13.0),
                ))
                .spacing(6.0)
                ,
                r#"// At app level:
App::new(root).backdrop(Backdrop::Mica).run()

// Runtime switching:
set_backdrop(Some(Backdrop::Acrylic));"#,
            ),
        ],
    )
}
