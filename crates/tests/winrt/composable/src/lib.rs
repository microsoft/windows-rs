mod bindings;
use windows::{core::*, Win32::Foundation::*, Win32::System::WinRT::*};

#[no_mangle]
unsafe extern "system" fn DllGetActivationFactory(
    name: Ref<HSTRING>,
    factory: OutRef<IActivationFactory>,
) -> HRESULT {
    if *name == "test_composable.Compositor" {
        factory.write(Some(CompositorFactory.into())).into()
    } else {
        _ = factory.write(None);
        CLASS_E_CLASSNOTAVAILABLE
    }
}

#[implement(IActivationFactory)]
struct CompositorFactory;

impl IActivationFactory_Impl for CompositorFactory_Impl {
    fn ActivateInstance(&self) -> Result<IInspectable> {
        Ok(Compositor.into())
    }
}

#[implement(bindings::Compositor)]
struct Compositor;

impl bindings::ICompositor_Impl for Compositor_Impl {
    fn CreateSpriteVisual(&self, brush: i32) -> Result<bindings::SpriteVisual> {
        Ok(SpriteVisual::new(ContainerVisual::new(self.to_object(), brush * 2), brush).into())
    }
    fn CreateContainerVisual(&self, children: i32) -> Result<bindings::ContainerVisual> {
        Ok(ContainerVisual::new(self.to_object(), children).into())
    }
}

#[implement(bindings::ContainerVisual, bindings::Visual)]
struct ContainerVisual {
    compositor: ComObject<Compositor>,
    children: i32,
}

impl ContainerVisual {
    fn new(compositor: ComObject<Compositor>, children: i32) -> Self {
        Self {
            compositor,
            children,
        }
    }
}

impl bindings::IContainerVisual_Impl for ContainerVisual_Impl {
    fn Children(&self) -> i32 {
        self.children
    }
}

impl bindings::IVisual_Impl for ContainerVisual_Impl {
    fn Compositor(&self) -> Result<bindings::Compositor> {
        Ok(self.compositor.to_interface())
    }
}

#[implement(bindings::SpriteVisual, bindings::ContainerVisual, bindings::Visual)]
struct SpriteVisual {
    container: ContainerVisual,
    brush: i32,
}

impl SpriteVisual {
    fn new(container: ContainerVisual, brush: i32) -> Self {
        Self { container, brush }
    }
}

impl bindings::ISpriteVisual_Impl for SpriteVisual_Impl {
    fn Brush(&self) -> i32 {
        self.brush
    }
}

impl bindings::IContainerVisual_Impl for SpriteVisual_Impl {
    fn Children(&self) -> i32 {
        self.container.children
    }
}

impl bindings::IVisual_Impl for SpriteVisual_Impl {
    fn Compositor(&self) -> Result<bindings::Compositor> {
        Ok(self.container.compositor.to_interface())
    }
}
