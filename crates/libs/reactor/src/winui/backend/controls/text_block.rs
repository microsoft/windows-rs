//! Typed handler for the `TextBlock` widget — replaces scattered match arms
//! in the main `set_prop` dispatch with direct field-comparison diffing.

use super::EventCtx;
use crate::bindings as Xaml;
use crate::core::widgets::TextBlock;
use crate::winui::backend::Handle;
use Xaml::FontWeight as WinFontWeight;

pub fn mount(w: &TextBlock, handle: &Handle, _ctx: &mut EventCtx) -> windows_core::Result<()> {
    let tb = handle.cast_inner::<Xaml::ITextBlock>()?;
    tb.put_Text(w.content.as_str())?;
    if let Some(fs) = w.font_size {
        tb.put_FontSize(fs)?;
    }
    if let Some(fw) = w.font_weight {
        tb.put_FontWeight(WinFontWeight { Weight: fw })?;
    }
    if w.wrap_text {
        tb.put_TextWrapping(Xaml::TextWrapping::Wrap)?;
    }
    if w.is_text_selection_enabled {
        tb.put_IsTextSelectionEnabled(true)?;
    }
    Ok(())
}

pub fn diff(
    old: &TextBlock,
    new: &TextBlock,
    handle: &Handle,
    _ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let tb = handle.cast_inner::<Xaml::ITextBlock>()?;
    let dep = handle.cast_inner::<Xaml::IDependencyObject>()?;

    if new.content != old.content {
        tb.put_Text(new.content.as_str())?;
    }
    if new.font_size != old.font_size {
        if let Some(v) = new.font_size {
            tb.put_FontSize(v)?;
        } else {
            dep.ClearValue(&Xaml::TextBlock::get_FontSizeProperty()?)?;
        }
    }
    if new.font_weight != old.font_weight {
        if let Some(v) = new.font_weight {
            tb.put_FontWeight(WinFontWeight { Weight: v })?;
        } else {
            dep.ClearValue(&Xaml::TextBlock::get_FontWeightProperty()?)?;
        }
    }
    if new.wrap_text != old.wrap_text {
        let mode = if new.wrap_text {
            Xaml::TextWrapping::Wrap
        } else {
            Xaml::TextWrapping::NoWrap
        };
        tb.put_TextWrapping(mode)?;
    }
    if new.is_text_selection_enabled != old.is_text_selection_enabled {
        tb.put_IsTextSelectionEnabled(new.is_text_selection_enabled)?;
    }
    Ok(())
}
