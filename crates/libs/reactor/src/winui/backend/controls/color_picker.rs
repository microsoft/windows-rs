//! Typed handler for the `ColorPicker` widget.

use super::EventCtx;
use crate::bindings as Xaml;
use crate::core::backend::{Event, EventHandler};
use crate::core::widgets::ColorPickerWidget;
use crate::winui::backend::Handle;

pub fn mount(
    widget: &ColorPickerWidget,
    handle: &Handle,
    ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let Handle::ColorPicker(cp) = handle else {
        return Ok(());
    };

    cp.put_Color(Xaml::Color {
        A: widget.color.a,
        R: widget.color.r,
        G: widget.color.g,
        B: widget.color.b,
    })?;

    if let Some(v) = widget.is_alpha_enabled {
        cp.put_IsAlphaEnabled(v)?;
    }
    if let Some(v) = widget.is_hex_input_visible {
        cp.put_IsHexInputVisible(v)?;
    }
    if let Some(v) = widget.is_color_slider_visible {
        cp.put_IsColorSliderVisible(v)?;
    }
    if let Some(v) = widget.is_color_channel_text_input_visible {
        cp.put_IsColorChannelTextInputVisible(v)?;
    }

    ctx.mount_event(
        &widget.on_changed,
        Event::ColorChanged,
        EventHandler::ColorChanged,
    );
    Ok(())
}

pub fn diff(
    old: &ColorPickerWidget,
    new: &ColorPickerWidget,
    handle: &Handle,
    ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let Handle::ColorPicker(cp) = handle else {
        return Ok(());
    };

    if old.color != new.color {
        cp.put_Color(Xaml::Color {
            A: new.color.a,
            R: new.color.r,
            G: new.color.g,
            B: new.color.b,
        })?;
    }
    if old.is_alpha_enabled != new.is_alpha_enabled
        && let Some(v) = new.is_alpha_enabled
    {
        cp.put_IsAlphaEnabled(v)?;
    }
    if old.is_hex_input_visible != new.is_hex_input_visible
        && let Some(v) = new.is_hex_input_visible
    {
        cp.put_IsHexInputVisible(v)?;
    }
    if old.is_color_slider_visible != new.is_color_slider_visible
        && let Some(v) = new.is_color_slider_visible
    {
        cp.put_IsColorSliderVisible(v)?;
    }
    if old.is_color_channel_text_input_visible != new.is_color_channel_text_input_visible
        && let Some(v) = new.is_color_channel_text_input_visible
    {
        cp.put_IsColorChannelTextInputVisible(v)?;
    }

    ctx.diff_event(
        &old.on_changed,
        &new.on_changed,
        Event::ColorChanged,
        EventHandler::ColorChanged,
    );
    Ok(())
}
