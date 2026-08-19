use std::any::{Any, TypeId};
use std::fmt;
use std::marker::PhantomData;
use std::rc::Rc;

use windows_composition::{Compositor, ContainerVisual, ShapeVisual, SpriteVisual, Visual};

use crate::element::tree::ElementKind;
use crate::element::{Element, Framework, RenderCx};
use crate::framework_properties::FrameworkProps;
use crate::references::{ElementRef, NativeElementRef};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CompositionHostLayout {
    pub width: f32,
    pub height: f32,
    pub rasterization_scale: f32,
}

pub struct CompositionContent<T> {
    state: T,
    root: Visual,
}

mod composition_root_sealed {
    pub trait Sealed {}
}

pub trait CompositionRoot: composition_root_sealed::Sealed {
    fn as_visual(&self) -> &Visual;
}

impl composition_root_sealed::Sealed for Visual {}

impl CompositionRoot for Visual {
    fn as_visual(&self) -> &Visual {
        self
    }
}

impl composition_root_sealed::Sealed for ContainerVisual {}

impl CompositionRoot for ContainerVisual {
    fn as_visual(&self) -> &Visual {
        self
    }
}

impl composition_root_sealed::Sealed for SpriteVisual {}

impl CompositionRoot for SpriteVisual {
    fn as_visual(&self) -> &Visual {
        self
    }
}

impl composition_root_sealed::Sealed for ShapeVisual {}

impl CompositionRoot for ShapeVisual {
    fn as_visual(&self) -> &Visual {
        self
    }
}

impl<T> CompositionContent<T> {
    pub fn new(state: T, root: impl CompositionRoot) -> Self {
        Self {
            state,
            root: root.as_visual().clone(),
        }
    }

    pub fn from_visual(state: T, root: Visual) -> Self {
        Self { state, root }
    }
}

struct ErasedCompositionContent {
    state: Box<dyn Any>,
    root: Visual,
}

#[derive(Clone)]
pub(crate) struct CompositionFactory {
    state_type: TypeId,
    callback: Rc<dyn Fn(&Compositor) -> windows_core::Result<ErasedCompositionContent>>,
}

impl CompositionFactory {
    pub(crate) fn new<T: 'static>(
        callback: impl Fn(&Compositor) -> windows_core::Result<CompositionContent<T>> + 'static,
    ) -> Self {
        Self {
            state_type: TypeId::of::<T>(),
            callback: Rc::new(move |compositor| {
                let content = callback(compositor)?;
                Ok(ErasedCompositionContent {
                    state: Box::new(content.state),
                    root: content.root,
                })
            }),
        }
    }

    pub(crate) fn create(
        &self,
        compositor: &Compositor,
    ) -> windows_core::Result<(Box<dyn Any>, Visual)> {
        let content = (self.callback)(compositor)?;
        Ok((content.state, content.root))
    }

    pub(crate) fn state_type(&self) -> TypeId {
        self.state_type
    }
}

impl fmt::Debug for CompositionFactory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("CompositionFactory")
    }
}

impl PartialEq for CompositionFactory {
    fn eq(&self, other: &Self) -> bool {
        self.state_type == other.state_type && Rc::ptr_eq(&self.callback, &other.callback)
    }
}

#[derive(Clone)]
pub(crate) struct CompositionLayoutCallback {
    state_type: TypeId,
    callback: Rc<dyn Fn(&mut dyn Any, CompositionHostLayout) -> windows_core::Result<()>>,
}

impl CompositionLayoutCallback {
    pub(crate) fn new<T: 'static>(
        callback: impl Fn(&mut T, CompositionHostLayout) -> windows_core::Result<()> + 'static,
    ) -> Self {
        Self {
            state_type: TypeId::of::<T>(),
            callback: Rc::new(move |state, layout| {
                callback(state.downcast_mut::<T>().unwrap(), layout)
            }),
        }
    }

    pub(crate) fn call(
        &self,
        state: &mut dyn Any,
        layout: CompositionHostLayout,
    ) -> windows_core::Result<()> {
        (self.callback)(state, layout)
    }

    pub(crate) fn state_type(&self) -> TypeId {
        self.state_type
    }
}

