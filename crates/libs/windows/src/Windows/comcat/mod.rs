#[repr(C)]
#[cfg(all(feature = "winnt", feature = "wtypesbase"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CATEGORYINFO {
    pub catid: CATID,
    pub lcid: super::winnt::LCID,
    pub szDescription: [super::wtypesbase::OLECHAR; 128],
}
#[cfg(all(feature = "winnt", feature = "wtypesbase"))]
impl Default for CATEGORYINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CATID = windows_core::GUID;
windows_core::imp::define_interface!(ICatInformation, ICatInformation_Vtbl, 0x0002e013_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(ICatInformation, windows_core::IUnknown);
impl ICatInformation {
    #[cfg(feature = "winnt")]
    pub unsafe fn EnumCategories(&self, lcid: super::winnt::LCID) -> windows_core::Result<IEnumCATEGORYINFO> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumCategories)(windows_core::Interface::as_raw(self), lcid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn GetCategoryDesc(&self, rcatid: REFCATID, lcid: super::winnt::LCID) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCategoryDesc)(windows_core::Interface::as_raw(self), rcatid, lcid, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnumClassesOfCategories(&self, cimplemented: u32, rgcatidimpl: *const CATID, crequired: u32, rgcatidreq: *const CATID) -> windows_core::Result<IEnumGUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumClassesOfCategories)(windows_core::Interface::as_raw(self), cimplemented, rgcatidimpl, crequired, rgcatidreq, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn IsClassOfCategories(&self, rclsid: *const windows_core::GUID, cimplemented: u32, rgcatidimpl: *const CATID, crequired: u32, rgcatidreq: *const CATID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsClassOfCategories)(windows_core::Interface::as_raw(self), rclsid, cimplemented, rgcatidimpl, crequired, rgcatidreq) }
    }
    pub unsafe fn EnumImplCategoriesOfClass(&self, rclsid: *const windows_core::GUID) -> windows_core::Result<IEnumGUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumImplCategoriesOfClass)(windows_core::Interface::as_raw(self), rclsid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumReqCategoriesOfClass(&self, rclsid: *const windows_core::GUID) -> windows_core::Result<IEnumGUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumReqCategoriesOfClass)(windows_core::Interface::as_raw(self), rclsid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICatInformation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "winnt")]
    pub EnumCategories: unsafe extern "system" fn(*mut core::ffi::c_void, super::winnt::LCID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    EnumCategories: usize,
    #[cfg(feature = "winnt")]
    pub GetCategoryDesc: unsafe extern "system" fn(*mut core::ffi::c_void, REFCATID, super::winnt::LCID, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    GetCategoryDesc: usize,
    pub EnumClassesOfCategories: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const CATID, u32, *const CATID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsClassOfCategories: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const CATID, u32, *const CATID) -> windows_core::HRESULT,
    pub EnumImplCategoriesOfClass: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumReqCategoriesOfClass: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "winnt")]
pub trait ICatInformation_Impl: windows_core::IUnknownImpl {
    fn EnumCategories(&self, lcid: super::winnt::LCID) -> windows_core::Result<IEnumCATEGORYINFO>;
    fn GetCategoryDesc(&self, rcatid: REFCATID, lcid: super::winnt::LCID) -> windows_core::Result<windows_core::PWSTR>;
    fn EnumClassesOfCategories(&self, cimplemented: u32, rgcatidimpl: *const CATID, crequired: u32, rgcatidreq: *const CATID) -> windows_core::Result<IEnumGUID>;
    fn IsClassOfCategories(&self, rclsid: *const windows_core::GUID, cimplemented: u32, rgcatidimpl: *const CATID, crequired: u32, rgcatidreq: *const CATID) -> windows_core::Result<()>;
    fn EnumImplCategoriesOfClass(&self, rclsid: *const windows_core::GUID) -> windows_core::Result<IEnumGUID>;
    fn EnumReqCategoriesOfClass(&self, rclsid: *const windows_core::GUID) -> windows_core::Result<IEnumGUID>;
}
#[cfg(feature = "winnt")]
impl ICatInformation_Vtbl {
    pub const fn new<Identity: ICatInformation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumCategories<Identity: ICatInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcid: super::winnt::LCID, ppenumcategoryinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetCategoryDesc<Identity: ICatInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rcatid: REFCATID, lcid: super::winnt::LCID, pszdesc: *mut windows_core::PWSTR) -> windows_core::HRESULT {
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
        unsafe extern "system" fn EnumClassesOfCategories<Identity: ICatInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cimplemented: u32, rgcatidimpl: *const CATID, crequired: u32, rgcatidreq: *const CATID, ppenumclsid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn IsClassOfCategories<Identity: ICatInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, cimplemented: u32, rgcatidimpl: *const CATID, crequired: u32, rgcatidreq: *const CATID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICatInformation_Impl::IsClassOfCategories(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&cimplemented), core::mem::transmute_copy(&rgcatidimpl), core::mem::transmute_copy(&crequired), core::mem::transmute_copy(&rgcatidreq)).into()
            }
        }
        unsafe extern "system" fn EnumImplCategoriesOfClass<Identity: ICatInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, ppenumcatid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn EnumReqCategoriesOfClass<Identity: ICatInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, ppenumcatid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
