#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsImpl: Sized + IDispatchImpl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Class(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GUID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ADsPath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Parent(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Schema(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetInfo(&mut self) -> ::windows::core::Result<()>;
    fn SetInfo(&mut self) -> ::windows::core::Result<()>;
    fn Get(&mut self, bstrname: super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Put(&mut self, bstrname: super::super::Foundation::BSTR, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GetEx(&mut self, bstrname: super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn PutEx(&mut self, lncontrolcode: i32, bstrname: super::super::Foundation::BSTR, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GetInfoEx(&mut self, vproperties: super::super::System::Com::VARIANT, lnreserved: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsVtbl {
        unsafe extern "system" fn Name<Impl: IADsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Class<Impl: IADsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Class() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GUID<Impl: IADsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GUID() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ADsPath<Impl: IADsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ADsPath() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Impl: IADsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parent() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Schema<Impl: IADsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Schema() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInfo<Impl: IADsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInfo().into()
        }
        unsafe extern "system" fn SetInfo<Impl: IADsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInfo().into()
        }
        unsafe extern "system" fn Get<Impl: IADsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Get(::core::mem::transmute_copy(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Put<Impl: IADsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, vprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Put(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&vprop)).into()
        }
        unsafe extern "system" fn GetEx<Impl: IADsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEx(::core::mem::transmute_copy(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutEx<Impl: IADsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lncontrolcode: i32, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, vprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PutEx(::core::mem::transmute_copy(&lncontrolcode), ::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&vprop)).into()
        }
        unsafe extern "system" fn GetInfoEx<Impl: IADsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vproperties: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, lnreserved: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInfoEx(::core::mem::transmute_copy(&vproperties), ::core::mem::transmute_copy(&lnreserved)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            Class: Class::<Impl, IMPL_OFFSET>,
            GUID: GUID::<Impl, IMPL_OFFSET>,
            ADsPath: ADsPath::<Impl, IMPL_OFFSET>,
            Parent: Parent::<Impl, IMPL_OFFSET>,
            Schema: Schema::<Impl, IMPL_OFFSET>,
            GetInfo: GetInfo::<Impl, IMPL_OFFSET>,
            SetInfo: SetInfo::<Impl, IMPL_OFFSET>,
            Get: Get::<Impl, IMPL_OFFSET>,
            Put: Put::<Impl, IMPL_OFFSET>,
            GetEx: GetEx::<Impl, IMPL_OFFSET>,
            PutEx: PutEx::<Impl, IMPL_OFFSET>,
            GetInfoEx: GetInfoEx::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsADSystemInfoImpl: Sized + IDispatchImpl {
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
    fn GetDCSiteName(&mut self, szserver: super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn RefreshSchemaCache(&mut self) -> ::windows::core::Result<()>;
    fn GetTrees(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsADSystemInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsADSystemInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsADSystemInfoVtbl {
        unsafe extern "system" fn UserName<Impl: IADsADSystemInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComputerName<Impl: IADsADSystemInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ComputerName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SiteName<Impl: IADsADSystemInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SiteName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DomainShortName<Impl: IADsADSystemInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DomainShortName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DomainDNSName<Impl: IADsADSystemInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DomainDNSName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ForestDNSName<Impl: IADsADSystemInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForestDNSName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PDCRoleOwner<Impl: IADsADSystemInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PDCRoleOwner() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SchemaRoleOwner<Impl: IADsADSystemInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SchemaRoleOwner() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsNativeMode<Impl: IADsADSystemInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsNativeMode() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAnyDCName<Impl: IADsADSystemInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdcname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAnyDCName() {
                ::core::result::Result::Ok(ok__) => {
                    *pszdcname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDCSiteName<Impl: IADsADSystemInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szserver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pszsitename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDCSiteName(::core::mem::transmute_copy(&szserver)) {
                ::core::result::Result::Ok(ok__) => {
                    *pszsitename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RefreshSchemaCache<Impl: IADsADSystemInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RefreshSchemaCache().into()
        }
        unsafe extern "system" fn GetTrees<Impl: IADsADSystemInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvtrees: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTrees() {
                ::core::result::Result::Ok(ok__) => {
                    *pvtrees = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            UserName: UserName::<Impl, IMPL_OFFSET>,
            ComputerName: ComputerName::<Impl, IMPL_OFFSET>,
            SiteName: SiteName::<Impl, IMPL_OFFSET>,
            DomainShortName: DomainShortName::<Impl, IMPL_OFFSET>,
            DomainDNSName: DomainDNSName::<Impl, IMPL_OFFSET>,
            ForestDNSName: ForestDNSName::<Impl, IMPL_OFFSET>,
            PDCRoleOwner: PDCRoleOwner::<Impl, IMPL_OFFSET>,
            SchemaRoleOwner: SchemaRoleOwner::<Impl, IMPL_OFFSET>,
            IsNativeMode: IsNativeMode::<Impl, IMPL_OFFSET>,
            GetAnyDCName: GetAnyDCName::<Impl, IMPL_OFFSET>,
            GetDCSiteName: GetDCSiteName::<Impl, IMPL_OFFSET>,
            RefreshSchemaCache: RefreshSchemaCache::<Impl, IMPL_OFFSET>,
            GetTrees: GetTrees::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsADSystemInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsAccessControlEntryImpl: Sized + IDispatchImpl {
    fn AccessMask(&mut self) -> ::windows::core::Result<i32>;
    fn SetAccessMask(&mut self, lnaccessmask: i32) -> ::windows::core::Result<()>;
    fn AceType(&mut self) -> ::windows::core::Result<i32>;
    fn SetAceType(&mut self, lnacetype: i32) -> ::windows::core::Result<()>;
    fn AceFlags(&mut self) -> ::windows::core::Result<i32>;
    fn SetAceFlags(&mut self, lnaceflags: i32) -> ::windows::core::Result<()>;
    fn Flags(&mut self) -> ::windows::core::Result<i32>;
    fn SetFlags(&mut self, lnflags: i32) -> ::windows::core::Result<()>;
    fn ObjectType(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetObjectType(&mut self, bstrobjecttype: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn InheritedObjectType(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetInheritedObjectType(&mut self, bstrinheritedobjecttype: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Trustee(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetTrustee(&mut self, bstrtrustee: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsAccessControlEntryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsAccessControlEntryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsAccessControlEntryVtbl {
        unsafe extern "system" fn AccessMask<Impl: IADsAccessControlEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccessMask() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccessMask<Impl: IADsAccessControlEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnaccessmask: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAccessMask(::core::mem::transmute_copy(&lnaccessmask)).into()
        }
        unsafe extern "system" fn AceType<Impl: IADsAccessControlEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AceType() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAceType<Impl: IADsAccessControlEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnacetype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAceType(::core::mem::transmute_copy(&lnacetype)).into()
        }
        unsafe extern "system" fn AceFlags<Impl: IADsAccessControlEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AceFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAceFlags<Impl: IADsAccessControlEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnaceflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAceFlags(::core::mem::transmute_copy(&lnaceflags)).into()
        }
        unsafe extern "system" fn Flags<Impl: IADsAccessControlEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Flags() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlags<Impl: IADsAccessControlEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFlags(::core::mem::transmute_copy(&lnflags)).into()
        }
        unsafe extern "system" fn ObjectType<Impl: IADsAccessControlEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ObjectType() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetObjectType<Impl: IADsAccessControlEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrobjecttype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetObjectType(::core::mem::transmute_copy(&bstrobjecttype)).into()
        }
        unsafe extern "system" fn InheritedObjectType<Impl: IADsAccessControlEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InheritedObjectType() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInheritedObjectType<Impl: IADsAccessControlEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinheritedobjecttype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInheritedObjectType(::core::mem::transmute_copy(&bstrinheritedobjecttype)).into()
        }
        unsafe extern "system" fn Trustee<Impl: IADsAccessControlEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Trustee() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrustee<Impl: IADsAccessControlEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtrustee: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTrustee(::core::mem::transmute_copy(&bstrtrustee)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AccessMask: AccessMask::<Impl, IMPL_OFFSET>,
            SetAccessMask: SetAccessMask::<Impl, IMPL_OFFSET>,
            AceType: AceType::<Impl, IMPL_OFFSET>,
            SetAceType: SetAceType::<Impl, IMPL_OFFSET>,
            AceFlags: AceFlags::<Impl, IMPL_OFFSET>,
            SetAceFlags: SetAceFlags::<Impl, IMPL_OFFSET>,
            Flags: Flags::<Impl, IMPL_OFFSET>,
            SetFlags: SetFlags::<Impl, IMPL_OFFSET>,
            ObjectType: ObjectType::<Impl, IMPL_OFFSET>,
            SetObjectType: SetObjectType::<Impl, IMPL_OFFSET>,
            InheritedObjectType: InheritedObjectType::<Impl, IMPL_OFFSET>,
            SetInheritedObjectType: SetInheritedObjectType::<Impl, IMPL_OFFSET>,
            Trustee: Trustee::<Impl, IMPL_OFFSET>,
            SetTrustee: SetTrustee::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsAccessControlEntry as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsAccessControlListImpl: Sized + IDispatchImpl {
    fn AclRevision(&mut self) -> ::windows::core::Result<i32>;
    fn SetAclRevision(&mut self, lnaclrevision: i32) -> ::windows::core::Result<()>;
    fn AceCount(&mut self) -> ::windows::core::Result<i32>;
    fn SetAceCount(&mut self, lnacecount: i32) -> ::windows::core::Result<()>;
    fn AddAce(&mut self, paccesscontrolentry: ::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn RemoveAce(&mut self, paccesscontrolentry: ::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn CopyAccessList(&mut self) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsAccessControlListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsAccessControlListImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsAccessControlListVtbl {
        unsafe extern "system" fn AclRevision<Impl: IADsAccessControlListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AclRevision() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAclRevision<Impl: IADsAccessControlListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnaclrevision: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAclRevision(::core::mem::transmute_copy(&lnaclrevision)).into()
        }
        unsafe extern "system" fn AceCount<Impl: IADsAccessControlListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AceCount() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAceCount<Impl: IADsAccessControlListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnacecount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAceCount(::core::mem::transmute_copy(&lnacecount)).into()
        }
        unsafe extern "system" fn AddAce<Impl: IADsAccessControlListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paccesscontrolentry: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddAce(::core::mem::transmute(&paccesscontrolentry)).into()
        }
        unsafe extern "system" fn RemoveAce<Impl: IADsAccessControlListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paccesscontrolentry: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAce(::core::mem::transmute(&paccesscontrolentry)).into()
        }
        unsafe extern "system" fn CopyAccessList<Impl: IADsAccessControlListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaccesscontrollist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopyAccessList() {
                ::core::result::Result::Ok(ok__) => {
                    *ppaccesscontrollist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IADsAccessControlListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AclRevision: AclRevision::<Impl, IMPL_OFFSET>,
            SetAclRevision: SetAclRevision::<Impl, IMPL_OFFSET>,
            AceCount: AceCount::<Impl, IMPL_OFFSET>,
            SetAceCount: SetAceCount::<Impl, IMPL_OFFSET>,
            AddAce: AddAce::<Impl, IMPL_OFFSET>,
            RemoveAce: RemoveAce::<Impl, IMPL_OFFSET>,
            CopyAccessList: CopyAccessList::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsAccessControlList as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsAclImpl: Sized + IDispatchImpl {
    fn ProtectedAttrName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetProtectedAttrName(&mut self, bstrprotectedattrname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SubjectName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetSubjectName(&mut self, bstrsubjectname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Privileges(&mut self) -> ::windows::core::Result<i32>;
    fn SetPrivileges(&mut self, lnprivileges: i32) -> ::windows::core::Result<()>;
    fn CopyAcl(&mut self) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsAclVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsAclImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsAclVtbl {
        unsafe extern "system" fn ProtectedAttrName<Impl: IADsAclImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProtectedAttrName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProtectedAttrName<Impl: IADsAclImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotectedattrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProtectedAttrName(::core::mem::transmute_copy(&bstrprotectedattrname)).into()
        }
        unsafe extern "system" fn SubjectName<Impl: IADsAclImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubjectName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubjectName<Impl: IADsAclImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsubjectname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSubjectName(::core::mem::transmute_copy(&bstrsubjectname)).into()
        }
        unsafe extern "system" fn Privileges<Impl: IADsAclImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Privileges() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivileges<Impl: IADsAclImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnprivileges: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrivileges(::core::mem::transmute_copy(&lnprivileges)).into()
        }
        unsafe extern "system" fn CopyAcl<Impl: IADsAclImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppacl: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopyAcl() {
                ::core::result::Result::Ok(ok__) => {
                    *ppacl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ProtectedAttrName: ProtectedAttrName::<Impl, IMPL_OFFSET>,
            SetProtectedAttrName: SetProtectedAttrName::<Impl, IMPL_OFFSET>,
            SubjectName: SubjectName::<Impl, IMPL_OFFSET>,
            SetSubjectName: SetSubjectName::<Impl, IMPL_OFFSET>,
            Privileges: Privileges::<Impl, IMPL_OFFSET>,
            SetPrivileges: SetPrivileges::<Impl, IMPL_OFFSET>,
            CopyAcl: CopyAcl::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsAcl as ::windows::core::Interface>::IID
    }
}
pub trait IADsAggregateeImpl: Sized {
    fn ConnectAsAggregatee(&mut self, pouterunknown: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn DisconnectAsAggregatee(&mut self) -> ::windows::core::Result<()>;
    fn RelinquishInterface(&mut self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RestoreInterface(&mut self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl IADsAggregateeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsAggregateeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsAggregateeVtbl {
        unsafe extern "system" fn ConnectAsAggregatee<Impl: IADsAggregateeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pouterunknown: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConnectAsAggregatee(::core::mem::transmute(&pouterunknown)).into()
        }
        unsafe extern "system" fn DisconnectAsAggregatee<Impl: IADsAggregateeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisconnectAsAggregatee().into()
        }
        unsafe extern "system" fn RelinquishInterface<Impl: IADsAggregateeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RelinquishInterface(::core::mem::transmute_copy(&riid)).into()
        }
        unsafe extern "system" fn RestoreInterface<Impl: IADsAggregateeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RestoreInterface(::core::mem::transmute_copy(&riid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ConnectAsAggregatee: ConnectAsAggregatee::<Impl, IMPL_OFFSET>,
            DisconnectAsAggregatee: DisconnectAsAggregatee::<Impl, IMPL_OFFSET>,
            RelinquishInterface: RelinquishInterface::<Impl, IMPL_OFFSET>,
            RestoreInterface: RestoreInterface::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsAggregatee as ::windows::core::Interface>::IID
    }
}
pub trait IADsAggregatorImpl: Sized {
    fn ConnectAsAggregator(&mut self, paggregatee: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn DisconnectAsAggregator(&mut self) -> ::windows::core::Result<()>;
}
impl IADsAggregatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsAggregatorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsAggregatorVtbl {
        unsafe extern "system" fn ConnectAsAggregator<Impl: IADsAggregatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paggregatee: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConnectAsAggregator(::core::mem::transmute(&paggregatee)).into()
        }
        unsafe extern "system" fn DisconnectAsAggregator<Impl: IADsAggregatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisconnectAsAggregator().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ConnectAsAggregator: ConnectAsAggregator::<Impl, IMPL_OFFSET>,
            DisconnectAsAggregator: DisconnectAsAggregator::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsAggregator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsBackLinkImpl: Sized + IDispatchImpl {
    fn RemoteID(&mut self) -> ::windows::core::Result<i32>;
    fn SetRemoteID(&mut self, lnremoteid: i32) -> ::windows::core::Result<()>;
    fn ObjectName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetObjectName(&mut self, bstrobjectname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsBackLinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsBackLinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsBackLinkVtbl {
        unsafe extern "system" fn RemoteID<Impl: IADsBackLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteID() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteID<Impl: IADsBackLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnremoteid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRemoteID(::core::mem::transmute_copy(&lnremoteid)).into()
        }
        unsafe extern "system" fn ObjectName<Impl: IADsBackLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ObjectName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetObjectName<Impl: IADsBackLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrobjectname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetObjectName(::core::mem::transmute_copy(&bstrobjectname)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            RemoteID: RemoteID::<Impl, IMPL_OFFSET>,
            SetRemoteID: SetRemoteID::<Impl, IMPL_OFFSET>,
            ObjectName: ObjectName::<Impl, IMPL_OFFSET>,
            SetObjectName: SetObjectName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsBackLink as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsCaseIgnoreListImpl: Sized + IDispatchImpl {
    fn CaseIgnoreList(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetCaseIgnoreList(&mut self, vcaseignorelist: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsCaseIgnoreListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsCaseIgnoreListImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsCaseIgnoreListVtbl {
        unsafe extern "system" fn CaseIgnoreList<Impl: IADsCaseIgnoreListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CaseIgnoreList() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCaseIgnoreList<Impl: IADsCaseIgnoreListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcaseignorelist: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCaseIgnoreList(::core::mem::transmute_copy(&vcaseignorelist)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CaseIgnoreList: CaseIgnoreList::<Impl, IMPL_OFFSET>,
            SetCaseIgnoreList: SetCaseIgnoreList::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsCaseIgnoreList as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsClassImpl: Sized + IDispatchImpl + IADsImpl {
    fn PrimaryInterface(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CLSID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetCLSID(&mut self, bstrclsid: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn OID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetOID(&mut self, bstroid: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Abstract(&mut self) -> ::windows::core::Result<i16>;
    fn SetAbstract(&mut self, fabstract: i16) -> ::windows::core::Result<()>;
    fn Auxiliary(&mut self) -> ::windows::core::Result<i16>;
    fn SetAuxiliary(&mut self, fauxiliary: i16) -> ::windows::core::Result<()>;
    fn MandatoryProperties(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetMandatoryProperties(&mut self, vmandatoryproperties: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn OptionalProperties(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetOptionalProperties(&mut self, voptionalproperties: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn NamingProperties(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetNamingProperties(&mut self, vnamingproperties: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DerivedFrom(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetDerivedFrom(&mut self, vderivedfrom: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AuxDerivedFrom(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetAuxDerivedFrom(&mut self, vauxderivedfrom: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn PossibleSuperiors(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetPossibleSuperiors(&mut self, vpossiblesuperiors: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Containment(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetContainment(&mut self, vcontainment: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Container(&mut self) -> ::windows::core::Result<i16>;
    fn SetContainer(&mut self, fcontainer: i16) -> ::windows::core::Result<()>;
    fn HelpFileName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetHelpFileName(&mut self, bstrhelpfilename: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn HelpFileContext(&mut self) -> ::windows::core::Result<i32>;
    fn SetHelpFileContext(&mut self, lnhelpfilecontext: i32) -> ::windows::core::Result<()>;
    fn Qualifiers(&mut self) -> ::windows::core::Result<IADsCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsClassVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsClassImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsClassVtbl {
        unsafe extern "system" fn PrimaryInterface<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrimaryInterface() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CLSID<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CLSID() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCLSID<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrclsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCLSID(::core::mem::transmute_copy(&bstrclsid)).into()
        }
        unsafe extern "system" fn OID<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OID() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOID<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstroid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOID(::core::mem::transmute_copy(&bstroid)).into()
        }
        unsafe extern "system" fn Abstract<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Abstract() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAbstract<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fabstract: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAbstract(::core::mem::transmute_copy(&fabstract)).into()
        }
        unsafe extern "system" fn Auxiliary<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Auxiliary() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuxiliary<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fauxiliary: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuxiliary(::core::mem::transmute_copy(&fauxiliary)).into()
        }
        unsafe extern "system" fn MandatoryProperties<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MandatoryProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMandatoryProperties<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vmandatoryproperties: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMandatoryProperties(::core::mem::transmute_copy(&vmandatoryproperties)).into()
        }
        unsafe extern "system" fn OptionalProperties<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OptionalProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOptionalProperties<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, voptionalproperties: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOptionalProperties(::core::mem::transmute_copy(&voptionalproperties)).into()
        }
        unsafe extern "system" fn NamingProperties<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NamingProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNamingProperties<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vnamingproperties: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNamingProperties(::core::mem::transmute_copy(&vnamingproperties)).into()
        }
        unsafe extern "system" fn DerivedFrom<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DerivedFrom() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDerivedFrom<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vderivedfrom: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDerivedFrom(::core::mem::transmute_copy(&vderivedfrom)).into()
        }
        unsafe extern "system" fn AuxDerivedFrom<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuxDerivedFrom() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuxDerivedFrom<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vauxderivedfrom: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuxDerivedFrom(::core::mem::transmute_copy(&vauxderivedfrom)).into()
        }
        unsafe extern "system" fn PossibleSuperiors<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PossibleSuperiors() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPossibleSuperiors<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vpossiblesuperiors: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPossibleSuperiors(::core::mem::transmute_copy(&vpossiblesuperiors)).into()
        }
        unsafe extern "system" fn Containment<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Containment() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContainment<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcontainment: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContainment(::core::mem::transmute_copy(&vcontainment)).into()
        }
        unsafe extern "system" fn Container<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Container() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContainer<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fcontainer: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContainer(::core::mem::transmute_copy(&fcontainer)).into()
        }
        unsafe extern "system" fn HelpFileName<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HelpFileName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHelpFileName<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrhelpfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHelpFileName(::core::mem::transmute_copy(&bstrhelpfilename)).into()
        }
        unsafe extern "system" fn HelpFileContext<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HelpFileContext() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHelpFileContext<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnhelpfilecontext: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHelpFileContext(::core::mem::transmute_copy(&lnhelpfilecontext)).into()
        }
        unsafe extern "system" fn Qualifiers<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqualifiers: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Qualifiers() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqualifiers = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IADsVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            PrimaryInterface: PrimaryInterface::<Impl, IMPL_OFFSET>,
            CLSID: CLSID::<Impl, IMPL_OFFSET>,
            SetCLSID: SetCLSID::<Impl, IMPL_OFFSET>,
            OID: OID::<Impl, IMPL_OFFSET>,
            SetOID: SetOID::<Impl, IMPL_OFFSET>,
            Abstract: Abstract::<Impl, IMPL_OFFSET>,
            SetAbstract: SetAbstract::<Impl, IMPL_OFFSET>,
            Auxiliary: Auxiliary::<Impl, IMPL_OFFSET>,
            SetAuxiliary: SetAuxiliary::<Impl, IMPL_OFFSET>,
            MandatoryProperties: MandatoryProperties::<Impl, IMPL_OFFSET>,
            SetMandatoryProperties: SetMandatoryProperties::<Impl, IMPL_OFFSET>,
            OptionalProperties: OptionalProperties::<Impl, IMPL_OFFSET>,
            SetOptionalProperties: SetOptionalProperties::<Impl, IMPL_OFFSET>,
            NamingProperties: NamingProperties::<Impl, IMPL_OFFSET>,
            SetNamingProperties: SetNamingProperties::<Impl, IMPL_OFFSET>,
            DerivedFrom: DerivedFrom::<Impl, IMPL_OFFSET>,
            SetDerivedFrom: SetDerivedFrom::<Impl, IMPL_OFFSET>,
            AuxDerivedFrom: AuxDerivedFrom::<Impl, IMPL_OFFSET>,
            SetAuxDerivedFrom: SetAuxDerivedFrom::<Impl, IMPL_OFFSET>,
            PossibleSuperiors: PossibleSuperiors::<Impl, IMPL_OFFSET>,
            SetPossibleSuperiors: SetPossibleSuperiors::<Impl, IMPL_OFFSET>,
            Containment: Containment::<Impl, IMPL_OFFSET>,
            SetContainment: SetContainment::<Impl, IMPL_OFFSET>,
            Container: Container::<Impl, IMPL_OFFSET>,
            SetContainer: SetContainer::<Impl, IMPL_OFFSET>,
            HelpFileName: HelpFileName::<Impl, IMPL_OFFSET>,
            SetHelpFileName: SetHelpFileName::<Impl, IMPL_OFFSET>,
            HelpFileContext: HelpFileContext::<Impl, IMPL_OFFSET>,
            SetHelpFileContext: SetHelpFileContext::<Impl, IMPL_OFFSET>,
            Qualifiers: Qualifiers::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsClass as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsCollectionImpl: Sized + IDispatchImpl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Add(&mut self, bstrname: super::super::Foundation::BSTR, vitem: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Remove(&mut self, bstritemtoberemoved: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetObject(&mut self, bstrname: super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsCollectionVtbl {
        unsafe extern "system" fn _NewEnum<Impl: IADsCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumerator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IADsCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, vitem: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&vitem)).into()
        }
        unsafe extern "system" fn Remove<Impl: IADsCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemtoberemoved: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&bstritemtoberemoved)).into()
        }
        unsafe extern "system" fn GetObject<Impl: IADsCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvitem: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetObject(::core::mem::transmute_copy(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            GetObject: GetObject::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsComputerImpl: Sized + IDispatchImpl + IADsImpl {
    fn ComputerID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Site(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, bstrdescription: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Location(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLocation(&mut self, bstrlocation: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PrimaryUser(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPrimaryUser(&mut self, bstrprimaryuser: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Owner(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetOwner(&mut self, bstrowner: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Division(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDivision(&mut self, bstrdivision: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Department(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDepartment(&mut self, bstrdepartment: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Role(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetRole(&mut self, bstrrole: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn OperatingSystem(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetOperatingSystem(&mut self, bstroperatingsystem: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn OperatingSystemVersion(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetOperatingSystemVersion(&mut self, bstroperatingsystemversion: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Model(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetModel(&mut self, bstrmodel: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Processor(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetProcessor(&mut self, bstrprocessor: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ProcessorCount(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetProcessorCount(&mut self, bstrprocessorcount: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn MemorySize(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetMemorySize(&mut self, bstrmemorysize: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn StorageCapacity(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetStorageCapacity(&mut self, bstrstoragecapacity: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn NetAddresses(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetNetAddresses(&mut self, vnetaddresses: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsComputerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsComputerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsComputerVtbl {
        unsafe extern "system" fn ComputerID<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ComputerID() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Site<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Site() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&bstrdescription)).into()
        }
        unsafe extern "system" fn Location<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Location() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocation<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlocation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocation(::core::mem::transmute_copy(&bstrlocation)).into()
        }
        unsafe extern "system" fn PrimaryUser<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrimaryUser() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrimaryUser<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprimaryuser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrimaryUser(::core::mem::transmute_copy(&bstrprimaryuser)).into()
        }
        unsafe extern "system" fn Owner<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Owner() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOwner<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrowner: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOwner(::core::mem::transmute_copy(&bstrowner)).into()
        }
        unsafe extern "system" fn Division<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Division() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDivision<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdivision: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDivision(::core::mem::transmute_copy(&bstrdivision)).into()
        }
        unsafe extern "system" fn Department<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Department() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDepartment<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdepartment: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDepartment(::core::mem::transmute_copy(&bstrdepartment)).into()
        }
        unsafe extern "system" fn Role<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Role() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRole<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRole(::core::mem::transmute_copy(&bstrrole)).into()
        }
        unsafe extern "system" fn OperatingSystem<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OperatingSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOperatingSystem<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstroperatingsystem: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOperatingSystem(::core::mem::transmute_copy(&bstroperatingsystem)).into()
        }
        unsafe extern "system" fn OperatingSystemVersion<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OperatingSystemVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOperatingSystemVersion<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstroperatingsystemversion: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOperatingSystemVersion(::core::mem::transmute_copy(&bstroperatingsystemversion)).into()
        }
        unsafe extern "system" fn Model<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Model() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetModel<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmodel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetModel(::core::mem::transmute_copy(&bstrmodel)).into()
        }
        unsafe extern "system" fn Processor<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Processor() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProcessor<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprocessor: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProcessor(::core::mem::transmute_copy(&bstrprocessor)).into()
        }
        unsafe extern "system" fn ProcessorCount<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessorCount() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProcessorCount<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprocessorcount: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProcessorCount(::core::mem::transmute_copy(&bstrprocessorcount)).into()
        }
        unsafe extern "system" fn MemorySize<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MemorySize() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMemorySize<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmemorysize: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMemorySize(::core::mem::transmute_copy(&bstrmemorysize)).into()
        }
        unsafe extern "system" fn StorageCapacity<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StorageCapacity() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStorageCapacity<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrstoragecapacity: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStorageCapacity(::core::mem::transmute_copy(&bstrstoragecapacity)).into()
        }
        unsafe extern "system" fn NetAddresses<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetAddresses() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetAddresses<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vnetaddresses: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNetAddresses(::core::mem::transmute_copy(&vnetaddresses)).into()
        }
        Self {
            base: IADsVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ComputerID: ComputerID::<Impl, IMPL_OFFSET>,
            Site: Site::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            SetDescription: SetDescription::<Impl, IMPL_OFFSET>,
            Location: Location::<Impl, IMPL_OFFSET>,
            SetLocation: SetLocation::<Impl, IMPL_OFFSET>,
            PrimaryUser: PrimaryUser::<Impl, IMPL_OFFSET>,
            SetPrimaryUser: SetPrimaryUser::<Impl, IMPL_OFFSET>,
            Owner: Owner::<Impl, IMPL_OFFSET>,
            SetOwner: SetOwner::<Impl, IMPL_OFFSET>,
            Division: Division::<Impl, IMPL_OFFSET>,
            SetDivision: SetDivision::<Impl, IMPL_OFFSET>,
            Department: Department::<Impl, IMPL_OFFSET>,
            SetDepartment: SetDepartment::<Impl, IMPL_OFFSET>,
            Role: Role::<Impl, IMPL_OFFSET>,
            SetRole: SetRole::<Impl, IMPL_OFFSET>,
            OperatingSystem: OperatingSystem::<Impl, IMPL_OFFSET>,
            SetOperatingSystem: SetOperatingSystem::<Impl, IMPL_OFFSET>,
            OperatingSystemVersion: OperatingSystemVersion::<Impl, IMPL_OFFSET>,
            SetOperatingSystemVersion: SetOperatingSystemVersion::<Impl, IMPL_OFFSET>,
            Model: Model::<Impl, IMPL_OFFSET>,
            SetModel: SetModel::<Impl, IMPL_OFFSET>,
            Processor: Processor::<Impl, IMPL_OFFSET>,
            SetProcessor: SetProcessor::<Impl, IMPL_OFFSET>,
            ProcessorCount: ProcessorCount::<Impl, IMPL_OFFSET>,
            SetProcessorCount: SetProcessorCount::<Impl, IMPL_OFFSET>,
            MemorySize: MemorySize::<Impl, IMPL_OFFSET>,
            SetMemorySize: SetMemorySize::<Impl, IMPL_OFFSET>,
            StorageCapacity: StorageCapacity::<Impl, IMPL_OFFSET>,
            SetStorageCapacity: SetStorageCapacity::<Impl, IMPL_OFFSET>,
            NetAddresses: NetAddresses::<Impl, IMPL_OFFSET>,
            SetNetAddresses: SetNetAddresses::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsComputer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsComputerOperationsImpl: Sized + IDispatchImpl + IADsImpl {
    fn Status(&mut self) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn Shutdown(&mut self, breboot: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsComputerOperationsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsComputerOperationsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsComputerOperationsVtbl {
        unsafe extern "system" fn Status<Impl: IADsComputerOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *ppobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shutdown<Impl: IADsComputerOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, breboot: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Shutdown(::core::mem::transmute_copy(&breboot)).into()
        }
        Self { base: IADsVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Status: Status::<Impl, IMPL_OFFSET>, Shutdown: Shutdown::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsComputerOperations as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsContainerImpl: Sized + IDispatchImpl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Filter(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetFilter(&mut self, var: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Hints(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetHints(&mut self, vhints: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GetObject(&mut self, classname: super::super::Foundation::BSTR, relativename: super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn Create(&mut self, classname: super::super::Foundation::BSTR, relativename: super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn Delete(&mut self, bstrclassname: super::super::Foundation::BSTR, bstrrelativename: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn CopyHere(&mut self, sourcename: super::super::Foundation::BSTR, newname: super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn MoveHere(&mut self, sourcename: super::super::Foundation::BSTR, newname: super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsContainerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsContainerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsContainerVtbl {
        unsafe extern "system" fn Count<Impl: IADsContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IADsContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Filter<Impl: IADsContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvar: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Filter() {
                ::core::result::Result::Ok(ok__) => {
                    *pvar = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFilter<Impl: IADsContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, var: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFilter(::core::mem::transmute_copy(&var)).into()
        }
        unsafe extern "system" fn Hints<Impl: IADsContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvfilter: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Hints() {
                ::core::result::Result::Ok(ok__) => {
                    *pvfilter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHints<Impl: IADsContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vhints: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHints(::core::mem::transmute_copy(&vhints)).into()
        }
        unsafe extern "system" fn GetObject<Impl: IADsContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, classname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, relativename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetObject(::core::mem::transmute_copy(&classname), ::core::mem::transmute_copy(&relativename)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Impl: IADsContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, classname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, relativename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(::core::mem::transmute_copy(&classname), ::core::mem::transmute_copy(&relativename)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IADsContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrclassname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrrelativename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete(::core::mem::transmute_copy(&bstrclassname), ::core::mem::transmute_copy(&bstrrelativename)).into()
        }
        unsafe extern "system" fn CopyHere<Impl: IADsContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, newname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopyHere(::core::mem::transmute_copy(&sourcename), ::core::mem::transmute_copy(&newname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveHere<Impl: IADsContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, newname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveHere(::core::mem::transmute_copy(&sourcename), ::core::mem::transmute_copy(&newname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Filter: Filter::<Impl, IMPL_OFFSET>,
            SetFilter: SetFilter::<Impl, IMPL_OFFSET>,
            Hints: Hints::<Impl, IMPL_OFFSET>,
            SetHints: SetHints::<Impl, IMPL_OFFSET>,
            GetObject: GetObject::<Impl, IMPL_OFFSET>,
            Create: Create::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
            CopyHere: CopyHere::<Impl, IMPL_OFFSET>,
            MoveHere: MoveHere::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsContainer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsDNWithBinaryImpl: Sized + IDispatchImpl {
    fn BinaryValue(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetBinaryValue(&mut self, vbinaryvalue: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DNString(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDNString(&mut self, bstrdnstring: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsDNWithBinaryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsDNWithBinaryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsDNWithBinaryVtbl {
        unsafe extern "system" fn BinaryValue<Impl: IADsDNWithBinaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BinaryValue() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBinaryValue<Impl: IADsDNWithBinaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vbinaryvalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBinaryValue(::core::mem::transmute_copy(&vbinaryvalue)).into()
        }
        unsafe extern "system" fn DNString<Impl: IADsDNWithBinaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DNString() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDNString<Impl: IADsDNWithBinaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdnstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDNString(::core::mem::transmute_copy(&bstrdnstring)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            BinaryValue: BinaryValue::<Impl, IMPL_OFFSET>,
            SetBinaryValue: SetBinaryValue::<Impl, IMPL_OFFSET>,
            DNString: DNString::<Impl, IMPL_OFFSET>,
            SetDNString: SetDNString::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsDNWithBinary as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsDNWithStringImpl: Sized + IDispatchImpl {
    fn StringValue(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetStringValue(&mut self, bstrstringvalue: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DNString(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDNString(&mut self, bstrdnstring: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsDNWithStringVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsDNWithStringImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsDNWithStringVtbl {
        unsafe extern "system" fn StringValue<Impl: IADsDNWithStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StringValue() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStringValue<Impl: IADsDNWithStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrstringvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStringValue(::core::mem::transmute_copy(&bstrstringvalue)).into()
        }
        unsafe extern "system" fn DNString<Impl: IADsDNWithStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DNString() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDNString<Impl: IADsDNWithStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdnstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDNString(::core::mem::transmute_copy(&bstrdnstring)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            StringValue: StringValue::<Impl, IMPL_OFFSET>,
            SetStringValue: SetStringValue::<Impl, IMPL_OFFSET>,
            DNString: DNString::<Impl, IMPL_OFFSET>,
            SetDNString: SetDNString::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsDNWithString as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsDeleteOpsImpl: Sized + IDispatchImpl {
    fn DeleteObject(&mut self, lnflags: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsDeleteOpsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsDeleteOpsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsDeleteOpsVtbl {
        unsafe extern "system" fn DeleteObject<Impl: IADsDeleteOpsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteObject(::core::mem::transmute_copy(&lnflags)).into()
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), DeleteObject: DeleteObject::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsDeleteOps as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsDomainImpl: Sized + IDispatchImpl + IADsImpl {
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
impl IADsDomainVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsDomainImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsDomainVtbl {
        unsafe extern "system" fn IsWorkgroup<Impl: IADsDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsWorkgroup() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinPasswordLength<Impl: IADsDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinPasswordLength() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinPasswordLength<Impl: IADsDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnminpasswordlength: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinPasswordLength(::core::mem::transmute_copy(&lnminpasswordlength)).into()
        }
        unsafe extern "system" fn MinPasswordAge<Impl: IADsDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinPasswordAge() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinPasswordAge<Impl: IADsDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnminpasswordage: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinPasswordAge(::core::mem::transmute_copy(&lnminpasswordage)).into()
        }
        unsafe extern "system" fn MaxPasswordAge<Impl: IADsDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxPasswordAge() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxPasswordAge<Impl: IADsDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnmaxpasswordage: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxPasswordAge(::core::mem::transmute_copy(&lnmaxpasswordage)).into()
        }
        unsafe extern "system" fn MaxBadPasswordsAllowed<Impl: IADsDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxBadPasswordsAllowed() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxBadPasswordsAllowed<Impl: IADsDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnmaxbadpasswordsallowed: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxBadPasswordsAllowed(::core::mem::transmute_copy(&lnmaxbadpasswordsallowed)).into()
        }
        unsafe extern "system" fn PasswordHistoryLength<Impl: IADsDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PasswordHistoryLength() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPasswordHistoryLength<Impl: IADsDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnpasswordhistorylength: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPasswordHistoryLength(::core::mem::transmute_copy(&lnpasswordhistorylength)).into()
        }
        unsafe extern "system" fn PasswordAttributes<Impl: IADsDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PasswordAttributes() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPasswordAttributes<Impl: IADsDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnpasswordattributes: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPasswordAttributes(::core::mem::transmute_copy(&lnpasswordattributes)).into()
        }
        unsafe extern "system" fn AutoUnlockInterval<Impl: IADsDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoUnlockInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoUnlockInterval<Impl: IADsDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnautounlockinterval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoUnlockInterval(::core::mem::transmute_copy(&lnautounlockinterval)).into()
        }
        unsafe extern "system" fn LockoutObservationInterval<Impl: IADsDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LockoutObservationInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLockoutObservationInterval<Impl: IADsDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnlockoutobservationinterval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLockoutObservationInterval(::core::mem::transmute_copy(&lnlockoutobservationinterval)).into()
        }
        Self {
            base: IADsVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            IsWorkgroup: IsWorkgroup::<Impl, IMPL_OFFSET>,
            MinPasswordLength: MinPasswordLength::<Impl, IMPL_OFFSET>,
            SetMinPasswordLength: SetMinPasswordLength::<Impl, IMPL_OFFSET>,
            MinPasswordAge: MinPasswordAge::<Impl, IMPL_OFFSET>,
            SetMinPasswordAge: SetMinPasswordAge::<Impl, IMPL_OFFSET>,
            MaxPasswordAge: MaxPasswordAge::<Impl, IMPL_OFFSET>,
            SetMaxPasswordAge: SetMaxPasswordAge::<Impl, IMPL_OFFSET>,
            MaxBadPasswordsAllowed: MaxBadPasswordsAllowed::<Impl, IMPL_OFFSET>,
            SetMaxBadPasswordsAllowed: SetMaxBadPasswordsAllowed::<Impl, IMPL_OFFSET>,
            PasswordHistoryLength: PasswordHistoryLength::<Impl, IMPL_OFFSET>,
            SetPasswordHistoryLength: SetPasswordHistoryLength::<Impl, IMPL_OFFSET>,
            PasswordAttributes: PasswordAttributes::<Impl, IMPL_OFFSET>,
            SetPasswordAttributes: SetPasswordAttributes::<Impl, IMPL_OFFSET>,
            AutoUnlockInterval: AutoUnlockInterval::<Impl, IMPL_OFFSET>,
            SetAutoUnlockInterval: SetAutoUnlockInterval::<Impl, IMPL_OFFSET>,
            LockoutObservationInterval: LockoutObservationInterval::<Impl, IMPL_OFFSET>,
            SetLockoutObservationInterval: SetLockoutObservationInterval::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsDomain as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsEmailImpl: Sized + IDispatchImpl {
    fn Type(&mut self) -> ::windows::core::Result<i32>;
    fn SetType(&mut self, lntype: i32) -> ::windows::core::Result<()>;
    fn Address(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetAddress(&mut self, bstraddress: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsEmailVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsEmailImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsEmailVtbl {
        unsafe extern "system" fn Type<Impl: IADsEmailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Impl: IADsEmailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lntype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetType(::core::mem::transmute_copy(&lntype)).into()
        }
        unsafe extern "system" fn Address<Impl: IADsEmailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Address() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAddress<Impl: IADsEmailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstraddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAddress(::core::mem::transmute_copy(&bstraddress)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Type: Type::<Impl, IMPL_OFFSET>,
            SetType: SetType::<Impl, IMPL_OFFSET>,
            Address: Address::<Impl, IMPL_OFFSET>,
            SetAddress: SetAddress::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsEmail as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsExtensionImpl: Sized {
    fn Operate(&mut self, dwcode: u32, vardata1: super::super::System::Com::VARIANT, vardata2: super::super::System::Com::VARIANT, vardata3: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn PrivateGetIDsOfNames(&mut self, riid: *const ::windows::core::GUID, rgsznames: *const *const u16, cnames: u32, lcid: u32) -> ::windows::core::Result<i32>;
    fn PrivateInvoke(&mut self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsExtensionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsExtensionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsExtensionVtbl {
        unsafe extern "system" fn Operate<Impl: IADsExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcode: u32, vardata1: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, vardata2: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, vardata3: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Operate(::core::mem::transmute_copy(&dwcode), ::core::mem::transmute_copy(&vardata1), ::core::mem::transmute_copy(&vardata2), ::core::mem::transmute_copy(&vardata3)).into()
        }
        unsafe extern "system" fn PrivateGetIDsOfNames<Impl: IADsExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const *const u16, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivateGetIDsOfNames(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&rgsznames), ::core::mem::transmute_copy(&cnames), ::core::mem::transmute_copy(&lcid)) {
                ::core::result::Result::Ok(ok__) => {
                    *rgdispid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateInvoke<Impl: IADsExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PrivateInvoke(::core::mem::transmute_copy(&dispidmember), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&lcid), ::core::mem::transmute_copy(&wflags), ::core::mem::transmute_copy(&pdispparams), ::core::mem::transmute_copy(&pvarresult), ::core::mem::transmute_copy(&pexcepinfo), ::core::mem::transmute_copy(&puargerr)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Operate: Operate::<Impl, IMPL_OFFSET>,
            PrivateGetIDsOfNames: PrivateGetIDsOfNames::<Impl, IMPL_OFFSET>,
            PrivateInvoke: PrivateInvoke::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsExtension as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsFaxNumberImpl: Sized + IDispatchImpl {
    fn TelephoneNumber(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetTelephoneNumber(&mut self, bstrtelephonenumber: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Parameters(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetParameters(&mut self, vparameters: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsFaxNumberVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsFaxNumberImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsFaxNumberVtbl {
        unsafe extern "system" fn TelephoneNumber<Impl: IADsFaxNumberImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TelephoneNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTelephoneNumber<Impl: IADsFaxNumberImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtelephonenumber: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTelephoneNumber(::core::mem::transmute_copy(&bstrtelephonenumber)).into()
        }
        unsafe extern "system" fn Parameters<Impl: IADsFaxNumberImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parameters() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParameters<Impl: IADsFaxNumberImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vparameters: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetParameters(::core::mem::transmute_copy(&vparameters)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            TelephoneNumber: TelephoneNumber::<Impl, IMPL_OFFSET>,
            SetTelephoneNumber: SetTelephoneNumber::<Impl, IMPL_OFFSET>,
            Parameters: Parameters::<Impl, IMPL_OFFSET>,
            SetParameters: SetParameters::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsFaxNumber as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsFileServiceImpl: Sized + IDispatchImpl + IADsImpl + IADsServiceImpl {
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, bstrdescription: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn MaxUserCount(&mut self) -> ::windows::core::Result<i32>;
    fn SetMaxUserCount(&mut self, lnmaxusercount: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsFileServiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsFileServiceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsFileServiceVtbl {
        unsafe extern "system" fn Description<Impl: IADsFileServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IADsFileServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&bstrdescription)).into()
        }
        unsafe extern "system" fn MaxUserCount<Impl: IADsFileServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxUserCount() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxUserCount<Impl: IADsFileServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnmaxusercount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxUserCount(::core::mem::transmute_copy(&lnmaxusercount)).into()
        }
        Self {
            base: IADsServiceVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Description: Description::<Impl, IMPL_OFFSET>,
            SetDescription: SetDescription::<Impl, IMPL_OFFSET>,
            MaxUserCount: MaxUserCount::<Impl, IMPL_OFFSET>,
            SetMaxUserCount: SetMaxUserCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsFileService as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsFileServiceOperationsImpl: Sized + IDispatchImpl + IADsImpl + IADsServiceOperationsImpl {
    fn Sessions(&mut self) -> ::windows::core::Result<IADsCollection>;
    fn Resources(&mut self) -> ::windows::core::Result<IADsCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsFileServiceOperationsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsFileServiceOperationsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsFileServiceOperationsVtbl {
        unsafe extern "system" fn Sessions<Impl: IADsFileServiceOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsessions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Sessions() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsessions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resources<Impl: IADsFileServiceOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Resources() {
                ::core::result::Result::Ok(ok__) => {
                    *ppresources = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IADsServiceOperationsVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Sessions: Sessions::<Impl, IMPL_OFFSET>,
            Resources: Resources::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsFileServiceOperations as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsFileShareImpl: Sized + IDispatchImpl + IADsImpl {
    fn CurrentUserCount(&mut self) -> ::windows::core::Result<i32>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, bstrdescription: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn HostComputer(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetHostComputer(&mut self, bstrhostcomputer: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Path(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPath(&mut self, bstrpath: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn MaxUserCount(&mut self) -> ::windows::core::Result<i32>;
    fn SetMaxUserCount(&mut self, lnmaxusercount: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsFileShareVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsFileShareImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsFileShareVtbl {
        unsafe extern "system" fn CurrentUserCount<Impl: IADsFileShareImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentUserCount() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IADsFileShareImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IADsFileShareImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&bstrdescription)).into()
        }
        unsafe extern "system" fn HostComputer<Impl: IADsFileShareImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HostComputer() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHostComputer<Impl: IADsFileShareImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrhostcomputer: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHostComputer(::core::mem::transmute_copy(&bstrhostcomputer)).into()
        }
        unsafe extern "system" fn Path<Impl: IADsFileShareImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPath<Impl: IADsFileShareImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPath(::core::mem::transmute_copy(&bstrpath)).into()
        }
        unsafe extern "system" fn MaxUserCount<Impl: IADsFileShareImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxUserCount() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxUserCount<Impl: IADsFileShareImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnmaxusercount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxUserCount(::core::mem::transmute_copy(&lnmaxusercount)).into()
        }
        Self {
            base: IADsVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CurrentUserCount: CurrentUserCount::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            SetDescription: SetDescription::<Impl, IMPL_OFFSET>,
            HostComputer: HostComputer::<Impl, IMPL_OFFSET>,
            SetHostComputer: SetHostComputer::<Impl, IMPL_OFFSET>,
            Path: Path::<Impl, IMPL_OFFSET>,
            SetPath: SetPath::<Impl, IMPL_OFFSET>,
            MaxUserCount: MaxUserCount::<Impl, IMPL_OFFSET>,
            SetMaxUserCount: SetMaxUserCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsFileShare as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsGroupImpl: Sized + IDispatchImpl + IADsImpl {
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, bstrdescription: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Members(&mut self) -> ::windows::core::Result<IADsMembers>;
    fn IsMember(&mut self, bstrmember: super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
    fn Add(&mut self, bstrnewitem: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Remove(&mut self, bstritemtoberemoved: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsGroupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsGroupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsGroupVtbl {
        unsafe extern "system" fn Description<Impl: IADsGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IADsGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&bstrdescription)).into()
        }
        unsafe extern "system" fn Members<Impl: IADsGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmembers: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Members() {
                ::core::result::Result::Ok(ok__) => {
                    *ppmembers = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMember<Impl: IADsGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmember: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bmember: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsMember(::core::mem::transmute_copy(&bstrmember)) {
                ::core::result::Result::Ok(ok__) => {
                    *bmember = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IADsGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrnewitem: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute_copy(&bstrnewitem)).into()
        }
        unsafe extern "system" fn Remove<Impl: IADsGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemtoberemoved: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&bstritemtoberemoved)).into()
        }
        Self {
            base: IADsVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Description: Description::<Impl, IMPL_OFFSET>,
            SetDescription: SetDescription::<Impl, IMPL_OFFSET>,
            Members: Members::<Impl, IMPL_OFFSET>,
            IsMember: IsMember::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsGroup as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsHoldImpl: Sized + IDispatchImpl {
    fn ObjectName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetObjectName(&mut self, bstrobjectname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Amount(&mut self) -> ::windows::core::Result<i32>;
    fn SetAmount(&mut self, lnamount: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsHoldVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsHoldImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsHoldVtbl {
        unsafe extern "system" fn ObjectName<Impl: IADsHoldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ObjectName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetObjectName<Impl: IADsHoldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrobjectname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetObjectName(::core::mem::transmute_copy(&bstrobjectname)).into()
        }
        unsafe extern "system" fn Amount<Impl: IADsHoldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Amount() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAmount<Impl: IADsHoldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnamount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAmount(::core::mem::transmute_copy(&lnamount)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ObjectName: ObjectName::<Impl, IMPL_OFFSET>,
            SetObjectName: SetObjectName::<Impl, IMPL_OFFSET>,
            Amount: Amount::<Impl, IMPL_OFFSET>,
            SetAmount: SetAmount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsHold as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsLargeIntegerImpl: Sized + IDispatchImpl {
    fn HighPart(&mut self) -> ::windows::core::Result<i32>;
    fn SetHighPart(&mut self, lnhighpart: i32) -> ::windows::core::Result<()>;
    fn LowPart(&mut self) -> ::windows::core::Result<i32>;
    fn SetLowPart(&mut self, lnlowpart: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsLargeIntegerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsLargeIntegerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsLargeIntegerVtbl {
        unsafe extern "system" fn HighPart<Impl: IADsLargeIntegerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HighPart() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHighPart<Impl: IADsLargeIntegerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnhighpart: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHighPart(::core::mem::transmute_copy(&lnhighpart)).into()
        }
        unsafe extern "system" fn LowPart<Impl: IADsLargeIntegerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LowPart() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLowPart<Impl: IADsLargeIntegerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnlowpart: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLowPart(::core::mem::transmute_copy(&lnlowpart)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            HighPart: HighPart::<Impl, IMPL_OFFSET>,
            SetHighPart: SetHighPart::<Impl, IMPL_OFFSET>,
            LowPart: LowPart::<Impl, IMPL_OFFSET>,
            SetLowPart: SetLowPart::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsLargeInteger as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsLocalityImpl: Sized + IDispatchImpl + IADsImpl {
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, bstrdescription: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn LocalityName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLocalityName(&mut self, bstrlocalityname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PostalAddress(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPostalAddress(&mut self, bstrpostaladdress: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SeeAlso(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetSeeAlso(&mut self, vseealso: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsLocalityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsLocalityImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsLocalityVtbl {
        unsafe extern "system" fn Description<Impl: IADsLocalityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IADsLocalityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&bstrdescription)).into()
        }
        unsafe extern "system" fn LocalityName<Impl: IADsLocalityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalityName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalityName<Impl: IADsLocalityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlocalityname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocalityName(::core::mem::transmute_copy(&bstrlocalityname)).into()
        }
        unsafe extern "system" fn PostalAddress<Impl: IADsLocalityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PostalAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPostalAddress<Impl: IADsLocalityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpostaladdress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPostalAddress(::core::mem::transmute_copy(&bstrpostaladdress)).into()
        }
        unsafe extern "system" fn SeeAlso<Impl: IADsLocalityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SeeAlso() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSeeAlso<Impl: IADsLocalityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vseealso: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSeeAlso(::core::mem::transmute_copy(&vseealso)).into()
        }
        Self {
            base: IADsVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Description: Description::<Impl, IMPL_OFFSET>,
            SetDescription: SetDescription::<Impl, IMPL_OFFSET>,
            LocalityName: LocalityName::<Impl, IMPL_OFFSET>,
            SetLocalityName: SetLocalityName::<Impl, IMPL_OFFSET>,
            PostalAddress: PostalAddress::<Impl, IMPL_OFFSET>,
            SetPostalAddress: SetPostalAddress::<Impl, IMPL_OFFSET>,
            SeeAlso: SeeAlso::<Impl, IMPL_OFFSET>,
            SetSeeAlso: SetSeeAlso::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsLocality as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsMembersImpl: Sized + IDispatchImpl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Filter(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetFilter(&mut self, pvfilter: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsMembersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsMembersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsMembersVtbl {
        unsafe extern "system" fn Count<Impl: IADsMembersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IADsMembersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumerator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Filter<Impl: IADsMembersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvfilter: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Filter() {
                ::core::result::Result::Ok(ok__) => {
                    *pvfilter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFilter<Impl: IADsMembersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvfilter: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFilter(::core::mem::transmute_copy(&pvfilter)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Filter: Filter::<Impl, IMPL_OFFSET>,
            SetFilter: SetFilter::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsMembers as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsNameTranslateImpl: Sized + IDispatchImpl {
    fn SetChaseReferral(&mut self, lnchasereferral: i32) -> ::windows::core::Result<()>;
    fn Init(&mut self, lnsettype: i32, bstradspath: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn InitEx(&mut self, lnsettype: i32, bstradspath: super::super::Foundation::BSTR, bstruserid: super::super::Foundation::BSTR, bstrdomain: super::super::Foundation::BSTR, bstrpassword: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Set(&mut self, lnsettype: i32, bstradspath: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Get(&mut self, lnformattype: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetEx(&mut self, lnformattype: i32, pvar: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GetEx(&mut self, lnformattype: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsNameTranslateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsNameTranslateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsNameTranslateVtbl {
        unsafe extern "system" fn SetChaseReferral<Impl: IADsNameTranslateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnchasereferral: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetChaseReferral(::core::mem::transmute_copy(&lnchasereferral)).into()
        }
        unsafe extern "system" fn Init<Impl: IADsNameTranslateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnsettype: i32, bstradspath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Init(::core::mem::transmute_copy(&lnsettype), ::core::mem::transmute_copy(&bstradspath)).into()
        }
        unsafe extern "system" fn InitEx<Impl: IADsNameTranslateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnsettype: i32, bstradspath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstruserid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdomain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitEx(::core::mem::transmute_copy(&lnsettype), ::core::mem::transmute_copy(&bstradspath), ::core::mem::transmute_copy(&bstruserid), ::core::mem::transmute_copy(&bstrdomain), ::core::mem::transmute_copy(&bstrpassword)).into()
        }
        unsafe extern "system" fn Set<Impl: IADsNameTranslateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnsettype: i32, bstradspath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Set(::core::mem::transmute_copy(&lnsettype), ::core::mem::transmute_copy(&bstradspath)).into()
        }
        unsafe extern "system" fn Get<Impl: IADsNameTranslateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnformattype: i32, pbstradspath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Get(::core::mem::transmute_copy(&lnformattype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstradspath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEx<Impl: IADsNameTranslateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnformattype: i32, pvar: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEx(::core::mem::transmute_copy(&lnformattype), ::core::mem::transmute_copy(&pvar)).into()
        }
        unsafe extern "system" fn GetEx<Impl: IADsNameTranslateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnformattype: i32, pvar: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEx(::core::mem::transmute_copy(&lnformattype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvar = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetChaseReferral: SetChaseReferral::<Impl, IMPL_OFFSET>,
            Init: Init::<Impl, IMPL_OFFSET>,
            InitEx: InitEx::<Impl, IMPL_OFFSET>,
            Set: Set::<Impl, IMPL_OFFSET>,
            Get: Get::<Impl, IMPL_OFFSET>,
            SetEx: SetEx::<Impl, IMPL_OFFSET>,
            GetEx: GetEx::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsNameTranslate as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsNamespacesImpl: Sized + IDispatchImpl + IADsImpl {
    fn DefaultContainer(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDefaultContainer(&mut self, bstrdefaultcontainer: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsNamespacesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsNamespacesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsNamespacesVtbl {
        unsafe extern "system" fn DefaultContainer<Impl: IADsNamespacesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultContainer() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultContainer<Impl: IADsNamespacesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdefaultcontainer: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultContainer(::core::mem::transmute_copy(&bstrdefaultcontainer)).into()
        }
        Self {
            base: IADsVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            DefaultContainer: DefaultContainer::<Impl, IMPL_OFFSET>,
            SetDefaultContainer: SetDefaultContainer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsNamespaces as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsNetAddressImpl: Sized + IDispatchImpl {
    fn AddressType(&mut self) -> ::windows::core::Result<i32>;
    fn SetAddressType(&mut self, lnaddresstype: i32) -> ::windows::core::Result<()>;
    fn Address(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetAddress(&mut self, vaddress: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsNetAddressVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsNetAddressImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsNetAddressVtbl {
        unsafe extern "system" fn AddressType<Impl: IADsNetAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddressType() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAddressType<Impl: IADsNetAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnaddresstype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAddressType(::core::mem::transmute_copy(&lnaddresstype)).into()
        }
        unsafe extern "system" fn Address<Impl: IADsNetAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Address() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAddress<Impl: IADsNetAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vaddress: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAddress(::core::mem::transmute_copy(&vaddress)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AddressType: AddressType::<Impl, IMPL_OFFSET>,
            SetAddressType: SetAddressType::<Impl, IMPL_OFFSET>,
            Address: Address::<Impl, IMPL_OFFSET>,
            SetAddress: SetAddress::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsNetAddress as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsOImpl: Sized + IDispatchImpl + IADsImpl {
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, bstrdescription: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn LocalityName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLocalityName(&mut self, bstrlocalityname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PostalAddress(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPostalAddress(&mut self, bstrpostaladdress: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn TelephoneNumber(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetTelephoneNumber(&mut self, bstrtelephonenumber: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FaxNumber(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetFaxNumber(&mut self, bstrfaxnumber: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SeeAlso(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetSeeAlso(&mut self, vseealso: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsOVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsOImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsOVtbl {
        unsafe extern "system" fn Description<Impl: IADsOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IADsOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&bstrdescription)).into()
        }
        unsafe extern "system" fn LocalityName<Impl: IADsOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalityName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalityName<Impl: IADsOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlocalityname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocalityName(::core::mem::transmute_copy(&bstrlocalityname)).into()
        }
        unsafe extern "system" fn PostalAddress<Impl: IADsOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PostalAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPostalAddress<Impl: IADsOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpostaladdress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPostalAddress(::core::mem::transmute_copy(&bstrpostaladdress)).into()
        }
        unsafe extern "system" fn TelephoneNumber<Impl: IADsOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TelephoneNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTelephoneNumber<Impl: IADsOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtelephonenumber: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTelephoneNumber(::core::mem::transmute_copy(&bstrtelephonenumber)).into()
        }
        unsafe extern "system" fn FaxNumber<Impl: IADsOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FaxNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFaxNumber<Impl: IADsOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfaxnumber: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFaxNumber(::core::mem::transmute_copy(&bstrfaxnumber)).into()
        }
        unsafe extern "system" fn SeeAlso<Impl: IADsOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SeeAlso() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSeeAlso<Impl: IADsOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vseealso: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSeeAlso(::core::mem::transmute_copy(&vseealso)).into()
        }
        Self {
            base: IADsVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Description: Description::<Impl, IMPL_OFFSET>,
            SetDescription: SetDescription::<Impl, IMPL_OFFSET>,
            LocalityName: LocalityName::<Impl, IMPL_OFFSET>,
            SetLocalityName: SetLocalityName::<Impl, IMPL_OFFSET>,
            PostalAddress: PostalAddress::<Impl, IMPL_OFFSET>,
            SetPostalAddress: SetPostalAddress::<Impl, IMPL_OFFSET>,
            TelephoneNumber: TelephoneNumber::<Impl, IMPL_OFFSET>,
            SetTelephoneNumber: SetTelephoneNumber::<Impl, IMPL_OFFSET>,
            FaxNumber: FaxNumber::<Impl, IMPL_OFFSET>,
            SetFaxNumber: SetFaxNumber::<Impl, IMPL_OFFSET>,
            SeeAlso: SeeAlso::<Impl, IMPL_OFFSET>,
            SetSeeAlso: SetSeeAlso::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsO as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsOUImpl: Sized + IDispatchImpl + IADsImpl {
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, bstrdescription: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn LocalityName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLocalityName(&mut self, bstrlocalityname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PostalAddress(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPostalAddress(&mut self, bstrpostaladdress: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn TelephoneNumber(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetTelephoneNumber(&mut self, bstrtelephonenumber: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FaxNumber(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetFaxNumber(&mut self, bstrfaxnumber: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SeeAlso(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetSeeAlso(&mut self, vseealso: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn BusinessCategory(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetBusinessCategory(&mut self, bstrbusinesscategory: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsOUVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsOUImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsOUVtbl {
        unsafe extern "system" fn Description<Impl: IADsOUImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IADsOUImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&bstrdescription)).into()
        }
        unsafe extern "system" fn LocalityName<Impl: IADsOUImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalityName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalityName<Impl: IADsOUImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlocalityname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocalityName(::core::mem::transmute_copy(&bstrlocalityname)).into()
        }
        unsafe extern "system" fn PostalAddress<Impl: IADsOUImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PostalAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPostalAddress<Impl: IADsOUImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpostaladdress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPostalAddress(::core::mem::transmute_copy(&bstrpostaladdress)).into()
        }
        unsafe extern "system" fn TelephoneNumber<Impl: IADsOUImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TelephoneNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTelephoneNumber<Impl: IADsOUImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtelephonenumber: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTelephoneNumber(::core::mem::transmute_copy(&bstrtelephonenumber)).into()
        }
        unsafe extern "system" fn FaxNumber<Impl: IADsOUImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FaxNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFaxNumber<Impl: IADsOUImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfaxnumber: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFaxNumber(::core::mem::transmute_copy(&bstrfaxnumber)).into()
        }
        unsafe extern "system" fn SeeAlso<Impl: IADsOUImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SeeAlso() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSeeAlso<Impl: IADsOUImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vseealso: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSeeAlso(::core::mem::transmute_copy(&vseealso)).into()
        }
        unsafe extern "system" fn BusinessCategory<Impl: IADsOUImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BusinessCategory() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBusinessCategory<Impl: IADsOUImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbusinesscategory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBusinessCategory(::core::mem::transmute_copy(&bstrbusinesscategory)).into()
        }
        Self {
            base: IADsVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Description: Description::<Impl, IMPL_OFFSET>,
            SetDescription: SetDescription::<Impl, IMPL_OFFSET>,
            LocalityName: LocalityName::<Impl, IMPL_OFFSET>,
            SetLocalityName: SetLocalityName::<Impl, IMPL_OFFSET>,
            PostalAddress: PostalAddress::<Impl, IMPL_OFFSET>,
            SetPostalAddress: SetPostalAddress::<Impl, IMPL_OFFSET>,
            TelephoneNumber: TelephoneNumber::<Impl, IMPL_OFFSET>,
            SetTelephoneNumber: SetTelephoneNumber::<Impl, IMPL_OFFSET>,
            FaxNumber: FaxNumber::<Impl, IMPL_OFFSET>,
            SetFaxNumber: SetFaxNumber::<Impl, IMPL_OFFSET>,
            SeeAlso: SeeAlso::<Impl, IMPL_OFFSET>,
            SetSeeAlso: SetSeeAlso::<Impl, IMPL_OFFSET>,
            BusinessCategory: BusinessCategory::<Impl, IMPL_OFFSET>,
            SetBusinessCategory: SetBusinessCategory::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsOU as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsObjectOptionsImpl: Sized + IDispatchImpl {
    fn GetOption(&mut self, lnoption: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetOption(&mut self, lnoption: i32, vvalue: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsObjectOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsObjectOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsObjectOptionsVtbl {
        unsafe extern "system" fn GetOption<Impl: IADsObjectOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnoption: i32, pvvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOption(::core::mem::transmute_copy(&lnoption)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOption<Impl: IADsObjectOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnoption: i32, vvalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOption(::core::mem::transmute_copy(&lnoption), ::core::mem::transmute_copy(&vvalue)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetOption: GetOption::<Impl, IMPL_OFFSET>,
            SetOption: SetOption::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsObjectOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsOctetListImpl: Sized + IDispatchImpl {
    fn OctetList(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetOctetList(&mut self, voctetlist: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsOctetListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsOctetListImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsOctetListVtbl {
        unsafe extern "system" fn OctetList<Impl: IADsOctetListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OctetList() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOctetList<Impl: IADsOctetListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, voctetlist: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOctetList(::core::mem::transmute_copy(&voctetlist)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            OctetList: OctetList::<Impl, IMPL_OFFSET>,
            SetOctetList: SetOctetList::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsOctetList as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsOpenDSObjectImpl: Sized + IDispatchImpl {
    fn OpenDSObject(&mut self, lpszdnname: super::super::Foundation::BSTR, lpszusername: super::super::Foundation::BSTR, lpszpassword: super::super::Foundation::BSTR, lnreserved: i32) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsOpenDSObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsOpenDSObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsOpenDSObjectVtbl {
        unsafe extern "system" fn OpenDSObject<Impl: IADsOpenDSObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszdnname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lpszusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lpszpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lnreserved: i32, ppoledsobj: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenDSObject(::core::mem::transmute_copy(&lpszdnname), ::core::mem::transmute_copy(&lpszusername), ::core::mem::transmute_copy(&lpszpassword), ::core::mem::transmute_copy(&lnreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppoledsobj = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), OpenDSObject: OpenDSObject::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsOpenDSObject as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsPathImpl: Sized + IDispatchImpl {
    fn Type(&mut self) -> ::windows::core::Result<i32>;
    fn SetType(&mut self, lntype: i32) -> ::windows::core::Result<()>;
    fn VolumeName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetVolumeName(&mut self, bstrvolumename: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Path(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPath(&mut self, bstrpath: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsPathVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsPathImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsPathVtbl {
        unsafe extern "system" fn Type<Impl: IADsPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Impl: IADsPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lntype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetType(::core::mem::transmute_copy(&lntype)).into()
        }
        unsafe extern "system" fn VolumeName<Impl: IADsPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VolumeName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVolumeName<Impl: IADsPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrvolumename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVolumeName(::core::mem::transmute_copy(&bstrvolumename)).into()
        }
        unsafe extern "system" fn Path<Impl: IADsPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPath<Impl: IADsPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPath(::core::mem::transmute_copy(&bstrpath)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Type: Type::<Impl, IMPL_OFFSET>,
            SetType: SetType::<Impl, IMPL_OFFSET>,
            VolumeName: VolumeName::<Impl, IMPL_OFFSET>,
            SetVolumeName: SetVolumeName::<Impl, IMPL_OFFSET>,
            Path: Path::<Impl, IMPL_OFFSET>,
            SetPath: SetPath::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsPath as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsPathnameImpl: Sized + IDispatchImpl {
    fn Set(&mut self, bstradspath: super::super::Foundation::BSTR, lnsettype: i32) -> ::windows::core::Result<()>;
    fn SetDisplayType(&mut self, lndisplaytype: i32) -> ::windows::core::Result<()>;
    fn Retrieve(&mut self, lnformattype: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetNumElements(&mut self) -> ::windows::core::Result<i32>;
    fn GetElement(&mut self, lnelementindex: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn AddLeafElement(&mut self, bstrleafelement: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RemoveLeafElement(&mut self) -> ::windows::core::Result<()>;
    fn CopyPath(&mut self) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn GetEscapedElement(&mut self, lnreserved: i32, bstrinstr: super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn EscapedMode(&mut self) -> ::windows::core::Result<i32>;
    fn SetEscapedMode(&mut self, lnescapedmode: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsPathnameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsPathnameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsPathnameVtbl {
        unsafe extern "system" fn Set<Impl: IADsPathnameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradspath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lnsettype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Set(::core::mem::transmute_copy(&bstradspath), ::core::mem::transmute_copy(&lnsettype)).into()
        }
        unsafe extern "system" fn SetDisplayType<Impl: IADsPathnameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lndisplaytype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayType(::core::mem::transmute_copy(&lndisplaytype)).into()
        }
        unsafe extern "system" fn Retrieve<Impl: IADsPathnameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnformattype: i32, pbstradspath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Retrieve(::core::mem::transmute_copy(&lnformattype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstradspath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumElements<Impl: IADsPathnameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plnnumpathelements: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumElements() {
                ::core::result::Result::Ok(ok__) => {
                    *plnnumpathelements = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetElement<Impl: IADsPathnameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnelementindex: i32, pbstrelement: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetElement(::core::mem::transmute_copy(&lnelementindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrelement = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddLeafElement<Impl: IADsPathnameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrleafelement: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddLeafElement(::core::mem::transmute_copy(&bstrleafelement)).into()
        }
        unsafe extern "system" fn RemoveLeafElement<Impl: IADsPathnameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveLeafElement().into()
        }
        unsafe extern "system" fn CopyPath<Impl: IADsPathnameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppadspath: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopyPath() {
                ::core::result::Result::Ok(ok__) => {
                    *ppadspath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEscapedElement<Impl: IADsPathnameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnreserved: i32, bstrinstr: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstroutstr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEscapedElement(::core::mem::transmute_copy(&lnreserved), ::core::mem::transmute_copy(&bstrinstr)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstroutstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EscapedMode<Impl: IADsPathnameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EscapedMode() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEscapedMode<Impl: IADsPathnameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnescapedmode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEscapedMode(::core::mem::transmute_copy(&lnescapedmode)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Set: Set::<Impl, IMPL_OFFSET>,
            SetDisplayType: SetDisplayType::<Impl, IMPL_OFFSET>,
            Retrieve: Retrieve::<Impl, IMPL_OFFSET>,
            GetNumElements: GetNumElements::<Impl, IMPL_OFFSET>,
            GetElement: GetElement::<Impl, IMPL_OFFSET>,
            AddLeafElement: AddLeafElement::<Impl, IMPL_OFFSET>,
            RemoveLeafElement: RemoveLeafElement::<Impl, IMPL_OFFSET>,
            CopyPath: CopyPath::<Impl, IMPL_OFFSET>,
            GetEscapedElement: GetEscapedElement::<Impl, IMPL_OFFSET>,
            EscapedMode: EscapedMode::<Impl, IMPL_OFFSET>,
            SetEscapedMode: SetEscapedMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsPathname as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsPostalAddressImpl: Sized + IDispatchImpl {
    fn PostalAddress(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetPostalAddress(&mut self, vpostaladdress: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsPostalAddressVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsPostalAddressImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsPostalAddressVtbl {
        unsafe extern "system" fn PostalAddress<Impl: IADsPostalAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PostalAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPostalAddress<Impl: IADsPostalAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vpostaladdress: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPostalAddress(::core::mem::transmute_copy(&vpostaladdress)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            PostalAddress: PostalAddress::<Impl, IMPL_OFFSET>,
            SetPostalAddress: SetPostalAddress::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsPostalAddress as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsPrintJobImpl: Sized + IDispatchImpl + IADsImpl {
    fn HostPrintQueue(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn User(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn UserPath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn TimeSubmitted(&mut self) -> ::windows::core::Result<f64>;
    fn TotalPages(&mut self) -> ::windows::core::Result<i32>;
    fn Size(&mut self) -> ::windows::core::Result<i32>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, bstrdescription: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Priority(&mut self) -> ::windows::core::Result<i32>;
    fn SetPriority(&mut self, lnpriority: i32) -> ::windows::core::Result<()>;
    fn StartTime(&mut self) -> ::windows::core::Result<f64>;
    fn SetStartTime(&mut self, dastarttime: f64) -> ::windows::core::Result<()>;
    fn UntilTime(&mut self) -> ::windows::core::Result<f64>;
    fn SetUntilTime(&mut self, dauntiltime: f64) -> ::windows::core::Result<()>;
    fn Notify(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetNotify(&mut self, bstrnotify: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn NotifyPath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetNotifyPath(&mut self, bstrnotifypath: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsPrintJobVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintJobImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsPrintJobVtbl {
        unsafe extern "system" fn HostPrintQueue<Impl: IADsPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HostPrintQueue() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn User<Impl: IADsPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).User() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserPath<Impl: IADsPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserPath() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TimeSubmitted<Impl: IADsPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TimeSubmitted() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalPages<Impl: IADsPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalPages() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<Impl: IADsPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Size() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IADsPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IADsPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&bstrdescription)).into()
        }
        unsafe extern "system" fn Priority<Impl: IADsPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Priority() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Impl: IADsPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnpriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPriority(::core::mem::transmute_copy(&lnpriority)).into()
        }
        unsafe extern "system" fn StartTime<Impl: IADsPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartTime<Impl: IADsPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dastarttime: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStartTime(::core::mem::transmute_copy(&dastarttime)).into()
        }
        unsafe extern "system" fn UntilTime<Impl: IADsPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UntilTime() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUntilTime<Impl: IADsPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dauntiltime: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUntilTime(::core::mem::transmute_copy(&dauntiltime)).into()
        }
        unsafe extern "system" fn Notify<Impl: IADsPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Notify() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotify<Impl: IADsPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrnotify: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNotify(::core::mem::transmute_copy(&bstrnotify)).into()
        }
        unsafe extern "system" fn NotifyPath<Impl: IADsPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotifyPath() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotifyPath<Impl: IADsPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrnotifypath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNotifyPath(::core::mem::transmute_copy(&bstrnotifypath)).into()
        }
        Self {
            base: IADsVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            HostPrintQueue: HostPrintQueue::<Impl, IMPL_OFFSET>,
            User: User::<Impl, IMPL_OFFSET>,
            UserPath: UserPath::<Impl, IMPL_OFFSET>,
            TimeSubmitted: TimeSubmitted::<Impl, IMPL_OFFSET>,
            TotalPages: TotalPages::<Impl, IMPL_OFFSET>,
            Size: Size::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            SetDescription: SetDescription::<Impl, IMPL_OFFSET>,
            Priority: Priority::<Impl, IMPL_OFFSET>,
            SetPriority: SetPriority::<Impl, IMPL_OFFSET>,
            StartTime: StartTime::<Impl, IMPL_OFFSET>,
            SetStartTime: SetStartTime::<Impl, IMPL_OFFSET>,
            UntilTime: UntilTime::<Impl, IMPL_OFFSET>,
            SetUntilTime: SetUntilTime::<Impl, IMPL_OFFSET>,
            Notify: Notify::<Impl, IMPL_OFFSET>,
            SetNotify: SetNotify::<Impl, IMPL_OFFSET>,
            NotifyPath: NotifyPath::<Impl, IMPL_OFFSET>,
            SetNotifyPath: SetNotifyPath::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsPrintJob as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsPrintJobOperationsImpl: Sized + IDispatchImpl + IADsImpl {
    fn Status(&mut self) -> ::windows::core::Result<i32>;
    fn TimeElapsed(&mut self) -> ::windows::core::Result<i32>;
    fn PagesPrinted(&mut self) -> ::windows::core::Result<i32>;
    fn Position(&mut self) -> ::windows::core::Result<i32>;
    fn SetPosition(&mut self, lnposition: i32) -> ::windows::core::Result<()>;
    fn Pause(&mut self) -> ::windows::core::Result<()>;
    fn Resume(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsPrintJobOperationsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintJobOperationsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsPrintJobOperationsVtbl {
        unsafe extern "system" fn Status<Impl: IADsPrintJobOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TimeElapsed<Impl: IADsPrintJobOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TimeElapsed() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PagesPrinted<Impl: IADsPrintJobOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PagesPrinted() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Impl: IADsPrintJobOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPosition<Impl: IADsPrintJobOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnposition: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPosition(::core::mem::transmute_copy(&lnposition)).into()
        }
        unsafe extern "system" fn Pause<Impl: IADsPrintJobOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn Resume<Impl: IADsPrintJobOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Resume().into()
        }
        Self {
            base: IADsVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            TimeElapsed: TimeElapsed::<Impl, IMPL_OFFSET>,
            PagesPrinted: PagesPrinted::<Impl, IMPL_OFFSET>,
            Position: Position::<Impl, IMPL_OFFSET>,
            SetPosition: SetPosition::<Impl, IMPL_OFFSET>,
            Pause: Pause::<Impl, IMPL_OFFSET>,
            Resume: Resume::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsPrintJobOperations as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsPrintQueueImpl: Sized + IDispatchImpl + IADsImpl {
    fn PrinterPath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPrinterPath(&mut self, bstrprinterpath: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Model(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetModel(&mut self, bstrmodel: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Datatype(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDatatype(&mut self, bstrdatatype: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PrintProcessor(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPrintProcessor(&mut self, bstrprintprocessor: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, bstrdescription: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Location(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLocation(&mut self, bstrlocation: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn StartTime(&mut self) -> ::windows::core::Result<f64>;
    fn SetStartTime(&mut self, dastarttime: f64) -> ::windows::core::Result<()>;
    fn UntilTime(&mut self) -> ::windows::core::Result<f64>;
    fn SetUntilTime(&mut self, dauntiltime: f64) -> ::windows::core::Result<()>;
    fn DefaultJobPriority(&mut self) -> ::windows::core::Result<i32>;
    fn SetDefaultJobPriority(&mut self, lndefaultjobpriority: i32) -> ::windows::core::Result<()>;
    fn Priority(&mut self) -> ::windows::core::Result<i32>;
    fn SetPriority(&mut self, lnpriority: i32) -> ::windows::core::Result<()>;
    fn BannerPage(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetBannerPage(&mut self, bstrbannerpage: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PrintDevices(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetPrintDevices(&mut self, vprintdevices: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn NetAddresses(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetNetAddresses(&mut self, vnetaddresses: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsPrintQueueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintQueueImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsPrintQueueVtbl {
        unsafe extern "system" fn PrinterPath<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrinterPath() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrinterPath<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprinterpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrinterPath(::core::mem::transmute_copy(&bstrprinterpath)).into()
        }
        unsafe extern "system" fn Model<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Model() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetModel<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmodel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetModel(::core::mem::transmute_copy(&bstrmodel)).into()
        }
        unsafe extern "system" fn Datatype<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Datatype() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDatatype<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdatatype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDatatype(::core::mem::transmute_copy(&bstrdatatype)).into()
        }
        unsafe extern "system" fn PrintProcessor<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrintProcessor() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrintProcessor<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprintprocessor: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrintProcessor(::core::mem::transmute_copy(&bstrprintprocessor)).into()
        }
        unsafe extern "system" fn Description<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&bstrdescription)).into()
        }
        unsafe extern "system" fn Location<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Location() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocation<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlocation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocation(::core::mem::transmute_copy(&bstrlocation)).into()
        }
        unsafe extern "system" fn StartTime<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartTime<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dastarttime: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStartTime(::core::mem::transmute_copy(&dastarttime)).into()
        }
        unsafe extern "system" fn UntilTime<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UntilTime() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUntilTime<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dauntiltime: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUntilTime(::core::mem::transmute_copy(&dauntiltime)).into()
        }
        unsafe extern "system" fn DefaultJobPriority<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultJobPriority() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultJobPriority<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lndefaultjobpriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultJobPriority(::core::mem::transmute_copy(&lndefaultjobpriority)).into()
        }
        unsafe extern "system" fn Priority<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Priority() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnpriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPriority(::core::mem::transmute_copy(&lnpriority)).into()
        }
        unsafe extern "system" fn BannerPage<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BannerPage() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBannerPage<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbannerpage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBannerPage(::core::mem::transmute_copy(&bstrbannerpage)).into()
        }
        unsafe extern "system" fn PrintDevices<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrintDevices() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrintDevices<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vprintdevices: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrintDevices(::core::mem::transmute_copy(&vprintdevices)).into()
        }
        unsafe extern "system" fn NetAddresses<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetAddresses() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetAddresses<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vnetaddresses: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNetAddresses(::core::mem::transmute_copy(&vnetaddresses)).into()
        }
        Self {
            base: IADsVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            PrinterPath: PrinterPath::<Impl, IMPL_OFFSET>,
            SetPrinterPath: SetPrinterPath::<Impl, IMPL_OFFSET>,
            Model: Model::<Impl, IMPL_OFFSET>,
            SetModel: SetModel::<Impl, IMPL_OFFSET>,
            Datatype: Datatype::<Impl, IMPL_OFFSET>,
            SetDatatype: SetDatatype::<Impl, IMPL_OFFSET>,
            PrintProcessor: PrintProcessor::<Impl, IMPL_OFFSET>,
            SetPrintProcessor: SetPrintProcessor::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            SetDescription: SetDescription::<Impl, IMPL_OFFSET>,
            Location: Location::<Impl, IMPL_OFFSET>,
            SetLocation: SetLocation::<Impl, IMPL_OFFSET>,
            StartTime: StartTime::<Impl, IMPL_OFFSET>,
            SetStartTime: SetStartTime::<Impl, IMPL_OFFSET>,
            UntilTime: UntilTime::<Impl, IMPL_OFFSET>,
            SetUntilTime: SetUntilTime::<Impl, IMPL_OFFSET>,
            DefaultJobPriority: DefaultJobPriority::<Impl, IMPL_OFFSET>,
            SetDefaultJobPriority: SetDefaultJobPriority::<Impl, IMPL_OFFSET>,
            Priority: Priority::<Impl, IMPL_OFFSET>,
            SetPriority: SetPriority::<Impl, IMPL_OFFSET>,
            BannerPage: BannerPage::<Impl, IMPL_OFFSET>,
            SetBannerPage: SetBannerPage::<Impl, IMPL_OFFSET>,
            PrintDevices: PrintDevices::<Impl, IMPL_OFFSET>,
            SetPrintDevices: SetPrintDevices::<Impl, IMPL_OFFSET>,
            NetAddresses: NetAddresses::<Impl, IMPL_OFFSET>,
            SetNetAddresses: SetNetAddresses::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsPrintQueue as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsPrintQueueOperationsImpl: Sized + IDispatchImpl + IADsImpl {
    fn Status(&mut self) -> ::windows::core::Result<i32>;
    fn PrintJobs(&mut self) -> ::windows::core::Result<IADsCollection>;
    fn Pause(&mut self) -> ::windows::core::Result<()>;
    fn Resume(&mut self) -> ::windows::core::Result<()>;
    fn Purge(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsPrintQueueOperationsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintQueueOperationsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsPrintQueueOperationsVtbl {
        unsafe extern "system" fn Status<Impl: IADsPrintQueueOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrintJobs<Impl: IADsPrintQueueOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrintJobs() {
                ::core::result::Result::Ok(ok__) => {
                    *pobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pause<Impl: IADsPrintQueueOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn Resume<Impl: IADsPrintQueueOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Resume().into()
        }
        unsafe extern "system" fn Purge<Impl: IADsPrintQueueOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Purge().into()
        }
        Self {
            base: IADsVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            PrintJobs: PrintJobs::<Impl, IMPL_OFFSET>,
            Pause: Pause::<Impl, IMPL_OFFSET>,
            Resume: Resume::<Impl, IMPL_OFFSET>,
            Purge: Purge::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsPrintQueueOperations as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsPropertyImpl: Sized + IDispatchImpl + IADsImpl {
    fn OID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetOID(&mut self, bstroid: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Syntax(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetSyntax(&mut self, bstrsyntax: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn MaxRange(&mut self) -> ::windows::core::Result<i32>;
    fn SetMaxRange(&mut self, lnmaxrange: i32) -> ::windows::core::Result<()>;
    fn MinRange(&mut self) -> ::windows::core::Result<i32>;
    fn SetMinRange(&mut self, lnminrange: i32) -> ::windows::core::Result<()>;
    fn MultiValued(&mut self) -> ::windows::core::Result<i16>;
    fn SetMultiValued(&mut self, fmultivalued: i16) -> ::windows::core::Result<()>;
    fn Qualifiers(&mut self) -> ::windows::core::Result<IADsCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsPropertyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsPropertyVtbl {
        unsafe extern "system" fn OID<Impl: IADsPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OID() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOID<Impl: IADsPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstroid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOID(::core::mem::transmute_copy(&bstroid)).into()
        }
        unsafe extern "system" fn Syntax<Impl: IADsPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Syntax() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSyntax<Impl: IADsPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsyntax: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSyntax(::core::mem::transmute_copy(&bstrsyntax)).into()
        }
        unsafe extern "system" fn MaxRange<Impl: IADsPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxRange() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxRange<Impl: IADsPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnmaxrange: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxRange(::core::mem::transmute_copy(&lnmaxrange)).into()
        }
        unsafe extern "system" fn MinRange<Impl: IADsPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinRange() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinRange<Impl: IADsPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnminrange: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinRange(::core::mem::transmute_copy(&lnminrange)).into()
        }
        unsafe extern "system" fn MultiValued<Impl: IADsPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MultiValued() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMultiValued<Impl: IADsPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmultivalued: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMultiValued(::core::mem::transmute_copy(&fmultivalued)).into()
        }
        unsafe extern "system" fn Qualifiers<Impl: IADsPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqualifiers: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Qualifiers() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqualifiers = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IADsVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            OID: OID::<Impl, IMPL_OFFSET>,
            SetOID: SetOID::<Impl, IMPL_OFFSET>,
            Syntax: Syntax::<Impl, IMPL_OFFSET>,
            SetSyntax: SetSyntax::<Impl, IMPL_OFFSET>,
            MaxRange: MaxRange::<Impl, IMPL_OFFSET>,
            SetMaxRange: SetMaxRange::<Impl, IMPL_OFFSET>,
            MinRange: MinRange::<Impl, IMPL_OFFSET>,
            SetMinRange: SetMinRange::<Impl, IMPL_OFFSET>,
            MultiValued: MultiValued::<Impl, IMPL_OFFSET>,
            SetMultiValued: SetMultiValued::<Impl, IMPL_OFFSET>,
            Qualifiers: Qualifiers::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsProperty as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsPropertyEntryImpl: Sized + IDispatchImpl {
    fn Clear(&mut self) -> ::windows::core::Result<()>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&mut self, bstrname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ADsType(&mut self) -> ::windows::core::Result<i32>;
    fn SetADsType(&mut self, lnadstype: i32) -> ::windows::core::Result<()>;
    fn ControlCode(&mut self) -> ::windows::core::Result<i32>;
    fn SetControlCode(&mut self, lncontrolcode: i32) -> ::windows::core::Result<()>;
    fn Values(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetValues(&mut self, vvalues: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsPropertyEntryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyEntryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsPropertyEntryVtbl {
        unsafe extern "system" fn Clear<Impl: IADsPropertyEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn Name<Impl: IADsPropertyEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IADsPropertyEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&bstrname)).into()
        }
        unsafe extern "system" fn ADsType<Impl: IADsPropertyEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ADsType() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetADsType<Impl: IADsPropertyEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnadstype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetADsType(::core::mem::transmute_copy(&lnadstype)).into()
        }
        unsafe extern "system" fn ControlCode<Impl: IADsPropertyEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ControlCode() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetControlCode<Impl: IADsPropertyEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lncontrolcode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetControlCode(::core::mem::transmute_copy(&lncontrolcode)).into()
        }
        unsafe extern "system" fn Values<Impl: IADsPropertyEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Values() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValues<Impl: IADsPropertyEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vvalues: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValues(::core::mem::transmute_copy(&vvalues)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Clear: Clear::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            ADsType: ADsType::<Impl, IMPL_OFFSET>,
            SetADsType: SetADsType::<Impl, IMPL_OFFSET>,
            ControlCode: ControlCode::<Impl, IMPL_OFFSET>,
            SetControlCode: SetControlCode::<Impl, IMPL_OFFSET>,
            Values: Values::<Impl, IMPL_OFFSET>,
            SetValues: SetValues::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsPropertyEntry as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsPropertyListImpl: Sized + IDispatchImpl {
    fn PropertyCount(&mut self) -> ::windows::core::Result<i32>;
    fn Next(&mut self, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT;
    fn Skip(&mut self, celements: i32) -> ::windows::core::HRESULT;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Item(&mut self, varindex: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn GetPropertyItem(&mut self, bstrname: super::super::Foundation::BSTR, lnadstype: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn PutPropertyItem(&mut self, vardata: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn ResetPropertyItem(&mut self, varentry: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn PurgePropertyList(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsPropertyListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyListImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsPropertyListVtbl {
        unsafe extern "system" fn PropertyCount<Impl: IADsPropertyListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PropertyCount() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IADsPropertyListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&pvariant))
        }
        unsafe extern "system" fn Skip<Impl: IADsPropertyListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celements: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celements))
        }
        unsafe extern "system" fn Reset<Impl: IADsPropertyListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Item<Impl: IADsPropertyListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyItem<Impl: IADsPropertyListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lnadstype: i32, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertyItem(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&lnadstype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutPropertyItem<Impl: IADsPropertyListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vardata: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PutPropertyItem(::core::mem::transmute_copy(&vardata)).into()
        }
        unsafe extern "system" fn ResetPropertyItem<Impl: IADsPropertyListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varentry: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResetPropertyItem(::core::mem::transmute_copy(&varentry)).into()
        }
        unsafe extern "system" fn PurgePropertyList<Impl: IADsPropertyListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PurgePropertyList().into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            PropertyCount: PropertyCount::<Impl, IMPL_OFFSET>,
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            GetPropertyItem: GetPropertyItem::<Impl, IMPL_OFFSET>,
            PutPropertyItem: PutPropertyItem::<Impl, IMPL_OFFSET>,
            ResetPropertyItem: ResetPropertyItem::<Impl, IMPL_OFFSET>,
            PurgePropertyList: PurgePropertyList::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsPropertyList as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsPropertyValueImpl: Sized + IDispatchImpl {
    fn Clear(&mut self) -> ::windows::core::Result<()>;
    fn ADsType(&mut self) -> ::windows::core::Result<i32>;
    fn SetADsType(&mut self, lnadstype: i32) -> ::windows::core::Result<()>;
    fn DNString(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDNString(&mut self, bstrdnstring: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn CaseExactString(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetCaseExactString(&mut self, bstrcaseexactstring: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn CaseIgnoreString(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetCaseIgnoreString(&mut self, bstrcaseignorestring: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PrintableString(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPrintableString(&mut self, bstrprintablestring: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn NumericString(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetNumericString(&mut self, bstrnumericstring: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Boolean(&mut self) -> ::windows::core::Result<i32>;
    fn SetBoolean(&mut self, lnboolean: i32) -> ::windows::core::Result<()>;
    fn Integer(&mut self) -> ::windows::core::Result<i32>;
    fn SetInteger(&mut self, lninteger: i32) -> ::windows::core::Result<()>;
    fn OctetString(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetOctetString(&mut self, voctetstring: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SecurityDescriptor(&mut self) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn SetSecurityDescriptor(&mut self, psecuritydescriptor: ::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn LargeInteger(&mut self) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn SetLargeInteger(&mut self, plargeinteger: ::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn UTCTime(&mut self) -> ::windows::core::Result<f64>;
    fn SetUTCTime(&mut self, dautctime: f64) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsPropertyValueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyValueImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsPropertyValueVtbl {
        unsafe extern "system" fn Clear<Impl: IADsPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn ADsType<Impl: IADsPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ADsType() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetADsType<Impl: IADsPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnadstype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetADsType(::core::mem::transmute_copy(&lnadstype)).into()
        }
        unsafe extern "system" fn DNString<Impl: IADsPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DNString() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDNString<Impl: IADsPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdnstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDNString(::core::mem::transmute_copy(&bstrdnstring)).into()
        }
        unsafe extern "system" fn CaseExactString<Impl: IADsPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CaseExactString() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCaseExactString<Impl: IADsPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcaseexactstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCaseExactString(::core::mem::transmute_copy(&bstrcaseexactstring)).into()
        }
        unsafe extern "system" fn CaseIgnoreString<Impl: IADsPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CaseIgnoreString() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCaseIgnoreString<Impl: IADsPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcaseignorestring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCaseIgnoreString(::core::mem::transmute_copy(&bstrcaseignorestring)).into()
        }
        unsafe extern "system" fn PrintableString<Impl: IADsPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrintableString() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrintableString<Impl: IADsPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprintablestring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrintableString(::core::mem::transmute_copy(&bstrprintablestring)).into()
        }
        unsafe extern "system" fn NumericString<Impl: IADsPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NumericString() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNumericString<Impl: IADsPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrnumericstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNumericString(::core::mem::transmute_copy(&bstrnumericstring)).into()
        }
        unsafe extern "system" fn Boolean<Impl: IADsPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Boolean() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBoolean<Impl: IADsPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnboolean: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBoolean(::core::mem::transmute_copy(&lnboolean)).into()
        }
        unsafe extern "system" fn Integer<Impl: IADsPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Integer() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInteger<Impl: IADsPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lninteger: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInteger(::core::mem::transmute_copy(&lninteger)).into()
        }
        unsafe extern "system" fn OctetString<Impl: IADsPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OctetString() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOctetString<Impl: IADsPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, voctetstring: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOctetString(::core::mem::transmute_copy(&voctetstring)).into()
        }
        unsafe extern "system" fn SecurityDescriptor<Impl: IADsPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SecurityDescriptor() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityDescriptor<Impl: IADsPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psecuritydescriptor: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSecurityDescriptor(::core::mem::transmute(&psecuritydescriptor)).into()
        }
        unsafe extern "system" fn LargeInteger<Impl: IADsPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LargeInteger() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLargeInteger<Impl: IADsPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plargeinteger: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLargeInteger(::core::mem::transmute(&plargeinteger)).into()
        }
        unsafe extern "system" fn UTCTime<Impl: IADsPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UTCTime() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUTCTime<Impl: IADsPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dautctime: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUTCTime(::core::mem::transmute_copy(&dautctime)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Clear: Clear::<Impl, IMPL_OFFSET>,
            ADsType: ADsType::<Impl, IMPL_OFFSET>,
            SetADsType: SetADsType::<Impl, IMPL_OFFSET>,
            DNString: DNString::<Impl, IMPL_OFFSET>,
            SetDNString: SetDNString::<Impl, IMPL_OFFSET>,
            CaseExactString: CaseExactString::<Impl, IMPL_OFFSET>,
            SetCaseExactString: SetCaseExactString::<Impl, IMPL_OFFSET>,
            CaseIgnoreString: CaseIgnoreString::<Impl, IMPL_OFFSET>,
            SetCaseIgnoreString: SetCaseIgnoreString::<Impl, IMPL_OFFSET>,
            PrintableString: PrintableString::<Impl, IMPL_OFFSET>,
            SetPrintableString: SetPrintableString::<Impl, IMPL_OFFSET>,
            NumericString: NumericString::<Impl, IMPL_OFFSET>,
            SetNumericString: SetNumericString::<Impl, IMPL_OFFSET>,
            Boolean: Boolean::<Impl, IMPL_OFFSET>,
            SetBoolean: SetBoolean::<Impl, IMPL_OFFSET>,
            Integer: Integer::<Impl, IMPL_OFFSET>,
            SetInteger: SetInteger::<Impl, IMPL_OFFSET>,
            OctetString: OctetString::<Impl, IMPL_OFFSET>,
            SetOctetString: SetOctetString::<Impl, IMPL_OFFSET>,
            SecurityDescriptor: SecurityDescriptor::<Impl, IMPL_OFFSET>,
            SetSecurityDescriptor: SetSecurityDescriptor::<Impl, IMPL_OFFSET>,
            LargeInteger: LargeInteger::<Impl, IMPL_OFFSET>,
            SetLargeInteger: SetLargeInteger::<Impl, IMPL_OFFSET>,
            UTCTime: UTCTime::<Impl, IMPL_OFFSET>,
            SetUTCTime: SetUTCTime::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsPropertyValue as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsPropertyValue2Impl: Sized + IDispatchImpl {
    fn GetObjectProperty(&mut self, lnadstype: *mut i32, pvprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn PutObjectProperty(&mut self, lnadstype: i32, vprop: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsPropertyValue2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyValue2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsPropertyValue2Vtbl {
        unsafe extern "system" fn GetObjectProperty<Impl: IADsPropertyValue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnadstype: *mut i32, pvprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetObjectProperty(::core::mem::transmute_copy(&lnadstype), ::core::mem::transmute_copy(&pvprop)).into()
        }
        unsafe extern "system" fn PutObjectProperty<Impl: IADsPropertyValue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnadstype: i32, vprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PutObjectProperty(::core::mem::transmute_copy(&lnadstype), ::core::mem::transmute_copy(&vprop)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetObjectProperty: GetObjectProperty::<Impl, IMPL_OFFSET>,
            PutObjectProperty: PutObjectProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsPropertyValue2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsReplicaPointerImpl: Sized + IDispatchImpl {
    fn ServerName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetServerName(&mut self, bstrservername: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ReplicaType(&mut self) -> ::windows::core::Result<i32>;
    fn SetReplicaType(&mut self, lnreplicatype: i32) -> ::windows::core::Result<()>;
    fn ReplicaNumber(&mut self) -> ::windows::core::Result<i32>;
    fn SetReplicaNumber(&mut self, lnreplicanumber: i32) -> ::windows::core::Result<()>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn SetCount(&mut self, lncount: i32) -> ::windows::core::Result<()>;
    fn ReplicaAddressHints(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetReplicaAddressHints(&mut self, vreplicaaddresshints: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsReplicaPointerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsReplicaPointerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsReplicaPointerVtbl {
        unsafe extern "system" fn ServerName<Impl: IADsReplicaPointerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServerName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServerName<Impl: IADsReplicaPointerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrservername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetServerName(::core::mem::transmute_copy(&bstrservername)).into()
        }
        unsafe extern "system" fn ReplicaType<Impl: IADsReplicaPointerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReplicaType() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReplicaType<Impl: IADsReplicaPointerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnreplicatype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReplicaType(::core::mem::transmute_copy(&lnreplicatype)).into()
        }
        unsafe extern "system" fn ReplicaNumber<Impl: IADsReplicaPointerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReplicaNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReplicaNumber<Impl: IADsReplicaPointerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnreplicanumber: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReplicaNumber(::core::mem::transmute_copy(&lnreplicanumber)).into()
        }
        unsafe extern "system" fn Count<Impl: IADsReplicaPointerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCount<Impl: IADsReplicaPointerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lncount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCount(::core::mem::transmute_copy(&lncount)).into()
        }
        unsafe extern "system" fn ReplicaAddressHints<Impl: IADsReplicaPointerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReplicaAddressHints() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReplicaAddressHints<Impl: IADsReplicaPointerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vreplicaaddresshints: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReplicaAddressHints(::core::mem::transmute_copy(&vreplicaaddresshints)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ServerName: ServerName::<Impl, IMPL_OFFSET>,
            SetServerName: SetServerName::<Impl, IMPL_OFFSET>,
            ReplicaType: ReplicaType::<Impl, IMPL_OFFSET>,
            SetReplicaType: SetReplicaType::<Impl, IMPL_OFFSET>,
            ReplicaNumber: ReplicaNumber::<Impl, IMPL_OFFSET>,
            SetReplicaNumber: SetReplicaNumber::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            SetCount: SetCount::<Impl, IMPL_OFFSET>,
            ReplicaAddressHints: ReplicaAddressHints::<Impl, IMPL_OFFSET>,
            SetReplicaAddressHints: SetReplicaAddressHints::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsReplicaPointer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsResourceImpl: Sized + IDispatchImpl + IADsImpl {
    fn User(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn UserPath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Path(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn LockCount(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsResourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsResourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsResourceVtbl {
        unsafe extern "system" fn User<Impl: IADsResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).User() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserPath<Impl: IADsResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserPath() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path<Impl: IADsResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockCount<Impl: IADsResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LockCount() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IADsVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            User: User::<Impl, IMPL_OFFSET>,
            UserPath: UserPath::<Impl, IMPL_OFFSET>,
            Path: Path::<Impl, IMPL_OFFSET>,
            LockCount: LockCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsResource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsSecurityDescriptorImpl: Sized + IDispatchImpl {
    fn Revision(&mut self) -> ::windows::core::Result<i32>;
    fn SetRevision(&mut self, lnrevision: i32) -> ::windows::core::Result<()>;
    fn Control(&mut self) -> ::windows::core::Result<i32>;
    fn SetControl(&mut self, lncontrol: i32) -> ::windows::core::Result<()>;
    fn Owner(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetOwner(&mut self, bstrowner: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn OwnerDefaulted(&mut self) -> ::windows::core::Result<i16>;
    fn SetOwnerDefaulted(&mut self, fownerdefaulted: i16) -> ::windows::core::Result<()>;
    fn Group(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetGroup(&mut self, bstrgroup: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GroupDefaulted(&mut self) -> ::windows::core::Result<i16>;
    fn SetGroupDefaulted(&mut self, fgroupdefaulted: i16) -> ::windows::core::Result<()>;
    fn DiscretionaryAcl(&mut self) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn SetDiscretionaryAcl(&mut self, pdiscretionaryacl: ::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn DaclDefaulted(&mut self) -> ::windows::core::Result<i16>;
    fn SetDaclDefaulted(&mut self, fdacldefaulted: i16) -> ::windows::core::Result<()>;
    fn SystemAcl(&mut self) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn SetSystemAcl(&mut self, psystemacl: ::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn SaclDefaulted(&mut self) -> ::windows::core::Result<i16>;
    fn SetSaclDefaulted(&mut self, fsacldefaulted: i16) -> ::windows::core::Result<()>;
    fn CopySecurityDescriptor(&mut self) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsSecurityDescriptorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsSecurityDescriptorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsSecurityDescriptorVtbl {
        unsafe extern "system" fn Revision<Impl: IADsSecurityDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Revision() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRevision<Impl: IADsSecurityDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnrevision: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRevision(::core::mem::transmute_copy(&lnrevision)).into()
        }
        unsafe extern "system" fn Control<Impl: IADsSecurityDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Control() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetControl<Impl: IADsSecurityDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lncontrol: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetControl(::core::mem::transmute_copy(&lncontrol)).into()
        }
        unsafe extern "system" fn Owner<Impl: IADsSecurityDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Owner() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOwner<Impl: IADsSecurityDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrowner: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOwner(::core::mem::transmute_copy(&bstrowner)).into()
        }
        unsafe extern "system" fn OwnerDefaulted<Impl: IADsSecurityDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OwnerDefaulted() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOwnerDefaulted<Impl: IADsSecurityDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fownerdefaulted: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOwnerDefaulted(::core::mem::transmute_copy(&fownerdefaulted)).into()
        }
        unsafe extern "system" fn Group<Impl: IADsSecurityDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Group() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGroup<Impl: IADsSecurityDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroup: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGroup(::core::mem::transmute_copy(&bstrgroup)).into()
        }
        unsafe extern "system" fn GroupDefaulted<Impl: IADsSecurityDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GroupDefaulted() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGroupDefaulted<Impl: IADsSecurityDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fgroupdefaulted: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGroupDefaulted(::core::mem::transmute_copy(&fgroupdefaulted)).into()
        }
        unsafe extern "system" fn DiscretionaryAcl<Impl: IADsSecurityDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DiscretionaryAcl() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDiscretionaryAcl<Impl: IADsSecurityDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdiscretionaryacl: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDiscretionaryAcl(::core::mem::transmute(&pdiscretionaryacl)).into()
        }
        unsafe extern "system" fn DaclDefaulted<Impl: IADsSecurityDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DaclDefaulted() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDaclDefaulted<Impl: IADsSecurityDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fdacldefaulted: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDaclDefaulted(::core::mem::transmute_copy(&fdacldefaulted)).into()
        }
        unsafe extern "system" fn SystemAcl<Impl: IADsSecurityDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemAcl() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSystemAcl<Impl: IADsSecurityDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psystemacl: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSystemAcl(::core::mem::transmute(&psystemacl)).into()
        }
        unsafe extern "system" fn SaclDefaulted<Impl: IADsSecurityDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaclDefaulted() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSaclDefaulted<Impl: IADsSecurityDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fsacldefaulted: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSaclDefaulted(::core::mem::transmute_copy(&fsacldefaulted)).into()
        }
        unsafe extern "system" fn CopySecurityDescriptor<Impl: IADsSecurityDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsecuritydescriptor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopySecurityDescriptor() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsecuritydescriptor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Revision: Revision::<Impl, IMPL_OFFSET>,
            SetRevision: SetRevision::<Impl, IMPL_OFFSET>,
            Control: Control::<Impl, IMPL_OFFSET>,
            SetControl: SetControl::<Impl, IMPL_OFFSET>,
            Owner: Owner::<Impl, IMPL_OFFSET>,
            SetOwner: SetOwner::<Impl, IMPL_OFFSET>,
            OwnerDefaulted: OwnerDefaulted::<Impl, IMPL_OFFSET>,
            SetOwnerDefaulted: SetOwnerDefaulted::<Impl, IMPL_OFFSET>,
            Group: Group::<Impl, IMPL_OFFSET>,
            SetGroup: SetGroup::<Impl, IMPL_OFFSET>,
            GroupDefaulted: GroupDefaulted::<Impl, IMPL_OFFSET>,
            SetGroupDefaulted: SetGroupDefaulted::<Impl, IMPL_OFFSET>,
            DiscretionaryAcl: DiscretionaryAcl::<Impl, IMPL_OFFSET>,
            SetDiscretionaryAcl: SetDiscretionaryAcl::<Impl, IMPL_OFFSET>,
            DaclDefaulted: DaclDefaulted::<Impl, IMPL_OFFSET>,
            SetDaclDefaulted: SetDaclDefaulted::<Impl, IMPL_OFFSET>,
            SystemAcl: SystemAcl::<Impl, IMPL_OFFSET>,
            SetSystemAcl: SetSystemAcl::<Impl, IMPL_OFFSET>,
            SaclDefaulted: SaclDefaulted::<Impl, IMPL_OFFSET>,
            SetSaclDefaulted: SetSaclDefaulted::<Impl, IMPL_OFFSET>,
            CopySecurityDescriptor: CopySecurityDescriptor::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsSecurityDescriptor as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsSecurityUtilityImpl: Sized + IDispatchImpl {
    fn GetSecurityDescriptor(&mut self, varpath: super::super::System::Com::VARIANT, lpathformat: i32, lformat: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetSecurityDescriptor(&mut self, varpath: super::super::System::Com::VARIANT, lpathformat: i32, vardata: super::super::System::Com::VARIANT, ldataformat: i32) -> ::windows::core::Result<()>;
    fn ConvertSecurityDescriptor(&mut self, varsd: super::super::System::Com::VARIANT, ldataformat: i32, loutformat: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SecurityMask(&mut self) -> ::windows::core::Result<i32>;
    fn SetSecurityMask(&mut self, lnsecuritymask: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsSecurityUtilityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsSecurityUtilityImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsSecurityUtilityVtbl {
        unsafe extern "system" fn GetSecurityDescriptor<Impl: IADsSecurityUtilityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varpath: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, lpathformat: i32, lformat: i32, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSecurityDescriptor(::core::mem::transmute_copy(&varpath), ::core::mem::transmute_copy(&lpathformat), ::core::mem::transmute_copy(&lformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityDescriptor<Impl: IADsSecurityUtilityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varpath: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, lpathformat: i32, vardata: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ldataformat: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSecurityDescriptor(::core::mem::transmute_copy(&varpath), ::core::mem::transmute_copy(&lpathformat), ::core::mem::transmute_copy(&vardata), ::core::mem::transmute_copy(&ldataformat)).into()
        }
        unsafe extern "system" fn ConvertSecurityDescriptor<Impl: IADsSecurityUtilityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsd: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ldataformat: i32, loutformat: i32, presult: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertSecurityDescriptor(::core::mem::transmute_copy(&varsd), ::core::mem::transmute_copy(&ldataformat), ::core::mem::transmute_copy(&loutformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *presult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SecurityMask<Impl: IADsSecurityUtilityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SecurityMask() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityMask<Impl: IADsSecurityUtilityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnsecuritymask: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSecurityMask(::core::mem::transmute_copy(&lnsecuritymask)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetSecurityDescriptor: GetSecurityDescriptor::<Impl, IMPL_OFFSET>,
            SetSecurityDescriptor: SetSecurityDescriptor::<Impl, IMPL_OFFSET>,
            ConvertSecurityDescriptor: ConvertSecurityDescriptor::<Impl, IMPL_OFFSET>,
            SecurityMask: SecurityMask::<Impl, IMPL_OFFSET>,
            SetSecurityMask: SetSecurityMask::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsSecurityUtility as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsServiceImpl: Sized + IDispatchImpl + IADsImpl {
    fn HostComputer(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetHostComputer(&mut self, bstrhostcomputer: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DisplayName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDisplayName(&mut self, bstrdisplayname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Version(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetVersion(&mut self, bstrversion: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ServiceType(&mut self) -> ::windows::core::Result<i32>;
    fn SetServiceType(&mut self, lnservicetype: i32) -> ::windows::core::Result<()>;
    fn StartType(&mut self) -> ::windows::core::Result<i32>;
    fn SetStartType(&mut self, lnstarttype: i32) -> ::windows::core::Result<()>;
    fn Path(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPath(&mut self, bstrpath: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn StartupParameters(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetStartupParameters(&mut self, bstrstartupparameters: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ErrorControl(&mut self) -> ::windows::core::Result<i32>;
    fn SetErrorControl(&mut self, lnerrorcontrol: i32) -> ::windows::core::Result<()>;
    fn LoadOrderGroup(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLoadOrderGroup(&mut self, bstrloadordergroup: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ServiceAccountName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetServiceAccountName(&mut self, bstrserviceaccountname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ServiceAccountPath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetServiceAccountPath(&mut self, bstrserviceaccountpath: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Dependencies(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetDependencies(&mut self, vdependencies: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsServiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsServiceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsServiceVtbl {
        unsafe extern "system" fn HostComputer<Impl: IADsServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HostComputer() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHostComputer<Impl: IADsServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrhostcomputer: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHostComputer(::core::mem::transmute_copy(&bstrhostcomputer)).into()
        }
        unsafe extern "system" fn DisplayName<Impl: IADsServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Impl: IADsServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdisplayname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayName(::core::mem::transmute_copy(&bstrdisplayname)).into()
        }
        unsafe extern "system" fn Version<Impl: IADsServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Version() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVersion<Impl: IADsServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrversion: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVersion(::core::mem::transmute_copy(&bstrversion)).into()
        }
        unsafe extern "system" fn ServiceType<Impl: IADsServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceType() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceType<Impl: IADsServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnservicetype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetServiceType(::core::mem::transmute_copy(&lnservicetype)).into()
        }
        unsafe extern "system" fn StartType<Impl: IADsServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartType() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartType<Impl: IADsServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnstarttype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStartType(::core::mem::transmute_copy(&lnstarttype)).into()
        }
        unsafe extern "system" fn Path<Impl: IADsServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPath<Impl: IADsServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPath(::core::mem::transmute_copy(&bstrpath)).into()
        }
        unsafe extern "system" fn StartupParameters<Impl: IADsServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartupParameters() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartupParameters<Impl: IADsServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrstartupparameters: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStartupParameters(::core::mem::transmute_copy(&bstrstartupparameters)).into()
        }
        unsafe extern "system" fn ErrorControl<Impl: IADsServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorControl() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetErrorControl<Impl: IADsServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnerrorcontrol: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetErrorControl(::core::mem::transmute_copy(&lnerrorcontrol)).into()
        }
        unsafe extern "system" fn LoadOrderGroup<Impl: IADsServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadOrderGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLoadOrderGroup<Impl: IADsServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrloadordergroup: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLoadOrderGroup(::core::mem::transmute_copy(&bstrloadordergroup)).into()
        }
        unsafe extern "system" fn ServiceAccountName<Impl: IADsServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceAccountName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceAccountName<Impl: IADsServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrserviceaccountname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetServiceAccountName(::core::mem::transmute_copy(&bstrserviceaccountname)).into()
        }
        unsafe extern "system" fn ServiceAccountPath<Impl: IADsServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceAccountPath() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceAccountPath<Impl: IADsServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrserviceaccountpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetServiceAccountPath(::core::mem::transmute_copy(&bstrserviceaccountpath)).into()
        }
        unsafe extern "system" fn Dependencies<Impl: IADsServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Dependencies() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDependencies<Impl: IADsServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vdependencies: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDependencies(::core::mem::transmute_copy(&vdependencies)).into()
        }
        Self {
            base: IADsVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            HostComputer: HostComputer::<Impl, IMPL_OFFSET>,
            SetHostComputer: SetHostComputer::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            SetDisplayName: SetDisplayName::<Impl, IMPL_OFFSET>,
            Version: Version::<Impl, IMPL_OFFSET>,
            SetVersion: SetVersion::<Impl, IMPL_OFFSET>,
            ServiceType: ServiceType::<Impl, IMPL_OFFSET>,
            SetServiceType: SetServiceType::<Impl, IMPL_OFFSET>,
            StartType: StartType::<Impl, IMPL_OFFSET>,
            SetStartType: SetStartType::<Impl, IMPL_OFFSET>,
            Path: Path::<Impl, IMPL_OFFSET>,
            SetPath: SetPath::<Impl, IMPL_OFFSET>,
            StartupParameters: StartupParameters::<Impl, IMPL_OFFSET>,
            SetStartupParameters: SetStartupParameters::<Impl, IMPL_OFFSET>,
            ErrorControl: ErrorControl::<Impl, IMPL_OFFSET>,
            SetErrorControl: SetErrorControl::<Impl, IMPL_OFFSET>,
            LoadOrderGroup: LoadOrderGroup::<Impl, IMPL_OFFSET>,
            SetLoadOrderGroup: SetLoadOrderGroup::<Impl, IMPL_OFFSET>,
            ServiceAccountName: ServiceAccountName::<Impl, IMPL_OFFSET>,
            SetServiceAccountName: SetServiceAccountName::<Impl, IMPL_OFFSET>,
            ServiceAccountPath: ServiceAccountPath::<Impl, IMPL_OFFSET>,
            SetServiceAccountPath: SetServiceAccountPath::<Impl, IMPL_OFFSET>,
            Dependencies: Dependencies::<Impl, IMPL_OFFSET>,
            SetDependencies: SetDependencies::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsService as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsServiceOperationsImpl: Sized + IDispatchImpl + IADsImpl {
    fn Status(&mut self) -> ::windows::core::Result<i32>;
    fn Start(&mut self) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
    fn Pause(&mut self) -> ::windows::core::Result<()>;
    fn Continue(&mut self) -> ::windows::core::Result<()>;
    fn SetPassword(&mut self, bstrnewpassword: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsServiceOperationsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsServiceOperationsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsServiceOperationsVtbl {
        unsafe extern "system" fn Status<Impl: IADsServiceOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Impl: IADsServiceOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IADsServiceOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn Pause<Impl: IADsServiceOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn Continue<Impl: IADsServiceOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Continue().into()
        }
        unsafe extern "system" fn SetPassword<Impl: IADsServiceOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrnewpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPassword(::core::mem::transmute_copy(&bstrnewpassword)).into()
        }
        Self {
            base: IADsVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            Start: Start::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
            Pause: Pause::<Impl, IMPL_OFFSET>,
            Continue: Continue::<Impl, IMPL_OFFSET>,
            SetPassword: SetPassword::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsServiceOperations as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsSessionImpl: Sized + IDispatchImpl + IADsImpl {
    fn User(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn UserPath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Computer(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ComputerPath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ConnectTime(&mut self) -> ::windows::core::Result<i32>;
    fn IdleTime(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsSessionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsSessionVtbl {
        unsafe extern "system" fn User<Impl: IADsSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).User() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserPath<Impl: IADsSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserPath() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Computer<Impl: IADsSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Computer() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComputerPath<Impl: IADsSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ComputerPath() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectTime<Impl: IADsSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectTime() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IdleTime<Impl: IADsSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IdleTime() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IADsVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            User: User::<Impl, IMPL_OFFSET>,
            UserPath: UserPath::<Impl, IMPL_OFFSET>,
            Computer: Computer::<Impl, IMPL_OFFSET>,
            ComputerPath: ComputerPath::<Impl, IMPL_OFFSET>,
            ConnectTime: ConnectTime::<Impl, IMPL_OFFSET>,
            IdleTime: IdleTime::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsSession as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsSyntaxImpl: Sized + IDispatchImpl + IADsImpl {
    fn OleAutoDataType(&mut self) -> ::windows::core::Result<i32>;
    fn SetOleAutoDataType(&mut self, lnoleautodatatype: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsSyntaxVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsSyntaxImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsSyntaxVtbl {
        unsafe extern "system" fn OleAutoDataType<Impl: IADsSyntaxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OleAutoDataType() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOleAutoDataType<Impl: IADsSyntaxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnoleautodatatype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOleAutoDataType(::core::mem::transmute_copy(&lnoleautodatatype)).into()
        }
        Self {
            base: IADsVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            OleAutoDataType: OleAutoDataType::<Impl, IMPL_OFFSET>,
            SetOleAutoDataType: SetOleAutoDataType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsSyntax as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsTimestampImpl: Sized + IDispatchImpl {
    fn WholeSeconds(&mut self) -> ::windows::core::Result<i32>;
    fn SetWholeSeconds(&mut self, lnwholeseconds: i32) -> ::windows::core::Result<()>;
    fn EventID(&mut self) -> ::windows::core::Result<i32>;
    fn SetEventID(&mut self, lneventid: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsTimestampVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsTimestampImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsTimestampVtbl {
        unsafe extern "system" fn WholeSeconds<Impl: IADsTimestampImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WholeSeconds() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWholeSeconds<Impl: IADsTimestampImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnwholeseconds: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWholeSeconds(::core::mem::transmute_copy(&lnwholeseconds)).into()
        }
        unsafe extern "system" fn EventID<Impl: IADsTimestampImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventID() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventID<Impl: IADsTimestampImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lneventid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEventID(::core::mem::transmute_copy(&lneventid)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            WholeSeconds: WholeSeconds::<Impl, IMPL_OFFSET>,
            SetWholeSeconds: SetWholeSeconds::<Impl, IMPL_OFFSET>,
            EventID: EventID::<Impl, IMPL_OFFSET>,
            SetEventID: SetEventID::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsTimestamp as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsTypedNameImpl: Sized + IDispatchImpl {
    fn ObjectName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetObjectName(&mut self, bstrobjectname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Level(&mut self) -> ::windows::core::Result<i32>;
    fn SetLevel(&mut self, lnlevel: i32) -> ::windows::core::Result<()>;
    fn Interval(&mut self) -> ::windows::core::Result<i32>;
    fn SetInterval(&mut self, lninterval: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsTypedNameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsTypedNameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsTypedNameVtbl {
        unsafe extern "system" fn ObjectName<Impl: IADsTypedNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ObjectName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetObjectName<Impl: IADsTypedNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrobjectname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetObjectName(::core::mem::transmute_copy(&bstrobjectname)).into()
        }
        unsafe extern "system" fn Level<Impl: IADsTypedNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Level() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLevel<Impl: IADsTypedNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLevel(::core::mem::transmute_copy(&lnlevel)).into()
        }
        unsafe extern "system" fn Interval<Impl: IADsTypedNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Interval() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInterval<Impl: IADsTypedNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lninterval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInterval(::core::mem::transmute_copy(&lninterval)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ObjectName: ObjectName::<Impl, IMPL_OFFSET>,
            SetObjectName: SetObjectName::<Impl, IMPL_OFFSET>,
            Level: Level::<Impl, IMPL_OFFSET>,
            SetLevel: SetLevel::<Impl, IMPL_OFFSET>,
            Interval: Interval::<Impl, IMPL_OFFSET>,
            SetInterval: SetInterval::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsTypedName as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsUserImpl: Sized + IDispatchImpl + IADsImpl {
    fn BadLoginAddress(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn BadLoginCount(&mut self) -> ::windows::core::Result<i32>;
    fn LastLogin(&mut self) -> ::windows::core::Result<f64>;
    fn LastLogoff(&mut self) -> ::windows::core::Result<f64>;
    fn LastFailedLogin(&mut self) -> ::windows::core::Result<f64>;
    fn PasswordLastChanged(&mut self) -> ::windows::core::Result<f64>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, bstrdescription: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Division(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDivision(&mut self, bstrdivision: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Department(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDepartment(&mut self, bstrdepartment: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn EmployeeID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetEmployeeID(&mut self, bstremployeeid: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FullName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetFullName(&mut self, bstrfullname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FirstName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetFirstName(&mut self, bstrfirstname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn LastName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLastName(&mut self, bstrlastname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn OtherName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetOtherName(&mut self, bstrothername: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn NamePrefix(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetNamePrefix(&mut self, bstrnameprefix: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn NameSuffix(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetNameSuffix(&mut self, bstrnamesuffix: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Title(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetTitle(&mut self, bstrtitle: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Manager(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetManager(&mut self, bstrmanager: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn TelephoneHome(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetTelephoneHome(&mut self, vtelephonehome: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn TelephoneMobile(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetTelephoneMobile(&mut self, vtelephonemobile: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn TelephoneNumber(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetTelephoneNumber(&mut self, vtelephonenumber: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn TelephonePager(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetTelephonePager(&mut self, vtelephonepager: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn FaxNumber(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetFaxNumber(&mut self, vfaxnumber: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn OfficeLocations(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetOfficeLocations(&mut self, vofficelocations: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn PostalAddresses(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetPostalAddresses(&mut self, vpostaladdresses: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn PostalCodes(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetPostalCodes(&mut self, vpostalcodes: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SeeAlso(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetSeeAlso(&mut self, vseealso: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
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
    fn SetLoginHours(&mut self, vloginhours: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn LoginWorkstations(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetLoginWorkstations(&mut self, vloginworkstations: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
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
    fn SetEmailAddress(&mut self, bstremailaddress: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn HomeDirectory(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetHomeDirectory(&mut self, bstrhomedirectory: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Languages(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetLanguages(&mut self, vlanguages: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Profile(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetProfile(&mut self, bstrprofile: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn LoginScript(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLoginScript(&mut self, bstrloginscript: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Picture(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetPicture(&mut self, vpicture: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn HomePage(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetHomePage(&mut self, bstrhomepage: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Groups(&mut self) -> ::windows::core::Result<IADsMembers>;
    fn SetPassword(&mut self, newpassword: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ChangePassword(&mut self, bstroldpassword: super::super::Foundation::BSTR, bstrnewpassword: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsUserVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsUserImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsUserVtbl {
        unsafe extern "system" fn BadLoginAddress<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BadLoginAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BadLoginCount<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BadLoginCount() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastLogin<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastLogin() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastLogoff<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastLogoff() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastFailedLogin<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastFailedLogin() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PasswordLastChanged<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PasswordLastChanged() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&bstrdescription)).into()
        }
        unsafe extern "system" fn Division<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Division() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDivision<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdivision: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDivision(::core::mem::transmute_copy(&bstrdivision)).into()
        }
        unsafe extern "system" fn Department<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Department() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDepartment<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdepartment: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDepartment(::core::mem::transmute_copy(&bstrdepartment)).into()
        }
        unsafe extern "system" fn EmployeeID<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EmployeeID() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEmployeeID<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstremployeeid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEmployeeID(::core::mem::transmute_copy(&bstremployeeid)).into()
        }
        unsafe extern "system" fn FullName<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FullName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFullName<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfullname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFullName(::core::mem::transmute_copy(&bstrfullname)).into()
        }
        unsafe extern "system" fn FirstName<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FirstName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFirstName<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfirstname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFirstName(::core::mem::transmute_copy(&bstrfirstname)).into()
        }
        unsafe extern "system" fn LastName<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastName<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlastname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLastName(::core::mem::transmute_copy(&bstrlastname)).into()
        }
        unsafe extern "system" fn OtherName<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OtherName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOtherName<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrothername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOtherName(::core::mem::transmute_copy(&bstrothername)).into()
        }
        unsafe extern "system" fn NamePrefix<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NamePrefix() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNamePrefix<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrnameprefix: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNamePrefix(::core::mem::transmute_copy(&bstrnameprefix)).into()
        }
        unsafe extern "system" fn NameSuffix<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NameSuffix() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNameSuffix<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrnamesuffix: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNameSuffix(::core::mem::transmute_copy(&bstrnamesuffix)).into()
        }
        unsafe extern "system" fn Title<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Title() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTitle<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTitle(::core::mem::transmute_copy(&bstrtitle)).into()
        }
        unsafe extern "system" fn Manager<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Manager() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetManager<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmanager: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetManager(::core::mem::transmute_copy(&bstrmanager)).into()
        }
        unsafe extern "system" fn TelephoneHome<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TelephoneHome() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTelephoneHome<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vtelephonehome: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTelephoneHome(::core::mem::transmute_copy(&vtelephonehome)).into()
        }
        unsafe extern "system" fn TelephoneMobile<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TelephoneMobile() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTelephoneMobile<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vtelephonemobile: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTelephoneMobile(::core::mem::transmute_copy(&vtelephonemobile)).into()
        }
        unsafe extern "system" fn TelephoneNumber<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TelephoneNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTelephoneNumber<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vtelephonenumber: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTelephoneNumber(::core::mem::transmute_copy(&vtelephonenumber)).into()
        }
        unsafe extern "system" fn TelephonePager<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TelephonePager() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTelephonePager<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vtelephonepager: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTelephonePager(::core::mem::transmute_copy(&vtelephonepager)).into()
        }
        unsafe extern "system" fn FaxNumber<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FaxNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFaxNumber<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vfaxnumber: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFaxNumber(::core::mem::transmute_copy(&vfaxnumber)).into()
        }
        unsafe extern "system" fn OfficeLocations<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OfficeLocations() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOfficeLocations<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vofficelocations: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOfficeLocations(::core::mem::transmute_copy(&vofficelocations)).into()
        }
        unsafe extern "system" fn PostalAddresses<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PostalAddresses() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPostalAddresses<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vpostaladdresses: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPostalAddresses(::core::mem::transmute_copy(&vpostaladdresses)).into()
        }
        unsafe extern "system" fn PostalCodes<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PostalCodes() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPostalCodes<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vpostalcodes: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPostalCodes(::core::mem::transmute_copy(&vpostalcodes)).into()
        }
        unsafe extern "system" fn SeeAlso<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SeeAlso() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSeeAlso<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vseealso: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSeeAlso(::core::mem::transmute_copy(&vseealso)).into()
        }
        unsafe extern "system" fn AccountDisabled<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccountDisabled() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccountDisabled<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, faccountdisabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAccountDisabled(::core::mem::transmute_copy(&faccountdisabled)).into()
        }
        unsafe extern "system" fn AccountExpirationDate<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccountExpirationDate() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccountExpirationDate<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, daaccountexpirationdate: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAccountExpirationDate(::core::mem::transmute_copy(&daaccountexpirationdate)).into()
        }
        unsafe extern "system" fn GraceLoginsAllowed<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GraceLoginsAllowed() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGraceLoginsAllowed<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lngraceloginsallowed: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGraceLoginsAllowed(::core::mem::transmute_copy(&lngraceloginsallowed)).into()
        }
        unsafe extern "system" fn GraceLoginsRemaining<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GraceLoginsRemaining() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGraceLoginsRemaining<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lngraceloginsremaining: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGraceLoginsRemaining(::core::mem::transmute_copy(&lngraceloginsremaining)).into()
        }
        unsafe extern "system" fn IsAccountLocked<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAccountLocked() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsAccountLocked<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fisaccountlocked: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsAccountLocked(::core::mem::transmute_copy(&fisaccountlocked)).into()
        }
        unsafe extern "system" fn LoginHours<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoginHours() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLoginHours<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vloginhours: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLoginHours(::core::mem::transmute_copy(&vloginhours)).into()
        }
        unsafe extern "system" fn LoginWorkstations<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoginWorkstations() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLoginWorkstations<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vloginworkstations: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLoginWorkstations(::core::mem::transmute_copy(&vloginworkstations)).into()
        }
        unsafe extern "system" fn MaxLogins<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxLogins() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxLogins<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnmaxlogins: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxLogins(::core::mem::transmute_copy(&lnmaxlogins)).into()
        }
        unsafe extern "system" fn MaxStorage<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxStorage() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxStorage<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnmaxstorage: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxStorage(::core::mem::transmute_copy(&lnmaxstorage)).into()
        }
        unsafe extern "system" fn PasswordExpirationDate<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PasswordExpirationDate() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPasswordExpirationDate<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dapasswordexpirationdate: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPasswordExpirationDate(::core::mem::transmute_copy(&dapasswordexpirationdate)).into()
        }
        unsafe extern "system" fn PasswordMinimumLength<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PasswordMinimumLength() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPasswordMinimumLength<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnpasswordminimumlength: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPasswordMinimumLength(::core::mem::transmute_copy(&lnpasswordminimumlength)).into()
        }
        unsafe extern "system" fn PasswordRequired<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PasswordRequired() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPasswordRequired<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fpasswordrequired: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPasswordRequired(::core::mem::transmute_copy(&fpasswordrequired)).into()
        }
        unsafe extern "system" fn RequireUniquePassword<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequireUniquePassword() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequireUniquePassword<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frequireuniquepassword: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRequireUniquePassword(::core::mem::transmute_copy(&frequireuniquepassword)).into()
        }
        unsafe extern "system" fn EmailAddress<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EmailAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEmailAddress<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstremailaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEmailAddress(::core::mem::transmute_copy(&bstremailaddress)).into()
        }
        unsafe extern "system" fn HomeDirectory<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HomeDirectory() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHomeDirectory<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrhomedirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHomeDirectory(::core::mem::transmute_copy(&bstrhomedirectory)).into()
        }
        unsafe extern "system" fn Languages<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Languages() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguages<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vlanguages: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLanguages(::core::mem::transmute_copy(&vlanguages)).into()
        }
        unsafe extern "system" fn Profile<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Profile() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProfile<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprofile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProfile(::core::mem::transmute_copy(&bstrprofile)).into()
        }
        unsafe extern "system" fn LoginScript<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoginScript() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLoginScript<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrloginscript: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLoginScript(::core::mem::transmute_copy(&bstrloginscript)).into()
        }
        unsafe extern "system" fn Picture<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Picture() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPicture<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vpicture: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPicture(::core::mem::transmute_copy(&vpicture)).into()
        }
        unsafe extern "system" fn HomePage<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HomePage() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHomePage<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrhomepage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHomePage(::core::mem::transmute_copy(&bstrhomepage)).into()
        }
        unsafe extern "system" fn Groups<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgroups: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Groups() {
                ::core::result::Result::Ok(ok__) => {
                    *ppgroups = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPassword<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPassword(::core::mem::transmute_copy(&newpassword)).into()
        }
        unsafe extern "system" fn ChangePassword<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstroldpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnewpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ChangePassword(::core::mem::transmute_copy(&bstroldpassword), ::core::mem::transmute_copy(&bstrnewpassword)).into()
        }
        Self {
            base: IADsVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            BadLoginAddress: BadLoginAddress::<Impl, IMPL_OFFSET>,
            BadLoginCount: BadLoginCount::<Impl, IMPL_OFFSET>,
            LastLogin: LastLogin::<Impl, IMPL_OFFSET>,
            LastLogoff: LastLogoff::<Impl, IMPL_OFFSET>,
            LastFailedLogin: LastFailedLogin::<Impl, IMPL_OFFSET>,
            PasswordLastChanged: PasswordLastChanged::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            SetDescription: SetDescription::<Impl, IMPL_OFFSET>,
            Division: Division::<Impl, IMPL_OFFSET>,
            SetDivision: SetDivision::<Impl, IMPL_OFFSET>,
            Department: Department::<Impl, IMPL_OFFSET>,
            SetDepartment: SetDepartment::<Impl, IMPL_OFFSET>,
            EmployeeID: EmployeeID::<Impl, IMPL_OFFSET>,
            SetEmployeeID: SetEmployeeID::<Impl, IMPL_OFFSET>,
            FullName: FullName::<Impl, IMPL_OFFSET>,
            SetFullName: SetFullName::<Impl, IMPL_OFFSET>,
            FirstName: FirstName::<Impl, IMPL_OFFSET>,
            SetFirstName: SetFirstName::<Impl, IMPL_OFFSET>,
            LastName: LastName::<Impl, IMPL_OFFSET>,
            SetLastName: SetLastName::<Impl, IMPL_OFFSET>,
            OtherName: OtherName::<Impl, IMPL_OFFSET>,
            SetOtherName: SetOtherName::<Impl, IMPL_OFFSET>,
            NamePrefix: NamePrefix::<Impl, IMPL_OFFSET>,
            SetNamePrefix: SetNamePrefix::<Impl, IMPL_OFFSET>,
            NameSuffix: NameSuffix::<Impl, IMPL_OFFSET>,
            SetNameSuffix: SetNameSuffix::<Impl, IMPL_OFFSET>,
            Title: Title::<Impl, IMPL_OFFSET>,
            SetTitle: SetTitle::<Impl, IMPL_OFFSET>,
            Manager: Manager::<Impl, IMPL_OFFSET>,
            SetManager: SetManager::<Impl, IMPL_OFFSET>,
            TelephoneHome: TelephoneHome::<Impl, IMPL_OFFSET>,
            SetTelephoneHome: SetTelephoneHome::<Impl, IMPL_OFFSET>,
            TelephoneMobile: TelephoneMobile::<Impl, IMPL_OFFSET>,
            SetTelephoneMobile: SetTelephoneMobile::<Impl, IMPL_OFFSET>,
            TelephoneNumber: TelephoneNumber::<Impl, IMPL_OFFSET>,
            SetTelephoneNumber: SetTelephoneNumber::<Impl, IMPL_OFFSET>,
            TelephonePager: TelephonePager::<Impl, IMPL_OFFSET>,
            SetTelephonePager: SetTelephonePager::<Impl, IMPL_OFFSET>,
            FaxNumber: FaxNumber::<Impl, IMPL_OFFSET>,
            SetFaxNumber: SetFaxNumber::<Impl, IMPL_OFFSET>,
            OfficeLocations: OfficeLocations::<Impl, IMPL_OFFSET>,
            SetOfficeLocations: SetOfficeLocations::<Impl, IMPL_OFFSET>,
            PostalAddresses: PostalAddresses::<Impl, IMPL_OFFSET>,
            SetPostalAddresses: SetPostalAddresses::<Impl, IMPL_OFFSET>,
            PostalCodes: PostalCodes::<Impl, IMPL_OFFSET>,
            SetPostalCodes: SetPostalCodes::<Impl, IMPL_OFFSET>,
            SeeAlso: SeeAlso::<Impl, IMPL_OFFSET>,
            SetSeeAlso: SetSeeAlso::<Impl, IMPL_OFFSET>,
            AccountDisabled: AccountDisabled::<Impl, IMPL_OFFSET>,
            SetAccountDisabled: SetAccountDisabled::<Impl, IMPL_OFFSET>,
            AccountExpirationDate: AccountExpirationDate::<Impl, IMPL_OFFSET>,
            SetAccountExpirationDate: SetAccountExpirationDate::<Impl, IMPL_OFFSET>,
            GraceLoginsAllowed: GraceLoginsAllowed::<Impl, IMPL_OFFSET>,
            SetGraceLoginsAllowed: SetGraceLoginsAllowed::<Impl, IMPL_OFFSET>,
            GraceLoginsRemaining: GraceLoginsRemaining::<Impl, IMPL_OFFSET>,
            SetGraceLoginsRemaining: SetGraceLoginsRemaining::<Impl, IMPL_OFFSET>,
            IsAccountLocked: IsAccountLocked::<Impl, IMPL_OFFSET>,
            SetIsAccountLocked: SetIsAccountLocked::<Impl, IMPL_OFFSET>,
            LoginHours: LoginHours::<Impl, IMPL_OFFSET>,
            SetLoginHours: SetLoginHours::<Impl, IMPL_OFFSET>,
            LoginWorkstations: LoginWorkstations::<Impl, IMPL_OFFSET>,
            SetLoginWorkstations: SetLoginWorkstations::<Impl, IMPL_OFFSET>,
            MaxLogins: MaxLogins::<Impl, IMPL_OFFSET>,
            SetMaxLogins: SetMaxLogins::<Impl, IMPL_OFFSET>,
            MaxStorage: MaxStorage::<Impl, IMPL_OFFSET>,
            SetMaxStorage: SetMaxStorage::<Impl, IMPL_OFFSET>,
            PasswordExpirationDate: PasswordExpirationDate::<Impl, IMPL_OFFSET>,
            SetPasswordExpirationDate: SetPasswordExpirationDate::<Impl, IMPL_OFFSET>,
            PasswordMinimumLength: PasswordMinimumLength::<Impl, IMPL_OFFSET>,
            SetPasswordMinimumLength: SetPasswordMinimumLength::<Impl, IMPL_OFFSET>,
            PasswordRequired: PasswordRequired::<Impl, IMPL_OFFSET>,
            SetPasswordRequired: SetPasswordRequired::<Impl, IMPL_OFFSET>,
            RequireUniquePassword: RequireUniquePassword::<Impl, IMPL_OFFSET>,
            SetRequireUniquePassword: SetRequireUniquePassword::<Impl, IMPL_OFFSET>,
            EmailAddress: EmailAddress::<Impl, IMPL_OFFSET>,
            SetEmailAddress: SetEmailAddress::<Impl, IMPL_OFFSET>,
            HomeDirectory: HomeDirectory::<Impl, IMPL_OFFSET>,
            SetHomeDirectory: SetHomeDirectory::<Impl, IMPL_OFFSET>,
            Languages: Languages::<Impl, IMPL_OFFSET>,
            SetLanguages: SetLanguages::<Impl, IMPL_OFFSET>,
            Profile: Profile::<Impl, IMPL_OFFSET>,
            SetProfile: SetProfile::<Impl, IMPL_OFFSET>,
            LoginScript: LoginScript::<Impl, IMPL_OFFSET>,
            SetLoginScript: SetLoginScript::<Impl, IMPL_OFFSET>,
            Picture: Picture::<Impl, IMPL_OFFSET>,
            SetPicture: SetPicture::<Impl, IMPL_OFFSET>,
            HomePage: HomePage::<Impl, IMPL_OFFSET>,
            SetHomePage: SetHomePage::<Impl, IMPL_OFFSET>,
            Groups: Groups::<Impl, IMPL_OFFSET>,
            SetPassword: SetPassword::<Impl, IMPL_OFFSET>,
            ChangePassword: ChangePassword::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsUser as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsWinNTSystemInfoImpl: Sized + IDispatchImpl {
    fn UserName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ComputerName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DomainName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn PDC(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsWinNTSystemInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsWinNTSystemInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsWinNTSystemInfoVtbl {
        unsafe extern "system" fn UserName<Impl: IADsWinNTSystemInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComputerName<Impl: IADsWinNTSystemInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ComputerName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DomainName<Impl: IADsWinNTSystemInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DomainName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PDC<Impl: IADsWinNTSystemInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PDC() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            UserName: UserName::<Impl, IMPL_OFFSET>,
            ComputerName: ComputerName::<Impl, IMPL_OFFSET>,
            DomainName: DomainName::<Impl, IMPL_OFFSET>,
            PDC: PDC::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsWinNTSystemInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait ICommonQueryImpl: Sized {
    fn OpenQueryWindow(&mut self, hwndparent: super::super::Foundation::HWND, pquerywnd: *mut OPENQUERYWINDOW, ppdataobject: *mut ::core::option::Option<super::super::System::Com::IDataObject>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl ICommonQueryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommonQueryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICommonQueryVtbl {
        unsafe extern "system" fn OpenQueryWindow<Impl: ICommonQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, pquerywnd: *mut OPENQUERYWINDOW, ppdataobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OpenQueryWindow(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&pquerywnd), ::core::mem::transmute_copy(&ppdataobject)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OpenQueryWindow: OpenQueryWindow::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommonQuery as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IDirectoryObjectImpl: Sized {
    fn GetObjectInformation(&mut self) -> ::windows::core::Result<*mut ADS_OBJECT_INFO>;
    fn GetObjectAttributes(&mut self, pattributenames: *const super::super::Foundation::PWSTR, dwnumberattributes: u32, ppattributeentries: *mut *mut ADS_ATTR_INFO, pdwnumattributesreturned: *mut u32) -> ::windows::core::Result<()>;
    fn SetObjectAttributes(&mut self, pattributeentries: *const ADS_ATTR_INFO, dwnumattributes: u32) -> ::windows::core::Result<u32>;
    fn CreateDSObject(&mut self, pszrdnname: super::super::Foundation::PWSTR, pattributeentries: *const ADS_ATTR_INFO, dwnumattributes: u32) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn DeleteDSObject(&mut self, pszrdnname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IDirectoryObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectoryObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectoryObjectVtbl {
        unsafe extern "system" fn GetObjectInformation<Impl: IDirectoryObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppobjinfo: *mut *mut ADS_OBJECT_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetObjectInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *ppobjinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectAttributes<Impl: IDirectoryObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattributenames: *const super::super::Foundation::PWSTR, dwnumberattributes: u32, ppattributeentries: *mut *mut ADS_ATTR_INFO, pdwnumattributesreturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetObjectAttributes(::core::mem::transmute_copy(&pattributenames), ::core::mem::transmute_copy(&dwnumberattributes), ::core::mem::transmute_copy(&ppattributeentries), ::core::mem::transmute_copy(&pdwnumattributesreturned)).into()
        }
        unsafe extern "system" fn SetObjectAttributes<Impl: IDirectoryObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattributeentries: *const ADS_ATTR_INFO, dwnumattributes: u32, pdwnumattributesmodified: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetObjectAttributes(::core::mem::transmute_copy(&pattributeentries), ::core::mem::transmute_copy(&dwnumattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwnumattributesmodified = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDSObject<Impl: IDirectoryObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszrdnname: super::super::Foundation::PWSTR, pattributeentries: *const ADS_ATTR_INFO, dwnumattributes: u32, ppobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDSObject(::core::mem::transmute_copy(&pszrdnname), ::core::mem::transmute_copy(&pattributeentries), ::core::mem::transmute_copy(&dwnumattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteDSObject<Impl: IDirectoryObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszrdnname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteDSObject(::core::mem::transmute_copy(&pszrdnname)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetObjectInformation: GetObjectInformation::<Impl, IMPL_OFFSET>,
            GetObjectAttributes: GetObjectAttributes::<Impl, IMPL_OFFSET>,
            SetObjectAttributes: SetObjectAttributes::<Impl, IMPL_OFFSET>,
            CreateDSObject: CreateDSObject::<Impl, IMPL_OFFSET>,
            DeleteDSObject: DeleteDSObject::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectoryObject as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectorySchemaMgmtImpl: Sized {
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
impl IDirectorySchemaMgmtVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectorySchemaMgmtImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectorySchemaMgmtVtbl {
        unsafe extern "system" fn EnumAttributes<Impl: IDirectorySchemaMgmtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszattrnames: *const super::super::Foundation::PWSTR, dwnumattributes: u32, ppattrdefinition: *const *const ADS_ATTR_DEF, pdwnumattributes: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumAttributes(::core::mem::transmute_copy(&ppszattrnames), ::core::mem::transmute_copy(&dwnumattributes), ::core::mem::transmute_copy(&ppattrdefinition), ::core::mem::transmute_copy(&pdwnumattributes)).into()
        }
        unsafe extern "system" fn CreateAttributeDefinition<Impl: IDirectorySchemaMgmtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszattributename: super::super::Foundation::PWSTR, pattributedefinition: *const ADS_ATTR_DEF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateAttributeDefinition(::core::mem::transmute_copy(&pszattributename), ::core::mem::transmute_copy(&pattributedefinition)).into()
        }
        unsafe extern "system" fn WriteAttributeDefinition<Impl: IDirectorySchemaMgmtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszattributename: super::super::Foundation::PWSTR, pattributedefinition: *const ADS_ATTR_DEF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteAttributeDefinition(::core::mem::transmute_copy(&pszattributename), ::core::mem::transmute_copy(&pattributedefinition)).into()
        }
        unsafe extern "system" fn DeleteAttributeDefinition<Impl: IDirectorySchemaMgmtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszattributename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteAttributeDefinition(::core::mem::transmute_copy(&pszattributename)).into()
        }
        unsafe extern "system" fn EnumClasses<Impl: IDirectorySchemaMgmtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszclassnames: *const super::super::Foundation::PWSTR, dwnumclasses: u32, ppclassdefinition: *const *const ADS_CLASS_DEF, pdwnumclasses: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumClasses(::core::mem::transmute_copy(&ppszclassnames), ::core::mem::transmute_copy(&dwnumclasses), ::core::mem::transmute_copy(&ppclassdefinition), ::core::mem::transmute_copy(&pdwnumclasses)).into()
        }
        unsafe extern "system" fn WriteClassDefinition<Impl: IDirectorySchemaMgmtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszclassname: super::super::Foundation::PWSTR, pclassdefinition: *const ADS_CLASS_DEF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteClassDefinition(::core::mem::transmute_copy(&pszclassname), ::core::mem::transmute_copy(&pclassdefinition)).into()
        }
        unsafe extern "system" fn CreateClassDefinition<Impl: IDirectorySchemaMgmtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszclassname: super::super::Foundation::PWSTR, pclassdefinition: *const ADS_CLASS_DEF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateClassDefinition(::core::mem::transmute_copy(&pszclassname), ::core::mem::transmute_copy(&pclassdefinition)).into()
        }
        unsafe extern "system" fn DeleteClassDefinition<Impl: IDirectorySchemaMgmtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszclassname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteClassDefinition(::core::mem::transmute_copy(&pszclassname)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            EnumAttributes: EnumAttributes::<Impl, IMPL_OFFSET>,
            CreateAttributeDefinition: CreateAttributeDefinition::<Impl, IMPL_OFFSET>,
            WriteAttributeDefinition: WriteAttributeDefinition::<Impl, IMPL_OFFSET>,
            DeleteAttributeDefinition: DeleteAttributeDefinition::<Impl, IMPL_OFFSET>,
            EnumClasses: EnumClasses::<Impl, IMPL_OFFSET>,
            WriteClassDefinition: WriteClassDefinition::<Impl, IMPL_OFFSET>,
            CreateClassDefinition: CreateClassDefinition::<Impl, IMPL_OFFSET>,
            DeleteClassDefinition: DeleteClassDefinition::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectorySchemaMgmt as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectorySearchImpl: Sized {
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
impl IDirectorySearchVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectorySearchImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectorySearchVtbl {
        unsafe extern "system" fn SetSearchPreference<Impl: IDirectorySearchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psearchprefs: *const ads_searchpref_info, dwnumprefs: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSearchPreference(::core::mem::transmute_copy(&psearchprefs), ::core::mem::transmute_copy(&dwnumprefs)).into()
        }
        unsafe extern "system" fn ExecuteSearch<Impl: IDirectorySearchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsearchfilter: super::super::Foundation::PWSTR, pattributenames: *const super::super::Foundation::PWSTR, dwnumberattributes: u32, phsearchresult: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExecuteSearch(::core::mem::transmute_copy(&pszsearchfilter), ::core::mem::transmute_copy(&pattributenames), ::core::mem::transmute_copy(&dwnumberattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *phsearchresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AbandonSearch<Impl: IDirectorySearchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phsearchresult: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AbandonSearch(::core::mem::transmute_copy(&phsearchresult)).into()
        }
        unsafe extern "system" fn GetFirstRow<Impl: IDirectorySearchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hsearchresult: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFirstRow(::core::mem::transmute_copy(&hsearchresult))
        }
        unsafe extern "system" fn GetNextRow<Impl: IDirectorySearchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hsearchresult: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNextRow(::core::mem::transmute_copy(&hsearchresult))
        }
        unsafe extern "system" fn GetPreviousRow<Impl: IDirectorySearchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hsearchresult: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPreviousRow(::core::mem::transmute_copy(&hsearchresult))
        }
        unsafe extern "system" fn GetNextColumnName<Impl: IDirectorySearchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hsearchhandle: isize, ppszcolumnname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNextColumnName(::core::mem::transmute_copy(&hsearchhandle), ::core::mem::transmute_copy(&ppszcolumnname))
        }
        unsafe extern "system" fn GetColumn<Impl: IDirectorySearchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hsearchresult: isize, szcolumnname: super::super::Foundation::PWSTR, psearchcolumn: *mut ads_search_column) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetColumn(::core::mem::transmute_copy(&hsearchresult), ::core::mem::transmute_copy(&szcolumnname)) {
                ::core::result::Result::Ok(ok__) => {
                    *psearchcolumn = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FreeColumn<Impl: IDirectorySearchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psearchcolumn: *const ads_search_column) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FreeColumn(::core::mem::transmute_copy(&psearchcolumn)).into()
        }
        unsafe extern "system" fn CloseSearchHandle<Impl: IDirectorySearchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hsearchresult: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CloseSearchHandle(::core::mem::transmute_copy(&hsearchresult)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetSearchPreference: SetSearchPreference::<Impl, IMPL_OFFSET>,
            ExecuteSearch: ExecuteSearch::<Impl, IMPL_OFFSET>,
            AbandonSearch: AbandonSearch::<Impl, IMPL_OFFSET>,
            GetFirstRow: GetFirstRow::<Impl, IMPL_OFFSET>,
            GetNextRow: GetNextRow::<Impl, IMPL_OFFSET>,
            GetPreviousRow: GetPreviousRow::<Impl, IMPL_OFFSET>,
            GetNextColumnName: GetNextColumnName::<Impl, IMPL_OFFSET>,
            GetColumn: GetColumn::<Impl, IMPL_OFFSET>,
            FreeColumn: FreeColumn::<Impl, IMPL_OFFSET>,
            CloseSearchHandle: CloseSearchHandle::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectorySearch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IDsAdminCreateObjImpl: Sized {
    fn Initialize(&mut self, padscontainerobj: ::core::option::Option<IADsContainer>, padscopysource: ::core::option::Option<IADs>, lpszclassname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn CreateModal(&mut self, hwndparent: super::super::Foundation::HWND) -> ::windows::core::Result<IADs>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IDsAdminCreateObjVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDsAdminCreateObjImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDsAdminCreateObjVtbl {
        unsafe extern "system" fn Initialize<Impl: IDsAdminCreateObjImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padscontainerobj: ::windows::core::RawPtr, padscopysource: ::windows::core::RawPtr, lpszclassname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&padscontainerobj), ::core::mem::transmute(&padscopysource), ::core::mem::transmute_copy(&lpszclassname)).into()
        }
        unsafe extern "system" fn CreateModal<Impl: IDsAdminCreateObjImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ppadsobj: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateModal(::core::mem::transmute_copy(&hwndparent)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppadsobj = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            CreateModal: CreateModal::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDsAdminCreateObj as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDsAdminNewObjImpl: Sized {
    fn SetButtons(&mut self, ncurrindex: u32, bvalid: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetPageCounts(&mut self, pntotal: *mut i32, pnstartindex: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDsAdminNewObjVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDsAdminNewObjImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDsAdminNewObjVtbl {
        unsafe extern "system" fn SetButtons<Impl: IDsAdminNewObjImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncurrindex: u32, bvalid: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetButtons(::core::mem::transmute_copy(&ncurrindex), ::core::mem::transmute_copy(&bvalid)).into()
        }
        unsafe extern "system" fn GetPageCounts<Impl: IDsAdminNewObjImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pntotal: *mut i32, pnstartindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPageCounts(::core::mem::transmute_copy(&pntotal), ::core::mem::transmute_copy(&pnstartindex)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetButtons: SetButtons::<Impl, IMPL_OFFSET>,
            GetPageCounts: GetPageCounts::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDsAdminNewObj as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IDsAdminNewObjExtImpl: Sized {
    fn Initialize(&mut self, padscontainerobj: ::core::option::Option<IADsContainer>, padscopysource: ::core::option::Option<IADs>, lpszclassname: super::super::Foundation::PWSTR, pdsadminnewobj: ::core::option::Option<IDsAdminNewObj>, pdispinfo: *mut DSA_NEWOBJ_DISPINFO) -> ::windows::core::Result<()>;
    fn AddPages(&mut self, lpfnaddpage: super::super::UI::Controls::LPFNSVADDPROPSHEETPAGE, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<()>;
    fn SetObject(&mut self, padsobj: ::core::option::Option<IADs>) -> ::windows::core::Result<()>;
    fn WriteData(&mut self, hwnd: super::super::Foundation::HWND, ucontext: u32) -> ::windows::core::Result<()>;
    fn OnError(&mut self, hwnd: super::super::Foundation::HWND, hr: ::windows::core::HRESULT, ucontext: u32) -> ::windows::core::Result<()>;
    fn GetSummaryInfo(&mut self, pbstrtext: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl IDsAdminNewObjExtVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDsAdminNewObjExtImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDsAdminNewObjExtVtbl {
        unsafe extern "system" fn Initialize<Impl: IDsAdminNewObjExtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padscontainerobj: ::windows::core::RawPtr, padscopysource: ::windows::core::RawPtr, lpszclassname: super::super::Foundation::PWSTR, pdsadminnewobj: ::windows::core::RawPtr, pdispinfo: *mut DSA_NEWOBJ_DISPINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&padscontainerobj), ::core::mem::transmute(&padscopysource), ::core::mem::transmute_copy(&lpszclassname), ::core::mem::transmute(&pdsadminnewobj), ::core::mem::transmute_copy(&pdispinfo)).into()
        }
        unsafe extern "system" fn AddPages<Impl: IDsAdminNewObjExtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpfnaddpage: ::windows::core::RawPtr, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPages(::core::mem::transmute_copy(&lpfnaddpage), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn SetObject<Impl: IDsAdminNewObjExtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padsobj: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetObject(::core::mem::transmute(&padsobj)).into()
        }
        unsafe extern "system" fn WriteData<Impl: IDsAdminNewObjExtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, ucontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteData(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&ucontext)).into()
        }
        unsafe extern "system" fn OnError<Impl: IDsAdminNewObjExtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, hr: ::windows::core::HRESULT, ucontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnError(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&hr), ::core::mem::transmute_copy(&ucontext)).into()
        }
        unsafe extern "system" fn GetSummaryInfo<Impl: IDsAdminNewObjExtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSummaryInfo(::core::mem::transmute_copy(&pbstrtext)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            AddPages: AddPages::<Impl, IMPL_OFFSET>,
            SetObject: SetObject::<Impl, IMPL_OFFSET>,
            WriteData: WriteData::<Impl, IMPL_OFFSET>,
            OnError: OnError::<Impl, IMPL_OFFSET>,
            GetSummaryInfo: GetSummaryInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDsAdminNewObjExt as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDsAdminNewObjPrimarySiteImpl: Sized {
    fn CreateNew(&mut self, pszname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Commit(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDsAdminNewObjPrimarySiteVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDsAdminNewObjPrimarySiteImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDsAdminNewObjPrimarySiteVtbl {
        unsafe extern "system" fn CreateNew<Impl: IDsAdminNewObjPrimarySiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateNew(::core::mem::transmute_copy(&pszname)).into()
        }
        unsafe extern "system" fn Commit<Impl: IDsAdminNewObjPrimarySiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Commit().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateNew: CreateNew::<Impl, IMPL_OFFSET>,
            Commit: Commit::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDsAdminNewObjPrimarySite as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IDsAdminNotifyHandlerImpl: Sized {
    fn Initialize(&mut self, pextrainfo: ::core::option::Option<super::super::System::Com::IDataObject>, pueventflags: *mut u32) -> ::windows::core::Result<()>;
    fn Begin(&mut self, uevent: u32, parg1: ::core::option::Option<super::super::System::Com::IDataObject>, parg2: ::core::option::Option<super::super::System::Com::IDataObject>, puflags: *mut u32, pbstr: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Notify(&mut self, nitem: u32, uflags: u32) -> ::windows::core::Result<()>;
    fn End(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IDsAdminNotifyHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDsAdminNotifyHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDsAdminNotifyHandlerVtbl {
        unsafe extern "system" fn Initialize<Impl: IDsAdminNotifyHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pextrainfo: ::windows::core::RawPtr, pueventflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&pextrainfo), ::core::mem::transmute_copy(&pueventflags)).into()
        }
        unsafe extern "system" fn Begin<Impl: IDsAdminNotifyHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uevent: u32, parg1: ::windows::core::RawPtr, parg2: ::windows::core::RawPtr, puflags: *mut u32, pbstr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin(::core::mem::transmute_copy(&uevent), ::core::mem::transmute(&parg1), ::core::mem::transmute(&parg2), ::core::mem::transmute_copy(&puflags), ::core::mem::transmute_copy(&pbstr)).into()
        }
        unsafe extern "system" fn Notify<Impl: IDsAdminNotifyHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nitem: u32, uflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Notify(::core::mem::transmute_copy(&nitem), ::core::mem::transmute_copy(&uflags)).into()
        }
        unsafe extern "system" fn End<Impl: IDsAdminNotifyHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).End().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            Begin: Begin::<Impl, IMPL_OFFSET>,
            Notify: Notify::<Impl, IMPL_OFFSET>,
            End: End::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDsAdminNotifyHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDsBrowseDomainTreeImpl: Sized {
    fn BrowseTo(&mut self, hwndparent: super::super::Foundation::HWND, ppsztargetpath: *mut super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::Result<()>;
    fn GetDomains(&mut self, ppdomaintree: *mut *mut DOMAIN_TREE, dwflags: u32) -> ::windows::core::Result<()>;
    fn FreeDomains(&mut self, ppdomaintree: *mut *mut DOMAIN_TREE) -> ::windows::core::Result<()>;
    fn FlushCachedDomains(&mut self) -> ::windows::core::Result<()>;
    fn SetComputer(&mut self, pszcomputername: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR, pszpassword: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDsBrowseDomainTreeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDsBrowseDomainTreeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDsBrowseDomainTreeVtbl {
        unsafe extern "system" fn BrowseTo<Impl: IDsBrowseDomainTreeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ppsztargetpath: *mut super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BrowseTo(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&ppsztargetpath), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetDomains<Impl: IDsBrowseDomainTreeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdomaintree: *mut *mut DOMAIN_TREE, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDomains(::core::mem::transmute_copy(&ppdomaintree), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn FreeDomains<Impl: IDsBrowseDomainTreeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdomaintree: *mut *mut DOMAIN_TREE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FreeDomains(::core::mem::transmute_copy(&ppdomaintree)).into()
        }
        unsafe extern "system" fn FlushCachedDomains<Impl: IDsBrowseDomainTreeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FlushCachedDomains().into()
        }
        unsafe extern "system" fn SetComputer<Impl: IDsBrowseDomainTreeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcomputername: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR, pszpassword: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetComputer(::core::mem::transmute_copy(&pszcomputername), ::core::mem::transmute_copy(&pszusername), ::core::mem::transmute_copy(&pszpassword)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            BrowseTo: BrowseTo::<Impl, IMPL_OFFSET>,
            GetDomains: GetDomains::<Impl, IMPL_OFFSET>,
            FreeDomains: FreeDomains::<Impl, IMPL_OFFSET>,
            FlushCachedDomains: FlushCachedDomains::<Impl, IMPL_OFFSET>,
            SetComputer: SetComputer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDsBrowseDomainTree as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IDsDisplaySpecifierImpl: Sized {
    fn SetServer(&mut self, pszserver: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR, pszpassword: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::Result<()>;
    fn SetLanguageID(&mut self, langid: u16) -> ::windows::core::Result<()>;
    fn GetDisplaySpecifier(&mut self, pszobjectclass: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetIconLocation(&mut self, pszobjectclass: super::super::Foundation::PWSTR, dwflags: u32, pszbuffer: super::super::Foundation::PWSTR, cchbuffer: i32, presid: *mut i32) -> ::windows::core::Result<()>;
    fn GetIcon(&mut self, pszobjectclass: super::super::Foundation::PWSTR, dwflags: u32, cxicon: i32, cyicon: i32) -> super::super::UI::WindowsAndMessaging::HICON;
    fn GetFriendlyClassName(&mut self, pszobjectclass: super::super::Foundation::PWSTR, pszbuffer: super::super::Foundation::PWSTR, cchbuffer: i32) -> ::windows::core::Result<()>;
    fn GetFriendlyAttributeName(&mut self, pszobjectclass: super::super::Foundation::PWSTR, pszattributename: super::super::Foundation::PWSTR, pszbuffer: super::super::Foundation::PWSTR, cchbuffer: u32) -> ::windows::core::Result<()>;
    fn IsClassContainer(&mut self, pszobjectclass: super::super::Foundation::PWSTR, pszadspath: super::super::Foundation::PWSTR, dwflags: u32) -> super::super::Foundation::BOOL;
    fn GetClassCreationInfo(&mut self, pszobjectclass: super::super::Foundation::PWSTR, ppdscci: *mut *mut DSCLASSCREATIONINFO) -> ::windows::core::Result<()>;
    fn EnumClassAttributes(&mut self, pszobjectclass: super::super::Foundation::PWSTR, pcbenum: LPDSENUMATTRIBUTES, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<()>;
    fn GetAttributeADsType(&mut self, pszattributename: super::super::Foundation::PWSTR) -> ADSTYPEENUM;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IDsDisplaySpecifierVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDsDisplaySpecifierImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDsDisplaySpecifierVtbl {
        unsafe extern "system" fn SetServer<Impl: IDsDisplaySpecifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszserver: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR, pszpassword: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetServer(::core::mem::transmute_copy(&pszserver), ::core::mem::transmute_copy(&pszusername), ::core::mem::transmute_copy(&pszpassword), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn SetLanguageID<Impl: IDsDisplaySpecifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLanguageID(::core::mem::transmute_copy(&langid)).into()
        }
        unsafe extern "system" fn GetDisplaySpecifier<Impl: IDsDisplaySpecifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectclass: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDisplaySpecifier(::core::mem::transmute_copy(&pszobjectclass), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn GetIconLocation<Impl: IDsDisplaySpecifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectclass: super::super::Foundation::PWSTR, dwflags: u32, pszbuffer: super::super::Foundation::PWSTR, cchbuffer: i32, presid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetIconLocation(::core::mem::transmute_copy(&pszobjectclass), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pszbuffer), ::core::mem::transmute_copy(&cchbuffer), ::core::mem::transmute_copy(&presid)).into()
        }
        unsafe extern "system" fn GetIcon<Impl: IDsDisplaySpecifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectclass: super::super::Foundation::PWSTR, dwflags: u32, cxicon: i32, cyicon: i32) -> super::super::UI::WindowsAndMessaging::HICON {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetIcon(::core::mem::transmute_copy(&pszobjectclass), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&cxicon), ::core::mem::transmute_copy(&cyicon))
        }
        unsafe extern "system" fn GetFriendlyClassName<Impl: IDsDisplaySpecifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectclass: super::super::Foundation::PWSTR, pszbuffer: super::super::Foundation::PWSTR, cchbuffer: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFriendlyClassName(::core::mem::transmute_copy(&pszobjectclass), ::core::mem::transmute_copy(&pszbuffer), ::core::mem::transmute_copy(&cchbuffer)).into()
        }
        unsafe extern "system" fn GetFriendlyAttributeName<Impl: IDsDisplaySpecifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectclass: super::super::Foundation::PWSTR, pszattributename: super::super::Foundation::PWSTR, pszbuffer: super::super::Foundation::PWSTR, cchbuffer: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFriendlyAttributeName(::core::mem::transmute_copy(&pszobjectclass), ::core::mem::transmute_copy(&pszattributename), ::core::mem::transmute_copy(&pszbuffer), ::core::mem::transmute_copy(&cchbuffer)).into()
        }
        unsafe extern "system" fn IsClassContainer<Impl: IDsDisplaySpecifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectclass: super::super::Foundation::PWSTR, pszadspath: super::super::Foundation::PWSTR, dwflags: u32) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsClassContainer(::core::mem::transmute_copy(&pszobjectclass), ::core::mem::transmute_copy(&pszadspath), ::core::mem::transmute_copy(&dwflags))
        }
        unsafe extern "system" fn GetClassCreationInfo<Impl: IDsDisplaySpecifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectclass: super::super::Foundation::PWSTR, ppdscci: *mut *mut DSCLASSCREATIONINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetClassCreationInfo(::core::mem::transmute_copy(&pszobjectclass), ::core::mem::transmute_copy(&ppdscci)).into()
        }
        unsafe extern "system" fn EnumClassAttributes<Impl: IDsDisplaySpecifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectclass: super::super::Foundation::PWSTR, pcbenum: ::windows::core::RawPtr, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumClassAttributes(::core::mem::transmute_copy(&pszobjectclass), ::core::mem::transmute_copy(&pcbenum), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn GetAttributeADsType<Impl: IDsDisplaySpecifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszattributename: super::super::Foundation::PWSTR) -> ADSTYPEENUM {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAttributeADsType(::core::mem::transmute_copy(&pszattributename))
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetServer: SetServer::<Impl, IMPL_OFFSET>,
            SetLanguageID: SetLanguageID::<Impl, IMPL_OFFSET>,
            GetDisplaySpecifier: GetDisplaySpecifier::<Impl, IMPL_OFFSET>,
            GetIconLocation: GetIconLocation::<Impl, IMPL_OFFSET>,
            GetIcon: GetIcon::<Impl, IMPL_OFFSET>,
            GetFriendlyClassName: GetFriendlyClassName::<Impl, IMPL_OFFSET>,
            GetFriendlyAttributeName: GetFriendlyAttributeName::<Impl, IMPL_OFFSET>,
            IsClassContainer: IsClassContainer::<Impl, IMPL_OFFSET>,
            GetClassCreationInfo: GetClassCreationInfo::<Impl, IMPL_OFFSET>,
            EnumClassAttributes: EnumClassAttributes::<Impl, IMPL_OFFSET>,
            GetAttributeADsType: GetAttributeADsType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDsDisplaySpecifier as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IDsObjectPickerImpl: Sized {
    fn Initialize(&mut self, pinitinfo: *mut DSOP_INIT_INFO) -> ::windows::core::Result<()>;
    fn InvokeDialog(&mut self, hwndparent: super::super::Foundation::HWND) -> ::windows::core::Result<super::super::System::Com::IDataObject>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IDsObjectPickerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDsObjectPickerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDsObjectPickerVtbl {
        unsafe extern "system" fn Initialize<Impl: IDsObjectPickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinitinfo: *mut DSOP_INIT_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&pinitinfo)).into()
        }
        unsafe extern "system" fn InvokeDialog<Impl: IDsObjectPickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ppdoselections: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InvokeDialog(::core::mem::transmute_copy(&hwndparent)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdoselections = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            InvokeDialog: InvokeDialog::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDsObjectPicker as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IDsObjectPickerCredentialsImpl: Sized + IDsObjectPickerImpl {
    fn SetCredentials(&mut self, szusername: super::super::Foundation::PWSTR, szpassword: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IDsObjectPickerCredentialsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDsObjectPickerCredentialsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDsObjectPickerCredentialsVtbl {
        unsafe extern "system" fn SetCredentials<Impl: IDsObjectPickerCredentialsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szusername: super::super::Foundation::PWSTR, szpassword: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCredentials(::core::mem::transmute_copy(&szusername), ::core::mem::transmute_copy(&szpassword)).into()
        }
        Self { base: IDsObjectPickerVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), SetCredentials: SetCredentials::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDsObjectPickerCredentials as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IPersistQueryImpl: Sized + IPersistImpl {
    fn WriteString(&mut self, psection: super::super::Foundation::PWSTR, pvaluename: super::super::Foundation::PWSTR, pvalue: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn ReadString(&mut self, psection: super::super::Foundation::PWSTR, pvaluename: super::super::Foundation::PWSTR, pbuffer: super::super::Foundation::PWSTR, cchbuffer: i32) -> ::windows::core::Result<()>;
    fn WriteInt(&mut self, psection: super::super::Foundation::PWSTR, pvaluename: super::super::Foundation::PWSTR, value: i32) -> ::windows::core::Result<()>;
    fn ReadInt(&mut self, psection: super::super::Foundation::PWSTR, pvaluename: super::super::Foundation::PWSTR, pvalue: *mut i32) -> ::windows::core::Result<()>;
    fn WriteStruct(&mut self, psection: super::super::Foundation::PWSTR, pvaluename: super::super::Foundation::PWSTR, pstruct: *mut ::core::ffi::c_void, cbstruct: u32) -> ::windows::core::Result<()>;
    fn ReadStruct(&mut self, psection: super::super::Foundation::PWSTR, pvaluename: super::super::Foundation::PWSTR, pstruct: *mut ::core::ffi::c_void, cbstruct: u32) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IPersistQueryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPersistQueryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPersistQueryVtbl {
        unsafe extern "system" fn WriteString<Impl: IPersistQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psection: super::super::Foundation::PWSTR, pvaluename: super::super::Foundation::PWSTR, pvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteString(::core::mem::transmute_copy(&psection), ::core::mem::transmute_copy(&pvaluename), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn ReadString<Impl: IPersistQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psection: super::super::Foundation::PWSTR, pvaluename: super::super::Foundation::PWSTR, pbuffer: super::super::Foundation::PWSTR, cchbuffer: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReadString(::core::mem::transmute_copy(&psection), ::core::mem::transmute_copy(&pvaluename), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&cchbuffer)).into()
        }
        unsafe extern "system" fn WriteInt<Impl: IPersistQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psection: super::super::Foundation::PWSTR, pvaluename: super::super::Foundation::PWSTR, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteInt(::core::mem::transmute_copy(&psection), ::core::mem::transmute_copy(&pvaluename), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ReadInt<Impl: IPersistQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psection: super::super::Foundation::PWSTR, pvaluename: super::super::Foundation::PWSTR, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReadInt(::core::mem::transmute_copy(&psection), ::core::mem::transmute_copy(&pvaluename), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn WriteStruct<Impl: IPersistQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psection: super::super::Foundation::PWSTR, pvaluename: super::super::Foundation::PWSTR, pstruct: *mut ::core::ffi::c_void, cbstruct: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteStruct(::core::mem::transmute_copy(&psection), ::core::mem::transmute_copy(&pvaluename), ::core::mem::transmute_copy(&pstruct), ::core::mem::transmute_copy(&cbstruct)).into()
        }
        unsafe extern "system" fn ReadStruct<Impl: IPersistQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psection: super::super::Foundation::PWSTR, pvaluename: super::super::Foundation::PWSTR, pstruct: *mut ::core::ffi::c_void, cbstruct: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReadStruct(::core::mem::transmute_copy(&psection), ::core::mem::transmute_copy(&pvaluename), ::core::mem::transmute_copy(&pstruct), ::core::mem::transmute_copy(&cbstruct)).into()
        }
        unsafe extern "system" fn Clear<Impl: IPersistQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        Self {
            base: IPersistVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            WriteString: WriteString::<Impl, IMPL_OFFSET>,
            ReadString: ReadString::<Impl, IMPL_OFFSET>,
            WriteInt: WriteInt::<Impl, IMPL_OFFSET>,
            ReadInt: ReadInt::<Impl, IMPL_OFFSET>,
            WriteStruct: WriteStruct::<Impl, IMPL_OFFSET>,
            ReadStruct: ReadStruct::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPersistQuery as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrivateDispatchImpl: Sized {
    fn ADSIInitializeDispatchManager(&mut self, dwextensionid: i32) -> ::windows::core::Result<()>;
    fn ADSIGetTypeInfoCount(&mut self) -> ::windows::core::Result<u32>;
    fn ADSIGetTypeInfo(&mut self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo>;
    fn ADSIGetIDsOfNames(&mut self, riid: *const ::windows::core::GUID, rgsznames: *const *const u16, cnames: u32, lcid: u32) -> ::windows::core::Result<i32>;
    fn ADSIInvoke(&mut self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrivateDispatchVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrivateDispatchImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrivateDispatchVtbl {
        unsafe extern "system" fn ADSIInitializeDispatchManager<Impl: IPrivateDispatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwextensionid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ADSIInitializeDispatchManager(::core::mem::transmute_copy(&dwextensionid)).into()
        }
        unsafe extern "system" fn ADSIGetTypeInfoCount<Impl: IPrivateDispatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ADSIGetTypeInfoCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pctinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ADSIGetTypeInfo<Impl: IPrivateDispatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ADSIGetTypeInfo(::core::mem::transmute_copy(&itinfo), ::core::mem::transmute_copy(&lcid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ADSIGetIDsOfNames<Impl: IPrivateDispatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const *const u16, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ADSIGetIDsOfNames(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&rgsznames), ::core::mem::transmute_copy(&cnames), ::core::mem::transmute_copy(&lcid)) {
                ::core::result::Result::Ok(ok__) => {
                    *rgdispid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ADSIInvoke<Impl: IPrivateDispatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ADSIInvoke(::core::mem::transmute_copy(&dispidmember), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&lcid), ::core::mem::transmute_copy(&wflags), ::core::mem::transmute_copy(&pdispparams), ::core::mem::transmute_copy(&pvarresult), ::core::mem::transmute_copy(&pexcepinfo), ::core::mem::transmute_copy(&puargerr)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ADSIInitializeDispatchManager: ADSIInitializeDispatchManager::<Impl, IMPL_OFFSET>,
            ADSIGetTypeInfoCount: ADSIGetTypeInfoCount::<Impl, IMPL_OFFSET>,
            ADSIGetTypeInfo: ADSIGetTypeInfo::<Impl, IMPL_OFFSET>,
            ADSIGetIDsOfNames: ADSIGetIDsOfNames::<Impl, IMPL_OFFSET>,
            ADSIInvoke: ADSIInvoke::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrivateDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPrivateUnknownImpl: Sized {
    fn ADSIInitializeObject(&mut self, lpszusername: super::super::Foundation::BSTR, lpszpassword: super::super::Foundation::BSTR, lnreserved: i32) -> ::windows::core::Result<()>;
    fn ADSIReleaseObject(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPrivateUnknownVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrivateUnknownImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrivateUnknownVtbl {
        unsafe extern "system" fn ADSIInitializeObject<Impl: IPrivateUnknownImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lpszpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lnreserved: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ADSIInitializeObject(::core::mem::transmute_copy(&lpszusername), ::core::mem::transmute_copy(&lpszpassword), ::core::mem::transmute_copy(&lnreserved)).into()
        }
        unsafe extern "system" fn ADSIReleaseObject<Impl: IPrivateUnknownImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ADSIReleaseObject().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ADSIInitializeObject: ADSIInitializeObject::<Impl, IMPL_OFFSET>,
            ADSIReleaseObject: ADSIReleaseObject::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrivateUnknown as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IQueryFormImpl: Sized {
    fn Initialize(&mut self, hkform: super::super::System::Registry::HKEY) -> ::windows::core::Result<()>;
    fn AddForms(&mut self, paddformsproc: LPCQADDFORMSPROC, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<()>;
    fn AddPages(&mut self, paddpagesproc: LPCQADDPAGESPROC, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry", feature = "Win32_UI_WindowsAndMessaging"))]
impl IQueryFormVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IQueryFormImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IQueryFormVtbl {
        unsafe extern "system" fn Initialize<Impl: IQueryFormImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkform: super::super::System::Registry::HKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&hkform)).into()
        }
        unsafe extern "system" fn AddForms<Impl: IQueryFormImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddformsproc: ::windows::core::RawPtr, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddForms(::core::mem::transmute_copy(&paddformsproc), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn AddPages<Impl: IQueryFormImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddpagesproc: ::windows::core::RawPtr, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPages(::core::mem::transmute_copy(&paddpagesproc), ::core::mem::transmute_copy(&lparam)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            AddForms: AddForms::<Impl, IMPL_OFFSET>,
            AddPages: AddPages::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IQueryForm as ::windows::core::Interface>::IID
    }
}
