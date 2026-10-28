use super::*;

/// Image load results that describe the current source; a source change discards queued results.
pub(super) const IMAGE_SOURCE_EVENTS: [EventId; 2] = [EventId::ImageOpened, EventId::ImageFailed];

pub(super) struct NativeImageSource {
    pub(super) value: native::ImageSource,
    state: Rc<ImageLoadState>,
    _opened: windows_core::EventRevoker,
    _failed: windows_core::EventRevoker,
}

pub(super) struct ImageSubscription {
    pub(super) property: PropertyId,
    pub(super) image: Rc<NativeImageSource>,
    pub(super) _callback: Option<Rc<dyn Fn(bool)>>,
}

/// Native image sources shared by a window's controls.
///
/// A URI source lives while a control uses it. A source decoded from [`EncodedImage`] bytes lives
/// while the app holds those bytes, so controls that leave and return render without decoding
/// again. Clones of an `EncodedImage` share its allocation and therefore one source.
pub(super) struct ImageCache<T = NativeImageSource> {
    uris: HashMap<Rc<str>, Weak<T>>,
    pub(super) bytes: HashMap<(usize, usize, bool), (Option<SyncWeak<[u8]>>, Rc<T>)>,
}

impl<T> Default for ImageCache<T> {
    fn default() -> Self {
        Self {
            uris: HashMap::new(),
            bytes: HashMap::new(),
        }
    }
}

impl<T> ImageCache<T> {
    pub(super) fn get_or_insert<E>(
        &mut self,
        source: &ImageSourceValue,
        create: impl FnOnce() -> Result<T, E>,
    ) -> Result<Rc<T>, E> {
        match source {
            ImageSourceValue::Uri(uri) => {
                if let Some(image) = self.uris.get(uri).and_then(Weak::upgrade) {
                    return Ok(image);
                }
                let image = Rc::new(create()?);
                self.uris.insert(Rc::clone(uri), Rc::downgrade(&image));
                Ok(image)
            }
            ImageSourceValue::Encoded(bytes) | ImageSourceValue::Svg(bytes) => {
                let key = (
                    bytes.as_bytes().as_ptr() as usize,
                    bytes.as_bytes().len(),
                    matches!(source, ImageSourceValue::Svg(_)),
                );
                if let Some((_, image)) = self.bytes.get(&key) {
                    return Ok(Rc::clone(image));
                }
                let image = Rc::new(create()?);
                self.bytes
                    .insert(key, (bytes.downgrade(), Rc::clone(&image)));
                Ok(image)
            }
        }
    }

    /// Drops URI sources no control uses and byte sources whose bytes the app has released.
    pub(super) fn prune(&mut self) {
        self.uris.retain(|_, image| image.strong_count() != 0);
        // Holding a weak reference keeps the allocation, and so its key, from being reused.
        self.bytes
            .retain(|_, (owner, _)| owner.as_ref().is_none_or(|owner| owner.strong_count() != 0));
    }
}

#[derive(Default)]
pub(super) struct ImageLoadState {
    result: Cell<Option<bool>>,
    pub(super) subscribers: RefCell<Vec<Weak<dyn Fn(bool)>>>,
}

impl ImageLoadState {
    pub(super) fn complete(&self, result: bool) {
        if self.result.get().is_none() {
            self.result.set(Some(result));
            let subscribers = std::mem::take(&mut *self.subscribers.borrow_mut());
            for subscriber in subscribers {
                if let Some(subscriber) = subscriber.upgrade() {
                    subscriber(result);
                }
            }
        }
    }

    pub(super) fn subscribe(&self, callback: &Rc<dyn Fn(bool)>) -> Option<bool> {
        if self.result.get().is_none() {
            let mut subscribers = self.subscribers.borrow_mut();
            subscribers.retain(|subscriber| subscriber.strong_count() != 0);
            subscribers.push(Rc::downgrade(callback));
        }
        self.result.get()
    }
}

