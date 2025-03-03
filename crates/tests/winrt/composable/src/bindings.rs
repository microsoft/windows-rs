#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Compositor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    Compositor,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl Compositor {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            Compositor,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn CreateSpriteVisual(&self, brush: i32) -> windows_core::Result<SpriteVisual> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateSpriteVisual)(
                windows_core::Interface::as_raw(this),
                brush,
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateContainerVisual(&self, children: i32) -> windows_core::Result<ContainerVisual> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateContainerVisual)(
                windows_core::Interface::as_raw(this),
                children,
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for Compositor {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICompositor>();
}
unsafe impl windows_core::Interface for Compositor {
    type Vtable = <ICompositor as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICompositor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Compositor {
    const NAME: &'static str = "test_composable.Compositor";
}
unsafe impl Send for Compositor {}
unsafe impl Sync for Compositor {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContainerVisual(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ContainerVisual,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(ContainerVisual, Visual);
impl ContainerVisual {
    pub fn Children(&self) -> i32 {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            let hresult__ = (windows_core::Interface::vtable(this).Children)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            );
            debug_assert!(hresult__.0 == 0);
            result__
        }
    }
    pub fn Compositor(&self) -> windows_core::Result<Compositor> {
        let this = &windows_core::Interface::cast::<IVisual>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Compositor)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ContainerVisual {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IContainerVisual>();
}
unsafe impl windows_core::Interface for ContainerVisual {
    type Vtable = <IContainerVisual as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IContainerVisual as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ContainerVisual {
    const NAME: &'static str = "test_composable.ContainerVisual";
}
unsafe impl Send for ContainerVisual {}
unsafe impl Sync for ContainerVisual {}
windows_core::imp::define_interface!(
    ICompositor,
    ICompositor_Vtbl,
    0xac7b49b8_e092_52ad_8456_48696a5a258e
);
impl windows_core::RuntimeType for ICompositor {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for ICompositor {
    const NAME: &'static str = "test_composable.ICompositor";
}
pub trait ICompositor_Impl: windows_core::IUnknownImpl {
    fn CreateSpriteVisual(&self, brush: i32) -> windows_core::Result<SpriteVisual>;
    fn CreateContainerVisual(&self, children: i32) -> windows_core::Result<ContainerVisual>;
}
impl ICompositor_Vtbl {
    pub const fn new<Identity: ICompositor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateSpriteVisual<
            Identity: ICompositor_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            brush: i32,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICompositor_Impl::CreateSpriteVisual(this, brush) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateContainerVisual<
            Identity: ICompositor_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            children: i32,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICompositor_Impl::CreateContainerVisual(this, children) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICompositor, OFFSET>(),
            CreateSpriteVisual: CreateSpriteVisual::<Identity, OFFSET>,
            CreateContainerVisual: CreateContainerVisual::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICompositor as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateSpriteVisual: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        i32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateContainerVisual: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        i32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IContainerVisual,
    IContainerVisual_Vtbl,
    0xb8accc46_3ff7_5a24_8247_f5a52e1f5a8d
);
impl windows_core::RuntimeType for IContainerVisual {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IContainerVisual {
    const NAME: &'static str = "test_composable.IContainerVisual";
}
pub trait IContainerVisual_Impl: windows_core::IUnknownImpl {
    fn Children(&self) -> i32;
}
impl IContainerVisual_Vtbl {
    pub const fn new<Identity: IContainerVisual_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Children<Identity: IContainerVisual_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                let ok__ = IContainerVisual_Impl::Children(this);
                result__.write(core::mem::transmute_copy(&ok__));
                windows_core::HRESULT(0)
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IContainerVisual, OFFSET>(),
            Children: Children::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContainerVisual as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContainerVisual_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Children:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IContainerVisualFactory,
    IContainerVisualFactory_Vtbl,
    0x558b6180_1a65_5f01_8be2_2cc0b2034c0e
);
impl windows_core::RuntimeType for IContainerVisualFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IContainerVisualFactory {
    const NAME: &'static str = "test_composable.IContainerVisualFactory";
}
pub trait IContainerVisualFactory_Impl: windows_core::IUnknownImpl {}
impl IContainerVisualFactory_Vtbl {
    pub const fn new<Identity: IContainerVisualFactory_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IContainerVisualFactory, OFFSET>(
            ),
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContainerVisualFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContainerVisualFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    ISpriteVisual,
    ISpriteVisual_Vtbl,
    0x25f23ebe_4cd3_5349_b16d_d88c4d852ea1
);
impl windows_core::RuntimeType for ISpriteVisual {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for ISpriteVisual {
    const NAME: &'static str = "test_composable.ISpriteVisual";
}
pub trait ISpriteVisual_Impl: windows_core::IUnknownImpl {
    fn Brush(&self) -> i32;
}
impl ISpriteVisual_Vtbl {
    pub const fn new<Identity: ISpriteVisual_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Brush<Identity: ISpriteVisual_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                let ok__ = ISpriteVisual_Impl::Brush(this);
                result__.write(core::mem::transmute_copy(&ok__));
                windows_core::HRESULT(0)
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISpriteVisual, OFFSET>(),
            Brush: Brush::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpriteVisual as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpriteVisual_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Brush: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IVisual,
    IVisual_Vtbl,
    0xce89606a_5b03_5861_af26_9dced3aab7e6
);
impl windows_core::RuntimeType for IVisual {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IVisual {
    const NAME: &'static str = "test_composable.IVisual";
}
pub trait IVisual_Impl: windows_core::IUnknownImpl {
    fn Compositor(&self) -> windows_core::Result<Compositor>;
}
impl IVisual_Vtbl {
    pub const fn new<Identity: IVisual_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Compositor<Identity: IVisual_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVisual_Impl::Compositor(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IVisual, OFFSET>(),
            Compositor: Compositor::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVisual as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisual_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Compositor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IVisualFactory,
    IVisualFactory_Vtbl,
    0x1974545d_259f_553c_8ea0_e505f897df81
);
impl windows_core::RuntimeType for IVisualFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IVisualFactory {
    const NAME: &'static str = "test_composable.IVisualFactory";
}
pub trait IVisualFactory_Impl: windows_core::IUnknownImpl {}
impl IVisualFactory_Vtbl {
    pub const fn new<Identity: IVisualFactory_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IVisualFactory, OFFSET>(),
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVisualFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpriteVisual(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    SpriteVisual,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(SpriteVisual, ContainerVisual, Visual);
impl SpriteVisual {
    pub fn Children(&self) -> i32 {
        let this = &windows_core::Interface::cast::<IContainerVisual>(self).unwrap();
        unsafe {
            let mut result__ = core::mem::zeroed();
            let hresult__ = (windows_core::Interface::vtable(this).Children)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            );
            debug_assert!(hresult__.0 == 0);
            result__
        }
    }
    pub fn Brush(&self) -> i32 {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            let hresult__ = (windows_core::Interface::vtable(this).Brush)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            );
            debug_assert!(hresult__.0 == 0);
            result__
        }
    }
    pub fn Compositor(&self) -> windows_core::Result<Compositor> {
        let this = &windows_core::Interface::cast::<IVisual>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Compositor)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SpriteVisual {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ISpriteVisual>();
}
unsafe impl windows_core::Interface for SpriteVisual {
    type Vtable = <ISpriteVisual as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISpriteVisual as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpriteVisual {
    const NAME: &'static str = "test_composable.SpriteVisual";
}
unsafe impl Send for SpriteVisual {}
unsafe impl Sync for SpriteVisual {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Visual(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Visual, windows_core::IUnknown, windows_core::IInspectable);
impl Visual {
    pub fn Compositor(&self) -> windows_core::Result<Compositor> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Compositor)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for Visual {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IVisual>();
}
unsafe impl windows_core::Interface for Visual {
    type Vtable = <IVisual as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IVisual as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Visual {
    const NAME: &'static str = "test_composable.Visual";
}
unsafe impl Send for Visual {}
unsafe impl Sync for Visual {}
