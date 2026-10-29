use super::*;

/// Image load results that describe the current source; a source change discards queued results.
pub(super) const IMAGE_SOURCE_EVENTS: [EventId; 2] = [EventId::ImageOpened, EventId::ImageFailed];

pub(super) struct NativeImageSource {
    pub(super) value: native::ImageSource,
    deferred: RefCell<Option<DeferredLoad>>,
    state: Rc<ImageLoadState>,
    _opened: windows_core::EventRevoker,
    _failed: windows_core::EventRevoker,
}

/// SVG bytes waiting for their source to enter the live tree.
///
/// WinUI rasterizes an `SvgImageSource` at its rendered size only when its stream is set while
/// the source is live. A stream set earlier rasterizes at the SVG's natural size or the window
/// size, and then scales.
struct DeferredLoad {
    bytes: EncodedImage,
    _loaded: Option<windows_core::EventRevoker>,
}

pub(super) struct ImageSubscription {
    pub(super) property: PropertyId,
    pub(super) image: ImageUse,
    pub(super) _callback: Option<Rc<dyn Fn(bool)>>,
}

/// A control's use of a cached image source.
///
/// WinUI rasterizes an SVG source once for all of its controls, so an SVG source has one user at
/// a time and becomes available to another control when this use is dropped.
pub(super) struct ImageUse<T = NativeImageSource> {
    image: Rc<T>,
    used: Option<Rc<Cell<bool>>>,
}

impl<T> ImageUse<T> {
    /// Ends the use, returning the source so it can be kept alive for a replacement control.
    pub(super) fn release(self) -> Rc<T> {
        Rc::clone(&self.image)
    }
}

impl<T> std::ops::Deref for ImageUse<T> {
    type Target = Rc<T>;

    fn deref(&self) -> &Rc<T> {
        &self.image
    }
}

impl<T> Drop for ImageUse<T> {
    fn drop(&mut self) {
        if let Some(used) = &self.used {
            used.set(false);
        }
    }
}

pub(super) struct Pooled<P> {
    image: P,
    used: Rc<Cell<bool>>,
}

/// Native image sources shared by a window's controls.
///
/// A URI source lives while a control uses it. A source decoded from [`EncodedImage`] bytes lives
/// while the app holds those bytes, so controls that leave and return render without decoding
/// again. Clones of an `EncodedImage` share its allocation and therefore its sources. Bitmap
/// sources are shared by all controls; SVG sources are pooled with one control per source.
pub(super) struct ImageCache<T = NativeImageSource> {
    uris: HashMap<Rc<str>, Vec<Pooled<Weak<T>>>>,
    pub(super) bytes: HashMap<(usize, usize, bool), (Option<SyncWeak<[u8]>>, Vec<Pooled<Rc<T>>>)>,
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
    ) -> Result<ImageUse<T>, E> {
        let exclusive = is_svg(source);
        match source {
            ImageSourceValue::Uri(uri) => acquire(
                self.uris.entry(Rc::clone(uri)).or_default(),
                exclusive,
                Weak::upgrade,
                Rc::downgrade,
                create,
            ),
            ImageSourceValue::Encoded(bytes) | ImageSourceValue::Svg(bytes) => {
                let key = (
                    bytes.as_bytes().as_ptr() as usize,
                    bytes.as_bytes().len(),
                    exclusive,
                );
                let (_, pool) = self
                    .bytes
                    .entry(key)
                    .or_insert_with(|| (bytes.downgrade(), Vec::new()));
                acquire(
                    pool,
                    exclusive,
                    |image| Some(Rc::clone(image)),
                    Rc::clone,
                    create,
                )
            }
        }
    }

    /// Drops URI sources no control uses, byte sources whose bytes the app has released, and
    /// unused byte sources that are still `pending`.
    ///
    /// A pending source has not started loading and holds its bytes, so a control removed before
    /// it was loaded would otherwise keep the bytes, and the source, alive.
    pub(super) fn prune(&mut self, pending: impl Fn(&T) -> bool) {
        self.uris.retain(|_, pool| {
            pool.retain(|pooled| pooled.image.strong_count() != 0);
            !pool.is_empty()
        });
        // Holding a weak reference keeps the allocation, and so its key, from being reused.
        self.bytes.retain(|_, (owner, pool)| {
            pool.retain(|pooled| pooled.used.get() || !pending(&pooled.image));
            !pool.is_empty() && owner.as_ref().is_none_or(|owner| owner.strong_count() != 0)
        });
    }
}

fn acquire<P, T, E>(
    pool: &mut Vec<Pooled<P>>,
    exclusive: bool,
    upgrade: impl Fn(&P) -> Option<Rc<T>>,
    store: impl FnOnce(&Rc<T>) -> P,
    create: impl FnOnce() -> Result<T, E>,
) -> Result<ImageUse<T>, E> {
    let available = pool.iter().find_map(|pooled| {
        if exclusive && pooled.used.get() {
            return None;
        }
        upgrade(&pooled.image).map(|image| (image, Rc::clone(&pooled.used)))
    });
    let (image, used) = if let Some(available) = available {
        available
    } else {
        let image = Rc::new(create()?);
        let used = Rc::new(Cell::new(false));
        pool.push(Pooled {
            image: store(&image),
            used: Rc::clone(&used),
        });
        (image, used)
    };
    used.set(exclusive);
    Ok(ImageUse {
        image,
        used: exclusive.then_some(used),
    })
}

