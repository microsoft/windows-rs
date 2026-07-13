#[inline]
pub unsafe fn IUnknown_AddRef_Proxy<P0>(this: P0) -> u32
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("rpcrt4.dll" "system" fn IUnknown_AddRef_Proxy(this : *mut core::ffi::c_void) -> u32);
    unsafe { IUnknown_AddRef_Proxy(this.param().abi()) }
}
#[inline]
pub unsafe fn IUnknown_QueryInterface_Proxy<P0, T>(this: P0) -> windows_core::Result<T>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    T: windows_core::Interface,
{
    windows_core::link!("rpcrt4.dll" "system" fn IUnknown_QueryInterface_Proxy(this : *mut core::ffi::c_void, riid : *const windows_core::GUID, ppvobject : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { IUnknown_QueryInterface_Proxy(this.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[inline]
pub unsafe fn IUnknown_Release_Proxy<P0>(this: P0) -> u32
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("rpcrt4.dll" "system" fn IUnknown_Release_Proxy(this : *mut core::ffi::c_void) -> u32);
    unsafe { IUnknown_Release_Proxy(this.param().abi()) }
}
windows_core::imp::define_interface!(AsyncIUnknown, AsyncIUnknown_Vtbl, 0x000e0000_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(AsyncIUnknown, windows_core::IUnknown);
impl AsyncIUnknown {
    pub unsafe fn Begin_QueryInterface(&self, riid: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Begin_QueryInterface)(windows_core::Interface::as_raw(self), riid) }
    }
    pub unsafe fn Finish_QueryInterface(&self, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Finish_QueryInterface)(windows_core::Interface::as_raw(self), ppvobject as _) }
    }
    pub unsafe fn Begin_AddRef(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Begin_AddRef)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Finish_AddRef(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).Finish_AddRef)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Begin_Release(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Begin_Release)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Finish_Release(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).Finish_Release)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIUnknown_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Begin_QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub Finish_QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Begin_AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Finish_AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Begin_Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Finish_Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
}
pub trait AsyncIUnknown_Impl: windows_core::IUnknownImpl {
    fn Begin_QueryInterface(&self, riid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn Finish_QueryInterface(&self, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Begin_AddRef(&self) -> windows_core::Result<()>;
    fn Finish_AddRef(&self) -> u32;
    fn Begin_Release(&self) -> windows_core::Result<()>;
    fn Finish_Release(&self) -> u32;
}
impl AsyncIUnknown_Vtbl {
    pub const fn new<Identity: AsyncIUnknown_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Begin_QueryInterface<Identity: AsyncIUnknown_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIUnknown_Impl::Begin_QueryInterface(this, core::mem::transmute_copy(&riid)).into()
            }
        }
        unsafe extern "system" fn Finish_QueryInterface<Identity: AsyncIUnknown_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIUnknown_Impl::Finish_QueryInterface(this, core::mem::transmute_copy(&ppvobject)).into()
            }
        }
        unsafe extern "system" fn Begin_AddRef<Identity: AsyncIUnknown_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIUnknown_Impl::Begin_AddRef(this).into()
            }
        }
        unsafe extern "system" fn Finish_AddRef<Identity: AsyncIUnknown_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIUnknown_Impl::Finish_AddRef(this)
            }
        }
        unsafe extern "system" fn Begin_Release<Identity: AsyncIUnknown_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIUnknown_Impl::Begin_Release(this).into()
            }
        }
        unsafe extern "system" fn Finish_Release<Identity: AsyncIUnknown_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIUnknown_Impl::Finish_Release(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Begin_QueryInterface: Begin_QueryInterface::<Identity, OFFSET>,
            Finish_QueryInterface: Finish_QueryInterface::<Identity, OFFSET>,
            Begin_AddRef: Begin_AddRef::<Identity, OFFSET>,
            Finish_AddRef: Finish_AddRef::<Identity, OFFSET>,
            Begin_Release: Begin_Release::<Identity, OFFSET>,
            Finish_Release: Finish_Release::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<AsyncIUnknown as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for AsyncIUnknown {}
windows_core::imp::define_interface!(IClassFactory, IClassFactory_Vtbl, 0x00000001_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IClassFactory, windows_core::IUnknown);
impl IClassFactory {
    pub unsafe fn CreateInstance<P0, T>(&self, punkouter: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateInstance)(windows_core::Interface::as_raw(self), punkouter.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn LockServer(&self, flock: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LockServer)(windows_core::Interface::as_raw(self), flock.into()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IClassFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LockServer: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IClassFactory_Impl: windows_core::IUnknownImpl {
    fn CreateInstance(&self, punkouter: windows_core::Ref<windows_core::IUnknown>, riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn LockServer(&self, flock: windows_core::BOOL) -> windows_core::Result<()>;
}
impl IClassFactory_Vtbl {
    pub const fn new<Identity: IClassFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstance<Identity: IClassFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IClassFactory_Impl::CreateInstance(this, core::mem::transmute_copy(&punkouter), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobject)).into()
            }
        }
        unsafe extern "system" fn LockServer<Identity: IClassFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flock: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IClassFactory_Impl::LockServer(this, core::mem::transmute_copy(&flock)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateInstance: CreateInstance::<Identity, OFFSET>,
            LockServer: LockServer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IClassFactory as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IClassFactory {}