#[cfg(feature = "winnt")]
impl windows_core::RuntimeName for ICatInformation {}
windows_core::imp::define_interface!(ICatRegister, ICatRegister_Vtbl, 0x0002e012_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(ICatRegister, windows_core::IUnknown);
impl ICatRegister {
    #[cfg(all(feature = "winnt", feature = "wtypesbase"))]
    pub unsafe fn RegisterCategories(&self, ccategories: u32, rgcategoryinfo: *const CATEGORYINFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RegisterCategories)(windows_core::Interface::as_raw(self), ccategories, rgcategoryinfo) }
    }
    pub unsafe fn UnRegisterCategories(&self, ccategories: u32, rgcatid: *const CATID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnRegisterCategories)(windows_core::Interface::as_raw(self), ccategories, rgcatid) }
    }
    pub unsafe fn RegisterClassImplCategories(&self, rclsid: *const windows_core::GUID, ccategories: u32, rgcatid: *const CATID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RegisterClassImplCategories)(windows_core::Interface::as_raw(self), rclsid, ccategories, rgcatid) }
    }
    pub unsafe fn UnRegisterClassImplCategories(&self, rclsid: *const windows_core::GUID, ccategories: u32, rgcatid: *const CATID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnRegisterClassImplCategories)(windows_core::Interface::as_raw(self), rclsid, ccategories, rgcatid) }
    }
    pub unsafe fn RegisterClassReqCategories(&self, rclsid: *const windows_core::GUID, ccategories: u32, rgcatid: *const CATID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RegisterClassReqCategories)(windows_core::Interface::as_raw(self), rclsid, ccategories, rgcatid) }
    }
    pub unsafe fn UnRegisterClassReqCategories(&self, rclsid: *const windows_core::GUID, ccategories: u32, rgcatid: *const CATID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnRegisterClassReqCategories)(windows_core::Interface::as_raw(self), rclsid, ccategories, rgcatid) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICatRegister_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "winnt", feature = "wtypesbase"))]
    pub RegisterCategories: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const CATEGORYINFO) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "winnt", feature = "wtypesbase")))]
    RegisterCategories: usize,
    pub UnRegisterCategories: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const CATID) -> windows_core::HRESULT,
    pub RegisterClassImplCategories: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const CATID) -> windows_core::HRESULT,
    pub UnRegisterClassImplCategories: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const CATID) -> windows_core::HRESULT,
    pub RegisterClassReqCategories: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const CATID) -> windows_core::HRESULT,
    pub UnRegisterClassReqCategories: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const CATID) -> windows_core::HRESULT,
}
#[cfg(all(feature = "winnt", feature = "wtypesbase"))]
pub trait ICatRegister_Impl: windows_core::IUnknownImpl {
    fn RegisterCategories(&self, ccategories: u32, rgcategoryinfo: *const CATEGORYINFO) -> windows_core::Result<()>;
    fn UnRegisterCategories(&self, ccategories: u32, rgcatid: *const CATID) -> windows_core::Result<()>;
    fn RegisterClassImplCategories(&self, rclsid: *const windows_core::GUID, ccategories: u32, rgcatid: *const CATID) -> windows_core::Result<()>;
    fn UnRegisterClassImplCategories(&self, rclsid: *const windows_core::GUID, ccategories: u32, rgcatid: *const CATID) -> windows_core::Result<()>;
    fn RegisterClassReqCategories(&self, rclsid: *const windows_core::GUID, ccategories: u32, rgcatid: *const CATID) -> windows_core::Result<()>;
    fn UnRegisterClassReqCategories(&self, rclsid: *const windows_core::GUID, ccategories: u32, rgcatid: *const CATID) -> windows_core::Result<()>;
}
#[cfg(all(feature = "winnt", feature = "wtypesbase"))]
impl ICatRegister_Vtbl {
    pub const fn new<Identity: ICatRegister_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterCategories<Identity: ICatRegister_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccategories: u32, rgcategoryinfo: *const CATEGORYINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICatRegister_Impl::RegisterCategories(this, core::mem::transmute_copy(&ccategories), core::mem::transmute_copy(&rgcategoryinfo)).into()
            }
        }
        unsafe extern "system" fn UnRegisterCategories<Identity: ICatRegister_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccategories: u32, rgcatid: *const CATID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICatRegister_Impl::UnRegisterCategories(this, core::mem::transmute_copy(&ccategories), core::mem::transmute_copy(&rgcatid)).into()
            }
        }
        unsafe extern "system" fn RegisterClassImplCategories<Identity: ICatRegister_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, ccategories: u32, rgcatid: *const CATID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICatRegister_Impl::RegisterClassImplCategories(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&ccategories), core::mem::transmute_copy(&rgcatid)).into()
            }
        }
        unsafe extern "system" fn UnRegisterClassImplCategories<Identity: ICatRegister_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, ccategories: u32, rgcatid: *const CATID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICatRegister_Impl::UnRegisterClassImplCategories(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&ccategories), core::mem::transmute_copy(&rgcatid)).into()
            }
        }
        unsafe extern "system" fn RegisterClassReqCategories<Identity: ICatRegister_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, ccategories: u32, rgcatid: *const CATID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICatRegister_Impl::RegisterClassReqCategories(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&ccategories), core::mem::transmute_copy(&rgcatid)).into()
            }
        }
        unsafe extern "system" fn UnRegisterClassReqCategories<Identity: ICatRegister_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, ccategories: u32, rgcatid: *const CATID) -> windows_core::HRESULT {
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
#[cfg(all(feature = "winnt", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ICatRegister {}
windows_core::imp::define_interface!(IEnumCATEGORYINFO, IEnumCATEGORYINFO_Vtbl, 0x0002e011_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IEnumCATEGORYINFO, windows_core::IUnknown);
impl IEnumCATEGORYINFO {
    #[cfg(all(feature = "winnt", feature = "wtypesbase"))]
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut CATEGORYINFO, pceltfetched: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, rgelt as _, pceltfetched as _) }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumCATEGORYINFO_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "winnt", feature = "wtypesbase"))]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut CATEGORYINFO, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "winnt", feature = "wtypesbase")))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "winnt", feature = "wtypesbase"))]
pub trait IEnumCATEGORYINFO_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgelt: *mut CATEGORYINFO, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumCATEGORYINFO>;
}
#[cfg(all(feature = "winnt", feature = "wtypesbase"))]
impl IEnumCATEGORYINFO_Vtbl {
    pub const fn new<Identity: IEnumCATEGORYINFO_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumCATEGORYINFO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut CATEGORYINFO, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumCATEGORYINFO_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumCATEGORYINFO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
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
#[cfg(all(feature = "winnt", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IEnumCATEGORYINFO {}
windows_core::imp::define_interface!(IEnumGUID, IEnumGUID_Vtbl, 0x0002e000_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IEnumGUID, windows_core::IUnknown);
impl IEnumGUID {
    pub unsafe fn Next(&self, rgelt: &mut [windows_core::GUID], pceltfetched: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), pceltfetched.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumGUID_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::GUID, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumGUID_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgelt: *mut windows_core::GUID, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumGUID>;
}
impl IEnumGUID_Vtbl {
    pub const fn new<Identity: IEnumGUID_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumGUID_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut windows_core::GUID, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumGUID_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumGUID_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
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
impl windows_core::RuntimeName for IEnumGUID {}
#[cfg(all(feature = "winnt", feature = "wtypesbase"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCATEGORYINFO(pub *mut CATEGORYINFO);
#[cfg(all(feature = "winnt", feature = "wtypesbase"))]
impl LPCATEGORYINFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "winnt", feature = "wtypesbase"))]
impl Default for LPCATEGORYINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct REFCATID(pub *const windows_core::GUID);
impl REFCATID {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for REFCATID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
