#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct A(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(A, windows_core::IUnknown, windows_core::IInspectable);
impl A {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<A, windows_core::imp::IGenericFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Method(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Method)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Method2(&self, a: i32) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Method2)(
                windows_core::Interface::as_raw(this),
                a,
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for A {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IA>();
}
unsafe impl windows_core::Interface for A {
    type Vtable = <IA as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IA as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for A {
    const NAME: &'static str = "test_overloads.A";
}
unsafe impl Send for A {}
unsafe impl Sync for A {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct B(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(B, windows_core::IUnknown, windows_core::IInspectable);
impl B {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<B, windows_core::imp::IGenericFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn MethodOne(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MethodOne)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn MethodTwo(&self, a: i32) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MethodTwo)(
                windows_core::Interface::as_raw(this),
                a,
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for B {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IB>();
}
unsafe impl windows_core::Interface for B {
    type Vtable = <IB as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IB as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for B {
    const NAME: &'static str = "test_overloads.B";
}
unsafe impl Send for B {}
unsafe impl Sync for B {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct C(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(C, windows_core::IUnknown, windows_core::IInspectable);
impl C {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<C, windows_core::imp::IGenericFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Method(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Method)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Method2(&self, a: i32) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Method2)(
                windows_core::Interface::as_raw(this),
                a,
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for C {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IC>();
}
unsafe impl windows_core::Interface for C {
    type Vtable = <IC as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IC as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for C {
    const NAME: &'static str = "test_overloads.C";
}
unsafe impl Send for C {}
unsafe impl Sync for C {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct D(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(D, windows_core::IUnknown, windows_core::IInspectable);
impl D {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<D, windows_core::imp::IGenericFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Method(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Method)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Method2(&self, a: i32) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Method2)(
                windows_core::Interface::as_raw(this),
                a,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Method3(&self, a: i32, b: i32) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<ID2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Method)(
                windows_core::Interface::as_raw(this),
                a,
                b,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Method4(&self, a: i32, b: i32, c: i32) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<ID2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Method2)(
                windows_core::Interface::as_raw(this),
                a,
                b,
                c,
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for D {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ID>();
}
unsafe impl windows_core::Interface for D {
    type Vtable = <ID as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ID as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for D {
    const NAME: &'static str = "test_overloads.D";
}
unsafe impl Send for D {}
unsafe impl Sync for D {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct E(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(E, windows_core::IUnknown, windows_core::IInspectable);
impl E {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<E, windows_core::imp::IGenericFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn MethodOne(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MethodOne)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn MethodTwo(&self, a: i32) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MethodTwo)(
                windows_core::Interface::as_raw(this),
                a,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn MethodThree(&self, a: i32, b: i32) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<IE2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MethodThree)(
                windows_core::Interface::as_raw(this),
                a,
                b,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn MethodFour(&self, a: i32, b: i32, c: i32) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<IE2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MethodFour)(
                windows_core::Interface::as_raw(this),
                a,
                b,
                c,
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for E {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IE>();
}
unsafe impl windows_core::Interface for E {
    type Vtable = <IE as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IE as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for E {
    const NAME: &'static str = "test_overloads.E";
}
unsafe impl Send for E {}
unsafe impl Sync for E {}
windows_core::imp::define_interface!(IA, IA_Vtbl, 0xea3ed6f8_2f81_5cfc_a281_4bf0d7535521);
impl windows_core::RuntimeType for IA {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IA_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Method:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Method2:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IB, IB_Vtbl, 0xc6f02ea8_68b6_5a1c_86fe_f8c0c0d655c4);
impl windows_core::RuntimeType for IB {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IB_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MethodOne:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub MethodTwo:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IC, IC_Vtbl, 0xdf8ad52f_5629_5e9b_a662_5723833b59e5);
impl windows_core::RuntimeType for IC {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IC_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Method:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Method2:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID, ID_Vtbl, 0xa9cf9a9f_9389_5f27_bb69_a094144cad72);
impl windows_core::RuntimeType for ID {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ID_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Method:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Method2:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID2, ID2_Vtbl, 0x5cbf6f2f_250f_57a9_82d9_d773fd84fbe9);
impl windows_core::RuntimeType for ID2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Method: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        i32,
        i32,
        *mut i32,
    ) -> windows_core::HRESULT,
    pub Method2: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        i32,
        i32,
        i32,
        *mut i32,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IE, IE_Vtbl, 0x179af921_706b_5a49_8624_7889b2eff9c1);
impl windows_core::RuntimeType for IE {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IE_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MethodOne:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub MethodTwo:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IE2, IE2_Vtbl, 0x9e8f2cad_09de_5f31_b940_8189d6323a19);
impl windows_core::RuntimeType for IE2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IE2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MethodThree: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        i32,
        i32,
        *mut i32,
    ) -> windows_core::HRESULT,
    pub MethodFour: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        i32,
        i32,
        i32,
        *mut i32,
    ) -> windows_core::HRESULT,
}