fn is_svg(source: &ImageSourceValue) -> bool {
    match source {
        ImageSourceValue::Svg(_) => true,
        ImageSourceValue::Encoded(_) => false,
        ImageSourceValue::Uri(uri) => uri
            .split(['?', '#'])
            .next()
            .unwrap()
            .to_ascii_lowercase()
            .ends_with(".svg"),
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
        let svg = is_svg(source);
        let mut deferred = None;
        let (value, opened, failed) = if svg {
            let image = native::SvgImageSource::new()?;
            let opened = image.Opened(move |_, _| opened.complete(true))?;
            let failed = image.OpenFailed(move |_, _| failed.complete(false))?;
            match source {
                ImageSourceValue::Uri(uri) => image.SetUriSource(&native::Uri::CreateUri(uri)?)?,
                ImageSourceValue::Svg(bytes) => {
                    deferred = Some(DeferredLoad {
                        bytes: bytes.clone(),
                        _loaded: None,
                    });
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
            deferred: RefCell::new(deferred),
            state,
            _opened: opened,
            _failed: failed,
        })
    }

    /// Whether this source is waiting for a control to load before it starts loading.
    pub(super) fn is_pending(&self) -> bool {
        self.deferred.borrow().is_some()
    }

    /// Starts a deferred load once `consumer`, the control displaying this source, is loaded.
    pub(super) fn attach(self: &Rc<Self>, consumer: &native::UIElement) -> Result<(), WinUiError> {
        self.when_loaded(consumer, |image| image.start_deferred())
    }

    /// Starts a deferred load once the icon element of `title_bar` is loaded.
    ///
    /// TitleBar creates its icon element from the icon source and creates it again when its
    /// template is applied, so the element that is displayed is known once the TitleBar loads.
    pub(super) fn attach_title_bar(
        self: &Rc<Self>,
        title_bar: &native::TitleBar,
    ) -> Result<(), WinUiError> {
        let host = title_bar.clone();
        self.when_loaded(&title_bar.cast()?, move |image| {
            image.attach(&host.TemplateSettings()?.IconElement()?.cast()?)
        })
    }

    fn when_loaded(
        self: &Rc<Self>,
        element: &native::UIElement,
        start: impl Fn(&Rc<Self>) -> Result<(), WinUiError> + 'static,
    ) -> Result<(), WinUiError> {
        if self.deferred.borrow().is_none() {
            return Ok(());
        }
        let element = element.cast::<native::IFrameworkElement>()?;
        if element.IsLoaded()? {
            return start(self);
        }
        let image = Rc::downgrade(self);
        let loaded = element.Loaded(move |_, _| {
            if let Some(image) = image.upgrade() {
                start(&image).unwrap();
            }
        })?;
        // Replaces the observer of a previous user that left before loading.
        if let Some(deferred) = self.deferred.borrow_mut().as_mut() {
            deferred._loaded = Some(loaded);
        }
        Ok(())
    }

    fn start_deferred(&self) -> Result<(), WinUiError> {
        let Some(deferred) = self.deferred.take() else {
            return Ok(());
        };
        let image = self.value.cast::<native::SvgImageSource>()?;
        load_encoded_image(&deferred.bytes, move |stream| {
            image.SetSourceAsync(stream).map(drop)
        })
    }
}

pub(super) fn icon_element(
    value: &Icon,
    image: Option<&Rc<NativeImageSource>>,
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
            let image = image.unwrap();
            let icon = native::ImageIcon::new()?;
            icon.SetSource(&image.value)?;
            image.attach(&icon.cast()?)?;
            icon.cast().map_err(Into::into)
        }
        IconValue::Path(value) => {
            let icon = native::PathIcon::new()?;
            icon.SetData(&parse_path_data(value)?)?;
            icon.cast().map_err(Into::into)
        }
    }
}

/// Creates an icon source. The control that receives it creates the icon element, so the caller
/// attaches `image` to that element.
pub(super) fn icon_source(
    value: &Icon,
    image: Option<&Rc<NativeImageSource>>,
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
            icon.SetImageSource(&image.unwrap().value)?;
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
    fn shared_image(&mut self, source: &ImageSource) -> Result<ImageUse, WinUiError> {
        self.images
            .get_or_insert(source.value(), || NativeImageSource::new(source.value()))
    }

    /// Returns the image source for `icon`, if it shows an image.
    pub(super) fn shared_icon_image(
        &mut self,
        icon: &Icon,
    ) -> Result<Option<ImageUse>, WinUiError> {
        match icon.value() {
            IconValue::Image(source) => self.shared_image(source).map(Some),
            _ => Ok(None),
        }
    }

    pub(super) fn release_image(&mut self, object: ObjectId) {
        // Keep released images until the batch completes so replacement controls can share them.
        if let Some(subscription) = self.image_subscriptions.remove(&object) {
            self.released_images.push(subscription.image.release());
        }
    }

    pub(super) fn set_image_source(
        &mut self,
        object: ObjectId,
        source: &ImageSource,
    ) -> Result<(), WinUiError> {
        // Release first so a control keeps its SVG source when the source is set again.
        self.release_image(object);
        let image = self.shared_image(source)?;
        let callback = match self.handles.get(&object) {
            Some(Handle::Generated(GeneratedHandle::Image(control))) => {
                control.value.SetSource(&image.value)?;
                image.attach(&control.value.cast()?)?;
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
                image.attach(&control.cast()?)?;
                None
            }
            _ => return Err(WinUiError::InvalidObject(object)),
        };
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
