//! Image — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Prop, PropValue};
use windows_core::Interface;

pub(in crate::winui::backend) fn set_prop(
    img: &Xaml::Image,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::ImageSource, PropValue::Str(s)) => Some((|| {
            let uri = Xaml::Uri::CreateUri(s.as_str())?;
            let bmp = Xaml::BitmapImage::new()?;
            bmp.cast::<Xaml::IBitmapImage>()?.put_UriSource(&uri)?;
            img.put_Source(&bmp.cast::<Xaml::ImageSource>()?)
        })()),
        (Prop::ImageSource, PropValue::Unset) => Some(img.put_Source(None)),
        (Prop::ImageStretch, PropValue::ImageStretch(s)) => {
            use crate::core::widgets::ImageStretch as E;
            use Xaml::Stretch as X;
            let mapped = match s {
                E::Uniform => X::Uniform,
                E::UniformToFill => X::UniformToFill,
                E::Fill => X::Fill,
                E::None => X::None,
            };
            Some(img.put_Stretch(mapped))
        }
        _ => None,
    }
}
