pub const CMM_READONLY: u32 = 2;
pub const CMM_REFRESHONLY: u32 = 1;
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ICertManageModule, ICertManageModule_Vtbl, 0xe7d7ad42_bd3d_11d1_9a4d_00c04fc297eb);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ICertManageModule {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ICertManageModule, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl ICertManageModule {
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetProperty(&self, strconfig: &windows_core::BSTR, strstoragelocation: &windows_core::BSTR, strpropertyname: &windows_core::BSTR, flags: i32) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strconfig), core::mem::transmute_copy(strstoragelocation), core::mem::transmute_copy(strpropertyname), flags, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetProperty(&self, strconfig: &windows_core::BSTR, strstoragelocation: &windows_core::BSTR, strpropertyname: &windows_core::BSTR, flags: i32, pvarproperty: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strconfig), core::mem::transmute_copy(strstoragelocation), core::mem::transmute_copy(strpropertyname), flags, pvarproperty) }
    }
    pub unsafe fn Configure(&self, strconfig: &windows_core::BSTR, strstoragelocation: &windows_core::BSTR, flags: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Configure)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strconfig), core::mem::transmute_copy(strstoragelocation), flags) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ICertManageModule_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    GetProperty: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *const super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    SetProperty: usize,
    pub Configure: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ICertManageModule_Impl: super::oaidl::IDispatch_Impl {
    fn GetProperty(&self, strconfig: &windows_core::BSTR, strstoragelocation: &windows_core::BSTR, strpropertyname: &windows_core::BSTR, flags: i32) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetProperty(&self, strconfig: &windows_core::BSTR, strstoragelocation: &windows_core::BSTR, strpropertyname: &windows_core::BSTR, flags: i32, pvarproperty: *const super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn Configure(&self, strconfig: &windows_core::BSTR, strstoragelocation: &windows_core::BSTR, flags: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ICertManageModule_Vtbl {
    pub const fn new<Identity: ICertManageModule_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetProperty<Identity: ICertManageModule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void, strstoragelocation: *mut core::ffi::c_void, strpropertyname: *mut core::ffi::c_void, flags: i32, pvarproperty: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertManageModule_Impl::GetProperty(this, core::mem::transmute(&strconfig), core::mem::transmute(&strstoragelocation), core::mem::transmute(&strpropertyname), core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        pvarproperty.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ICertManageModule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void, strstoragelocation: *mut core::ffi::c_void, strpropertyname: *mut core::ffi::c_void, flags: i32, pvarproperty: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertManageModule_Impl::SetProperty(this, core::mem::transmute(&strconfig), core::mem::transmute(&strstoragelocation), core::mem::transmute(&strpropertyname), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&pvarproperty)).into()
            }
        }
        unsafe extern "system" fn Configure<Identity: ICertManageModule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void, strstoragelocation: *mut core::ffi::c_void, flags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertManageModule_Impl::Configure(this, core::mem::transmute(&strconfig), core::mem::transmute(&strstoragelocation), core::mem::transmute_copy(&flags)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetProperty: GetProperty::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
            Configure: Configure::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertManageModule as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ICertManageModule {}
