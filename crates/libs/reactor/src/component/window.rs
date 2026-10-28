use super::*;

pub struct WindowHandle<'a> {
    raw: *mut core::ffi::c_void,
    marker: PhantomData<(&'a mut (), Rc<()>)>,
}

impl WindowHandle<'_> {
    pub(super) fn new(raw: *mut core::ffi::c_void) -> Self {
        Self {
            raw,
            marker: PhantomData,
        }
    }

    pub fn as_raw(&self) -> *mut core::ffi::c_void {
        self.raw
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct WindowPublication {
    pub(crate) on_color_scheme: Option<WindowObserverDeclaration<ColorScheme>>,
    pub(crate) on_size: Option<WindowObserverDeclaration<WindowSize>>,
    pub(crate) on_placement: Option<WindowObserverDeclaration<WindowPlacement>>,
    pub(crate) title: Option<String>,
    pub(crate) visuals: Option<WindowVisuals>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct WindowObserverDeclaration<T> {
    pub(crate) owner: ComponentId,
    pub(crate) generation: u64,
    pub(crate) callback: Callback<T>,
}

pub(super) struct WindowObserverLifetime<T> {
    owner: Option<ComponentId>,
    generation: u64,
    callback: Option<Callback<T>>,
}

impl<T> Default for WindowObserverLifetime<T> {
    fn default() -> Self {
        Self {
            owner: None,
            generation: 0,
            callback: None,
        }
    }
}

impl<T> WindowObserverLifetime<T> {
    fn commit(&mut self, observer: &mut Option<WindowObserverDeclaration<T>>) {
        let owner = observer.as_ref().map(|observer| observer.owner);
        if self.owner != owner {
            self.generation = self.generation.checked_add(1).unwrap();
            self.owner = owner;
        }
        if let Some(observer) = observer {
            observer.generation = self.generation;
        }
        self.callback = observer.as_ref().map(|observer| observer.callback.clone());
    }

    fn callback(&self, generation: u64) -> Option<Callback<T>> {
        (self.generation == generation)
            .then(|| self.callback.clone())
            .flatten()
    }
}

impl<A: Adapter> ComponentHost<A> {
    pub(crate) fn window_color_scheme_callback(
        &self,
        generation: u64,
    ) -> Option<Callback<ColorScheme>> {
        self.window_color_scheme.callback(generation)
    }

    pub(crate) fn window_size_callback(&self, generation: u64) -> Option<Callback<WindowSize>> {
        self.window_size.callback(generation)
    }

    pub(crate) fn window_placement_callback(
        &self,
        generation: u64,
    ) -> Option<Callback<WindowPlacement>> {
        self.window_placement.callback(generation)
    }

    pub(super) fn resolve_window_publication(
        &self,
        pending: &[PendingScopeRender],
        retired: &[ComponentId],
    ) -> WindowPublication {
        let mut publication = WindowPublication::default();
        for (index, slot) in self.scopes.iter().enumerate() {
            let Some(scope) = &slot.scope else {
                continue;
            };
            let id = ComponentId {
                index: u32::try_from(index).unwrap(),
                generation: slot.generation,
            };
            if retired.iter().any(|retired| self.owns_scope(*retired, id)) {
                continue;
            }
            let window = match pending.iter().find(|pending| pending.id == id) {
                Some(pending) => pending.window.as_deref(),
                None => scope.window.as_deref(),
            };
            let Some(window) = window else {
                continue;
            };
            if let Some(on_color_scheme) = &window.on_color_scheme
                && publication
                    .on_color_scheme
                    .replace(on_color_scheme.clone())
                    .is_some()
            {
                panic!("color scheme observer declared by more than one component");
            }
            if let Some(on_size) = &window.on_size
                && publication.on_size.replace(on_size.clone()).is_some()
            {
                panic!("window size observer declared by more than one component");
            }
            if let Some(on_placement) = &window.on_placement
                && publication
                    .on_placement
                    .replace(on_placement.clone())
                    .is_some()
            {
                panic!("window placement observer declared by more than one component");
            }
            if let Some(title) = &window.title
                && publication.title.replace(title.clone()).is_some()
            {
                panic!("window title declared by more than one component");
            }
            if let Some(visuals) = &window.visuals
                && publication.visuals.replace(visuals.clone()).is_some()
            {
                panic!("window visuals declared by more than one component");
            }
        }
        publication
    }

    pub(super) fn publish_committed_window(&mut self, mut publication: WindowPublication) {
        // Record lifetime transitions before the native queue can coalesce publications.
        self.window_color_scheme
            .commit(&mut publication.on_color_scheme);
        self.window_size.commit(&mut publication.on_size);
        self.window_placement.commit(&mut publication.on_placement);
        self.ui_services.publish_window(publication);
    }

    pub(super) fn publish_window(&mut self) {
        let publication = self.resolve_window_publication(&[], &[]);
        self.publish_committed_window(publication);
    }
}
