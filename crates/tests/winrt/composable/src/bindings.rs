#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
windows_core::imp::define_interface!(
    ICompositor,
    ICompositor_Vtbl,
    0x08d43dfb_d41b_5e02_9546_510ee6d43f67
);
impl windows_core::RuntimeType for ICompositor {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICompositor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateSpriteVisual: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateContainerVisual: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IContainerVisual,
    IContainerVisual_Vtbl,
    0xc995b6bb_4ed4_504a_9b2f_58a99477ac31
);
impl windows_core::RuntimeType for IContainerVisual {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IContainerVisual_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Children:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetChildren:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
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
#[repr(C)]
pub struct IContainerVisualFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    ISpriteVisual,
    ISpriteVisual_Vtbl,
    0xfbf10e20_acf0_59f2_8dd0_31772072a763
);
impl windows_core::RuntimeType for ISpriteVisual {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpriteVisual_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Brush: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetBrush: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ISpriteVisualFactory,
    ISpriteVisualFactory_Vtbl,
    0x903d2850_1624_5406_ae22_ab0529c140f4
);
impl windows_core::RuntimeType for ISpriteVisualFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpriteVisualFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IVisual,
    IVisual_Vtbl,
    0x8ec3cbe1_c3f2_5834_a0b9_fb8b2c2fdcd1
);
impl windows_core::RuntimeType for IVisual {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVisual_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Compositor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetCompositor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
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
#[repr(C)]
pub struct IVisualFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
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
    pub fn CreateSpriteVisual(&self) -> windows_core::Result<SpriteVisual> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateSpriteVisual)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateContainerVisual(&self) -> windows_core::Result<ContainerVisual> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateContainerVisual)(
                windows_core::Interface::as_raw(this),
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
    type Vtable = ICompositor_Vtbl;
    const IID: windows_core::GUID = <ICompositor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Compositor {
    const NAME: &'static str = "test_composable.Compositor";
}
unsafe impl Send for Compositor {}
unsafe impl Sync for Compositor {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ContainerVisual(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ContainerVisual,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(ContainerVisual, Visual);
impl ContainerVisual {
    pub fn Children(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Children)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetChildren(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetChildren)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
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
    pub fn SetCompositor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Compositor>,
    {
        let this = &windows_core::Interface::cast::<IVisual>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetCompositor)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
}
impl windows_core::RuntimeType for ContainerVisual {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IContainerVisual>();
}
unsafe impl windows_core::Interface for ContainerVisual {
    type Vtable = IContainerVisual_Vtbl;
    const IID: windows_core::GUID = <IContainerVisual as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ContainerVisual {
    const NAME: &'static str = "test_composable.ContainerVisual";
}
unsafe impl Send for ContainerVisual {}
unsafe impl Sync for ContainerVisual {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SpriteVisual(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    SpriteVisual,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(SpriteVisual, ContainerVisual, Visual);
impl SpriteVisual {
    pub fn Children(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<IContainerVisual>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Children)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetChildren(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IContainerVisual>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetChildren)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Brush(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Brush)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetBrush(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetBrush)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
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
    pub fn SetCompositor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Compositor>,
    {
        let this = &windows_core::Interface::cast::<IVisual>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).SetCompositor)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
}
impl windows_core::RuntimeType for SpriteVisual {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ISpriteVisual>();
}
unsafe impl windows_core::Interface for SpriteVisual {
    type Vtable = ISpriteVisual_Vtbl;
    const IID: windows_core::GUID = <ISpriteVisual as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpriteVisual {
    const NAME: &'static str = "test_composable.SpriteVisual";
}
unsafe impl Send for SpriteVisual {}
unsafe impl Sync for SpriteVisual {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
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
    pub fn SetCompositor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Compositor>,
    {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetCompositor)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
}
impl windows_core::RuntimeType for Visual {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IVisual>();
}
unsafe impl windows_core::Interface for Visual {
    type Vtable = IVisual_Vtbl;
    const IID: windows_core::GUID = <IVisual as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Visual {
    const NAME: &'static str = "test_composable.Visual";
}
unsafe impl Send for Visual {}
unsafe impl Sync for Visual {}
pub trait ICompositor_Impl: Sized + windows_core::IUnknownImpl {
    fn CreateSpriteVisual(&self) -> windows_core::Result<SpriteVisual>;
    fn CreateContainerVisual(&self) -> windows_core::Result<ContainerVisual>;
}
impl windows_core::RuntimeName for ICompositor {
    const NAME: &'static str = "test_composable.ICompositor";
}
impl ICompositor_Vtbl {
    pub const fn new<Identity: ICompositor_Impl, const OFFSET: isize>() -> ICompositor_Vtbl {
        unsafe extern "system" fn CreateSpriteVisual<
            Identity: ICompositor_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICompositor_Impl::CreateSpriteVisual(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateContainerVisual<
            Identity: ICompositor_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICompositor_Impl::CreateContainerVisual(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
pub trait IContainerVisual_Impl: Sized + windows_core::IUnknownImpl {
    fn Children(&self) -> windows_core::Result<i32>;
    fn SetChildren(&self, value: i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IContainerVisual {
    const NAME: &'static str = "test_composable.IContainerVisual";
}
impl IContainerVisual_Vtbl {
    pub const fn new<Identity: IContainerVisual_Impl, const OFFSET: isize>() -> IContainerVisual_Vtbl
    {
        unsafe extern "system" fn Children<Identity: IContainerVisual_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut i32,
        ) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContainerVisual_Impl::Children(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChildren<
            Identity: IContainerVisual_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: i32,
        ) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContainerVisual_Impl::SetChildren(this, value).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IContainerVisual, OFFSET>(),
            Children: Children::<Identity, OFFSET>,
            SetChildren: SetChildren::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContainerVisual as windows_core::Interface>::IID
    }
}
pub trait IContainerVisualFactory_Impl: Sized + windows_core::IUnknownImpl {}
impl windows_core::RuntimeName for IContainerVisualFactory {
    const NAME: &'static str = "test_composable.IContainerVisualFactory";
}
impl IContainerVisualFactory_Vtbl {
    pub const fn new<Identity: IContainerVisualFactory_Impl, const OFFSET: isize>(
    ) -> IContainerVisualFactory_Vtbl {
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IContainerVisualFactory, OFFSET>(
            ),
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContainerVisualFactory as windows_core::Interface>::IID
    }
}
pub trait ISpriteVisual_Impl: Sized + windows_core::IUnknownImpl {
    fn Brush(&self) -> windows_core::Result<i32>;
    fn SetBrush(&self, value: i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISpriteVisual {
    const NAME: &'static str = "test_composable.ISpriteVisual";
}
impl ISpriteVisual_Vtbl {
    pub const fn new<Identity: ISpriteVisual_Impl, const OFFSET: isize>() -> ISpriteVisual_Vtbl {
        unsafe extern "system" fn Brush<Identity: ISpriteVisual_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut i32,
        ) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpriteVisual_Impl::Brush(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBrush<Identity: ISpriteVisual_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            value: i32,
        ) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpriteVisual_Impl::SetBrush(this, value).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISpriteVisual, OFFSET>(),
            Brush: Brush::<Identity, OFFSET>,
            SetBrush: SetBrush::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpriteVisual as windows_core::Interface>::IID
    }
}
pub trait ISpriteVisualFactory_Impl: Sized + windows_core::IUnknownImpl {}
impl windows_core::RuntimeName for ISpriteVisualFactory {
    const NAME: &'static str = "test_composable.ISpriteVisualFactory";
}
impl ISpriteVisualFactory_Vtbl {
    pub const fn new<Identity: ISpriteVisualFactory_Impl, const OFFSET: isize>(
    ) -> ISpriteVisualFactory_Vtbl {
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISpriteVisualFactory, OFFSET>(
            ),
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpriteVisualFactory as windows_core::Interface>::IID
    }
}
pub trait IVisual_Impl: Sized + windows_core::IUnknownImpl {
    fn Compositor(&self) -> windows_core::Result<Compositor>;
    fn SetCompositor(&self, value: Option<&Compositor>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVisual {
    const NAME: &'static str = "test_composable.IVisual";
}
impl IVisual_Vtbl {
    pub const fn new<Identity: IVisual_Impl, const OFFSET: isize>() -> IVisual_Vtbl {
        unsafe extern "system" fn Compositor<Identity: IVisual_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVisual_Impl::Compositor(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompositor<Identity: IVisual_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            value: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVisual_Impl::SetCompositor(this, windows_core::from_raw_borrowed(&value)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IVisual, OFFSET>(),
            Compositor: Compositor::<Identity, OFFSET>,
            SetCompositor: SetCompositor::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVisual as windows_core::Interface>::IID
    }
}
pub trait IVisualFactory_Impl: Sized + windows_core::IUnknownImpl {}
impl windows_core::RuntimeName for IVisualFactory {
    const NAME: &'static str = "test_composable.IVisualFactory";
}
impl IVisualFactory_Vtbl {
    pub const fn new<Identity: IVisualFactory_Impl, const OFFSET: isize>() -> IVisualFactory_Vtbl {
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IVisualFactory, OFFSET>(),
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVisualFactory as windows_core::Interface>::IID
    }
}
