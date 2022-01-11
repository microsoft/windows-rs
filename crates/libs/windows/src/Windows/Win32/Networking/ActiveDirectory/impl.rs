#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsImpl: Sized + IDispatchImpl {
    fn Name();
    fn Class();
    fn GUID();
    fn ADsPath();
    fn Parent();
    fn Schema();
    fn GetInfo();
    fn SetInfo();
    fn Get();
    fn Put();
    fn GetEx();
    fn PutEx();
    fn GetInfoEx();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsVtbl {
        unsafe extern "system" fn Name<Impl: IADsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Class<Impl: IADsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GUID<Impl: IADsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ADsPath<Impl: IADsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Parent<Impl: IADsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Schema<Impl: IADsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInfo<Impl: IADsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInfo<Impl: IADsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Get<Impl: IADsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Put<Impl: IADsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, vprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEx<Impl: IADsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PutEx<Impl: IADsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lncontrolcode: i32, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, vprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInfoEx<Impl: IADsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vproperties: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, lnreserved: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            Class::<Impl, IMPL_OFFSET>,
            GUID::<Impl, IMPL_OFFSET>,
            ADsPath::<Impl, IMPL_OFFSET>,
            Parent::<Impl, IMPL_OFFSET>,
            Schema::<Impl, IMPL_OFFSET>,
            GetInfo::<Impl, IMPL_OFFSET>,
            SetInfo::<Impl, IMPL_OFFSET>,
            Get::<Impl, IMPL_OFFSET>,
            Put::<Impl, IMPL_OFFSET>,
            GetEx::<Impl, IMPL_OFFSET>,
            PutEx::<Impl, IMPL_OFFSET>,
            GetInfoEx::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsADSystemInfoImpl: Sized + IDispatchImpl {
    fn UserName();
    fn ComputerName();
    fn SiteName();
    fn DomainShortName();
    fn DomainDNSName();
    fn ForestDNSName();
    fn PDCRoleOwner();
    fn SchemaRoleOwner();
    fn IsNativeMode();
    fn GetAnyDCName();
    fn GetDCSiteName();
    fn RefreshSchemaCache();
    fn GetTrees();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsADSystemInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsADSystemInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsADSystemInfoVtbl {
        unsafe extern "system" fn UserName<Impl: IADsADSystemInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ComputerName<Impl: IADsADSystemInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SiteName<Impl: IADsADSystemInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DomainShortName<Impl: IADsADSystemInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DomainDNSName<Impl: IADsADSystemInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ForestDNSName<Impl: IADsADSystemInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PDCRoleOwner<Impl: IADsADSystemInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SchemaRoleOwner<Impl: IADsADSystemInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsNativeMode<Impl: IADsADSystemInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAnyDCName<Impl: IADsADSystemInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdcname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDCSiteName<Impl: IADsADSystemInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szserver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pszsitename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RefreshSchemaCache<Impl: IADsADSystemInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTrees<Impl: IADsADSystemInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvtrees: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            UserName::<Impl, IMPL_OFFSET>,
            ComputerName::<Impl, IMPL_OFFSET>,
            SiteName::<Impl, IMPL_OFFSET>,
            DomainShortName::<Impl, IMPL_OFFSET>,
            DomainDNSName::<Impl, IMPL_OFFSET>,
            ForestDNSName::<Impl, IMPL_OFFSET>,
            PDCRoleOwner::<Impl, IMPL_OFFSET>,
            SchemaRoleOwner::<Impl, IMPL_OFFSET>,
            IsNativeMode::<Impl, IMPL_OFFSET>,
            GetAnyDCName::<Impl, IMPL_OFFSET>,
            GetDCSiteName::<Impl, IMPL_OFFSET>,
            RefreshSchemaCache::<Impl, IMPL_OFFSET>,
            GetTrees::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsADSystemInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsAccessControlEntryImpl: Sized + IDispatchImpl {
    fn AccessMask();
    fn SetAccessMask();
    fn AceType();
    fn SetAceType();
    fn AceFlags();
    fn SetAceFlags();
    fn Flags();
    fn SetFlags();
    fn ObjectType();
    fn SetObjectType();
    fn InheritedObjectType();
    fn SetInheritedObjectType();
    fn Trustee();
    fn SetTrustee();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsAccessControlEntryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsAccessControlEntryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsAccessControlEntryVtbl {
        unsafe extern "system" fn AccessMask<Impl: IADsAccessControlEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAccessMask<Impl: IADsAccessControlEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnaccessmask: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AceType<Impl: IADsAccessControlEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAceType<Impl: IADsAccessControlEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnacetype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AceFlags<Impl: IADsAccessControlEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAceFlags<Impl: IADsAccessControlEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnaceflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Flags<Impl: IADsAccessControlEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFlags<Impl: IADsAccessControlEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ObjectType<Impl: IADsAccessControlEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetObjectType<Impl: IADsAccessControlEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrobjecttype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InheritedObjectType<Impl: IADsAccessControlEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInheritedObjectType<Impl: IADsAccessControlEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinheritedobjecttype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Trustee<Impl: IADsAccessControlEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTrustee<Impl: IADsAccessControlEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtrustee: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            AccessMask::<Impl, IMPL_OFFSET>,
            SetAccessMask::<Impl, IMPL_OFFSET>,
            AceType::<Impl, IMPL_OFFSET>,
            SetAceType::<Impl, IMPL_OFFSET>,
            AceFlags::<Impl, IMPL_OFFSET>,
            SetAceFlags::<Impl, IMPL_OFFSET>,
            Flags::<Impl, IMPL_OFFSET>,
            SetFlags::<Impl, IMPL_OFFSET>,
            ObjectType::<Impl, IMPL_OFFSET>,
            SetObjectType::<Impl, IMPL_OFFSET>,
            InheritedObjectType::<Impl, IMPL_OFFSET>,
            SetInheritedObjectType::<Impl, IMPL_OFFSET>,
            Trustee::<Impl, IMPL_OFFSET>,
            SetTrustee::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsAccessControlEntry as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsAccessControlListImpl: Sized + IDispatchImpl {
    fn AclRevision();
    fn SetAclRevision();
    fn AceCount();
    fn SetAceCount();
    fn AddAce();
    fn RemoveAce();
    fn CopyAccessList();
    fn _NewEnum();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsAccessControlListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsAccessControlListImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsAccessControlListVtbl {
        unsafe extern "system" fn AclRevision<Impl: IADsAccessControlListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAclRevision<Impl: IADsAccessControlListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnaclrevision: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AceCount<Impl: IADsAccessControlListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAceCount<Impl: IADsAccessControlListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnacecount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddAce<Impl: IADsAccessControlListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paccesscontrolentry: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveAce<Impl: IADsAccessControlListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paccesscontrolentry: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CopyAccessList<Impl: IADsAccessControlListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaccesscontrollist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: IADsAccessControlListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            AclRevision::<Impl, IMPL_OFFSET>,
            SetAclRevision::<Impl, IMPL_OFFSET>,
            AceCount::<Impl, IMPL_OFFSET>,
            SetAceCount::<Impl, IMPL_OFFSET>,
            AddAce::<Impl, IMPL_OFFSET>,
            RemoveAce::<Impl, IMPL_OFFSET>,
            CopyAccessList::<Impl, IMPL_OFFSET>,
            _NewEnum::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsAccessControlList as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsAclImpl: Sized + IDispatchImpl {
    fn ProtectedAttrName();
    fn SetProtectedAttrName();
    fn SubjectName();
    fn SetSubjectName();
    fn Privileges();
    fn SetPrivileges();
    fn CopyAcl();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsAclVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsAclImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsAclVtbl {
        unsafe extern "system" fn ProtectedAttrName<Impl: IADsAclImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProtectedAttrName<Impl: IADsAclImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotectedattrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SubjectName<Impl: IADsAclImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSubjectName<Impl: IADsAclImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsubjectname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Privileges<Impl: IADsAclImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPrivileges<Impl: IADsAclImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnprivileges: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CopyAcl<Impl: IADsAclImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppacl: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            ProtectedAttrName::<Impl, IMPL_OFFSET>,
            SetProtectedAttrName::<Impl, IMPL_OFFSET>,
            SubjectName::<Impl, IMPL_OFFSET>,
            SetSubjectName::<Impl, IMPL_OFFSET>,
            Privileges::<Impl, IMPL_OFFSET>,
            SetPrivileges::<Impl, IMPL_OFFSET>,
            CopyAcl::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsAcl as ::windows::core::Interface>::IID
    }
}
pub trait IADsAggregateeImpl: Sized {
    fn ConnectAsAggregatee();
    fn DisconnectAsAggregatee();
    fn RelinquishInterface();
    fn RestoreInterface();
}
impl IADsAggregateeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsAggregateeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsAggregateeVtbl {
        unsafe extern "system" fn ConnectAsAggregatee<Impl: IADsAggregateeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pouterunknown: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisconnectAsAggregatee<Impl: IADsAggregateeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RelinquishInterface<Impl: IADsAggregateeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RestoreInterface<Impl: IADsAggregateeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ConnectAsAggregatee::<Impl, IMPL_OFFSET>, DisconnectAsAggregatee::<Impl, IMPL_OFFSET>, RelinquishInterface::<Impl, IMPL_OFFSET>, RestoreInterface::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsAggregatee as ::windows::core::Interface>::IID
    }
}
pub trait IADsAggregatorImpl: Sized {
    fn ConnectAsAggregator();
    fn DisconnectAsAggregator();
}
impl IADsAggregatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsAggregatorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsAggregatorVtbl {
        unsafe extern "system" fn ConnectAsAggregator<Impl: IADsAggregatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paggregatee: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisconnectAsAggregator<Impl: IADsAggregatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ConnectAsAggregator::<Impl, IMPL_OFFSET>, DisconnectAsAggregator::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsAggregator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsBackLinkImpl: Sized + IDispatchImpl {
    fn RemoteID();
    fn SetRemoteID();
    fn ObjectName();
    fn SetObjectName();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsBackLinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsBackLinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsBackLinkVtbl {
        unsafe extern "system" fn RemoteID<Impl: IADsBackLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRemoteID<Impl: IADsBackLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnremoteid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ObjectName<Impl: IADsBackLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetObjectName<Impl: IADsBackLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrobjectname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, RemoteID::<Impl, IMPL_OFFSET>, SetRemoteID::<Impl, IMPL_OFFSET>, ObjectName::<Impl, IMPL_OFFSET>, SetObjectName::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsBackLink as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsCaseIgnoreListImpl: Sized + IDispatchImpl {
    fn CaseIgnoreList();
    fn SetCaseIgnoreList();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsCaseIgnoreListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsCaseIgnoreListImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsCaseIgnoreListVtbl {
        unsafe extern "system" fn CaseIgnoreList<Impl: IADsCaseIgnoreListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCaseIgnoreList<Impl: IADsCaseIgnoreListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcaseignorelist: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, CaseIgnoreList::<Impl, IMPL_OFFSET>, SetCaseIgnoreList::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsCaseIgnoreList as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsClassImpl: Sized + IADsImpl + IDispatchImpl {
    fn PrimaryInterface();
    fn CLSID();
    fn SetCLSID();
    fn OID();
    fn SetOID();
    fn Abstract();
    fn SetAbstract();
    fn Auxiliary();
    fn SetAuxiliary();
    fn MandatoryProperties();
    fn SetMandatoryProperties();
    fn OptionalProperties();
    fn SetOptionalProperties();
    fn NamingProperties();
    fn SetNamingProperties();
    fn DerivedFrom();
    fn SetDerivedFrom();
    fn AuxDerivedFrom();
    fn SetAuxDerivedFrom();
    fn PossibleSuperiors();
    fn SetPossibleSuperiors();
    fn Containment();
    fn SetContainment();
    fn Container();
    fn SetContainer();
    fn HelpFileName();
    fn SetHelpFileName();
    fn HelpFileContext();
    fn SetHelpFileContext();
    fn Qualifiers();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsClassVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsClassImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsClassVtbl {
        unsafe extern "system" fn PrimaryInterface<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CLSID<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCLSID<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrclsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OID<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOID<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstroid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Abstract<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAbstract<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fabstract: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Auxiliary<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAuxiliary<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fauxiliary: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MandatoryProperties<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMandatoryProperties<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vmandatoryproperties: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OptionalProperties<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOptionalProperties<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, voptionalproperties: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NamingProperties<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNamingProperties<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vnamingproperties: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DerivedFrom<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDerivedFrom<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vderivedfrom: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AuxDerivedFrom<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAuxDerivedFrom<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vauxderivedfrom: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PossibleSuperiors<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPossibleSuperiors<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vpossiblesuperiors: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Containment<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetContainment<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcontainment: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Container<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetContainer<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fcontainer: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HelpFileName<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHelpFileName<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrhelpfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HelpFileContext<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHelpFileContext<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnhelpfilecontext: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Qualifiers<Impl: IADsClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqualifiers: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            Class::<Impl, IMPL_OFFSET>,
            GUID::<Impl, IMPL_OFFSET>,
            ADsPath::<Impl, IMPL_OFFSET>,
            Parent::<Impl, IMPL_OFFSET>,
            Schema::<Impl, IMPL_OFFSET>,
            GetInfo::<Impl, IMPL_OFFSET>,
            SetInfo::<Impl, IMPL_OFFSET>,
            Get::<Impl, IMPL_OFFSET>,
            Put::<Impl, IMPL_OFFSET>,
            GetEx::<Impl, IMPL_OFFSET>,
            PutEx::<Impl, IMPL_OFFSET>,
            GetInfoEx::<Impl, IMPL_OFFSET>,
            PrimaryInterface::<Impl, IMPL_OFFSET>,
            CLSID::<Impl, IMPL_OFFSET>,
            SetCLSID::<Impl, IMPL_OFFSET>,
            OID::<Impl, IMPL_OFFSET>,
            SetOID::<Impl, IMPL_OFFSET>,
            Abstract::<Impl, IMPL_OFFSET>,
            SetAbstract::<Impl, IMPL_OFFSET>,
            Auxiliary::<Impl, IMPL_OFFSET>,
            SetAuxiliary::<Impl, IMPL_OFFSET>,
            MandatoryProperties::<Impl, IMPL_OFFSET>,
            SetMandatoryProperties::<Impl, IMPL_OFFSET>,
            OptionalProperties::<Impl, IMPL_OFFSET>,
            SetOptionalProperties::<Impl, IMPL_OFFSET>,
            NamingProperties::<Impl, IMPL_OFFSET>,
            SetNamingProperties::<Impl, IMPL_OFFSET>,
            DerivedFrom::<Impl, IMPL_OFFSET>,
            SetDerivedFrom::<Impl, IMPL_OFFSET>,
            AuxDerivedFrom::<Impl, IMPL_OFFSET>,
            SetAuxDerivedFrom::<Impl, IMPL_OFFSET>,
            PossibleSuperiors::<Impl, IMPL_OFFSET>,
            SetPossibleSuperiors::<Impl, IMPL_OFFSET>,
            Containment::<Impl, IMPL_OFFSET>,
            SetContainment::<Impl, IMPL_OFFSET>,
            Container::<Impl, IMPL_OFFSET>,
            SetContainer::<Impl, IMPL_OFFSET>,
            HelpFileName::<Impl, IMPL_OFFSET>,
            SetHelpFileName::<Impl, IMPL_OFFSET>,
            HelpFileContext::<Impl, IMPL_OFFSET>,
            SetHelpFileContext::<Impl, IMPL_OFFSET>,
            Qualifiers::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsClass as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsCollectionImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Add();
    fn Remove();
    fn GetObject();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsCollectionVtbl {
        unsafe extern "system" fn _NewEnum<Impl: IADsCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: IADsCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, vitem: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: IADsCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemtoberemoved: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetObject<Impl: IADsCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvitem: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>, Add::<Impl, IMPL_OFFSET>, Remove::<Impl, IMPL_OFFSET>, GetObject::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsComputerImpl: Sized + IADsImpl + IDispatchImpl {
    fn ComputerID();
    fn Site();
    fn Description();
    fn SetDescription();
    fn Location();
    fn SetLocation();
    fn PrimaryUser();
    fn SetPrimaryUser();
    fn Owner();
    fn SetOwner();
    fn Division();
    fn SetDivision();
    fn Department();
    fn SetDepartment();
    fn Role();
    fn SetRole();
    fn OperatingSystem();
    fn SetOperatingSystem();
    fn OperatingSystemVersion();
    fn SetOperatingSystemVersion();
    fn Model();
    fn SetModel();
    fn Processor();
    fn SetProcessor();
    fn ProcessorCount();
    fn SetProcessorCount();
    fn MemorySize();
    fn SetMemorySize();
    fn StorageCapacity();
    fn SetStorageCapacity();
    fn NetAddresses();
    fn SetNetAddresses();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsComputerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsComputerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsComputerVtbl {
        unsafe extern "system" fn ComputerID<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Site<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Description<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDescription<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Location<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLocation<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlocation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PrimaryUser<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPrimaryUser<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprimaryuser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Owner<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOwner<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrowner: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Division<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDivision<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdivision: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Department<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDepartment<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdepartment: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Role<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRole<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OperatingSystem<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOperatingSystem<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstroperatingsystem: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OperatingSystemVersion<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOperatingSystemVersion<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstroperatingsystemversion: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Model<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetModel<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmodel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Processor<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProcessor<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprocessor: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProcessorCount<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProcessorCount<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprocessorcount: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MemorySize<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMemorySize<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmemorysize: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StorageCapacity<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStorageCapacity<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrstoragecapacity: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NetAddresses<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNetAddresses<Impl: IADsComputerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vnetaddresses: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            Class::<Impl, IMPL_OFFSET>,
            GUID::<Impl, IMPL_OFFSET>,
            ADsPath::<Impl, IMPL_OFFSET>,
            Parent::<Impl, IMPL_OFFSET>,
            Schema::<Impl, IMPL_OFFSET>,
            GetInfo::<Impl, IMPL_OFFSET>,
            SetInfo::<Impl, IMPL_OFFSET>,
            Get::<Impl, IMPL_OFFSET>,
            Put::<Impl, IMPL_OFFSET>,
            GetEx::<Impl, IMPL_OFFSET>,
            PutEx::<Impl, IMPL_OFFSET>,
            GetInfoEx::<Impl, IMPL_OFFSET>,
            ComputerID::<Impl, IMPL_OFFSET>,
            Site::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            Location::<Impl, IMPL_OFFSET>,
            SetLocation::<Impl, IMPL_OFFSET>,
            PrimaryUser::<Impl, IMPL_OFFSET>,
            SetPrimaryUser::<Impl, IMPL_OFFSET>,
            Owner::<Impl, IMPL_OFFSET>,
            SetOwner::<Impl, IMPL_OFFSET>,
            Division::<Impl, IMPL_OFFSET>,
            SetDivision::<Impl, IMPL_OFFSET>,
            Department::<Impl, IMPL_OFFSET>,
            SetDepartment::<Impl, IMPL_OFFSET>,
            Role::<Impl, IMPL_OFFSET>,
            SetRole::<Impl, IMPL_OFFSET>,
            OperatingSystem::<Impl, IMPL_OFFSET>,
            SetOperatingSystem::<Impl, IMPL_OFFSET>,
            OperatingSystemVersion::<Impl, IMPL_OFFSET>,
            SetOperatingSystemVersion::<Impl, IMPL_OFFSET>,
            Model::<Impl, IMPL_OFFSET>,
            SetModel::<Impl, IMPL_OFFSET>,
            Processor::<Impl, IMPL_OFFSET>,
            SetProcessor::<Impl, IMPL_OFFSET>,
            ProcessorCount::<Impl, IMPL_OFFSET>,
            SetProcessorCount::<Impl, IMPL_OFFSET>,
            MemorySize::<Impl, IMPL_OFFSET>,
            SetMemorySize::<Impl, IMPL_OFFSET>,
            StorageCapacity::<Impl, IMPL_OFFSET>,
            SetStorageCapacity::<Impl, IMPL_OFFSET>,
            NetAddresses::<Impl, IMPL_OFFSET>,
            SetNetAddresses::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsComputer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsComputerOperationsImpl: Sized + IADsImpl + IDispatchImpl {
    fn Status();
    fn Shutdown();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsComputerOperationsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsComputerOperationsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsComputerOperationsVtbl {
        unsafe extern "system" fn Status<Impl: IADsComputerOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Shutdown<Impl: IADsComputerOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, breboot: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            Class::<Impl, IMPL_OFFSET>,
            GUID::<Impl, IMPL_OFFSET>,
            ADsPath::<Impl, IMPL_OFFSET>,
            Parent::<Impl, IMPL_OFFSET>,
            Schema::<Impl, IMPL_OFFSET>,
            GetInfo::<Impl, IMPL_OFFSET>,
            SetInfo::<Impl, IMPL_OFFSET>,
            Get::<Impl, IMPL_OFFSET>,
            Put::<Impl, IMPL_OFFSET>,
            GetEx::<Impl, IMPL_OFFSET>,
            PutEx::<Impl, IMPL_OFFSET>,
            GetInfoEx::<Impl, IMPL_OFFSET>,
            Status::<Impl, IMPL_OFFSET>,
            Shutdown::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsComputerOperations as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsContainerImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Filter();
    fn SetFilter();
    fn Hints();
    fn SetHints();
    fn GetObject();
    fn Create();
    fn Delete();
    fn CopyHere();
    fn MoveHere();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsContainerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsContainerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsContainerVtbl {
        unsafe extern "system" fn Count<Impl: IADsContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: IADsContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Filter<Impl: IADsContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvar: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFilter<Impl: IADsContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, var: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Hints<Impl: IADsContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvfilter: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHints<Impl: IADsContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vhints: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetObject<Impl: IADsContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, classname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, relativename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Create<Impl: IADsContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, classname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, relativename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: IADsContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrclassname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrrelativename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CopyHere<Impl: IADsContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, newname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MoveHere<Impl: IADsContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, newname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Count::<Impl, IMPL_OFFSET>,
            _NewEnum::<Impl, IMPL_OFFSET>,
            Filter::<Impl, IMPL_OFFSET>,
            SetFilter::<Impl, IMPL_OFFSET>,
            Hints::<Impl, IMPL_OFFSET>,
            SetHints::<Impl, IMPL_OFFSET>,
            GetObject::<Impl, IMPL_OFFSET>,
            Create::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
            CopyHere::<Impl, IMPL_OFFSET>,
            MoveHere::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsContainer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsDNWithBinaryImpl: Sized + IDispatchImpl {
    fn BinaryValue();
    fn SetBinaryValue();
    fn DNString();
    fn SetDNString();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsDNWithBinaryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsDNWithBinaryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsDNWithBinaryVtbl {
        unsafe extern "system" fn BinaryValue<Impl: IADsDNWithBinaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBinaryValue<Impl: IADsDNWithBinaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vbinaryvalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DNString<Impl: IADsDNWithBinaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDNString<Impl: IADsDNWithBinaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdnstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, BinaryValue::<Impl, IMPL_OFFSET>, SetBinaryValue::<Impl, IMPL_OFFSET>, DNString::<Impl, IMPL_OFFSET>, SetDNString::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsDNWithBinary as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsDNWithStringImpl: Sized + IDispatchImpl {
    fn StringValue();
    fn SetStringValue();
    fn DNString();
    fn SetDNString();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsDNWithStringVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsDNWithStringImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsDNWithStringVtbl {
        unsafe extern "system" fn StringValue<Impl: IADsDNWithStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStringValue<Impl: IADsDNWithStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrstringvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DNString<Impl: IADsDNWithStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDNString<Impl: IADsDNWithStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdnstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, StringValue::<Impl, IMPL_OFFSET>, SetStringValue::<Impl, IMPL_OFFSET>, DNString::<Impl, IMPL_OFFSET>, SetDNString::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsDNWithString as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsDeleteOpsImpl: Sized + IDispatchImpl {
    fn DeleteObject();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsDeleteOpsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsDeleteOpsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsDeleteOpsVtbl {
        unsafe extern "system" fn DeleteObject<Impl: IADsDeleteOpsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, DeleteObject::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsDeleteOps as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsDomainImpl: Sized + IADsImpl + IDispatchImpl {
    fn IsWorkgroup();
    fn MinPasswordLength();
    fn SetMinPasswordLength();
    fn MinPasswordAge();
    fn SetMinPasswordAge();
    fn MaxPasswordAge();
    fn SetMaxPasswordAge();
    fn MaxBadPasswordsAllowed();
    fn SetMaxBadPasswordsAllowed();
    fn PasswordHistoryLength();
    fn SetPasswordHistoryLength();
    fn PasswordAttributes();
    fn SetPasswordAttributes();
    fn AutoUnlockInterval();
    fn SetAutoUnlockInterval();
    fn LockoutObservationInterval();
    fn SetLockoutObservationInterval();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsDomainVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsDomainImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsDomainVtbl {
        unsafe extern "system" fn IsWorkgroup<Impl: IADsDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MinPasswordLength<Impl: IADsDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMinPasswordLength<Impl: IADsDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnminpasswordlength: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MinPasswordAge<Impl: IADsDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMinPasswordAge<Impl: IADsDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnminpasswordage: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MaxPasswordAge<Impl: IADsDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMaxPasswordAge<Impl: IADsDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnmaxpasswordage: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MaxBadPasswordsAllowed<Impl: IADsDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMaxBadPasswordsAllowed<Impl: IADsDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnmaxbadpasswordsallowed: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PasswordHistoryLength<Impl: IADsDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPasswordHistoryLength<Impl: IADsDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnpasswordhistorylength: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PasswordAttributes<Impl: IADsDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPasswordAttributes<Impl: IADsDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnpasswordattributes: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AutoUnlockInterval<Impl: IADsDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAutoUnlockInterval<Impl: IADsDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnautounlockinterval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LockoutObservationInterval<Impl: IADsDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLockoutObservationInterval<Impl: IADsDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnlockoutobservationinterval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            Class::<Impl, IMPL_OFFSET>,
            GUID::<Impl, IMPL_OFFSET>,
            ADsPath::<Impl, IMPL_OFFSET>,
            Parent::<Impl, IMPL_OFFSET>,
            Schema::<Impl, IMPL_OFFSET>,
            GetInfo::<Impl, IMPL_OFFSET>,
            SetInfo::<Impl, IMPL_OFFSET>,
            Get::<Impl, IMPL_OFFSET>,
            Put::<Impl, IMPL_OFFSET>,
            GetEx::<Impl, IMPL_OFFSET>,
            PutEx::<Impl, IMPL_OFFSET>,
            GetInfoEx::<Impl, IMPL_OFFSET>,
            IsWorkgroup::<Impl, IMPL_OFFSET>,
            MinPasswordLength::<Impl, IMPL_OFFSET>,
            SetMinPasswordLength::<Impl, IMPL_OFFSET>,
            MinPasswordAge::<Impl, IMPL_OFFSET>,
            SetMinPasswordAge::<Impl, IMPL_OFFSET>,
            MaxPasswordAge::<Impl, IMPL_OFFSET>,
            SetMaxPasswordAge::<Impl, IMPL_OFFSET>,
            MaxBadPasswordsAllowed::<Impl, IMPL_OFFSET>,
            SetMaxBadPasswordsAllowed::<Impl, IMPL_OFFSET>,
            PasswordHistoryLength::<Impl, IMPL_OFFSET>,
            SetPasswordHistoryLength::<Impl, IMPL_OFFSET>,
            PasswordAttributes::<Impl, IMPL_OFFSET>,
            SetPasswordAttributes::<Impl, IMPL_OFFSET>,
            AutoUnlockInterval::<Impl, IMPL_OFFSET>,
            SetAutoUnlockInterval::<Impl, IMPL_OFFSET>,
            LockoutObservationInterval::<Impl, IMPL_OFFSET>,
            SetLockoutObservationInterval::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsDomain as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsEmailImpl: Sized + IDispatchImpl {
    fn Type();
    fn SetType();
    fn Address();
    fn SetAddress();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsEmailVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsEmailImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsEmailVtbl {
        unsafe extern "system" fn Type<Impl: IADsEmailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetType<Impl: IADsEmailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lntype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Address<Impl: IADsEmailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAddress<Impl: IADsEmailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstraddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Type::<Impl, IMPL_OFFSET>, SetType::<Impl, IMPL_OFFSET>, Address::<Impl, IMPL_OFFSET>, SetAddress::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsEmail as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsExtensionImpl: Sized {
    fn Operate();
    fn PrivateGetIDsOfNames();
    fn PrivateInvoke();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsExtensionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsExtensionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsExtensionVtbl {
        unsafe extern "system" fn Operate<Impl: IADsExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcode: u32, vardata1: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, vardata2: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, vardata3: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PrivateGetIDsOfNames<Impl: IADsExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const *const u16, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PrivateInvoke<Impl: IADsExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Operate::<Impl, IMPL_OFFSET>, PrivateGetIDsOfNames::<Impl, IMPL_OFFSET>, PrivateInvoke::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsExtension as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsFaxNumberImpl: Sized + IDispatchImpl {
    fn TelephoneNumber();
    fn SetTelephoneNumber();
    fn Parameters();
    fn SetParameters();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsFaxNumberVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsFaxNumberImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsFaxNumberVtbl {
        unsafe extern "system" fn TelephoneNumber<Impl: IADsFaxNumberImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTelephoneNumber<Impl: IADsFaxNumberImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtelephonenumber: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Parameters<Impl: IADsFaxNumberImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetParameters<Impl: IADsFaxNumberImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vparameters: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, TelephoneNumber::<Impl, IMPL_OFFSET>, SetTelephoneNumber::<Impl, IMPL_OFFSET>, Parameters::<Impl, IMPL_OFFSET>, SetParameters::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsFaxNumber as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsFileServiceImpl: Sized + IADsServiceImpl + IADsImpl + IDispatchImpl {
    fn Description();
    fn SetDescription();
    fn MaxUserCount();
    fn SetMaxUserCount();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsFileServiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsFileServiceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsFileServiceVtbl {
        unsafe extern "system" fn Description<Impl: IADsFileServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDescription<Impl: IADsFileServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MaxUserCount<Impl: IADsFileServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMaxUserCount<Impl: IADsFileServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnmaxusercount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            Class::<Impl, IMPL_OFFSET>,
            GUID::<Impl, IMPL_OFFSET>,
            ADsPath::<Impl, IMPL_OFFSET>,
            Parent::<Impl, IMPL_OFFSET>,
            Schema::<Impl, IMPL_OFFSET>,
            GetInfo::<Impl, IMPL_OFFSET>,
            SetInfo::<Impl, IMPL_OFFSET>,
            Get::<Impl, IMPL_OFFSET>,
            Put::<Impl, IMPL_OFFSET>,
            GetEx::<Impl, IMPL_OFFSET>,
            PutEx::<Impl, IMPL_OFFSET>,
            GetInfoEx::<Impl, IMPL_OFFSET>,
            HostComputer::<Impl, IMPL_OFFSET>,
            SetHostComputer::<Impl, IMPL_OFFSET>,
            DisplayName::<Impl, IMPL_OFFSET>,
            SetDisplayName::<Impl, IMPL_OFFSET>,
            Version::<Impl, IMPL_OFFSET>,
            SetVersion::<Impl, IMPL_OFFSET>,
            ServiceType::<Impl, IMPL_OFFSET>,
            SetServiceType::<Impl, IMPL_OFFSET>,
            StartType::<Impl, IMPL_OFFSET>,
            SetStartType::<Impl, IMPL_OFFSET>,
            Path::<Impl, IMPL_OFFSET>,
            SetPath::<Impl, IMPL_OFFSET>,
            StartupParameters::<Impl, IMPL_OFFSET>,
            SetStartupParameters::<Impl, IMPL_OFFSET>,
            ErrorControl::<Impl, IMPL_OFFSET>,
            SetErrorControl::<Impl, IMPL_OFFSET>,
            LoadOrderGroup::<Impl, IMPL_OFFSET>,
            SetLoadOrderGroup::<Impl, IMPL_OFFSET>,
            ServiceAccountName::<Impl, IMPL_OFFSET>,
            SetServiceAccountName::<Impl, IMPL_OFFSET>,
            ServiceAccountPath::<Impl, IMPL_OFFSET>,
            SetServiceAccountPath::<Impl, IMPL_OFFSET>,
            Dependencies::<Impl, IMPL_OFFSET>,
            SetDependencies::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            MaxUserCount::<Impl, IMPL_OFFSET>,
            SetMaxUserCount::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsFileService as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsFileServiceOperationsImpl: Sized + IADsServiceOperationsImpl + IADsImpl + IDispatchImpl {
    fn Sessions();
    fn Resources();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsFileServiceOperationsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsFileServiceOperationsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsFileServiceOperationsVtbl {
        unsafe extern "system" fn Sessions<Impl: IADsFileServiceOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsessions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Resources<Impl: IADsFileServiceOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            Class::<Impl, IMPL_OFFSET>,
            GUID::<Impl, IMPL_OFFSET>,
            ADsPath::<Impl, IMPL_OFFSET>,
            Parent::<Impl, IMPL_OFFSET>,
            Schema::<Impl, IMPL_OFFSET>,
            GetInfo::<Impl, IMPL_OFFSET>,
            SetInfo::<Impl, IMPL_OFFSET>,
            Get::<Impl, IMPL_OFFSET>,
            Put::<Impl, IMPL_OFFSET>,
            GetEx::<Impl, IMPL_OFFSET>,
            PutEx::<Impl, IMPL_OFFSET>,
            GetInfoEx::<Impl, IMPL_OFFSET>,
            Status::<Impl, IMPL_OFFSET>,
            Start::<Impl, IMPL_OFFSET>,
            Stop::<Impl, IMPL_OFFSET>,
            Pause::<Impl, IMPL_OFFSET>,
            Continue::<Impl, IMPL_OFFSET>,
            SetPassword::<Impl, IMPL_OFFSET>,
            Sessions::<Impl, IMPL_OFFSET>,
            Resources::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsFileServiceOperations as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsFileShareImpl: Sized + IADsImpl + IDispatchImpl {
    fn CurrentUserCount();
    fn Description();
    fn SetDescription();
    fn HostComputer();
    fn SetHostComputer();
    fn Path();
    fn SetPath();
    fn MaxUserCount();
    fn SetMaxUserCount();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsFileShareVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsFileShareImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsFileShareVtbl {
        unsafe extern "system" fn CurrentUserCount<Impl: IADsFileShareImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Description<Impl: IADsFileShareImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDescription<Impl: IADsFileShareImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HostComputer<Impl: IADsFileShareImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHostComputer<Impl: IADsFileShareImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrhostcomputer: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Path<Impl: IADsFileShareImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPath<Impl: IADsFileShareImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MaxUserCount<Impl: IADsFileShareImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMaxUserCount<Impl: IADsFileShareImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnmaxusercount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            Class::<Impl, IMPL_OFFSET>,
            GUID::<Impl, IMPL_OFFSET>,
            ADsPath::<Impl, IMPL_OFFSET>,
            Parent::<Impl, IMPL_OFFSET>,
            Schema::<Impl, IMPL_OFFSET>,
            GetInfo::<Impl, IMPL_OFFSET>,
            SetInfo::<Impl, IMPL_OFFSET>,
            Get::<Impl, IMPL_OFFSET>,
            Put::<Impl, IMPL_OFFSET>,
            GetEx::<Impl, IMPL_OFFSET>,
            PutEx::<Impl, IMPL_OFFSET>,
            GetInfoEx::<Impl, IMPL_OFFSET>,
            CurrentUserCount::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            HostComputer::<Impl, IMPL_OFFSET>,
            SetHostComputer::<Impl, IMPL_OFFSET>,
            Path::<Impl, IMPL_OFFSET>,
            SetPath::<Impl, IMPL_OFFSET>,
            MaxUserCount::<Impl, IMPL_OFFSET>,
            SetMaxUserCount::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsFileShare as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsGroupImpl: Sized + IADsImpl + IDispatchImpl {
    fn Description();
    fn SetDescription();
    fn Members();
    fn IsMember();
    fn Add();
    fn Remove();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsGroupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsGroupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsGroupVtbl {
        unsafe extern "system" fn Description<Impl: IADsGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDescription<Impl: IADsGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Members<Impl: IADsGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmembers: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsMember<Impl: IADsGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmember: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bmember: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: IADsGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrnewitem: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: IADsGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstritemtoberemoved: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            Class::<Impl, IMPL_OFFSET>,
            GUID::<Impl, IMPL_OFFSET>,
            ADsPath::<Impl, IMPL_OFFSET>,
            Parent::<Impl, IMPL_OFFSET>,
            Schema::<Impl, IMPL_OFFSET>,
            GetInfo::<Impl, IMPL_OFFSET>,
            SetInfo::<Impl, IMPL_OFFSET>,
            Get::<Impl, IMPL_OFFSET>,
            Put::<Impl, IMPL_OFFSET>,
            GetEx::<Impl, IMPL_OFFSET>,
            PutEx::<Impl, IMPL_OFFSET>,
            GetInfoEx::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            Members::<Impl, IMPL_OFFSET>,
            IsMember::<Impl, IMPL_OFFSET>,
            Add::<Impl, IMPL_OFFSET>,
            Remove::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsGroup as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsHoldImpl: Sized + IDispatchImpl {
    fn ObjectName();
    fn SetObjectName();
    fn Amount();
    fn SetAmount();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsHoldVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsHoldImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsHoldVtbl {
        unsafe extern "system" fn ObjectName<Impl: IADsHoldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetObjectName<Impl: IADsHoldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrobjectname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Amount<Impl: IADsHoldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAmount<Impl: IADsHoldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnamount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, ObjectName::<Impl, IMPL_OFFSET>, SetObjectName::<Impl, IMPL_OFFSET>, Amount::<Impl, IMPL_OFFSET>, SetAmount::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsHold as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsLargeIntegerImpl: Sized + IDispatchImpl {
    fn HighPart();
    fn SetHighPart();
    fn LowPart();
    fn SetLowPart();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsLargeIntegerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsLargeIntegerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsLargeIntegerVtbl {
        unsafe extern "system" fn HighPart<Impl: IADsLargeIntegerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHighPart<Impl: IADsLargeIntegerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnhighpart: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LowPart<Impl: IADsLargeIntegerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLowPart<Impl: IADsLargeIntegerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnlowpart: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, HighPart::<Impl, IMPL_OFFSET>, SetHighPart::<Impl, IMPL_OFFSET>, LowPart::<Impl, IMPL_OFFSET>, SetLowPart::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsLargeInteger as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsLocalityImpl: Sized + IADsImpl + IDispatchImpl {
    fn Description();
    fn SetDescription();
    fn LocalityName();
    fn SetLocalityName();
    fn PostalAddress();
    fn SetPostalAddress();
    fn SeeAlso();
    fn SetSeeAlso();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsLocalityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsLocalityImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsLocalityVtbl {
        unsafe extern "system" fn Description<Impl: IADsLocalityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDescription<Impl: IADsLocalityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LocalityName<Impl: IADsLocalityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLocalityName<Impl: IADsLocalityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlocalityname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PostalAddress<Impl: IADsLocalityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPostalAddress<Impl: IADsLocalityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpostaladdress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SeeAlso<Impl: IADsLocalityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSeeAlso<Impl: IADsLocalityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vseealso: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            Class::<Impl, IMPL_OFFSET>,
            GUID::<Impl, IMPL_OFFSET>,
            ADsPath::<Impl, IMPL_OFFSET>,
            Parent::<Impl, IMPL_OFFSET>,
            Schema::<Impl, IMPL_OFFSET>,
            GetInfo::<Impl, IMPL_OFFSET>,
            SetInfo::<Impl, IMPL_OFFSET>,
            Get::<Impl, IMPL_OFFSET>,
            Put::<Impl, IMPL_OFFSET>,
            GetEx::<Impl, IMPL_OFFSET>,
            PutEx::<Impl, IMPL_OFFSET>,
            GetInfoEx::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            LocalityName::<Impl, IMPL_OFFSET>,
            SetLocalityName::<Impl, IMPL_OFFSET>,
            PostalAddress::<Impl, IMPL_OFFSET>,
            SetPostalAddress::<Impl, IMPL_OFFSET>,
            SeeAlso::<Impl, IMPL_OFFSET>,
            SetSeeAlso::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsLocality as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsMembersImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Filter();
    fn SetFilter();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsMembersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsMembersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsMembersVtbl {
        unsafe extern "system" fn Count<Impl: IADsMembersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: IADsMembersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Filter<Impl: IADsMembersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvfilter: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFilter<Impl: IADsMembersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvfilter: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>, Filter::<Impl, IMPL_OFFSET>, SetFilter::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsMembers as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsNameTranslateImpl: Sized + IDispatchImpl {
    fn SetChaseReferral();
    fn Init();
    fn InitEx();
    fn Set();
    fn Get();
    fn SetEx();
    fn GetEx();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsNameTranslateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsNameTranslateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsNameTranslateVtbl {
        unsafe extern "system" fn SetChaseReferral<Impl: IADsNameTranslateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnchasereferral: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Init<Impl: IADsNameTranslateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnsettype: i32, bstradspath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitEx<Impl: IADsNameTranslateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnsettype: i32, bstradspath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstruserid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdomain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Set<Impl: IADsNameTranslateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnsettype: i32, bstradspath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Get<Impl: IADsNameTranslateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnformattype: i32, pbstradspath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEx<Impl: IADsNameTranslateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnformattype: i32, pvar: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEx<Impl: IADsNameTranslateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnformattype: i32, pvar: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            SetChaseReferral::<Impl, IMPL_OFFSET>,
            Init::<Impl, IMPL_OFFSET>,
            InitEx::<Impl, IMPL_OFFSET>,
            Set::<Impl, IMPL_OFFSET>,
            Get::<Impl, IMPL_OFFSET>,
            SetEx::<Impl, IMPL_OFFSET>,
            GetEx::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsNameTranslate as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsNamespacesImpl: Sized + IADsImpl + IDispatchImpl {
    fn DefaultContainer();
    fn SetDefaultContainer();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsNamespacesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsNamespacesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsNamespacesVtbl {
        unsafe extern "system" fn DefaultContainer<Impl: IADsNamespacesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDefaultContainer<Impl: IADsNamespacesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdefaultcontainer: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            Class::<Impl, IMPL_OFFSET>,
            GUID::<Impl, IMPL_OFFSET>,
            ADsPath::<Impl, IMPL_OFFSET>,
            Parent::<Impl, IMPL_OFFSET>,
            Schema::<Impl, IMPL_OFFSET>,
            GetInfo::<Impl, IMPL_OFFSET>,
            SetInfo::<Impl, IMPL_OFFSET>,
            Get::<Impl, IMPL_OFFSET>,
            Put::<Impl, IMPL_OFFSET>,
            GetEx::<Impl, IMPL_OFFSET>,
            PutEx::<Impl, IMPL_OFFSET>,
            GetInfoEx::<Impl, IMPL_OFFSET>,
            DefaultContainer::<Impl, IMPL_OFFSET>,
            SetDefaultContainer::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsNamespaces as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsNetAddressImpl: Sized + IDispatchImpl {
    fn AddressType();
    fn SetAddressType();
    fn Address();
    fn SetAddress();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsNetAddressVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsNetAddressImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsNetAddressVtbl {
        unsafe extern "system" fn AddressType<Impl: IADsNetAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAddressType<Impl: IADsNetAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnaddresstype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Address<Impl: IADsNetAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAddress<Impl: IADsNetAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vaddress: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, AddressType::<Impl, IMPL_OFFSET>, SetAddressType::<Impl, IMPL_OFFSET>, Address::<Impl, IMPL_OFFSET>, SetAddress::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsNetAddress as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsOImpl: Sized + IADsImpl + IDispatchImpl {
    fn Description();
    fn SetDescription();
    fn LocalityName();
    fn SetLocalityName();
    fn PostalAddress();
    fn SetPostalAddress();
    fn TelephoneNumber();
    fn SetTelephoneNumber();
    fn FaxNumber();
    fn SetFaxNumber();
    fn SeeAlso();
    fn SetSeeAlso();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsOVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsOImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsOVtbl {
        unsafe extern "system" fn Description<Impl: IADsOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDescription<Impl: IADsOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LocalityName<Impl: IADsOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLocalityName<Impl: IADsOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlocalityname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PostalAddress<Impl: IADsOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPostalAddress<Impl: IADsOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpostaladdress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TelephoneNumber<Impl: IADsOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTelephoneNumber<Impl: IADsOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtelephonenumber: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FaxNumber<Impl: IADsOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFaxNumber<Impl: IADsOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfaxnumber: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SeeAlso<Impl: IADsOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSeeAlso<Impl: IADsOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vseealso: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            Class::<Impl, IMPL_OFFSET>,
            GUID::<Impl, IMPL_OFFSET>,
            ADsPath::<Impl, IMPL_OFFSET>,
            Parent::<Impl, IMPL_OFFSET>,
            Schema::<Impl, IMPL_OFFSET>,
            GetInfo::<Impl, IMPL_OFFSET>,
            SetInfo::<Impl, IMPL_OFFSET>,
            Get::<Impl, IMPL_OFFSET>,
            Put::<Impl, IMPL_OFFSET>,
            GetEx::<Impl, IMPL_OFFSET>,
            PutEx::<Impl, IMPL_OFFSET>,
            GetInfoEx::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            LocalityName::<Impl, IMPL_OFFSET>,
            SetLocalityName::<Impl, IMPL_OFFSET>,
            PostalAddress::<Impl, IMPL_OFFSET>,
            SetPostalAddress::<Impl, IMPL_OFFSET>,
            TelephoneNumber::<Impl, IMPL_OFFSET>,
            SetTelephoneNumber::<Impl, IMPL_OFFSET>,
            FaxNumber::<Impl, IMPL_OFFSET>,
            SetFaxNumber::<Impl, IMPL_OFFSET>,
            SeeAlso::<Impl, IMPL_OFFSET>,
            SetSeeAlso::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsO as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsOUImpl: Sized + IADsImpl + IDispatchImpl {
    fn Description();
    fn SetDescription();
    fn LocalityName();
    fn SetLocalityName();
    fn PostalAddress();
    fn SetPostalAddress();
    fn TelephoneNumber();
    fn SetTelephoneNumber();
    fn FaxNumber();
    fn SetFaxNumber();
    fn SeeAlso();
    fn SetSeeAlso();
    fn BusinessCategory();
    fn SetBusinessCategory();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsOUVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsOUImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsOUVtbl {
        unsafe extern "system" fn Description<Impl: IADsOUImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDescription<Impl: IADsOUImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LocalityName<Impl: IADsOUImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLocalityName<Impl: IADsOUImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlocalityname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PostalAddress<Impl: IADsOUImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPostalAddress<Impl: IADsOUImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpostaladdress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TelephoneNumber<Impl: IADsOUImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTelephoneNumber<Impl: IADsOUImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtelephonenumber: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FaxNumber<Impl: IADsOUImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFaxNumber<Impl: IADsOUImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfaxnumber: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SeeAlso<Impl: IADsOUImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSeeAlso<Impl: IADsOUImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vseealso: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BusinessCategory<Impl: IADsOUImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBusinessCategory<Impl: IADsOUImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbusinesscategory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            Class::<Impl, IMPL_OFFSET>,
            GUID::<Impl, IMPL_OFFSET>,
            ADsPath::<Impl, IMPL_OFFSET>,
            Parent::<Impl, IMPL_OFFSET>,
            Schema::<Impl, IMPL_OFFSET>,
            GetInfo::<Impl, IMPL_OFFSET>,
            SetInfo::<Impl, IMPL_OFFSET>,
            Get::<Impl, IMPL_OFFSET>,
            Put::<Impl, IMPL_OFFSET>,
            GetEx::<Impl, IMPL_OFFSET>,
            PutEx::<Impl, IMPL_OFFSET>,
            GetInfoEx::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            LocalityName::<Impl, IMPL_OFFSET>,
            SetLocalityName::<Impl, IMPL_OFFSET>,
            PostalAddress::<Impl, IMPL_OFFSET>,
            SetPostalAddress::<Impl, IMPL_OFFSET>,
            TelephoneNumber::<Impl, IMPL_OFFSET>,
            SetTelephoneNumber::<Impl, IMPL_OFFSET>,
            FaxNumber::<Impl, IMPL_OFFSET>,
            SetFaxNumber::<Impl, IMPL_OFFSET>,
            SeeAlso::<Impl, IMPL_OFFSET>,
            SetSeeAlso::<Impl, IMPL_OFFSET>,
            BusinessCategory::<Impl, IMPL_OFFSET>,
            SetBusinessCategory::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsOU as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsObjectOptionsImpl: Sized + IDispatchImpl {
    fn GetOption();
    fn SetOption();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsObjectOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsObjectOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsObjectOptionsVtbl {
        unsafe extern "system" fn GetOption<Impl: IADsObjectOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnoption: i32, pvvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOption<Impl: IADsObjectOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnoption: i32, vvalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, GetOption::<Impl, IMPL_OFFSET>, SetOption::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsObjectOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsOctetListImpl: Sized + IDispatchImpl {
    fn OctetList();
    fn SetOctetList();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsOctetListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsOctetListImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsOctetListVtbl {
        unsafe extern "system" fn OctetList<Impl: IADsOctetListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOctetList<Impl: IADsOctetListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, voctetlist: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, OctetList::<Impl, IMPL_OFFSET>, SetOctetList::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsOctetList as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsOpenDSObjectImpl: Sized + IDispatchImpl {
    fn OpenDSObject();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsOpenDSObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsOpenDSObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsOpenDSObjectVtbl {
        unsafe extern "system" fn OpenDSObject<Impl: IADsOpenDSObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszdnname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lpszusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lpszpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lnreserved: i32, ppoledsobj: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, OpenDSObject::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsOpenDSObject as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsPathImpl: Sized + IDispatchImpl {
    fn Type();
    fn SetType();
    fn VolumeName();
    fn SetVolumeName();
    fn Path();
    fn SetPath();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsPathVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsPathImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsPathVtbl {
        unsafe extern "system" fn Type<Impl: IADsPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetType<Impl: IADsPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lntype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VolumeName<Impl: IADsPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetVolumeName<Impl: IADsPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrvolumename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Path<Impl: IADsPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPath<Impl: IADsPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Type::<Impl, IMPL_OFFSET>, SetType::<Impl, IMPL_OFFSET>, VolumeName::<Impl, IMPL_OFFSET>, SetVolumeName::<Impl, IMPL_OFFSET>, Path::<Impl, IMPL_OFFSET>, SetPath::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsPath as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsPathnameImpl: Sized + IDispatchImpl {
    fn Set();
    fn SetDisplayType();
    fn Retrieve();
    fn GetNumElements();
    fn GetElement();
    fn AddLeafElement();
    fn RemoveLeafElement();
    fn CopyPath();
    fn GetEscapedElement();
    fn EscapedMode();
    fn SetEscapedMode();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsPathnameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsPathnameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsPathnameVtbl {
        unsafe extern "system" fn Set<Impl: IADsPathnameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradspath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lnsettype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDisplayType<Impl: IADsPathnameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lndisplaytype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Retrieve<Impl: IADsPathnameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnformattype: i32, pbstradspath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNumElements<Impl: IADsPathnameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plnnumpathelements: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetElement<Impl: IADsPathnameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnelementindex: i32, pbstrelement: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddLeafElement<Impl: IADsPathnameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrleafelement: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveLeafElement<Impl: IADsPathnameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CopyPath<Impl: IADsPathnameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppadspath: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEscapedElement<Impl: IADsPathnameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnreserved: i32, bstrinstr: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstroutstr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EscapedMode<Impl: IADsPathnameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEscapedMode<Impl: IADsPathnameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnescapedmode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Set::<Impl, IMPL_OFFSET>,
            SetDisplayType::<Impl, IMPL_OFFSET>,
            Retrieve::<Impl, IMPL_OFFSET>,
            GetNumElements::<Impl, IMPL_OFFSET>,
            GetElement::<Impl, IMPL_OFFSET>,
            AddLeafElement::<Impl, IMPL_OFFSET>,
            RemoveLeafElement::<Impl, IMPL_OFFSET>,
            CopyPath::<Impl, IMPL_OFFSET>,
            GetEscapedElement::<Impl, IMPL_OFFSET>,
            EscapedMode::<Impl, IMPL_OFFSET>,
            SetEscapedMode::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsPathname as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsPostalAddressImpl: Sized + IDispatchImpl {
    fn PostalAddress();
    fn SetPostalAddress();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsPostalAddressVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsPostalAddressImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsPostalAddressVtbl {
        unsafe extern "system" fn PostalAddress<Impl: IADsPostalAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPostalAddress<Impl: IADsPostalAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vpostaladdress: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, PostalAddress::<Impl, IMPL_OFFSET>, SetPostalAddress::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsPostalAddress as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsPrintJobImpl: Sized + IADsImpl + IDispatchImpl {
    fn HostPrintQueue();
    fn User();
    fn UserPath();
    fn TimeSubmitted();
    fn TotalPages();
    fn Size();
    fn Description();
    fn SetDescription();
    fn Priority();
    fn SetPriority();
    fn StartTime();
    fn SetStartTime();
    fn UntilTime();
    fn SetUntilTime();
    fn Notify();
    fn SetNotify();
    fn NotifyPath();
    fn SetNotifyPath();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsPrintJobVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintJobImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsPrintJobVtbl {
        unsafe extern "system" fn HostPrintQueue<Impl: IADsPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn User<Impl: IADsPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UserPath<Impl: IADsPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TimeSubmitted<Impl: IADsPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TotalPages<Impl: IADsPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Size<Impl: IADsPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Description<Impl: IADsPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDescription<Impl: IADsPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Priority<Impl: IADsPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPriority<Impl: IADsPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnpriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StartTime<Impl: IADsPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStartTime<Impl: IADsPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dastarttime: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UntilTime<Impl: IADsPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUntilTime<Impl: IADsPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dauntiltime: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Notify<Impl: IADsPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNotify<Impl: IADsPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrnotify: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NotifyPath<Impl: IADsPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNotifyPath<Impl: IADsPrintJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrnotifypath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            Class::<Impl, IMPL_OFFSET>,
            GUID::<Impl, IMPL_OFFSET>,
            ADsPath::<Impl, IMPL_OFFSET>,
            Parent::<Impl, IMPL_OFFSET>,
            Schema::<Impl, IMPL_OFFSET>,
            GetInfo::<Impl, IMPL_OFFSET>,
            SetInfo::<Impl, IMPL_OFFSET>,
            Get::<Impl, IMPL_OFFSET>,
            Put::<Impl, IMPL_OFFSET>,
            GetEx::<Impl, IMPL_OFFSET>,
            PutEx::<Impl, IMPL_OFFSET>,
            GetInfoEx::<Impl, IMPL_OFFSET>,
            HostPrintQueue::<Impl, IMPL_OFFSET>,
            User::<Impl, IMPL_OFFSET>,
            UserPath::<Impl, IMPL_OFFSET>,
            TimeSubmitted::<Impl, IMPL_OFFSET>,
            TotalPages::<Impl, IMPL_OFFSET>,
            Size::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            Priority::<Impl, IMPL_OFFSET>,
            SetPriority::<Impl, IMPL_OFFSET>,
            StartTime::<Impl, IMPL_OFFSET>,
            SetStartTime::<Impl, IMPL_OFFSET>,
            UntilTime::<Impl, IMPL_OFFSET>,
            SetUntilTime::<Impl, IMPL_OFFSET>,
            Notify::<Impl, IMPL_OFFSET>,
            SetNotify::<Impl, IMPL_OFFSET>,
            NotifyPath::<Impl, IMPL_OFFSET>,
            SetNotifyPath::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsPrintJob as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsPrintJobOperationsImpl: Sized + IADsImpl + IDispatchImpl {
    fn Status();
    fn TimeElapsed();
    fn PagesPrinted();
    fn Position();
    fn SetPosition();
    fn Pause();
    fn Resume();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsPrintJobOperationsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintJobOperationsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsPrintJobOperationsVtbl {
        unsafe extern "system" fn Status<Impl: IADsPrintJobOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TimeElapsed<Impl: IADsPrintJobOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PagesPrinted<Impl: IADsPrintJobOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Position<Impl: IADsPrintJobOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPosition<Impl: IADsPrintJobOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnposition: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Pause<Impl: IADsPrintJobOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Resume<Impl: IADsPrintJobOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            Class::<Impl, IMPL_OFFSET>,
            GUID::<Impl, IMPL_OFFSET>,
            ADsPath::<Impl, IMPL_OFFSET>,
            Parent::<Impl, IMPL_OFFSET>,
            Schema::<Impl, IMPL_OFFSET>,
            GetInfo::<Impl, IMPL_OFFSET>,
            SetInfo::<Impl, IMPL_OFFSET>,
            Get::<Impl, IMPL_OFFSET>,
            Put::<Impl, IMPL_OFFSET>,
            GetEx::<Impl, IMPL_OFFSET>,
            PutEx::<Impl, IMPL_OFFSET>,
            GetInfoEx::<Impl, IMPL_OFFSET>,
            Status::<Impl, IMPL_OFFSET>,
            TimeElapsed::<Impl, IMPL_OFFSET>,
            PagesPrinted::<Impl, IMPL_OFFSET>,
            Position::<Impl, IMPL_OFFSET>,
            SetPosition::<Impl, IMPL_OFFSET>,
            Pause::<Impl, IMPL_OFFSET>,
            Resume::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsPrintJobOperations as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsPrintQueueImpl: Sized + IADsImpl + IDispatchImpl {
    fn PrinterPath();
    fn SetPrinterPath();
    fn Model();
    fn SetModel();
    fn Datatype();
    fn SetDatatype();
    fn PrintProcessor();
    fn SetPrintProcessor();
    fn Description();
    fn SetDescription();
    fn Location();
    fn SetLocation();
    fn StartTime();
    fn SetStartTime();
    fn UntilTime();
    fn SetUntilTime();
    fn DefaultJobPriority();
    fn SetDefaultJobPriority();
    fn Priority();
    fn SetPriority();
    fn BannerPage();
    fn SetBannerPage();
    fn PrintDevices();
    fn SetPrintDevices();
    fn NetAddresses();
    fn SetNetAddresses();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsPrintQueueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintQueueImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsPrintQueueVtbl {
        unsafe extern "system" fn PrinterPath<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPrinterPath<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprinterpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Model<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetModel<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmodel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Datatype<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDatatype<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdatatype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PrintProcessor<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPrintProcessor<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprintprocessor: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Description<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDescription<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Location<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLocation<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlocation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StartTime<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStartTime<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dastarttime: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UntilTime<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUntilTime<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dauntiltime: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DefaultJobPriority<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDefaultJobPriority<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lndefaultjobpriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Priority<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPriority<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnpriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BannerPage<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBannerPage<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbannerpage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PrintDevices<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPrintDevices<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vprintdevices: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NetAddresses<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNetAddresses<Impl: IADsPrintQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vnetaddresses: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            Class::<Impl, IMPL_OFFSET>,
            GUID::<Impl, IMPL_OFFSET>,
            ADsPath::<Impl, IMPL_OFFSET>,
            Parent::<Impl, IMPL_OFFSET>,
            Schema::<Impl, IMPL_OFFSET>,
            GetInfo::<Impl, IMPL_OFFSET>,
            SetInfo::<Impl, IMPL_OFFSET>,
            Get::<Impl, IMPL_OFFSET>,
            Put::<Impl, IMPL_OFFSET>,
            GetEx::<Impl, IMPL_OFFSET>,
            PutEx::<Impl, IMPL_OFFSET>,
            GetInfoEx::<Impl, IMPL_OFFSET>,
            PrinterPath::<Impl, IMPL_OFFSET>,
            SetPrinterPath::<Impl, IMPL_OFFSET>,
            Model::<Impl, IMPL_OFFSET>,
            SetModel::<Impl, IMPL_OFFSET>,
            Datatype::<Impl, IMPL_OFFSET>,
            SetDatatype::<Impl, IMPL_OFFSET>,
            PrintProcessor::<Impl, IMPL_OFFSET>,
            SetPrintProcessor::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            Location::<Impl, IMPL_OFFSET>,
            SetLocation::<Impl, IMPL_OFFSET>,
            StartTime::<Impl, IMPL_OFFSET>,
            SetStartTime::<Impl, IMPL_OFFSET>,
            UntilTime::<Impl, IMPL_OFFSET>,
            SetUntilTime::<Impl, IMPL_OFFSET>,
            DefaultJobPriority::<Impl, IMPL_OFFSET>,
            SetDefaultJobPriority::<Impl, IMPL_OFFSET>,
            Priority::<Impl, IMPL_OFFSET>,
            SetPriority::<Impl, IMPL_OFFSET>,
            BannerPage::<Impl, IMPL_OFFSET>,
            SetBannerPage::<Impl, IMPL_OFFSET>,
            PrintDevices::<Impl, IMPL_OFFSET>,
            SetPrintDevices::<Impl, IMPL_OFFSET>,
            NetAddresses::<Impl, IMPL_OFFSET>,
            SetNetAddresses::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsPrintQueue as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsPrintQueueOperationsImpl: Sized + IADsImpl + IDispatchImpl {
    fn Status();
    fn PrintJobs();
    fn Pause();
    fn Resume();
    fn Purge();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsPrintQueueOperationsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsPrintQueueOperationsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsPrintQueueOperationsVtbl {
        unsafe extern "system" fn Status<Impl: IADsPrintQueueOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PrintJobs<Impl: IADsPrintQueueOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Pause<Impl: IADsPrintQueueOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Resume<Impl: IADsPrintQueueOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Purge<Impl: IADsPrintQueueOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            Class::<Impl, IMPL_OFFSET>,
            GUID::<Impl, IMPL_OFFSET>,
            ADsPath::<Impl, IMPL_OFFSET>,
            Parent::<Impl, IMPL_OFFSET>,
            Schema::<Impl, IMPL_OFFSET>,
            GetInfo::<Impl, IMPL_OFFSET>,
            SetInfo::<Impl, IMPL_OFFSET>,
            Get::<Impl, IMPL_OFFSET>,
            Put::<Impl, IMPL_OFFSET>,
            GetEx::<Impl, IMPL_OFFSET>,
            PutEx::<Impl, IMPL_OFFSET>,
            GetInfoEx::<Impl, IMPL_OFFSET>,
            Status::<Impl, IMPL_OFFSET>,
            PrintJobs::<Impl, IMPL_OFFSET>,
            Pause::<Impl, IMPL_OFFSET>,
            Resume::<Impl, IMPL_OFFSET>,
            Purge::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsPrintQueueOperations as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsPropertyImpl: Sized + IADsImpl + IDispatchImpl {
    fn OID();
    fn SetOID();
    fn Syntax();
    fn SetSyntax();
    fn MaxRange();
    fn SetMaxRange();
    fn MinRange();
    fn SetMinRange();
    fn MultiValued();
    fn SetMultiValued();
    fn Qualifiers();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsPropertyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsPropertyVtbl {
        unsafe extern "system" fn OID<Impl: IADsPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOID<Impl: IADsPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstroid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Syntax<Impl: IADsPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSyntax<Impl: IADsPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsyntax: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MaxRange<Impl: IADsPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMaxRange<Impl: IADsPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnmaxrange: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MinRange<Impl: IADsPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMinRange<Impl: IADsPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnminrange: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MultiValued<Impl: IADsPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMultiValued<Impl: IADsPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmultivalued: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Qualifiers<Impl: IADsPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqualifiers: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            Class::<Impl, IMPL_OFFSET>,
            GUID::<Impl, IMPL_OFFSET>,
            ADsPath::<Impl, IMPL_OFFSET>,
            Parent::<Impl, IMPL_OFFSET>,
            Schema::<Impl, IMPL_OFFSET>,
            GetInfo::<Impl, IMPL_OFFSET>,
            SetInfo::<Impl, IMPL_OFFSET>,
            Get::<Impl, IMPL_OFFSET>,
            Put::<Impl, IMPL_OFFSET>,
            GetEx::<Impl, IMPL_OFFSET>,
            PutEx::<Impl, IMPL_OFFSET>,
            GetInfoEx::<Impl, IMPL_OFFSET>,
            OID::<Impl, IMPL_OFFSET>,
            SetOID::<Impl, IMPL_OFFSET>,
            Syntax::<Impl, IMPL_OFFSET>,
            SetSyntax::<Impl, IMPL_OFFSET>,
            MaxRange::<Impl, IMPL_OFFSET>,
            SetMaxRange::<Impl, IMPL_OFFSET>,
            MinRange::<Impl, IMPL_OFFSET>,
            SetMinRange::<Impl, IMPL_OFFSET>,
            MultiValued::<Impl, IMPL_OFFSET>,
            SetMultiValued::<Impl, IMPL_OFFSET>,
            Qualifiers::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsProperty as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsPropertyEntryImpl: Sized + IDispatchImpl {
    fn Clear();
    fn Name();
    fn SetName();
    fn ADsType();
    fn SetADsType();
    fn ControlCode();
    fn SetControlCode();
    fn Values();
    fn SetValues();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsPropertyEntryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyEntryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsPropertyEntryVtbl {
        unsafe extern "system" fn Clear<Impl: IADsPropertyEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Name<Impl: IADsPropertyEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetName<Impl: IADsPropertyEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ADsType<Impl: IADsPropertyEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetADsType<Impl: IADsPropertyEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnadstype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ControlCode<Impl: IADsPropertyEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetControlCode<Impl: IADsPropertyEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lncontrolcode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Values<Impl: IADsPropertyEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetValues<Impl: IADsPropertyEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vvalues: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Clear::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            SetName::<Impl, IMPL_OFFSET>,
            ADsType::<Impl, IMPL_OFFSET>,
            SetADsType::<Impl, IMPL_OFFSET>,
            ControlCode::<Impl, IMPL_OFFSET>,
            SetControlCode::<Impl, IMPL_OFFSET>,
            Values::<Impl, IMPL_OFFSET>,
            SetValues::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsPropertyEntry as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsPropertyListImpl: Sized + IDispatchImpl {
    fn PropertyCount();
    fn Next();
    fn Skip();
    fn Reset();
    fn Item();
    fn GetPropertyItem();
    fn PutPropertyItem();
    fn ResetPropertyItem();
    fn PurgePropertyList();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsPropertyListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyListImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsPropertyListVtbl {
        unsafe extern "system" fn PropertyCount<Impl: IADsPropertyListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Next<Impl: IADsPropertyListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IADsPropertyListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celements: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IADsPropertyListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: IADsPropertyListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropertyItem<Impl: IADsPropertyListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lnadstype: i32, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PutPropertyItem<Impl: IADsPropertyListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vardata: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResetPropertyItem<Impl: IADsPropertyListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varentry: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PurgePropertyList<Impl: IADsPropertyListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            PropertyCount::<Impl, IMPL_OFFSET>,
            Next::<Impl, IMPL_OFFSET>,
            Skip::<Impl, IMPL_OFFSET>,
            Reset::<Impl, IMPL_OFFSET>,
            Item::<Impl, IMPL_OFFSET>,
            GetPropertyItem::<Impl, IMPL_OFFSET>,
            PutPropertyItem::<Impl, IMPL_OFFSET>,
            ResetPropertyItem::<Impl, IMPL_OFFSET>,
            PurgePropertyList::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsPropertyList as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsPropertyValueImpl: Sized + IDispatchImpl {
    fn Clear();
    fn ADsType();
    fn SetADsType();
    fn DNString();
    fn SetDNString();
    fn CaseExactString();
    fn SetCaseExactString();
    fn CaseIgnoreString();
    fn SetCaseIgnoreString();
    fn PrintableString();
    fn SetPrintableString();
    fn NumericString();
    fn SetNumericString();
    fn Boolean();
    fn SetBoolean();
    fn Integer();
    fn SetInteger();
    fn OctetString();
    fn SetOctetString();
    fn SecurityDescriptor();
    fn SetSecurityDescriptor();
    fn LargeInteger();
    fn SetLargeInteger();
    fn UTCTime();
    fn SetUTCTime();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsPropertyValueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyValueImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsPropertyValueVtbl {
        unsafe extern "system" fn Clear<Impl: IADsPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ADsType<Impl: IADsPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetADsType<Impl: IADsPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnadstype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DNString<Impl: IADsPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDNString<Impl: IADsPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdnstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CaseExactString<Impl: IADsPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCaseExactString<Impl: IADsPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcaseexactstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CaseIgnoreString<Impl: IADsPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCaseIgnoreString<Impl: IADsPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcaseignorestring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PrintableString<Impl: IADsPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPrintableString<Impl: IADsPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprintablestring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NumericString<Impl: IADsPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNumericString<Impl: IADsPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrnumericstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Boolean<Impl: IADsPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBoolean<Impl: IADsPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnboolean: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Integer<Impl: IADsPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInteger<Impl: IADsPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lninteger: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OctetString<Impl: IADsPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOctetString<Impl: IADsPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, voctetstring: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SecurityDescriptor<Impl: IADsPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSecurityDescriptor<Impl: IADsPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psecuritydescriptor: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LargeInteger<Impl: IADsPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLargeInteger<Impl: IADsPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plargeinteger: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UTCTime<Impl: IADsPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUTCTime<Impl: IADsPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dautctime: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Clear::<Impl, IMPL_OFFSET>,
            ADsType::<Impl, IMPL_OFFSET>,
            SetADsType::<Impl, IMPL_OFFSET>,
            DNString::<Impl, IMPL_OFFSET>,
            SetDNString::<Impl, IMPL_OFFSET>,
            CaseExactString::<Impl, IMPL_OFFSET>,
            SetCaseExactString::<Impl, IMPL_OFFSET>,
            CaseIgnoreString::<Impl, IMPL_OFFSET>,
            SetCaseIgnoreString::<Impl, IMPL_OFFSET>,
            PrintableString::<Impl, IMPL_OFFSET>,
            SetPrintableString::<Impl, IMPL_OFFSET>,
            NumericString::<Impl, IMPL_OFFSET>,
            SetNumericString::<Impl, IMPL_OFFSET>,
            Boolean::<Impl, IMPL_OFFSET>,
            SetBoolean::<Impl, IMPL_OFFSET>,
            Integer::<Impl, IMPL_OFFSET>,
            SetInteger::<Impl, IMPL_OFFSET>,
            OctetString::<Impl, IMPL_OFFSET>,
            SetOctetString::<Impl, IMPL_OFFSET>,
            SecurityDescriptor::<Impl, IMPL_OFFSET>,
            SetSecurityDescriptor::<Impl, IMPL_OFFSET>,
            LargeInteger::<Impl, IMPL_OFFSET>,
            SetLargeInteger::<Impl, IMPL_OFFSET>,
            UTCTime::<Impl, IMPL_OFFSET>,
            SetUTCTime::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsPropertyValue as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsPropertyValue2Impl: Sized + IDispatchImpl {
    fn GetObjectProperty();
    fn PutObjectProperty();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsPropertyValue2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsPropertyValue2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsPropertyValue2Vtbl {
        unsafe extern "system" fn GetObjectProperty<Impl: IADsPropertyValue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnadstype: *mut i32, pvprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PutObjectProperty<Impl: IADsPropertyValue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnadstype: i32, vprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, GetObjectProperty::<Impl, IMPL_OFFSET>, PutObjectProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsPropertyValue2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsReplicaPointerImpl: Sized + IDispatchImpl {
    fn ServerName();
    fn SetServerName();
    fn ReplicaType();
    fn SetReplicaType();
    fn ReplicaNumber();
    fn SetReplicaNumber();
    fn Count();
    fn SetCount();
    fn ReplicaAddressHints();
    fn SetReplicaAddressHints();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsReplicaPointerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsReplicaPointerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsReplicaPointerVtbl {
        unsafe extern "system" fn ServerName<Impl: IADsReplicaPointerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetServerName<Impl: IADsReplicaPointerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrservername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReplicaType<Impl: IADsReplicaPointerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetReplicaType<Impl: IADsReplicaPointerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnreplicatype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReplicaNumber<Impl: IADsReplicaPointerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetReplicaNumber<Impl: IADsReplicaPointerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnreplicanumber: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: IADsReplicaPointerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCount<Impl: IADsReplicaPointerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lncount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReplicaAddressHints<Impl: IADsReplicaPointerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetReplicaAddressHints<Impl: IADsReplicaPointerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vreplicaaddresshints: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            ServerName::<Impl, IMPL_OFFSET>,
            SetServerName::<Impl, IMPL_OFFSET>,
            ReplicaType::<Impl, IMPL_OFFSET>,
            SetReplicaType::<Impl, IMPL_OFFSET>,
            ReplicaNumber::<Impl, IMPL_OFFSET>,
            SetReplicaNumber::<Impl, IMPL_OFFSET>,
            Count::<Impl, IMPL_OFFSET>,
            SetCount::<Impl, IMPL_OFFSET>,
            ReplicaAddressHints::<Impl, IMPL_OFFSET>,
            SetReplicaAddressHints::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsReplicaPointer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsResourceImpl: Sized + IADsImpl + IDispatchImpl {
    fn User();
    fn UserPath();
    fn Path();
    fn LockCount();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsResourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsResourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsResourceVtbl {
        unsafe extern "system" fn User<Impl: IADsResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UserPath<Impl: IADsResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Path<Impl: IADsResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LockCount<Impl: IADsResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            Class::<Impl, IMPL_OFFSET>,
            GUID::<Impl, IMPL_OFFSET>,
            ADsPath::<Impl, IMPL_OFFSET>,
            Parent::<Impl, IMPL_OFFSET>,
            Schema::<Impl, IMPL_OFFSET>,
            GetInfo::<Impl, IMPL_OFFSET>,
            SetInfo::<Impl, IMPL_OFFSET>,
            Get::<Impl, IMPL_OFFSET>,
            Put::<Impl, IMPL_OFFSET>,
            GetEx::<Impl, IMPL_OFFSET>,
            PutEx::<Impl, IMPL_OFFSET>,
            GetInfoEx::<Impl, IMPL_OFFSET>,
            User::<Impl, IMPL_OFFSET>,
            UserPath::<Impl, IMPL_OFFSET>,
            Path::<Impl, IMPL_OFFSET>,
            LockCount::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsResource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsSecurityDescriptorImpl: Sized + IDispatchImpl {
    fn Revision();
    fn SetRevision();
    fn Control();
    fn SetControl();
    fn Owner();
    fn SetOwner();
    fn OwnerDefaulted();
    fn SetOwnerDefaulted();
    fn Group();
    fn SetGroup();
    fn GroupDefaulted();
    fn SetGroupDefaulted();
    fn DiscretionaryAcl();
    fn SetDiscretionaryAcl();
    fn DaclDefaulted();
    fn SetDaclDefaulted();
    fn SystemAcl();
    fn SetSystemAcl();
    fn SaclDefaulted();
    fn SetSaclDefaulted();
    fn CopySecurityDescriptor();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsSecurityDescriptorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsSecurityDescriptorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsSecurityDescriptorVtbl {
        unsafe extern "system" fn Revision<Impl: IADsSecurityDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRevision<Impl: IADsSecurityDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnrevision: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Control<Impl: IADsSecurityDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetControl<Impl: IADsSecurityDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lncontrol: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Owner<Impl: IADsSecurityDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOwner<Impl: IADsSecurityDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrowner: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OwnerDefaulted<Impl: IADsSecurityDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOwnerDefaulted<Impl: IADsSecurityDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fownerdefaulted: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Group<Impl: IADsSecurityDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetGroup<Impl: IADsSecurityDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroup: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GroupDefaulted<Impl: IADsSecurityDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetGroupDefaulted<Impl: IADsSecurityDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fgroupdefaulted: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DiscretionaryAcl<Impl: IADsSecurityDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDiscretionaryAcl<Impl: IADsSecurityDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdiscretionaryacl: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DaclDefaulted<Impl: IADsSecurityDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDaclDefaulted<Impl: IADsSecurityDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fdacldefaulted: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SystemAcl<Impl: IADsSecurityDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSystemAcl<Impl: IADsSecurityDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psystemacl: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SaclDefaulted<Impl: IADsSecurityDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSaclDefaulted<Impl: IADsSecurityDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fsacldefaulted: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CopySecurityDescriptor<Impl: IADsSecurityDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsecuritydescriptor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Revision::<Impl, IMPL_OFFSET>,
            SetRevision::<Impl, IMPL_OFFSET>,
            Control::<Impl, IMPL_OFFSET>,
            SetControl::<Impl, IMPL_OFFSET>,
            Owner::<Impl, IMPL_OFFSET>,
            SetOwner::<Impl, IMPL_OFFSET>,
            OwnerDefaulted::<Impl, IMPL_OFFSET>,
            SetOwnerDefaulted::<Impl, IMPL_OFFSET>,
            Group::<Impl, IMPL_OFFSET>,
            SetGroup::<Impl, IMPL_OFFSET>,
            GroupDefaulted::<Impl, IMPL_OFFSET>,
            SetGroupDefaulted::<Impl, IMPL_OFFSET>,
            DiscretionaryAcl::<Impl, IMPL_OFFSET>,
            SetDiscretionaryAcl::<Impl, IMPL_OFFSET>,
            DaclDefaulted::<Impl, IMPL_OFFSET>,
            SetDaclDefaulted::<Impl, IMPL_OFFSET>,
            SystemAcl::<Impl, IMPL_OFFSET>,
            SetSystemAcl::<Impl, IMPL_OFFSET>,
            SaclDefaulted::<Impl, IMPL_OFFSET>,
            SetSaclDefaulted::<Impl, IMPL_OFFSET>,
            CopySecurityDescriptor::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsSecurityDescriptor as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsSecurityUtilityImpl: Sized + IDispatchImpl {
    fn GetSecurityDescriptor();
    fn SetSecurityDescriptor();
    fn ConvertSecurityDescriptor();
    fn SecurityMask();
    fn SetSecurityMask();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsSecurityUtilityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsSecurityUtilityImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsSecurityUtilityVtbl {
        unsafe extern "system" fn GetSecurityDescriptor<Impl: IADsSecurityUtilityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varpath: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, lpathformat: i32, lformat: i32, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSecurityDescriptor<Impl: IADsSecurityUtilityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varpath: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, lpathformat: i32, vardata: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ldataformat: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConvertSecurityDescriptor<Impl: IADsSecurityUtilityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsd: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ldataformat: i32, loutformat: i32, presult: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SecurityMask<Impl: IADsSecurityUtilityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSecurityMask<Impl: IADsSecurityUtilityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnsecuritymask: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            GetSecurityDescriptor::<Impl, IMPL_OFFSET>,
            SetSecurityDescriptor::<Impl, IMPL_OFFSET>,
            ConvertSecurityDescriptor::<Impl, IMPL_OFFSET>,
            SecurityMask::<Impl, IMPL_OFFSET>,
            SetSecurityMask::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsSecurityUtility as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsServiceImpl: Sized + IADsImpl + IDispatchImpl {
    fn HostComputer();
    fn SetHostComputer();
    fn DisplayName();
    fn SetDisplayName();
    fn Version();
    fn SetVersion();
    fn ServiceType();
    fn SetServiceType();
    fn StartType();
    fn SetStartType();
    fn Path();
    fn SetPath();
    fn StartupParameters();
    fn SetStartupParameters();
    fn ErrorControl();
    fn SetErrorControl();
    fn LoadOrderGroup();
    fn SetLoadOrderGroup();
    fn ServiceAccountName();
    fn SetServiceAccountName();
    fn ServiceAccountPath();
    fn SetServiceAccountPath();
    fn Dependencies();
    fn SetDependencies();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsServiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsServiceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsServiceVtbl {
        unsafe extern "system" fn HostComputer<Impl: IADsServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHostComputer<Impl: IADsServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrhostcomputer: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisplayName<Impl: IADsServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDisplayName<Impl: IADsServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdisplayname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Version<Impl: IADsServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetVersion<Impl: IADsServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrversion: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ServiceType<Impl: IADsServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetServiceType<Impl: IADsServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnservicetype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StartType<Impl: IADsServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStartType<Impl: IADsServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnstarttype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Path<Impl: IADsServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPath<Impl: IADsServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StartupParameters<Impl: IADsServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStartupParameters<Impl: IADsServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrstartupparameters: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ErrorControl<Impl: IADsServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetErrorControl<Impl: IADsServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnerrorcontrol: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LoadOrderGroup<Impl: IADsServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLoadOrderGroup<Impl: IADsServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrloadordergroup: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ServiceAccountName<Impl: IADsServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetServiceAccountName<Impl: IADsServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrserviceaccountname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ServiceAccountPath<Impl: IADsServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetServiceAccountPath<Impl: IADsServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrserviceaccountpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Dependencies<Impl: IADsServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDependencies<Impl: IADsServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vdependencies: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            Class::<Impl, IMPL_OFFSET>,
            GUID::<Impl, IMPL_OFFSET>,
            ADsPath::<Impl, IMPL_OFFSET>,
            Parent::<Impl, IMPL_OFFSET>,
            Schema::<Impl, IMPL_OFFSET>,
            GetInfo::<Impl, IMPL_OFFSET>,
            SetInfo::<Impl, IMPL_OFFSET>,
            Get::<Impl, IMPL_OFFSET>,
            Put::<Impl, IMPL_OFFSET>,
            GetEx::<Impl, IMPL_OFFSET>,
            PutEx::<Impl, IMPL_OFFSET>,
            GetInfoEx::<Impl, IMPL_OFFSET>,
            HostComputer::<Impl, IMPL_OFFSET>,
            SetHostComputer::<Impl, IMPL_OFFSET>,
            DisplayName::<Impl, IMPL_OFFSET>,
            SetDisplayName::<Impl, IMPL_OFFSET>,
            Version::<Impl, IMPL_OFFSET>,
            SetVersion::<Impl, IMPL_OFFSET>,
            ServiceType::<Impl, IMPL_OFFSET>,
            SetServiceType::<Impl, IMPL_OFFSET>,
            StartType::<Impl, IMPL_OFFSET>,
            SetStartType::<Impl, IMPL_OFFSET>,
            Path::<Impl, IMPL_OFFSET>,
            SetPath::<Impl, IMPL_OFFSET>,
            StartupParameters::<Impl, IMPL_OFFSET>,
            SetStartupParameters::<Impl, IMPL_OFFSET>,
            ErrorControl::<Impl, IMPL_OFFSET>,
            SetErrorControl::<Impl, IMPL_OFFSET>,
            LoadOrderGroup::<Impl, IMPL_OFFSET>,
            SetLoadOrderGroup::<Impl, IMPL_OFFSET>,
            ServiceAccountName::<Impl, IMPL_OFFSET>,
            SetServiceAccountName::<Impl, IMPL_OFFSET>,
            ServiceAccountPath::<Impl, IMPL_OFFSET>,
            SetServiceAccountPath::<Impl, IMPL_OFFSET>,
            Dependencies::<Impl, IMPL_OFFSET>,
            SetDependencies::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsService as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsServiceOperationsImpl: Sized + IADsImpl + IDispatchImpl {
    fn Status();
    fn Start();
    fn Stop();
    fn Pause();
    fn Continue();
    fn SetPassword();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsServiceOperationsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsServiceOperationsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsServiceOperationsVtbl {
        unsafe extern "system" fn Status<Impl: IADsServiceOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Start<Impl: IADsServiceOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Stop<Impl: IADsServiceOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Pause<Impl: IADsServiceOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Continue<Impl: IADsServiceOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPassword<Impl: IADsServiceOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrnewpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            Class::<Impl, IMPL_OFFSET>,
            GUID::<Impl, IMPL_OFFSET>,
            ADsPath::<Impl, IMPL_OFFSET>,
            Parent::<Impl, IMPL_OFFSET>,
            Schema::<Impl, IMPL_OFFSET>,
            GetInfo::<Impl, IMPL_OFFSET>,
            SetInfo::<Impl, IMPL_OFFSET>,
            Get::<Impl, IMPL_OFFSET>,
            Put::<Impl, IMPL_OFFSET>,
            GetEx::<Impl, IMPL_OFFSET>,
            PutEx::<Impl, IMPL_OFFSET>,
            GetInfoEx::<Impl, IMPL_OFFSET>,
            Status::<Impl, IMPL_OFFSET>,
            Start::<Impl, IMPL_OFFSET>,
            Stop::<Impl, IMPL_OFFSET>,
            Pause::<Impl, IMPL_OFFSET>,
            Continue::<Impl, IMPL_OFFSET>,
            SetPassword::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsServiceOperations as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsSessionImpl: Sized + IADsImpl + IDispatchImpl {
    fn User();
    fn UserPath();
    fn Computer();
    fn ComputerPath();
    fn ConnectTime();
    fn IdleTime();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsSessionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsSessionVtbl {
        unsafe extern "system" fn User<Impl: IADsSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UserPath<Impl: IADsSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Computer<Impl: IADsSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ComputerPath<Impl: IADsSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConnectTime<Impl: IADsSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IdleTime<Impl: IADsSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            Class::<Impl, IMPL_OFFSET>,
            GUID::<Impl, IMPL_OFFSET>,
            ADsPath::<Impl, IMPL_OFFSET>,
            Parent::<Impl, IMPL_OFFSET>,
            Schema::<Impl, IMPL_OFFSET>,
            GetInfo::<Impl, IMPL_OFFSET>,
            SetInfo::<Impl, IMPL_OFFSET>,
            Get::<Impl, IMPL_OFFSET>,
            Put::<Impl, IMPL_OFFSET>,
            GetEx::<Impl, IMPL_OFFSET>,
            PutEx::<Impl, IMPL_OFFSET>,
            GetInfoEx::<Impl, IMPL_OFFSET>,
            User::<Impl, IMPL_OFFSET>,
            UserPath::<Impl, IMPL_OFFSET>,
            Computer::<Impl, IMPL_OFFSET>,
            ComputerPath::<Impl, IMPL_OFFSET>,
            ConnectTime::<Impl, IMPL_OFFSET>,
            IdleTime::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsSession as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsSyntaxImpl: Sized + IADsImpl + IDispatchImpl {
    fn OleAutoDataType();
    fn SetOleAutoDataType();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsSyntaxVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsSyntaxImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsSyntaxVtbl {
        unsafe extern "system" fn OleAutoDataType<Impl: IADsSyntaxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOleAutoDataType<Impl: IADsSyntaxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnoleautodatatype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            Class::<Impl, IMPL_OFFSET>,
            GUID::<Impl, IMPL_OFFSET>,
            ADsPath::<Impl, IMPL_OFFSET>,
            Parent::<Impl, IMPL_OFFSET>,
            Schema::<Impl, IMPL_OFFSET>,
            GetInfo::<Impl, IMPL_OFFSET>,
            SetInfo::<Impl, IMPL_OFFSET>,
            Get::<Impl, IMPL_OFFSET>,
            Put::<Impl, IMPL_OFFSET>,
            GetEx::<Impl, IMPL_OFFSET>,
            PutEx::<Impl, IMPL_OFFSET>,
            GetInfoEx::<Impl, IMPL_OFFSET>,
            OleAutoDataType::<Impl, IMPL_OFFSET>,
            SetOleAutoDataType::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsSyntax as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsTimestampImpl: Sized + IDispatchImpl {
    fn WholeSeconds();
    fn SetWholeSeconds();
    fn EventID();
    fn SetEventID();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsTimestampVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsTimestampImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsTimestampVtbl {
        unsafe extern "system" fn WholeSeconds<Impl: IADsTimestampImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetWholeSeconds<Impl: IADsTimestampImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnwholeseconds: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EventID<Impl: IADsTimestampImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEventID<Impl: IADsTimestampImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lneventid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, WholeSeconds::<Impl, IMPL_OFFSET>, SetWholeSeconds::<Impl, IMPL_OFFSET>, EventID::<Impl, IMPL_OFFSET>, SetEventID::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsTimestamp as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsTypedNameImpl: Sized + IDispatchImpl {
    fn ObjectName();
    fn SetObjectName();
    fn Level();
    fn SetLevel();
    fn Interval();
    fn SetInterval();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsTypedNameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsTypedNameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsTypedNameVtbl {
        unsafe extern "system" fn ObjectName<Impl: IADsTypedNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetObjectName<Impl: IADsTypedNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrobjectname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Level<Impl: IADsTypedNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLevel<Impl: IADsTypedNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Interval<Impl: IADsTypedNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInterval<Impl: IADsTypedNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lninterval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            ObjectName::<Impl, IMPL_OFFSET>,
            SetObjectName::<Impl, IMPL_OFFSET>,
            Level::<Impl, IMPL_OFFSET>,
            SetLevel::<Impl, IMPL_OFFSET>,
            Interval::<Impl, IMPL_OFFSET>,
            SetInterval::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsTypedName as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsUserImpl: Sized + IADsImpl + IDispatchImpl {
    fn BadLoginAddress();
    fn BadLoginCount();
    fn LastLogin();
    fn LastLogoff();
    fn LastFailedLogin();
    fn PasswordLastChanged();
    fn Description();
    fn SetDescription();
    fn Division();
    fn SetDivision();
    fn Department();
    fn SetDepartment();
    fn EmployeeID();
    fn SetEmployeeID();
    fn FullName();
    fn SetFullName();
    fn FirstName();
    fn SetFirstName();
    fn LastName();
    fn SetLastName();
    fn OtherName();
    fn SetOtherName();
    fn NamePrefix();
    fn SetNamePrefix();
    fn NameSuffix();
    fn SetNameSuffix();
    fn Title();
    fn SetTitle();
    fn Manager();
    fn SetManager();
    fn TelephoneHome();
    fn SetTelephoneHome();
    fn TelephoneMobile();
    fn SetTelephoneMobile();
    fn TelephoneNumber();
    fn SetTelephoneNumber();
    fn TelephonePager();
    fn SetTelephonePager();
    fn FaxNumber();
    fn SetFaxNumber();
    fn OfficeLocations();
    fn SetOfficeLocations();
    fn PostalAddresses();
    fn SetPostalAddresses();
    fn PostalCodes();
    fn SetPostalCodes();
    fn SeeAlso();
    fn SetSeeAlso();
    fn AccountDisabled();
    fn SetAccountDisabled();
    fn AccountExpirationDate();
    fn SetAccountExpirationDate();
    fn GraceLoginsAllowed();
    fn SetGraceLoginsAllowed();
    fn GraceLoginsRemaining();
    fn SetGraceLoginsRemaining();
    fn IsAccountLocked();
    fn SetIsAccountLocked();
    fn LoginHours();
    fn SetLoginHours();
    fn LoginWorkstations();
    fn SetLoginWorkstations();
    fn MaxLogins();
    fn SetMaxLogins();
    fn MaxStorage();
    fn SetMaxStorage();
    fn PasswordExpirationDate();
    fn SetPasswordExpirationDate();
    fn PasswordMinimumLength();
    fn SetPasswordMinimumLength();
    fn PasswordRequired();
    fn SetPasswordRequired();
    fn RequireUniquePassword();
    fn SetRequireUniquePassword();
    fn EmailAddress();
    fn SetEmailAddress();
    fn HomeDirectory();
    fn SetHomeDirectory();
    fn Languages();
    fn SetLanguages();
    fn Profile();
    fn SetProfile();
    fn LoginScript();
    fn SetLoginScript();
    fn Picture();
    fn SetPicture();
    fn HomePage();
    fn SetHomePage();
    fn Groups();
    fn SetPassword();
    fn ChangePassword();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsUserVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsUserImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsUserVtbl {
        unsafe extern "system" fn BadLoginAddress<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BadLoginCount<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LastLogin<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LastLogoff<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LastFailedLogin<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PasswordLastChanged<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Description<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDescription<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Division<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDivision<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdivision: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Department<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDepartment<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdepartment: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EmployeeID<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEmployeeID<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstremployeeid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FullName<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFullName<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfullname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FirstName<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFirstName<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfirstname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LastName<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLastName<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlastname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OtherName<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOtherName<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrothername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NamePrefix<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNamePrefix<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrnameprefix: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NameSuffix<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNameSuffix<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrnamesuffix: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Title<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTitle<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Manager<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetManager<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmanager: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TelephoneHome<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTelephoneHome<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vtelephonehome: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TelephoneMobile<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTelephoneMobile<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vtelephonemobile: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TelephoneNumber<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTelephoneNumber<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vtelephonenumber: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TelephonePager<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTelephonePager<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vtelephonepager: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FaxNumber<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFaxNumber<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vfaxnumber: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OfficeLocations<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOfficeLocations<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vofficelocations: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PostalAddresses<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPostalAddresses<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vpostaladdresses: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PostalCodes<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPostalCodes<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vpostalcodes: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SeeAlso<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSeeAlso<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vseealso: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AccountDisabled<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAccountDisabled<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, faccountdisabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AccountExpirationDate<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAccountExpirationDate<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, daaccountexpirationdate: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GraceLoginsAllowed<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetGraceLoginsAllowed<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lngraceloginsallowed: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GraceLoginsRemaining<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetGraceLoginsRemaining<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lngraceloginsremaining: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsAccountLocked<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIsAccountLocked<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fisaccountlocked: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LoginHours<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLoginHours<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vloginhours: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LoginWorkstations<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLoginWorkstations<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vloginworkstations: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MaxLogins<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMaxLogins<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnmaxlogins: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MaxStorage<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMaxStorage<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnmaxstorage: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PasswordExpirationDate<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPasswordExpirationDate<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dapasswordexpirationdate: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PasswordMinimumLength<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPasswordMinimumLength<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnpasswordminimumlength: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PasswordRequired<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPasswordRequired<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fpasswordrequired: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequireUniquePassword<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRequireUniquePassword<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frequireuniquepassword: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EmailAddress<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEmailAddress<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstremailaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HomeDirectory<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHomeDirectory<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrhomedirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Languages<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLanguages<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vlanguages: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Profile<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProfile<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprofile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LoginScript<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLoginScript<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrloginscript: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Picture<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPicture<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vpicture: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HomePage<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHomePage<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrhomepage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Groups<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgroups: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPassword<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ChangePassword<Impl: IADsUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstroldpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnewpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            Class::<Impl, IMPL_OFFSET>,
            GUID::<Impl, IMPL_OFFSET>,
            ADsPath::<Impl, IMPL_OFFSET>,
            Parent::<Impl, IMPL_OFFSET>,
            Schema::<Impl, IMPL_OFFSET>,
            GetInfo::<Impl, IMPL_OFFSET>,
            SetInfo::<Impl, IMPL_OFFSET>,
            Get::<Impl, IMPL_OFFSET>,
            Put::<Impl, IMPL_OFFSET>,
            GetEx::<Impl, IMPL_OFFSET>,
            PutEx::<Impl, IMPL_OFFSET>,
            GetInfoEx::<Impl, IMPL_OFFSET>,
            BadLoginAddress::<Impl, IMPL_OFFSET>,
            BadLoginCount::<Impl, IMPL_OFFSET>,
            LastLogin::<Impl, IMPL_OFFSET>,
            LastLogoff::<Impl, IMPL_OFFSET>,
            LastFailedLogin::<Impl, IMPL_OFFSET>,
            PasswordLastChanged::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            Division::<Impl, IMPL_OFFSET>,
            SetDivision::<Impl, IMPL_OFFSET>,
            Department::<Impl, IMPL_OFFSET>,
            SetDepartment::<Impl, IMPL_OFFSET>,
            EmployeeID::<Impl, IMPL_OFFSET>,
            SetEmployeeID::<Impl, IMPL_OFFSET>,
            FullName::<Impl, IMPL_OFFSET>,
            SetFullName::<Impl, IMPL_OFFSET>,
            FirstName::<Impl, IMPL_OFFSET>,
            SetFirstName::<Impl, IMPL_OFFSET>,
            LastName::<Impl, IMPL_OFFSET>,
            SetLastName::<Impl, IMPL_OFFSET>,
            OtherName::<Impl, IMPL_OFFSET>,
            SetOtherName::<Impl, IMPL_OFFSET>,
            NamePrefix::<Impl, IMPL_OFFSET>,
            SetNamePrefix::<Impl, IMPL_OFFSET>,
            NameSuffix::<Impl, IMPL_OFFSET>,
            SetNameSuffix::<Impl, IMPL_OFFSET>,
            Title::<Impl, IMPL_OFFSET>,
            SetTitle::<Impl, IMPL_OFFSET>,
            Manager::<Impl, IMPL_OFFSET>,
            SetManager::<Impl, IMPL_OFFSET>,
            TelephoneHome::<Impl, IMPL_OFFSET>,
            SetTelephoneHome::<Impl, IMPL_OFFSET>,
            TelephoneMobile::<Impl, IMPL_OFFSET>,
            SetTelephoneMobile::<Impl, IMPL_OFFSET>,
            TelephoneNumber::<Impl, IMPL_OFFSET>,
            SetTelephoneNumber::<Impl, IMPL_OFFSET>,
            TelephonePager::<Impl, IMPL_OFFSET>,
            SetTelephonePager::<Impl, IMPL_OFFSET>,
            FaxNumber::<Impl, IMPL_OFFSET>,
            SetFaxNumber::<Impl, IMPL_OFFSET>,
            OfficeLocations::<Impl, IMPL_OFFSET>,
            SetOfficeLocations::<Impl, IMPL_OFFSET>,
            PostalAddresses::<Impl, IMPL_OFFSET>,
            SetPostalAddresses::<Impl, IMPL_OFFSET>,
            PostalCodes::<Impl, IMPL_OFFSET>,
            SetPostalCodes::<Impl, IMPL_OFFSET>,
            SeeAlso::<Impl, IMPL_OFFSET>,
            SetSeeAlso::<Impl, IMPL_OFFSET>,
            AccountDisabled::<Impl, IMPL_OFFSET>,
            SetAccountDisabled::<Impl, IMPL_OFFSET>,
            AccountExpirationDate::<Impl, IMPL_OFFSET>,
            SetAccountExpirationDate::<Impl, IMPL_OFFSET>,
            GraceLoginsAllowed::<Impl, IMPL_OFFSET>,
            SetGraceLoginsAllowed::<Impl, IMPL_OFFSET>,
            GraceLoginsRemaining::<Impl, IMPL_OFFSET>,
            SetGraceLoginsRemaining::<Impl, IMPL_OFFSET>,
            IsAccountLocked::<Impl, IMPL_OFFSET>,
            SetIsAccountLocked::<Impl, IMPL_OFFSET>,
            LoginHours::<Impl, IMPL_OFFSET>,
            SetLoginHours::<Impl, IMPL_OFFSET>,
            LoginWorkstations::<Impl, IMPL_OFFSET>,
            SetLoginWorkstations::<Impl, IMPL_OFFSET>,
            MaxLogins::<Impl, IMPL_OFFSET>,
            SetMaxLogins::<Impl, IMPL_OFFSET>,
            MaxStorage::<Impl, IMPL_OFFSET>,
            SetMaxStorage::<Impl, IMPL_OFFSET>,
            PasswordExpirationDate::<Impl, IMPL_OFFSET>,
            SetPasswordExpirationDate::<Impl, IMPL_OFFSET>,
            PasswordMinimumLength::<Impl, IMPL_OFFSET>,
            SetPasswordMinimumLength::<Impl, IMPL_OFFSET>,
            PasswordRequired::<Impl, IMPL_OFFSET>,
            SetPasswordRequired::<Impl, IMPL_OFFSET>,
            RequireUniquePassword::<Impl, IMPL_OFFSET>,
            SetRequireUniquePassword::<Impl, IMPL_OFFSET>,
            EmailAddress::<Impl, IMPL_OFFSET>,
            SetEmailAddress::<Impl, IMPL_OFFSET>,
            HomeDirectory::<Impl, IMPL_OFFSET>,
            SetHomeDirectory::<Impl, IMPL_OFFSET>,
            Languages::<Impl, IMPL_OFFSET>,
            SetLanguages::<Impl, IMPL_OFFSET>,
            Profile::<Impl, IMPL_OFFSET>,
            SetProfile::<Impl, IMPL_OFFSET>,
            LoginScript::<Impl, IMPL_OFFSET>,
            SetLoginScript::<Impl, IMPL_OFFSET>,
            Picture::<Impl, IMPL_OFFSET>,
            SetPicture::<Impl, IMPL_OFFSET>,
            HomePage::<Impl, IMPL_OFFSET>,
            SetHomePage::<Impl, IMPL_OFFSET>,
            Groups::<Impl, IMPL_OFFSET>,
            SetPassword::<Impl, IMPL_OFFSET>,
            ChangePassword::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsUser as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsWinNTSystemInfoImpl: Sized + IDispatchImpl {
    fn UserName();
    fn ComputerName();
    fn DomainName();
    fn PDC();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsWinNTSystemInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsWinNTSystemInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsWinNTSystemInfoVtbl {
        unsafe extern "system" fn UserName<Impl: IADsWinNTSystemInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ComputerName<Impl: IADsWinNTSystemInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DomainName<Impl: IADsWinNTSystemInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PDC<Impl: IADsWinNTSystemInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, UserName::<Impl, IMPL_OFFSET>, ComputerName::<Impl, IMPL_OFFSET>, DomainName::<Impl, IMPL_OFFSET>, PDC::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsWinNTSystemInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait ICommonQueryImpl: Sized {
    fn OpenQueryWindow();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl ICommonQueryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommonQueryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICommonQueryVtbl {
        unsafe extern "system" fn OpenQueryWindow<Impl: ICommonQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, pquerywnd: *mut OPENQUERYWINDOW, ppdataobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OpenQueryWindow::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommonQuery as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IDirectoryObjectImpl: Sized {
    fn GetObjectInformation();
    fn GetObjectAttributes();
    fn SetObjectAttributes();
    fn CreateDSObject();
    fn DeleteDSObject();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IDirectoryObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectoryObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectoryObjectVtbl {
        unsafe extern "system" fn GetObjectInformation<Impl: IDirectoryObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppobjinfo: *mut *mut ADS_OBJECT_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetObjectAttributes<Impl: IDirectoryObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattributenames: *const super::super::Foundation::PWSTR, dwnumberattributes: u32, ppattributeentries: *mut *mut ADS_ATTR_INFO, pdwnumattributesreturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetObjectAttributes<Impl: IDirectoryObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattributeentries: *const ADS_ATTR_INFO, dwnumattributes: u32, pdwnumattributesmodified: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateDSObject<Impl: IDirectoryObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszrdnname: super::super::Foundation::PWSTR, pattributeentries: *const ADS_ATTR_INFO, dwnumattributes: u32, ppobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteDSObject<Impl: IDirectoryObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszrdnname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetObjectInformation::<Impl, IMPL_OFFSET>, GetObjectAttributes::<Impl, IMPL_OFFSET>, SetObjectAttributes::<Impl, IMPL_OFFSET>, CreateDSObject::<Impl, IMPL_OFFSET>, DeleteDSObject::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectoryObject as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectorySchemaMgmtImpl: Sized {
    fn EnumAttributes();
    fn CreateAttributeDefinition();
    fn WriteAttributeDefinition();
    fn DeleteAttributeDefinition();
    fn EnumClasses();
    fn WriteClassDefinition();
    fn CreateClassDefinition();
    fn DeleteClassDefinition();
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectorySchemaMgmtVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectorySchemaMgmtImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectorySchemaMgmtVtbl {
        unsafe extern "system" fn EnumAttributes<Impl: IDirectorySchemaMgmtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszattrnames: *const super::super::Foundation::PWSTR, dwnumattributes: u32, ppattrdefinition: *const *const ADS_ATTR_DEF, pdwnumattributes: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateAttributeDefinition<Impl: IDirectorySchemaMgmtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszattributename: super::super::Foundation::PWSTR, pattributedefinition: *const ADS_ATTR_DEF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteAttributeDefinition<Impl: IDirectorySchemaMgmtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszattributename: super::super::Foundation::PWSTR, pattributedefinition: *const ADS_ATTR_DEF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteAttributeDefinition<Impl: IDirectorySchemaMgmtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszattributename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumClasses<Impl: IDirectorySchemaMgmtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszclassnames: *const super::super::Foundation::PWSTR, dwnumclasses: u32, ppclassdefinition: *const *const ADS_CLASS_DEF, pdwnumclasses: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteClassDefinition<Impl: IDirectorySchemaMgmtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszclassname: super::super::Foundation::PWSTR, pclassdefinition: *const ADS_CLASS_DEF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateClassDefinition<Impl: IDirectorySchemaMgmtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszclassname: super::super::Foundation::PWSTR, pclassdefinition: *const ADS_CLASS_DEF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteClassDefinition<Impl: IDirectorySchemaMgmtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszclassname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            EnumAttributes::<Impl, IMPL_OFFSET>,
            CreateAttributeDefinition::<Impl, IMPL_OFFSET>,
            WriteAttributeDefinition::<Impl, IMPL_OFFSET>,
            DeleteAttributeDefinition::<Impl, IMPL_OFFSET>,
            EnumClasses::<Impl, IMPL_OFFSET>,
            WriteClassDefinition::<Impl, IMPL_OFFSET>,
            CreateClassDefinition::<Impl, IMPL_OFFSET>,
            DeleteClassDefinition::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectorySchemaMgmt as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectorySearchImpl: Sized {
    fn SetSearchPreference();
    fn ExecuteSearch();
    fn AbandonSearch();
    fn GetFirstRow();
    fn GetNextRow();
    fn GetPreviousRow();
    fn GetNextColumnName();
    fn GetColumn();
    fn FreeColumn();
    fn CloseSearchHandle();
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectorySearchVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectorySearchImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectorySearchVtbl {
        unsafe extern "system" fn SetSearchPreference<Impl: IDirectorySearchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psearchprefs: *const ads_searchpref_info, dwnumprefs: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExecuteSearch<Impl: IDirectorySearchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsearchfilter: super::super::Foundation::PWSTR, pattributenames: *const super::super::Foundation::PWSTR, dwnumberattributes: u32, phsearchresult: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AbandonSearch<Impl: IDirectorySearchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phsearchresult: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFirstRow<Impl: IDirectorySearchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hsearchresult: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNextRow<Impl: IDirectorySearchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hsearchresult: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPreviousRow<Impl: IDirectorySearchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hsearchresult: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNextColumnName<Impl: IDirectorySearchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hsearchhandle: isize, ppszcolumnname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetColumn<Impl: IDirectorySearchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hsearchresult: isize, szcolumnname: super::super::Foundation::PWSTR, psearchcolumn: *mut ads_search_column) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FreeColumn<Impl: IDirectorySearchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psearchcolumn: *const ads_search_column) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CloseSearchHandle<Impl: IDirectorySearchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hsearchresult: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetSearchPreference::<Impl, IMPL_OFFSET>,
            ExecuteSearch::<Impl, IMPL_OFFSET>,
            AbandonSearch::<Impl, IMPL_OFFSET>,
            GetFirstRow::<Impl, IMPL_OFFSET>,
            GetNextRow::<Impl, IMPL_OFFSET>,
            GetPreviousRow::<Impl, IMPL_OFFSET>,
            GetNextColumnName::<Impl, IMPL_OFFSET>,
            GetColumn::<Impl, IMPL_OFFSET>,
            FreeColumn::<Impl, IMPL_OFFSET>,
            CloseSearchHandle::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectorySearch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDsAdminCreateObjImpl: Sized {
    fn Initialize();
    fn CreateModal();
}
#[cfg(feature = "Win32_Foundation")]
impl IDsAdminCreateObjVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDsAdminCreateObjImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDsAdminCreateObjVtbl {
        unsafe extern "system" fn Initialize<Impl: IDsAdminCreateObjImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padscontainerobj: ::windows::core::RawPtr, padscopysource: ::windows::core::RawPtr, lpszclassname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateModal<Impl: IDsAdminCreateObjImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ppadsobj: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, CreateModal::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDsAdminCreateObj as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDsAdminNewObjImpl: Sized {
    fn SetButtons();
    fn GetPageCounts();
}
#[cfg(feature = "Win32_Foundation")]
impl IDsAdminNewObjVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDsAdminNewObjImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDsAdminNewObjVtbl {
        unsafe extern "system" fn SetButtons<Impl: IDsAdminNewObjImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncurrindex: u32, bvalid: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPageCounts<Impl: IDsAdminNewObjImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pntotal: *mut i32, pnstartindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetButtons::<Impl, IMPL_OFFSET>, GetPageCounts::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDsAdminNewObj as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IDsAdminNewObjExtImpl: Sized {
    fn Initialize();
    fn AddPages();
    fn SetObject();
    fn WriteData();
    fn OnError();
    fn GetSummaryInfo();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl IDsAdminNewObjExtVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDsAdminNewObjExtImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDsAdminNewObjExtVtbl {
        unsafe extern "system" fn Initialize<Impl: IDsAdminNewObjExtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padscontainerobj: ::windows::core::RawPtr, padscopysource: ::windows::core::RawPtr, lpszclassname: super::super::Foundation::PWSTR, pdsadminnewobj: ::windows::core::RawPtr, pdispinfo: *mut DSA_NEWOBJ_DISPINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddPages<Impl: IDsAdminNewObjExtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpfnaddpage: ::windows::core::RawPtr, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetObject<Impl: IDsAdminNewObjExtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padsobj: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteData<Impl: IDsAdminNewObjExtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, ucontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnError<Impl: IDsAdminNewObjExtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, hr: ::windows::core::HRESULT, ucontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSummaryInfo<Impl: IDsAdminNewObjExtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, AddPages::<Impl, IMPL_OFFSET>, SetObject::<Impl, IMPL_OFFSET>, WriteData::<Impl, IMPL_OFFSET>, OnError::<Impl, IMPL_OFFSET>, GetSummaryInfo::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDsAdminNewObjExt as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDsAdminNewObjPrimarySiteImpl: Sized {
    fn CreateNew();
    fn Commit();
}
#[cfg(feature = "Win32_Foundation")]
impl IDsAdminNewObjPrimarySiteVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDsAdminNewObjPrimarySiteImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDsAdminNewObjPrimarySiteVtbl {
        unsafe extern "system" fn CreateNew<Impl: IDsAdminNewObjPrimarySiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Commit<Impl: IDsAdminNewObjPrimarySiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateNew::<Impl, IMPL_OFFSET>, Commit::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDsAdminNewObjPrimarySite as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IDsAdminNotifyHandlerImpl: Sized {
    fn Initialize();
    fn Begin();
    fn Notify();
    fn End();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IDsAdminNotifyHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDsAdminNotifyHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDsAdminNotifyHandlerVtbl {
        unsafe extern "system" fn Initialize<Impl: IDsAdminNotifyHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pextrainfo: ::windows::core::RawPtr, pueventflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Begin<Impl: IDsAdminNotifyHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uevent: u32, parg1: ::windows::core::RawPtr, parg2: ::windows::core::RawPtr, puflags: *mut u32, pbstr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Notify<Impl: IDsAdminNotifyHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nitem: u32, uflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn End<Impl: IDsAdminNotifyHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, Begin::<Impl, IMPL_OFFSET>, Notify::<Impl, IMPL_OFFSET>, End::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDsAdminNotifyHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDsBrowseDomainTreeImpl: Sized {
    fn BrowseTo();
    fn GetDomains();
    fn FreeDomains();
    fn FlushCachedDomains();
    fn SetComputer();
}
#[cfg(feature = "Win32_Foundation")]
impl IDsBrowseDomainTreeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDsBrowseDomainTreeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDsBrowseDomainTreeVtbl {
        unsafe extern "system" fn BrowseTo<Impl: IDsBrowseDomainTreeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ppsztargetpath: *mut super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDomains<Impl: IDsBrowseDomainTreeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdomaintree: *mut *mut DOMAIN_TREE, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FreeDomains<Impl: IDsBrowseDomainTreeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdomaintree: *mut *mut DOMAIN_TREE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FlushCachedDomains<Impl: IDsBrowseDomainTreeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetComputer<Impl: IDsBrowseDomainTreeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcomputername: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR, pszpassword: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, BrowseTo::<Impl, IMPL_OFFSET>, GetDomains::<Impl, IMPL_OFFSET>, FreeDomains::<Impl, IMPL_OFFSET>, FlushCachedDomains::<Impl, IMPL_OFFSET>, SetComputer::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDsBrowseDomainTree as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IDsDisplaySpecifierImpl: Sized {
    fn SetServer();
    fn SetLanguageID();
    fn GetDisplaySpecifier();
    fn GetIconLocation();
    fn GetIcon();
    fn GetFriendlyClassName();
    fn GetFriendlyAttributeName();
    fn IsClassContainer();
    fn GetClassCreationInfo();
    fn EnumClassAttributes();
    fn GetAttributeADsType();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IDsDisplaySpecifierVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDsDisplaySpecifierImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDsDisplaySpecifierVtbl {
        unsafe extern "system" fn SetServer<Impl: IDsDisplaySpecifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszserver: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR, pszpassword: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLanguageID<Impl: IDsDisplaySpecifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDisplaySpecifier<Impl: IDsDisplaySpecifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectclass: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIconLocation<Impl: IDsDisplaySpecifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectclass: super::super::Foundation::PWSTR, dwflags: u32, pszbuffer: super::super::Foundation::PWSTR, cchbuffer: i32, presid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIcon<Impl: IDsDisplaySpecifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectclass: super::super::Foundation::PWSTR, dwflags: u32, cxicon: i32, cyicon: i32) -> super::super::UI::WindowsAndMessaging::HICON {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFriendlyClassName<Impl: IDsDisplaySpecifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectclass: super::super::Foundation::PWSTR, pszbuffer: super::super::Foundation::PWSTR, cchbuffer: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFriendlyAttributeName<Impl: IDsDisplaySpecifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectclass: super::super::Foundation::PWSTR, pszattributename: super::super::Foundation::PWSTR, pszbuffer: super::super::Foundation::PWSTR, cchbuffer: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsClassContainer<Impl: IDsDisplaySpecifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectclass: super::super::Foundation::PWSTR, pszadspath: super::super::Foundation::PWSTR, dwflags: u32) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetClassCreationInfo<Impl: IDsDisplaySpecifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectclass: super::super::Foundation::PWSTR, ppdscci: *mut *mut DSCLASSCREATIONINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumClassAttributes<Impl: IDsDisplaySpecifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectclass: super::super::Foundation::PWSTR, pcbenum: ::windows::core::RawPtr, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAttributeADsType<Impl: IDsDisplaySpecifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszattributename: super::super::Foundation::PWSTR) -> ADSTYPEENUM {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetServer::<Impl, IMPL_OFFSET>,
            SetLanguageID::<Impl, IMPL_OFFSET>,
            GetDisplaySpecifier::<Impl, IMPL_OFFSET>,
            GetIconLocation::<Impl, IMPL_OFFSET>,
            GetIcon::<Impl, IMPL_OFFSET>,
            GetFriendlyClassName::<Impl, IMPL_OFFSET>,
            GetFriendlyAttributeName::<Impl, IMPL_OFFSET>,
            IsClassContainer::<Impl, IMPL_OFFSET>,
            GetClassCreationInfo::<Impl, IMPL_OFFSET>,
            EnumClassAttributes::<Impl, IMPL_OFFSET>,
            GetAttributeADsType::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDsDisplaySpecifier as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IDsObjectPickerImpl: Sized {
    fn Initialize();
    fn InvokeDialog();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IDsObjectPickerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDsObjectPickerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDsObjectPickerVtbl {
        unsafe extern "system" fn Initialize<Impl: IDsObjectPickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinitinfo: *mut DSOP_INIT_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InvokeDialog<Impl: IDsObjectPickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ppdoselections: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, InvokeDialog::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDsObjectPicker as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IDsObjectPickerCredentialsImpl: Sized + IDsObjectPickerImpl {
    fn SetCredentials();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IDsObjectPickerCredentialsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDsObjectPickerCredentialsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDsObjectPickerCredentialsVtbl {
        unsafe extern "system" fn SetCredentials<Impl: IDsObjectPickerCredentialsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szusername: super::super::Foundation::PWSTR, szpassword: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, InvokeDialog::<Impl, IMPL_OFFSET>, SetCredentials::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDsObjectPickerCredentials as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IPersistQueryImpl: Sized + IPersistImpl {
    fn WriteString();
    fn ReadString();
    fn WriteInt();
    fn ReadInt();
    fn WriteStruct();
    fn ReadStruct();
    fn Clear();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IPersistQueryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPersistQueryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPersistQueryVtbl {
        unsafe extern "system" fn WriteString<Impl: IPersistQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psection: super::super::Foundation::PWSTR, pvaluename: super::super::Foundation::PWSTR, pvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReadString<Impl: IPersistQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psection: super::super::Foundation::PWSTR, pvaluename: super::super::Foundation::PWSTR, pbuffer: super::super::Foundation::PWSTR, cchbuffer: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteInt<Impl: IPersistQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psection: super::super::Foundation::PWSTR, pvaluename: super::super::Foundation::PWSTR, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReadInt<Impl: IPersistQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psection: super::super::Foundation::PWSTR, pvaluename: super::super::Foundation::PWSTR, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteStruct<Impl: IPersistQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psection: super::super::Foundation::PWSTR, pvaluename: super::super::Foundation::PWSTR, pstruct: *mut ::core::ffi::c_void, cbstruct: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReadStruct<Impl: IPersistQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psection: super::super::Foundation::PWSTR, pvaluename: super::super::Foundation::PWSTR, pstruct: *mut ::core::ffi::c_void, cbstruct: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clear<Impl: IPersistQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetClassID::<Impl, IMPL_OFFSET>, WriteString::<Impl, IMPL_OFFSET>, ReadString::<Impl, IMPL_OFFSET>, WriteInt::<Impl, IMPL_OFFSET>, ReadInt::<Impl, IMPL_OFFSET>, WriteStruct::<Impl, IMPL_OFFSET>, ReadStruct::<Impl, IMPL_OFFSET>, Clear::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPersistQuery as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrivateDispatchImpl: Sized {
    fn ADSIInitializeDispatchManager();
    fn ADSIGetTypeInfoCount();
    fn ADSIGetTypeInfo();
    fn ADSIGetIDsOfNames();
    fn ADSIInvoke();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrivateDispatchVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrivateDispatchImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrivateDispatchVtbl {
        unsafe extern "system" fn ADSIInitializeDispatchManager<Impl: IPrivateDispatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwextensionid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ADSIGetTypeInfoCount<Impl: IPrivateDispatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ADSIGetTypeInfo<Impl: IPrivateDispatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ADSIGetIDsOfNames<Impl: IPrivateDispatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const *const u16, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ADSIInvoke<Impl: IPrivateDispatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ADSIInitializeDispatchManager::<Impl, IMPL_OFFSET>, ADSIGetTypeInfoCount::<Impl, IMPL_OFFSET>, ADSIGetTypeInfo::<Impl, IMPL_OFFSET>, ADSIGetIDsOfNames::<Impl, IMPL_OFFSET>, ADSIInvoke::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrivateDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPrivateUnknownImpl: Sized {
    fn ADSIInitializeObject();
    fn ADSIReleaseObject();
}
#[cfg(feature = "Win32_Foundation")]
impl IPrivateUnknownVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrivateUnknownImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrivateUnknownVtbl {
        unsafe extern "system" fn ADSIInitializeObject<Impl: IPrivateUnknownImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lpszpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lnreserved: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ADSIReleaseObject<Impl: IPrivateUnknownImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ADSIInitializeObject::<Impl, IMPL_OFFSET>, ADSIReleaseObject::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrivateUnknown as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IQueryFormImpl: Sized {
    fn Initialize();
    fn AddForms();
    fn AddPages();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry", feature = "Win32_UI_WindowsAndMessaging"))]
impl IQueryFormVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IQueryFormImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IQueryFormVtbl {
        unsafe extern "system" fn Initialize<Impl: IQueryFormImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkform: super::super::System::Registry::HKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddForms<Impl: IQueryFormImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddformsproc: ::windows::core::RawPtr, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddPages<Impl: IQueryFormImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddpagesproc: ::windows::core::RawPtr, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, AddForms::<Impl, IMPL_OFFSET>, AddPages::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IQueryForm as ::windows::core::Interface>::IID
    }
}
