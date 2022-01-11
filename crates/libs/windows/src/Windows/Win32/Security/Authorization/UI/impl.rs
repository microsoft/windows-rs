#[cfg(feature = "Win32_Foundation")]
pub trait IEffectivePermissionImpl: Sized {
    fn GetEffectivePermission();
}
#[cfg(feature = "Win32_Foundation")]
impl IEffectivePermissionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEffectivePermissionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEffectivePermissionVtbl {
        unsafe extern "system" fn GetEffectivePermission<Impl: IEffectivePermissionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidobjecttype: *const ::windows::core::GUID, pusersid: super::super::super::Foundation::PSID, pszservername: super::super::super::Foundation::PWSTR, psd: *mut super::super::SECURITY_DESCRIPTOR, ppobjecttypelist: *mut *mut super::super::OBJECT_TYPE_LIST, pcobjecttypelistlength: *mut u32, ppgrantedaccesslist: *mut *mut u32, pcgrantedaccesslistlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetEffectivePermission: GetEffectivePermission::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEffectivePermission as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEffectivePermission2Impl: Sized {
    fn ComputeEffectivePermissionWithSecondarySecurity();
}
#[cfg(feature = "Win32_Foundation")]
impl IEffectivePermission2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEffectivePermission2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEffectivePermission2Vtbl {
        unsafe extern "system" fn ComputeEffectivePermissionWithSecondarySecurity<Impl: IEffectivePermission2Impl, const OFFSET: isize>(
            this: *mut ::core::ffi::c_void,
            psid: super::super::super::Foundation::PSID,
            pdevicesid: super::super::super::Foundation::PSID,
            pszservername: super::super::super::Foundation::PWSTR,
            psecurityobjects: *mut SECURITY_OBJECT,
            dwsecurityobjectcount: u32,
            pusergroups: *const super::super::TOKEN_GROUPS,
            pauthzusergroupsoperations: *const super::AUTHZ_SID_OPERATION,
            pdevicegroups: *const super::super::TOKEN_GROUPS,
            pauthzdevicegroupsoperations: *const super::AUTHZ_SID_OPERATION,
            pauthzuserclaims: *const super::AUTHZ_SECURITY_ATTRIBUTES_INFORMATION,
            pauthzuserclaimsoperations: *const super::AUTHZ_SECURITY_ATTRIBUTE_OPERATION,
            pauthzdeviceclaims: *const super::AUTHZ_SECURITY_ATTRIBUTES_INFORMATION,
            pauthzdeviceclaimsoperations: *const super::AUTHZ_SECURITY_ATTRIBUTE_OPERATION,
            peffpermresultlists: *mut EFFPERM_RESULT_LIST,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ComputeEffectivePermissionWithSecondarySecurity: ComputeEffectivePermissionWithSecondarySecurity::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEffectivePermission2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
pub trait ISecurityInformationImpl: Sized {
    fn GetObjectInformation();
    fn GetSecurity();
    fn SetSecurity();
    fn GetAccessRights();
    fn MapGeneric();
    fn GetInheritTypes();
    fn PropertySheetPageCallback();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ISecurityInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecurityInformationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISecurityInformationVtbl {
        unsafe extern "system" fn GetObjectInformation<Impl: ISecurityInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobjectinfo: *mut SI_OBJECT_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSecurity<Impl: ISecurityInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestedinformation: super::super::OBJECT_SECURITY_INFORMATION, ppsecuritydescriptor: *mut *mut super::super::SECURITY_DESCRIPTOR, fdefault: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSecurity<Impl: ISecurityInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, securityinformation: super::super::OBJECT_SECURITY_INFORMATION, psecuritydescriptor: *mut super::super::SECURITY_DESCRIPTOR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAccessRights<Impl: ISecurityInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidobjecttype: *const ::windows::core::GUID, dwflags: SECURITY_INFO_PAGE_FLAGS, ppaccess: *mut *mut SI_ACCESS, pcaccesses: *mut u32, pidefaultaccess: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MapGeneric<Impl: ISecurityInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidobjecttype: *const ::windows::core::GUID, paceflags: *mut u8, pmask: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInheritTypes<Impl: ISecurityInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppinherittypes: *mut *mut SI_INHERIT_TYPE, pcinherittypes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PropertySheetPageCallback<Impl: ISecurityInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, umsg: super::super::super::UI::Controls::PSPCB_MESSAGE, upage: SI_PAGE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetObjectInformation: GetObjectInformation::<Impl, IMPL_OFFSET>,
            GetSecurity: GetSecurity::<Impl, IMPL_OFFSET>,
            SetSecurity: SetSecurity::<Impl, IMPL_OFFSET>,
            GetAccessRights: GetAccessRights::<Impl, IMPL_OFFSET>,
            MapGeneric: MapGeneric::<Impl, IMPL_OFFSET>,
            GetInheritTypes: GetInheritTypes::<Impl, IMPL_OFFSET>,
            PropertySheetPageCallback: PropertySheetPageCallback::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISecurityInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ISecurityInformation2Impl: Sized {
    fn IsDaclCanonical();
    fn LookupSids();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ISecurityInformation2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecurityInformation2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISecurityInformation2Vtbl {
        unsafe extern "system" fn IsDaclCanonical<Impl: ISecurityInformation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdacl: *mut super::super::ACL) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LookupSids<Impl: ISecurityInformation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, csids: u32, rgpsids: *mut super::super::super::Foundation::PSID, ppdo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            IsDaclCanonical: IsDaclCanonical::<Impl, IMPL_OFFSET>,
            LookupSids: LookupSids::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISecurityInformation2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISecurityInformation3Impl: Sized {
    fn GetFullResourceName();
    fn OpenElevatedEditor();
}
#[cfg(feature = "Win32_Foundation")]
impl ISecurityInformation3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecurityInformation3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISecurityInformation3Vtbl {
        unsafe extern "system" fn GetFullResourceName<Impl: ISecurityInformation3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszresourcename: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OpenElevatedEditor<Impl: ISecurityInformation3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, upage: SI_PAGE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetFullResourceName: GetFullResourceName::<Impl, IMPL_OFFSET>,
            OpenElevatedEditor: OpenElevatedEditor::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISecurityInformation3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISecurityInformation4Impl: Sized {
    fn GetSecondarySecurity();
}
#[cfg(feature = "Win32_Foundation")]
impl ISecurityInformation4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecurityInformation4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISecurityInformation4Vtbl {
        unsafe extern "system" fn GetSecondarySecurity<Impl: ISecurityInformation4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psecurityobjects: *mut *mut SECURITY_OBJECT, psecurityobjectcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetSecondarySecurity: GetSecondarySecurity::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISecurityInformation4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISecurityObjectTypeInfoImpl: Sized {
    fn GetInheritSource();
}
#[cfg(feature = "Win32_Foundation")]
impl ISecurityObjectTypeInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecurityObjectTypeInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISecurityObjectTypeInfoVtbl {
        unsafe extern "system" fn GetInheritSource<Impl: ISecurityObjectTypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, si: u32, pacl: *mut super::super::ACL, ppinheritarray: *mut *mut super::INHERITED_FROMA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetInheritSource: GetInheritSource::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISecurityObjectTypeInfo as ::windows::core::Interface>::IID
    }
}
