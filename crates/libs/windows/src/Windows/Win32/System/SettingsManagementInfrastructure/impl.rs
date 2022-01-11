#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IItemEnumeratorImpl: Sized {
    fn Current();
    fn MoveNext();
    fn Reset();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IItemEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IItemEnumeratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IItemEnumeratorVtbl {
        unsafe extern "system" fn Current<Impl: IItemEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MoveNext<Impl: IItemEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemvalid: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IItemEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Current::<Impl, IMPL_OFFSET>, MoveNext::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IItemEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ISettingsContextImpl: Sized {
    fn Serialize();
    fn Deserialize();
    fn SetUserData();
    fn GetUserData();
    fn GetNamespaces();
    fn GetStoredSettings();
    fn RevertSetting();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ISettingsContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISettingsContextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISettingsContextVtbl {
        unsafe extern "system" fn Serialize<Impl: ISettingsContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, ptarget: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Deserialize<Impl: ISettingsContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, ptarget: ::windows::core::RawPtr, pppresults: *mut *mut ::windows::core::RawPtr, pcresultcount: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUserData<Impl: ISettingsContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puserdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUserData<Impl: ISettingsContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puserdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNamespaces<Impl: ISettingsContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnamespaceids: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStoredSettings<Impl: ISettingsContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidentity: ::windows::core::RawPtr, ppaddedsettings: *mut ::windows::core::RawPtr, ppmodifiedsettings: *mut ::windows::core::RawPtr, ppdeletedsettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RevertSetting<Impl: ISettingsContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidentity: ::windows::core::RawPtr, pwzsetting: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Serialize::<Impl, IMPL_OFFSET>, Deserialize::<Impl, IMPL_OFFSET>, SetUserData::<Impl, IMPL_OFFSET>, GetUserData::<Impl, IMPL_OFFSET>, GetNamespaces::<Impl, IMPL_OFFSET>, GetStoredSettings::<Impl, IMPL_OFFSET>, RevertSetting::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISettingsContext as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISettingsEngineImpl: Sized {
    fn GetNamespaces();
    fn GetNamespace();
    fn GetErrorDescription();
    fn CreateSettingsIdentity();
    fn GetStoreStatus();
    fn LoadStore();
    fn UnloadStore();
    fn RegisterNamespace();
    fn UnregisterNamespace();
    fn CreateTargetInfo();
    fn GetTargetInfo();
    fn SetTargetInfo();
    fn CreateSettingsContext();
    fn SetSettingsContext();
    fn ApplySettingsContext();
    fn GetSettingsContext();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISettingsEngineVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISettingsEngineImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISettingsEngineVtbl {
        unsafe extern "system" fn GetNamespaces<Impl: ISettingsEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: WcmNamespaceEnumerationFlags, reserved: *const ::core::ffi::c_void, namespaces: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNamespace<Impl: ISettingsEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settingsid: ::windows::core::RawPtr, access: WcmNamespaceAccess, reserved: *const ::core::ffi::c_void, namespaceitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetErrorDescription<Impl: ISettingsEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresult: i32, message: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSettingsIdentity<Impl: ISettingsEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settingsid: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStoreStatus<Impl: ISettingsEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reserved: *const ::core::ffi::c_void, status: *mut WcmUserStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LoadStore<Impl: ISettingsEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnloadStore<Impl: ISettingsEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reserved: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RegisterNamespace<Impl: ISettingsEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settingsid: ::windows::core::RawPtr, stream: ::windows::core::RawPtr, pushsettings: super::super::Foundation::BOOL, results: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnregisterNamespace<Impl: ISettingsEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settingsid: ::windows::core::RawPtr, removesettings: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateTargetInfo<Impl: ISettingsEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTargetInfo<Impl: ISettingsEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTargetInfo<Impl: ISettingsEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSettingsContext<Impl: ISettingsEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: u32, reserved: *const ::core::ffi::c_void, settingscontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSettingsContext<Impl: ISettingsEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settingscontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ApplySettingsContext<Impl: ISettingsEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settingscontext: ::windows::core::RawPtr, pppwzidentities: *mut *mut super::super::Foundation::PWSTR, pcidentities: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSettingsContext<Impl: ISettingsEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settingscontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetNamespaces::<Impl, IMPL_OFFSET>,
            GetNamespace::<Impl, IMPL_OFFSET>,
            GetErrorDescription::<Impl, IMPL_OFFSET>,
            CreateSettingsIdentity::<Impl, IMPL_OFFSET>,
            GetStoreStatus::<Impl, IMPL_OFFSET>,
            LoadStore::<Impl, IMPL_OFFSET>,
            UnloadStore::<Impl, IMPL_OFFSET>,
            RegisterNamespace::<Impl, IMPL_OFFSET>,
            UnregisterNamespace::<Impl, IMPL_OFFSET>,
            CreateTargetInfo::<Impl, IMPL_OFFSET>,
            GetTargetInfo::<Impl, IMPL_OFFSET>,
            SetTargetInfo::<Impl, IMPL_OFFSET>,
            CreateSettingsContext::<Impl, IMPL_OFFSET>,
            SetSettingsContext::<Impl, IMPL_OFFSET>,
            ApplySettingsContext::<Impl, IMPL_OFFSET>,
            GetSettingsContext::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISettingsEngine as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISettingsIdentityImpl: Sized {
    fn GetAttribute();
    fn SetAttribute();
    fn GetFlags();
    fn SetFlags();
}
#[cfg(feature = "Win32_Foundation")]
impl ISettingsIdentityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISettingsIdentityImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISettingsIdentityVtbl {
        unsafe extern "system" fn GetAttribute<Impl: ISettingsIdentityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reserved: *const ::core::ffi::c_void, name: super::super::Foundation::PWSTR, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAttribute<Impl: ISettingsIdentityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reserved: *const ::core::ffi::c_void, name: super::super::Foundation::PWSTR, value: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFlags<Impl: ISettingsIdentityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFlags<Impl: ISettingsIdentityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetAttribute::<Impl, IMPL_OFFSET>, SetAttribute::<Impl, IMPL_OFFSET>, GetFlags::<Impl, IMPL_OFFSET>, SetFlags::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISettingsIdentity as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISettingsItemImpl: Sized {
    fn GetName();
    fn GetValue();
    fn SetValue();
    fn GetSettingType();
    fn GetDataType();
    fn GetValueRaw();
    fn SetValueRaw();
    fn HasChild();
    fn Children();
    fn GetChild();
    fn GetSettingByPath();
    fn CreateSettingByPath();
    fn RemoveSettingByPath();
    fn GetListKeyInformation();
    fn CreateListElement();
    fn RemoveListElement();
    fn Attributes();
    fn GetAttribute();
    fn GetPath();
    fn GetRestrictionFacets();
    fn GetRestriction();
    fn GetKeyValue();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISettingsItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISettingsItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISettingsItemVtbl {
        unsafe extern "system" fn GetName<Impl: ISettingsItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetValue<Impl: ISettingsItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetValue<Impl: ISettingsItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSettingType<Impl: ISettingsItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut WcmSettingType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDataType<Impl: ISettingsItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut WcmDataType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetValueRaw<Impl: ISettingsItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: *mut *mut u8, datasize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetValueRaw<Impl: ISettingsItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datatype: i32, data: *const u8, datasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HasChild<Impl: ISettingsItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemhaschild: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Children<Impl: ISettingsItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, children: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetChild<Impl: ISettingsItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, child: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSettingByPath<Impl: ISettingsItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: super::super::Foundation::PWSTR, setting: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSettingByPath<Impl: ISettingsItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: super::super::Foundation::PWSTR, setting: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveSettingByPath<Impl: ISettingsItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetListKeyInformation<Impl: ISettingsItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keyname: *mut super::super::Foundation::BSTR, datatype: *mut WcmDataType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateListElement<Impl: ISettingsItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keydata: *const super::Com::VARIANT, child: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveListElement<Impl: ISettingsItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, elementname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Attributes<Impl: ISettingsItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAttribute<Impl: ISettingsItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, value: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPath<Impl: ISettingsItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRestrictionFacets<Impl: ISettingsItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, restrictionfacets: *mut WcmRestrictionFacets) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRestriction<Impl: ISettingsItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, restrictionfacet: WcmRestrictionFacets, facetdata: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetKeyValue<Impl: ISettingsItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetName::<Impl, IMPL_OFFSET>,
            GetValue::<Impl, IMPL_OFFSET>,
            SetValue::<Impl, IMPL_OFFSET>,
            GetSettingType::<Impl, IMPL_OFFSET>,
            GetDataType::<Impl, IMPL_OFFSET>,
            GetValueRaw::<Impl, IMPL_OFFSET>,
            SetValueRaw::<Impl, IMPL_OFFSET>,
            HasChild::<Impl, IMPL_OFFSET>,
            Children::<Impl, IMPL_OFFSET>,
            GetChild::<Impl, IMPL_OFFSET>,
            GetSettingByPath::<Impl, IMPL_OFFSET>,
            CreateSettingByPath::<Impl, IMPL_OFFSET>,
            RemoveSettingByPath::<Impl, IMPL_OFFSET>,
            GetListKeyInformation::<Impl, IMPL_OFFSET>,
            CreateListElement::<Impl, IMPL_OFFSET>,
            RemoveListElement::<Impl, IMPL_OFFSET>,
            Attributes::<Impl, IMPL_OFFSET>,
            GetAttribute::<Impl, IMPL_OFFSET>,
            GetPath::<Impl, IMPL_OFFSET>,
            GetRestrictionFacets::<Impl, IMPL_OFFSET>,
            GetRestriction::<Impl, IMPL_OFFSET>,
            GetKeyValue::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISettingsItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISettingsNamespaceImpl: Sized {
    fn GetIdentity();
    fn Settings();
    fn Save();
    fn GetSettingByPath();
    fn CreateSettingByPath();
    fn RemoveSettingByPath();
    fn GetAttribute();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISettingsNamespaceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISettingsNamespaceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISettingsNamespaceVtbl {
        unsafe extern "system" fn GetIdentity<Impl: ISettingsNamespaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settingsid: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Settings<Impl: ISettingsNamespaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Save<Impl: ISettingsNamespaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pushsettings: super::super::Foundation::BOOL, result: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSettingByPath<Impl: ISettingsNamespaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: super::super::Foundation::PWSTR, setting: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSettingByPath<Impl: ISettingsNamespaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: super::super::Foundation::PWSTR, setting: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveSettingByPath<Impl: ISettingsNamespaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAttribute<Impl: ISettingsNamespaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, value: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetIdentity::<Impl, IMPL_OFFSET>, Settings::<Impl, IMPL_OFFSET>, Save::<Impl, IMPL_OFFSET>, GetSettingByPath::<Impl, IMPL_OFFSET>, CreateSettingByPath::<Impl, IMPL_OFFSET>, RemoveSettingByPath::<Impl, IMPL_OFFSET>, GetAttribute::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISettingsNamespace as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISettingsResultImpl: Sized {
    fn GetDescription();
    fn GetErrorCode();
    fn GetContextDescription();
    fn GetLine();
    fn GetColumn();
    fn GetSource();
}
#[cfg(feature = "Win32_Foundation")]
impl ISettingsResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISettingsResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISettingsResultVtbl {
        unsafe extern "system" fn GetDescription<Impl: ISettingsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetErrorCode<Impl: ISettingsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrout: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetContextDescription<Impl: ISettingsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLine<Impl: ISettingsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwline: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetColumn<Impl: ISettingsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcolumn: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSource<Impl: ISettingsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDescription::<Impl, IMPL_OFFSET>, GetErrorCode::<Impl, IMPL_OFFSET>, GetContextDescription::<Impl, IMPL_OFFSET>, GetLine::<Impl, IMPL_OFFSET>, GetColumn::<Impl, IMPL_OFFSET>, GetSource::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISettingsResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITargetInfoImpl: Sized {
    fn GetTargetMode();
    fn SetTargetMode();
    fn GetTemporaryStoreLocation();
    fn SetTemporaryStoreLocation();
    fn GetTargetID();
    fn SetTargetID();
    fn GetTargetProcessorArchitecture();
    fn SetTargetProcessorArchitecture();
    fn GetProperty();
    fn SetProperty();
    fn GetEnumerator();
    fn ExpandTarget();
    fn ExpandTargetPath();
    fn SetModulePath();
    fn LoadModule();
    fn SetWow64Context();
    fn TranslateWow64();
    fn SetSchemaHiveLocation();
    fn GetSchemaHiveLocation();
    fn SetSchemaHiveMountName();
    fn GetSchemaHiveMountName();
}
#[cfg(feature = "Win32_Foundation")]
impl ITargetInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITargetInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITargetInfoVtbl {
        unsafe extern "system" fn GetTargetMode<Impl: ITargetInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetmode: *mut WcmTargetMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTargetMode<Impl: ITargetInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetmode: WcmTargetMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTemporaryStoreLocation<Impl: ITargetInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, temporarystorelocation: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTemporaryStoreLocation<Impl: ITargetInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, temporarystorelocation: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTargetID<Impl: ITargetInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTargetID<Impl: ITargetInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTargetProcessorArchitecture<Impl: ITargetInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, processorarchitecture: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTargetProcessorArchitecture<Impl: ITargetInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, processorarchitecture: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProperty<Impl: ITargetInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offline: super::super::Foundation::BOOL, property: super::super::Foundation::PWSTR, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProperty<Impl: ITargetInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offline: super::super::Foundation::BOOL, property: super::super::Foundation::PWSTR, value: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEnumerator<Impl: ITargetInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExpandTarget<Impl: ITargetInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offline: super::super::Foundation::BOOL, location: super::super::Foundation::PWSTR, expandedlocation: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExpandTargetPath<Impl: ITargetInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offline: super::super::Foundation::BOOL, location: super::super::Foundation::PWSTR, expandedlocation: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetModulePath<Impl: ITargetInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, module: super::super::Foundation::PWSTR, path: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LoadModule<Impl: ITargetInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, module: super::super::Foundation::PWSTR, modulehandle: *mut super::super::Foundation::HINSTANCE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetWow64Context<Impl: ITargetInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, installermodule: super::super::Foundation::PWSTR, wow64context: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TranslateWow64<Impl: ITargetInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientarchitecture: super::super::Foundation::PWSTR, value: super::super::Foundation::PWSTR, translatedvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSchemaHiveLocation<Impl: ITargetInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzhivedir: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSchemaHiveLocation<Impl: ITargetInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phivelocation: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSchemaHiveMountName<Impl: ITargetInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzmountname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSchemaHiveMountName<Impl: ITargetInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmountname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTargetMode::<Impl, IMPL_OFFSET>,
            SetTargetMode::<Impl, IMPL_OFFSET>,
            GetTemporaryStoreLocation::<Impl, IMPL_OFFSET>,
            SetTemporaryStoreLocation::<Impl, IMPL_OFFSET>,
            GetTargetID::<Impl, IMPL_OFFSET>,
            SetTargetID::<Impl, IMPL_OFFSET>,
            GetTargetProcessorArchitecture::<Impl, IMPL_OFFSET>,
            SetTargetProcessorArchitecture::<Impl, IMPL_OFFSET>,
            GetProperty::<Impl, IMPL_OFFSET>,
            SetProperty::<Impl, IMPL_OFFSET>,
            GetEnumerator::<Impl, IMPL_OFFSET>,
            ExpandTarget::<Impl, IMPL_OFFSET>,
            ExpandTargetPath::<Impl, IMPL_OFFSET>,
            SetModulePath::<Impl, IMPL_OFFSET>,
            LoadModule::<Impl, IMPL_OFFSET>,
            SetWow64Context::<Impl, IMPL_OFFSET>,
            TranslateWow64::<Impl, IMPL_OFFSET>,
            SetSchemaHiveLocation::<Impl, IMPL_OFFSET>,
            GetSchemaHiveLocation::<Impl, IMPL_OFFSET>,
            SetSchemaHiveMountName::<Impl, IMPL_OFFSET>,
            GetSchemaHiveMountName::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITargetInfo as ::windows::core::Interface>::IID
    }
}
