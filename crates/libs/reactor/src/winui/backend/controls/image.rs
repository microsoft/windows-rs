//! Typed handler for the `Image` widget.

use crate::bindings as Xaml;
use crate::core::widgets::{Image, ImageStretch};
use crate::winui::backend::Handle;
use windows_core::Interface as _;

pub fn mount(widget: &Image, handle: &Handle) -> windows_core::Result<bool> {
    let Handle::Image(img) = handle else {
        return Ok(false);
    };

    if !widget.source.is_empty() {
        set_source(img, &widget.source)?;
    }

    set_stretch(img, widget.stretch)?;
    Ok(true)
}

pub fn diff(old: &Image, new: &Image, handle: &Handle) -> windows_core::Result<bool> {
    let Handle::Image(img) = handle else {
        return Ok(false);
    };

    if old.source != new.source {
        if new.source.is_empty() {
            img.put_Source(None)?;
        } else {
            set_source(img, &new.source)?;
        }
    }

    if old.stretch != new.stretch {
        set_stretch(img, new.stretch)?;
    }

    Ok(true)
}

fn set_source(img: &Xaml::Image, source: &str) -> windows_core::Result<()> {
    let uri = Xaml::Uri::CreateUri(source)?;
    let bmp = Xaml::BitmapImage::new()?;
    bmp.cast::<Xaml::IBitmapImage>()?.put_UriSource(&uri)?;
    img.put_Source(&bmp.cast::<Xaml::ImageSource>()?)
}

fn set_stretch(img: &Xaml::Image, stretch: ImageStretch) -> windows_core::Result<()> {
    let mapped = match stretch {
        ImageStretch::Uniform => Xaml::Stretch::Uniform,
        ImageStretch::UniformToFill => Xaml::Stretch::UniformToFill,
        ImageStretch::Fill => Xaml::Stretch::Fill,
        ImageStretch::None => Xaml::Stretch::None,
    };
    img.put_Stretch(mapped)
}
