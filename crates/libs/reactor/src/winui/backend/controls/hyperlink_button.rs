//! Typed handler for the `HyperlinkButton` widget.

use crate::bindings as Xaml;
use crate::core::widgets::HyperlinkButton;
use crate::winui::backend::Handle;
use crate::winui::backend::convert::string_as_textblock;

pub fn mount(w: &HyperlinkButton, handle: &Handle) -> windows_core::Result<()> {
    let txt = string_as_textblock(&w.label)?;
    handle
        .cast_inner::<Xaml::IContentControl>()?
        .put_Content(&txt)?;
    if let Some(uri) = &w.navigate_uri {
        let u = Xaml::Uri::CreateUri(uri.as_str())?;
        handle
            .cast_inner::<Xaml::IHyperlinkButton>()?
            .put_NavigateUri(&u)?;
    }
    if !w.is_enabled {
        handle
            .cast_inner::<Xaml::IControl>()?
            .put_IsEnabled(false)?;
    }
    Ok(())
}

pub fn diff(
    old: &HyperlinkButton,
    new: &HyperlinkButton,
    handle: &Handle,
) -> windows_core::Result<()> {
    if new.label != old.label {
        let txt = string_as_textblock(&new.label)?;
        handle
            .cast_inner::<Xaml::IContentControl>()?
            .put_Content(&txt)?;
    }
    if new.navigate_uri != old.navigate_uri {
        let h = handle.cast_inner::<Xaml::IHyperlinkButton>()?;
        match &new.navigate_uri {
            Some(uri) => h.put_NavigateUri(&Xaml::Uri::CreateUri(uri.as_str())?)?,
            None => h.put_NavigateUri(None)?,
        }
    }
    if new.is_enabled != old.is_enabled {
        if new.is_enabled {
            handle
                .cast_inner::<Xaml::IDependencyObject>()?
                .ClearValue(&Xaml::Control::get_IsEnabledProperty()?)?;
        } else {
            handle
                .cast_inner::<Xaml::IControl>()?
                .put_IsEnabled(false)?;
        }
    }
    Ok(())
}
