use super::*;

#[derive(Clone, Copy)]
struct NativeCompositionLayout {
    width: f32,
    height: f32,
    rasterization_scale: f32,
}

pub(super) struct CompositionHostState {
    _value: bindings::Grid,
    ui: bindings::UIElement,
    framework: bindings::FrameworkElement,
    content: Option<Box<dyn std::any::Any>>,
    root: Option<windows_composition::Visual>,
    layout: Option<crate::composition::CompositionLayoutCallback>,
    metrics: Rc<Cell<NativeCompositionLayout>>,
    loaded: Rc<Cell<bool>>,
    xaml_root: Option<bindings::XamlRoot>,
    scale_revoker: Option<windows_core::EventRevoker>,
    _revokers: [windows_core::EventRevoker; 2],
}

impl CompositionHostState {
    pub(super) fn ui_element(&self) -> bindings::UIElement {
        self.ui.clone()
    }

    pub(super) fn framework_element(&self) -> bindings::FrameworkElement {
        self.framework.clone()
    }

    pub(super) fn detach(&mut self) -> WindowsResult<()> {
        bindings::ElementCompositionPreview::SetElementChildVisual(
            &self.ui,
            None::<&bindings::Visual>,
        )?;
        self.content = None;
        self.root = None;
        self.layout = None;
        self.scale_revoker = None;
        self.xaml_root = None;
        Ok(())
    }
}

fn queue_composition_layout(
    target: NodeId,
    layout: NativeCompositionLayout,
    events: &RefCell<VecDeque<NativeEvent>>,
    waker: &RefCell<Option<Rc<dyn Fn()>>>,
) {
    queue_latest_event(
        events,
        NativeEvent::CompositionLayout {
            target,
            width: layout.width,
            height: layout.height,
            rasterization_scale: layout.rasterization_scale,
        },
    );
    if let Some(wake) = waker.borrow().as_ref() {
        wake();
    }
}

impl WinUiRuntime {
    pub(super) fn create_composition_host(&mut self, id: NodeId) -> WindowsResult<Handle> {
        let value = bindings::Grid::new()?;
        let ui = value.cast()?;
        let framework: bindings::FrameworkElement = value.cast()?;
        let metrics = Rc::new(Cell::new(NativeCompositionLayout {
            width: 0.0,
            height: 0.0,
            rasterization_scale: 1.0,
        }));
        let loaded = Rc::new(Cell::new(false));

        let size_metrics = Rc::clone(&metrics);
        let size_events = Rc::clone(&self.events);
        let size_waker = Rc::clone(&self.waker);
        let size_changed = framework.SizeChanged(move |_sender, args| {
            let size = args.as_ref().unwrap().NewSize().unwrap();
            let mut current = size_metrics.get();
            current.width = size.width;
            current.height = size.height;
            size_metrics.set(current);
            queue_composition_layout(id, current, &size_events, &size_waker);
        })?;

        let loaded_metrics = Rc::clone(&metrics);
        let loaded_state = Rc::clone(&loaded);
        let loaded_events = Rc::clone(&self.events);
        let loaded_waker = Rc::clone(&self.waker);
        let loaded_revoker = framework.Loaded(move |_sender, _args| {
            loaded_state.set(true);
            queue_composition_layout(id, loaded_metrics.get(), &loaded_events, &loaded_waker);
        })?;

        Ok(Handle::CompositionHost(Box::new(CompositionHostState {
            _value: value,
            ui,
            framework,
            content: None,
            root: None,
            layout: None,
            metrics,
            loaded,
            xaml_root: None,
            scale_revoker: None,
            _revokers: [size_changed, loaded_revoker],
        })))
    }

