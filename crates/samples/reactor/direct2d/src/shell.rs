//! Hub shell hosting the Direct2D samples in a `NavigationView`. Each sample is
//! a self-contained component, so its state (such as the render thread) is
//! created and torn down as the user navigates between samples.
//!
//! The shell owns the app-wide shared GPU [`Device`] and publishes it through the
//! [`DEVICE`] context, so every sample renders with the same device.

use crate::device::{Device, device_context};
use crate::surface_image_source::surface_image_source_sample;
use crate::swap_chain::swap_chain_sample;
use windows_reactor::*;

pub fn shell(cx: &mut RenderCx) -> Element {
    let (selected_tag, set_selected_tag) = cx.use_state(String::from("swap-chain"));

    // The single shared device for the whole app. Created once on mount; the
    // samples below read it from the `DEVICE` context.
    let (device, set_device) = cx.use_state::<Option<Device>>(None);
    cx.use_effect((), move || match Device::new() {
        Ok(d) => set_device.call(Some(d)),
        Err(e) => eprintln!("failed to create shared device: {e}"),
    });

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

    NavigationView::new(nav_items, content)
        .selected_tag(&selected_tag)
        .settings_visible(false)
        .pane_title("Direct2D Samples")
        .pane_display_mode(NavigationViewPaneDisplayMode::Left)
        .on_selection_changed(move |tag: String| {
            if !tag.is_empty() {
                set_selected_tag.call(tag);
            }
        })
        .provide(&device_context(), device)
}