impl NativeImageSource {
    fn new(source: &ImageSourceValue) -> Result<Self, WinUiError> {
        let state = Rc::new(ImageLoadState::default());
        let opened = Rc::clone(&state);
        let failed = Rc::clone(&state);
        let svg = matches!(source, ImageSourceValue::Svg(_))
            || matches!(source, ImageSourceValue::Uri(uri)
                if uri.split(['?', '#']).next().unwrap().to_ascii_lowercase().ends_with(".svg"));
        let (value, opened, failed) = if svg {
            let image = native::SvgImageSource::new()?;
            let opened = image.Opened(move |_, _| opened.complete(true))?;
            let failed = image.OpenFailed(move |_, _| failed.complete(false))?;
            match source {
                ImageSourceValue::Uri(uri) => image.SetUriSource(&native::Uri::CreateUri(uri)?)?,
                ImageSourceValue::Svg(value) => {
                    let decode_image = image.clone();
                    load_encoded_image(value, move |stream| {
                        decode_image.SetSourceAsync(stream).map(drop)
                    })?;
                }
                ImageSourceValue::Encoded(_) => unreachable!(),
            }
            (image.cast()?, opened, failed)
        } else {
            let image = native::BitmapImage::new()?;
            let opened = image.ImageOpened(move |_, _| opened.complete(true))?;
            let failed = image.ImageFailed(move |_, _| failed.complete(false))?;
            match source {
                ImageSourceValue::Uri(uri) => image.SetUriSource(&native::Uri::CreateUri(uri)?)?,
                ImageSourceValue::Encoded(value) => {
                    let decode_image = image.clone();
                    load_encoded_image(value, move |stream| {
                        decode_image
                            .cast::<native::IBitmapSource>()?
                            .SetSourceAsync(stream)
                            .map(drop)
                    })?;
                }
                ImageSourceValue::Svg(_) => unreachable!(),
            }
            (image.cast()?, opened, failed)
        };
        Ok(Self {
            value,
            state,
            _opened: opened,
            _failed: failed,
        })
    }
}

pub(super) fn icon_element(
    value: &Icon,
    image_source: Option<&native::ImageSource>,
) -> Result<native::IconElement, WinUiError> {
    match value.value() {
        IconValue::Symbol(value) => {
            let icon = native::SymbolIcon::new()?;
            icon.SetSymbol(native_symbol(*value))?;
            icon.cast().map_err(Into::into)
        }
        IconValue::Font(value) => {
            let icon = native::FontIcon::new()?;
            icon.SetGlyph(value)?;
            icon.cast().map_err(Into::into)
        }
        IconValue::Bitmap {
            uri,
            show_as_monochrome,
        } => {
            let icon = native::BitmapIcon::new()?;
            icon.SetUriSource(&native::Uri::CreateUri(uri)?)?;
            icon.SetShowAsMonochrome(*show_as_monochrome)?;
            icon.cast().map_err(Into::into)
        }
        IconValue::Image(_) => {
            let icon = native::ImageIcon::new()?;
            icon.SetSource(image_source.unwrap())?;
            icon.cast().map_err(Into::into)
        }
        IconValue::Path(value) => {
            let icon = native::PathIcon::new()?;
            icon.SetData(&parse_path_data(value)?)?;
            icon.cast().map_err(Into::into)
        }
    }
}

pub(super) fn icon_source(
    value: &Icon,
    image_source: Option<&native::ImageSource>,
) -> Result<native::IconSource, WinUiError> {
    match value.value() {
        IconValue::Symbol(value) => {
            let icon = native::SymbolIconSource::new()?;
            icon.SetSymbol(native_symbol(*value))?;
            icon.cast().map_err(Into::into)
        }
        IconValue::Font(value) => {
            let icon = native::FontIconSource::new()?;
            icon.SetGlyph(value)?;
            icon.cast().map_err(Into::into)
        }
        IconValue::Bitmap {
            uri,
            show_as_monochrome,
        } => {
            let icon = native::BitmapIconSource::new()?;
            icon.SetUriSource(&native::Uri::CreateUri(uri)?)?;
            icon.SetShowAsMonochrome(*show_as_monochrome)?;
            icon.cast().map_err(Into::into)
        }
        IconValue::Image(_) => {
            let icon = native::ImageIconSource::new()?;
            icon.SetImageSource(image_source.unwrap())?;
            icon.cast().map_err(Into::into)
        }
        IconValue::Path(value) => {
            let icon = native::PathIconSource::new()?;
            icon.SetData(&parse_path_data(value)?)?;
            icon.cast().map_err(Into::into)
        }
    }
}

