windows_core::imp::define_interface!(IWeakReference, IWeakReference_Vtbl, 0x00000037_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IWeakReference, windows_core::IUnknown);
impl IWeakReference {
    pub unsafe fn Resolve<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).Resolve)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWeakReference_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Resolve: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWeakReference_Impl: windows_core::IUnknownImpl {
    fn Resolve(&self, riid: *const windows_core::GUID, objectreference: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IWeakReference_Vtbl {
    pub const fn new<Identity: IWeakReference_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Resolve<Identity: IWeakReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, objectreference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWeakReference_Impl::Resolve(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&objectreference)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Resolve: Resolve::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWeakReference as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWeakReference {}
windows_core::imp::define_interface!(IWeakReferenceSource, IWeakReferenceSource_Vtbl, 0x00000038_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IWeakReferenceSource, windows_core::IUnknown);
impl IWeakReferenceSource {
    pub unsafe fn GetWeakReference(&self) -> windows_core::Result<IWeakReference> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWeakReference)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWeakReferenceSource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetWeakReference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWeakReferenceSource_Impl: windows_core::IUnknownImpl {
    fn GetWeakReference(&self) -> windows_core::Result<IWeakReference>;
}
impl IWeakReferenceSource_Vtbl {
    pub const fn new<Identity: IWeakReferenceSource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetWeakReference<Identity: IWeakReferenceSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, weakreference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWeakReferenceSource_Impl::GetWeakReference(this) {
                    Ok(ok__) => {
                        weakreference.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetWeakReference: GetWeakReference::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWeakReferenceSource as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWeakReferenceSource {}
