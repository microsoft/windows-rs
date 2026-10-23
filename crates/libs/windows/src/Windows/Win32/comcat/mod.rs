#[repr(C)]
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CATEGORYINFO {
    pub catid: CATID,
    pub lcid: super::LCID,
    pub szDescription: [super::OLECHAR; 128],
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
impl Default for CATEGORYINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CATID = windows_core::GUID;
pub const CATID_NULL: windows_core::GUID = windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000000);
windows_core::imp::define_interface!(ICatInformation, ICatInformation_Vtbl, 0x0002e013_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(ICatInformation, windows_core::IUnknown);
impl ICatInformation {
    #[cfg(feature = "shared")]
    pub unsafe fn EnumCategories(&self, lcid: super::LCID) -> windows_core::Result<IEnumCATEGORYINFO> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumCategories)(windows_core::Interface::as_raw(self), lcid, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetCategoryDesc(&self, rcatid: *const windows_core::GUID, lcid: super::LCID) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCategoryDesc)(windows_core::Interface::as_raw(self), rcatid, lcid, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn EnumClassesOfCategories(&self, cimplemented: super::ULONG, rgcatidimpl: *const CATID, crequired: super::ULONG, rgcatidreq: *const CATID) -> windows_core::Result<IEnumGUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumClassesOfCategories)(windows_core::Interface::as_raw(self), cimplemented, rgcatidimpl, crequired, rgcatidreq, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn IsClassOfCategories(&self, rclsid: *const super::IID, cimplemented: super::ULONG, rgcatidimpl: *const CATID, crequired: super::ULONG, rgcatidreq: *const CATID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsClassOfCategories)(windows_core::Interface::as_raw(self), rclsid, cimplemented, rgcatidimpl, crequired, rgcatidreq) }
    }
    #[cfg(feature = "guiddef")]
    pub unsafe fn EnumImplCategoriesOfClass(&self, rclsid: *const super::IID) -> windows_core::Result<IEnumGUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumImplCategoriesOfClass)(windows_core::Interface::as_raw(self), rclsid, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "guiddef")]
    pub unsafe fn EnumReqCategoriesOfClass(&self, rclsid: *const super::IID) -> windows_core::Result<IEnumGUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumReqCategoriesOfClass)(windows_core::Interface::as_raw(self), rclsid, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICatInformation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub EnumCategories: unsafe extern "system" fn(*mut core::ffi::c_void, super::LCID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    EnumCategories: usize,
    #[cfg(feature = "shared")]
    pub GetCategoryDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, super::LCID, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetCategoryDesc: usize,
    #[cfg(feature = "shared")]
    pub EnumClassesOfCategories: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *const CATID, super::ULONG, *const CATID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    EnumClassesOfCategories: usize,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub IsClassOfCategories: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, super::ULONG, *const CATID, super::ULONG, *const CATID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    IsClassOfCategories: usize,
    #[cfg(feature = "guiddef")]
    pub EnumImplCategoriesOfClass: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "guiddef"))]
    EnumImplCategoriesOfClass: usize,
    #[cfg(feature = "guiddef")]
    pub EnumReqCategoriesOfClass: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "guiddef"))]
    EnumReqCategoriesOfClass: usize,
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
pub trait ICatInformation_Impl: windows_core::IUnknownImpl {
    fn EnumCategories(&self, lcid: super::LCID) -> windows_core::Result<IEnumCATEGORYINFO>;
    fn GetCategoryDesc(&self, rcatid: *const windows_core::GUID, lcid: super::LCID) -> windows_core::Result<windows_core::PWSTR>;
    fn EnumClassesOfCategories(&self, cimplemented: super::ULONG, rgcatidimpl: *const CATID, crequired: super::ULONG, rgcatidreq: *const CATID) -> windows_core::Result<IEnumGUID>;
    fn IsClassOfCategories(&self, rclsid: *const super::IID, cimplemented: super::ULONG, rgcatidimpl: *const CATID, crequired: super::ULONG, rgcatidreq: *const CATID) -> windows_core::Result<()>;
    fn EnumImplCategoriesOfClass(&self, rclsid: *const super::IID) -> windows_core::Result<IEnumGUID>;
    fn EnumReqCategoriesOfClass(&self, rclsid: *const super::IID) -> windows_core::Result<IEnumGUID>;
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl ICatInformation_Vtbl {
    pub const fn new<Identity: ICatInformation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumCategories<Identity: ICatInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcid: super::LCID, ppenumcategoryinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICatInformation_Impl::EnumCategories(this, core::mem::transmute_copy(&lcid)) {
                    Ok(ok__) => {
                        ppenumcategoryinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCategoryDesc<Identity: ICatInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rcatid: *const windows_core::GUID, lcid: super::LCID, pszdesc: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICatInformation_Impl::GetCategoryDesc(this, core::mem::transmute_copy(&rcatid), core::mem::transmute_copy(&lcid)) {
                    Ok(ok__) => {
                        pszdesc.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumClassesOfCategories<Identity: ICatInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cimplemented: super::ULONG, rgcatidimpl: *const CATID, crequired: super::ULONG, rgcatidreq: *const CATID, ppenumclsid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICatInformation_Impl::EnumClassesOfCategories(this, core::mem::transmute_copy(&cimplemented), core::mem::transmute_copy(&rgcatidimpl), core::mem::transmute_copy(&crequired), core::mem::transmute_copy(&rgcatidreq)) {
                    Ok(ok__) => {
                        ppenumclsid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsClassOfCategories<Identity: ICatInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const super::IID, cimplemented: super::ULONG, rgcatidimpl: *const CATID, crequired: super::ULONG, rgcatidreq: *const CATID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICatInformation_Impl::IsClassOfCategories(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&cimplemented), core::mem::transmute_copy(&rgcatidimpl), core::mem::transmute_copy(&crequired), core::mem::transmute_copy(&rgcatidreq)).into()
            }
        }
        unsafe extern "system" fn EnumImplCategoriesOfClass<Identity: ICatInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const super::IID, ppenumcatid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICatInformation_Impl::EnumImplCategoriesOfClass(this, core::mem::transmute_copy(&rclsid)) {
                    Ok(ok__) => {
                        ppenumcatid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumReqCategoriesOfClass<Identity: ICatInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const super::IID, ppenumcatid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICatInformation_Impl::EnumReqCategoriesOfClass(this, core::mem::transmute_copy(&rclsid)) {
                    Ok(ok__) => {
                        ppenumcatid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnumCategories: EnumCategories::<Identity, OFFSET>,
            GetCategoryDesc: GetCategoryDesc::<Identity, OFFSET>,
            EnumClassesOfCategories: EnumClassesOfCategories::<Identity, OFFSET>,
            IsClassOfCategories: IsClassOfCategories::<Identity, OFFSET>,
            EnumImplCategoriesOfClass: EnumImplCategoriesOfClass::<Identity, OFFSET>,
            EnumReqCategoriesOfClass: EnumReqCategoriesOfClass::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICatInformation as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl windows_core::RuntimeName for ICatInformation {}
windows_core::imp::define_interface!(ICatRegister, ICatRegister_Vtbl, 0x0002e012_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(ICatRegister, windows_core::IUnknown);
impl ICatRegister {
    #[cfg(all(feature = "shared", feature = "wtypesbase"))]
    pub unsafe fn RegisterCategories(&self, rgcategoryinfo: &[CATEGORYINFO]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RegisterCategories)(windows_core::Interface::as_raw(self), rgcategoryinfo.len().try_into().unwrap(), rgcategoryinfo.as_ptr()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn UnRegisterCategories(&self, rgcatid: &[CATID]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnRegisterCategories)(windows_core::Interface::as_raw(self), rgcatid.len().try_into().unwrap(), rgcatid.as_ptr()) }
    }
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn RegisterClassImplCategories(&self, rclsid: *const super::IID, rgcatid: &[CATID]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RegisterClassImplCategories)(windows_core::Interface::as_raw(self), rclsid, rgcatid.len().try_into().unwrap(), rgcatid.as_ptr()) }
    }
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn UnRegisterClassImplCategories(&self, rclsid: *const super::IID, rgcatid: &[CATID]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnRegisterClassImplCategories)(windows_core::Interface::as_raw(self), rclsid, rgcatid.len().try_into().unwrap(), rgcatid.as_ptr()) }
    }
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn RegisterClassReqCategories(&self, rclsid: *const super::IID, rgcatid: &[CATID]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RegisterClassReqCategories)(windows_core::Interface::as_raw(self), rclsid, rgcatid.len().try_into().unwrap(), rgcatid.as_ptr()) }
    }
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn UnRegisterClassReqCategories(&self, rclsid: *const super::IID, rgcatid: &[CATID]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnRegisterClassReqCategories)(windows_core::Interface::as_raw(self), rclsid, rgcatid.len().try_into().unwrap(), rgcatid.as_ptr()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICatRegister_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "shared", feature = "wtypesbase"))]
    pub RegisterCategories: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *const CATEGORYINFO) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypesbase")))]
    RegisterCategories: usize,
    #[cfg(feature = "shared")]
    pub UnRegisterCategories: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *const CATID) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    UnRegisterCategories: usize,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub RegisterClassImplCategories: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, super::ULONG, *const CATID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    RegisterClassImplCategories: usize,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub UnRegisterClassImplCategories: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, super::ULONG, *const CATID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    UnRegisterClassImplCategories: usize,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub RegisterClassReqCategories: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, super::ULONG, *const CATID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    RegisterClassReqCategories: usize,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub UnRegisterClassReqCategories: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, super::ULONG, *const CATID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    UnRegisterClassReqCategories: usize,
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "wtypesbase"))]
pub trait ICatRegister_Impl: windows_core::IUnknownImpl {
    fn RegisterCategories(&self, ccategories: super::ULONG, rgcategoryinfo: *const CATEGORYINFO) -> windows_core::Result<()>;
    fn UnRegisterCategories(&self, ccategories: super::ULONG, rgcatid: *const CATID) -> windows_core::Result<()>;
    fn RegisterClassImplCategories(&self, rclsid: *const super::IID, ccategories: super::ULONG, rgcatid: *const CATID) -> windows_core::Result<()>;
    fn UnRegisterClassImplCategories(&self, rclsid: *const super::IID, ccategories: super::ULONG, rgcatid: *const CATID) -> windows_core::Result<()>;
    fn RegisterClassReqCategories(&self, rclsid: *const super::IID, ccategories: super::ULONG, rgcatid: *const CATID) -> windows_core::Result<()>;
    fn UnRegisterClassReqCategories(&self, rclsid: *const super::IID, ccategories: super::ULONG, rgcatid: *const CATID) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "wtypesbase"))]