fn load_encoded_image(
    value: &EncodedImage,
    load: impl FnOnce(&native::InMemoryRandomAccessStream) -> windows_core::Result<()> + Send + 'static,
) -> Result<(), WinUiError> {
    let stream = native::InMemoryRandomAccessStream::new()?;
    let output = stream.GetOutputStreamAt(0)?;
    let writer = native::DataWriter::CreateDataWriter(&output)?;
    writer.WriteBytes(value.as_bytes())?;
    let store = writer.StoreAsync()?;
    store.when(move |result| {
        result.unwrap();
        writer.DetachStream().unwrap();
        stream.Seek(0).unwrap();
        load(&stream).unwrap();
    })?;
    Ok(())
}

impl WinUiAdapter {
    fn shared_image(&mut self, source: &ImageSource) -> Result<Rc<NativeImageSource>, WinUiError> {
        self.images
            .get_or_insert(source.value(), || NativeImageSource::new(source.value()))
    }

    pub(super) fn shared_icon_image(
        &mut self,
        icon: &Icon,
    ) -> Result<Option<Rc<NativeImageSource>>, WinUiError> {
        match icon.value() {
            IconValue::Image(source) => self.shared_image(source).map(Some),
            _ => Ok(None),
        }
    }

    pub(super) fn release_image(&mut self, object: ObjectId) {
        // Keep released images until the batch completes so replacement controls can share them.
        if let Some(subscription) = self.image_subscriptions.remove(&object) {
            self.released_images.push(subscription.image);
        }
    }

    pub(super) fn set_image_source(
        &mut self,
        object: ObjectId,
        source: &ImageSource,
    ) -> Result<(), WinUiError> {
        let image = self.shared_image(source)?;
        let callback = match self.handles.get(&object) {
            Some(Handle::Generated(GeneratedHandle::Image(control))) => {
                control.value.SetSource(&image.value)?;
                self.event_queue
                    .discard_events(object, &IMAGE_SOURCE_EVENTS);
                control.source.set(true);
                let opened = Rc::clone(&control.image_opened);
                let failed = Rc::clone(&control.image_failed);
                let event_queue = Rc::clone(&self.event_queue);
                let callback: Rc<dyn Fn(bool)> = Rc::new(move |success| {
                    let (event, id) = if success {
                        (&opened, EventId::ImageOpened)
                    } else {
                        (&failed, EventId::ImageFailed)
                    };
                    Self::dispatch(event, &event_queue, object, id, None, ());
                });
                Some(callback)
            }
            Some(Handle::Generated(GeneratedHandle::ImageIcon(control))) => {
                control.SetSource(&image.value)?;
                None
            }
            _ => return Err(WinUiError::InvalidObject(object)),
        };
        self.release_image(object);
        let result = callback
            .as_ref()
            .and_then(|callback| image.state.subscribe(callback));
        let replay = callback.as_ref().map(Rc::downgrade);
        self.image_subscriptions.insert(
            object,
            ImageSubscription {
                property: PropertyId::Source,
                image,
                _callback: callback,
            },
        );
        if let (Some(result), Some(replay)) = (result, replay) {
            // Replay a completed load after the current batch applies event handlers.
            let handler = native::DispatcherQueueHandler::new(move || {
                if let Some(callback) = replay.upgrade() {
                    callback(result);
                }
            });
            // The replay is dropped if the dispatcher is shutting down.
            native::DispatcherQueue::GetForCurrentThread()?
                .TryEnqueueWithPriority(native::DispatcherQueuePriority::Normal, &handler)?;
        }
        Ok(())
    }
}
