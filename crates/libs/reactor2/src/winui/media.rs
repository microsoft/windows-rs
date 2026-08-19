use super::*;

pub(super) struct ImageState {
    value: bindings::Image,
    ui: bindings::UIElement,
    framework: bindings::FrameworkElement,
    source: Option<NativeImageSource>,
}

impl ImageState {
    pub(super) fn ui_element(&self) -> bindings::UIElement {
        self.ui.clone()
    }

    pub(super) fn framework_element(&self) -> bindings::FrameworkElement {
        self.framework.clone()
    }
}

struct NativeImageSource {
    value: bindings::ImageSource,
    _revokers: Box<[windows_core::EventRevoker]>,
}

impl WinUiRuntime {
    pub(super) fn create_image(&self) -> WindowsResult<Handle> {
        let value = bindings::Image::new()?;
        let ui = value.cast()?;
        let framework = value.cast()?;
        Ok(Handle::Image(Box::new(ImageState {
            value,
            ui,
            framework,
            source: None,
        })))
    }

    pub(super) fn apply_image_update(
        &mut self,
        id: NodeId,
        update: &ImageUpdate,
    ) -> WindowsResult<()> {
        let source = update
            .source_changed
            .then(|| {
                create_image_source(
                    &update.source,
                    id,
                    update.source_revision,
                    Rc::clone(&self.events),
                    Rc::clone(&self.waker),
                )
            })
            .transpose()?;
        let Handle::Image(state) = &mut self.node_mut(id)?.handle else {
            panic!("Image update target is not an Image");
        };
        state.value.SetStretch(native_stretch(update.stretch))?;
        if let Some(source) = source {
            state
                .value
                .SetSource(source.as_ref().map(|source| &source.value))?;
            state.source = source;
        }
        Ok(())
    }
}

fn create_image_source(
    source: &ImageSource,
    target: NodeId,
    source_revision: u64,
    events: Rc<RefCell<VecDeque<NativeEvent>>>,
    waker: Rc<RefCell<Option<Rc<dyn Fn()>>>>,
) -> WindowsResult<Option<NativeImageSource>> {
    match source.kind() {
        ImageSourceKind::None => Ok(None),
        ImageSourceKind::Bitmap(uri) => {
            let value = bindings::BitmapImage::new()?;
            let opened_events = Rc::clone(&events);
            let opened_waker = Rc::clone(&waker);
            let opened = value.ImageOpened(move |_sender, _args| {
                opened_events
                    .borrow_mut()
                    .push_back(NativeEvent::ImageLoad {
                        target,
                        source_revision,
                        result: Ok(()),
                    });
                if let Some(wake) = opened_waker.borrow().as_ref() {
                    wake();
                }
            })?;
            let failed = value.ImageFailed(move |_sender, args| {
                let args = args.as_ref().unwrap();
                let message = args.ErrorMessage().unwrap();
                events.borrow_mut().push_back(NativeEvent::ImageLoad {
                    target,
                    source_revision,
                    result: Err(image_load_error(message)),
                });
                if let Some(wake) = waker.borrow().as_ref() {
                    wake();
                }
            })?;
            let uri = bindings::Uri::CreateUri(uri)?;
            value.SetUriSource(&uri)?;
            let source = value.cast()?;
            Ok(Some(NativeImageSource {
                value: source,
                _revokers: Box::new([opened, failed]),
            }))
        }
        ImageSourceKind::Svg(uri) => {
            let value = bindings::SvgImageSource::new()?;
            let opened_events = Rc::clone(&events);
            let opened_waker = Rc::clone(&waker);
            let opened = value.Opened(move |_sender, _args| {
                opened_events
                    .borrow_mut()
                    .push_back(NativeEvent::ImageLoad {
                        target,
                        source_revision,
                        result: Ok(()),
                    });
                if let Some(wake) = opened_waker.borrow().as_ref() {
                    wake();
                }
            })?;
            let failed = value.OpenFailed(move |_sender, args| {
                let args = args.as_ref().unwrap();
                let status = args.Status().unwrap();
                let message = match status {
                    bindings::SvgImageSourceLoadStatus::NetworkError => "SVG network error",
                    bindings::SvgImageSourceLoadStatus::InvalidFormat => "invalid SVG image",
                    bindings::SvgImageSourceLoadStatus::Other => "SVG image load failed",
                    _ => "unknown SVG image load failure",
                };
                events.borrow_mut().push_back(NativeEvent::ImageLoad {
                    target,
                    source_revision,
                    result: Err(image_load_error(message)),
                });
                if let Some(wake) = waker.borrow().as_ref() {
                    wake();
                }
            })?;
            let uri = bindings::Uri::CreateUri(uri)?;
            value.SetUriSource(&uri)?;
            let source = value.cast()?;
            Ok(Some(NativeImageSource {
                value: source,
                _revokers: Box::new([opened, failed]),
            }))
        }
    }
}