impl ICatRegister_Vtbl {
    pub const fn new<Identity: ICatRegister_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterCategories<Identity: ICatRegister_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccategories: super::ULONG, rgcategoryinfo: *const CATEGORYINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICatRegister_Impl::RegisterCategories(this, core::mem::transmute_copy(&ccategories), core::mem::transmute_copy(&rgcategoryinfo)).into()
            }
        }
        unsafe extern "system" fn UnRegisterCategories<Identity: ICatRegister_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccategories: super::ULONG, rgcatid: *const CATID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICatRegister_Impl::UnRegisterCategories(this, core::mem::transmute_copy(&ccategories), core::mem::transmute_copy(&rgcatid)).into()
            }
        }
        unsafe extern "system" fn RegisterClassImplCategories<Identity: ICatRegister_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const super::IID, ccategories: super::ULONG, rgcatid: *const CATID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICatRegister_Impl::RegisterClassImplCategories(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&ccategories), core::mem::transmute_copy(&rgcatid)).into()
            }
        }
        unsafe extern "system" fn UnRegisterClassImplCategories<Identity: ICatRegister_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const super::IID, ccategories: super::ULONG, rgcatid: *const CATID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICatRegister_Impl::UnRegisterClassImplCategories(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&ccategories), core::mem::transmute_copy(&rgcatid)).into()
            }
        }
        unsafe extern "system" fn RegisterClassReqCategories<Identity: ICatRegister_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const super::IID, ccategories: super::ULONG, rgcatid: *const CATID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICatRegister_Impl::RegisterClassReqCategories(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&ccategories), core::mem::transmute_copy(&rgcatid)).into()
            }
        }
        unsafe extern "system" fn UnRegisterClassReqCategories<Identity: ICatRegister_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const super::IID, ccategories: super::ULONG, rgcatid: *const CATID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICatRegister_Impl::UnRegisterClassReqCategories(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&ccategories), core::mem::transmute_copy(&rgcatid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterCategories: RegisterCategories::<Identity, OFFSET>,
            UnRegisterCategories: UnRegisterCategories::<Identity, OFFSET>,
            RegisterClassImplCategories: RegisterClassImplCategories::<Identity, OFFSET>,
            UnRegisterClassImplCategories: UnRegisterClassImplCategories::<Identity, OFFSET>,
            RegisterClassReqCategories: RegisterClassReqCategories::<Identity, OFFSET>,
            UnRegisterClassReqCategories: UnRegisterClassReqCategories::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICatRegister as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ICatRegister {}
windows_core::imp::define_interface!(IEnumCATEGORYINFO, IEnumCATEGORYINFO_Vtbl, 0x0002e011_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IEnumCATEGORYINFO, windows_core::IUnknown);
impl IEnumCATEGORYINFO {
    #[cfg(all(feature = "shared", feature = "wtypesbase"))]
    pub unsafe fn Next(&self, celt: super::ULONG, rgelt: *mut CATEGORYINFO, pceltfetched: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, rgelt as _, pceltfetched as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Skip(&self, celt: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumCATEGORYINFO_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "shared", feature = "wtypesbase"))]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *mut CATEGORYINFO, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypesbase")))]
    Next: usize,
    #[cfg(feature = "shared")]
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Skip: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
pub trait IEnumCATEGORYINFO_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: super::ULONG, rgelt: *mut CATEGORYINFO, pceltfetched: *mut super::ULONG) -> windows_core::Result<()>;
    fn Skip(&self, celt: super::ULONG) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumCATEGORYINFO>;
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
impl IEnumCATEGORYINFO_Vtbl {
    pub const fn new<Identity: IEnumCATEGORYINFO_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumCATEGORYINFO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: super::ULONG, rgelt: *mut CATEGORYINFO, pceltfetched: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumCATEGORYINFO_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumCATEGORYINFO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumCATEGORYINFO_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumCATEGORYINFO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumCATEGORYINFO_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumCATEGORYINFO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumCATEGORYINFO_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumCATEGORYINFO as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IEnumCATEGORYINFO {}
windows_core::imp::define_interface!(IEnumGUID, IEnumGUID_Vtbl, 0x0002e000_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IEnumGUID, windows_core::IUnknown);
impl IEnumGUID {
    #[cfg(feature = "shared")]
    pub unsafe fn Next(&self, celt: super::ULONG, rgelt: *mut windows_core::GUID, pceltfetched: Option<*mut super::ULONG>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, rgelt as _, pceltfetched.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Skip(&self, celt: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumGUID_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *mut windows_core::GUID, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Next: usize,
    #[cfg(feature = "shared")]
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Skip: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait IEnumGUID_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: super::ULONG, rgelt: *mut windows_core::GUID, pceltfetched: *mut super::ULONG) -> windows_core::Result<()>;
    fn Skip(&self, celt: super::ULONG) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumGUID>;
}
#[cfg(feature = "shared")]
impl IEnumGUID_Vtbl {
    pub const fn new<Identity: IEnumGUID_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumGUID_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: super::ULONG, rgelt: *mut windows_core::GUID, pceltfetched: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumGUID_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumGUID_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumGUID_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumGUID_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumGUID_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumGUID_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumGUID_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumGUID as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IEnumGUID {}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
pub type LPCATEGORYINFO = *mut CATEGORYINFO;
pub type LPCATINFORMATION = ICatInformation;
pub type LPCATREGISTER = ICatRegister;
pub type LPENUMCATEGORYINFO = IEnumCATEGORYINFO;
pub type LPENUMGUID = IEnumGUID;
