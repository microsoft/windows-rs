windows_core::imp::define_interface!(IPortableDeviceKeyCollection, IPortableDeviceKeyCollection_Vtbl, 0xdada2357_e0ad_492e_98db_dd61c53ba353);
windows_core::imp::interface_hierarchy!(IPortableDeviceKeyCollection, windows_core::IUnknown);
impl IPortableDeviceKeyCollection {
    #[cfg(feature = "shared")]
    pub unsafe fn GetCount(&self, pcelems: *const super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), pcelems) }
    }
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub unsafe fn GetAt(&self, dwindex: super::DWORD, pkey: *const super::PROPERTYKEY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), dwindex, pkey) }
    }
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub unsafe fn Add(&self, key: *const super::PROPERTYKEY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), key) }
    }
    pub unsafe fn Clear(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn RemoveAt(&self, dwindex: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), dwindex) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceKeyCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetCount: usize,
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *const super::PROPERTYKEY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes")))]
    GetAt: usize,
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::PROPERTYKEY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes")))]
    Add: usize,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub RemoveAt: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RemoveAt: usize,
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub trait IPortableDeviceKeyCollection_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self, pcelems: *const super::DWORD) -> windows_core::Result<()>;
    fn GetAt(&self, dwindex: super::DWORD, pkey: *const super::PROPERTYKEY) -> windows_core::Result<()>;
    fn Add(&self, key: *const super::PROPERTYKEY) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn RemoveAt(&self, dwindex: super::DWORD) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
impl IPortableDeviceKeyCollection_Vtbl {
    pub const fn new<Identity: IPortableDeviceKeyCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IPortableDeviceKeyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelems: *const super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceKeyCollection_Impl::GetCount(this, core::mem::transmute_copy(&pcelems)).into()
            }
        }
        unsafe extern "system" fn GetAt<Identity: IPortableDeviceKeyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: super::DWORD, pkey: *const super::PROPERTYKEY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceKeyCollection_Impl::GetAt(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&pkey)).into()
            }
        }
        unsafe extern "system" fn Add<Identity: IPortableDeviceKeyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::PROPERTYKEY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceKeyCollection_Impl::Add(this, core::mem::transmute_copy(&key)).into()
            }
        }
        unsafe extern "system" fn Clear<Identity: IPortableDeviceKeyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceKeyCollection_Impl::Clear(this).into()
            }
        }
        unsafe extern "system" fn RemoveAt<Identity: IPortableDeviceKeyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceKeyCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&dwindex)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceKeyCollection as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
impl windows_core::RuntimeName for IPortableDeviceKeyCollection {}
windows_core::imp::define_interface!(IPortableDevicePropVariantCollection, IPortableDevicePropVariantCollection_Vtbl, 0x89b2e422_4f1b_4316_bcef_a44afea83eb3);
windows_core::imp::interface_hierarchy!(IPortableDevicePropVariantCollection, windows_core::IUnknown);
impl IPortableDevicePropVariantCollection {
    #[cfg(feature = "shared")]
    pub unsafe fn GetCount(&self, pcelems: *const super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), pcelems) }
    }
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetAt(&self, dwindex: super::DWORD, pvalue: *const super::PROPVARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), dwindex, pvalue) }
    }
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Add(&self, pvalue: *const super::PROPVARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), pvalue) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn GetType(&self) -> windows_core::Result<super::VARTYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn ChangeType(&self, vt: super::VARTYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ChangeType)(windows_core::Interface::as_raw(self), vt) }
    }
    pub unsafe fn Clear(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn RemoveAt(&self, dwindex: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), dwindex) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDevicePropVariantCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetCount: usize,
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *const super::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetAt: usize,
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    Add: usize,
    #[cfg(feature = "wtypes")]
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARTYPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    GetType: usize,
    #[cfg(feature = "wtypes")]
    pub ChangeType: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARTYPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    ChangeType: usize,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub RemoveAt: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RemoveAt: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IPortableDevicePropVariantCollection_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self, pcelems: *const super::DWORD) -> windows_core::Result<()>;
    fn GetAt(&self, dwindex: super::DWORD, pvalue: *const super::PROPVARIANT) -> windows_core::Result<()>;
    fn Add(&self, pvalue: *const super::PROPVARIANT) -> windows_core::Result<()>;
    fn GetType(&self) -> windows_core::Result<super::VARTYPE>;
    fn ChangeType(&self, vt: super::VARTYPE) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn RemoveAt(&self, dwindex: super::DWORD) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IPortableDevicePropVariantCollection_Vtbl {
    pub const fn new<Identity: IPortableDevicePropVariantCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IPortableDevicePropVariantCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelems: *const super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDevicePropVariantCollection_Impl::GetCount(this, core::mem::transmute_copy(&pcelems)).into()
            }
        }
        unsafe extern "system" fn GetAt<Identity: IPortableDevicePropVariantCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: super::DWORD, pvalue: *const super::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDevicePropVariantCollection_Impl::GetAt(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&pvalue)).into()
            }
        }
        unsafe extern "system" fn Add<Identity: IPortableDevicePropVariantCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *const super::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDevicePropVariantCollection_Impl::Add(this, core::mem::transmute_copy(&pvalue)).into()
            }
        }
        unsafe extern "system" fn GetType<Identity: IPortableDevicePropVariantCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvt: *mut super::VARTYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDevicePropVariantCollection_Impl::GetType(this) {
                    Ok(ok__) => {
                        pvt.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ChangeType<Identity: IPortableDevicePropVariantCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vt: super::VARTYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDevicePropVariantCollection_Impl::ChangeType(this, core::mem::transmute_copy(&vt)).into()
            }
        }
        unsafe extern "system" fn Clear<Identity: IPortableDevicePropVariantCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDevicePropVariantCollection_Impl::Clear(this).into()
            }
        }
        unsafe extern "system" fn RemoveAt<Identity: IPortableDevicePropVariantCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDevicePropVariantCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&dwindex)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            GetType: GetType::<Identity, OFFSET>,
            ChangeType: ChangeType::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDevicePropVariantCollection as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IPortableDevicePropVariantCollection {}
