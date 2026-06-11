//! Hub shell hosting the Direct2D samples in a `NavigationView`. Each sample is
//! a self-contained component, so its state (such as the render thread) is
//! created and torn down as the user navigates between samples.
//!
//! The shell owns the app-wide shared GPU [`Device`] and publishes it (with the
//! recovery signal) through the [`Gpu`](crate::device::Gpu) context, so every
//! sample renders with the same device.

use crate::device::{Device, Gpu, gpu_context};
use crate::surface_image_source::surface_image_source_sample;
use crate::swap_chain::swap_chain_sample;
use windows_reactor::*;

pub fn shell(cx: &mut RenderCx) -> Element {
    let (selected_tag, set_selected_tag) = cx.use_state(String::from("swap-chain"));

    // The single shared device for the whole app. `use_reducer` gives a
    // functional updater whose closure sees the current device at apply time.
    let (device, update_device) = cx.use_reducer::<Option<Device>>(None);

    // Bumped on each `Gpu::request_recovery`; also seeds the create/recover
    // effect, so the device is created once on mount.
    let (recover_gen, bump_recover) = cx.use_reducer::<u32>(0);

    // Create the device on mount and recreate it on every recovery request.
    cx.use_effect(recover_gen, {
        let update_device = update_device.clone();
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

    // Memoize the recovery updater so the `Gpu` context value keeps a stable
    // identity across renders (the raw updater is freshly allocated each render).
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

    // Test aid: recreate the shared device on demand, without a real GPU
    // device-removal event.
    let recreate_device = {
        move || {
            update_device.call(|current| match Device::new() {
                Ok(d) => Some(d),
                Err(e) => {
                    eprintln!("failed to recreate shared device: {e}");
                    current
                }
            });
        }
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
        .provide(&gpu_context(), Some(gpu));

    // Stack a "Recreate Device" button beneath the navigation view.
    grid((
        nav_view.grid_row(0),
        button("Recreate Device")
            .icon(SymbolGlyph::Refresh)
            .on_click(recreate_device)
            .margin(Thickness::uniform(12.0))
            .grid_row(1),
    ))
    .rows([GridLength::Star(1.0), GridLength::Auto])
    .columns([GridLength::Star(1.0)])
    .into()
}
