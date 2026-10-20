use super::*;
use ClassHierarchy::*;

pub fn factory(name: &str) -> Option<IActivationFactory> {
    (name == "ClassHierarchy.Compositor")
        .then(activatable::<Compositor, ClassHierarchy::Compositor>)
}

#[implement(ClassHierarchy::Compositor)]
#[derive(Default)]
struct Compositor;

impl ICompositor_Impl for Compositor_Impl {
    fn CreateContainerVisual(&self, children: i32) -> Result<ClassHierarchy::ContainerVisual> {
        Ok(ContainerVisual::new(self, children).into())
    }

    fn CreateSpriteVisual(&self, brush: i32) -> Result<ClassHierarchy::SpriteVisual> {
        Ok(SpriteVisual {
            container: ContainerVisual::new(self, brush * 2),
            brush,
        }
        .into())
    }

    fn Echo(&self, visual: Ref<Visual>) -> Result<Visual> {
        visual.ok().cloned()
    }
}

/// An instance implements the interfaces of its class and every base class.
#[implement(ClassHierarchy::ContainerVisual, ClassHierarchy::Visual)]
struct ContainerVisual {
    compositor: ComObject<Compositor>,
    children: i32,
}

impl ContainerVisual {
    fn new(compositor: &Compositor_Impl, children: i32) -> Self {
        Self {
            compositor: compositor.to_object(),
            children,
        }
    }
}

impl IContainerVisual_Impl for ContainerVisual_Impl {
    fn Children(&self) -> i32 {
        self.children
    }
}

impl IVisual_Impl for ContainerVisual_Impl {
    fn Compositor(&self) -> Result<ClassHierarchy::Compositor> {
        Ok(self.compositor.to_interface())
    }
}

#[implement(
    ClassHierarchy::SpriteVisual,
    ClassHierarchy::ContainerVisual,
    ClassHierarchy::Visual
)]
struct SpriteVisual {
    container: ContainerVisual,
    brush: i32,
}

impl ISpriteVisual_Impl for SpriteVisual_Impl {
    fn Brush(&self) -> i32 {
        self.brush
    }
}

impl IContainerVisual_Impl for SpriteVisual_Impl {
    fn Children(&self) -> i32 {
        self.container.children
    }
}

impl IVisual_Impl for SpriteVisual_Impl {
    fn Compositor(&self) -> Result<ClassHierarchy::Compositor> {
        Ok(self.container.compositor.to_interface())
    }
}
