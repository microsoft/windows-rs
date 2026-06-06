use super::*;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Image {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub source: ImageSource,
    pub stretch: ImageStretch,
}
#[derive(Clone, Default, Debug, PartialEq)]
pub enum ImageSource {
    #[default]
    None,
    Uri(String),
    Surface(SurfaceImageSource),
}
impl From<SurfaceImageSource> for ImageSource {
    fn from(source: SurfaceImageSource) -> Self {
        ImageSource::Surface(source)
    }
}
impl From<Option<SurfaceImageSource>> for ImageSource {
    fn from(source: Option<SurfaceImageSource>) -> Self {
        source.map_or(ImageSource::None, ImageSource::Surface)
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum ImageStretch {
    #[default]
    Uniform,
    UniformToFill,
    Fill,
    None,
}
impl Image {
    pub fn new(source: ImageSource) -> Self {
        Self {
            source,
            ..Default::default()
        }
    }

    pub fn new_with_uri(source: impl Into<String>) -> Self {
        Self::new(ImageSource::Uri(source.into()))
    }

    pub fn stretch(mut self, v: ImageStretch) -> Self {
        self.stretch = v;
        self
    }
}

impl Widget for Image {
    widget_header!(ControlKind::Image);
    fn bindings(&self) -> PropBindings {
        let mut out = crate::core::generated_bindings::image_bindings(self);
        // ImageSource is a compound type not expressible in TOML.
        match &self.source {
            ImageSource::Uri(uri) => {
                out.push(Binding::Prop(Prop::ImageSource, PropValue::Str(uri.clone())));
            }
            ImageSource::Surface(s) => {
                out.push(Binding::Prop(
                    Prop::ImageSource,
                    PropValue::SurfaceImageSource(s.clone()),
                ));
            }
            ImageSource::None => {}
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn image_source(image: &Image) -> Option<PropValue> {
        image.bindings().into_iter().find_map(|b| match b {
            Binding::Prop(Prop::ImageSource, v) => Some(v),
            _ => None,
        })
    }

    #[test]
    fn new_emits_uri_image_source() {
        let image = Image::new_with_uri("file:///pic.png");
        assert_eq!(
            image_source(&image),
            Some(PropValue::Str("file:///pic.png".into()))
        );
    }

    #[test]
    fn default_emits_no_image_source() {
        // An `Image` with no source emits no `ImageSource` binding and
        // no `Stretch` binding (Stretch is `non_default`, so the default
        // value `Uniform` is omitted).
        let image = Image::default();
        assert_eq!(image_source(&image), None);
        assert!(image.bindings().is_empty());
    }
}