    fn subscribe_composition_host_scale(&mut self, id: NodeId) -> WindowsResult<()> {
        let (loaded, ui) = {
            let Handle::CompositionHost(state) = &self.node(id)?.handle else {
                panic!("Composition host scale target is not a CompositionHost");
            };
            (state.loaded.get(), state.ui.clone())
        };
        if !loaded {
            return Ok(());
        }

        let xaml_root = ui.XamlRoot()?;
        let subscribed_to_root = {
            let Handle::CompositionHost(state) = &self.node(id)?.handle else {
                unreachable!()
            };
            state.scale_revoker.is_some() && state.xaml_root.as_ref() == Some(&xaml_root)
        };
        if subscribed_to_root {
            return Ok(());
        }

        let scale = xaml_root.RasterizationScale()? as f32;
        let metrics = {
            let Handle::CompositionHost(state) = &self.node(id)?.handle else {
                unreachable!()
            };
            Rc::clone(&state.metrics)
        };
        let events = Rc::clone(&self.events);
        let waker = Rc::clone(&self.waker);
        let changed = xaml_root.Changed(move |sender, _args| {
            let mut current = metrics.get();
            current.rasterization_scale =
                sender.as_ref().unwrap().RasterizationScale().unwrap() as f32;
            metrics.set(current);
            queue_composition_layout(id, current, &events, &waker);
        })?;

        let Handle::CompositionHost(state) = &mut self.node_mut(id)?.handle else {
            unreachable!()
        };
        let mut current = state.metrics.get();
        current.rasterization_scale = scale;
        state.metrics.set(current);
        state.xaml_root = Some(xaml_root);
        state.scale_revoker = Some(changed);
        Ok(())
    }

    pub(super) fn apply_composition_host_update(
        &mut self,
        id: NodeId,
        update: &CompositionHostUpdate,
    ) -> WindowsResult<()> {
        match update {
            CompositionHostUpdate::Initialize { factory, layout } => {
                let element_visual = {
                    let Handle::CompositionHost(state) = &self.node(id)?.handle else {
                        panic!("CompositionHost update target is not a CompositionHost");
                    };
                    bindings::ElementCompositionPreview::GetElementVisual(&state.ui)?
                };
                let visual = windows_composition::Visual::from_host(element_visual.into())?;
                let (content, root) = factory.create(&visual.compositor())?;
                let child = root.as_raw().cast::<bindings::Visual>()?;
                let Handle::CompositionHost(state) = &mut self.node_mut(id)?.handle else {
                    unreachable!()
                };
                bindings::ElementCompositionPreview::SetElementChildVisual(&state.ui, &child)?;
                state.root = Some(root);
                state.content = Some(content);
                state.layout = Some(layout.clone());
                self.subscribe_composition_host_scale(id)
            }
            CompositionHostUpdate::LayoutCallback(layout) => {
                let Handle::CompositionHost(state) = &mut self.node_mut(id)?.handle else {
                    panic!("CompositionHost update target is not a CompositionHost");
                };
                state.layout = Some(layout.clone());
                Ok(())
            }
            CompositionHostUpdate::Action(action) => {
                let Handle::CompositionHost(state) = &mut self.node_mut(id)?.handle else {
                    panic!("CompositionHost update target is not a CompositionHost");
                };
                let content = state
                    .content
                    .as_mut()
                    .expect("CompositionHost action ran before initialization");
                action.call(content.as_mut())?;
                Ok(())
            }
        }
    }

    pub(super) fn apply_composition_layout(
        &mut self,
        id: NodeId,
        _width: f32,
        _height: f32,
        _rasterization_scale: f32,
    ) -> WindowsResult<()> {
        self.subscribe_composition_host_scale(id)?;
        let Handle::CompositionHost(state) = &mut self.node_mut(id)?.handle else {
            panic!("CompositionHost layout target is not a CompositionHost");
        };
        let current = state.metrics.get();
        let layout = crate::composition::CompositionHostLayout {
            width: current.width,
            height: current.height,
            rasterization_scale: current.rasterization_scale,
        };
        if let (Some(callback), Some(content)) = (&state.layout, state.content.as_mut()) {
            callback.call(content.as_mut(), layout)?;
        }
        Ok(())
    }
}
