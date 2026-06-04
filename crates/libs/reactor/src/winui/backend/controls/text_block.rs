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

    super::diff_val!(old, new, content, tb.put_Text(new.content.as_str()));
    super::diff_opt!(
        old,
        new,
        font_size,
        |v| tb.put_FontSize(*v),
        dep.ClearValue(&Xaml::TextBlock::get_FontSizeProperty()?)
    );
    super::diff_opt!(
        old,
        new,
        font_weight,
        |v| tb.put_FontWeight(WinFontWeight { Weight: *v }),
        dep.ClearValue(&Xaml::TextBlock::get_FontWeightProperty()?)
    );
    super::diff_val!(
        old,
        new,
        wrap_text,
        tb.put_TextWrapping(if new.wrap_text {
            Xaml::TextWrapping::Wrap
        } else {
            Xaml::TextWrapping::NoWrap
        })
    );
    super::diff_val!(
        old,
        new,
        is_text_selection_enabled,
        tb.put_IsTextSelectionEnabled(new.is_text_selection_enabled)
    );
    Ok(())
}
