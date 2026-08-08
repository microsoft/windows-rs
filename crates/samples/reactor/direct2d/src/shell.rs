use crate::device::{Device, Gpu, gpu_context};
use crate::surface_image_source::surface_image_source_sample;
use crate::swap_chain::swap_chain_sample;
use windows_reactor::*;

pub fn shell(cx: &mut RenderCx) -> Element {
    let (selected_tag, set_selected_tag) = cx.use_state(String::from("swap-chain"));

    let (device, update_device) = cx.use_reducer::<Option<Device>>(None);

    let (recover_gen, bump_recover) = cx.use_reducer::<u32>(0);

    cx.use_effect(recover_gen, {
        move || {
            update_device.call(|current| match Device::new() {
                Ok(d) => Some(d),
                Err(e) => {
                    eprintln!("failed to create shared device: {e}");
                    current
                }
            });
        }
    });

    let bump_recover = cx.use_memo((), move || bump_recover);
    let gpu = Gpu::new(device, bump_recover);

    let nav_items = vec![
        NavViewItem::new("Swap Chain Panel")
            .tag("swap-chain")
            .icon(Symbol::Play),
        NavViewItem::new("Surface Image Source")
            .tag("surface-image-source")
            .icon(Symbol::Camera),
    ];

    let content: Element = match selected_tag.as_str() {
        "surface-image-source" => component(surface_image_source_sample, ()),
        _ => component(swap_chain_sample, ()),
    };

    let recreate_device = {
        let gpu = gpu.clone();
        move || gpu.request_recovery()
    };

    let nav_view = NavigationView::new(nav_items, content)
        .selected_tag(&selected_tag)
        .settings_visible(false)
        .pane_title("Direct2D Samples")
        .pane_display_mode(NavigationViewPaneDisplayMode::Left)
        .on_selection_changed(move |tag: String| {
            if !tag.is_empty() {
                set_selected_tag.call(tag);
            }
        })
        .grid_row(0)
        .provide(&gpu_context(), Some(gpu));

    grid((
        nav_view,
        button("Recreate Device")
            .icon(Symbol::Refresh)
            .on_click(recreate_device)
            .margin(Thickness::uniform(12.0))
            .grid_row(1),
    ))
    .rows([GridLength::Star(1.0), GridLength::Auto])
    .columns([GridLength::Star(1.0)])
    .into()
}