fn create_image_source_without_events(
    source: &ImageSource,
) -> WindowsResult<Option<bindings::ImageSource>> {
    match source.kind() {
        ImageSourceKind::None => Ok(None),
        ImageSourceKind::Bitmap(uri) => {
            let value = bindings::BitmapImage::new()?;
            value.SetUriSource(&bindings::Uri::CreateUri(uri)?)?;
            value.cast().map(Some)
        }
        ImageSourceKind::Svg(uri) => {
            let value = bindings::SvgImageSource::new()?;
            value.SetUriSource(&bindings::Uri::CreateUri(uri)?)?;
            value.cast().map(Some)
        }
    }
}

fn image_load_error(message: impl AsRef<str>) -> windows_core::Error {
    windows_core::Error::new(
        windows_core::HRESULT(0x80004005_u32 as i32),
        message.as_ref(),
    )
}

pub(super) fn create_icon(icon: Option<&Icon>) -> WindowsResult<Option<bindings::IconElement>> {
    let Some(icon) = icon else {
        return Ok(None);
    };
    let value = match icon.kind() {
        IconKind::Symbol(symbol) => {
            bindings::SymbolIcon::CreateInstanceWithSymbol(bindings::Symbol(symbol.value()))?
                .cast()?
        }
        IconKind::Font { glyph, family } => {
            let value = bindings::FontIcon::new()?;
            value.SetGlyph(glyph)?;
            value.SetFontFamily(&bindings::FontFamily::CreateInstanceWithName(family)?)?;
            value.cast()?
        }
        IconKind::Bitmap { uri, monochrome } => {
            let value = bindings::BitmapIcon::new()?;
            value.SetUriSource(&bindings::Uri::CreateUri(uri)?)?;
            value.SetShowAsMonochrome(*monochrome)?;
            value.cast()?
        }
        IconKind::Image(source) => {
            let value = bindings::ImageIcon::new()?;
            let source = create_image_source_without_events(source)?;
            value.SetSource(source.as_ref())?;
            let framework: bindings::FrameworkElement = value.cast()?;
            framework.SetMaxWidth(20.0)?;
            framework.SetMaxHeight(20.0)?;
            value.cast()?
        }
        IconKind::Path(data) => create_path_icon(data)?.cast()?,
    };
    Ok(Some(value))
}

impl WinUiRuntime {
    pub(super) fn apply_icon_update(&mut self, id: NodeId, icon: &Icon) -> WindowsResult<()> {
        if let IconKind::Path(data) = icon.kind() {
            let value = create_path_icon(data)?;
            let parent = self
                .node(id)?
                .parent
                .map(|parent| (parent, self.node(id).unwrap().attachment.unwrap()));
            if let Some((parent, _)) = parent {
                self.detach(parent, id)?;
            }
            self.node_mut(id)?.handle = Handle::PathIcon(value);
            if let Some((parent, attachment)) = parent {
                self.attach(parent, id, attachment)?;
            }
            return Ok(());
        }
        match (&self.node(id)?.handle, icon.kind()) {
            (Handle::SymbolIcon(value), IconKind::Symbol(symbol)) => {
                value.SetSymbol(bindings::Symbol(symbol.value()))
            }
            (Handle::FontIcon(value), IconKind::Font { glyph, family }) => {
                value.SetGlyph(glyph)?;
                value.SetFontFamily(&bindings::FontFamily::CreateInstanceWithName(family)?)
            }
            (Handle::BitmapIcon(value), IconKind::Bitmap { uri, monochrome }) => {
                value.SetUriSource(&bindings::Uri::CreateUri(uri)?)?;
                value.SetShowAsMonochrome(*monochrome)
            }
            (Handle::ImageIcon(value), IconKind::Image(source)) => {
                let source = create_image_source_without_events(source)?;
                value.SetSource(source.as_ref())?;
                let framework: bindings::FrameworkElement = value.cast()?;
                framework.SetMaxWidth(20.0)?;
                framework.SetMaxHeight(20.0)
            }
            _ => panic!("icon update target does not match the icon kind"),
        }
    }
}

fn create_path_icon(data: &str) -> WindowsResult<bindings::PathIcon> {
    let data = data
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;");
    bindings::XamlReader::Load(&format!(
        "<PathIcon xmlns='http://schemas.microsoft.com/winfx/2006/xaml/presentation' Data=\"{data}\"/>"
    ))?
    .cast()
}