impl fmt::Debug for CompositionLayoutCallback {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("CompositionLayoutCallback")
    }
}

impl PartialEq for CompositionLayoutCallback {
    fn eq(&self, other: &Self) -> bool {
        self.state_type == other.state_type && Rc::ptr_eq(&self.callback, &other.callback)
    }
}

#[derive(Clone)]
pub(crate) struct CompositionAction {
    state_type: TypeId,
    callback: Rc<dyn Fn(&mut dyn Any) -> windows_core::Result<()>>,
}

impl CompositionAction {
    pub(crate) fn new<T: 'static>(
        callback: impl Fn(&mut T) -> windows_core::Result<()> + 'static,
    ) -> Self {
        Self {
            state_type: TypeId::of::<T>(),
            callback: Rc::new(move |state| callback(state.downcast_mut::<T>().unwrap())),
        }
    }

    pub(crate) fn call(&self, state: &mut dyn Any) -> windows_core::Result<()> {
        (self.callback)(state)
    }

    pub(crate) fn state_type(&self) -> TypeId {
        self.state_type
    }
}

impl fmt::Debug for CompositionAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("CompositionAction")
    }
}

impl PartialEq for CompositionAction {
    fn eq(&self, other: &Self) -> bool {
        self.state_type == other.state_type && Rc::ptr_eq(&self.callback, &other.callback)
    }
}

pub(crate) struct CompositionHostProps {
    pub factory: CompositionFactory,
    pub layout: CompositionLayoutCallback,
    pub framework: FrameworkProps,
}

pub struct CompositionHost {
    props: CompositionHostProps,
    reference: NativeElementRef,
}

pub struct CompositionHostRef<T> {
    reference: ElementRef<CompositionHost>,
    marker: PhantomData<fn() -> T>,
}

impl<T: 'static> CompositionHostRef<T> {
    pub fn new() -> Self {
        Self {
            reference: ElementRef::new(),
            marker: PhantomData,
        }
    }

    pub fn is_mounted(&self) -> bool {
        self.reference.is_mounted()
    }

    pub fn update(&self, action: impl Fn(&mut T) -> windows_core::Result<()> + 'static) -> bool {
        self.reference.schedule(|scheduler, target| {
            scheduler.run_composition_action(target, CompositionAction::new(action));
        })
    }
}

impl<T> Clone for CompositionHostRef<T> {
    fn clone(&self) -> Self {
        Self {
            reference: self.reference.clone(),
            marker: PhantomData,
        }
    }
}

impl<T: 'static> Default for CompositionHostRef<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: 'static> fmt::Debug for CompositionHostRef<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CompositionHostRef")
            .field("mounted", &self.is_mounted())
            .finish()
    }
}

impl CompositionHost {
    pub fn new<T: 'static>(
        reference: &CompositionHostRef<T>,
        create: impl Fn(&Compositor) -> windows_core::Result<CompositionContent<T>> + 'static,
        layout: impl Fn(&mut T, CompositionHostLayout) -> windows_core::Result<()> + 'static,
    ) -> Framework<Self> {
        Framework::new(Self {
            props: CompositionHostProps {
                factory: CompositionFactory::new(create),
                layout: CompositionLayoutCallback::new(layout),
                framework: FrameworkProps::default(),
            },
            reference: reference.reference.binding(),
        })
    }

    pub(crate) fn build_with_framework(mut self, framework: FrameworkProps) -> Element {
        self.props.framework = framework;
        let element = Element::new(ElementKind::CompositionHost(Box::new(self.props)));
        Element::new(ElementKind::Reference {
            reference: self.reference,
            child: Box::new(element),
        })
    }
}

impl RenderCx<'_> {
    pub fn use_composition_host_ref<T: 'static>(&mut self) -> CompositionHostRef<T> {
        CompositionHostRef {
            reference: self.use_element_ref(),
            marker: PhantomData,
        }
    }
}
