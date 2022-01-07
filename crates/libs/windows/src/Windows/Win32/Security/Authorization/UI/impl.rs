pub trait IEffectivePermissionImpl: Sized {
    fn GetEffectivePermission();
}
impl ::windows::core::RuntimeName for IEffectivePermission {
    const NAME: &'static str = "Windows.Win32.Security.Authorization.UI.IEffectivePermission";
}
impl IEffectivePermissionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEffectivePermissionImpl, const OFFSET: isize>() -> IEffectivePermissionVtbl {
        unsafe extern "system" fn GetEffectivePermission<Impl: IEffectivePermissionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidobjecttype: *const ::windows::core::GUID, pusersid: super::super::super::Foundation::PSID, pszservername: super::super::super::Foundation::PWSTR, psd: *mut super::super::SECURITY_DESCRIPTOR, ppobjecttypelist: *mut *mut super::super::OBJECT_TYPE_LIST, pcobjecttypelistlength: *mut u32, ppgrantedaccesslist: *mut *mut u32, pcgrantedaccesslistlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEffectivePermission(
                &*(&pguidobjecttype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&pusersid as *const <super::super::super::Foundation::PSID as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PSID as ::windows::core::DefaultType>::DefaultType),
                &*(&pszservername as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&psd as *const <super::super::SECURITY_DESCRIPTOR as ::windows::core::Abi>::Abi as *const <super::super::SECURITY_DESCRIPTOR as ::windows::core::DefaultType>::DefaultType),
                &*(&ppobjecttypelist as *const <super::super::OBJECT_TYPE_LIST as ::windows::core::Abi>::Abi as *const <super::super::OBJECT_TYPE_LIST as ::windows::core::DefaultType>::DefaultType),
                pcobjecttypelistlength,
                ppgrantedaccesslist,
                pcgrantedaccesslistlength,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEffectivePermission>, ::windows::core::GetTrustLevel, GetEffectivePermission::<Impl, OFFSET>)
    }
}
pub trait IEffectivePermission2Impl: Sized {
    fn ComputeEffectivePermissionWithSecondarySecurity();
}
impl ::windows::core::RuntimeName for IEffectivePermission2 {
    const NAME: &'static str = "Windows.Win32.Security.Authorization.UI.IEffectivePermission2";
}
impl IEffectivePermission2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEffectivePermission2Impl, const OFFSET: isize>() -> IEffectivePermission2Vtbl {
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
            match (*this).ComputeEffectivePermissionWithSecondarySecurity(
                &*(&psid as *const <super::super::super::Foundation::PSID as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PSID as ::windows::core::DefaultType>::DefaultType),
                &*(&pdevicesid as *const <super::super::super::Foundation::PSID as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PSID as ::windows::core::DefaultType>::DefaultType),
                &*(&pszservername as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&psecurityobjects as *const <SECURITY_OBJECT as ::windows::core::Abi>::Abi as *const <SECURITY_OBJECT as ::windows::core::DefaultType>::DefaultType),
                dwsecurityobjectcount,
                &*(&pusergroups as *const <super::super::TOKEN_GROUPS as ::windows::core::Abi>::Abi as *const <super::super::TOKEN_GROUPS as ::windows::core::DefaultType>::DefaultType),
                pauthzusergroupsoperations,
                &*(&pdevicegroups as *const <super::super::TOKEN_GROUPS as ::windows::core::Abi>::Abi as *const <super::super::TOKEN_GROUPS as ::windows::core::DefaultType>::DefaultType),
                pauthzdevicegroupsoperations,
                &*(&pauthzuserclaims as *const <super::AUTHZ_SECURITY_ATTRIBUTES_INFORMATION as ::windows::core::Abi>::Abi as *const <super::AUTHZ_SECURITY_ATTRIBUTES_INFORMATION as ::windows::core::DefaultType>::DefaultType),
                pauthzuserclaimsoperations,
                &*(&pauthzdeviceclaims as *const <super::AUTHZ_SECURITY_ATTRIBUTES_INFORMATION as ::windows::core::Abi>::Abi as *const <super::AUTHZ_SECURITY_ATTRIBUTES_INFORMATION as ::windows::core::DefaultType>::DefaultType),
                pauthzdeviceclaimsoperations,
                &*(&peffpermresultlists as *const <EFFPERM_RESULT_LIST as ::windows::core::Abi>::Abi as *const <EFFPERM_RESULT_LIST as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEffectivePermission2>, ::windows::core::GetTrustLevel, ComputeEffectivePermissionWithSecondarySecurity::<Impl, OFFSET>)
    }
}
pub trait ISecurityInformationImpl: Sized {
    fn GetObjectInformation();
    fn GetSecurity();
    fn SetSecurity();
    fn GetAccessRights();
    fn MapGeneric();
    fn GetInheritTypes();
    fn PropertySheetPageCallback();
}
impl ::windows::core::RuntimeName for ISecurityInformation {
    const NAME: &'static str = "Windows.Win32.Security.Authorization.UI.ISecurityInformation";
}
impl ISecurityInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecurityInformationImpl, const OFFSET: isize>() -> ISecurityInformationVtbl {
        unsafe extern "system" fn GetObjectInformation<Impl: ISecurityInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobjectinfo: *mut SI_OBJECT_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetObjectInformation(&*(&pobjectinfo as *const <SI_OBJECT_INFO as ::windows::core::Abi>::Abi as *const <SI_OBJECT_INFO as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSecurity<Impl: ISecurityInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestedinformation: super::super::OBJECT_SECURITY_INFORMATION, ppsecuritydescriptor: *mut *mut super::super::SECURITY_DESCRIPTOR, fdefault: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSecurity(requestedinformation, &*(&ppsecuritydescriptor as *const <super::super::SECURITY_DESCRIPTOR as ::windows::core::Abi>::Abi as *const <super::super::SECURITY_DESCRIPTOR as ::windows::core::DefaultType>::DefaultType), &*(&fdefault as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurity<Impl: ISecurityInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, securityinformation: super::super::OBJECT_SECURITY_INFORMATION, psecuritydescriptor: *mut super::super::SECURITY_DESCRIPTOR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSecurity(securityinformation, &*(&psecuritydescriptor as *const <super::super::SECURITY_DESCRIPTOR as ::windows::core::Abi>::Abi as *const <super::super::SECURITY_DESCRIPTOR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAccessRights<Impl: ISecurityInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidobjecttype: *const ::windows::core::GUID, dwflags: SECURITY_INFO_PAGE_FLAGS, ppaccess: *mut *mut SI_ACCESS, pcaccesses: *mut u32, pidefaultaccess: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAccessRights(&*(&pguidobjecttype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwflags, &*(&ppaccess as *const <SI_ACCESS as ::windows::core::Abi>::Abi as *const <SI_ACCESS as ::windows::core::DefaultType>::DefaultType), pcaccesses, pidefaultaccess) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MapGeneric<Impl: ISecurityInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidobjecttype: *const ::windows::core::GUID, paceflags: *mut u8, pmask: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapGeneric(&*(&pguidobjecttype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), paceflags, pmask) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInheritTypes<Impl: ISecurityInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppinherittypes: *mut *mut SI_INHERIT_TYPE, pcinherittypes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInheritTypes(&*(&ppinherittypes as *const <SI_INHERIT_TYPE as ::windows::core::Abi>::Abi as *const <SI_INHERIT_TYPE as ::windows::core::DefaultType>::DefaultType), pcinherittypes) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PropertySheetPageCallback<Impl: ISecurityInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, umsg: super::super::super::UI::Controls::PSPCB_MESSAGE, upage: SI_PAGE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PropertySheetPageCallback(&*(&hwnd as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), umsg, upage) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ISecurityInformation>,
            ::windows::core::GetTrustLevel,
            GetObjectInformation::<Impl, OFFSET>,
            GetSecurity::<Impl, OFFSET>,
            SetSecurity::<Impl, OFFSET>,
            GetAccessRights::<Impl, OFFSET>,
            MapGeneric::<Impl, OFFSET>,
            GetInheritTypes::<Impl, OFFSET>,
            PropertySheetPageCallback::<Impl, OFFSET>,
        )
    }
}
pub trait ISecurityInformation2Impl: Sized {
    fn IsDaclCanonical();
    fn LookupSids();
}
impl ::windows::core::RuntimeName for ISecurityInformation2 {
    const NAME: &'static str = "Windows.Win32.Security.Authorization.UI.ISecurityInformation2";
}
impl ISecurityInformation2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecurityInformation2Impl, const OFFSET: isize>() -> ISecurityInformation2Vtbl {
        unsafe extern "system" fn IsDaclCanonical<Impl: ISecurityInformation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdacl: *mut super::super::ACL) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDaclCanonical(&*(&pdacl as *const <super::super::ACL as ::windows::core::Abi>::Abi as *const <super::super::ACL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupSids<Impl: ISecurityInformation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, csids: u32, rgpsids: *mut super::super::super::Foundation::PSID, ppdo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LookupSids(csids, &*(&rgpsids as *const <super::super::super::Foundation::PSID as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PSID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppdo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISecurityInformation2>, ::windows::core::GetTrustLevel, IsDaclCanonical::<Impl, OFFSET>, LookupSids::<Impl, OFFSET>)
    }
}
pub trait ISecurityInformation3Impl: Sized {
    fn GetFullResourceName();
    fn OpenElevatedEditor();
}
impl ::windows::core::RuntimeName for ISecurityInformation3 {
    const NAME: &'static str = "Windows.Win32.Security.Authorization.UI.ISecurityInformation3";
}
impl ISecurityInformation3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecurityInformation3Impl, const OFFSET: isize>() -> ISecurityInformation3Vtbl {
        unsafe extern "system" fn GetFullResourceName<Impl: ISecurityInformation3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszresourcename: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFullResourceName(::core::mem::transmute_copy(&ppszresourcename)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenElevatedEditor<Impl: ISecurityInformation3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, upage: SI_PAGE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenElevatedEditor(&*(&hwnd as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), upage) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISecurityInformation3>, ::windows::core::GetTrustLevel, GetFullResourceName::<Impl, OFFSET>, OpenElevatedEditor::<Impl, OFFSET>)
    }
}
pub trait ISecurityInformation4Impl: Sized {
    fn GetSecondarySecurity();
}
impl ::windows::core::RuntimeName for ISecurityInformation4 {
    const NAME: &'static str = "Windows.Win32.Security.Authorization.UI.ISecurityInformation4";
}
impl ISecurityInformation4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecurityInformation4Impl, const OFFSET: isize>() -> ISecurityInformation4Vtbl {
        unsafe extern "system" fn GetSecondarySecurity<Impl: ISecurityInformation4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psecurityobjects: *mut *mut SECURITY_OBJECT, psecurityobjectcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSecondarySecurity(::core::mem::transmute_copy(&psecurityobjects), ::core::mem::transmute_copy(&psecurityobjectcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISecurityInformation4>, ::windows::core::GetTrustLevel, GetSecondarySecurity::<Impl, OFFSET>)
    }
}
pub trait ISecurityObjectTypeInfoImpl: Sized {
    fn GetInheritSource();
}
impl ::windows::core::RuntimeName for ISecurityObjectTypeInfo {
    const NAME: &'static str = "Windows.Win32.Security.Authorization.UI.ISecurityObjectTypeInfo";
}
impl ISecurityObjectTypeInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecurityObjectTypeInfoImpl, const OFFSET: isize>() -> ISecurityObjectTypeInfoVtbl {
        unsafe extern "system" fn GetInheritSource<Impl: ISecurityObjectTypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, si: u32, pacl: *mut super::super::ACL, ppinheritarray: *mut *mut super::INHERITED_FROMA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInheritSource(si, &*(&pacl as *const <super::super::ACL as ::windows::core::Abi>::Abi as *const <super::super::ACL as ::windows::core::DefaultType>::DefaultType), &*(&ppinheritarray as *const <super::INHERITED_FROMA as ::windows::core::Abi>::Abi as *const <super::INHERITED_FROMA as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISecurityObjectTypeInfo>, ::windows::core::GetTrustLevel, GetInheritSource::<Impl, OFFSET>)
    }
}