windows_core::imp::define_interface!(IPortableDeviceValues, IPortableDeviceValues_Vtbl, 0x6848f6f2_3155_4f86_b6f5_263eeeab3143);
windows_core::imp::interface_hierarchy!(IPortableDeviceValues, windows_core::IUnknown);
impl IPortableDeviceValues {
    #[cfg(feature = "shared")]
    pub unsafe fn GetCount(&self, pcelt: *const super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), pcelt) }
    }
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetAt(&self, index: super::DWORD, pkey: Option<*mut super::PROPERTYKEY>, pvalue: Option<*mut super::PROPVARIANT>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), index, pkey.unwrap_or(core::mem::zeroed()) as _, pvalue.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetValue(&self, key: *const super::PROPERTYKEY, pvalue: *const super::PROPVARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), key, pvalue) }
    }
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetValue(&self, key: *const super::PROPERTYKEY) -> windows_core::Result<super::PROPVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), key, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub unsafe fn SetStringValue<P1>(&self, key: *const super::PROPERTYKEY, value: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetStringValue)(windows_core::Interface::as_raw(self), key, value.param().abi()) }
    }
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub unsafe fn GetStringValue(&self, key: *const super::PROPERTYKEY) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStringValue)(windows_core::Interface::as_raw(self), key, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub unsafe fn SetUnsignedIntegerValue(&self, key: *const super::PROPERTYKEY, value: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetUnsignedIntegerValue)(windows_core::Interface::as_raw(self), key, value) }
    }
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub unsafe fn GetUnsignedIntegerValue(&self, key: *const super::PROPERTYKEY) -> windows_core::Result<super::ULONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUnsignedIntegerValue)(windows_core::Interface::as_raw(self), key, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub unsafe fn SetSignedIntegerValue(&self, key: *const super::PROPERTYKEY, value: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSignedIntegerValue)(windows_core::Interface::as_raw(self), key, value) }
    }
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub unsafe fn GetSignedIntegerValue(&self, key: *const super::PROPERTYKEY) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSignedIntegerValue)(windows_core::Interface::as_raw(self), key, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub unsafe fn SetUnsignedLargeIntegerValue(&self, key: *const super::PROPERTYKEY, value: super::ULONGLONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetUnsignedLargeIntegerValue)(windows_core::Interface::as_raw(self), key, value) }
    }
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub unsafe fn GetUnsignedLargeIntegerValue(&self, key: *const super::PROPERTYKEY) -> windows_core::Result<super::ULONGLONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUnsignedLargeIntegerValue)(windows_core::Interface::as_raw(self), key, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub unsafe fn SetSignedLargeIntegerValue(&self, key: *const super::PROPERTYKEY, value: super::LONGLONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSignedLargeIntegerValue)(windows_core::Interface::as_raw(self), key, value) }
    }
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub unsafe fn GetSignedLargeIntegerValue(&self, key: *const super::PROPERTYKEY) -> windows_core::Result<super::LONGLONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSignedLargeIntegerValue)(windows_core::Interface::as_raw(self), key, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "wtypes"))]
    pub unsafe fn SetFloatValue(&self, key: *const super::PROPERTYKEY, value: super::FLOAT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFloatValue)(windows_core::Interface::as_raw(self), key, value) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "wtypes"))]
    pub unsafe fn GetFloatValue(&self, key: *const super::PROPERTYKEY) -> windows_core::Result<super::FLOAT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFloatValue)(windows_core::Interface::as_raw(self), key, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub unsafe fn SetErrorValue(&self, key: *const super::PROPERTYKEY, value: windows_core::HRESULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetErrorValue)(windows_core::Interface::as_raw(self), key, value) }
    }
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub unsafe fn GetErrorValue(&self, key: *const super::PROPERTYKEY) -> windows_core::Result<windows_core::HRESULT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetErrorValue)(windows_core::Interface::as_raw(self), key, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub unsafe fn SetKeyValue(&self, key: *const super::PROPERTYKEY, value: *const super::PROPERTYKEY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetKeyValue)(windows_core::Interface::as_raw(self), key, value) }
    }
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub unsafe fn GetKeyValue(&self, key: *const super::PROPERTYKEY, pvalue: *mut super::PROPERTYKEY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetKeyValue)(windows_core::Interface::as_raw(self), key, pvalue as _) }
    }
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub unsafe fn SetBoolValue(&self, key: *const super::PROPERTYKEY, value: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBoolValue)(windows_core::Interface::as_raw(self), key, value.into()) }
    }
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub unsafe fn GetBoolValue(&self, key: *const super::PROPERTYKEY) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBoolValue)(windows_core::Interface::as_raw(self), key, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub unsafe fn SetIUnknownValue<P1>(&self, key: *const super::PROPERTYKEY, pvalue: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetIUnknownValue)(windows_core::Interface::as_raw(self), key, pvalue.param().abi()) }
    }
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub unsafe fn GetIUnknownValue(&self, key: *const super::PROPERTYKEY) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIUnknownValue)(windows_core::Interface::as_raw(self), key, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub unsafe fn SetGuidValue(&self, key: *const super::PROPERTYKEY, value: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetGuidValue)(windows_core::Interface::as_raw(self), key, value) }
    }
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub unsafe fn GetGuidValue(&self, key: *const super::PROPERTYKEY) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGuidValue)(windows_core::Interface::as_raw(self), key, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub unsafe fn SetBufferValue(&self, key: *const super::PROPERTYKEY, pvalue: &[super::BYTE]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBufferValue)(windows_core::Interface::as_raw(self), key, pvalue.as_ptr(), pvalue.len().try_into().unwrap()) }
    }
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub unsafe fn GetBufferValue(&self, key: *const super::PROPERTYKEY, ppvalue: *mut *mut super::BYTE, pcbvalue: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetBufferValue)(windows_core::Interface::as_raw(self), key, ppvalue as _, pcbvalue as _) }
    }
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub unsafe fn SetIPortableDeviceValuesValue<P1>(&self, key: *const super::PROPERTYKEY, pvalue: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetIPortableDeviceValuesValue)(windows_core::Interface::as_raw(self), key, pvalue.param().abi()) }
    }
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub unsafe fn GetIPortableDeviceValuesValue(&self, key: *const super::PROPERTYKEY) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIPortableDeviceValuesValue)(windows_core::Interface::as_raw(self), key, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub unsafe fn SetIPortableDevicePropVariantCollectionValue<P1>(&self, key: *const super::PROPERTYKEY, pvalue: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IPortableDevicePropVariantCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetIPortableDevicePropVariantCollectionValue)(windows_core::Interface::as_raw(self), key, pvalue.param().abi()) }
    }
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub unsafe fn GetIPortableDevicePropVariantCollectionValue(&self, key: *const super::PROPERTYKEY) -> windows_core::Result<IPortableDevicePropVariantCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIPortableDevicePropVariantCollectionValue)(windows_core::Interface::as_raw(self), key, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub unsafe fn SetIPortableDeviceKeyCollectionValue<P1>(&self, key: *const super::PROPERTYKEY, pvalue: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IPortableDeviceKeyCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetIPortableDeviceKeyCollectionValue)(windows_core::Interface::as_raw(self), key, pvalue.param().abi()) }
    }
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub unsafe fn GetIPortableDeviceKeyCollectionValue(&self, key: *const super::PROPERTYKEY) -> windows_core::Result<IPortableDeviceKeyCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIPortableDeviceKeyCollectionValue)(windows_core::Interface::as_raw(self), key, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub unsafe fn SetIPortableDeviceValuesCollectionValue<P1>(&self, key: *const super::PROPERTYKEY, pvalue: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IPortableDeviceValuesCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetIPortableDeviceValuesCollectionValue)(windows_core::Interface::as_raw(self), key, pvalue.param().abi()) }
    }
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub unsafe fn GetIPortableDeviceValuesCollectionValue(&self, key: *const super::PROPERTYKEY) -> windows_core::Result<IPortableDeviceValuesCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIPortableDeviceValuesCollectionValue)(windows_core::Interface::as_raw(self), key, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub unsafe fn RemoveValue(&self, key: *const super::PROPERTYKEY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveValue)(windows_core::Interface::as_raw(self), key) }
    }
    #[cfg(feature = "propsys")]
    pub unsafe fn CopyValuesFromPropertyStore<P0>(&self, pstore: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IPropertyStore>,
    {
        unsafe { (windows_core::Interface::vtable(self).CopyValuesFromPropertyStore)(windows_core::Interface::as_raw(self), pstore.param().abi()) }
    }
    #[cfg(feature = "propsys")]
    pub unsafe fn CopyValuesToPropertyStore<P0>(&self, pstore: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IPropertyStore>,
    {
        unsafe { (windows_core::Interface::vtable(self).CopyValuesToPropertyStore)(windows_core::Interface::as_raw(self), pstore.param().abi()) }
    }
    pub unsafe fn Clear(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceValues_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetCount: usize,
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut super::PROPERTYKEY, *mut super::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetAt: usize,
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::PROPERTYKEY, *const super::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    SetValue: usize,
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::PROPERTYKEY, *mut super::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetValue: usize,
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub SetStringValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::PROPERTYKEY, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes")))]
    SetStringValue: usize,
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub GetStringValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::PROPERTYKEY, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes")))]
    GetStringValue: usize,
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub SetUnsignedIntegerValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::PROPERTYKEY, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes")))]
    SetUnsignedIntegerValue: usize,
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub GetUnsignedIntegerValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::PROPERTYKEY, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes")))]
    GetUnsignedIntegerValue: usize,
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub SetSignedIntegerValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::PROPERTYKEY, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes")))]
    SetSignedIntegerValue: usize,
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub GetSignedIntegerValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::PROPERTYKEY, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes")))]
    GetSignedIntegerValue: usize,
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub SetUnsignedLargeIntegerValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::PROPERTYKEY, super::ULONGLONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes")))]
    SetUnsignedLargeIntegerValue: usize,
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub GetUnsignedLargeIntegerValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::PROPERTYKEY, *mut super::ULONGLONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes")))]
    GetUnsignedLargeIntegerValue: usize,
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub SetSignedLargeIntegerValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::PROPERTYKEY, super::LONGLONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes")))]
    SetSignedLargeIntegerValue: usize,
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub GetSignedLargeIntegerValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::PROPERTYKEY, *mut super::LONGLONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes")))]
    GetSignedLargeIntegerValue: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "wtypes"))]
    pub SetFloatValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::PROPERTYKEY, super::FLOAT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "wtypes")))]
    SetFloatValue: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "wtypes"))]
    pub GetFloatValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::PROPERTYKEY, *mut super::FLOAT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "wtypes")))]
    GetFloatValue: usize,
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub SetErrorValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::PROPERTYKEY, windows_core::HRESULT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes")))]
    SetErrorValue: usize,
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub GetErrorValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::PROPERTYKEY, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes")))]
    GetErrorValue: usize,
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub SetKeyValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::PROPERTYKEY, *const super::PROPERTYKEY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes")))]
    SetKeyValue: usize,
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub GetKeyValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::PROPERTYKEY, *mut super::PROPERTYKEY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes")))]
    GetKeyValue: usize,
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub SetBoolValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::PROPERTYKEY, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes")))]
    SetBoolValue: usize,
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub GetBoolValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::PROPERTYKEY, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes")))]
    GetBoolValue: usize,
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub SetIUnknownValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::PROPERTYKEY, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes")))]
    SetIUnknownValue: usize,
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub GetIUnknownValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::PROPERTYKEY, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes")))]
    GetIUnknownValue: usize,
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub SetGuidValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::PROPERTYKEY, *const windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes")))]
    SetGuidValue: usize,
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub GetGuidValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::PROPERTYKEY, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes")))]
    GetGuidValue: usize,
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub SetBufferValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::PROPERTYKEY, *const super::BYTE, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes")))]
    SetBufferValue: usize,
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub GetBufferValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::PROPERTYKEY, *mut *mut super::BYTE, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes")))]
    GetBufferValue: usize,
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub SetIPortableDeviceValuesValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::PROPERTYKEY, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes")))]
    SetIPortableDeviceValuesValue: usize,
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub GetIPortableDeviceValuesValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::PROPERTYKEY, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes")))]
    GetIPortableDeviceValuesValue: usize,
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub SetIPortableDevicePropVariantCollectionValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::PROPERTYKEY, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes")))]
    SetIPortableDevicePropVariantCollectionValue: usize,
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub GetIPortableDevicePropVariantCollectionValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::PROPERTYKEY, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes")))]
    GetIPortableDevicePropVariantCollectionValue: usize,
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub SetIPortableDeviceKeyCollectionValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::PROPERTYKEY, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes")))]
    SetIPortableDeviceKeyCollectionValue: usize,
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub GetIPortableDeviceKeyCollectionValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::PROPERTYKEY, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes")))]
    GetIPortableDeviceKeyCollectionValue: usize,
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub SetIPortableDeviceValuesCollectionValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::PROPERTYKEY, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes")))]
    SetIPortableDeviceValuesCollectionValue: usize,
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub GetIPortableDeviceValuesCollectionValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::PROPERTYKEY, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes")))]
    GetIPortableDeviceValuesCollectionValue: usize,
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub RemoveValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::PROPERTYKEY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes")))]
    RemoveValue: usize,
    #[cfg(feature = "propsys")]
    pub CopyValuesFromPropertyStore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "propsys"))]
    CopyValuesFromPropertyStore: usize,
    #[cfg(feature = "propsys")]
    pub CopyValuesToPropertyStore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "propsys"))]
    CopyValuesToPropertyStore: usize,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "propsys", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IPortableDeviceValues_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self, pcelt: *const super::DWORD) -> windows_core::Result<()>;
    fn GetAt(&self, index: super::DWORD, pkey: *mut super::PROPERTYKEY, pvalue: *mut super::PROPVARIANT) -> windows_core::Result<()>;
    fn SetValue(&self, key: *const super::PROPERTYKEY, pvalue: *const super::PROPVARIANT) -> windows_core::Result<()>;
    fn GetValue(&self, key: *const super::PROPERTYKEY) -> windows_core::Result<super::PROPVARIANT>;
    fn SetStringValue(&self, key: *const super::PROPERTYKEY, value: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetStringValue(&self, key: *const super::PROPERTYKEY) -> windows_core::Result<windows_core::PWSTR>;
    fn SetUnsignedIntegerValue(&self, key: *const super::PROPERTYKEY, value: super::ULONG) -> windows_core::Result<()>;
    fn GetUnsignedIntegerValue(&self, key: *const super::PROPERTYKEY) -> windows_core::Result<super::ULONG>;
    fn SetSignedIntegerValue(&self, key: *const super::PROPERTYKEY, value: super::LONG) -> windows_core::Result<()>;
    fn GetSignedIntegerValue(&self, key: *const super::PROPERTYKEY) -> windows_core::Result<super::LONG>;
    fn SetUnsignedLargeIntegerValue(&self, key: *const super::PROPERTYKEY, value: super::ULONGLONG) -> windows_core::Result<()>;
    fn GetUnsignedLargeIntegerValue(&self, key: *const super::PROPERTYKEY) -> windows_core::Result<super::ULONGLONG>;
    fn SetSignedLargeIntegerValue(&self, key: *const super::PROPERTYKEY, value: super::LONGLONG) -> windows_core::Result<()>;
    fn GetSignedLargeIntegerValue(&self, key: *const super::PROPERTYKEY) -> windows_core::Result<super::LONGLONG>;
    fn SetFloatValue(&self, key: *const super::PROPERTYKEY, value: super::FLOAT) -> windows_core::Result<()>;
    fn GetFloatValue(&self, key: *const super::PROPERTYKEY) -> windows_core::Result<super::FLOAT>;
    fn SetErrorValue(&self, key: *const super::PROPERTYKEY, value: windows_core::HRESULT) -> windows_core::Result<()>;
    fn GetErrorValue(&self, key: *const super::PROPERTYKEY) -> windows_core::Result<windows_core::HRESULT>;
    fn SetKeyValue(&self, key: *const super::PROPERTYKEY, value: *const super::PROPERTYKEY) -> windows_core::Result<()>;
    fn GetKeyValue(&self, key: *const super::PROPERTYKEY, pvalue: *mut super::PROPERTYKEY) -> windows_core::Result<()>;
    fn SetBoolValue(&self, key: *const super::PROPERTYKEY, value: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetBoolValue(&self, key: *const super::PROPERTYKEY) -> windows_core::Result<windows_core::BOOL>;
    fn SetIUnknownValue(&self, key: *const super::PROPERTYKEY, pvalue: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetIUnknownValue(&self, key: *const super::PROPERTYKEY) -> windows_core::Result<windows_core::IUnknown>;
    fn SetGuidValue(&self, key: *const super::PROPERTYKEY, value: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetGuidValue(&self, key: *const super::PROPERTYKEY) -> windows_core::Result<windows_core::GUID>;
    fn SetBufferValue(&self, key: *const super::PROPERTYKEY, pvalue: *const super::BYTE, cbvalue: super::DWORD) -> windows_core::Result<()>;
    fn GetBufferValue(&self, key: *const super::PROPERTYKEY, ppvalue: *mut *mut super::BYTE, pcbvalue: *mut super::DWORD) -> windows_core::Result<()>;
    fn SetIPortableDeviceValuesValue(&self, key: *const super::PROPERTYKEY, pvalue: windows_core::Ref<IPortableDeviceValues>) -> windows_core::Result<()>;
    fn GetIPortableDeviceValuesValue(&self, key: *const super::PROPERTYKEY) -> windows_core::Result<IPortableDeviceValues>;
    fn SetIPortableDevicePropVariantCollectionValue(&self, key: *const super::PROPERTYKEY, pvalue: windows_core::Ref<IPortableDevicePropVariantCollection>) -> windows_core::Result<()>;
    fn GetIPortableDevicePropVariantCollectionValue(&self, key: *const super::PROPERTYKEY) -> windows_core::Result<IPortableDevicePropVariantCollection>;
    fn SetIPortableDeviceKeyCollectionValue(&self, key: *const super::PROPERTYKEY, pvalue: windows_core::Ref<IPortableDeviceKeyCollection>) -> windows_core::Result<()>;
    fn GetIPortableDeviceKeyCollectionValue(&self, key: *const super::PROPERTYKEY) -> windows_core::Result<IPortableDeviceKeyCollection>;
    fn SetIPortableDeviceValuesCollectionValue(&self, key: *const super::PROPERTYKEY, pvalue: windows_core::Ref<IPortableDeviceValuesCollection>) -> windows_core::Result<()>;
    fn GetIPortableDeviceValuesCollectionValue(&self, key: *const super::PROPERTYKEY) -> windows_core::Result<IPortableDeviceValuesCollection>;
    fn RemoveValue(&self, key: *const super::PROPERTYKEY) -> windows_core::Result<()>;
    fn CopyValuesFromPropertyStore(&self, pstore: windows_core::Ref<super::IPropertyStore>) -> windows_core::Result<()>;
    fn CopyValuesToPropertyStore(&self, pstore: windows_core::Ref<super::IPropertyStore>) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "propsys", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IPortableDeviceValues_Vtbl {
    pub const fn new<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelt: *const super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValues_Impl::GetCount(this, core::mem::transmute_copy(&pcelt)).into()
            }
        }
        unsafe extern "system" fn GetAt<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::DWORD, pkey: *mut super::PROPERTYKEY, pvalue: *mut super::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValues_Impl::GetAt(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&pkey), core::mem::transmute_copy(&pvalue)).into()
            }
        }
        unsafe extern "system" fn SetValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::PROPERTYKEY, pvalue: *const super::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValues_Impl::SetValue(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&pvalue)).into()
            }
        }
        unsafe extern "system" fn GetValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::PROPERTYKEY, pvalue: *mut super::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceValues_Impl::GetValue(this, core::mem::transmute_copy(&key)) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStringValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::PROPERTYKEY, value: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValues_Impl::SetStringValue(this, core::mem::transmute_copy(&key), core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn GetStringValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::PROPERTYKEY, pvalue: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceValues_Impl::GetStringValue(this, core::mem::transmute_copy(&key)) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUnsignedIntegerValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::PROPERTYKEY, value: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValues_Impl::SetUnsignedIntegerValue(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetUnsignedIntegerValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::PROPERTYKEY, pvalue: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceValues_Impl::GetUnsignedIntegerValue(this, core::mem::transmute_copy(&key)) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSignedIntegerValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::PROPERTYKEY, value: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValues_Impl::SetSignedIntegerValue(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetSignedIntegerValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::PROPERTYKEY, pvalue: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceValues_Impl::GetSignedIntegerValue(this, core::mem::transmute_copy(&key)) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUnsignedLargeIntegerValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::PROPERTYKEY, value: super::ULONGLONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValues_Impl::SetUnsignedLargeIntegerValue(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetUnsignedLargeIntegerValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::PROPERTYKEY, pvalue: *mut super::ULONGLONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceValues_Impl::GetUnsignedLargeIntegerValue(this, core::mem::transmute_copy(&key)) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSignedLargeIntegerValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::PROPERTYKEY, value: super::LONGLONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValues_Impl::SetSignedLargeIntegerValue(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetSignedLargeIntegerValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::PROPERTYKEY, pvalue: *mut super::LONGLONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceValues_Impl::GetSignedLargeIntegerValue(this, core::mem::transmute_copy(&key)) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFloatValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::PROPERTYKEY, value: super::FLOAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValues_Impl::SetFloatValue(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetFloatValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::PROPERTYKEY, pvalue: *mut super::FLOAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceValues_Impl::GetFloatValue(this, core::mem::transmute_copy(&key)) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetErrorValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::PROPERTYKEY, value: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValues_Impl::SetErrorValue(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetErrorValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::PROPERTYKEY, pvalue: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceValues_Impl::GetErrorValue(this, core::mem::transmute_copy(&key)) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetKeyValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::PROPERTYKEY, value: *const super::PROPERTYKEY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValues_Impl::SetKeyValue(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetKeyValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::PROPERTYKEY, pvalue: *mut super::PROPERTYKEY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValues_Impl::GetKeyValue(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&pvalue)).into()
            }
        }
        unsafe extern "system" fn SetBoolValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::PROPERTYKEY, value: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValues_Impl::SetBoolValue(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetBoolValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::PROPERTYKEY, pvalue: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceValues_Impl::GetBoolValue(this, core::mem::transmute_copy(&key)) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIUnknownValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::PROPERTYKEY, pvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValues_Impl::SetIUnknownValue(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&pvalue)).into()
            }
        }
        unsafe extern "system" fn GetIUnknownValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::PROPERTYKEY, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceValues_Impl::GetIUnknownValue(this, core::mem::transmute_copy(&key)) {
                    Ok(ok__) => {
                        ppvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetGuidValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::PROPERTYKEY, value: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValues_Impl::SetGuidValue(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetGuidValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::PROPERTYKEY, pvalue: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceValues_Impl::GetGuidValue(this, core::mem::transmute_copy(&key)) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBufferValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::PROPERTYKEY, pvalue: *const super::BYTE, cbvalue: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValues_Impl::SetBufferValue(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&cbvalue)).into()
            }
        }
        unsafe extern "system" fn GetBufferValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::PROPERTYKEY, ppvalue: *mut *mut super::BYTE, pcbvalue: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValues_Impl::GetBufferValue(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&ppvalue), core::mem::transmute_copy(&pcbvalue)).into()
            }
        }
        unsafe extern "system" fn SetIPortableDeviceValuesValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::PROPERTYKEY, pvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValues_Impl::SetIPortableDeviceValuesValue(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&pvalue)).into()
            }
        }
        unsafe extern "system" fn GetIPortableDeviceValuesValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::PROPERTYKEY, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceValues_Impl::GetIPortableDeviceValuesValue(this, core::mem::transmute_copy(&key)) {
                    Ok(ok__) => {
                        ppvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIPortableDevicePropVariantCollectionValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::PROPERTYKEY, pvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValues_Impl::SetIPortableDevicePropVariantCollectionValue(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&pvalue)).into()
            }
        }
        unsafe extern "system" fn GetIPortableDevicePropVariantCollectionValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::PROPERTYKEY, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceValues_Impl::GetIPortableDevicePropVariantCollectionValue(this, core::mem::transmute_copy(&key)) {
                    Ok(ok__) => {
                        ppvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIPortableDeviceKeyCollectionValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::PROPERTYKEY, pvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValues_Impl::SetIPortableDeviceKeyCollectionValue(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&pvalue)).into()
            }
        }
        unsafe extern "system" fn GetIPortableDeviceKeyCollectionValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::PROPERTYKEY, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceValues_Impl::GetIPortableDeviceKeyCollectionValue(this, core::mem::transmute_copy(&key)) {
                    Ok(ok__) => {
                        ppvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIPortableDeviceValuesCollectionValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::PROPERTYKEY, pvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValues_Impl::SetIPortableDeviceValuesCollectionValue(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&pvalue)).into()
            }
        }
        unsafe extern "system" fn GetIPortableDeviceValuesCollectionValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::PROPERTYKEY, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceValues_Impl::GetIPortableDeviceValuesCollectionValue(this, core::mem::transmute_copy(&key)) {
                    Ok(ok__) => {
                        ppvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveValue<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::PROPERTYKEY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValues_Impl::RemoveValue(this, core::mem::transmute_copy(&key)).into()
            }
        }
        unsafe extern "system" fn CopyValuesFromPropertyStore<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstore: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValues_Impl::CopyValuesFromPropertyStore(this, core::mem::transmute_copy(&pstore)).into()
            }
        }
        unsafe extern "system" fn CopyValuesToPropertyStore<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstore: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValues_Impl::CopyValuesToPropertyStore(this, core::mem::transmute_copy(&pstore)).into()
            }
        }
        unsafe extern "system" fn Clear<Identity: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValues_Impl::Clear(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
            GetValue: GetValue::<Identity, OFFSET>,
            SetStringValue: SetStringValue::<Identity, OFFSET>,
            GetStringValue: GetStringValue::<Identity, OFFSET>,
            SetUnsignedIntegerValue: SetUnsignedIntegerValue::<Identity, OFFSET>,
            GetUnsignedIntegerValue: GetUnsignedIntegerValue::<Identity, OFFSET>,
            SetSignedIntegerValue: SetSignedIntegerValue::<Identity, OFFSET>,
            GetSignedIntegerValue: GetSignedIntegerValue::<Identity, OFFSET>,
            SetUnsignedLargeIntegerValue: SetUnsignedLargeIntegerValue::<Identity, OFFSET>,
            GetUnsignedLargeIntegerValue: GetUnsignedLargeIntegerValue::<Identity, OFFSET>,
            SetSignedLargeIntegerValue: SetSignedLargeIntegerValue::<Identity, OFFSET>,
            GetSignedLargeIntegerValue: GetSignedLargeIntegerValue::<Identity, OFFSET>,
            SetFloatValue: SetFloatValue::<Identity, OFFSET>,
            GetFloatValue: GetFloatValue::<Identity, OFFSET>,
            SetErrorValue: SetErrorValue::<Identity, OFFSET>,
            GetErrorValue: GetErrorValue::<Identity, OFFSET>,
            SetKeyValue: SetKeyValue::<Identity, OFFSET>,
            GetKeyValue: GetKeyValue::<Identity, OFFSET>,
            SetBoolValue: SetBoolValue::<Identity, OFFSET>,
            GetBoolValue: GetBoolValue::<Identity, OFFSET>,
            SetIUnknownValue: SetIUnknownValue::<Identity, OFFSET>,
            GetIUnknownValue: GetIUnknownValue::<Identity, OFFSET>,
            SetGuidValue: SetGuidValue::<Identity, OFFSET>,
            GetGuidValue: GetGuidValue::<Identity, OFFSET>,
            SetBufferValue: SetBufferValue::<Identity, OFFSET>,
            GetBufferValue: GetBufferValue::<Identity, OFFSET>,
            SetIPortableDeviceValuesValue: SetIPortableDeviceValuesValue::<Identity, OFFSET>,
            GetIPortableDeviceValuesValue: GetIPortableDeviceValuesValue::<Identity, OFFSET>,
            SetIPortableDevicePropVariantCollectionValue: SetIPortableDevicePropVariantCollectionValue::<Identity, OFFSET>,
            GetIPortableDevicePropVariantCollectionValue: GetIPortableDevicePropVariantCollectionValue::<Identity, OFFSET>,
            SetIPortableDeviceKeyCollectionValue: SetIPortableDeviceKeyCollectionValue::<Identity, OFFSET>,
            GetIPortableDeviceKeyCollectionValue: GetIPortableDeviceKeyCollectionValue::<Identity, OFFSET>,
            SetIPortableDeviceValuesCollectionValue: SetIPortableDeviceValuesCollectionValue::<Identity, OFFSET>,
            GetIPortableDeviceValuesCollectionValue: GetIPortableDeviceValuesCollectionValue::<Identity, OFFSET>,
            RemoveValue: RemoveValue::<Identity, OFFSET>,
            CopyValuesFromPropertyStore: CopyValuesFromPropertyStore::<Identity, OFFSET>,
            CopyValuesToPropertyStore: CopyValuesToPropertyStore::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceValues as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "propsys", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IPortableDeviceValues {}
windows_core::imp::define_interface!(IPortableDeviceValuesCollection, IPortableDeviceValuesCollection_Vtbl, 0x6e3f2d79_4e07_48c4_8208_d8c2e5af4a99);
windows_core::imp::interface_hierarchy!(IPortableDeviceValuesCollection, windows_core::IUnknown);
impl IPortableDeviceValuesCollection {
    #[cfg(feature = "shared")]
    pub unsafe fn GetCount(&self, pcelems: *const super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), pcelems) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetAt(&self, dwindex: super::DWORD) -> windows_core::Result<IPortableDeviceValues> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), dwindex, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Add<P0>(&self, pvalues: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IPortableDeviceValues>,
    {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), pvalues.param().abi()) }
    }
    pub unsafe fn Clear(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn RemoveAt(&self, dwindex: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), dwindex) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceValuesCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetCount: usize,
    #[cfg(feature = "shared")]
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetAt: usize,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub RemoveAt: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RemoveAt: usize,
}
#[cfg(feature = "shared")]
pub trait IPortableDeviceValuesCollection_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self, pcelems: *const super::DWORD) -> windows_core::Result<()>;
    fn GetAt(&self, dwindex: super::DWORD) -> windows_core::Result<IPortableDeviceValues>;
    fn Add(&self, pvalues: windows_core::Ref<IPortableDeviceValues>) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn RemoveAt(&self, dwindex: super::DWORD) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IPortableDeviceValuesCollection_Vtbl {
    pub const fn new<Identity: IPortableDeviceValuesCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IPortableDeviceValuesCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelems: *const super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValuesCollection_Impl::GetCount(this, core::mem::transmute_copy(&pcelems)).into()
            }
        }
        unsafe extern "system" fn GetAt<Identity: IPortableDeviceValuesCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: super::DWORD, ppvalues: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPortableDeviceValuesCollection_Impl::GetAt(this, core::mem::transmute_copy(&dwindex)) {
                    Ok(ok__) => {
                        ppvalues.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: IPortableDeviceValuesCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalues: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValuesCollection_Impl::Add(this, core::mem::transmute_copy(&pvalues)).into()
            }
        }
        unsafe extern "system" fn Clear<Identity: IPortableDeviceValuesCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValuesCollection_Impl::Clear(this).into()
            }
        }
        unsafe extern "system" fn RemoveAt<Identity: IPortableDeviceValuesCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPortableDeviceValuesCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&dwindex)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPortableDeviceValuesCollection as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IPortableDeviceValuesCollection {}
