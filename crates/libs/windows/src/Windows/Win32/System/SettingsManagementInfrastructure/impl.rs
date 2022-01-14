#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IItemEnumerator_Impl: Sized {
    fn Current(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn MoveNext(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IItemEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IItemEnumerator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IItemEnumerator_Vtbl {
        unsafe extern "system" fn Current<Impl: IItemEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Current() {
                ::core::result::Result::Ok(ok__) => {
                    *item = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Impl: IItemEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemvalid: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    *itemvalid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IItemEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Current: Current::<Impl, IMPL_OFFSET>,
            MoveNext: MoveNext::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IItemEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ISettingsContext_Impl: Sized {
    fn Serialize(&mut self, pstream: ::core::option::Option<super::Com::IStream>, ptarget: ::core::option::Option<ITargetInfo>) -> ::windows::core::Result<()>;
    fn Deserialize(&mut self, pstream: ::core::option::Option<super::Com::IStream>, ptarget: ::core::option::Option<ITargetInfo>, pppresults: *mut *mut ::core::option::Option<ISettingsResult>, pcresultcount: *mut usize) -> ::windows::core::Result<()>;
    fn SetUserData(&mut self, puserdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetUserData(&mut self, puserdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetNamespaces(&mut self) -> ::windows::core::Result<IItemEnumerator>;
    fn GetStoredSettings(&mut self, pidentity: ::core::option::Option<ISettingsIdentity>, ppaddedsettings: *mut ::core::option::Option<IItemEnumerator>, ppmodifiedsettings: *mut ::core::option::Option<IItemEnumerator>, ppdeletedsettings: *mut ::core::option::Option<IItemEnumerator>) -> ::windows::core::Result<()>;
    fn RevertSetting(&mut self, pidentity: ::core::option::Option<ISettingsIdentity>, pwzsetting: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ISettingsContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISettingsContext_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISettingsContext_Vtbl {
        unsafe extern "system" fn Serialize<Impl: ISettingsContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, ptarget: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Serialize(::core::mem::transmute(&pstream), ::core::mem::transmute(&ptarget)).into()
        }
        unsafe extern "system" fn Deserialize<Impl: ISettingsContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, ptarget: ::windows::core::RawPtr, pppresults: *mut *mut ::windows::core::RawPtr, pcresultcount: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Deserialize(::core::mem::transmute(&pstream), ::core::mem::transmute(&ptarget), ::core::mem::transmute_copy(&pppresults), ::core::mem::transmute_copy(&pcresultcount)).into()
        }
        unsafe extern "system" fn SetUserData<Impl: ISettingsContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puserdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUserData(::core::mem::transmute_copy(&puserdata)).into()
        }
        unsafe extern "system" fn GetUserData<Impl: ISettingsContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puserdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetUserData(::core::mem::transmute_copy(&puserdata)).into()
        }
        unsafe extern "system" fn GetNamespaces<Impl: ISettingsContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnamespaceids: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNamespaces() {
                ::core::result::Result::Ok(ok__) => {
                    *ppnamespaceids = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStoredSettings<Impl: ISettingsContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidentity: ::windows::core::RawPtr, ppaddedsettings: *mut ::windows::core::RawPtr, ppmodifiedsettings: *mut ::windows::core::RawPtr, ppdeletedsettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStoredSettings(::core::mem::transmute(&pidentity), ::core::mem::transmute_copy(&ppaddedsettings), ::core::mem::transmute_copy(&ppmodifiedsettings), ::core::mem::transmute_copy(&ppdeletedsettings)).into()
        }
        unsafe extern "system" fn RevertSetting<Impl: ISettingsContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidentity: ::windows::core::RawPtr, pwzsetting: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RevertSetting(::core::mem::transmute(&pidentity), ::core::mem::transmute_copy(&pwzsetting)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Serialize: Serialize::<Impl, IMPL_OFFSET>,
            Deserialize: Deserialize::<Impl, IMPL_OFFSET>,
            SetUserData: SetUserData::<Impl, IMPL_OFFSET>,
            GetUserData: GetUserData::<Impl, IMPL_OFFSET>,
            GetNamespaces: GetNamespaces::<Impl, IMPL_OFFSET>,
            GetStoredSettings: GetStoredSettings::<Impl, IMPL_OFFSET>,
            RevertSetting: RevertSetting::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISettingsContext as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISettingsEngine_Impl: Sized {
    fn GetNamespaces(&mut self, flags: WcmNamespaceEnumerationFlags, reserved: *const ::core::ffi::c_void) -> ::windows::core::Result<IItemEnumerator>;
    fn GetNamespace(&mut self, settingsid: ::core::option::Option<ISettingsIdentity>, access: WcmNamespaceAccess, reserved: *const ::core::ffi::c_void) -> ::windows::core::Result<ISettingsNamespace>;
    fn GetErrorDescription(&mut self, hresult: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CreateSettingsIdentity(&mut self) -> ::windows::core::Result<ISettingsIdentity>;
    fn GetStoreStatus(&mut self, reserved: *const ::core::ffi::c_void) -> ::windows::core::Result<WcmUserStatus>;
    fn LoadStore(&mut self, flags: u32) -> ::windows::core::Result<()>;
    fn UnloadStore(&mut self, reserved: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn RegisterNamespace(&mut self, settingsid: ::core::option::Option<ISettingsIdentity>, stream: ::core::option::Option<super::Com::IStream>, pushsettings: super::super::Foundation::BOOL) -> ::windows::core::Result<super::Com::VARIANT>;
    fn UnregisterNamespace(&mut self, settingsid: ::core::option::Option<ISettingsIdentity>, removesettings: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn CreateTargetInfo(&mut self) -> ::windows::core::Result<ITargetInfo>;
    fn GetTargetInfo(&mut self) -> ::windows::core::Result<ITargetInfo>;
    fn SetTargetInfo(&mut self, target: ::core::option::Option<ITargetInfo>) -> ::windows::core::Result<()>;
    fn CreateSettingsContext(&mut self, flags: u32, reserved: *const ::core::ffi::c_void) -> ::windows::core::Result<ISettingsContext>;
    fn SetSettingsContext(&mut self, settingscontext: ::core::option::Option<ISettingsContext>) -> ::windows::core::Result<()>;
    fn ApplySettingsContext(&mut self, settingscontext: ::core::option::Option<ISettingsContext>, pppwzidentities: *mut *mut super::super::Foundation::PWSTR, pcidentities: *mut usize) -> ::windows::core::Result<()>;
    fn GetSettingsContext(&mut self) -> ::windows::core::Result<ISettingsContext>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISettingsEngine_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISettingsEngine_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISettingsEngine_Vtbl {
        unsafe extern "system" fn GetNamespaces<Impl: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: WcmNamespaceEnumerationFlags, reserved: *const ::core::ffi::c_void, namespaces: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNamespaces(::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&reserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *namespaces = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNamespace<Impl: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settingsid: ::windows::core::RawPtr, access: WcmNamespaceAccess, reserved: *const ::core::ffi::c_void, namespaceitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNamespace(::core::mem::transmute(&settingsid), ::core::mem::transmute_copy(&access), ::core::mem::transmute_copy(&reserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *namespaceitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorDescription<Impl: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresult: i32, message: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetErrorDescription(::core::mem::transmute_copy(&hresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *message = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSettingsIdentity<Impl: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settingsid: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSettingsIdentity() {
                ::core::result::Result::Ok(ok__) => {
                    *settingsid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStoreStatus<Impl: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reserved: *const ::core::ffi::c_void, status: *mut WcmUserStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStoreStatus(::core::mem::transmute_copy(&reserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *status = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadStore<Impl: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LoadStore(::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn UnloadStore<Impl: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reserved: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnloadStore(::core::mem::transmute_copy(&reserved)).into()
        }
        unsafe extern "system" fn RegisterNamespace<Impl: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settingsid: ::windows::core::RawPtr, stream: ::windows::core::RawPtr, pushsettings: super::super::Foundation::BOOL, results: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterNamespace(::core::mem::transmute(&settingsid), ::core::mem::transmute(&stream), ::core::mem::transmute_copy(&pushsettings)) {
                ::core::result::Result::Ok(ok__) => {
                    *results = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterNamespace<Impl: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settingsid: ::windows::core::RawPtr, removesettings: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterNamespace(::core::mem::transmute(&settingsid), ::core::mem::transmute_copy(&removesettings)).into()
        }
        unsafe extern "system" fn CreateTargetInfo<Impl: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTargetInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *target = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTargetInfo<Impl: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTargetInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *target = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetInfo<Impl: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetInfo(::core::mem::transmute(&target)).into()
        }
        unsafe extern "system" fn CreateSettingsContext<Impl: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: u32, reserved: *const ::core::ffi::c_void, settingscontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSettingsContext(::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&reserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *settingscontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSettingsContext<Impl: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settingscontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSettingsContext(::core::mem::transmute(&settingscontext)).into()
        }
        unsafe extern "system" fn ApplySettingsContext<Impl: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settingscontext: ::windows::core::RawPtr, pppwzidentities: *mut *mut super::super::Foundation::PWSTR, pcidentities: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ApplySettingsContext(::core::mem::transmute(&settingscontext), ::core::mem::transmute_copy(&pppwzidentities), ::core::mem::transmute_copy(&pcidentities)).into()
        }
        unsafe extern "system" fn GetSettingsContext<Impl: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settingscontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSettingsContext() {
                ::core::result::Result::Ok(ok__) => {
                    *settingscontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetNamespaces: GetNamespaces::<Impl, IMPL_OFFSET>,
            GetNamespace: GetNamespace::<Impl, IMPL_OFFSET>,
            GetErrorDescription: GetErrorDescription::<Impl, IMPL_OFFSET>,
            CreateSettingsIdentity: CreateSettingsIdentity::<Impl, IMPL_OFFSET>,
            GetStoreStatus: GetStoreStatus::<Impl, IMPL_OFFSET>,
            LoadStore: LoadStore::<Impl, IMPL_OFFSET>,
            UnloadStore: UnloadStore::<Impl, IMPL_OFFSET>,
            RegisterNamespace: RegisterNamespace::<Impl, IMPL_OFFSET>,
            UnregisterNamespace: UnregisterNamespace::<Impl, IMPL_OFFSET>,
            CreateTargetInfo: CreateTargetInfo::<Impl, IMPL_OFFSET>,
            GetTargetInfo: GetTargetInfo::<Impl, IMPL_OFFSET>,
            SetTargetInfo: SetTargetInfo::<Impl, IMPL_OFFSET>,
            CreateSettingsContext: CreateSettingsContext::<Impl, IMPL_OFFSET>,
            SetSettingsContext: SetSettingsContext::<Impl, IMPL_OFFSET>,
            ApplySettingsContext: ApplySettingsContext::<Impl, IMPL_OFFSET>,
            GetSettingsContext: GetSettingsContext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISettingsEngine as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISettingsIdentity_Impl: Sized {
    fn GetAttribute(&mut self, reserved: *const ::core::ffi::c_void, name: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetAttribute(&mut self, reserved: *const ::core::ffi::c_void, name: super::super::Foundation::PWSTR, value: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetFlags(&mut self) -> ::windows::core::Result<u32>;
    fn SetFlags(&mut self, flags: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISettingsIdentity_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISettingsIdentity_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISettingsIdentity_Vtbl {
        unsafe extern "system" fn GetAttribute<Impl: ISettingsIdentity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reserved: *const ::core::ffi::c_void, name: super::super::Foundation::PWSTR, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAttribute(::core::mem::transmute_copy(&reserved), ::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttribute<Impl: ISettingsIdentity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reserved: *const ::core::ffi::c_void, name: super::super::Foundation::PWSTR, value: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAttribute(::core::mem::transmute_copy(&reserved), ::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetFlags<Impl: ISettingsIdentity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlags<Impl: ISettingsIdentity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFlags(::core::mem::transmute_copy(&flags)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetAttribute: GetAttribute::<Impl, IMPL_OFFSET>,
            SetAttribute: SetAttribute::<Impl, IMPL_OFFSET>,
            GetFlags: GetFlags::<Impl, IMPL_OFFSET>,
            SetFlags: SetFlags::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISettingsIdentity as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISettingsItem_Impl: Sized {
    fn GetName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetValue(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetValue(&mut self, value: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GetSettingType(&mut self) -> ::windows::core::Result<WcmSettingType>;
    fn GetDataType(&mut self) -> ::windows::core::Result<WcmDataType>;
    fn GetValueRaw(&mut self, data: *mut *mut u8, datasize: *mut u32) -> ::windows::core::Result<()>;
    fn SetValueRaw(&mut self, datatype: i32, data: *const u8, datasize: u32) -> ::windows::core::Result<()>;
    fn HasChild(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn Children(&mut self) -> ::windows::core::Result<IItemEnumerator>;
    fn GetChild(&mut self, name: super::super::Foundation::PWSTR) -> ::windows::core::Result<ISettingsItem>;
    fn GetSettingByPath(&mut self, path: super::super::Foundation::PWSTR) -> ::windows::core::Result<ISettingsItem>;
    fn CreateSettingByPath(&mut self, path: super::super::Foundation::PWSTR) -> ::windows::core::Result<ISettingsItem>;
    fn RemoveSettingByPath(&mut self, path: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetListKeyInformation(&mut self, keyname: *mut super::super::Foundation::BSTR, datatype: *mut WcmDataType) -> ::windows::core::Result<()>;
    fn CreateListElement(&mut self, keydata: *const super::Com::VARIANT) -> ::windows::core::Result<ISettingsItem>;
    fn RemoveListElement(&mut self, elementname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Attributes(&mut self) -> ::windows::core::Result<IItemEnumerator>;
    fn GetAttribute(&mut self, name: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::Com::VARIANT>;
    fn GetPath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetRestrictionFacets(&mut self) -> ::windows::core::Result<WcmRestrictionFacets>;
    fn GetRestriction(&mut self, restrictionfacet: WcmRestrictionFacets) -> ::windows::core::Result<super::Com::VARIANT>;
    fn GetKeyValue(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISettingsItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISettingsItem_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISettingsItem_Vtbl {
        unsafe extern "system" fn GetName<Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetValue() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetSettingType<Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut WcmSettingType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSettingType() {
                ::core::result::Result::Ok(ok__) => {
                    *r#type = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataType<Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut WcmDataType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDataType() {
                ::core::result::Result::Ok(ok__) => {
                    *r#type = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValueRaw<Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: *mut *mut u8, datasize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetValueRaw(::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&datasize)).into()
        }
        unsafe extern "system" fn SetValueRaw<Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datatype: i32, data: *const u8, datasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValueRaw(::core::mem::transmute_copy(&datatype), ::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&datasize)).into()
        }
        unsafe extern "system" fn HasChild<Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemhaschild: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasChild() {
                ::core::result::Result::Ok(ok__) => {
                    *itemhaschild = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Children<Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, children: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Children() {
                ::core::result::Result::Ok(ok__) => {
                    *children = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChild<Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, child: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChild(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *child = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSettingByPath<Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: super::super::Foundation::PWSTR, setting: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSettingByPath(::core::mem::transmute_copy(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    *setting = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSettingByPath<Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: super::super::Foundation::PWSTR, setting: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSettingByPath(::core::mem::transmute_copy(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    *setting = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSettingByPath<Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSettingByPath(::core::mem::transmute_copy(&path)).into()
        }
        unsafe extern "system" fn GetListKeyInformation<Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keyname: *mut super::super::Foundation::BSTR, datatype: *mut WcmDataType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetListKeyInformation(::core::mem::transmute_copy(&keyname), ::core::mem::transmute_copy(&datatype)).into()
        }
        unsafe extern "system" fn CreateListElement<Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keydata: *const super::Com::VARIANT, child: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateListElement(::core::mem::transmute_copy(&keydata)) {
                ::core::result::Result::Ok(ok__) => {
                    *child = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveListElement<Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, elementname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveListElement(::core::mem::transmute_copy(&elementname)).into()
        }
        unsafe extern "system" fn Attributes<Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Attributes() {
                ::core::result::Result::Ok(ok__) => {
                    *attributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttribute<Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, value: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAttribute(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPath<Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPath() {
                ::core::result::Result::Ok(ok__) => {
                    *path = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRestrictionFacets<Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, restrictionfacets: *mut WcmRestrictionFacets) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRestrictionFacets() {
                ::core::result::Result::Ok(ok__) => {
                    *restrictionfacets = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRestriction<Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, restrictionfacet: WcmRestrictionFacets, facetdata: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRestriction(::core::mem::transmute_copy(&restrictionfacet)) {
                ::core::result::Result::Ok(ok__) => {
                    *facetdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKeyValue<Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetKeyValue() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetName: GetName::<Impl, IMPL_OFFSET>,
            GetValue: GetValue::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
            GetSettingType: GetSettingType::<Impl, IMPL_OFFSET>,
            GetDataType: GetDataType::<Impl, IMPL_OFFSET>,
            GetValueRaw: GetValueRaw::<Impl, IMPL_OFFSET>,
            SetValueRaw: SetValueRaw::<Impl, IMPL_OFFSET>,
            HasChild: HasChild::<Impl, IMPL_OFFSET>,
            Children: Children::<Impl, IMPL_OFFSET>,
            GetChild: GetChild::<Impl, IMPL_OFFSET>,
            GetSettingByPath: GetSettingByPath::<Impl, IMPL_OFFSET>,
            CreateSettingByPath: CreateSettingByPath::<Impl, IMPL_OFFSET>,
            RemoveSettingByPath: RemoveSettingByPath::<Impl, IMPL_OFFSET>,
            GetListKeyInformation: GetListKeyInformation::<Impl, IMPL_OFFSET>,
            CreateListElement: CreateListElement::<Impl, IMPL_OFFSET>,
            RemoveListElement: RemoveListElement::<Impl, IMPL_OFFSET>,
            Attributes: Attributes::<Impl, IMPL_OFFSET>,
            GetAttribute: GetAttribute::<Impl, IMPL_OFFSET>,
            GetPath: GetPath::<Impl, IMPL_OFFSET>,
            GetRestrictionFacets: GetRestrictionFacets::<Impl, IMPL_OFFSET>,
            GetRestriction: GetRestriction::<Impl, IMPL_OFFSET>,
            GetKeyValue: GetKeyValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISettingsItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISettingsNamespace_Impl: Sized {
    fn GetIdentity(&mut self) -> ::windows::core::Result<ISettingsIdentity>;
    fn Settings(&mut self) -> ::windows::core::Result<IItemEnumerator>;
    fn Save(&mut self, pushsettings: super::super::Foundation::BOOL) -> ::windows::core::Result<ISettingsResult>;
    fn GetSettingByPath(&mut self, path: super::super::Foundation::PWSTR) -> ::windows::core::Result<ISettingsItem>;
    fn CreateSettingByPath(&mut self, path: super::super::Foundation::PWSTR) -> ::windows::core::Result<ISettingsItem>;
    fn RemoveSettingByPath(&mut self, path: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetAttribute(&mut self, name: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISettingsNamespace_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISettingsNamespace_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISettingsNamespace_Vtbl {
        unsafe extern "system" fn GetIdentity<Impl: ISettingsNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settingsid: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIdentity() {
                ::core::result::Result::Ok(ok__) => {
                    *settingsid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Settings<Impl: ISettingsNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Settings() {
                ::core::result::Result::Ok(ok__) => {
                    *settings = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Save<Impl: ISettingsNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pushsettings: super::super::Foundation::BOOL, result: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Save(::core::mem::transmute_copy(&pushsettings)) {
                ::core::result::Result::Ok(ok__) => {
                    *result = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSettingByPath<Impl: ISettingsNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: super::super::Foundation::PWSTR, setting: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSettingByPath(::core::mem::transmute_copy(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    *setting = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSettingByPath<Impl: ISettingsNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: super::super::Foundation::PWSTR, setting: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSettingByPath(::core::mem::transmute_copy(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    *setting = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSettingByPath<Impl: ISettingsNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSettingByPath(::core::mem::transmute_copy(&path)).into()
        }
        unsafe extern "system" fn GetAttribute<Impl: ISettingsNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, value: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAttribute(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetIdentity: GetIdentity::<Impl, IMPL_OFFSET>,
            Settings: Settings::<Impl, IMPL_OFFSET>,
            Save: Save::<Impl, IMPL_OFFSET>,
            GetSettingByPath: GetSettingByPath::<Impl, IMPL_OFFSET>,
            CreateSettingByPath: CreateSettingByPath::<Impl, IMPL_OFFSET>,
            RemoveSettingByPath: RemoveSettingByPath::<Impl, IMPL_OFFSET>,
            GetAttribute: GetAttribute::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISettingsNamespace as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISettingsResult_Impl: Sized {
    fn GetDescription(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetErrorCode(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn GetContextDescription(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetLine(&mut self) -> ::windows::core::Result<u32>;
    fn GetColumn(&mut self) -> ::windows::core::Result<u32>;
    fn GetSource(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISettingsResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISettingsResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISettingsResult_Vtbl {
        unsafe extern "system" fn GetDescription<Impl: ISettingsResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *description = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorCode<Impl: ISettingsResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrout: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetErrorCode() {
                ::core::result::Result::Ok(ok__) => {
                    *hrout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContextDescription<Impl: ISettingsResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContextDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *description = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLine<Impl: ISettingsResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwline: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLine() {
                ::core::result::Result::Ok(ok__) => {
                    *dwline = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumn<Impl: ISettingsResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcolumn: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetColumn() {
                ::core::result::Result::Ok(ok__) => {
                    *dwcolumn = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSource<Impl: ISettingsResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSource() {
                ::core::result::Result::Ok(ok__) => {
                    *file = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetDescription: GetDescription::<Impl, IMPL_OFFSET>,
            GetErrorCode: GetErrorCode::<Impl, IMPL_OFFSET>,
            GetContextDescription: GetContextDescription::<Impl, IMPL_OFFSET>,
            GetLine: GetLine::<Impl, IMPL_OFFSET>,
            GetColumn: GetColumn::<Impl, IMPL_OFFSET>,
            GetSource: GetSource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISettingsResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITargetInfo_Impl: Sized {
    fn GetTargetMode(&mut self) -> ::windows::core::Result<WcmTargetMode>;
    fn SetTargetMode(&mut self, targetmode: WcmTargetMode) -> ::windows::core::Result<()>;
    fn GetTemporaryStoreLocation(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetTemporaryStoreLocation(&mut self, temporarystorelocation: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetTargetID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetTargetID(&mut self, targetid: ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetTargetProcessorArchitecture(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetTargetProcessorArchitecture(&mut self, processorarchitecture: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetProperty(&mut self, offline: super::super::Foundation::BOOL, property: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetProperty(&mut self, offline: super::super::Foundation::BOOL, property: super::super::Foundation::PWSTR, value: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetEnumerator(&mut self) -> ::windows::core::Result<IItemEnumerator>;
    fn ExpandTarget(&mut self, offline: super::super::Foundation::BOOL, location: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ExpandTargetPath(&mut self, offline: super::super::Foundation::BOOL, location: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetModulePath(&mut self, module: super::super::Foundation::PWSTR, path: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn LoadModule(&mut self, module: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::Foundation::HINSTANCE>;
    fn SetWow64Context(&mut self, installermodule: super::super::Foundation::PWSTR, wow64context: *const u8) -> ::windows::core::Result<()>;
    fn TranslateWow64(&mut self, clientarchitecture: super::super::Foundation::PWSTR, value: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetSchemaHiveLocation(&mut self, pwzhivedir: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetSchemaHiveLocation(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetSchemaHiveMountName(&mut self, pwzmountname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetSchemaHiveMountName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITargetInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITargetInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITargetInfo_Vtbl {
        unsafe extern "system" fn GetTargetMode<Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetmode: *mut WcmTargetMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTargetMode() {
                ::core::result::Result::Ok(ok__) => {
                    *targetmode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetMode<Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetmode: WcmTargetMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetMode(::core::mem::transmute_copy(&targetmode)).into()
        }
        unsafe extern "system" fn GetTemporaryStoreLocation<Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, temporarystorelocation: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTemporaryStoreLocation() {
                ::core::result::Result::Ok(ok__) => {
                    *temporarystorelocation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTemporaryStoreLocation<Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, temporarystorelocation: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTemporaryStoreLocation(::core::mem::transmute_copy(&temporarystorelocation)).into()
        }
        unsafe extern "system" fn GetTargetID<Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTargetID() {
                ::core::result::Result::Ok(ok__) => {
                    *targetid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetID<Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetID(::core::mem::transmute_copy(&targetid)).into()
        }
        unsafe extern "system" fn GetTargetProcessorArchitecture<Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, processorarchitecture: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTargetProcessorArchitecture() {
                ::core::result::Result::Ok(ok__) => {
                    *processorarchitecture = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetProcessorArchitecture<Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, processorarchitecture: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetProcessorArchitecture(::core::mem::transmute_copy(&processorarchitecture)).into()
        }
        unsafe extern "system" fn GetProperty<Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offline: super::super::Foundation::BOOL, property: super::super::Foundation::PWSTR, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&offline), ::core::mem::transmute_copy(&property)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offline: super::super::Foundation::BOOL, property: super::super::Foundation::PWSTR, value: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&offline), ::core::mem::transmute_copy(&property), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetEnumerator<Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    *enumerator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpandTarget<Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offline: super::super::Foundation::BOOL, location: super::super::Foundation::PWSTR, expandedlocation: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExpandTarget(::core::mem::transmute_copy(&offline), ::core::mem::transmute_copy(&location)) {
                ::core::result::Result::Ok(ok__) => {
                    *expandedlocation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpandTargetPath<Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offline: super::super::Foundation::BOOL, location: super::super::Foundation::PWSTR, expandedlocation: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExpandTargetPath(::core::mem::transmute_copy(&offline), ::core::mem::transmute_copy(&location)) {
                ::core::result::Result::Ok(ok__) => {
                    *expandedlocation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetModulePath<Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, module: super::super::Foundation::PWSTR, path: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetModulePath(::core::mem::transmute_copy(&module), ::core::mem::transmute_copy(&path)).into()
        }
        unsafe extern "system" fn LoadModule<Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, module: super::super::Foundation::PWSTR, modulehandle: *mut super::super::Foundation::HINSTANCE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadModule(::core::mem::transmute_copy(&module)) {
                ::core::result::Result::Ok(ok__) => {
                    *modulehandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWow64Context<Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, installermodule: super::super::Foundation::PWSTR, wow64context: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWow64Context(::core::mem::transmute_copy(&installermodule), ::core::mem::transmute_copy(&wow64context)).into()
        }
        unsafe extern "system" fn TranslateWow64<Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientarchitecture: super::super::Foundation::PWSTR, value: super::super::Foundation::PWSTR, translatedvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TranslateWow64(::core::mem::transmute_copy(&clientarchitecture), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *translatedvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSchemaHiveLocation<Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzhivedir: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSchemaHiveLocation(::core::mem::transmute_copy(&pwzhivedir)).into()
        }
        unsafe extern "system" fn GetSchemaHiveLocation<Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phivelocation: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSchemaHiveLocation() {
                ::core::result::Result::Ok(ok__) => {
                    *phivelocation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSchemaHiveMountName<Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzmountname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSchemaHiveMountName(::core::mem::transmute_copy(&pwzmountname)).into()
        }
        unsafe extern "system" fn GetSchemaHiveMountName<Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmountname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSchemaHiveMountName() {
                ::core::result::Result::Ok(ok__) => {
                    *pmountname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetTargetMode: GetTargetMode::<Impl, IMPL_OFFSET>,
            SetTargetMode: SetTargetMode::<Impl, IMPL_OFFSET>,
            GetTemporaryStoreLocation: GetTemporaryStoreLocation::<Impl, IMPL_OFFSET>,
            SetTemporaryStoreLocation: SetTemporaryStoreLocation::<Impl, IMPL_OFFSET>,
            GetTargetID: GetTargetID::<Impl, IMPL_OFFSET>,
            SetTargetID: SetTargetID::<Impl, IMPL_OFFSET>,
            GetTargetProcessorArchitecture: GetTargetProcessorArchitecture::<Impl, IMPL_OFFSET>,
            SetTargetProcessorArchitecture: SetTargetProcessorArchitecture::<Impl, IMPL_OFFSET>,
            GetProperty: GetProperty::<Impl, IMPL_OFFSET>,
            SetProperty: SetProperty::<Impl, IMPL_OFFSET>,
            GetEnumerator: GetEnumerator::<Impl, IMPL_OFFSET>,
            ExpandTarget: ExpandTarget::<Impl, IMPL_OFFSET>,
            ExpandTargetPath: ExpandTargetPath::<Impl, IMPL_OFFSET>,
            SetModulePath: SetModulePath::<Impl, IMPL_OFFSET>,
            LoadModule: LoadModule::<Impl, IMPL_OFFSET>,
            SetWow64Context: SetWow64Context::<Impl, IMPL_OFFSET>,
            TranslateWow64: TranslateWow64::<Impl, IMPL_OFFSET>,
            SetSchemaHiveLocation: SetSchemaHiveLocation::<Impl, IMPL_OFFSET>,
            GetSchemaHiveLocation: GetSchemaHiveLocation::<Impl, IMPL_OFFSET>,
            SetSchemaHiveMountName: SetSchemaHiveMountName::<Impl, IMPL_OFFSET>,
            GetSchemaHiveMountName: GetSchemaHiveMountName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITargetInfo as ::windows::core::Interface>::IID
    }
}
