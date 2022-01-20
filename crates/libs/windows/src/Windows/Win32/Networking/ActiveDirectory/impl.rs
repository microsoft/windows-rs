#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADs_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Class(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GUID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ADsPath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Parent(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Schema(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetInfo(&mut self) -> ::windows::core::Result<()>;
    fn SetInfo(&mut self) -> ::windows::core::Result<()>;
    fn Get(&mut self, bstrname: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Put(&mut self, bstrname: &super::super::Foundation::BSTR, vprop: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GetEx(&mut self, bstrname: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn PutEx(&mut self, lncontrolcode: i32, bstrname: &super::super::Foundation::BSTR, vprop: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GetInfoEx(&mut self, vproperties: &super::super::System::Com::VARIANT, lnreserved: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADs_Impl, const OFFSET: isize>() -> IADs_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: IADs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Class<Identity: ::windows::core::IUnknownImpl, Impl: IADs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Class() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GUID<Identity: ::windows::core::IUnknownImpl, Impl: IADs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GUID() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ADsPath<Identity: ::windows::core::IUnknownImpl, Impl: IADs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ADsPath() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Identity: ::windows::core::IUnknownImpl, Impl: IADs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Parent() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Schema<Identity: ::windows::core::IUnknownImpl, Impl: IADs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Schema() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInfo<Identity: ::windows::core::IUnknownImpl, Impl: IADs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetInfo().into()
        }
        unsafe extern "system" fn SetInfo<Identity: ::windows::core::IUnknownImpl, Impl: IADs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInfo().into()
        }
        unsafe extern "system" fn Get<Identity: ::windows::core::IUnknownImpl, Impl: IADs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Get(::core::mem::transmute_copy(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Put<Identity: ::windows::core::IUnknownImpl, Impl: IADs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, vprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Put(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&vprop)).into()
        }
        unsafe extern "system" fn GetEx<Identity: ::windows::core::IUnknownImpl, Impl: IADs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEx(::core::mem::transmute_copy(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutEx<Identity: ::windows::core::IUnknownImpl, Impl: IADs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lncontrolcode: i32, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, vprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PutEx(::core::mem::transmute_copy(&lncontrolcode), ::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&vprop)).into()
        }
        unsafe extern "system" fn GetInfoEx<Identity: ::windows::core::IUnknownImpl, Impl: IADs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vproperties: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, lnreserved: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetInfoEx(::core::mem::transmute_copy(&vproperties), ::core::mem::transmute_copy(&lnreserved)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            Class: Class::<Identity, Impl, OFFSET>,
            GUID: GUID::<Identity, Impl, OFFSET>,
            ADsPath: ADsPath::<Identity, Impl, OFFSET>,
            Parent: Parent::<Identity, Impl, OFFSET>,
            Schema: Schema::<Identity, Impl, OFFSET>,
            GetInfo: GetInfo::<Identity, Impl, OFFSET>,
            SetInfo: SetInfo::<Identity, Impl, OFFSET>,
            Get: Get::<Identity, Impl, OFFSET>,
            Put: Put::<Identity, Impl, OFFSET>,
            GetEx: GetEx::<Identity, Impl, OFFSET>,
            PutEx: PutEx::<Identity, Impl, OFFSET>,
            GetInfoEx: GetInfoEx::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADs as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsADSystemInfo_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn UserName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ComputerName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SiteName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DomainShortName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DomainDNSName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ForestDNSName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn PDCRoleOwner(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SchemaRoleOwner(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn IsNativeMode(&mut self) -> ::windows::core::Result<i16>;
    fn GetAnyDCName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetDCSiteName(&mut self, szserver: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn RefreshSchemaCache(&mut self) -> ::windows::core::Result<()>;
    fn GetTrees(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsADSystemInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsADSystemInfo_Impl, const OFFSET: isize>() -> IADsADSystemInfo_Vtbl {
        unsafe extern "system" fn UserName<Identity: ::windows::core::IUnknownImpl, Impl: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UserName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComputerName<Identity: ::windows::core::IUnknownImpl, Impl: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ComputerName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SiteName<Identity: ::windows::core::IUnknownImpl, Impl: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SiteName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DomainShortName<Identity: ::windows::core::IUnknownImpl, Impl: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DomainShortName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DomainDNSName<Identity: ::windows::core::IUnknownImpl, Impl: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DomainDNSName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ForestDNSName<Identity: ::windows::core::IUnknownImpl, Impl: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ForestDNSName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PDCRoleOwner<Identity: ::windows::core::IUnknownImpl, Impl: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PDCRoleOwner() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SchemaRoleOwner<Identity: ::windows::core::IUnknownImpl, Impl: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SchemaRoleOwner() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsNativeMode<Identity: ::windows::core::IUnknownImpl, Impl: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsNativeMode() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAnyDCName<Identity: ::windows::core::IUnknownImpl, Impl: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdcname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAnyDCName() {
                ::core::result::Result::Ok(ok__) => {
                    *pszdcname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDCSiteName<Identity: ::windows::core::IUnknownImpl, Impl: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szserver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pszsitename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDCSiteName(::core::mem::transmute_copy(&szserver)) {
                ::core::result::Result::Ok(ok__) => {
                    *pszsitename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RefreshSchemaCache<Identity: ::windows::core::IUnknownImpl, Impl: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RefreshSchemaCache().into()
        }
        unsafe extern "system" fn GetTrees<Identity: ::windows::core::IUnknownImpl, Impl: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvtrees: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetTrees() {
                ::core::result::Result::Ok(ok__) => {
                    *pvtrees = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            UserName: UserName::<Identity, Impl, OFFSET>,
            ComputerName: ComputerName::<Identity, Impl, OFFSET>,
            SiteName: SiteName::<Identity, Impl, OFFSET>,
            DomainShortName: DomainShortName::<Identity, Impl, OFFSET>,
            DomainDNSName: DomainDNSName::<Identity, Impl, OFFSET>,
            ForestDNSName: ForestDNSName::<Identity, Impl, OFFSET>,
            PDCRoleOwner: PDCRoleOwner::<Identity, Impl, OFFSET>,
            SchemaRoleOwner: SchemaRoleOwner::<Identity, Impl, OFFSET>,
            IsNativeMode: IsNativeMode::<Identity, Impl, OFFSET>,
            GetAnyDCName: GetAnyDCName::<Identity, Impl, OFFSET>,
            GetDCSiteName: GetDCSiteName::<Identity, Impl, OFFSET>,
            RefreshSchemaCache: RefreshSchemaCache::<Identity, Impl, OFFSET>,
            GetTrees: GetTrees::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsADSystemInfo as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsAccessControlEntry_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn AccessMask(&mut self) -> ::windows::core::Result<i32>;
    fn SetAccessMask(&mut self, lnaccessmask: i32) -> ::windows::core::Result<()>;
    fn AceType(&mut self) -> ::windows::core::Result<i32>;
    fn SetAceType(&mut self, lnacetype: i32) -> ::windows::core::Result<()>;
    fn AceFlags(&mut self) -> ::windows::core::Result<i32>;
    fn SetAceFlags(&mut self, lnaceflags: i32) -> ::windows::core::Result<()>;
    fn Flags(&mut self) -> ::windows::core::Result<i32>;
    fn SetFlags(&mut self, lnflags: i32) -> ::windows::core::Result<()>;
    fn ObjectType(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetObjectType(&mut self, bstrobjecttype: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn InheritedObjectType(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetInheritedObjectType(&mut self, bstrinheritedobjecttype: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Trustee(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetTrustee(&mut self, bstrtrustee: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsAccessControlEntry_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsAccessControlEntry_Impl, const OFFSET: isize>() -> IADsAccessControlEntry_Vtbl {
        unsafe extern "system" fn AccessMask<Identity: ::windows::core::IUnknownImpl, Impl: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AccessMask() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccessMask<Identity: ::windows::core::IUnknownImpl, Impl: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnaccessmask: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAccessMask(::core::mem::transmute_copy(&lnaccessmask)).into()
        }
        unsafe extern "system" fn AceType<Identity: ::windows::core::IUnknownImpl, Impl: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AceType() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAceType<Identity: ::windows::core::IUnknownImpl, Impl: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnacetype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAceType(::core::mem::transmute_copy(&lnacetype)).into()
        }
        unsafe extern "system" fn AceFlags<Identity: ::windows::core::IUnknownImpl, Impl: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AceFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAceFlags<Identity: ::windows::core::IUnknownImpl, Impl: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnaceflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAceFlags(::core::mem::transmute_copy(&lnaceflags)).into()
        }
        unsafe extern "system" fn Flags<Identity: ::windows::core::IUnknownImpl, Impl: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Flags() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlags<Identity: ::windows::core::IUnknownImpl, Impl: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFlags(::core::mem::transmute_copy(&lnflags)).into()
        }
        unsafe extern "system" fn ObjectType<Identity: ::windows::core::IUnknownImpl, Impl: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ObjectType() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetObjectType<Identity: ::windows::core::IUnknownImpl, Impl: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrobjecttype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetObjectType(::core::mem::transmute_copy(&bstrobjecttype)).into()
        }
        unsafe extern "system" fn InheritedObjectType<Identity: ::windows::core::IUnknownImpl, Impl: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InheritedObjectType() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInheritedObjectType<Identity: ::windows::core::IUnknownImpl, Impl: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinheritedobjecttype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInheritedObjectType(::core::mem::transmute_copy(&bstrinheritedobjecttype)).into()
        }
        unsafe extern "system" fn Trustee<Identity: ::windows::core::IUnknownImpl, Impl: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Trustee() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrustee<Identity: ::windows::core::IUnknownImpl, Impl: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtrustee: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTrustee(::core::mem::transmute_copy(&bstrtrustee)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            AccessMask: AccessMask::<Identity, Impl, OFFSET>,
            SetAccessMask: SetAccessMask::<Identity, Impl, OFFSET>,
            AceType: AceType::<Identity, Impl, OFFSET>,
            SetAceType: SetAceType::<Identity, Impl, OFFSET>,
            AceFlags: AceFlags::<Identity, Impl, OFFSET>,
            SetAceFlags: SetAceFlags::<Identity, Impl, OFFSET>,
            Flags: Flags::<Identity, Impl, OFFSET>,
            SetFlags: SetFlags::<Identity, Impl, OFFSET>,
            ObjectType: ObjectType::<Identity, Impl, OFFSET>,
            SetObjectType: SetObjectType::<Identity, Impl, OFFSET>,
            InheritedObjectType: InheritedObjectType::<Identity, Impl, OFFSET>,
            SetInheritedObjectType: SetInheritedObjectType::<Identity, Impl, OFFSET>,
            Trustee: Trustee::<Identity, Impl, OFFSET>,
            SetTrustee: SetTrustee::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsAccessControlEntry as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsAccessControlList_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn AclRevision(&mut self) -> ::windows::core::Result<i32>;
    fn SetAclRevision(&mut self, lnaclrevision: i32) -> ::windows::core::Result<()>;
    fn AceCount(&mut self) -> ::windows::core::Result<i32>;
    fn SetAceCount(&mut self, lnacecount: i32) -> ::windows::core::Result<()>;
    fn AddAce(&mut self, paccesscontrolentry: &::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn RemoveAce(&mut self, paccesscontrolentry: &::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn CopyAccessList(&mut self) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsAccessControlList_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsAccessControlList_Impl, const OFFSET: isize>() -> IADsAccessControlList_Vtbl {
        unsafe extern "system" fn AclRevision<Identity: ::windows::core::IUnknownImpl, Impl: IADsAccessControlList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AclRevision() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAclRevision<Identity: ::windows::core::IUnknownImpl, Impl: IADsAccessControlList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnaclrevision: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAclRevision(::core::mem::transmute_copy(&lnaclrevision)).into()
        }
        unsafe extern "system" fn AceCount<Identity: ::windows::core::IUnknownImpl, Impl: IADsAccessControlList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AceCount() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAceCount<Identity: ::windows::core::IUnknownImpl, Impl: IADsAccessControlList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnacecount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAceCount(::core::mem::transmute_copy(&lnacecount)).into()
        }
        unsafe extern "system" fn AddAce<Identity: ::windows::core::IUnknownImpl, Impl: IADsAccessControlList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paccesscontrolentry: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddAce(::core::mem::transmute(&paccesscontrolentry)).into()
        }
        unsafe extern "system" fn RemoveAce<Identity: ::windows::core::IUnknownImpl, Impl: IADsAccessControlList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paccesscontrolentry: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveAce(::core::mem::transmute(&paccesscontrolentry)).into()
        }
        unsafe extern "system" fn CopyAccessList<Identity: ::windows::core::IUnknownImpl, Impl: IADsAccessControlList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaccesscontrollist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CopyAccessList() {
                ::core::result::Result::Ok(ok__) => {
                    *ppaccesscontrollist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IADsAccessControlList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            AclRevision: AclRevision::<Identity, Impl, OFFSET>,
            SetAclRevision: SetAclRevision::<Identity, Impl, OFFSET>,
            AceCount: AceCount::<Identity, Impl, OFFSET>,
            SetAceCount: SetAceCount::<Identity, Impl, OFFSET>,
            AddAce: AddAce::<Identity, Impl, OFFSET>,
            RemoveAce: RemoveAce::<Identity, Impl, OFFSET>,
            CopyAccessList: CopyAccessList::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsAccessControlList as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsAcl_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ProtectedAttrName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetProtectedAttrName(&mut self, bstrprotectedattrname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SubjectName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetSubjectName(&mut self, bstrsubjectname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Privileges(&mut self) -> ::windows::core::Result<i32>;
    fn SetPrivileges(&mut self, lnprivileges: i32) -> ::windows::core::Result<()>;
    fn CopyAcl(&mut self) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsAcl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsAcl_Impl, const OFFSET: isize>() -> IADsAcl_Vtbl {
        unsafe extern "system" fn ProtectedAttrName<Identity: ::windows::core::IUnknownImpl, Impl: IADsAcl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ProtectedAttrName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProtectedAttrName<Identity: ::windows::core::IUnknownImpl, Impl: IADsAcl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotectedattrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProtectedAttrName(::core::mem::transmute_copy(&bstrprotectedattrname)).into()
        }
        unsafe extern "system" fn SubjectName<Identity: ::windows::core::IUnknownImpl, Impl: IADsAcl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SubjectName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubjectName<Identity: ::windows::core::IUnknownImpl, Impl: IADsAcl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsubjectname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSubjectName(::core::mem::transmute_copy(&bstrsubjectname)).into()
        }
        unsafe extern "system" fn Privileges<Identity: ::windows::core::IUnknownImpl, Impl: IADsAcl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Privileges() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivileges<Identity: ::windows::core::IUnknownImpl, Impl: IADsAcl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnprivileges: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPrivileges(::core::mem::transmute_copy(&lnprivileges)).into()
        }
        unsafe extern "system" fn CopyAcl<Identity: ::windows::core::IUnknownImpl, Impl: IADsAcl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppacl: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CopyAcl() {
                ::core::result::Result::Ok(ok__) => {
                    *ppacl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ProtectedAttrName: ProtectedAttrName::<Identity, Impl, OFFSET>,
            SetProtectedAttrName: SetProtectedAttrName::<Identity, Impl, OFFSET>,
            SubjectName: SubjectName::<Identity, Impl, OFFSET>,
            SetSubjectName: SetSubjectName::<Identity, Impl, OFFSET>,
            Privileges: Privileges::<Identity, Impl, OFFSET>,
            SetPrivileges: SetPrivileges::<Identity, Impl, OFFSET>,
            CopyAcl: CopyAcl::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsAcl as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
pub trait IADsAggregatee_Impl: Sized {
    fn ConnectAsAggregatee(&mut self, pouterunknown: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn DisconnectAsAggregatee(&mut self) -> ::windows::core::Result<()>;
    fn RelinquishInterface(&mut self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RestoreInterface(&mut self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl IADsAggregatee_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsAggregatee_Impl, const OFFSET: isize>() -> IADsAggregatee_Vtbl {
        unsafe extern "system" fn ConnectAsAggregatee<Identity: ::windows::core::IUnknownImpl, Impl: IADsAggregatee_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pouterunknown: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ConnectAsAggregatee(::core::mem::transmute(&pouterunknown)).into()
        }
        unsafe extern "system" fn DisconnectAsAggregatee<Identity: ::windows::core::IUnknownImpl, Impl: IADsAggregatee_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DisconnectAsAggregatee().into()
        }
        unsafe extern "system" fn RelinquishInterface<Identity: ::windows::core::IUnknownImpl, Impl: IADsAggregatee_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RelinquishInterface(::core::mem::transmute_copy(&riid)).into()
        }
        unsafe extern "system" fn RestoreInterface<Identity: ::windows::core::IUnknownImpl, Impl: IADsAggregatee_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RestoreInterface(::core::mem::transmute_copy(&riid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ConnectAsAggregatee: ConnectAsAggregatee::<Identity, Impl, OFFSET>,
            DisconnectAsAggregatee: DisconnectAsAggregatee::<Identity, Impl, OFFSET>,
            RelinquishInterface: RelinquishInterface::<Identity, Impl, OFFSET>,
            RestoreInterface: RestoreInterface::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsAggregatee as ::windows::core::Interface>::IID
    }
}
pub trait IADsAggregator_Impl: Sized {
    fn ConnectAsAggregator(&mut self, paggregatee: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn DisconnectAsAggregator(&mut self) -> ::windows::core::Result<()>;
}
impl IADsAggregator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsAggregator_Impl, const OFFSET: isize>() -> IADsAggregator_Vtbl {
        unsafe extern "system" fn ConnectAsAggregator<Identity: ::windows::core::IUnknownImpl, Impl: IADsAggregator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paggregatee: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ConnectAsAggregator(::core::mem::transmute(&paggregatee)).into()
        }
        unsafe extern "system" fn DisconnectAsAggregator<Identity: ::windows::core::IUnknownImpl, Impl: IADsAggregator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DisconnectAsAggregator().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ConnectAsAggregator: ConnectAsAggregator::<Identity, Impl, OFFSET>,
            DisconnectAsAggregator: DisconnectAsAggregator::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsAggregator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsBackLink_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn RemoteID(&mut self) -> ::windows::core::Result<i32>;
    fn SetRemoteID(&mut self, lnremoteid: i32) -> ::windows::core::Result<()>;
    fn ObjectName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetObjectName(&mut self, bstrobjectname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsBackLink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsBackLink_Impl, const OFFSET: isize>() -> IADsBackLink_Vtbl {
        unsafe extern "system" fn RemoteID<Identity: ::windows::core::IUnknownImpl, Impl: IADsBackLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RemoteID() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteID<Identity: ::windows::core::IUnknownImpl, Impl: IADsBackLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnremoteid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRemoteID(::core::mem::transmute_copy(&lnremoteid)).into()
        }
        unsafe extern "system" fn ObjectName<Identity: ::windows::core::IUnknownImpl, Impl: IADsBackLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ObjectName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetObjectName<Identity: ::windows::core::IUnknownImpl, Impl: IADsBackLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrobjectname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetObjectName(::core::mem::transmute_copy(&bstrobjectname)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            RemoteID: RemoteID::<Identity, Impl, OFFSET>,
            SetRemoteID: SetRemoteID::<Identity, Impl, OFFSET>,
            ObjectName: ObjectName::<Identity, Impl, OFFSET>,
            SetObjectName: SetObjectName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsBackLink as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsCaseIgnoreList_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CaseIgnoreList(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetCaseIgnoreList(&mut self, vcaseignorelist: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsCaseIgnoreList_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsCaseIgnoreList_Impl, const OFFSET: isize>() -> IADsCaseIgnoreList_Vtbl {
        unsafe extern "system" fn CaseIgnoreList<Identity: ::windows::core::IUnknownImpl, Impl: IADsCaseIgnoreList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CaseIgnoreList() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCaseIgnoreList<Identity: ::windows::core::IUnknownImpl, Impl: IADsCaseIgnoreList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcaseignorelist: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCaseIgnoreList(::core::mem::transmute_copy(&vcaseignorelist)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            CaseIgnoreList: CaseIgnoreList::<Identity, Impl, OFFSET>,
            SetCaseIgnoreList: SetCaseIgnoreList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsCaseIgnoreList as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsClass_Impl: Sized + super::super::System::Com::IDispatch_Impl + IADs_Impl {
    fn PrimaryInterface(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CLSID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetCLSID(&mut self, bstrclsid: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn OID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetOID(&mut self, bstroid: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Abstract(&mut self) -> ::windows::core::Result<i16>;
    fn SetAbstract(&mut self, fabstract: i16) -> ::windows::core::Result<()>;
    fn Auxiliary(&mut self) -> ::windows::core::Result<i16>;
    fn SetAuxiliary(&mut self, fauxiliary: i16) -> ::windows::core::Result<()>;
    fn MandatoryProperties(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetMandatoryProperties(&mut self, vmandatoryproperties: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn OptionalProperties(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetOptionalProperties(&mut self, voptionalproperties: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn NamingProperties(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetNamingProperties(&mut self, vnamingproperties: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DerivedFrom(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetDerivedFrom(&mut self, vderivedfrom: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AuxDerivedFrom(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetAuxDerivedFrom(&mut self, vauxderivedfrom: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn PossibleSuperiors(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetPossibleSuperiors(&mut self, vpossiblesuperiors: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Containment(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetContainment(&mut self, vcontainment: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Container(&mut self) -> ::windows::core::Result<i16>;
    fn SetContainer(&mut self, fcontainer: i16) -> ::windows::core::Result<()>;
    fn HelpFileName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetHelpFileName(&mut self, bstrhelpfilename: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn HelpFileContext(&mut self) -> ::windows::core::Result<i32>;
    fn SetHelpFileContext(&mut self, lnhelpfilecontext: i32) -> ::windows::core::Result<()>;
    fn Qualifiers(&mut self) -> ::windows::core::Result<IADsCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsClass_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsClass_Impl, const OFFSET: isize>() -> IADsClass_Vtbl {
        unsafe extern "system" fn PrimaryInterface<Identity: ::windows::core::IUnknownImpl, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PrimaryInterface() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CLSID<Identity: ::windows::core::IUnknownImpl, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CLSID() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCLSID<Identity: ::windows::core::IUnknownImpl, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrclsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCLSID(::core::mem::transmute_copy(&bstrclsid)).into()
        }
        unsafe extern "system" fn OID<Identity: ::windows::core::IUnknownImpl, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OID() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOID<Identity: ::windows::core::IUnknownImpl, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstroid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOID(::core::mem::transmute_copy(&bstroid)).into()
        }
        unsafe extern "system" fn Abstract<Identity: ::windows::core::IUnknownImpl, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Abstract() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAbstract<Identity: ::windows::core::IUnknownImpl, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fabstract: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAbstract(::core::mem::transmute_copy(&fabstract)).into()
        }
        unsafe extern "system" fn Auxiliary<Identity: ::windows::core::IUnknownImpl, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Auxiliary() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuxiliary<Identity: ::windows::core::IUnknownImpl, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fauxiliary: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAuxiliary(::core::mem::transmute_copy(&fauxiliary)).into()
        }
        unsafe extern "system" fn MandatoryProperties<Identity: ::windows::core::IUnknownImpl, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MandatoryProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMandatoryProperties<Identity: ::windows::core::IUnknownImpl, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vmandatoryproperties: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMandatoryProperties(::core::mem::transmute_copy(&vmandatoryproperties)).into()
        }
        unsafe extern "system" fn OptionalProperties<Identity: ::windows::core::IUnknownImpl, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OptionalProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOptionalProperties<Identity: ::windows::core::IUnknownImpl, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, voptionalproperties: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOptionalProperties(::core::mem::transmute_copy(&voptionalproperties)).into()
        }
        unsafe extern "system" fn NamingProperties<Identity: ::windows::core::IUnknownImpl, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NamingProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNamingProperties<Identity: ::windows::core::IUnknownImpl, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vnamingproperties: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNamingProperties(::core::mem::transmute_copy(&vnamingproperties)).into()
        }
        unsafe extern "system" fn DerivedFrom<Identity: ::windows::core::IUnknownImpl, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DerivedFrom() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDerivedFrom<Identity: ::windows::core::IUnknownImpl, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vderivedfrom: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDerivedFrom(::core::mem::transmute_copy(&vderivedfrom)).into()
        }
        unsafe extern "system" fn AuxDerivedFrom<Identity: ::windows::core::IUnknownImpl, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AuxDerivedFrom() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuxDerivedFrom<Identity: ::windows::core::IUnknownImpl, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vauxderivedfrom: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAuxDerivedFrom(::core::mem::transmute_copy(&vauxderivedfrom)).into()
        }
        unsafe extern "system" fn PossibleSuperiors<Identity: ::windows::core::IUnknownImpl, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PossibleSuperiors() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPossibleSuperiors<Identity: ::windows::core::IUnknownImpl, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vpossiblesuperiors: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPossibleSuperiors(::core::mem::transmute_copy(&vpossiblesuperiors)).into()
        }
        unsafe extern "system" fn Containment<Identity: ::windows::core::IUnknownImpl, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Containment() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContainment<Identity: ::windows::core::IUnknownImpl, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcontainment: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetContainment(::core::mem::transmute_copy(&vcontainment)).into()
        }
        unsafe extern "system" fn Container<Identity: ::windows::core::IUnknownImpl, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Container() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContainer<Identity: ::windows::core::IUnknownImpl, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fcontainer: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetContainer(::core::mem::transmute_copy(&fcontainer)).into()
        }
        unsafe extern "system" fn HelpFileName<Identity: ::windows::core::IUnknownImpl, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HelpFileName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHelpFileName<Identity: ::windows::core::IUnknownImpl, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrhelpfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHelpFileName(::core::mem::transmute_copy(&bstrhelpfilename)).into()
        }
        unsafe extern "system" fn HelpFileContext<Identity: ::windows::core::IUnknownImpl, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HelpFileContext() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHelpFileContext<Identity: ::windows::core::IUnknownImpl, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnhelpfilecontext: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHelpFileContext(::core::mem::transmute_copy(&lnhelpfilecontext)).into()
        }
        unsafe extern "system" fn Qualifiers<Identity: ::windows::core::IUnknownImpl, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqualifiers: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Qualifiers() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqualifiers = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            PrimaryInterface: PrimaryInterface::<Identity, Impl, OFFSET>,
            CLSID: CLSID::<Identity, Impl, OFFSET>,
            SetCLSID: SetCLSID::<Identity, Impl, OFFSET>,
            OID: OID::<Identity, Impl, OFFSET>,
            SetOID: SetOID::<Identity, Impl, OFFSET>,
            Abstract: Abstract::<Identity, Impl, OFFSET>,
            SetAbstract: SetAbstract::<Identity, Impl, OFFSET>,
            Auxiliary: Auxiliary::<Identity, Impl, OFFSET>,
            SetAuxiliary: SetAuxiliary::<Identity, Impl, OFFSET>,
            MandatoryProperties: MandatoryProperties::<Identity, Impl, OFFSET>,
            SetMandatoryProperties: SetMandatoryProperties::<Identity, Impl, OFFSET>,
            OptionalProperties: OptionalProperties::<Identity, Impl, OFFSET>,
            SetOptionalProperties: SetOptionalProperties::<Identity, Impl, OFFSET>,
            NamingProperties: NamingProperties::<Identity, Impl, OFFSET>,
            SetNamingProperties: SetNamingProperties::<Identity, Impl, OFFSET>,
            DerivedFrom: DerivedFrom::<Identity, Impl, OFFSET>,
            SetDerivedFrom: SetDerivedFrom::<Identity, Impl, OFFSET>,
            AuxDerivedFrom: AuxDerivedFrom::<Identity, Impl, OFFSET>,
            SetAuxDerivedFrom: SetAuxDerivedFrom::<Identity, Impl, OFFSET>,
            PossibleSuperiors: PossibleSuperiors::<Identity, Impl, OFFSET>,
            SetPossibleSuperiors: SetPossibleSuperiors::<Identity, Impl, OFFSET>,
            Containment: Containment::<Identity, Impl, OFFSET>,
            SetContainment: SetContainment::<Identity, Impl, OFFSET>,
            Container: Container::<Identity, Impl, OFFSET>,
            SetContainer: SetContainer::<Identity, Impl, OFFSET>,
            HelpFileName: HelpFileName::<Identity, Impl, OFFSET>,
            SetHelpFileName: SetHelpFileName::<Identity, Impl, OFFSET>,
            HelpFileContext: HelpFileContext::<Identity, Impl, OFFSET>,
            SetHelpFileContext: SetHelpFileContext::<Identity, Impl, OFFSET>,
            Qualifiers: Qualifiers::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsClass as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IADs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Add(&mut self, bstrname: &super::super::Foundation::BSTR, vitem: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Remove(&mut self, bstritemtoberemoved: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetObject(&mut self, bstrname: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsCollection_Impl, const OFFSET: isize>() -> IADsCollection_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IADsCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumerator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl, Impl: IADsCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, vitem: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Add(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&vitem)).into()
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl, Impl: IADsCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemtoberemoved: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&bstritemtoberemoved)).into()
        }
        unsafe extern "system" fn GetObject<Identity: ::windows::core::IUnknownImpl, Impl: IADsCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvitem: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetObject(::core::mem::transmute_copy(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            GetObject: GetObject::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsCollection as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsComputer_Impl: Sized + super::super::System::Com::IDispatch_Impl + IADs_Impl {
    fn ComputerID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Site(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, bstrdescription: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Location(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLocation(&mut self, bstrlocation: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PrimaryUser(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPrimaryUser(&mut self, bstrprimaryuser: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Owner(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetOwner(&mut self, bstrowner: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Division(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDivision(&mut self, bstrdivision: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Department(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDepartment(&mut self, bstrdepartment: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Role(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetRole(&mut self, bstrrole: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn OperatingSystem(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetOperatingSystem(&mut self, bstroperatingsystem: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn OperatingSystemVersion(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetOperatingSystemVersion(&mut self, bstroperatingsystemversion: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Model(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetModel(&mut self, bstrmodel: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Processor(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetProcessor(&mut self, bstrprocessor: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ProcessorCount(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetProcessorCount(&mut self, bstrprocessorcount: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn MemorySize(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetMemorySize(&mut self, bstrmemorysize: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn StorageCapacity(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetStorageCapacity(&mut self, bstrstoragecapacity: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn NetAddresses(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetNetAddresses(&mut self, vnetaddresses: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsComputer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsComputer_Impl, const OFFSET: isize>() -> IADsComputer_Vtbl {
        unsafe extern "system" fn ComputerID<Identity: ::windows::core::IUnknownImpl, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ComputerID() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Site<Identity: ::windows::core::IUnknownImpl, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Site() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&bstrdescription)).into()
        }
        unsafe extern "system" fn Location<Identity: ::windows::core::IUnknownImpl, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Location() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocation<Identity: ::windows::core::IUnknownImpl, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlocation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLocation(::core::mem::transmute_copy(&bstrlocation)).into()
        }
        unsafe extern "system" fn PrimaryUser<Identity: ::windows::core::IUnknownImpl, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PrimaryUser() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrimaryUser<Identity: ::windows::core::IUnknownImpl, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprimaryuser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPrimaryUser(::core::mem::transmute_copy(&bstrprimaryuser)).into()
        }
        unsafe extern "system" fn Owner<Identity: ::windows::core::IUnknownImpl, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Owner() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOwner<Identity: ::windows::core::IUnknownImpl, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrowner: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOwner(::core::mem::transmute_copy(&bstrowner)).into()
        }
        unsafe extern "system" fn Division<Identity: ::windows::core::IUnknownImpl, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Division() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDivision<Identity: ::windows::core::IUnknownImpl, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdivision: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDivision(::core::mem::transmute_copy(&bstrdivision)).into()
        }
        unsafe extern "system" fn Department<Identity: ::windows::core::IUnknownImpl, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Department() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDepartment<Identity: ::windows::core::IUnknownImpl, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdepartment: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDepartment(::core::mem::transmute_copy(&bstrdepartment)).into()
        }
        unsafe extern "system" fn Role<Identity: ::windows::core::IUnknownImpl, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Role() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRole<Identity: ::windows::core::IUnknownImpl, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRole(::core::mem::transmute_copy(&bstrrole)).into()
        }
        unsafe extern "system" fn OperatingSystem<Identity: ::windows::core::IUnknownImpl, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OperatingSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOperatingSystem<Identity: ::windows::core::IUnknownImpl, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstroperatingsystem: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOperatingSystem(::core::mem::transmute_copy(&bstroperatingsystem)).into()
        }
        unsafe extern "system" fn OperatingSystemVersion<Identity: ::windows::core::IUnknownImpl, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OperatingSystemVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOperatingSystemVersion<Identity: ::windows::core::IUnknownImpl, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstroperatingsystemversion: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOperatingSystemVersion(::core::mem::transmute_copy(&bstroperatingsystemversion)).into()
        }
        unsafe extern "system" fn Model<Identity: ::windows::core::IUnknownImpl, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Model() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetModel<Identity: ::windows::core::IUnknownImpl, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmodel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetModel(::core::mem::transmute_copy(&bstrmodel)).into()
        }
        unsafe extern "system" fn Processor<Identity: ::windows::core::IUnknownImpl, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Processor() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProcessor<Identity: ::windows::core::IUnknownImpl, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprocessor: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProcessor(::core::mem::transmute_copy(&bstrprocessor)).into()
        }
        unsafe extern "system" fn ProcessorCount<Identity: ::windows::core::IUnknownImpl, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ProcessorCount() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProcessorCount<Identity: ::windows::core::IUnknownImpl, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprocessorcount: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProcessorCount(::core::mem::transmute_copy(&bstrprocessorcount)).into()
        }
        unsafe extern "system" fn MemorySize<Identity: ::windows::core::IUnknownImpl, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MemorySize() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMemorySize<Identity: ::windows::core::IUnknownImpl, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmemorysize: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMemorySize(::core::mem::transmute_copy(&bstrmemorysize)).into()
        }
        unsafe extern "system" fn StorageCapacity<Identity: ::windows::core::IUnknownImpl, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StorageCapacity() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStorageCapacity<Identity: ::windows::core::IUnknownImpl, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrstoragecapacity: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStorageCapacity(::core::mem::transmute_copy(&bstrstoragecapacity)).into()
        }
        unsafe extern "system" fn NetAddresses<Identity: ::windows::core::IUnknownImpl, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NetAddresses() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetAddresses<Identity: ::windows::core::IUnknownImpl, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vnetaddresses: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNetAddresses(::core::mem::transmute_copy(&vnetaddresses)).into()
        }
        Self {
            base: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            ComputerID: ComputerID::<Identity, Impl, OFFSET>,
            Site: Site::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            Location: Location::<Identity, Impl, OFFSET>,
            SetLocation: SetLocation::<Identity, Impl, OFFSET>,
            PrimaryUser: PrimaryUser::<Identity, Impl, OFFSET>,
            SetPrimaryUser: SetPrimaryUser::<Identity, Impl, OFFSET>,
            Owner: Owner::<Identity, Impl, OFFSET>,
            SetOwner: SetOwner::<Identity, Impl, OFFSET>,
            Division: Division::<Identity, Impl, OFFSET>,
            SetDivision: SetDivision::<Identity, Impl, OFFSET>,
            Department: Department::<Identity, Impl, OFFSET>,
            SetDepartment: SetDepartment::<Identity, Impl, OFFSET>,
            Role: Role::<Identity, Impl, OFFSET>,
            SetRole: SetRole::<Identity, Impl, OFFSET>,
            OperatingSystem: OperatingSystem::<Identity, Impl, OFFSET>,
            SetOperatingSystem: SetOperatingSystem::<Identity, Impl, OFFSET>,
            OperatingSystemVersion: OperatingSystemVersion::<Identity, Impl, OFFSET>,
            SetOperatingSystemVersion: SetOperatingSystemVersion::<Identity, Impl, OFFSET>,
            Model: Model::<Identity, Impl, OFFSET>,
            SetModel: SetModel::<Identity, Impl, OFFSET>,
            Processor: Processor::<Identity, Impl, OFFSET>,
            SetProcessor: SetProcessor::<Identity, Impl, OFFSET>,
            ProcessorCount: ProcessorCount::<Identity, Impl, OFFSET>,
            SetProcessorCount: SetProcessorCount::<Identity, Impl, OFFSET>,
            MemorySize: MemorySize::<Identity, Impl, OFFSET>,
            SetMemorySize: SetMemorySize::<Identity, Impl, OFFSET>,
            StorageCapacity: StorageCapacity::<Identity, Impl, OFFSET>,
            SetStorageCapacity: SetStorageCapacity::<Identity, Impl, OFFSET>,
            NetAddresses: NetAddresses::<Identity, Impl, OFFSET>,
            SetNetAddresses: SetNetAddresses::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsComputer as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IADs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsComputerOperations_Impl: Sized + super::super::System::Com::IDispatch_Impl + IADs_Impl {
    fn Status(&mut self) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn Shutdown(&mut self, breboot: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsComputerOperations_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsComputerOperations_Impl, const OFFSET: isize>() -> IADsComputerOperations_Vtbl {
        unsafe extern "system" fn Status<Identity: ::windows::core::IUnknownImpl, Impl: IADsComputerOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *ppobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shutdown<Identity: ::windows::core::IUnknownImpl, Impl: IADsComputerOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, breboot: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Shutdown(::core::mem::transmute_copy(&breboot)).into()
        }
        Self { base: IADs_Vtbl::new::<Identity, Impl, OFFSET>(), Status: Status::<Identity, Impl, OFFSET>, Shutdown: Shutdown::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsComputerOperations as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IADs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsContainer_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Filter(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetFilter(&mut self, var: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Hints(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetHints(&mut self, vhints: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GetObject(&mut self, classname: &super::super::Foundation::BSTR, relativename: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn Create(&mut self, classname: &super::super::Foundation::BSTR, relativename: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn Delete(&mut self, bstrclassname: &super::super::Foundation::BSTR, bstrrelativename: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn CopyHere(&mut self, sourcename: &super::super::Foundation::BSTR, newname: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn MoveHere(&mut self, sourcename: &super::super::Foundation::BSTR, newname: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsContainer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsContainer_Impl, const OFFSET: isize>() -> IADsContainer_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IADsContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IADsContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Filter<Identity: ::windows::core::IUnknownImpl, Impl: IADsContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvar: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Filter() {
                ::core::result::Result::Ok(ok__) => {
                    *pvar = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFilter<Identity: ::windows::core::IUnknownImpl, Impl: IADsContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, var: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFilter(::core::mem::transmute_copy(&var)).into()
        }
        unsafe extern "system" fn Hints<Identity: ::windows::core::IUnknownImpl, Impl: IADsContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvfilter: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Hints() {
                ::core::result::Result::Ok(ok__) => {
                    *pvfilter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHints<Identity: ::windows::core::IUnknownImpl, Impl: IADsContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vhints: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHints(::core::mem::transmute_copy(&vhints)).into()
        }
        unsafe extern "system" fn GetObject<Identity: ::windows::core::IUnknownImpl, Impl: IADsContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, classname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, relativename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetObject(::core::mem::transmute_copy(&classname), ::core::mem::transmute_copy(&relativename)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Identity: ::windows::core::IUnknownImpl, Impl: IADsContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, classname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, relativename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Create(::core::mem::transmute_copy(&classname), ::core::mem::transmute_copy(&relativename)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl, Impl: IADsContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrclassname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrrelativename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Delete(::core::mem::transmute_copy(&bstrclassname), ::core::mem::transmute_copy(&bstrrelativename)).into()
        }
        unsafe extern "system" fn CopyHere<Identity: ::windows::core::IUnknownImpl, Impl: IADsContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, newname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CopyHere(::core::mem::transmute_copy(&sourcename), ::core::mem::transmute_copy(&newname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveHere<Identity: ::windows::core::IUnknownImpl, Impl: IADsContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, newname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MoveHere(::core::mem::transmute_copy(&sourcename), ::core::mem::transmute_copy(&newname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Filter: Filter::<Identity, Impl, OFFSET>,
            SetFilter: SetFilter::<Identity, Impl, OFFSET>,
            Hints: Hints::<Identity, Impl, OFFSET>,
            SetHints: SetHints::<Identity, Impl, OFFSET>,
            GetObject: GetObject::<Identity, Impl, OFFSET>,
            Create: Create::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            CopyHere: CopyHere::<Identity, Impl, OFFSET>,
            MoveHere: MoveHere::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsContainer as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsDNWithBinary_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn BinaryValue(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetBinaryValue(&mut self, vbinaryvalue: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DNString(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDNString(&mut self, bstrdnstring: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsDNWithBinary_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsDNWithBinary_Impl, const OFFSET: isize>() -> IADsDNWithBinary_Vtbl {
        unsafe extern "system" fn BinaryValue<Identity: ::windows::core::IUnknownImpl, Impl: IADsDNWithBinary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BinaryValue() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBinaryValue<Identity: ::windows::core::IUnknownImpl, Impl: IADsDNWithBinary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vbinaryvalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBinaryValue(::core::mem::transmute_copy(&vbinaryvalue)).into()
        }
        unsafe extern "system" fn DNString<Identity: ::windows::core::IUnknownImpl, Impl: IADsDNWithBinary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DNString() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDNString<Identity: ::windows::core::IUnknownImpl, Impl: IADsDNWithBinary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdnstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDNString(::core::mem::transmute_copy(&bstrdnstring)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            BinaryValue: BinaryValue::<Identity, Impl, OFFSET>,
            SetBinaryValue: SetBinaryValue::<Identity, Impl, OFFSET>,
            DNString: DNString::<Identity, Impl, OFFSET>,
            SetDNString: SetDNString::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsDNWithBinary as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsDNWithString_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn StringValue(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetStringValue(&mut self, bstrstringvalue: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DNString(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDNString(&mut self, bstrdnstring: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsDNWithString_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsDNWithString_Impl, const OFFSET: isize>() -> IADsDNWithString_Vtbl {
        unsafe extern "system" fn StringValue<Identity: ::windows::core::IUnknownImpl, Impl: IADsDNWithString_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StringValue() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStringValue<Identity: ::windows::core::IUnknownImpl, Impl: IADsDNWithString_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrstringvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStringValue(::core::mem::transmute_copy(&bstrstringvalue)).into()
        }
        unsafe extern "system" fn DNString<Identity: ::windows::core::IUnknownImpl, Impl: IADsDNWithString_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DNString() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDNString<Identity: ::windows::core::IUnknownImpl, Impl: IADsDNWithString_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdnstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDNString(::core::mem::transmute_copy(&bstrdnstring)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            StringValue: StringValue::<Identity, Impl, OFFSET>,
            SetStringValue: SetStringValue::<Identity, Impl, OFFSET>,
            DNString: DNString::<Identity, Impl, OFFSET>,
            SetDNString: SetDNString::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsDNWithString as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsDeleteOps_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn DeleteObject(&mut self, lnflags: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsDeleteOps_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsDeleteOps_Impl, const OFFSET: isize>() -> IADsDeleteOps_Vtbl {
        unsafe extern "system" fn DeleteObject<Identity: ::windows::core::IUnknownImpl, Impl: IADsDeleteOps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteObject(::core::mem::transmute_copy(&lnflags)).into()
        }
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), DeleteObject: DeleteObject::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsDeleteOps as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsDomain_Impl: Sized + super::super::System::Com::IDispatch_Impl + IADs_Impl {
    fn IsWorkgroup(&mut self) -> ::windows::core::Result<i16>;
    fn MinPasswordLength(&mut self) -> ::windows::core::Result<i32>;
    fn SetMinPasswordLength(&mut self, lnminpasswordlength: i32) -> ::windows::core::Result<()>;
    fn MinPasswordAge(&mut self) -> ::windows::core::Result<i32>;
    fn SetMinPasswordAge(&mut self, lnminpasswordage: i32) -> ::windows::core::Result<()>;
    fn MaxPasswordAge(&mut self) -> ::windows::core::Result<i32>;
    fn SetMaxPasswordAge(&mut self, lnmaxpasswordage: i32) -> ::windows::core::Result<()>;
    fn MaxBadPasswordsAllowed(&mut self) -> ::windows::core::Result<i32>;
    fn SetMaxBadPasswordsAllowed(&mut self, lnmaxbadpasswordsallowed: i32) -> ::windows::core::Result<()>;
    fn PasswordHistoryLength(&mut self) -> ::windows::core::Result<i32>;
    fn SetPasswordHistoryLength(&mut self, lnpasswordhistorylength: i32) -> ::windows::core::Result<()>;
    fn PasswordAttributes(&mut self) -> ::windows::core::Result<i32>;
    fn SetPasswordAttributes(&mut self, lnpasswordattributes: i32) -> ::windows::core::Result<()>;
    fn AutoUnlockInterval(&mut self) -> ::windows::core::Result<i32>;
    fn SetAutoUnlockInterval(&mut self, lnautounlockinterval: i32) -> ::windows::core::Result<()>;
    fn LockoutObservationInterval(&mut self) -> ::windows::core::Result<i32>;
    fn SetLockoutObservationInterval(&mut self, lnlockoutobservationinterval: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsDomain_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsDomain_Impl, const OFFSET: isize>() -> IADsDomain_Vtbl {
        unsafe extern "system" fn IsWorkgroup<Identity: ::windows::core::IUnknownImpl, Impl: IADsDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsWorkgroup() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinPasswordLength<Identity: ::windows::core::IUnknownImpl, Impl: IADsDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MinPasswordLength() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinPasswordLength<Identity: ::windows::core::IUnknownImpl, Impl: IADsDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnminpasswordlength: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMinPasswordLength(::core::mem::transmute_copy(&lnminpasswordlength)).into()
        }
        unsafe extern "system" fn MinPasswordAge<Identity: ::windows::core::IUnknownImpl, Impl: IADsDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MinPasswordAge() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinPasswordAge<Identity: ::windows::core::IUnknownImpl, Impl: IADsDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnminpasswordage: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMinPasswordAge(::core::mem::transmute_copy(&lnminpasswordage)).into()
        }
        unsafe extern "system" fn MaxPasswordAge<Identity: ::windows::core::IUnknownImpl, Impl: IADsDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MaxPasswordAge() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxPasswordAge<Identity: ::windows::core::IUnknownImpl, Impl: IADsDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnmaxpasswordage: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMaxPasswordAge(::core::mem::transmute_copy(&lnmaxpasswordage)).into()
        }
        unsafe extern "system" fn MaxBadPasswordsAllowed<Identity: ::windows::core::IUnknownImpl, Impl: IADsDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MaxBadPasswordsAllowed() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxBadPasswordsAllowed<Identity: ::windows::core::IUnknownImpl, Impl: IADsDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnmaxbadpasswordsallowed: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMaxBadPasswordsAllowed(::core::mem::transmute_copy(&lnmaxbadpasswordsallowed)).into()
        }
        unsafe extern "system" fn PasswordHistoryLength<Identity: ::windows::core::IUnknownImpl, Impl: IADsDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PasswordHistoryLength() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPasswordHistoryLength<Identity: ::windows::core::IUnknownImpl, Impl: IADsDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnpasswordhistorylength: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPasswordHistoryLength(::core::mem::transmute_copy(&lnpasswordhistorylength)).into()
        }
        unsafe extern "system" fn PasswordAttributes<Identity: ::windows::core::IUnknownImpl, Impl: IADsDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PasswordAttributes() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPasswordAttributes<Identity: ::windows::core::IUnknownImpl, Impl: IADsDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnpasswordattributes: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPasswordAttributes(::core::mem::transmute_copy(&lnpasswordattributes)).into()
        }
        unsafe extern "system" fn AutoUnlockInterval<Identity: ::windows::core::IUnknownImpl, Impl: IADsDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AutoUnlockInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoUnlockInterval<Identity: ::windows::core::IUnknownImpl, Impl: IADsDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnautounlockinterval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAutoUnlockInterval(::core::mem::transmute_copy(&lnautounlockinterval)).into()
        }
        unsafe extern "system" fn LockoutObservationInterval<Identity: ::windows::core::IUnknownImpl, Impl: IADsDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LockoutObservationInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLockoutObservationInterval<Identity: ::windows::core::IUnknownImpl, Impl: IADsDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnlockoutobservationinterval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLockoutObservationInterval(::core::mem::transmute_copy(&lnlockoutobservationinterval)).into()
        }
        Self {
            base: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            IsWorkgroup: IsWorkgroup::<Identity, Impl, OFFSET>,
            MinPasswordLength: MinPasswordLength::<Identity, Impl, OFFSET>,
            SetMinPasswordLength: SetMinPasswordLength::<Identity, Impl, OFFSET>,
            MinPasswordAge: MinPasswordAge::<Identity, Impl, OFFSET>,
            SetMinPasswordAge: SetMinPasswordAge::<Identity, Impl, OFFSET>,
            MaxPasswordAge: MaxPasswordAge::<Identity, Impl, OFFSET>,
            SetMaxPasswordAge: SetMaxPasswordAge::<Identity, Impl, OFFSET>,
            MaxBadPasswordsAllowed: MaxBadPasswordsAllowed::<Identity, Impl, OFFSET>,
            SetMaxBadPasswordsAllowed: SetMaxBadPasswordsAllowed::<Identity, Impl, OFFSET>,
            PasswordHistoryLength: PasswordHistoryLength::<Identity, Impl, OFFSET>,
            SetPasswordHistoryLength: SetPasswordHistoryLength::<Identity, Impl, OFFSET>,
            PasswordAttributes: PasswordAttributes::<Identity, Impl, OFFSET>,
            SetPasswordAttributes: SetPasswordAttributes::<Identity, Impl, OFFSET>,
            AutoUnlockInterval: AutoUnlockInterval::<Identity, Impl, OFFSET>,
            SetAutoUnlockInterval: SetAutoUnlockInterval::<Identity, Impl, OFFSET>,
            LockoutObservationInterval: LockoutObservationInterval::<Identity, Impl, OFFSET>,
            SetLockoutObservationInterval: SetLockoutObservationInterval::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsDomain as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IADs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsEmail_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Type(&mut self) -> ::windows::core::Result<i32>;
    fn SetType(&mut self, lntype: i32) -> ::windows::core::Result<()>;
    fn Address(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetAddress(&mut self, bstraddress: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsEmail_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsEmail_Impl, const OFFSET: isize>() -> IADsEmail_Vtbl {
        unsafe extern "system" fn Type<Identity: ::windows::core::IUnknownImpl, Impl: IADsEmail_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Identity: ::windows::core::IUnknownImpl, Impl: IADsEmail_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lntype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetType(::core::mem::transmute_copy(&lntype)).into()
        }
        unsafe extern "system" fn Address<Identity: ::windows::core::IUnknownImpl, Impl: IADsEmail_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Address() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAddress<Identity: ::windows::core::IUnknownImpl, Impl: IADsEmail_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstraddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAddress(::core::mem::transmute_copy(&bstraddress)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Type: Type::<Identity, Impl, OFFSET>,
            SetType: SetType::<Identity, Impl, OFFSET>,
            Address: Address::<Identity, Impl, OFFSET>,
            SetAddress: SetAddress::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsEmail as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsExtension_Impl: Sized {
    fn Operate(&mut self, dwcode: u32, vardata1: &super::super::System::Com::VARIANT, vardata2: &super::super::System::Com::VARIANT, vardata3: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn PrivateGetIDsOfNames(&mut self, riid: *const ::windows::core::GUID, rgsznames: *const *const u16, cnames: u32, lcid: u32) -> ::windows::core::Result<i32>;
    fn PrivateInvoke(&mut self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsExtension_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsExtension_Impl, const OFFSET: isize>() -> IADsExtension_Vtbl {
        unsafe extern "system" fn Operate<Identity: ::windows::core::IUnknownImpl, Impl: IADsExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcode: u32, vardata1: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, vardata2: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, vardata3: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Operate(::core::mem::transmute_copy(&dwcode), ::core::mem::transmute_copy(&vardata1), ::core::mem::transmute_copy(&vardata2), ::core::mem::transmute_copy(&vardata3)).into()
        }
        unsafe extern "system" fn PrivateGetIDsOfNames<Identity: ::windows::core::IUnknownImpl, Impl: IADsExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const *const u16, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PrivateGetIDsOfNames(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&rgsznames), ::core::mem::transmute_copy(&cnames), ::core::mem::transmute_copy(&lcid)) {
                ::core::result::Result::Ok(ok__) => {
                    *rgdispid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateInvoke<Identity: ::windows::core::IUnknownImpl, Impl: IADsExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PrivateInvoke(::core::mem::transmute_copy(&dispidmember), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&lcid), ::core::mem::transmute_copy(&wflags), ::core::mem::transmute_copy(&pdispparams), ::core::mem::transmute_copy(&pvarresult), ::core::mem::transmute_copy(&pexcepinfo), ::core::mem::transmute_copy(&puargerr)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Operate: Operate::<Identity, Impl, OFFSET>,
            PrivateGetIDsOfNames: PrivateGetIDsOfNames::<Identity, Impl, OFFSET>,
            PrivateInvoke: PrivateInvoke::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsExtension as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsFaxNumber_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn TelephoneNumber(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetTelephoneNumber(&mut self, bstrtelephonenumber: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Parameters(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetParameters(&mut self, vparameters: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsFaxNumber_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsFaxNumber_Impl, const OFFSET: isize>() -> IADsFaxNumber_Vtbl {
        unsafe extern "system" fn TelephoneNumber<Identity: ::windows::core::IUnknownImpl, Impl: IADsFaxNumber_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TelephoneNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTelephoneNumber<Identity: ::windows::core::IUnknownImpl, Impl: IADsFaxNumber_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtelephonenumber: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTelephoneNumber(::core::mem::transmute_copy(&bstrtelephonenumber)).into()
        }
        unsafe extern "system" fn Parameters<Identity: ::windows::core::IUnknownImpl, Impl: IADsFaxNumber_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Parameters() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParameters<Identity: ::windows::core::IUnknownImpl, Impl: IADsFaxNumber_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vparameters: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetParameters(::core::mem::transmute_copy(&vparameters)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            TelephoneNumber: TelephoneNumber::<Identity, Impl, OFFSET>,
            SetTelephoneNumber: SetTelephoneNumber::<Identity, Impl, OFFSET>,
            Parameters: Parameters::<Identity, Impl, OFFSET>,
            SetParameters: SetParameters::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsFaxNumber as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsFileService_Impl: Sized + super::super::System::Com::IDispatch_Impl + IADs_Impl + IADsService_Impl {
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, bstrdescription: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn MaxUserCount(&mut self) -> ::windows::core::Result<i32>;
    fn SetMaxUserCount(&mut self, lnmaxusercount: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsFileService_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsFileService_Impl, const OFFSET: isize>() -> IADsFileService_Vtbl {
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl, Impl: IADsFileService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl, Impl: IADsFileService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&bstrdescription)).into()
        }
        unsafe extern "system" fn MaxUserCount<Identity: ::windows::core::IUnknownImpl, Impl: IADsFileService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MaxUserCount() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxUserCount<Identity: ::windows::core::IUnknownImpl, Impl: IADsFileService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnmaxusercount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMaxUserCount(::core::mem::transmute_copy(&lnmaxusercount)).into()
        }
        Self {
            base: IADsService_Vtbl::new::<Identity, Impl, OFFSET>(),
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            MaxUserCount: MaxUserCount::<Identity, Impl, OFFSET>,
            SetMaxUserCount: SetMaxUserCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsFileService as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IADs as ::windows::core::Interface>::IID || iid == &<IADsService as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsFileServiceOperations_Impl: Sized + super::super::System::Com::IDispatch_Impl + IADs_Impl + IADsServiceOperations_Impl {
    fn Sessions(&mut self) -> ::windows::core::Result<IADsCollection>;
    fn Resources(&mut self) -> ::windows::core::Result<IADsCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsFileServiceOperations_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsFileServiceOperations_Impl, const OFFSET: isize>() -> IADsFileServiceOperations_Vtbl {
        unsafe extern "system" fn Sessions<Identity: ::windows::core::IUnknownImpl, Impl: IADsFileServiceOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsessions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Sessions() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsessions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resources<Identity: ::windows::core::IUnknownImpl, Impl: IADsFileServiceOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Resources() {
                ::core::result::Result::Ok(ok__) => {
                    *ppresources = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IADsServiceOperations_Vtbl::new::<Identity, Impl, OFFSET>(),
            Sessions: Sessions::<Identity, Impl, OFFSET>,
            Resources: Resources::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsFileServiceOperations as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IADs as ::windows::core::Interface>::IID || iid == &<IADsServiceOperations as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsFileShare_Impl: Sized + super::super::System::Com::IDispatch_Impl + IADs_Impl {
    fn CurrentUserCount(&mut self) -> ::windows::core::Result<i32>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, bstrdescription: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn HostComputer(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetHostComputer(&mut self, bstrhostcomputer: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Path(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPath(&mut self, bstrpath: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn MaxUserCount(&mut self) -> ::windows::core::Result<i32>;
    fn SetMaxUserCount(&mut self, lnmaxusercount: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsFileShare_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsFileShare_Impl, const OFFSET: isize>() -> IADsFileShare_Vtbl {
        unsafe extern "system" fn CurrentUserCount<Identity: ::windows::core::IUnknownImpl, Impl: IADsFileShare_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentUserCount() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl, Impl: IADsFileShare_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl, Impl: IADsFileShare_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&bstrdescription)).into()
        }
        unsafe extern "system" fn HostComputer<Identity: ::windows::core::IUnknownImpl, Impl: IADsFileShare_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HostComputer() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHostComputer<Identity: ::windows::core::IUnknownImpl, Impl: IADsFileShare_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrhostcomputer: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHostComputer(::core::mem::transmute_copy(&bstrhostcomputer)).into()
        }
        unsafe extern "system" fn Path<Identity: ::windows::core::IUnknownImpl, Impl: IADsFileShare_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPath<Identity: ::windows::core::IUnknownImpl, Impl: IADsFileShare_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPath(::core::mem::transmute_copy(&bstrpath)).into()
        }
        unsafe extern "system" fn MaxUserCount<Identity: ::windows::core::IUnknownImpl, Impl: IADsFileShare_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MaxUserCount() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxUserCount<Identity: ::windows::core::IUnknownImpl, Impl: IADsFileShare_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnmaxusercount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMaxUserCount(::core::mem::transmute_copy(&lnmaxusercount)).into()
        }
        Self {
            base: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            CurrentUserCount: CurrentUserCount::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            HostComputer: HostComputer::<Identity, Impl, OFFSET>,
            SetHostComputer: SetHostComputer::<Identity, Impl, OFFSET>,
            Path: Path::<Identity, Impl, OFFSET>,
            SetPath: SetPath::<Identity, Impl, OFFSET>,
            MaxUserCount: MaxUserCount::<Identity, Impl, OFFSET>,
            SetMaxUserCount: SetMaxUserCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsFileShare as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IADs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsGroup_Impl: Sized + super::super::System::Com::IDispatch_Impl + IADs_Impl {
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, bstrdescription: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Members(&mut self) -> ::windows::core::Result<IADsMembers>;
    fn IsMember(&mut self, bstrmember: &super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
    fn Add(&mut self, bstrnewitem: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Remove(&mut self, bstritemtoberemoved: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsGroup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsGroup_Impl, const OFFSET: isize>() -> IADsGroup_Vtbl {
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl, Impl: IADsGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl, Impl: IADsGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&bstrdescription)).into()
        }
        unsafe extern "system" fn Members<Identity: ::windows::core::IUnknownImpl, Impl: IADsGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmembers: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Members() {
                ::core::result::Result::Ok(ok__) => {
                    *ppmembers = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMember<Identity: ::windows::core::IUnknownImpl, Impl: IADsGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmember: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bmember: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsMember(::core::mem::transmute_copy(&bstrmember)) {
                ::core::result::Result::Ok(ok__) => {
                    *bmember = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl, Impl: IADsGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrnewitem: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Add(::core::mem::transmute_copy(&bstrnewitem)).into()
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl, Impl: IADsGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemtoberemoved: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&bstritemtoberemoved)).into()
        }
        Self {
            base: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            Members: Members::<Identity, Impl, OFFSET>,
            IsMember: IsMember::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsGroup as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IADs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsHold_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ObjectName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetObjectName(&mut self, bstrobjectname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Amount(&mut self) -> ::windows::core::Result<i32>;
    fn SetAmount(&mut self, lnamount: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsHold_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsHold_Impl, const OFFSET: isize>() -> IADsHold_Vtbl {
        unsafe extern "system" fn ObjectName<Identity: ::windows::core::IUnknownImpl, Impl: IADsHold_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ObjectName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetObjectName<Identity: ::windows::core::IUnknownImpl, Impl: IADsHold_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrobjectname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetObjectName(::core::mem::transmute_copy(&bstrobjectname)).into()
        }
        unsafe extern "system" fn Amount<Identity: ::windows::core::IUnknownImpl, Impl: IADsHold_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Amount() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAmount<Identity: ::windows::core::IUnknownImpl, Impl: IADsHold_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnamount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAmount(::core::mem::transmute_copy(&lnamount)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ObjectName: ObjectName::<Identity, Impl, OFFSET>,
            SetObjectName: SetObjectName::<Identity, Impl, OFFSET>,
            Amount: Amount::<Identity, Impl, OFFSET>,
            SetAmount: SetAmount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsHold as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsLargeInteger_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn HighPart(&mut self) -> ::windows::core::Result<i32>;
    fn SetHighPart(&mut self, lnhighpart: i32) -> ::windows::core::Result<()>;
    fn LowPart(&mut self) -> ::windows::core::Result<i32>;
    fn SetLowPart(&mut self, lnlowpart: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsLargeInteger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsLargeInteger_Impl, const OFFSET: isize>() -> IADsLargeInteger_Vtbl {
        unsafe extern "system" fn HighPart<Identity: ::windows::core::IUnknownImpl, Impl: IADsLargeInteger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HighPart() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHighPart<Identity: ::windows::core::IUnknownImpl, Impl: IADsLargeInteger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnhighpart: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHighPart(::core::mem::transmute_copy(&lnhighpart)).into()
        }
        unsafe extern "system" fn LowPart<Identity: ::windows::core::IUnknownImpl, Impl: IADsLargeInteger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LowPart() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLowPart<Identity: ::windows::core::IUnknownImpl, Impl: IADsLargeInteger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnlowpart: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLowPart(::core::mem::transmute_copy(&lnlowpart)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            HighPart: HighPart::<Identity, Impl, OFFSET>,
            SetHighPart: SetHighPart::<Identity, Impl, OFFSET>,
            LowPart: LowPart::<Identity, Impl, OFFSET>,
            SetLowPart: SetLowPart::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsLargeInteger as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsLocality_Impl: Sized + super::super::System::Com::IDispatch_Impl + IADs_Impl {
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, bstrdescription: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn LocalityName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLocalityName(&mut self, bstrlocalityname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PostalAddress(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPostalAddress(&mut self, bstrpostaladdress: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SeeAlso(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetSeeAlso(&mut self, vseealso: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsLocality_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsLocality_Impl, const OFFSET: isize>() -> IADsLocality_Vtbl {
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl, Impl: IADsLocality_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl, Impl: IADsLocality_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&bstrdescription)).into()
        }
        unsafe extern "system" fn LocalityName<Identity: ::windows::core::IUnknownImpl, Impl: IADsLocality_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LocalityName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalityName<Identity: ::windows::core::IUnknownImpl, Impl: IADsLocality_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlocalityname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLocalityName(::core::mem::transmute_copy(&bstrlocalityname)).into()
        }
        unsafe extern "system" fn PostalAddress<Identity: ::windows::core::IUnknownImpl, Impl: IADsLocality_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PostalAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPostalAddress<Identity: ::windows::core::IUnknownImpl, Impl: IADsLocality_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpostaladdress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPostalAddress(::core::mem::transmute_copy(&bstrpostaladdress)).into()
        }
        unsafe extern "system" fn SeeAlso<Identity: ::windows::core::IUnknownImpl, Impl: IADsLocality_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SeeAlso() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSeeAlso<Identity: ::windows::core::IUnknownImpl, Impl: IADsLocality_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vseealso: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSeeAlso(::core::mem::transmute_copy(&vseealso)).into()
        }
        Self {
            base: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            LocalityName: LocalityName::<Identity, Impl, OFFSET>,
            SetLocalityName: SetLocalityName::<Identity, Impl, OFFSET>,
            PostalAddress: PostalAddress::<Identity, Impl, OFFSET>,
            SetPostalAddress: SetPostalAddress::<Identity, Impl, OFFSET>,
            SeeAlso: SeeAlso::<Identity, Impl, OFFSET>,
            SetSeeAlso: SetSeeAlso::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsLocality as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IADs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsMembers_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Filter(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetFilter(&mut self, pvfilter: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsMembers_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsMembers_Impl, const OFFSET: isize>() -> IADsMembers_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IADsMembers_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IADsMembers_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumerator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Filter<Identity: ::windows::core::IUnknownImpl, Impl: IADsMembers_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvfilter: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Filter() {
                ::core::result::Result::Ok(ok__) => {
                    *pvfilter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFilter<Identity: ::windows::core::IUnknownImpl, Impl: IADsMembers_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvfilter: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFilter(::core::mem::transmute_copy(&pvfilter)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Filter: Filter::<Identity, Impl, OFFSET>,
            SetFilter: SetFilter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsMembers as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsNameTranslate_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SetChaseReferral(&mut self, lnchasereferral: i32) -> ::windows::core::Result<()>;
    fn Init(&mut self, lnsettype: i32, bstradspath: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn InitEx(&mut self, lnsettype: i32, bstradspath: &super::super::Foundation::BSTR, bstruserid: &super::super::Foundation::BSTR, bstrdomain: &super::super::Foundation::BSTR, bstrpassword: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Set(&mut self, lnsettype: i32, bstradspath: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Get(&mut self, lnformattype: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetEx(&mut self, lnformattype: i32, pvar: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GetEx(&mut self, lnformattype: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsNameTranslate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsNameTranslate_Impl, const OFFSET: isize>() -> IADsNameTranslate_Vtbl {
        unsafe extern "system" fn SetChaseReferral<Identity: ::windows::core::IUnknownImpl, Impl: IADsNameTranslate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnchasereferral: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetChaseReferral(::core::mem::transmute_copy(&lnchasereferral)).into()
        }
        unsafe extern "system" fn Init<Identity: ::windows::core::IUnknownImpl, Impl: IADsNameTranslate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnsettype: i32, bstradspath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Init(::core::mem::transmute_copy(&lnsettype), ::core::mem::transmute_copy(&bstradspath)).into()
        }
        unsafe extern "system" fn InitEx<Identity: ::windows::core::IUnknownImpl, Impl: IADsNameTranslate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnsettype: i32, bstradspath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstruserid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdomain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InitEx(::core::mem::transmute_copy(&lnsettype), ::core::mem::transmute_copy(&bstradspath), ::core::mem::transmute_copy(&bstruserid), ::core::mem::transmute_copy(&bstrdomain), ::core::mem::transmute_copy(&bstrpassword)).into()
        }
        unsafe extern "system" fn Set<Identity: ::windows::core::IUnknownImpl, Impl: IADsNameTranslate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnsettype: i32, bstradspath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Set(::core::mem::transmute_copy(&lnsettype), ::core::mem::transmute_copy(&bstradspath)).into()
        }
        unsafe extern "system" fn Get<Identity: ::windows::core::IUnknownImpl, Impl: IADsNameTranslate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnformattype: i32, pbstradspath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Get(::core::mem::transmute_copy(&lnformattype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstradspath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEx<Identity: ::windows::core::IUnknownImpl, Impl: IADsNameTranslate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnformattype: i32, pvar: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEx(::core::mem::transmute_copy(&lnformattype), ::core::mem::transmute_copy(&pvar)).into()
        }
        unsafe extern "system" fn GetEx<Identity: ::windows::core::IUnknownImpl, Impl: IADsNameTranslate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnformattype: i32, pvar: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEx(::core::mem::transmute_copy(&lnformattype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvar = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetChaseReferral: SetChaseReferral::<Identity, Impl, OFFSET>,
            Init: Init::<Identity, Impl, OFFSET>,
            InitEx: InitEx::<Identity, Impl, OFFSET>,
            Set: Set::<Identity, Impl, OFFSET>,
            Get: Get::<Identity, Impl, OFFSET>,
            SetEx: SetEx::<Identity, Impl, OFFSET>,
            GetEx: GetEx::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsNameTranslate as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsNamespaces_Impl: Sized + super::super::System::Com::IDispatch_Impl + IADs_Impl {
    fn DefaultContainer(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDefaultContainer(&mut self, bstrdefaultcontainer: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsNamespaces_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsNamespaces_Impl, const OFFSET: isize>() -> IADsNamespaces_Vtbl {
        unsafe extern "system" fn DefaultContainer<Identity: ::windows::core::IUnknownImpl, Impl: IADsNamespaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DefaultContainer() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultContainer<Identity: ::windows::core::IUnknownImpl, Impl: IADsNamespaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdefaultcontainer: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDefaultContainer(::core::mem::transmute_copy(&bstrdefaultcontainer)).into()
        }
        Self {
            base: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            DefaultContainer: DefaultContainer::<Identity, Impl, OFFSET>,
            SetDefaultContainer: SetDefaultContainer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsNamespaces as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IADs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsNetAddress_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn AddressType(&mut self) -> ::windows::core::Result<i32>;
    fn SetAddressType(&mut self, lnaddresstype: i32) -> ::windows::core::Result<()>;
    fn Address(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetAddress(&mut self, vaddress: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsNetAddress_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsNetAddress_Impl, const OFFSET: isize>() -> IADsNetAddress_Vtbl {
        unsafe extern "system" fn AddressType<Identity: ::windows::core::IUnknownImpl, Impl: IADsNetAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AddressType() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAddressType<Identity: ::windows::core::IUnknownImpl, Impl: IADsNetAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnaddresstype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAddressType(::core::mem::transmute_copy(&lnaddresstype)).into()
        }
        unsafe extern "system" fn Address<Identity: ::windows::core::IUnknownImpl, Impl: IADsNetAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Address() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAddress<Identity: ::windows::core::IUnknownImpl, Impl: IADsNetAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vaddress: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAddress(::core::mem::transmute_copy(&vaddress)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            AddressType: AddressType::<Identity, Impl, OFFSET>,
            SetAddressType: SetAddressType::<Identity, Impl, OFFSET>,
            Address: Address::<Identity, Impl, OFFSET>,
            SetAddress: SetAddress::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsNetAddress as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsO_Impl: Sized + super::super::System::Com::IDispatch_Impl + IADs_Impl {
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, bstrdescription: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn LocalityName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLocalityName(&mut self, bstrlocalityname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PostalAddress(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPostalAddress(&mut self, bstrpostaladdress: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn TelephoneNumber(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetTelephoneNumber(&mut self, bstrtelephonenumber: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FaxNumber(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetFaxNumber(&mut self, bstrfaxnumber: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SeeAlso(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetSeeAlso(&mut self, vseealso: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsO_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsO_Impl, const OFFSET: isize>() -> IADsO_Vtbl {
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl, Impl: IADsO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl, Impl: IADsO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&bstrdescription)).into()
        }
        unsafe extern "system" fn LocalityName<Identity: ::windows::core::IUnknownImpl, Impl: IADsO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LocalityName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalityName<Identity: ::windows::core::IUnknownImpl, Impl: IADsO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlocalityname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLocalityName(::core::mem::transmute_copy(&bstrlocalityname)).into()
        }
        unsafe extern "system" fn PostalAddress<Identity: ::windows::core::IUnknownImpl, Impl: IADsO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PostalAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPostalAddress<Identity: ::windows::core::IUnknownImpl, Impl: IADsO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpostaladdress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPostalAddress(::core::mem::transmute_copy(&bstrpostaladdress)).into()
        }
        unsafe extern "system" fn TelephoneNumber<Identity: ::windows::core::IUnknownImpl, Impl: IADsO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TelephoneNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTelephoneNumber<Identity: ::windows::core::IUnknownImpl, Impl: IADsO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtelephonenumber: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTelephoneNumber(::core::mem::transmute_copy(&bstrtelephonenumber)).into()
        }
        unsafe extern "system" fn FaxNumber<Identity: ::windows::core::IUnknownImpl, Impl: IADsO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FaxNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFaxNumber<Identity: ::windows::core::IUnknownImpl, Impl: IADsO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfaxnumber: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFaxNumber(::core::mem::transmute_copy(&bstrfaxnumber)).into()
        }
        unsafe extern "system" fn SeeAlso<Identity: ::windows::core::IUnknownImpl, Impl: IADsO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SeeAlso() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSeeAlso<Identity: ::windows::core::IUnknownImpl, Impl: IADsO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vseealso: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSeeAlso(::core::mem::transmute_copy(&vseealso)).into()
        }
        Self {
            base: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            LocalityName: LocalityName::<Identity, Impl, OFFSET>,
            SetLocalityName: SetLocalityName::<Identity, Impl, OFFSET>,
            PostalAddress: PostalAddress::<Identity, Impl, OFFSET>,
            SetPostalAddress: SetPostalAddress::<Identity, Impl, OFFSET>,
            TelephoneNumber: TelephoneNumber::<Identity, Impl, OFFSET>,
            SetTelephoneNumber: SetTelephoneNumber::<Identity, Impl, OFFSET>,
            FaxNumber: FaxNumber::<Identity, Impl, OFFSET>,
            SetFaxNumber: SetFaxNumber::<Identity, Impl, OFFSET>,
            SeeAlso: SeeAlso::<Identity, Impl, OFFSET>,
            SetSeeAlso: SetSeeAlso::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsO as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IADs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsOU_Impl: Sized + super::super::System::Com::IDispatch_Impl + IADs_Impl {
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, bstrdescription: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn LocalityName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLocalityName(&mut self, bstrlocalityname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PostalAddress(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPostalAddress(&mut self, bstrpostaladdress: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn TelephoneNumber(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetTelephoneNumber(&mut self, bstrtelephonenumber: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FaxNumber(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetFaxNumber(&mut self, bstrfaxnumber: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SeeAlso(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetSeeAlso(&mut self, vseealso: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn BusinessCategory(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetBusinessCategory(&mut self, bstrbusinesscategory: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsOU_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsOU_Impl, const OFFSET: isize>() -> IADsOU_Vtbl {
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl, Impl: IADsOU_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl, Impl: IADsOU_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&bstrdescription)).into()
        }
        unsafe extern "system" fn LocalityName<Identity: ::windows::core::IUnknownImpl, Impl: IADsOU_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LocalityName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalityName<Identity: ::windows::core::IUnknownImpl, Impl: IADsOU_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlocalityname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLocalityName(::core::mem::transmute_copy(&bstrlocalityname)).into()
        }
        unsafe extern "system" fn PostalAddress<Identity: ::windows::core::IUnknownImpl, Impl: IADsOU_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PostalAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPostalAddress<Identity: ::windows::core::IUnknownImpl, Impl: IADsOU_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpostaladdress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPostalAddress(::core::mem::transmute_copy(&bstrpostaladdress)).into()
        }
        unsafe extern "system" fn TelephoneNumber<Identity: ::windows::core::IUnknownImpl, Impl: IADsOU_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TelephoneNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTelephoneNumber<Identity: ::windows::core::IUnknownImpl, Impl: IADsOU_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtelephonenumber: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTelephoneNumber(::core::mem::transmute_copy(&bstrtelephonenumber)).into()
        }
        unsafe extern "system" fn FaxNumber<Identity: ::windows::core::IUnknownImpl, Impl: IADsOU_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FaxNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFaxNumber<Identity: ::windows::core::IUnknownImpl, Impl: IADsOU_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfaxnumber: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFaxNumber(::core::mem::transmute_copy(&bstrfaxnumber)).into()
        }
        unsafe extern "system" fn SeeAlso<Identity: ::windows::core::IUnknownImpl, Impl: IADsOU_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SeeAlso() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSeeAlso<Identity: ::windows::core::IUnknownImpl, Impl: IADsOU_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vseealso: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSeeAlso(::core::mem::transmute_copy(&vseealso)).into()
        }
        unsafe extern "system" fn BusinessCategory<Identity: ::windows::core::IUnknownImpl, Impl: IADsOU_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BusinessCategory() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBusinessCategory<Identity: ::windows::core::IUnknownImpl, Impl: IADsOU_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbusinesscategory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBusinessCategory(::core::mem::transmute_copy(&bstrbusinesscategory)).into()
        }
        Self {
            base: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            LocalityName: LocalityName::<Identity, Impl, OFFSET>,
            SetLocalityName: SetLocalityName::<Identity, Impl, OFFSET>,
            PostalAddress: PostalAddress::<Identity, Impl, OFFSET>,
            SetPostalAddress: SetPostalAddress::<Identity, Impl, OFFSET>,
            TelephoneNumber: TelephoneNumber::<Identity, Impl, OFFSET>,
            SetTelephoneNumber: SetTelephoneNumber::<Identity, Impl, OFFSET>,
            FaxNumber: FaxNumber::<Identity, Impl, OFFSET>,
            SetFaxNumber: SetFaxNumber::<Identity, Impl, OFFSET>,
            SeeAlso: SeeAlso::<Identity, Impl, OFFSET>,
            SetSeeAlso: SetSeeAlso::<Identity, Impl, OFFSET>,
            BusinessCategory: BusinessCategory::<Identity, Impl, OFFSET>,
            SetBusinessCategory: SetBusinessCategory::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsOU as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IADs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsObjectOptions_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetOption(&mut self, lnoption: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetOption(&mut self, lnoption: i32, vvalue: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsObjectOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsObjectOptions_Impl, const OFFSET: isize>() -> IADsObjectOptions_Vtbl {
        unsafe extern "system" fn GetOption<Identity: ::windows::core::IUnknownImpl, Impl: IADsObjectOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnoption: i32, pvvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOption(::core::mem::transmute_copy(&lnoption)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOption<Identity: ::windows::core::IUnknownImpl, Impl: IADsObjectOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnoption: i32, vvalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOption(::core::mem::transmute_copy(&lnoption), ::core::mem::transmute_copy(&vvalue)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetOption: GetOption::<Identity, Impl, OFFSET>,
            SetOption: SetOption::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsObjectOptions as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsOctetList_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn OctetList(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetOctetList(&mut self, voctetlist: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsOctetList_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsOctetList_Impl, const OFFSET: isize>() -> IADsOctetList_Vtbl {
        unsafe extern "system" fn OctetList<Identity: ::windows::core::IUnknownImpl, Impl: IADsOctetList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OctetList() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOctetList<Identity: ::windows::core::IUnknownImpl, Impl: IADsOctetList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, voctetlist: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOctetList(::core::mem::transmute_copy(&voctetlist)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            OctetList: OctetList::<Identity, Impl, OFFSET>,
            SetOctetList: SetOctetList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsOctetList as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsOpenDSObject_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn OpenDSObject(&mut self, lpszdnname: &super::super::Foundation::BSTR, lpszusername: &super::super::Foundation::BSTR, lpszpassword: &super::super::Foundation::BSTR, lnreserved: i32) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsOpenDSObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsOpenDSObject_Impl, const OFFSET: isize>() -> IADsOpenDSObject_Vtbl {
        unsafe extern "system" fn OpenDSObject<Identity: ::windows::core::IUnknownImpl, Impl: IADsOpenDSObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszdnname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lpszusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lpszpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lnreserved: i32, ppoledsobj: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OpenDSObject(::core::mem::transmute_copy(&lpszdnname), ::core::mem::transmute_copy(&lpszusername), ::core::mem::transmute_copy(&lpszpassword), ::core::mem::transmute_copy(&lnreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppoledsobj = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), OpenDSObject: OpenDSObject::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsOpenDSObject as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsPath_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Type(&mut self) -> ::windows::core::Result<i32>;
    fn SetType(&mut self, lntype: i32) -> ::windows::core::Result<()>;
    fn VolumeName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetVolumeName(&mut self, bstrvolumename: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Path(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPath(&mut self, bstrpath: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsPath_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsPath_Impl, const OFFSET: isize>() -> IADsPath_Vtbl {
        unsafe extern "system" fn Type<Identity: ::windows::core::IUnknownImpl, Impl: IADsPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Identity: ::windows::core::IUnknownImpl, Impl: IADsPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lntype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetType(::core::mem::transmute_copy(&lntype)).into()
        }
        unsafe extern "system" fn VolumeName<Identity: ::windows::core::IUnknownImpl, Impl: IADsPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).VolumeName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVolumeName<Identity: ::windows::core::IUnknownImpl, Impl: IADsPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrvolumename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetVolumeName(::core::mem::transmute_copy(&bstrvolumename)).into()
        }
        unsafe extern "system" fn Path<Identity: ::windows::core::IUnknownImpl, Impl: IADsPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPath<Identity: ::windows::core::IUnknownImpl, Impl: IADsPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPath(::core::mem::transmute_copy(&bstrpath)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Type: Type::<Identity, Impl, OFFSET>,
            SetType: SetType::<Identity, Impl, OFFSET>,
            VolumeName: VolumeName::<Identity, Impl, OFFSET>,
            SetVolumeName: SetVolumeName::<Identity, Impl, OFFSET>,
            Path: Path::<Identity, Impl, OFFSET>,
            SetPath: SetPath::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsPath as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsPathname_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Set(&mut self, bstradspath: &super::super::Foundation::BSTR, lnsettype: i32) -> ::windows::core::Result<()>;
    fn SetDisplayType(&mut self, lndisplaytype: i32) -> ::windows::core::Result<()>;
    fn Retrieve(&mut self, lnformattype: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetNumElements(&mut self) -> ::windows::core::Result<i32>;
    fn GetElement(&mut self, lnelementindex: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn AddLeafElement(&mut self, bstrleafelement: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RemoveLeafElement(&mut self) -> ::windows::core::Result<()>;
    fn CopyPath(&mut self) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn GetEscapedElement(&mut self, lnreserved: i32, bstrinstr: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn EscapedMode(&mut self) -> ::windows::core::Result<i32>;
    fn SetEscapedMode(&mut self, lnescapedmode: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsPathname_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsPathname_Impl, const OFFSET: isize>() -> IADsPathname_Vtbl {
        unsafe extern "system" fn Set<Identity: ::windows::core::IUnknownImpl, Impl: IADsPathname_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradspath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lnsettype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Set(::core::mem::transmute_copy(&bstradspath), ::core::mem::transmute_copy(&lnsettype)).into()
        }
        unsafe extern "system" fn SetDisplayType<Identity: ::windows::core::IUnknownImpl, Impl: IADsPathname_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lndisplaytype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDisplayType(::core::mem::transmute_copy(&lndisplaytype)).into()
        }
        unsafe extern "system" fn Retrieve<Identity: ::windows::core::IUnknownImpl, Impl: IADsPathname_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnformattype: i32, pbstradspath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Retrieve(::core::mem::transmute_copy(&lnformattype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstradspath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumElements<Identity: ::windows::core::IUnknownImpl, Impl: IADsPathname_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plnnumpathelements: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetNumElements() {
                ::core::result::Result::Ok(ok__) => {
                    *plnnumpathelements = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetElement<Identity: ::windows::core::IUnknownImpl, Impl: IADsPathname_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnelementindex: i32, pbstrelement: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetElement(::core::mem::transmute_copy(&lnelementindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrelement = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddLeafElement<Identity: ::windows::core::IUnknownImpl, Impl: IADsPathname_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrleafelement: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddLeafElement(::core::mem::transmute_copy(&bstrleafelement)).into()
        }
        unsafe extern "system" fn RemoveLeafElement<Identity: ::windows::core::IUnknownImpl, Impl: IADsPathname_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveLeafElement().into()
        }
        unsafe extern "system" fn CopyPath<Identity: ::windows::core::IUnknownImpl, Impl: IADsPathname_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppadspath: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CopyPath() {
                ::core::result::Result::Ok(ok__) => {
                    *ppadspath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEscapedElement<Identity: ::windows::core::IUnknownImpl, Impl: IADsPathname_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnreserved: i32, bstrinstr: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstroutstr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEscapedElement(::core::mem::transmute_copy(&lnreserved), ::core::mem::transmute_copy(&bstrinstr)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstroutstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EscapedMode<Identity: ::windows::core::IUnknownImpl, Impl: IADsPathname_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EscapedMode() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEscapedMode<Identity: ::windows::core::IUnknownImpl, Impl: IADsPathname_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnescapedmode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEscapedMode(::core::mem::transmute_copy(&lnescapedmode)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Set: Set::<Identity, Impl, OFFSET>,
            SetDisplayType: SetDisplayType::<Identity, Impl, OFFSET>,
            Retrieve: Retrieve::<Identity, Impl, OFFSET>,
            GetNumElements: GetNumElements::<Identity, Impl, OFFSET>,
            GetElement: GetElement::<Identity, Impl, OFFSET>,
            AddLeafElement: AddLeafElement::<Identity, Impl, OFFSET>,
            RemoveLeafElement: RemoveLeafElement::<Identity, Impl, OFFSET>,
            CopyPath: CopyPath::<Identity, Impl, OFFSET>,
            GetEscapedElement: GetEscapedElement::<Identity, Impl, OFFSET>,
            EscapedMode: EscapedMode::<Identity, Impl, OFFSET>,
            SetEscapedMode: SetEscapedMode::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsPathname as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsPostalAddress_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn PostalAddress(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetPostalAddress(&mut self, vpostaladdress: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsPostalAddress_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsPostalAddress_Impl, const OFFSET: isize>() -> IADsPostalAddress_Vtbl {
        unsafe extern "system" fn PostalAddress<Identity: ::windows::core::IUnknownImpl, Impl: IADsPostalAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PostalAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPostalAddress<Identity: ::windows::core::IUnknownImpl, Impl: IADsPostalAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vpostaladdress: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPostalAddress(::core::mem::transmute_copy(&vpostaladdress)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            PostalAddress: PostalAddress::<Identity, Impl, OFFSET>,
            SetPostalAddress: SetPostalAddress::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsPostalAddress as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsPrintJob_Impl: Sized + super::super::System::Com::IDispatch_Impl + IADs_Impl {
    fn HostPrintQueue(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn User(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn UserPath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn TimeSubmitted(&mut self) -> ::windows::core::Result<f64>;
    fn TotalPages(&mut self) -> ::windows::core::Result<i32>;
    fn Size(&mut self) -> ::windows::core::Result<i32>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, bstrdescription: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Priority(&mut self) -> ::windows::core::Result<i32>;
    fn SetPriority(&mut self, lnpriority: i32) -> ::windows::core::Result<()>;
    fn StartTime(&mut self) -> ::windows::core::Result<f64>;
    fn SetStartTime(&mut self, dastarttime: f64) -> ::windows::core::Result<()>;
    fn UntilTime(&mut self) -> ::windows::core::Result<f64>;
    fn SetUntilTime(&mut self, dauntiltime: f64) -> ::windows::core::Result<()>;
    fn Notify(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetNotify(&mut self, bstrnotify: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn NotifyPath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetNotifyPath(&mut self, bstrnotifypath: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsPrintJob_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintJob_Impl, const OFFSET: isize>() -> IADsPrintJob_Vtbl {
        unsafe extern "system" fn HostPrintQueue<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HostPrintQueue() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn User<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).User() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserPath<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UserPath() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TimeSubmitted<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TimeSubmitted() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalPages<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TotalPages() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Size() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&bstrdescription)).into()
        }
        unsafe extern "system" fn Priority<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Priority() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnpriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPriority(::core::mem::transmute_copy(&lnpriority)).into()
        }
        unsafe extern "system" fn StartTime<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartTime<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dastarttime: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStartTime(::core::mem::transmute_copy(&dastarttime)).into()
        }
        unsafe extern "system" fn UntilTime<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UntilTime() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUntilTime<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dauntiltime: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUntilTime(::core::mem::transmute_copy(&dauntiltime)).into()
        }
        unsafe extern "system" fn Notify<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Notify() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotify<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrnotify: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNotify(::core::mem::transmute_copy(&bstrnotify)).into()
        }
        unsafe extern "system" fn NotifyPath<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NotifyPath() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotifyPath<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrnotifypath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNotifyPath(::core::mem::transmute_copy(&bstrnotifypath)).into()
        }
        Self {
            base: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            HostPrintQueue: HostPrintQueue::<Identity, Impl, OFFSET>,
            User: User::<Identity, Impl, OFFSET>,
            UserPath: UserPath::<Identity, Impl, OFFSET>,
            TimeSubmitted: TimeSubmitted::<Identity, Impl, OFFSET>,
            TotalPages: TotalPages::<Identity, Impl, OFFSET>,
            Size: Size::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            Priority: Priority::<Identity, Impl, OFFSET>,
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
            StartTime: StartTime::<Identity, Impl, OFFSET>,
            SetStartTime: SetStartTime::<Identity, Impl, OFFSET>,
            UntilTime: UntilTime::<Identity, Impl, OFFSET>,
            SetUntilTime: SetUntilTime::<Identity, Impl, OFFSET>,
            Notify: Notify::<Identity, Impl, OFFSET>,
            SetNotify: SetNotify::<Identity, Impl, OFFSET>,
            NotifyPath: NotifyPath::<Identity, Impl, OFFSET>,
            SetNotifyPath: SetNotifyPath::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsPrintJob as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IADs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsPrintJobOperations_Impl: Sized + super::super::System::Com::IDispatch_Impl + IADs_Impl {
    fn Status(&mut self) -> ::windows::core::Result<i32>;
    fn TimeElapsed(&mut self) -> ::windows::core::Result<i32>;
    fn PagesPrinted(&mut self) -> ::windows::core::Result<i32>;
    fn Position(&mut self) -> ::windows::core::Result<i32>;
    fn SetPosition(&mut self, lnposition: i32) -> ::windows::core::Result<()>;
    fn Pause(&mut self) -> ::windows::core::Result<()>;
    fn Resume(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsPrintJobOperations_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintJobOperations_Impl, const OFFSET: isize>() -> IADsPrintJobOperations_Vtbl {
        unsafe extern "system" fn Status<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintJobOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TimeElapsed<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintJobOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TimeElapsed() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PagesPrinted<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintJobOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PagesPrinted() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintJobOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPosition<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintJobOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnposition: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPosition(::core::mem::transmute_copy(&lnposition)).into()
        }
        unsafe extern "system" fn Pause<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintJobOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn Resume<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintJobOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Resume().into()
        }
        Self {
            base: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            Status: Status::<Identity, Impl, OFFSET>,
            TimeElapsed: TimeElapsed::<Identity, Impl, OFFSET>,
            PagesPrinted: PagesPrinted::<Identity, Impl, OFFSET>,
            Position: Position::<Identity, Impl, OFFSET>,
            SetPosition: SetPosition::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsPrintJobOperations as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IADs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsPrintQueue_Impl: Sized + super::super::System::Com::IDispatch_Impl + IADs_Impl {
    fn PrinterPath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPrinterPath(&mut self, bstrprinterpath: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Model(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetModel(&mut self, bstrmodel: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Datatype(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDatatype(&mut self, bstrdatatype: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PrintProcessor(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPrintProcessor(&mut self, bstrprintprocessor: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, bstrdescription: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Location(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLocation(&mut self, bstrlocation: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn StartTime(&mut self) -> ::windows::core::Result<f64>;
    fn SetStartTime(&mut self, dastarttime: f64) -> ::windows::core::Result<()>;
    fn UntilTime(&mut self) -> ::windows::core::Result<f64>;
    fn SetUntilTime(&mut self, dauntiltime: f64) -> ::windows::core::Result<()>;
    fn DefaultJobPriority(&mut self) -> ::windows::core::Result<i32>;
    fn SetDefaultJobPriority(&mut self, lndefaultjobpriority: i32) -> ::windows::core::Result<()>;
    fn Priority(&mut self) -> ::windows::core::Result<i32>;
    fn SetPriority(&mut self, lnpriority: i32) -> ::windows::core::Result<()>;
    fn BannerPage(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetBannerPage(&mut self, bstrbannerpage: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PrintDevices(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetPrintDevices(&mut self, vprintdevices: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn NetAddresses(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetNetAddresses(&mut self, vnetaddresses: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsPrintQueue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintQueue_Impl, const OFFSET: isize>() -> IADsPrintQueue_Vtbl {
        unsafe extern "system" fn PrinterPath<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PrinterPath() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrinterPath<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprinterpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPrinterPath(::core::mem::transmute_copy(&bstrprinterpath)).into()
        }
        unsafe extern "system" fn Model<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Model() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetModel<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmodel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetModel(::core::mem::transmute_copy(&bstrmodel)).into()
        }
        unsafe extern "system" fn Datatype<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Datatype() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDatatype<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdatatype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDatatype(::core::mem::transmute_copy(&bstrdatatype)).into()
        }
        unsafe extern "system" fn PrintProcessor<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PrintProcessor() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrintProcessor<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprintprocessor: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPrintProcessor(::core::mem::transmute_copy(&bstrprintprocessor)).into()
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&bstrdescription)).into()
        }
        unsafe extern "system" fn Location<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Location() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocation<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlocation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLocation(::core::mem::transmute_copy(&bstrlocation)).into()
        }
        unsafe extern "system" fn StartTime<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartTime<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dastarttime: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStartTime(::core::mem::transmute_copy(&dastarttime)).into()
        }
        unsafe extern "system" fn UntilTime<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UntilTime() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUntilTime<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dauntiltime: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUntilTime(::core::mem::transmute_copy(&dauntiltime)).into()
        }
        unsafe extern "system" fn DefaultJobPriority<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DefaultJobPriority() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultJobPriority<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lndefaultjobpriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDefaultJobPriority(::core::mem::transmute_copy(&lndefaultjobpriority)).into()
        }
        unsafe extern "system" fn Priority<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Priority() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnpriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPriority(::core::mem::transmute_copy(&lnpriority)).into()
        }
        unsafe extern "system" fn BannerPage<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BannerPage() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBannerPage<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbannerpage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBannerPage(::core::mem::transmute_copy(&bstrbannerpage)).into()
        }
        unsafe extern "system" fn PrintDevices<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PrintDevices() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrintDevices<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vprintdevices: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPrintDevices(::core::mem::transmute_copy(&vprintdevices)).into()
        }
        unsafe extern "system" fn NetAddresses<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NetAddresses() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetAddresses<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vnetaddresses: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNetAddresses(::core::mem::transmute_copy(&vnetaddresses)).into()
        }
        Self {
            base: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            PrinterPath: PrinterPath::<Identity, Impl, OFFSET>,
            SetPrinterPath: SetPrinterPath::<Identity, Impl, OFFSET>,
            Model: Model::<Identity, Impl, OFFSET>,
            SetModel: SetModel::<Identity, Impl, OFFSET>,
            Datatype: Datatype::<Identity, Impl, OFFSET>,
            SetDatatype: SetDatatype::<Identity, Impl, OFFSET>,
            PrintProcessor: PrintProcessor::<Identity, Impl, OFFSET>,
            SetPrintProcessor: SetPrintProcessor::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            Location: Location::<Identity, Impl, OFFSET>,
            SetLocation: SetLocation::<Identity, Impl, OFFSET>,
            StartTime: StartTime::<Identity, Impl, OFFSET>,
            SetStartTime: SetStartTime::<Identity, Impl, OFFSET>,
            UntilTime: UntilTime::<Identity, Impl, OFFSET>,
            SetUntilTime: SetUntilTime::<Identity, Impl, OFFSET>,
            DefaultJobPriority: DefaultJobPriority::<Identity, Impl, OFFSET>,
            SetDefaultJobPriority: SetDefaultJobPriority::<Identity, Impl, OFFSET>,
            Priority: Priority::<Identity, Impl, OFFSET>,
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
            BannerPage: BannerPage::<Identity, Impl, OFFSET>,
            SetBannerPage: SetBannerPage::<Identity, Impl, OFFSET>,
            PrintDevices: PrintDevices::<Identity, Impl, OFFSET>,
            SetPrintDevices: SetPrintDevices::<Identity, Impl, OFFSET>,
            NetAddresses: NetAddresses::<Identity, Impl, OFFSET>,
            SetNetAddresses: SetNetAddresses::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsPrintQueue as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IADs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsPrintQueueOperations_Impl: Sized + super::super::System::Com::IDispatch_Impl + IADs_Impl {
    fn Status(&mut self) -> ::windows::core::Result<i32>;
    fn PrintJobs(&mut self) -> ::windows::core::Result<IADsCollection>;
    fn Pause(&mut self) -> ::windows::core::Result<()>;
    fn Resume(&mut self) -> ::windows::core::Result<()>;
    fn Purge(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsPrintQueueOperations_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintQueueOperations_Impl, const OFFSET: isize>() -> IADsPrintQueueOperations_Vtbl {
        unsafe extern "system" fn Status<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintQueueOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrintJobs<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintQueueOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PrintJobs() {
                ::core::result::Result::Ok(ok__) => {
                    *pobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pause<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintQueueOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn Resume<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintQueueOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Resume().into()
        }
        unsafe extern "system" fn Purge<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintQueueOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Purge().into()
        }
        Self {
            base: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            Status: Status::<Identity, Impl, OFFSET>,
            PrintJobs: PrintJobs::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
            Purge: Purge::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsPrintQueueOperations as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IADs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsProperty_Impl: Sized + super::super::System::Com::IDispatch_Impl + IADs_Impl {
    fn OID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetOID(&mut self, bstroid: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Syntax(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetSyntax(&mut self, bstrsyntax: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn MaxRange(&mut self) -> ::windows::core::Result<i32>;
    fn SetMaxRange(&mut self, lnmaxrange: i32) -> ::windows::core::Result<()>;
    fn MinRange(&mut self) -> ::windows::core::Result<i32>;
    fn SetMinRange(&mut self, lnminrange: i32) -> ::windows::core::Result<()>;
    fn MultiValued(&mut self) -> ::windows::core::Result<i16>;
    fn SetMultiValued(&mut self, fmultivalued: i16) -> ::windows::core::Result<()>;
    fn Qualifiers(&mut self) -> ::windows::core::Result<IADsCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsProperty_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsProperty_Impl, const OFFSET: isize>() -> IADsProperty_Vtbl {
        unsafe extern "system" fn OID<Identity: ::windows::core::IUnknownImpl, Impl: IADsProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OID() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOID<Identity: ::windows::core::IUnknownImpl, Impl: IADsProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstroid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOID(::core::mem::transmute_copy(&bstroid)).into()
        }
        unsafe extern "system" fn Syntax<Identity: ::windows::core::IUnknownImpl, Impl: IADsProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Syntax() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSyntax<Identity: ::windows::core::IUnknownImpl, Impl: IADsProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsyntax: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSyntax(::core::mem::transmute_copy(&bstrsyntax)).into()
        }
        unsafe extern "system" fn MaxRange<Identity: ::windows::core::IUnknownImpl, Impl: IADsProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MaxRange() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxRange<Identity: ::windows::core::IUnknownImpl, Impl: IADsProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnmaxrange: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMaxRange(::core::mem::transmute_copy(&lnmaxrange)).into()
        }
        unsafe extern "system" fn MinRange<Identity: ::windows::core::IUnknownImpl, Impl: IADsProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MinRange() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinRange<Identity: ::windows::core::IUnknownImpl, Impl: IADsProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnminrange: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMinRange(::core::mem::transmute_copy(&lnminrange)).into()
        }
        unsafe extern "system" fn MultiValued<Identity: ::windows::core::IUnknownImpl, Impl: IADsProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MultiValued() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMultiValued<Identity: ::windows::core::IUnknownImpl, Impl: IADsProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmultivalued: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMultiValued(::core::mem::transmute_copy(&fmultivalued)).into()
        }
        unsafe extern "system" fn Qualifiers<Identity: ::windows::core::IUnknownImpl, Impl: IADsProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqualifiers: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Qualifiers() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqualifiers = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            OID: OID::<Identity, Impl, OFFSET>,
            SetOID: SetOID::<Identity, Impl, OFFSET>,
            Syntax: Syntax::<Identity, Impl, OFFSET>,
            SetSyntax: SetSyntax::<Identity, Impl, OFFSET>,
            MaxRange: MaxRange::<Identity, Impl, OFFSET>,
            SetMaxRange: SetMaxRange::<Identity, Impl, OFFSET>,
            MinRange: MinRange::<Identity, Impl, OFFSET>,
            SetMinRange: SetMinRange::<Identity, Impl, OFFSET>,
            MultiValued: MultiValued::<Identity, Impl, OFFSET>,
            SetMultiValued: SetMultiValued::<Identity, Impl, OFFSET>,
            Qualifiers: Qualifiers::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsProperty as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IADs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsPropertyEntry_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Clear(&mut self) -> ::windows::core::Result<()>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&mut self, bstrname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ADsType(&mut self) -> ::windows::core::Result<i32>;
    fn SetADsType(&mut self, lnadstype: i32) -> ::windows::core::Result<()>;
    fn ControlCode(&mut self) -> ::windows::core::Result<i32>;
    fn SetControlCode(&mut self, lncontrolcode: i32) -> ::windows::core::Result<()>;
    fn Values(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetValues(&mut self, vvalues: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsPropertyEntry_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyEntry_Impl, const OFFSET: isize>() -> IADsPropertyEntry_Vtbl {
        unsafe extern "system" fn Clear<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&bstrname)).into()
        }
        unsafe extern "system" fn ADsType<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ADsType() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetADsType<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnadstype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetADsType(::core::mem::transmute_copy(&lnadstype)).into()
        }
        unsafe extern "system" fn ControlCode<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ControlCode() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetControlCode<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lncontrolcode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetControlCode(::core::mem::transmute_copy(&lncontrolcode)).into()
        }
        unsafe extern "system" fn Values<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Values() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValues<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vvalues: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetValues(::core::mem::transmute_copy(&vvalues)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Clear: Clear::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            ADsType: ADsType::<Identity, Impl, OFFSET>,
            SetADsType: SetADsType::<Identity, Impl, OFFSET>,
            ControlCode: ControlCode::<Identity, Impl, OFFSET>,
            SetControlCode: SetControlCode::<Identity, Impl, OFFSET>,
            Values: Values::<Identity, Impl, OFFSET>,
            SetValues: SetValues::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsPropertyEntry as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsPropertyList_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn PropertyCount(&mut self) -> ::windows::core::Result<i32>;
    fn Next(&mut self, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT;
    fn Skip(&mut self, celements: i32) -> ::windows::core::HRESULT;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Item(&mut self, varindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn GetPropertyItem(&mut self, bstrname: &super::super::Foundation::BSTR, lnadstype: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn PutPropertyItem(&mut self, vardata: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn ResetPropertyItem(&mut self, varentry: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn PurgePropertyList(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsPropertyList_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyList_Impl, const OFFSET: isize>() -> IADsPropertyList_Vtbl {
        unsafe extern "system" fn PropertyCount<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PropertyCount() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&pvariant))
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celements: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celements))
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyItem<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lnadstype: i32, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPropertyItem(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&lnadstype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutPropertyItem<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vardata: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PutPropertyItem(::core::mem::transmute_copy(&vardata)).into()
        }
        unsafe extern "system" fn ResetPropertyItem<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varentry: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ResetPropertyItem(::core::mem::transmute_copy(&varentry)).into()
        }
        unsafe extern "system" fn PurgePropertyList<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PurgePropertyList().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            PropertyCount: PropertyCount::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            GetPropertyItem: GetPropertyItem::<Identity, Impl, OFFSET>,
            PutPropertyItem: PutPropertyItem::<Identity, Impl, OFFSET>,
            ResetPropertyItem: ResetPropertyItem::<Identity, Impl, OFFSET>,
            PurgePropertyList: PurgePropertyList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsPropertyList as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsPropertyValue_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Clear(&mut self) -> ::windows::core::Result<()>;
    fn ADsType(&mut self) -> ::windows::core::Result<i32>;
    fn SetADsType(&mut self, lnadstype: i32) -> ::windows::core::Result<()>;
    fn DNString(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDNString(&mut self, bstrdnstring: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn CaseExactString(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetCaseExactString(&mut self, bstrcaseexactstring: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn CaseIgnoreString(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetCaseIgnoreString(&mut self, bstrcaseignorestring: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PrintableString(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPrintableString(&mut self, bstrprintablestring: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn NumericString(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetNumericString(&mut self, bstrnumericstring: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Boolean(&mut self) -> ::windows::core::Result<i32>;
    fn SetBoolean(&mut self, lnboolean: i32) -> ::windows::core::Result<()>;
    fn Integer(&mut self) -> ::windows::core::Result<i32>;
    fn SetInteger(&mut self, lninteger: i32) -> ::windows::core::Result<()>;
    fn OctetString(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetOctetString(&mut self, voctetstring: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SecurityDescriptor(&mut self) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn SetSecurityDescriptor(&mut self, psecuritydescriptor: &::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn LargeInteger(&mut self) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn SetLargeInteger(&mut self, plargeinteger: &::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn UTCTime(&mut self) -> ::windows::core::Result<f64>;
    fn SetUTCTime(&mut self, dautctime: f64) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsPropertyValue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyValue_Impl, const OFFSET: isize>() -> IADsPropertyValue_Vtbl {
        unsafe extern "system" fn Clear<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn ADsType<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ADsType() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetADsType<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnadstype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetADsType(::core::mem::transmute_copy(&lnadstype)).into()
        }
        unsafe extern "system" fn DNString<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DNString() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDNString<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdnstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDNString(::core::mem::transmute_copy(&bstrdnstring)).into()
        }
        unsafe extern "system" fn CaseExactString<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CaseExactString() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCaseExactString<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcaseexactstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCaseExactString(::core::mem::transmute_copy(&bstrcaseexactstring)).into()
        }
        unsafe extern "system" fn CaseIgnoreString<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CaseIgnoreString() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCaseIgnoreString<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcaseignorestring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCaseIgnoreString(::core::mem::transmute_copy(&bstrcaseignorestring)).into()
        }
        unsafe extern "system" fn PrintableString<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PrintableString() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrintableString<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprintablestring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPrintableString(::core::mem::transmute_copy(&bstrprintablestring)).into()
        }
        unsafe extern "system" fn NumericString<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NumericString() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNumericString<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrnumericstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNumericString(::core::mem::transmute_copy(&bstrnumericstring)).into()
        }
        unsafe extern "system" fn Boolean<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Boolean() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBoolean<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnboolean: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBoolean(::core::mem::transmute_copy(&lnboolean)).into()
        }
        unsafe extern "system" fn Integer<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Integer() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInteger<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lninteger: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInteger(::core::mem::transmute_copy(&lninteger)).into()
        }
        unsafe extern "system" fn OctetString<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OctetString() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOctetString<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, voctetstring: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOctetString(::core::mem::transmute_copy(&voctetstring)).into()
        }
        unsafe extern "system" fn SecurityDescriptor<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SecurityDescriptor() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityDescriptor<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psecuritydescriptor: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSecurityDescriptor(::core::mem::transmute(&psecuritydescriptor)).into()
        }
        unsafe extern "system" fn LargeInteger<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LargeInteger() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLargeInteger<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plargeinteger: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLargeInteger(::core::mem::transmute(&plargeinteger)).into()
        }
        unsafe extern "system" fn UTCTime<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UTCTime() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUTCTime<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dautctime: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUTCTime(::core::mem::transmute_copy(&dautctime)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Clear: Clear::<Identity, Impl, OFFSET>,
            ADsType: ADsType::<Identity, Impl, OFFSET>,
            SetADsType: SetADsType::<Identity, Impl, OFFSET>,
            DNString: DNString::<Identity, Impl, OFFSET>,
            SetDNString: SetDNString::<Identity, Impl, OFFSET>,
            CaseExactString: CaseExactString::<Identity, Impl, OFFSET>,
            SetCaseExactString: SetCaseExactString::<Identity, Impl, OFFSET>,
            CaseIgnoreString: CaseIgnoreString::<Identity, Impl, OFFSET>,
            SetCaseIgnoreString: SetCaseIgnoreString::<Identity, Impl, OFFSET>,
            PrintableString: PrintableString::<Identity, Impl, OFFSET>,
            SetPrintableString: SetPrintableString::<Identity, Impl, OFFSET>,
            NumericString: NumericString::<Identity, Impl, OFFSET>,
            SetNumericString: SetNumericString::<Identity, Impl, OFFSET>,
            Boolean: Boolean::<Identity, Impl, OFFSET>,
            SetBoolean: SetBoolean::<Identity, Impl, OFFSET>,
            Integer: Integer::<Identity, Impl, OFFSET>,
            SetInteger: SetInteger::<Identity, Impl, OFFSET>,
            OctetString: OctetString::<Identity, Impl, OFFSET>,
            SetOctetString: SetOctetString::<Identity, Impl, OFFSET>,
            SecurityDescriptor: SecurityDescriptor::<Identity, Impl, OFFSET>,
            SetSecurityDescriptor: SetSecurityDescriptor::<Identity, Impl, OFFSET>,
            LargeInteger: LargeInteger::<Identity, Impl, OFFSET>,
            SetLargeInteger: SetLargeInteger::<Identity, Impl, OFFSET>,
            UTCTime: UTCTime::<Identity, Impl, OFFSET>,
            SetUTCTime: SetUTCTime::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsPropertyValue as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsPropertyValue2_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetObjectProperty(&mut self, lnadstype: *mut i32, pvprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn PutObjectProperty(&mut self, lnadstype: i32, vprop: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsPropertyValue2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyValue2_Impl, const OFFSET: isize>() -> IADsPropertyValue2_Vtbl {
        unsafe extern "system" fn GetObjectProperty<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyValue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnadstype: *mut i32, pvprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetObjectProperty(::core::mem::transmute_copy(&lnadstype), ::core::mem::transmute_copy(&pvprop)).into()
        }
        unsafe extern "system" fn PutObjectProperty<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyValue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnadstype: i32, vprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PutObjectProperty(::core::mem::transmute_copy(&lnadstype), ::core::mem::transmute_copy(&vprop)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetObjectProperty: GetObjectProperty::<Identity, Impl, OFFSET>,
            PutObjectProperty: PutObjectProperty::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsPropertyValue2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsReplicaPointer_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ServerName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetServerName(&mut self, bstrservername: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ReplicaType(&mut self) -> ::windows::core::Result<i32>;
    fn SetReplicaType(&mut self, lnreplicatype: i32) -> ::windows::core::Result<()>;
    fn ReplicaNumber(&mut self) -> ::windows::core::Result<i32>;
    fn SetReplicaNumber(&mut self, lnreplicanumber: i32) -> ::windows::core::Result<()>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn SetCount(&mut self, lncount: i32) -> ::windows::core::Result<()>;
    fn ReplicaAddressHints(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetReplicaAddressHints(&mut self, vreplicaaddresshints: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsReplicaPointer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsReplicaPointer_Impl, const OFFSET: isize>() -> IADsReplicaPointer_Vtbl {
        unsafe extern "system" fn ServerName<Identity: ::windows::core::IUnknownImpl, Impl: IADsReplicaPointer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ServerName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServerName<Identity: ::windows::core::IUnknownImpl, Impl: IADsReplicaPointer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrservername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetServerName(::core::mem::transmute_copy(&bstrservername)).into()
        }
        unsafe extern "system" fn ReplicaType<Identity: ::windows::core::IUnknownImpl, Impl: IADsReplicaPointer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReplicaType() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReplicaType<Identity: ::windows::core::IUnknownImpl, Impl: IADsReplicaPointer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnreplicatype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetReplicaType(::core::mem::transmute_copy(&lnreplicatype)).into()
        }
        unsafe extern "system" fn ReplicaNumber<Identity: ::windows::core::IUnknownImpl, Impl: IADsReplicaPointer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReplicaNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReplicaNumber<Identity: ::windows::core::IUnknownImpl, Impl: IADsReplicaPointer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnreplicanumber: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetReplicaNumber(::core::mem::transmute_copy(&lnreplicanumber)).into()
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IADsReplicaPointer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCount<Identity: ::windows::core::IUnknownImpl, Impl: IADsReplicaPointer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lncount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCount(::core::mem::transmute_copy(&lncount)).into()
        }
        unsafe extern "system" fn ReplicaAddressHints<Identity: ::windows::core::IUnknownImpl, Impl: IADsReplicaPointer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReplicaAddressHints() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReplicaAddressHints<Identity: ::windows::core::IUnknownImpl, Impl: IADsReplicaPointer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vreplicaaddresshints: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetReplicaAddressHints(::core::mem::transmute_copy(&vreplicaaddresshints)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ServerName: ServerName::<Identity, Impl, OFFSET>,
            SetServerName: SetServerName::<Identity, Impl, OFFSET>,
            ReplicaType: ReplicaType::<Identity, Impl, OFFSET>,
            SetReplicaType: SetReplicaType::<Identity, Impl, OFFSET>,
            ReplicaNumber: ReplicaNumber::<Identity, Impl, OFFSET>,
            SetReplicaNumber: SetReplicaNumber::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            SetCount: SetCount::<Identity, Impl, OFFSET>,
            ReplicaAddressHints: ReplicaAddressHints::<Identity, Impl, OFFSET>,
            SetReplicaAddressHints: SetReplicaAddressHints::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsReplicaPointer as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsResource_Impl: Sized + super::super::System::Com::IDispatch_Impl + IADs_Impl {
    fn User(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn UserPath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Path(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn LockCount(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsResource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsResource_Impl, const OFFSET: isize>() -> IADsResource_Vtbl {
        unsafe extern "system" fn User<Identity: ::windows::core::IUnknownImpl, Impl: IADsResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).User() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserPath<Identity: ::windows::core::IUnknownImpl, Impl: IADsResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UserPath() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path<Identity: ::windows::core::IUnknownImpl, Impl: IADsResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockCount<Identity: ::windows::core::IUnknownImpl, Impl: IADsResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LockCount() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            User: User::<Identity, Impl, OFFSET>,
            UserPath: UserPath::<Identity, Impl, OFFSET>,
            Path: Path::<Identity, Impl, OFFSET>,
            LockCount: LockCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsResource as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IADs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsSecurityDescriptor_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Revision(&mut self) -> ::windows::core::Result<i32>;
    fn SetRevision(&mut self, lnrevision: i32) -> ::windows::core::Result<()>;
    fn Control(&mut self) -> ::windows::core::Result<i32>;
    fn SetControl(&mut self, lncontrol: i32) -> ::windows::core::Result<()>;
    fn Owner(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetOwner(&mut self, bstrowner: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn OwnerDefaulted(&mut self) -> ::windows::core::Result<i16>;
    fn SetOwnerDefaulted(&mut self, fownerdefaulted: i16) -> ::windows::core::Result<()>;
    fn Group(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetGroup(&mut self, bstrgroup: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GroupDefaulted(&mut self) -> ::windows::core::Result<i16>;
    fn SetGroupDefaulted(&mut self, fgroupdefaulted: i16) -> ::windows::core::Result<()>;
    fn DiscretionaryAcl(&mut self) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn SetDiscretionaryAcl(&mut self, pdiscretionaryacl: &::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn DaclDefaulted(&mut self) -> ::windows::core::Result<i16>;
    fn SetDaclDefaulted(&mut self, fdacldefaulted: i16) -> ::windows::core::Result<()>;
    fn SystemAcl(&mut self) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn SetSystemAcl(&mut self, psystemacl: &::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn SaclDefaulted(&mut self) -> ::windows::core::Result<i16>;
    fn SetSaclDefaulted(&mut self, fsacldefaulted: i16) -> ::windows::core::Result<()>;
    fn CopySecurityDescriptor(&mut self) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsSecurityDescriptor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>() -> IADsSecurityDescriptor_Vtbl {
        unsafe extern "system" fn Revision<Identity: ::windows::core::IUnknownImpl, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Revision() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRevision<Identity: ::windows::core::IUnknownImpl, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnrevision: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRevision(::core::mem::transmute_copy(&lnrevision)).into()
        }
        unsafe extern "system" fn Control<Identity: ::windows::core::IUnknownImpl, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Control() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetControl<Identity: ::windows::core::IUnknownImpl, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lncontrol: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetControl(::core::mem::transmute_copy(&lncontrol)).into()
        }
        unsafe extern "system" fn Owner<Identity: ::windows::core::IUnknownImpl, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Owner() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOwner<Identity: ::windows::core::IUnknownImpl, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrowner: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOwner(::core::mem::transmute_copy(&bstrowner)).into()
        }
        unsafe extern "system" fn OwnerDefaulted<Identity: ::windows::core::IUnknownImpl, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OwnerDefaulted() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOwnerDefaulted<Identity: ::windows::core::IUnknownImpl, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fownerdefaulted: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOwnerDefaulted(::core::mem::transmute_copy(&fownerdefaulted)).into()
        }
        unsafe extern "system" fn Group<Identity: ::windows::core::IUnknownImpl, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Group() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGroup<Identity: ::windows::core::IUnknownImpl, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroup: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGroup(::core::mem::transmute_copy(&bstrgroup)).into()
        }
        unsafe extern "system" fn GroupDefaulted<Identity: ::windows::core::IUnknownImpl, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GroupDefaulted() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGroupDefaulted<Identity: ::windows::core::IUnknownImpl, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fgroupdefaulted: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGroupDefaulted(::core::mem::transmute_copy(&fgroupdefaulted)).into()
        }
        unsafe extern "system" fn DiscretionaryAcl<Identity: ::windows::core::IUnknownImpl, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DiscretionaryAcl() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDiscretionaryAcl<Identity: ::windows::core::IUnknownImpl, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdiscretionaryacl: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDiscretionaryAcl(::core::mem::transmute(&pdiscretionaryacl)).into()
        }
        unsafe extern "system" fn DaclDefaulted<Identity: ::windows::core::IUnknownImpl, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DaclDefaulted() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDaclDefaulted<Identity: ::windows::core::IUnknownImpl, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fdacldefaulted: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDaclDefaulted(::core::mem::transmute_copy(&fdacldefaulted)).into()
        }
        unsafe extern "system" fn SystemAcl<Identity: ::windows::core::IUnknownImpl, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SystemAcl() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSystemAcl<Identity: ::windows::core::IUnknownImpl, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psystemacl: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSystemAcl(::core::mem::transmute(&psystemacl)).into()
        }
        unsafe extern "system" fn SaclDefaulted<Identity: ::windows::core::IUnknownImpl, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SaclDefaulted() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSaclDefaulted<Identity: ::windows::core::IUnknownImpl, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fsacldefaulted: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSaclDefaulted(::core::mem::transmute_copy(&fsacldefaulted)).into()
        }
        unsafe extern "system" fn CopySecurityDescriptor<Identity: ::windows::core::IUnknownImpl, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsecuritydescriptor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CopySecurityDescriptor() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsecuritydescriptor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Revision: Revision::<Identity, Impl, OFFSET>,
            SetRevision: SetRevision::<Identity, Impl, OFFSET>,
            Control: Control::<Identity, Impl, OFFSET>,
            SetControl: SetControl::<Identity, Impl, OFFSET>,
            Owner: Owner::<Identity, Impl, OFFSET>,
            SetOwner: SetOwner::<Identity, Impl, OFFSET>,
            OwnerDefaulted: OwnerDefaulted::<Identity, Impl, OFFSET>,
            SetOwnerDefaulted: SetOwnerDefaulted::<Identity, Impl, OFFSET>,
            Group: Group::<Identity, Impl, OFFSET>,
            SetGroup: SetGroup::<Identity, Impl, OFFSET>,
            GroupDefaulted: GroupDefaulted::<Identity, Impl, OFFSET>,
            SetGroupDefaulted: SetGroupDefaulted::<Identity, Impl, OFFSET>,
            DiscretionaryAcl: DiscretionaryAcl::<Identity, Impl, OFFSET>,
            SetDiscretionaryAcl: SetDiscretionaryAcl::<Identity, Impl, OFFSET>,
            DaclDefaulted: DaclDefaulted::<Identity, Impl, OFFSET>,
            SetDaclDefaulted: SetDaclDefaulted::<Identity, Impl, OFFSET>,
            SystemAcl: SystemAcl::<Identity, Impl, OFFSET>,
            SetSystemAcl: SetSystemAcl::<Identity, Impl, OFFSET>,
            SaclDefaulted: SaclDefaulted::<Identity, Impl, OFFSET>,
            SetSaclDefaulted: SetSaclDefaulted::<Identity, Impl, OFFSET>,
            CopySecurityDescriptor: CopySecurityDescriptor::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsSecurityDescriptor as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsSecurityUtility_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetSecurityDescriptor(&mut self, varpath: &super::super::System::Com::VARIANT, lpathformat: i32, lformat: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetSecurityDescriptor(&mut self, varpath: &super::super::System::Com::VARIANT, lpathformat: i32, vardata: &super::super::System::Com::VARIANT, ldataformat: i32) -> ::windows::core::Result<()>;
    fn ConvertSecurityDescriptor(&mut self, varsd: &super::super::System::Com::VARIANT, ldataformat: i32, loutformat: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SecurityMask(&mut self) -> ::windows::core::Result<i32>;
    fn SetSecurityMask(&mut self, lnsecuritymask: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsSecurityUtility_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsSecurityUtility_Impl, const OFFSET: isize>() -> IADsSecurityUtility_Vtbl {
        unsafe extern "system" fn GetSecurityDescriptor<Identity: ::windows::core::IUnknownImpl, Impl: IADsSecurityUtility_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varpath: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, lpathformat: i32, lformat: i32, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSecurityDescriptor(::core::mem::transmute_copy(&varpath), ::core::mem::transmute_copy(&lpathformat), ::core::mem::transmute_copy(&lformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityDescriptor<Identity: ::windows::core::IUnknownImpl, Impl: IADsSecurityUtility_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varpath: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, lpathformat: i32, vardata: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ldataformat: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSecurityDescriptor(::core::mem::transmute_copy(&varpath), ::core::mem::transmute_copy(&lpathformat), ::core::mem::transmute_copy(&vardata), ::core::mem::transmute_copy(&ldataformat)).into()
        }
        unsafe extern "system" fn ConvertSecurityDescriptor<Identity: ::windows::core::IUnknownImpl, Impl: IADsSecurityUtility_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsd: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ldataformat: i32, loutformat: i32, presult: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ConvertSecurityDescriptor(::core::mem::transmute_copy(&varsd), ::core::mem::transmute_copy(&ldataformat), ::core::mem::transmute_copy(&loutformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *presult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SecurityMask<Identity: ::windows::core::IUnknownImpl, Impl: IADsSecurityUtility_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SecurityMask() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityMask<Identity: ::windows::core::IUnknownImpl, Impl: IADsSecurityUtility_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnsecuritymask: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSecurityMask(::core::mem::transmute_copy(&lnsecuritymask)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetSecurityDescriptor: GetSecurityDescriptor::<Identity, Impl, OFFSET>,
            SetSecurityDescriptor: SetSecurityDescriptor::<Identity, Impl, OFFSET>,
            ConvertSecurityDescriptor: ConvertSecurityDescriptor::<Identity, Impl, OFFSET>,
            SecurityMask: SecurityMask::<Identity, Impl, OFFSET>,
            SetSecurityMask: SetSecurityMask::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsSecurityUtility as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsService_Impl: Sized + super::super::System::Com::IDispatch_Impl + IADs_Impl {
    fn HostComputer(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetHostComputer(&mut self, bstrhostcomputer: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DisplayName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDisplayName(&mut self, bstrdisplayname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Version(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetVersion(&mut self, bstrversion: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ServiceType(&mut self) -> ::windows::core::Result<i32>;
    fn SetServiceType(&mut self, lnservicetype: i32) -> ::windows::core::Result<()>;
    fn StartType(&mut self) -> ::windows::core::Result<i32>;
    fn SetStartType(&mut self, lnstarttype: i32) -> ::windows::core::Result<()>;
    fn Path(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPath(&mut self, bstrpath: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn StartupParameters(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetStartupParameters(&mut self, bstrstartupparameters: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ErrorControl(&mut self) -> ::windows::core::Result<i32>;
    fn SetErrorControl(&mut self, lnerrorcontrol: i32) -> ::windows::core::Result<()>;
    fn LoadOrderGroup(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLoadOrderGroup(&mut self, bstrloadordergroup: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ServiceAccountName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetServiceAccountName(&mut self, bstrserviceaccountname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ServiceAccountPath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetServiceAccountPath(&mut self, bstrserviceaccountpath: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Dependencies(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetDependencies(&mut self, vdependencies: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsService_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsService_Impl, const OFFSET: isize>() -> IADsService_Vtbl {
        unsafe extern "system" fn HostComputer<Identity: ::windows::core::IUnknownImpl, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HostComputer() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHostComputer<Identity: ::windows::core::IUnknownImpl, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrhostcomputer: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHostComputer(::core::mem::transmute_copy(&bstrhostcomputer)).into()
        }
        unsafe extern "system" fn DisplayName<Identity: ::windows::core::IUnknownImpl, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Identity: ::windows::core::IUnknownImpl, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdisplayname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDisplayName(::core::mem::transmute_copy(&bstrdisplayname)).into()
        }
        unsafe extern "system" fn Version<Identity: ::windows::core::IUnknownImpl, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Version() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVersion<Identity: ::windows::core::IUnknownImpl, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrversion: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetVersion(::core::mem::transmute_copy(&bstrversion)).into()
        }
        unsafe extern "system" fn ServiceType<Identity: ::windows::core::IUnknownImpl, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ServiceType() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceType<Identity: ::windows::core::IUnknownImpl, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnservicetype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetServiceType(::core::mem::transmute_copy(&lnservicetype)).into()
        }
        unsafe extern "system" fn StartType<Identity: ::windows::core::IUnknownImpl, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StartType() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartType<Identity: ::windows::core::IUnknownImpl, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnstarttype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStartType(::core::mem::transmute_copy(&lnstarttype)).into()
        }
        unsafe extern "system" fn Path<Identity: ::windows::core::IUnknownImpl, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPath<Identity: ::windows::core::IUnknownImpl, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPath(::core::mem::transmute_copy(&bstrpath)).into()
        }
        unsafe extern "system" fn StartupParameters<Identity: ::windows::core::IUnknownImpl, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StartupParameters() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartupParameters<Identity: ::windows::core::IUnknownImpl, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrstartupparameters: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStartupParameters(::core::mem::transmute_copy(&bstrstartupparameters)).into()
        }
        unsafe extern "system" fn ErrorControl<Identity: ::windows::core::IUnknownImpl, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ErrorControl() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetErrorControl<Identity: ::windows::core::IUnknownImpl, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnerrorcontrol: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetErrorControl(::core::mem::transmute_copy(&lnerrorcontrol)).into()
        }
        unsafe extern "system" fn LoadOrderGroup<Identity: ::windows::core::IUnknownImpl, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LoadOrderGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLoadOrderGroup<Identity: ::windows::core::IUnknownImpl, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrloadordergroup: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLoadOrderGroup(::core::mem::transmute_copy(&bstrloadordergroup)).into()
        }
        unsafe extern "system" fn ServiceAccountName<Identity: ::windows::core::IUnknownImpl, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ServiceAccountName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceAccountName<Identity: ::windows::core::IUnknownImpl, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrserviceaccountname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetServiceAccountName(::core::mem::transmute_copy(&bstrserviceaccountname)).into()
        }
        unsafe extern "system" fn ServiceAccountPath<Identity: ::windows::core::IUnknownImpl, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ServiceAccountPath() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceAccountPath<Identity: ::windows::core::IUnknownImpl, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrserviceaccountpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetServiceAccountPath(::core::mem::transmute_copy(&bstrserviceaccountpath)).into()
        }
        unsafe extern "system" fn Dependencies<Identity: ::windows::core::IUnknownImpl, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Dependencies() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDependencies<Identity: ::windows::core::IUnknownImpl, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vdependencies: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDependencies(::core::mem::transmute_copy(&vdependencies)).into()
        }
        Self {
            base: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            HostComputer: HostComputer::<Identity, Impl, OFFSET>,
            SetHostComputer: SetHostComputer::<Identity, Impl, OFFSET>,
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, Impl, OFFSET>,
            Version: Version::<Identity, Impl, OFFSET>,
            SetVersion: SetVersion::<Identity, Impl, OFFSET>,
            ServiceType: ServiceType::<Identity, Impl, OFFSET>,
            SetServiceType: SetServiceType::<Identity, Impl, OFFSET>,
            StartType: StartType::<Identity, Impl, OFFSET>,
            SetStartType: SetStartType::<Identity, Impl, OFFSET>,
            Path: Path::<Identity, Impl, OFFSET>,
            SetPath: SetPath::<Identity, Impl, OFFSET>,
            StartupParameters: StartupParameters::<Identity, Impl, OFFSET>,
            SetStartupParameters: SetStartupParameters::<Identity, Impl, OFFSET>,
            ErrorControl: ErrorControl::<Identity, Impl, OFFSET>,
            SetErrorControl: SetErrorControl::<Identity, Impl, OFFSET>,
            LoadOrderGroup: LoadOrderGroup::<Identity, Impl, OFFSET>,
            SetLoadOrderGroup: SetLoadOrderGroup::<Identity, Impl, OFFSET>,
            ServiceAccountName: ServiceAccountName::<Identity, Impl, OFFSET>,
            SetServiceAccountName: SetServiceAccountName::<Identity, Impl, OFFSET>,
            ServiceAccountPath: ServiceAccountPath::<Identity, Impl, OFFSET>,
            SetServiceAccountPath: SetServiceAccountPath::<Identity, Impl, OFFSET>,
            Dependencies: Dependencies::<Identity, Impl, OFFSET>,
            SetDependencies: SetDependencies::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsService as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IADs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsServiceOperations_Impl: Sized + super::super::System::Com::IDispatch_Impl + IADs_Impl {
    fn Status(&mut self) -> ::windows::core::Result<i32>;
    fn Start(&mut self) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
    fn Pause(&mut self) -> ::windows::core::Result<()>;
    fn Continue(&mut self) -> ::windows::core::Result<()>;
    fn SetPassword(&mut self, bstrnewpassword: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsServiceOperations_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsServiceOperations_Impl, const OFFSET: isize>() -> IADsServiceOperations_Vtbl {
        unsafe extern "system" fn Status<Identity: ::windows::core::IUnknownImpl, Impl: IADsServiceOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Identity: ::windows::core::IUnknownImpl, Impl: IADsServiceOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Identity: ::windows::core::IUnknownImpl, Impl: IADsServiceOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn Pause<Identity: ::windows::core::IUnknownImpl, Impl: IADsServiceOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn Continue<Identity: ::windows::core::IUnknownImpl, Impl: IADsServiceOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Continue().into()
        }
        unsafe extern "system" fn SetPassword<Identity: ::windows::core::IUnknownImpl, Impl: IADsServiceOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrnewpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPassword(::core::mem::transmute_copy(&bstrnewpassword)).into()
        }
        Self {
            base: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            Status: Status::<Identity, Impl, OFFSET>,
            Start: Start::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Continue: Continue::<Identity, Impl, OFFSET>,
            SetPassword: SetPassword::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsServiceOperations as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IADs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsSession_Impl: Sized + super::super::System::Com::IDispatch_Impl + IADs_Impl {
    fn User(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn UserPath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Computer(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ComputerPath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ConnectTime(&mut self) -> ::windows::core::Result<i32>;
    fn IdleTime(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsSession_Impl, const OFFSET: isize>() -> IADsSession_Vtbl {
        unsafe extern "system" fn User<Identity: ::windows::core::IUnknownImpl, Impl: IADsSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).User() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserPath<Identity: ::windows::core::IUnknownImpl, Impl: IADsSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UserPath() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Computer<Identity: ::windows::core::IUnknownImpl, Impl: IADsSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Computer() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComputerPath<Identity: ::windows::core::IUnknownImpl, Impl: IADsSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ComputerPath() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectTime<Identity: ::windows::core::IUnknownImpl, Impl: IADsSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ConnectTime() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IdleTime<Identity: ::windows::core::IUnknownImpl, Impl: IADsSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IdleTime() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            User: User::<Identity, Impl, OFFSET>,
            UserPath: UserPath::<Identity, Impl, OFFSET>,
            Computer: Computer::<Identity, Impl, OFFSET>,
            ComputerPath: ComputerPath::<Identity, Impl, OFFSET>,
            ConnectTime: ConnectTime::<Identity, Impl, OFFSET>,
            IdleTime: IdleTime::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsSession as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IADs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsSyntax_Impl: Sized + super::super::System::Com::IDispatch_Impl + IADs_Impl {
    fn OleAutoDataType(&mut self) -> ::windows::core::Result<i32>;
    fn SetOleAutoDataType(&mut self, lnoleautodatatype: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsSyntax_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsSyntax_Impl, const OFFSET: isize>() -> IADsSyntax_Vtbl {
        unsafe extern "system" fn OleAutoDataType<Identity: ::windows::core::IUnknownImpl, Impl: IADsSyntax_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OleAutoDataType() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOleAutoDataType<Identity: ::windows::core::IUnknownImpl, Impl: IADsSyntax_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnoleautodatatype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOleAutoDataType(::core::mem::transmute_copy(&lnoleautodatatype)).into()
        }
        Self {
            base: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            OleAutoDataType: OleAutoDataType::<Identity, Impl, OFFSET>,
            SetOleAutoDataType: SetOleAutoDataType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsSyntax as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IADs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsTimestamp_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn WholeSeconds(&mut self) -> ::windows::core::Result<i32>;
    fn SetWholeSeconds(&mut self, lnwholeseconds: i32) -> ::windows::core::Result<()>;
    fn EventID(&mut self) -> ::windows::core::Result<i32>;
    fn SetEventID(&mut self, lneventid: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsTimestamp_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsTimestamp_Impl, const OFFSET: isize>() -> IADsTimestamp_Vtbl {
        unsafe extern "system" fn WholeSeconds<Identity: ::windows::core::IUnknownImpl, Impl: IADsTimestamp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).WholeSeconds() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWholeSeconds<Identity: ::windows::core::IUnknownImpl, Impl: IADsTimestamp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnwholeseconds: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetWholeSeconds(::core::mem::transmute_copy(&lnwholeseconds)).into()
        }
        unsafe extern "system" fn EventID<Identity: ::windows::core::IUnknownImpl, Impl: IADsTimestamp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EventID() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventID<Identity: ::windows::core::IUnknownImpl, Impl: IADsTimestamp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lneventid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEventID(::core::mem::transmute_copy(&lneventid)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            WholeSeconds: WholeSeconds::<Identity, Impl, OFFSET>,
            SetWholeSeconds: SetWholeSeconds::<Identity, Impl, OFFSET>,
            EventID: EventID::<Identity, Impl, OFFSET>,
            SetEventID: SetEventID::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsTimestamp as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsTypedName_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ObjectName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetObjectName(&mut self, bstrobjectname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Level(&mut self) -> ::windows::core::Result<i32>;
    fn SetLevel(&mut self, lnlevel: i32) -> ::windows::core::Result<()>;
    fn Interval(&mut self) -> ::windows::core::Result<i32>;
    fn SetInterval(&mut self, lninterval: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsTypedName_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsTypedName_Impl, const OFFSET: isize>() -> IADsTypedName_Vtbl {
        unsafe extern "system" fn ObjectName<Identity: ::windows::core::IUnknownImpl, Impl: IADsTypedName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ObjectName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetObjectName<Identity: ::windows::core::IUnknownImpl, Impl: IADsTypedName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrobjectname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetObjectName(::core::mem::transmute_copy(&bstrobjectname)).into()
        }
        unsafe extern "system" fn Level<Identity: ::windows::core::IUnknownImpl, Impl: IADsTypedName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Level() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLevel<Identity: ::windows::core::IUnknownImpl, Impl: IADsTypedName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLevel(::core::mem::transmute_copy(&lnlevel)).into()
        }
        unsafe extern "system" fn Interval<Identity: ::windows::core::IUnknownImpl, Impl: IADsTypedName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Interval() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInterval<Identity: ::windows::core::IUnknownImpl, Impl: IADsTypedName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lninterval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInterval(::core::mem::transmute_copy(&lninterval)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ObjectName: ObjectName::<Identity, Impl, OFFSET>,
            SetObjectName: SetObjectName::<Identity, Impl, OFFSET>,
            Level: Level::<Identity, Impl, OFFSET>,
            SetLevel: SetLevel::<Identity, Impl, OFFSET>,
            Interval: Interval::<Identity, Impl, OFFSET>,
            SetInterval: SetInterval::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsTypedName as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsUser_Impl: Sized + super::super::System::Com::IDispatch_Impl + IADs_Impl {
    fn BadLoginAddress(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn BadLoginCount(&mut self) -> ::windows::core::Result<i32>;
    fn LastLogin(&mut self) -> ::windows::core::Result<f64>;
    fn LastLogoff(&mut self) -> ::windows::core::Result<f64>;
    fn LastFailedLogin(&mut self) -> ::windows::core::Result<f64>;
    fn PasswordLastChanged(&mut self) -> ::windows::core::Result<f64>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, bstrdescription: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Division(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDivision(&mut self, bstrdivision: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Department(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDepartment(&mut self, bstrdepartment: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn EmployeeID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetEmployeeID(&mut self, bstremployeeid: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FullName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetFullName(&mut self, bstrfullname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FirstName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetFirstName(&mut self, bstrfirstname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn LastName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLastName(&mut self, bstrlastname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn OtherName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetOtherName(&mut self, bstrothername: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn NamePrefix(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetNamePrefix(&mut self, bstrnameprefix: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn NameSuffix(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetNameSuffix(&mut self, bstrnamesuffix: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Title(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetTitle(&mut self, bstrtitle: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Manager(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetManager(&mut self, bstrmanager: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn TelephoneHome(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetTelephoneHome(&mut self, vtelephonehome: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn TelephoneMobile(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetTelephoneMobile(&mut self, vtelephonemobile: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn TelephoneNumber(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetTelephoneNumber(&mut self, vtelephonenumber: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn TelephonePager(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetTelephonePager(&mut self, vtelephonepager: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn FaxNumber(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetFaxNumber(&mut self, vfaxnumber: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn OfficeLocations(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetOfficeLocations(&mut self, vofficelocations: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn PostalAddresses(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetPostalAddresses(&mut self, vpostaladdresses: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn PostalCodes(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetPostalCodes(&mut self, vpostalcodes: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SeeAlso(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetSeeAlso(&mut self, vseealso: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AccountDisabled(&mut self) -> ::windows::core::Result<i16>;
    fn SetAccountDisabled(&mut self, faccountdisabled: i16) -> ::windows::core::Result<()>;
    fn AccountExpirationDate(&mut self) -> ::windows::core::Result<f64>;
    fn SetAccountExpirationDate(&mut self, daaccountexpirationdate: f64) -> ::windows::core::Result<()>;
    fn GraceLoginsAllowed(&mut self) -> ::windows::core::Result<i32>;
    fn SetGraceLoginsAllowed(&mut self, lngraceloginsallowed: i32) -> ::windows::core::Result<()>;
    fn GraceLoginsRemaining(&mut self) -> ::windows::core::Result<i32>;
    fn SetGraceLoginsRemaining(&mut self, lngraceloginsremaining: i32) -> ::windows::core::Result<()>;
    fn IsAccountLocked(&mut self) -> ::windows::core::Result<i16>;
    fn SetIsAccountLocked(&mut self, fisaccountlocked: i16) -> ::windows::core::Result<()>;
    fn LoginHours(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetLoginHours(&mut self, vloginhours: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn LoginWorkstations(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetLoginWorkstations(&mut self, vloginworkstations: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn MaxLogins(&mut self) -> ::windows::core::Result<i32>;
    fn SetMaxLogins(&mut self, lnmaxlogins: i32) -> ::windows::core::Result<()>;
    fn MaxStorage(&mut self) -> ::windows::core::Result<i32>;
    fn SetMaxStorage(&mut self, lnmaxstorage: i32) -> ::windows::core::Result<()>;
    fn PasswordExpirationDate(&mut self) -> ::windows::core::Result<f64>;
    fn SetPasswordExpirationDate(&mut self, dapasswordexpirationdate: f64) -> ::windows::core::Result<()>;
    fn PasswordMinimumLength(&mut self) -> ::windows::core::Result<i32>;
    fn SetPasswordMinimumLength(&mut self, lnpasswordminimumlength: i32) -> ::windows::core::Result<()>;
    fn PasswordRequired(&mut self) -> ::windows::core::Result<i16>;
    fn SetPasswordRequired(&mut self, fpasswordrequired: i16) -> ::windows::core::Result<()>;
    fn RequireUniquePassword(&mut self) -> ::windows::core::Result<i16>;
    fn SetRequireUniquePassword(&mut self, frequireuniquepassword: i16) -> ::windows::core::Result<()>;
    fn EmailAddress(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetEmailAddress(&mut self, bstremailaddress: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn HomeDirectory(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetHomeDirectory(&mut self, bstrhomedirectory: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Languages(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetLanguages(&mut self, vlanguages: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Profile(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetProfile(&mut self, bstrprofile: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn LoginScript(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLoginScript(&mut self, bstrloginscript: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Picture(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetPicture(&mut self, vpicture: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn HomePage(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetHomePage(&mut self, bstrhomepage: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Groups(&mut self) -> ::windows::core::Result<IADsMembers>;
    fn SetPassword(&mut self, newpassword: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ChangePassword(&mut self, bstroldpassword: &super::super::Foundation::BSTR, bstrnewpassword: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsUser_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>() -> IADsUser_Vtbl {
        unsafe extern "system" fn BadLoginAddress<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BadLoginAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BadLoginCount<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BadLoginCount() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastLogin<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LastLogin() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastLogoff<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LastLogoff() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastFailedLogin<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LastFailedLogin() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PasswordLastChanged<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PasswordLastChanged() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&bstrdescription)).into()
        }
        unsafe extern "system" fn Division<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Division() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDivision<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdivision: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDivision(::core::mem::transmute_copy(&bstrdivision)).into()
        }
        unsafe extern "system" fn Department<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Department() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDepartment<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdepartment: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDepartment(::core::mem::transmute_copy(&bstrdepartment)).into()
        }
        unsafe extern "system" fn EmployeeID<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EmployeeID() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEmployeeID<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstremployeeid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEmployeeID(::core::mem::transmute_copy(&bstremployeeid)).into()
        }
        unsafe extern "system" fn FullName<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FullName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFullName<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfullname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFullName(::core::mem::transmute_copy(&bstrfullname)).into()
        }
        unsafe extern "system" fn FirstName<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FirstName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFirstName<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfirstname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFirstName(::core::mem::transmute_copy(&bstrfirstname)).into()
        }
        unsafe extern "system" fn LastName<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LastName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastName<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlastname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLastName(::core::mem::transmute_copy(&bstrlastname)).into()
        }
        unsafe extern "system" fn OtherName<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OtherName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOtherName<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrothername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOtherName(::core::mem::transmute_copy(&bstrothername)).into()
        }
        unsafe extern "system" fn NamePrefix<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NamePrefix() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNamePrefix<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrnameprefix: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNamePrefix(::core::mem::transmute_copy(&bstrnameprefix)).into()
        }
        unsafe extern "system" fn NameSuffix<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NameSuffix() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNameSuffix<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrnamesuffix: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNameSuffix(::core::mem::transmute_copy(&bstrnamesuffix)).into()
        }
        unsafe extern "system" fn Title<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Title() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTitle<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTitle(::core::mem::transmute_copy(&bstrtitle)).into()
        }
        unsafe extern "system" fn Manager<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Manager() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetManager<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmanager: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetManager(::core::mem::transmute_copy(&bstrmanager)).into()
        }
        unsafe extern "system" fn TelephoneHome<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TelephoneHome() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTelephoneHome<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vtelephonehome: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTelephoneHome(::core::mem::transmute_copy(&vtelephonehome)).into()
        }
        unsafe extern "system" fn TelephoneMobile<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TelephoneMobile() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTelephoneMobile<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vtelephonemobile: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTelephoneMobile(::core::mem::transmute_copy(&vtelephonemobile)).into()
        }
        unsafe extern "system" fn TelephoneNumber<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TelephoneNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTelephoneNumber<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vtelephonenumber: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTelephoneNumber(::core::mem::transmute_copy(&vtelephonenumber)).into()
        }
        unsafe extern "system" fn TelephonePager<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TelephonePager() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTelephonePager<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vtelephonepager: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTelephonePager(::core::mem::transmute_copy(&vtelephonepager)).into()
        }
        unsafe extern "system" fn FaxNumber<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FaxNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFaxNumber<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vfaxnumber: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFaxNumber(::core::mem::transmute_copy(&vfaxnumber)).into()
        }
        unsafe extern "system" fn OfficeLocations<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OfficeLocations() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOfficeLocations<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vofficelocations: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOfficeLocations(::core::mem::transmute_copy(&vofficelocations)).into()
        }
        unsafe extern "system" fn PostalAddresses<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PostalAddresses() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPostalAddresses<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vpostaladdresses: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPostalAddresses(::core::mem::transmute_copy(&vpostaladdresses)).into()
        }
        unsafe extern "system" fn PostalCodes<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PostalCodes() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPostalCodes<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vpostalcodes: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPostalCodes(::core::mem::transmute_copy(&vpostalcodes)).into()
        }
        unsafe extern "system" fn SeeAlso<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SeeAlso() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSeeAlso<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vseealso: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSeeAlso(::core::mem::transmute_copy(&vseealso)).into()
        }
        unsafe extern "system" fn AccountDisabled<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AccountDisabled() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccountDisabled<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, faccountdisabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAccountDisabled(::core::mem::transmute_copy(&faccountdisabled)).into()
        }
        unsafe extern "system" fn AccountExpirationDate<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AccountExpirationDate() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccountExpirationDate<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, daaccountexpirationdate: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAccountExpirationDate(::core::mem::transmute_copy(&daaccountexpirationdate)).into()
        }
        unsafe extern "system" fn GraceLoginsAllowed<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GraceLoginsAllowed() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGraceLoginsAllowed<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lngraceloginsallowed: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGraceLoginsAllowed(::core::mem::transmute_copy(&lngraceloginsallowed)).into()
        }
        unsafe extern "system" fn GraceLoginsRemaining<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GraceLoginsRemaining() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGraceLoginsRemaining<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lngraceloginsremaining: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGraceLoginsRemaining(::core::mem::transmute_copy(&lngraceloginsremaining)).into()
        }
        unsafe extern "system" fn IsAccountLocked<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsAccountLocked() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsAccountLocked<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fisaccountlocked: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIsAccountLocked(::core::mem::transmute_copy(&fisaccountlocked)).into()
        }
        unsafe extern "system" fn LoginHours<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LoginHours() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLoginHours<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vloginhours: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLoginHours(::core::mem::transmute_copy(&vloginhours)).into()
        }
        unsafe extern "system" fn LoginWorkstations<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LoginWorkstations() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLoginWorkstations<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vloginworkstations: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLoginWorkstations(::core::mem::transmute_copy(&vloginworkstations)).into()
        }
        unsafe extern "system" fn MaxLogins<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MaxLogins() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxLogins<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnmaxlogins: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMaxLogins(::core::mem::transmute_copy(&lnmaxlogins)).into()
        }
        unsafe extern "system" fn MaxStorage<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MaxStorage() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxStorage<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnmaxstorage: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMaxStorage(::core::mem::transmute_copy(&lnmaxstorage)).into()
        }
        unsafe extern "system" fn PasswordExpirationDate<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PasswordExpirationDate() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPasswordExpirationDate<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dapasswordexpirationdate: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPasswordExpirationDate(::core::mem::transmute_copy(&dapasswordexpirationdate)).into()
        }
        unsafe extern "system" fn PasswordMinimumLength<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PasswordMinimumLength() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPasswordMinimumLength<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnpasswordminimumlength: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPasswordMinimumLength(::core::mem::transmute_copy(&lnpasswordminimumlength)).into()
        }
        unsafe extern "system" fn PasswordRequired<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PasswordRequired() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPasswordRequired<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fpasswordrequired: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPasswordRequired(::core::mem::transmute_copy(&fpasswordrequired)).into()
        }
        unsafe extern "system" fn RequireUniquePassword<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RequireUniquePassword() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequireUniquePassword<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frequireuniquepassword: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRequireUniquePassword(::core::mem::transmute_copy(&frequireuniquepassword)).into()
        }
        unsafe extern "system" fn EmailAddress<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EmailAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEmailAddress<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstremailaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEmailAddress(::core::mem::transmute_copy(&bstremailaddress)).into()
        }
        unsafe extern "system" fn HomeDirectory<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HomeDirectory() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHomeDirectory<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrhomedirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHomeDirectory(::core::mem::transmute_copy(&bstrhomedirectory)).into()
        }
        unsafe extern "system" fn Languages<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Languages() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguages<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vlanguages: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLanguages(::core::mem::transmute_copy(&vlanguages)).into()
        }
        unsafe extern "system" fn Profile<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Profile() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProfile<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprofile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProfile(::core::mem::transmute_copy(&bstrprofile)).into()
        }
        unsafe extern "system" fn LoginScript<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LoginScript() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLoginScript<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrloginscript: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLoginScript(::core::mem::transmute_copy(&bstrloginscript)).into()
        }
        unsafe extern "system" fn Picture<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Picture() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPicture<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vpicture: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPicture(::core::mem::transmute_copy(&vpicture)).into()
        }
        unsafe extern "system" fn HomePage<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HomePage() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHomePage<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrhomepage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHomePage(::core::mem::transmute_copy(&bstrhomepage)).into()
        }
        unsafe extern "system" fn Groups<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgroups: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Groups() {
                ::core::result::Result::Ok(ok__) => {
                    *ppgroups = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPassword<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPassword(::core::mem::transmute_copy(&newpassword)).into()
        }
        unsafe extern "system" fn ChangePassword<Identity: ::windows::core::IUnknownImpl, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstroldpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnewpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ChangePassword(::core::mem::transmute_copy(&bstroldpassword), ::core::mem::transmute_copy(&bstrnewpassword)).into()
        }
        Self {
            base: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            BadLoginAddress: BadLoginAddress::<Identity, Impl, OFFSET>,
            BadLoginCount: BadLoginCount::<Identity, Impl, OFFSET>,
            LastLogin: LastLogin::<Identity, Impl, OFFSET>,
            LastLogoff: LastLogoff::<Identity, Impl, OFFSET>,
            LastFailedLogin: LastFailedLogin::<Identity, Impl, OFFSET>,
            PasswordLastChanged: PasswordLastChanged::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            Division: Division::<Identity, Impl, OFFSET>,
            SetDivision: SetDivision::<Identity, Impl, OFFSET>,
            Department: Department::<Identity, Impl, OFFSET>,
            SetDepartment: SetDepartment::<Identity, Impl, OFFSET>,
            EmployeeID: EmployeeID::<Identity, Impl, OFFSET>,
            SetEmployeeID: SetEmployeeID::<Identity, Impl, OFFSET>,
            FullName: FullName::<Identity, Impl, OFFSET>,
            SetFullName: SetFullName::<Identity, Impl, OFFSET>,
            FirstName: FirstName::<Identity, Impl, OFFSET>,
            SetFirstName: SetFirstName::<Identity, Impl, OFFSET>,
            LastName: LastName::<Identity, Impl, OFFSET>,
            SetLastName: SetLastName::<Identity, Impl, OFFSET>,
            OtherName: OtherName::<Identity, Impl, OFFSET>,
            SetOtherName: SetOtherName::<Identity, Impl, OFFSET>,
            NamePrefix: NamePrefix::<Identity, Impl, OFFSET>,
            SetNamePrefix: SetNamePrefix::<Identity, Impl, OFFSET>,
            NameSuffix: NameSuffix::<Identity, Impl, OFFSET>,
            SetNameSuffix: SetNameSuffix::<Identity, Impl, OFFSET>,
            Title: Title::<Identity, Impl, OFFSET>,
            SetTitle: SetTitle::<Identity, Impl, OFFSET>,
            Manager: Manager::<Identity, Impl, OFFSET>,
            SetManager: SetManager::<Identity, Impl, OFFSET>,
            TelephoneHome: TelephoneHome::<Identity, Impl, OFFSET>,
            SetTelephoneHome: SetTelephoneHome::<Identity, Impl, OFFSET>,
            TelephoneMobile: TelephoneMobile::<Identity, Impl, OFFSET>,
            SetTelephoneMobile: SetTelephoneMobile::<Identity, Impl, OFFSET>,
            TelephoneNumber: TelephoneNumber::<Identity, Impl, OFFSET>,
            SetTelephoneNumber: SetTelephoneNumber::<Identity, Impl, OFFSET>,
            TelephonePager: TelephonePager::<Identity, Impl, OFFSET>,
            SetTelephonePager: SetTelephonePager::<Identity, Impl, OFFSET>,
            FaxNumber: FaxNumber::<Identity, Impl, OFFSET>,
            SetFaxNumber: SetFaxNumber::<Identity, Impl, OFFSET>,
            OfficeLocations: OfficeLocations::<Identity, Impl, OFFSET>,
            SetOfficeLocations: SetOfficeLocations::<Identity, Impl, OFFSET>,
            PostalAddresses: PostalAddresses::<Identity, Impl, OFFSET>,
            SetPostalAddresses: SetPostalAddresses::<Identity, Impl, OFFSET>,
            PostalCodes: PostalCodes::<Identity, Impl, OFFSET>,
            SetPostalCodes: SetPostalCodes::<Identity, Impl, OFFSET>,
            SeeAlso: SeeAlso::<Identity, Impl, OFFSET>,
            SetSeeAlso: SetSeeAlso::<Identity, Impl, OFFSET>,
            AccountDisabled: AccountDisabled::<Identity, Impl, OFFSET>,
            SetAccountDisabled: SetAccountDisabled::<Identity, Impl, OFFSET>,
            AccountExpirationDate: AccountExpirationDate::<Identity, Impl, OFFSET>,
            SetAccountExpirationDate: SetAccountExpirationDate::<Identity, Impl, OFFSET>,
            GraceLoginsAllowed: GraceLoginsAllowed::<Identity, Impl, OFFSET>,
            SetGraceLoginsAllowed: SetGraceLoginsAllowed::<Identity, Impl, OFFSET>,
            GraceLoginsRemaining: GraceLoginsRemaining::<Identity, Impl, OFFSET>,
            SetGraceLoginsRemaining: SetGraceLoginsRemaining::<Identity, Impl, OFFSET>,
            IsAccountLocked: IsAccountLocked::<Identity, Impl, OFFSET>,
            SetIsAccountLocked: SetIsAccountLocked::<Identity, Impl, OFFSET>,
            LoginHours: LoginHours::<Identity, Impl, OFFSET>,
            SetLoginHours: SetLoginHours::<Identity, Impl, OFFSET>,
            LoginWorkstations: LoginWorkstations::<Identity, Impl, OFFSET>,
            SetLoginWorkstations: SetLoginWorkstations::<Identity, Impl, OFFSET>,
            MaxLogins: MaxLogins::<Identity, Impl, OFFSET>,
            SetMaxLogins: SetMaxLogins::<Identity, Impl, OFFSET>,
            MaxStorage: MaxStorage::<Identity, Impl, OFFSET>,
            SetMaxStorage: SetMaxStorage::<Identity, Impl, OFFSET>,
            PasswordExpirationDate: PasswordExpirationDate::<Identity, Impl, OFFSET>,
            SetPasswordExpirationDate: SetPasswordExpirationDate::<Identity, Impl, OFFSET>,
            PasswordMinimumLength: PasswordMinimumLength::<Identity, Impl, OFFSET>,
            SetPasswordMinimumLength: SetPasswordMinimumLength::<Identity, Impl, OFFSET>,
            PasswordRequired: PasswordRequired::<Identity, Impl, OFFSET>,
            SetPasswordRequired: SetPasswordRequired::<Identity, Impl, OFFSET>,
            RequireUniquePassword: RequireUniquePassword::<Identity, Impl, OFFSET>,
            SetRequireUniquePassword: SetRequireUniquePassword::<Identity, Impl, OFFSET>,
            EmailAddress: EmailAddress::<Identity, Impl, OFFSET>,
            SetEmailAddress: SetEmailAddress::<Identity, Impl, OFFSET>,
            HomeDirectory: HomeDirectory::<Identity, Impl, OFFSET>,
            SetHomeDirectory: SetHomeDirectory::<Identity, Impl, OFFSET>,
            Languages: Languages::<Identity, Impl, OFFSET>,
            SetLanguages: SetLanguages::<Identity, Impl, OFFSET>,
            Profile: Profile::<Identity, Impl, OFFSET>,
            SetProfile: SetProfile::<Identity, Impl, OFFSET>,
            LoginScript: LoginScript::<Identity, Impl, OFFSET>,
            SetLoginScript: SetLoginScript::<Identity, Impl, OFFSET>,
            Picture: Picture::<Identity, Impl, OFFSET>,
            SetPicture: SetPicture::<Identity, Impl, OFFSET>,
            HomePage: HomePage::<Identity, Impl, OFFSET>,
            SetHomePage: SetHomePage::<Identity, Impl, OFFSET>,
            Groups: Groups::<Identity, Impl, OFFSET>,
            SetPassword: SetPassword::<Identity, Impl, OFFSET>,
            ChangePassword: ChangePassword::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsUser as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IADs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsWinNTSystemInfo_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn UserName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ComputerName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DomainName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn PDC(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsWinNTSystemInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsWinNTSystemInfo_Impl, const OFFSET: isize>() -> IADsWinNTSystemInfo_Vtbl {
        unsafe extern "system" fn UserName<Identity: ::windows::core::IUnknownImpl, Impl: IADsWinNTSystemInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UserName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComputerName<Identity: ::windows::core::IUnknownImpl, Impl: IADsWinNTSystemInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ComputerName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DomainName<Identity: ::windows::core::IUnknownImpl, Impl: IADsWinNTSystemInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DomainName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PDC<Identity: ::windows::core::IUnknownImpl, Impl: IADsWinNTSystemInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PDC() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            UserName: UserName::<Identity, Impl, OFFSET>,
            ComputerName: ComputerName::<Identity, Impl, OFFSET>,
            DomainName: DomainName::<Identity, Impl, OFFSET>,
            PDC: PDC::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsWinNTSystemInfo as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait ICommonQuery_Impl: Sized {
    fn OpenQueryWindow(&mut self, hwndparent: super::super::Foundation::HWND, pquerywnd: *mut OPENQUERYWINDOW, ppdataobject: *mut ::core::option::Option<super::super::System::Com::IDataObject>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl ICommonQuery_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommonQuery_Impl, const OFFSET: isize>() -> ICommonQuery_Vtbl {
        unsafe extern "system" fn OpenQueryWindow<Identity: ::windows::core::IUnknownImpl, Impl: ICommonQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, pquerywnd: *mut OPENQUERYWINDOW, ppdataobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OpenQueryWindow(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&pquerywnd), ::core::mem::transmute_copy(&ppdataobject)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OpenQueryWindow: OpenQueryWindow::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommonQuery as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IDirectoryObject_Impl: Sized {
    fn GetObjectInformation(&mut self) -> ::windows::core::Result<*mut ADS_OBJECT_INFO>;
    fn GetObjectAttributes(&mut self, pattributenames: *const super::super::Foundation::PWSTR, dwnumberattributes: u32, ppattributeentries: *mut *mut ADS_ATTR_INFO, pdwnumattributesreturned: *mut u32) -> ::windows::core::Result<()>;
    fn SetObjectAttributes(&mut self, pattributeentries: *const ADS_ATTR_INFO, dwnumattributes: u32) -> ::windows::core::Result<u32>;
    fn CreateDSObject(&mut self, pszrdnname: super::super::Foundation::PWSTR, pattributeentries: *const ADS_ATTR_INFO, dwnumattributes: u32) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn DeleteDSObject(&mut self, pszrdnname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IDirectoryObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectoryObject_Impl, const OFFSET: isize>() -> IDirectoryObject_Vtbl {
        unsafe extern "system" fn GetObjectInformation<Identity: ::windows::core::IUnknownImpl, Impl: IDirectoryObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppobjinfo: *mut *mut ADS_OBJECT_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetObjectInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *ppobjinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectAttributes<Identity: ::windows::core::IUnknownImpl, Impl: IDirectoryObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattributenames: *const super::super::Foundation::PWSTR, dwnumberattributes: u32, ppattributeentries: *mut *mut ADS_ATTR_INFO, pdwnumattributesreturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetObjectAttributes(::core::mem::transmute_copy(&pattributenames), ::core::mem::transmute_copy(&dwnumberattributes), ::core::mem::transmute_copy(&ppattributeentries), ::core::mem::transmute_copy(&pdwnumattributesreturned)).into()
        }
        unsafe extern "system" fn SetObjectAttributes<Identity: ::windows::core::IUnknownImpl, Impl: IDirectoryObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattributeentries: *const ADS_ATTR_INFO, dwnumattributes: u32, pdwnumattributesmodified: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SetObjectAttributes(::core::mem::transmute_copy(&pattributeentries), ::core::mem::transmute_copy(&dwnumattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwnumattributesmodified = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDSObject<Identity: ::windows::core::IUnknownImpl, Impl: IDirectoryObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszrdnname: super::super::Foundation::PWSTR, pattributeentries: *const ADS_ATTR_INFO, dwnumattributes: u32, ppobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateDSObject(::core::mem::transmute_copy(&pszrdnname), ::core::mem::transmute_copy(&pattributeentries), ::core::mem::transmute_copy(&dwnumattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteDSObject<Identity: ::windows::core::IUnknownImpl, Impl: IDirectoryObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszrdnname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteDSObject(::core::mem::transmute_copy(&pszrdnname)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetObjectInformation: GetObjectInformation::<Identity, Impl, OFFSET>,
            GetObjectAttributes: GetObjectAttributes::<Identity, Impl, OFFSET>,
            SetObjectAttributes: SetObjectAttributes::<Identity, Impl, OFFSET>,
            CreateDSObject: CreateDSObject::<Identity, Impl, OFFSET>,
            DeleteDSObject: DeleteDSObject::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectoryObject as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectorySchemaMgmt_Impl: Sized {
    fn EnumAttributes(&mut self, ppszattrnames: *const super::super::Foundation::PWSTR, dwnumattributes: u32, ppattrdefinition: *const *const ADS_ATTR_DEF, pdwnumattributes: *const u32) -> ::windows::core::Result<()>;
    fn CreateAttributeDefinition(&mut self, pszattributename: super::super::Foundation::PWSTR, pattributedefinition: *const ADS_ATTR_DEF) -> ::windows::core::Result<()>;
    fn WriteAttributeDefinition(&mut self, pszattributename: super::super::Foundation::PWSTR, pattributedefinition: *const ADS_ATTR_DEF) -> ::windows::core::Result<()>;
    fn DeleteAttributeDefinition(&mut self, pszattributename: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn EnumClasses(&mut self, ppszclassnames: *const super::super::Foundation::PWSTR, dwnumclasses: u32, ppclassdefinition: *const *const ADS_CLASS_DEF, pdwnumclasses: *const u32) -> ::windows::core::Result<()>;
    fn WriteClassDefinition(&mut self, pszclassname: super::super::Foundation::PWSTR, pclassdefinition: *const ADS_CLASS_DEF) -> ::windows::core::Result<()>;
    fn CreateClassDefinition(&mut self, pszclassname: super::super::Foundation::PWSTR, pclassdefinition: *const ADS_CLASS_DEF) -> ::windows::core::Result<()>;
    fn DeleteClassDefinition(&mut self, pszclassname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectorySchemaMgmt_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectorySchemaMgmt_Impl, const OFFSET: isize>() -> IDirectorySchemaMgmt_Vtbl {
        unsafe extern "system" fn EnumAttributes<Identity: ::windows::core::IUnknownImpl, Impl: IDirectorySchemaMgmt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszattrnames: *const super::super::Foundation::PWSTR, dwnumattributes: u32, ppattrdefinition: *const *const ADS_ATTR_DEF, pdwnumattributes: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumAttributes(::core::mem::transmute_copy(&ppszattrnames), ::core::mem::transmute_copy(&dwnumattributes), ::core::mem::transmute_copy(&ppattrdefinition), ::core::mem::transmute_copy(&pdwnumattributes)).into()
        }
        unsafe extern "system" fn CreateAttributeDefinition<Identity: ::windows::core::IUnknownImpl, Impl: IDirectorySchemaMgmt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszattributename: super::super::Foundation::PWSTR, pattributedefinition: *const ADS_ATTR_DEF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateAttributeDefinition(::core::mem::transmute_copy(&pszattributename), ::core::mem::transmute_copy(&pattributedefinition)).into()
        }
        unsafe extern "system" fn WriteAttributeDefinition<Identity: ::windows::core::IUnknownImpl, Impl: IDirectorySchemaMgmt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszattributename: super::super::Foundation::PWSTR, pattributedefinition: *const ADS_ATTR_DEF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteAttributeDefinition(::core::mem::transmute_copy(&pszattributename), ::core::mem::transmute_copy(&pattributedefinition)).into()
        }
        unsafe extern "system" fn DeleteAttributeDefinition<Identity: ::windows::core::IUnknownImpl, Impl: IDirectorySchemaMgmt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszattributename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteAttributeDefinition(::core::mem::transmute_copy(&pszattributename)).into()
        }
        unsafe extern "system" fn EnumClasses<Identity: ::windows::core::IUnknownImpl, Impl: IDirectorySchemaMgmt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszclassnames: *const super::super::Foundation::PWSTR, dwnumclasses: u32, ppclassdefinition: *const *const ADS_CLASS_DEF, pdwnumclasses: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumClasses(::core::mem::transmute_copy(&ppszclassnames), ::core::mem::transmute_copy(&dwnumclasses), ::core::mem::transmute_copy(&ppclassdefinition), ::core::mem::transmute_copy(&pdwnumclasses)).into()
        }
        unsafe extern "system" fn WriteClassDefinition<Identity: ::windows::core::IUnknownImpl, Impl: IDirectorySchemaMgmt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszclassname: super::super::Foundation::PWSTR, pclassdefinition: *const ADS_CLASS_DEF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteClassDefinition(::core::mem::transmute_copy(&pszclassname), ::core::mem::transmute_copy(&pclassdefinition)).into()
        }
        unsafe extern "system" fn CreateClassDefinition<Identity: ::windows::core::IUnknownImpl, Impl: IDirectorySchemaMgmt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszclassname: super::super::Foundation::PWSTR, pclassdefinition: *const ADS_CLASS_DEF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateClassDefinition(::core::mem::transmute_copy(&pszclassname), ::core::mem::transmute_copy(&pclassdefinition)).into()
        }
        unsafe extern "system" fn DeleteClassDefinition<Identity: ::windows::core::IUnknownImpl, Impl: IDirectorySchemaMgmt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszclassname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteClassDefinition(::core::mem::transmute_copy(&pszclassname)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            EnumAttributes: EnumAttributes::<Identity, Impl, OFFSET>,
            CreateAttributeDefinition: CreateAttributeDefinition::<Identity, Impl, OFFSET>,
            WriteAttributeDefinition: WriteAttributeDefinition::<Identity, Impl, OFFSET>,
            DeleteAttributeDefinition: DeleteAttributeDefinition::<Identity, Impl, OFFSET>,
            EnumClasses: EnumClasses::<Identity, Impl, OFFSET>,
            WriteClassDefinition: WriteClassDefinition::<Identity, Impl, OFFSET>,
            CreateClassDefinition: CreateClassDefinition::<Identity, Impl, OFFSET>,
            DeleteClassDefinition: DeleteClassDefinition::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectorySchemaMgmt as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectorySearch_Impl: Sized {
    fn SetSearchPreference(&mut self, psearchprefs: *const ads_searchpref_info, dwnumprefs: u32) -> ::windows::core::Result<()>;
    fn ExecuteSearch(&mut self, pszsearchfilter: super::super::Foundation::PWSTR, pattributenames: *const super::super::Foundation::PWSTR, dwnumberattributes: u32) -> ::windows::core::Result<isize>;
    fn AbandonSearch(&mut self, phsearchresult: isize) -> ::windows::core::Result<()>;
    fn GetFirstRow(&mut self, hsearchresult: isize) -> ::windows::core::HRESULT;
    fn GetNextRow(&mut self, hsearchresult: isize) -> ::windows::core::HRESULT;
    fn GetPreviousRow(&mut self, hsearchresult: isize) -> ::windows::core::HRESULT;
    fn GetNextColumnName(&mut self, hsearchhandle: isize, ppszcolumnname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
    fn GetColumn(&mut self, hsearchresult: isize, szcolumnname: super::super::Foundation::PWSTR) -> ::windows::core::Result<ads_search_column>;
    fn FreeColumn(&mut self, psearchcolumn: *const ads_search_column) -> ::windows::core::Result<()>;
    fn CloseSearchHandle(&mut self, hsearchresult: isize) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectorySearch_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectorySearch_Impl, const OFFSET: isize>() -> IDirectorySearch_Vtbl {
        unsafe extern "system" fn SetSearchPreference<Identity: ::windows::core::IUnknownImpl, Impl: IDirectorySearch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psearchprefs: *const ads_searchpref_info, dwnumprefs: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSearchPreference(::core::mem::transmute_copy(&psearchprefs), ::core::mem::transmute_copy(&dwnumprefs)).into()
        }
        unsafe extern "system" fn ExecuteSearch<Identity: ::windows::core::IUnknownImpl, Impl: IDirectorySearch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsearchfilter: super::super::Foundation::PWSTR, pattributenames: *const super::super::Foundation::PWSTR, dwnumberattributes: u32, phsearchresult: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ExecuteSearch(::core::mem::transmute_copy(&pszsearchfilter), ::core::mem::transmute_copy(&pattributenames), ::core::mem::transmute_copy(&dwnumberattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *phsearchresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AbandonSearch<Identity: ::windows::core::IUnknownImpl, Impl: IDirectorySearch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phsearchresult: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AbandonSearch(::core::mem::transmute_copy(&phsearchresult)).into()
        }
        unsafe extern "system" fn GetFirstRow<Identity: ::windows::core::IUnknownImpl, Impl: IDirectorySearch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hsearchresult: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFirstRow(::core::mem::transmute_copy(&hsearchresult))
        }
        unsafe extern "system" fn GetNextRow<Identity: ::windows::core::IUnknownImpl, Impl: IDirectorySearch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hsearchresult: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetNextRow(::core::mem::transmute_copy(&hsearchresult))
        }
        unsafe extern "system" fn GetPreviousRow<Identity: ::windows::core::IUnknownImpl, Impl: IDirectorySearch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hsearchresult: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPreviousRow(::core::mem::transmute_copy(&hsearchresult))
        }
        unsafe extern "system" fn GetNextColumnName<Identity: ::windows::core::IUnknownImpl, Impl: IDirectorySearch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hsearchhandle: isize, ppszcolumnname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetNextColumnName(::core::mem::transmute_copy(&hsearchhandle), ::core::mem::transmute_copy(&ppszcolumnname))
        }
        unsafe extern "system" fn GetColumn<Identity: ::windows::core::IUnknownImpl, Impl: IDirectorySearch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hsearchresult: isize, szcolumnname: super::super::Foundation::PWSTR, psearchcolumn: *mut ads_search_column) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetColumn(::core::mem::transmute_copy(&hsearchresult), ::core::mem::transmute_copy(&szcolumnname)) {
                ::core::result::Result::Ok(ok__) => {
                    *psearchcolumn = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FreeColumn<Identity: ::windows::core::IUnknownImpl, Impl: IDirectorySearch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psearchcolumn: *const ads_search_column) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FreeColumn(::core::mem::transmute_copy(&psearchcolumn)).into()
        }
        unsafe extern "system" fn CloseSearchHandle<Identity: ::windows::core::IUnknownImpl, Impl: IDirectorySearch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hsearchresult: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CloseSearchHandle(::core::mem::transmute_copy(&hsearchresult)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetSearchPreference: SetSearchPreference::<Identity, Impl, OFFSET>,
            ExecuteSearch: ExecuteSearch::<Identity, Impl, OFFSET>,
            AbandonSearch: AbandonSearch::<Identity, Impl, OFFSET>,
            GetFirstRow: GetFirstRow::<Identity, Impl, OFFSET>,
            GetNextRow: GetNextRow::<Identity, Impl, OFFSET>,
            GetPreviousRow: GetPreviousRow::<Identity, Impl, OFFSET>,
            GetNextColumnName: GetNextColumnName::<Identity, Impl, OFFSET>,
            GetColumn: GetColumn::<Identity, Impl, OFFSET>,
            FreeColumn: FreeColumn::<Identity, Impl, OFFSET>,
            CloseSearchHandle: CloseSearchHandle::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectorySearch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IDsAdminCreateObj_Impl: Sized {
    fn Initialize(&mut self, padscontainerobj: &::core::option::Option<IADsContainer>, padscopysource: &::core::option::Option<IADs>, lpszclassname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn CreateModal(&mut self, hwndparent: super::super::Foundation::HWND) -> ::windows::core::Result<IADs>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IDsAdminCreateObj_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDsAdminCreateObj_Impl, const OFFSET: isize>() -> IDsAdminCreateObj_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IDsAdminCreateObj_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padscontainerobj: ::windows::core::RawPtr, padscopysource: ::windows::core::RawPtr, lpszclassname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&padscontainerobj), ::core::mem::transmute(&padscopysource), ::core::mem::transmute_copy(&lpszclassname)).into()
        }
        unsafe extern "system" fn CreateModal<Identity: ::windows::core::IUnknownImpl, Impl: IDsAdminCreateObj_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ppadsobj: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateModal(::core::mem::transmute_copy(&hwndparent)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppadsobj = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            CreateModal: CreateModal::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDsAdminCreateObj as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDsAdminNewObj_Impl: Sized {
    fn SetButtons(&mut self, ncurrindex: u32, bvalid: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetPageCounts(&mut self, pntotal: *mut i32, pnstartindex: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDsAdminNewObj_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDsAdminNewObj_Impl, const OFFSET: isize>() -> IDsAdminNewObj_Vtbl {
        unsafe extern "system" fn SetButtons<Identity: ::windows::core::IUnknownImpl, Impl: IDsAdminNewObj_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncurrindex: u32, bvalid: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetButtons(::core::mem::transmute_copy(&ncurrindex), ::core::mem::transmute_copy(&bvalid)).into()
        }
        unsafe extern "system" fn GetPageCounts<Identity: ::windows::core::IUnknownImpl, Impl: IDsAdminNewObj_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pntotal: *mut i32, pnstartindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPageCounts(::core::mem::transmute_copy(&pntotal), ::core::mem::transmute_copy(&pnstartindex)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetButtons: SetButtons::<Identity, Impl, OFFSET>,
            GetPageCounts: GetPageCounts::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDsAdminNewObj as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IDsAdminNewObjExt_Impl: Sized {
    fn Initialize(&mut self, padscontainerobj: &::core::option::Option<IADsContainer>, padscopysource: &::core::option::Option<IADs>, lpszclassname: super::super::Foundation::PWSTR, pdsadminnewobj: &::core::option::Option<IDsAdminNewObj>, pdispinfo: *mut DSA_NEWOBJ_DISPINFO) -> ::windows::core::Result<()>;
    fn AddPages(&mut self, lpfnaddpage: &super::super::UI::Controls::LPFNSVADDPROPSHEETPAGE, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<()>;
    fn SetObject(&mut self, padsobj: &::core::option::Option<IADs>) -> ::windows::core::Result<()>;
    fn WriteData(&mut self, hwnd: super::super::Foundation::HWND, ucontext: u32) -> ::windows::core::Result<()>;
    fn OnError(&mut self, hwnd: super::super::Foundation::HWND, hr: ::windows::core::HRESULT, ucontext: u32) -> ::windows::core::Result<()>;
    fn GetSummaryInfo(&mut self, pbstrtext: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl IDsAdminNewObjExt_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDsAdminNewObjExt_Impl, const OFFSET: isize>() -> IDsAdminNewObjExt_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IDsAdminNewObjExt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padscontainerobj: ::windows::core::RawPtr, padscopysource: ::windows::core::RawPtr, lpszclassname: super::super::Foundation::PWSTR, pdsadminnewobj: ::windows::core::RawPtr, pdispinfo: *mut DSA_NEWOBJ_DISPINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&padscontainerobj), ::core::mem::transmute(&padscopysource), ::core::mem::transmute_copy(&lpszclassname), ::core::mem::transmute(&pdsadminnewobj), ::core::mem::transmute_copy(&pdispinfo)).into()
        }
        unsafe extern "system" fn AddPages<Identity: ::windows::core::IUnknownImpl, Impl: IDsAdminNewObjExt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpfnaddpage: ::windows::core::RawPtr, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddPages(::core::mem::transmute_copy(&lpfnaddpage), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn SetObject<Identity: ::windows::core::IUnknownImpl, Impl: IDsAdminNewObjExt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padsobj: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetObject(::core::mem::transmute(&padsobj)).into()
        }
        unsafe extern "system" fn WriteData<Identity: ::windows::core::IUnknownImpl, Impl: IDsAdminNewObjExt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, ucontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteData(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&ucontext)).into()
        }
        unsafe extern "system" fn OnError<Identity: ::windows::core::IUnknownImpl, Impl: IDsAdminNewObjExt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, hr: ::windows::core::HRESULT, ucontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnError(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&hr), ::core::mem::transmute_copy(&ucontext)).into()
        }
        unsafe extern "system" fn GetSummaryInfo<Identity: ::windows::core::IUnknownImpl, Impl: IDsAdminNewObjExt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSummaryInfo(::core::mem::transmute_copy(&pbstrtext)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            AddPages: AddPages::<Identity, Impl, OFFSET>,
            SetObject: SetObject::<Identity, Impl, OFFSET>,
            WriteData: WriteData::<Identity, Impl, OFFSET>,
            OnError: OnError::<Identity, Impl, OFFSET>,
            GetSummaryInfo: GetSummaryInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDsAdminNewObjExt as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDsAdminNewObjPrimarySite_Impl: Sized {
    fn CreateNew(&mut self, pszname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Commit(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDsAdminNewObjPrimarySite_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDsAdminNewObjPrimarySite_Impl, const OFFSET: isize>() -> IDsAdminNewObjPrimarySite_Vtbl {
        unsafe extern "system" fn CreateNew<Identity: ::windows::core::IUnknownImpl, Impl: IDsAdminNewObjPrimarySite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateNew(::core::mem::transmute_copy(&pszname)).into()
        }
        unsafe extern "system" fn Commit<Identity: ::windows::core::IUnknownImpl, Impl: IDsAdminNewObjPrimarySite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Commit().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateNew: CreateNew::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDsAdminNewObjPrimarySite as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IDsAdminNotifyHandler_Impl: Sized {
    fn Initialize(&mut self, pextrainfo: &::core::option::Option<super::super::System::Com::IDataObject>, pueventflags: *mut u32) -> ::windows::core::Result<()>;
    fn Begin(&mut self, uevent: u32, parg1: &::core::option::Option<super::super::System::Com::IDataObject>, parg2: &::core::option::Option<super::super::System::Com::IDataObject>, puflags: *mut u32, pbstr: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Notify(&mut self, nitem: u32, uflags: u32) -> ::windows::core::Result<()>;
    fn End(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IDsAdminNotifyHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDsAdminNotifyHandler_Impl, const OFFSET: isize>() -> IDsAdminNotifyHandler_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IDsAdminNotifyHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pextrainfo: ::windows::core::RawPtr, pueventflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&pextrainfo), ::core::mem::transmute_copy(&pueventflags)).into()
        }
        unsafe extern "system" fn Begin<Identity: ::windows::core::IUnknownImpl, Impl: IDsAdminNotifyHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uevent: u32, parg1: ::windows::core::RawPtr, parg2: ::windows::core::RawPtr, puflags: *mut u32, pbstr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Begin(::core::mem::transmute_copy(&uevent), ::core::mem::transmute(&parg1), ::core::mem::transmute(&parg2), ::core::mem::transmute_copy(&puflags), ::core::mem::transmute_copy(&pbstr)).into()
        }
        unsafe extern "system" fn Notify<Identity: ::windows::core::IUnknownImpl, Impl: IDsAdminNotifyHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nitem: u32, uflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Notify(::core::mem::transmute_copy(&nitem), ::core::mem::transmute_copy(&uflags)).into()
        }
        unsafe extern "system" fn End<Identity: ::windows::core::IUnknownImpl, Impl: IDsAdminNotifyHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).End().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Begin: Begin::<Identity, Impl, OFFSET>,
            Notify: Notify::<Identity, Impl, OFFSET>,
            End: End::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDsAdminNotifyHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDsBrowseDomainTree_Impl: Sized {
    fn BrowseTo(&mut self, hwndparent: super::super::Foundation::HWND, ppsztargetpath: *mut super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::Result<()>;
    fn GetDomains(&mut self, ppdomaintree: *mut *mut DOMAIN_TREE, dwflags: u32) -> ::windows::core::Result<()>;
    fn FreeDomains(&mut self, ppdomaintree: *mut *mut DOMAIN_TREE) -> ::windows::core::Result<()>;
    fn FlushCachedDomains(&mut self) -> ::windows::core::Result<()>;
    fn SetComputer(&mut self, pszcomputername: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR, pszpassword: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDsBrowseDomainTree_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDsBrowseDomainTree_Impl, const OFFSET: isize>() -> IDsBrowseDomainTree_Vtbl {
        unsafe extern "system" fn BrowseTo<Identity: ::windows::core::IUnknownImpl, Impl: IDsBrowseDomainTree_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ppsztargetpath: *mut super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BrowseTo(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&ppsztargetpath), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetDomains<Identity: ::windows::core::IUnknownImpl, Impl: IDsBrowseDomainTree_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdomaintree: *mut *mut DOMAIN_TREE, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDomains(::core::mem::transmute_copy(&ppdomaintree), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn FreeDomains<Identity: ::windows::core::IUnknownImpl, Impl: IDsBrowseDomainTree_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdomaintree: *mut *mut DOMAIN_TREE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FreeDomains(::core::mem::transmute_copy(&ppdomaintree)).into()
        }
        unsafe extern "system" fn FlushCachedDomains<Identity: ::windows::core::IUnknownImpl, Impl: IDsBrowseDomainTree_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FlushCachedDomains().into()
        }
        unsafe extern "system" fn SetComputer<Identity: ::windows::core::IUnknownImpl, Impl: IDsBrowseDomainTree_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcomputername: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR, pszpassword: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetComputer(::core::mem::transmute_copy(&pszcomputername), ::core::mem::transmute_copy(&pszusername), ::core::mem::transmute_copy(&pszpassword)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            BrowseTo: BrowseTo::<Identity, Impl, OFFSET>,
            GetDomains: GetDomains::<Identity, Impl, OFFSET>,
            FreeDomains: FreeDomains::<Identity, Impl, OFFSET>,
            FlushCachedDomains: FlushCachedDomains::<Identity, Impl, OFFSET>,
            SetComputer: SetComputer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDsBrowseDomainTree as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IDsDisplaySpecifier_Impl: Sized {
    fn SetServer(&mut self, pszserver: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR, pszpassword: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::Result<()>;
    fn SetLanguageID(&mut self, langid: u16) -> ::windows::core::Result<()>;
    fn GetDisplaySpecifier(&mut self, pszobjectclass: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetIconLocation(&mut self, pszobjectclass: super::super::Foundation::PWSTR, dwflags: u32, pszbuffer: super::super::Foundation::PWSTR, cchbuffer: i32, presid: *mut i32) -> ::windows::core::Result<()>;
    fn GetIcon(&mut self, pszobjectclass: super::super::Foundation::PWSTR, dwflags: u32, cxicon: i32, cyicon: i32) -> super::super::UI::WindowsAndMessaging::HICON;
    fn GetFriendlyClassName(&mut self, pszobjectclass: super::super::Foundation::PWSTR, pszbuffer: super::super::Foundation::PWSTR, cchbuffer: i32) -> ::windows::core::Result<()>;
    fn GetFriendlyAttributeName(&mut self, pszobjectclass: super::super::Foundation::PWSTR, pszattributename: super::super::Foundation::PWSTR, pszbuffer: super::super::Foundation::PWSTR, cchbuffer: u32) -> ::windows::core::Result<()>;
    fn IsClassContainer(&mut self, pszobjectclass: super::super::Foundation::PWSTR, pszadspath: super::super::Foundation::PWSTR, dwflags: u32) -> super::super::Foundation::BOOL;
    fn GetClassCreationInfo(&mut self, pszobjectclass: super::super::Foundation::PWSTR, ppdscci: *mut *mut DSCLASSCREATIONINFO) -> ::windows::core::Result<()>;
    fn EnumClassAttributes(&mut self, pszobjectclass: super::super::Foundation::PWSTR, pcbenum: &LPDSENUMATTRIBUTES, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<()>;
    fn GetAttributeADsType(&mut self, pszattributename: super::super::Foundation::PWSTR) -> ADSTYPEENUM;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IDsDisplaySpecifier_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDsDisplaySpecifier_Impl, const OFFSET: isize>() -> IDsDisplaySpecifier_Vtbl {
        unsafe extern "system" fn SetServer<Identity: ::windows::core::IUnknownImpl, Impl: IDsDisplaySpecifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszserver: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR, pszpassword: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetServer(::core::mem::transmute_copy(&pszserver), ::core::mem::transmute_copy(&pszusername), ::core::mem::transmute_copy(&pszpassword), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn SetLanguageID<Identity: ::windows::core::IUnknownImpl, Impl: IDsDisplaySpecifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLanguageID(::core::mem::transmute_copy(&langid)).into()
        }
        unsafe extern "system" fn GetDisplaySpecifier<Identity: ::windows::core::IUnknownImpl, Impl: IDsDisplaySpecifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectclass: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDisplaySpecifier(::core::mem::transmute_copy(&pszobjectclass), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn GetIconLocation<Identity: ::windows::core::IUnknownImpl, Impl: IDsDisplaySpecifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectclass: super::super::Foundation::PWSTR, dwflags: u32, pszbuffer: super::super::Foundation::PWSTR, cchbuffer: i32, presid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetIconLocation(::core::mem::transmute_copy(&pszobjectclass), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pszbuffer), ::core::mem::transmute_copy(&cchbuffer), ::core::mem::transmute_copy(&presid)).into()
        }
        unsafe extern "system" fn GetIcon<Identity: ::windows::core::IUnknownImpl, Impl: IDsDisplaySpecifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectclass: super::super::Foundation::PWSTR, dwflags: u32, cxicon: i32, cyicon: i32) -> super::super::UI::WindowsAndMessaging::HICON {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetIcon(::core::mem::transmute_copy(&pszobjectclass), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&cxicon), ::core::mem::transmute_copy(&cyicon))
        }
        unsafe extern "system" fn GetFriendlyClassName<Identity: ::windows::core::IUnknownImpl, Impl: IDsDisplaySpecifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectclass: super::super::Foundation::PWSTR, pszbuffer: super::super::Foundation::PWSTR, cchbuffer: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFriendlyClassName(::core::mem::transmute_copy(&pszobjectclass), ::core::mem::transmute_copy(&pszbuffer), ::core::mem::transmute_copy(&cchbuffer)).into()
        }
        unsafe extern "system" fn GetFriendlyAttributeName<Identity: ::windows::core::IUnknownImpl, Impl: IDsDisplaySpecifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectclass: super::super::Foundation::PWSTR, pszattributename: super::super::Foundation::PWSTR, pszbuffer: super::super::Foundation::PWSTR, cchbuffer: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFriendlyAttributeName(::core::mem::transmute_copy(&pszobjectclass), ::core::mem::transmute_copy(&pszattributename), ::core::mem::transmute_copy(&pszbuffer), ::core::mem::transmute_copy(&cchbuffer)).into()
        }
        unsafe extern "system" fn IsClassContainer<Identity: ::windows::core::IUnknownImpl, Impl: IDsDisplaySpecifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectclass: super::super::Foundation::PWSTR, pszadspath: super::super::Foundation::PWSTR, dwflags: u32) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsClassContainer(::core::mem::transmute_copy(&pszobjectclass), ::core::mem::transmute_copy(&pszadspath), ::core::mem::transmute_copy(&dwflags))
        }
        unsafe extern "system" fn GetClassCreationInfo<Identity: ::windows::core::IUnknownImpl, Impl: IDsDisplaySpecifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectclass: super::super::Foundation::PWSTR, ppdscci: *mut *mut DSCLASSCREATIONINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetClassCreationInfo(::core::mem::transmute_copy(&pszobjectclass), ::core::mem::transmute_copy(&ppdscci)).into()
        }
        unsafe extern "system" fn EnumClassAttributes<Identity: ::windows::core::IUnknownImpl, Impl: IDsDisplaySpecifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectclass: super::super::Foundation::PWSTR, pcbenum: ::windows::core::RawPtr, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumClassAttributes(::core::mem::transmute_copy(&pszobjectclass), ::core::mem::transmute_copy(&pcbenum), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn GetAttributeADsType<Identity: ::windows::core::IUnknownImpl, Impl: IDsDisplaySpecifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszattributename: super::super::Foundation::PWSTR) -> ADSTYPEENUM {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAttributeADsType(::core::mem::transmute_copy(&pszattributename))
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetServer: SetServer::<Identity, Impl, OFFSET>,
            SetLanguageID: SetLanguageID::<Identity, Impl, OFFSET>,
            GetDisplaySpecifier: GetDisplaySpecifier::<Identity, Impl, OFFSET>,
            GetIconLocation: GetIconLocation::<Identity, Impl, OFFSET>,
            GetIcon: GetIcon::<Identity, Impl, OFFSET>,
            GetFriendlyClassName: GetFriendlyClassName::<Identity, Impl, OFFSET>,
            GetFriendlyAttributeName: GetFriendlyAttributeName::<Identity, Impl, OFFSET>,
            IsClassContainer: IsClassContainer::<Identity, Impl, OFFSET>,
            GetClassCreationInfo: GetClassCreationInfo::<Identity, Impl, OFFSET>,
            EnumClassAttributes: EnumClassAttributes::<Identity, Impl, OFFSET>,
            GetAttributeADsType: GetAttributeADsType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDsDisplaySpecifier as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IDsObjectPicker_Impl: Sized {
    fn Initialize(&mut self, pinitinfo: *mut DSOP_INIT_INFO) -> ::windows::core::Result<()>;
    fn InvokeDialog(&mut self, hwndparent: super::super::Foundation::HWND) -> ::windows::core::Result<super::super::System::Com::IDataObject>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IDsObjectPicker_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDsObjectPicker_Impl, const OFFSET: isize>() -> IDsObjectPicker_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IDsObjectPicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinitinfo: *mut DSOP_INIT_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&pinitinfo)).into()
        }
        unsafe extern "system" fn InvokeDialog<Identity: ::windows::core::IUnknownImpl, Impl: IDsObjectPicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ppdoselections: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InvokeDialog(::core::mem::transmute_copy(&hwndparent)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdoselections = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            InvokeDialog: InvokeDialog::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDsObjectPicker as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IDsObjectPickerCredentials_Impl: Sized + IDsObjectPicker_Impl {
    fn SetCredentials(&mut self, szusername: super::super::Foundation::PWSTR, szpassword: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IDsObjectPickerCredentials_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDsObjectPickerCredentials_Impl, const OFFSET: isize>() -> IDsObjectPickerCredentials_Vtbl {
        unsafe extern "system" fn SetCredentials<Identity: ::windows::core::IUnknownImpl, Impl: IDsObjectPickerCredentials_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szusername: super::super::Foundation::PWSTR, szpassword: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCredentials(::core::mem::transmute_copy(&szusername), ::core::mem::transmute_copy(&szpassword)).into()
        }
        Self { base: IDsObjectPicker_Vtbl::new::<Identity, Impl, OFFSET>(), SetCredentials: SetCredentials::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDsObjectPickerCredentials as ::windows::core::Interface>::IID || iid == &<IDsObjectPicker as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IPersistQuery_Impl: Sized + super::super::System::Com::IPersist_Impl {
    fn WriteString(&mut self, psection: super::super::Foundation::PWSTR, pvaluename: super::super::Foundation::PWSTR, pvalue: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn ReadString(&mut self, psection: super::super::Foundation::PWSTR, pvaluename: super::super::Foundation::PWSTR, pbuffer: super::super::Foundation::PWSTR, cchbuffer: i32) -> ::windows::core::Result<()>;
    fn WriteInt(&mut self, psection: super::super::Foundation::PWSTR, pvaluename: super::super::Foundation::PWSTR, value: i32) -> ::windows::core::Result<()>;
    fn ReadInt(&mut self, psection: super::super::Foundation::PWSTR, pvaluename: super::super::Foundation::PWSTR, pvalue: *mut i32) -> ::windows::core::Result<()>;
    fn WriteStruct(&mut self, psection: super::super::Foundation::PWSTR, pvaluename: super::super::Foundation::PWSTR, pstruct: *mut ::core::ffi::c_void, cbstruct: u32) -> ::windows::core::Result<()>;
    fn ReadStruct(&mut self, psection: super::super::Foundation::PWSTR, pvaluename: super::super::Foundation::PWSTR, pstruct: *mut ::core::ffi::c_void, cbstruct: u32) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IPersistQuery_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPersistQuery_Impl, const OFFSET: isize>() -> IPersistQuery_Vtbl {
        unsafe extern "system" fn WriteString<Identity: ::windows::core::IUnknownImpl, Impl: IPersistQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psection: super::super::Foundation::PWSTR, pvaluename: super::super::Foundation::PWSTR, pvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteString(::core::mem::transmute_copy(&psection), ::core::mem::transmute_copy(&pvaluename), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn ReadString<Identity: ::windows::core::IUnknownImpl, Impl: IPersistQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psection: super::super::Foundation::PWSTR, pvaluename: super::super::Foundation::PWSTR, pbuffer: super::super::Foundation::PWSTR, cchbuffer: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReadString(::core::mem::transmute_copy(&psection), ::core::mem::transmute_copy(&pvaluename), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&cchbuffer)).into()
        }
        unsafe extern "system" fn WriteInt<Identity: ::windows::core::IUnknownImpl, Impl: IPersistQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psection: super::super::Foundation::PWSTR, pvaluename: super::super::Foundation::PWSTR, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteInt(::core::mem::transmute_copy(&psection), ::core::mem::transmute_copy(&pvaluename), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ReadInt<Identity: ::windows::core::IUnknownImpl, Impl: IPersistQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psection: super::super::Foundation::PWSTR, pvaluename: super::super::Foundation::PWSTR, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReadInt(::core::mem::transmute_copy(&psection), ::core::mem::transmute_copy(&pvaluename), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn WriteStruct<Identity: ::windows::core::IUnknownImpl, Impl: IPersistQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psection: super::super::Foundation::PWSTR, pvaluename: super::super::Foundation::PWSTR, pstruct: *mut ::core::ffi::c_void, cbstruct: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteStruct(::core::mem::transmute_copy(&psection), ::core::mem::transmute_copy(&pvaluename), ::core::mem::transmute_copy(&pstruct), ::core::mem::transmute_copy(&cbstruct)).into()
        }
        unsafe extern "system" fn ReadStruct<Identity: ::windows::core::IUnknownImpl, Impl: IPersistQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psection: super::super::Foundation::PWSTR, pvaluename: super::super::Foundation::PWSTR, pstruct: *mut ::core::ffi::c_void, cbstruct: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReadStruct(::core::mem::transmute_copy(&psection), ::core::mem::transmute_copy(&pvaluename), ::core::mem::transmute_copy(&pstruct), ::core::mem::transmute_copy(&cbstruct)).into()
        }
        unsafe extern "system" fn Clear<Identity: ::windows::core::IUnknownImpl, Impl: IPersistQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Clear().into()
        }
        Self {
            base: super::super::System::Com::IPersist_Vtbl::new::<Identity, Impl, OFFSET>(),
            WriteString: WriteString::<Identity, Impl, OFFSET>,
            ReadString: ReadString::<Identity, Impl, OFFSET>,
            WriteInt: WriteInt::<Identity, Impl, OFFSET>,
            ReadInt: ReadInt::<Identity, Impl, OFFSET>,
            WriteStruct: WriteStruct::<Identity, Impl, OFFSET>,
            ReadStruct: ReadStruct::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPersistQuery as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IPersist as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrivateDispatch_Impl: Sized {
    fn ADSIInitializeDispatchManager(&mut self, dwextensionid: i32) -> ::windows::core::Result<()>;
    fn ADSIGetTypeInfoCount(&mut self) -> ::windows::core::Result<u32>;
    fn ADSIGetTypeInfo(&mut self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo>;
    fn ADSIGetIDsOfNames(&mut self, riid: *const ::windows::core::GUID, rgsznames: *const *const u16, cnames: u32, lcid: u32) -> ::windows::core::Result<i32>;
    fn ADSIInvoke(&mut self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrivateDispatch_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrivateDispatch_Impl, const OFFSET: isize>() -> IPrivateDispatch_Vtbl {
        unsafe extern "system" fn ADSIInitializeDispatchManager<Identity: ::windows::core::IUnknownImpl, Impl: IPrivateDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwextensionid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ADSIInitializeDispatchManager(::core::mem::transmute_copy(&dwextensionid)).into()
        }
        unsafe extern "system" fn ADSIGetTypeInfoCount<Identity: ::windows::core::IUnknownImpl, Impl: IPrivateDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ADSIGetTypeInfoCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pctinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ADSIGetTypeInfo<Identity: ::windows::core::IUnknownImpl, Impl: IPrivateDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ADSIGetTypeInfo(::core::mem::transmute_copy(&itinfo), ::core::mem::transmute_copy(&lcid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ADSIGetIDsOfNames<Identity: ::windows::core::IUnknownImpl, Impl: IPrivateDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const *const u16, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ADSIGetIDsOfNames(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&rgsznames), ::core::mem::transmute_copy(&cnames), ::core::mem::transmute_copy(&lcid)) {
                ::core::result::Result::Ok(ok__) => {
                    *rgdispid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ADSIInvoke<Identity: ::windows::core::IUnknownImpl, Impl: IPrivateDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ADSIInvoke(::core::mem::transmute_copy(&dispidmember), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&lcid), ::core::mem::transmute_copy(&wflags), ::core::mem::transmute_copy(&pdispparams), ::core::mem::transmute_copy(&pvarresult), ::core::mem::transmute_copy(&pexcepinfo), ::core::mem::transmute_copy(&puargerr)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ADSIInitializeDispatchManager: ADSIInitializeDispatchManager::<Identity, Impl, OFFSET>,
            ADSIGetTypeInfoCount: ADSIGetTypeInfoCount::<Identity, Impl, OFFSET>,
            ADSIGetTypeInfo: ADSIGetTypeInfo::<Identity, Impl, OFFSET>,
            ADSIGetIDsOfNames: ADSIGetIDsOfNames::<Identity, Impl, OFFSET>,
            ADSIInvoke: ADSIInvoke::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrivateDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPrivateUnknown_Impl: Sized {
    fn ADSIInitializeObject(&mut self, lpszusername: &super::super::Foundation::BSTR, lpszpassword: &super::super::Foundation::BSTR, lnreserved: i32) -> ::windows::core::Result<()>;
    fn ADSIReleaseObject(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPrivateUnknown_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrivateUnknown_Impl, const OFFSET: isize>() -> IPrivateUnknown_Vtbl {
        unsafe extern "system" fn ADSIInitializeObject<Identity: ::windows::core::IUnknownImpl, Impl: IPrivateUnknown_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lpszpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lnreserved: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ADSIInitializeObject(::core::mem::transmute_copy(&lpszusername), ::core::mem::transmute_copy(&lpszpassword), ::core::mem::transmute_copy(&lnreserved)).into()
        }
        unsafe extern "system" fn ADSIReleaseObject<Identity: ::windows::core::IUnknownImpl, Impl: IPrivateUnknown_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ADSIReleaseObject().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ADSIInitializeObject: ADSIInitializeObject::<Identity, Impl, OFFSET>,
            ADSIReleaseObject: ADSIReleaseObject::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrivateUnknown as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IQueryForm_Impl: Sized {
    fn Initialize(&mut self, hkform: super::super::System::Registry::HKEY) -> ::windows::core::Result<()>;
    fn AddForms(&mut self, paddformsproc: &LPCQADDFORMSPROC, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<()>;
    fn AddPages(&mut self, paddpagesproc: &LPCQADDPAGESPROC, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry", feature = "Win32_UI_WindowsAndMessaging"))]
impl IQueryForm_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IQueryForm_Impl, const OFFSET: isize>() -> IQueryForm_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IQueryForm_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkform: super::super::System::Registry::HKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&hkform)).into()
        }
        unsafe extern "system" fn AddForms<Identity: ::windows::core::IUnknownImpl, Impl: IQueryForm_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddformsproc: ::windows::core::RawPtr, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddForms(::core::mem::transmute_copy(&paddformsproc), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn AddPages<Identity: ::windows::core::IUnknownImpl, Impl: IQueryForm_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddpagesproc: ::windows::core::RawPtr, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddPages(::core::mem::transmute_copy(&paddpagesproc), ::core::mem::transmute_copy(&lparam)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            AddForms: AddForms::<Identity, Impl, OFFSET>,
            AddPages: AddPages::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IQueryForm as ::windows::core::Interface>::IID
    }
}