windows_core::imp::define_interface!(IWpdSerializer, IWpdSerializer_Vtbl, 0xb32f4002_bb27_45ff_af4f_06631c1e8dad);
windows_core::imp::interface_hierarchy!(IWpdSerializer, windows_core::IUnknown);
impl IWpdSerializer {
    #[cfg(feature = "shared")]
    pub unsafe fn GetIPortableDeviceValuesFromBuffer(&self, pbuffer: &[super::BYTE]) -> windows_core::Result<IPortableDeviceValues> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIPortableDeviceValuesFromBuffer)(windows_core::Interface::as_raw(self), pbuffer.as_ptr(), pbuffer.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn WriteIPortableDeviceValuesToBuffer<P1>(&self, dwoutputbufferlength: super::DWORD, presults: P1, pbuffer: *mut super::BYTE, pdwbyteswritten: *mut super::DWORD) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IPortableDeviceValues>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteIPortableDeviceValuesToBuffer)(windows_core::Interface::as_raw(self), dwoutputbufferlength, presults.param().abi(), pbuffer as _, pdwbyteswritten as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetBufferFromIPortableDeviceValues<P0>(&self, psource: P0, ppbuffer: *mut *mut super::BYTE, pdwbuffersize: *mut super::DWORD) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IPortableDeviceValues>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetBufferFromIPortableDeviceValues)(windows_core::Interface::as_raw(self), psource.param().abi(), ppbuffer as _, pdwbuffersize as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetSerializedSize<P0>(&self, psource: P0) -> windows_core::Result<super::DWORD>
    where
        P0: windows_core::Param<IPortableDeviceValues>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSerializedSize)(windows_core::Interface::as_raw(self), psource.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWpdSerializer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetIPortableDeviceValuesFromBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::BYTE, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetIPortableDeviceValuesFromBuffer: usize,
    #[cfg(feature = "shared")]
    pub WriteIPortableDeviceValuesToBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut core::ffi::c_void, *mut super::BYTE, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    WriteIPortableDeviceValuesToBuffer: usize,
    #[cfg(feature = "shared")]
    pub GetBufferFromIPortableDeviceValues: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut super::BYTE, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetBufferFromIPortableDeviceValues: usize,
    #[cfg(feature = "shared")]
    pub GetSerializedSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetSerializedSize: usize,
}
#[cfg(feature = "shared")]
pub trait IWpdSerializer_Impl: windows_core::IUnknownImpl {
    fn GetIPortableDeviceValuesFromBuffer(&self, pbuffer: *const super::BYTE, dwinputbufferlength: super::DWORD) -> windows_core::Result<IPortableDeviceValues>;
    fn WriteIPortableDeviceValuesToBuffer(&self, dwoutputbufferlength: super::DWORD, presults: windows_core::Ref<IPortableDeviceValues>, pbuffer: *mut super::BYTE, pdwbyteswritten: *mut super::DWORD) -> windows_core::Result<()>;
    fn GetBufferFromIPortableDeviceValues(&self, psource: windows_core::Ref<IPortableDeviceValues>, ppbuffer: *mut *mut super::BYTE, pdwbuffersize: *mut super::DWORD) -> windows_core::Result<()>;
    fn GetSerializedSize(&self, psource: windows_core::Ref<IPortableDeviceValues>) -> windows_core::Result<super::DWORD>;
}
#[cfg(feature = "shared")]
impl IWpdSerializer_Vtbl {
    pub const fn new<Identity: IWpdSerializer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetIPortableDeviceValuesFromBuffer<Identity: IWpdSerializer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffer: *const super::BYTE, dwinputbufferlength: super::DWORD, ppparams: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWpdSerializer_Impl::GetIPortableDeviceValuesFromBuffer(this, core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&dwinputbufferlength)) {
                    Ok(ok__) => {
                        ppparams.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn WriteIPortableDeviceValuesToBuffer<Identity: IWpdSerializer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputbufferlength: super::DWORD, presults: *mut core::ffi::c_void, pbuffer: *mut super::BYTE, pdwbyteswritten: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWpdSerializer_Impl::WriteIPortableDeviceValuesToBuffer(this, core::mem::transmute_copy(&dwoutputbufferlength), core::mem::transmute_copy(&presults), core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&pdwbyteswritten)).into()
            }
        }
        unsafe extern "system" fn GetBufferFromIPortableDeviceValues<Identity: IWpdSerializer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psource: *mut core::ffi::c_void, ppbuffer: *mut *mut super::BYTE, pdwbuffersize: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWpdSerializer_Impl::GetBufferFromIPortableDeviceValues(this, core::mem::transmute_copy(&psource), core::mem::transmute_copy(&ppbuffer), core::mem::transmute_copy(&pdwbuffersize)).into()
            }
        }
        unsafe extern "system" fn GetSerializedSize<Identity: IWpdSerializer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psource: *mut core::ffi::c_void, pdwsize: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWpdSerializer_Impl::GetSerializedSize(this, core::mem::transmute_copy(&psource)) {
                    Ok(ok__) => {
                        pdwsize.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetIPortableDeviceValuesFromBuffer: GetIPortableDeviceValuesFromBuffer::<Identity, OFFSET>,
            WriteIPortableDeviceValuesToBuffer: WriteIPortableDeviceValuesToBuffer::<Identity, OFFSET>,
            GetBufferFromIPortableDeviceValues: GetBufferFromIPortableDeviceValues::<Identity, OFFSET>,
            GetSerializedSize: GetSerializedSize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWpdSerializer as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWpdSerializer {}
pub const PortableDeviceKeyCollection: windows_core::GUID = windows_core::GUID::from_u128(0xde2d022d_2480_43be_97f0_d1fa2cf98f4f);
pub const PortableDevicePropVariantCollection: windows_core::GUID = windows_core::GUID::from_u128(0x08a99e2f_6d6d_4b80_af5a_baf2bcbe4cb9);
pub const PortableDeviceValues: windows_core::GUID = windows_core::GUID::from_u128(0x0c15d503_d017_47ce_9016_7b3f978721cc);
pub const PortableDeviceValuesCollection: windows_core::GUID = windows_core::GUID::from_u128(0x3882134d_14cf_4220_9cb4_435f86d83f60);
pub type WPD_STREAM_UNITS = i32;
pub const WPD_STREAM_UNITS_BYTES: WPD_STREAM_UNITS = 0;
pub const WPD_STREAM_UNITS_FRAMES: WPD_STREAM_UNITS = 1;
pub const WPD_STREAM_UNITS_MICROSECONDS: WPD_STREAM_UNITS = 8;
pub const WPD_STREAM_UNITS_MILLISECONDS: WPD_STREAM_UNITS = 4;
pub const WPD_STREAM_UNITS_ROWS: WPD_STREAM_UNITS = 2;
pub const WpdSerializer: windows_core::GUID = windows_core::GUID::from_u128(0x0b91a74b_ad7c_4a9d_b563_29eef9167172);
