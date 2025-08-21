#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

windows_core::imp::define_interface!(
    Callback,
    Callback_Vtbl,
    0xe39afc7e_93f1_5a1d_92ef_bd5f71c62cb8
);
impl windows_core::RuntimeType for Callback {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl Callback {
    pub fn new<F: Fn(i32) -> windows_core::Result<i32> + Send + 'static>(invoke: F) -> Self {
        let com = CallbackBox {
            vtable: &CallbackBox::<F>::VTABLE,
            count: windows_core::imp::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
    }
    pub fn Invoke(&self, a: i32) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Invoke)(
                windows_core::Interface::as_raw(this),
                a,
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct Callback_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        a: i32,
        result__: *mut i32,
    ) -> windows_core::HRESULT,
}
#[repr(C)]
struct CallbackBox<F: Fn(i32) -> windows_core::Result<i32> + Send + 'static> {
    vtable: *const Callback_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<F: Fn(i32) -> windows_core::Result<i32> + Send + 'static> CallbackBox<F> {
    const VTABLE: Callback_Vtbl = Callback_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface: Self::QueryInterface,
            AddRef: Self::AddRef,
            Release: Self::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(
        this: *mut core::ffi::c_void,
        iid: *const windows_core::GUID,
        interface: *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            if iid.is_null() || interface.is_null() {
                return windows_core::HRESULT(-2147467261);
            }
            *interface = if *iid == <Callback as windows_core::Interface>::IID
                || *iid == <windows_core::IUnknown as windows_core::Interface>::IID
                || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID
            {
                &mut (*this).vtable as *mut _ as _
            } else if *iid == <windows_core::imp::IMarshal as windows_core::Interface>::IID {
                (*this).count.add_ref();
                return windows_core::imp::marshaler(
                    core::mem::transmute(&mut (*this).vtable as *mut _ as *mut core::ffi::c_void),
                    interface,
                );
            } else {
                core::ptr::null_mut()
            };
            if (*interface).is_null() {
                windows_core::HRESULT(-2147467262)
            } else {
                (*this).count.add_ref();
                windows_core::HRESULT(0)
            }
        }
    }
    unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            (*this).count.add_ref()
        }
    }
    unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            let remaining = (*this).count.release();
            if remaining == 0 {
                let _ = windows_core::imp::Box::from_raw(this);
            }
            remaining
        }
    }
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        a: i32,
        result__: *mut i32,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
            match (this.invoke)(a) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Class(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Class, windows_core::IUnknown, windows_core::IInspectable);
impl Class {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Class, windows_core::imp::IGenericFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Property(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Property)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetProperty(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetProperty)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Flags(&self) -> windows_core::Result<Flags> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Flags)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Int32Array(
        &self,
        a: &[i32],
        b: &mut [i32],
        c: &mut windows_core::Array<i32>,
    ) -> windows_core::Result<windows_core::Array<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).Int32Array)(
                windows_core::Interface::as_raw(this),
                a.len().try_into().unwrap(),
                a.as_ptr(),
                b.len().try_into().unwrap(),
                b.as_mut_ptr(),
                c.set_abi_len(),
                c as *mut _ as _,
                windows_core::Array::<i32>::set_abi_len(core::mem::transmute(&mut result__)),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .map(|| result__.assume_init())
        }
    }
    pub fn StringArray(
        &self,
        a: &[windows_core::HSTRING],
        b: &mut [windows_core::HSTRING],
        c: &mut windows_core::Array<windows_core::HSTRING>,
    ) -> windows_core::Result<windows_core::Array<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).StringArray)(
                windows_core::Interface::as_raw(this),
                a.len().try_into().unwrap(),
                core::mem::transmute(a.as_ptr()),
                b.len().try_into().unwrap(),
                core::mem::transmute_copy(&b),
                c.set_abi_len(),
                c as *mut _ as _,
                windows_core::Array::<windows_core::HSTRING>::set_abi_len(core::mem::transmute(
                    &mut result__,
                )),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .map(|| result__.assume_init())
        }
    }
    pub fn Input<P0, P1, P2, P3>(&self, a: P0, b: P1, c: P2, d: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<Class>,
        P2: windows_core::Param<windows::Foundation::IStringable>,
        P3: windows_core::Param<Callback>,
    {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).Input)(
                windows_core::Interface::as_raw(this),
                a.param().abi(),
                b.param().abi(),
                c.param().abi(),
                d.param().abi(),
            )
            .ok()
        }
    }
}
impl windows_core::RuntimeType for Class {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IClass>();
}
unsafe impl windows_core::Interface for Class {
    type Vtable = <IClass as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IClass as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Class {
    const NAME: &'static str = "test_component.Class";
}
unsafe impl Send for Class {}
unsafe impl Sync for Class {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Flags(pub u32);
impl Flags {
    pub const Ok: Self = Self(0u32);
}
impl windows_core::TypeKind for Flags {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Flags {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(test_component.Flags;u4)");
}
impl Flags {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for Flags {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for Flags {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for Flags {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for Flags {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for Flags {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
windows_core::imp::define_interface!(IClass, IClass_Vtbl, 0x97540591_1323_59c0_9ae0_f510cae62e54);
impl windows_core::RuntimeType for IClass {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IClass_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Property:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetProperty:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Flags:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut Flags) -> windows_core::HRESULT,
    pub Int32Array: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *const i32,
        u32,
        *mut i32,
        *mut u32,
        *mut *mut i32,
        *mut u32,
        *mut *mut i32,
    ) -> windows_core::HRESULT,
    pub StringArray: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *const windows_core::HSTRING,
        u32,
        *mut windows_core::HSTRING,
        *mut u32,
        *mut *mut windows_core::HSTRING,
        *mut u32,
        *mut *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Input: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IThing, IThing_Vtbl, 0x5448be22_9873_5ae6_9106_f6e8455d2fdd);
impl windows_core::RuntimeType for IThing {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IThing, windows_core::IUnknown, windows_core::IInspectable);
impl IThing {
    pub fn Method(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).Method)(windows_core::Interface::as_raw(this))
                .ok()
        }
    }
}
impl windows_core::RuntimeName for IThing {
    const NAME: &'static str = "test_component.Nested.IThing";
}
pub trait IThing_Impl: windows_core::IUnknownImpl {
    fn Method(&self) -> windows_core::Result<()>;
}
impl IThing_Vtbl {
    pub const fn new<Identity: IThing_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Method<Identity: IThing_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IThing_Impl::Method(this).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IThing, OFFSET>(),
            Method: Method::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IThing as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IThing_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Method: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
