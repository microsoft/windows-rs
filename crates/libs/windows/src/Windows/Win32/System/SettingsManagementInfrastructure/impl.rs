#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IItemEnumerator_Impl: Sized {
    fn Current(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn MoveNext(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn Reset(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IItemEnumerator {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IItemEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IItemEnumerator_Impl, const OFFSET: isize>() -> IItemEnumerator_Vtbl {
        unsafe extern "system" fn Current<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IItemEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Current() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(item, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IItemEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemvalid: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(itemvalid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IItemEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Current: Current::<Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IItemEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISettingsContext_Impl: Sized {
    fn Serialize(&self, pstream: &::core::option::Option<super::Com::IStream>, ptarget: &::core::option::Option<ITargetInfo>) -> ::windows::core::Result<()>;
    fn Deserialize(&self, pstream: &::core::option::Option<super::Com::IStream>, ptarget: &::core::option::Option<ITargetInfo>, pppresults: *mut *mut ::core::option::Option<ISettingsResult>, pcresultcount: *mut usize) -> ::windows::core::Result<()>;
    fn SetUserData(&self, puserdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetUserData(&self) -> ::windows::core::Result<*mut ::core::ffi::c_void>;
    fn GetNamespaces(&self) -> ::windows::core::Result<IItemEnumerator>;
    fn GetStoredSettings(&self, pidentity: &::core::option::Option<ISettingsIdentity>, ppaddedsettings: *mut ::core::option::Option<IItemEnumerator>, ppmodifiedsettings: *mut ::core::option::Option<IItemEnumerator>, ppdeletedsettings: *mut ::core::option::Option<IItemEnumerator>) -> ::windows::core::Result<()>;
    fn RevertSetting(&self, pidentity: &::core::option::Option<ISettingsIdentity>, pwzsetting: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISettingsContext {}
#[cfg(feature = "Win32_System_Com")]
impl ISettingsContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsContext_Impl, const OFFSET: isize>() -> ISettingsContext_Vtbl {
        unsafe extern "system" fn Serialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, ptarget: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Serialize(::core::mem::transmute(&pstream), ::core::mem::transmute(&ptarget)).into()
        }
        unsafe extern "system" fn Deserialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, ptarget: *mut ::core::ffi::c_void, pppresults: *mut *mut *mut ::core::ffi::c_void, pcresultcount: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Deserialize(::core::mem::transmute(&pstream), ::core::mem::transmute(&ptarget), ::core::mem::transmute_copy(&pppresults), ::core::mem::transmute_copy(&pcresultcount)).into()
        }
        unsafe extern "system" fn SetUserData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puserdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUserData(::core::mem::transmute_copy(&puserdata)).into()
        }
        unsafe extern "system" fn GetUserData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puserdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUserData() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puserdata, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNamespaces<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnamespaceids: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNamespaces() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnamespaceids, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStoredSettings<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidentity: *mut ::core::ffi::c_void, ppaddedsettings: *mut *mut ::core::ffi::c_void, ppmodifiedsettings: *mut *mut ::core::ffi::c_void, ppdeletedsettings: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStoredSettings(::core::mem::transmute(&pidentity), ::core::mem::transmute_copy(&ppaddedsettings), ::core::mem::transmute_copy(&ppmodifiedsettings), ::core::mem::transmute_copy(&ppdeletedsettings)).into()
        }
        unsafe extern "system" fn RevertSetting<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidentity: *mut ::core::ffi::c_void, pwzsetting: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RevertSetting(::core::mem::transmute(&pidentity), ::core::mem::transmute(&pwzsetting)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Serialize: Serialize::<Identity, Impl, OFFSET>,
            Deserialize: Deserialize::<Identity, Impl, OFFSET>,
            SetUserData: SetUserData::<Identity, Impl, OFFSET>,
            GetUserData: GetUserData::<Identity, Impl, OFFSET>,
            GetNamespaces: GetNamespaces::<Identity, Impl, OFFSET>,
            GetStoredSettings: GetStoredSettings::<Identity, Impl, OFFSET>,
            RevertSetting: RevertSetting::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISettingsContext as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISettingsEngine_Impl: Sized {
    fn GetNamespaces(&self, flags: WcmNamespaceEnumerationFlags, reserved: *const ::core::ffi::c_void) -> ::windows::core::Result<IItemEnumerator>;
    fn GetNamespace(&self, settingsid: &::core::option::Option<ISettingsIdentity>, access: WcmNamespaceAccess, reserved: *const ::core::ffi::c_void) -> ::windows::core::Result<ISettingsNamespace>;
    fn GetErrorDescription(&self, hresult: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CreateSettingsIdentity(&self) -> ::windows::core::Result<ISettingsIdentity>;
    fn GetStoreStatus(&self, reserved: *const ::core::ffi::c_void) -> ::windows::core::Result<WcmUserStatus>;
    fn LoadStore(&self, flags: u32) -> ::windows::core::Result<()>;
    fn UnloadStore(&self, reserved: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn RegisterNamespace(&self, settingsid: &::core::option::Option<ISettingsIdentity>, stream: &::core::option::Option<super::Com::IStream>, pushsettings: super::super::Foundation::BOOL) -> ::windows::core::Result<super::Com::VARIANT>;
    fn UnregisterNamespace(&self, settingsid: &::core::option::Option<ISettingsIdentity>, removesettings: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn CreateTargetInfo(&self) -> ::windows::core::Result<ITargetInfo>;
    fn GetTargetInfo(&self) -> ::windows::core::Result<ITargetInfo>;
    fn SetTargetInfo(&self, target: &::core::option::Option<ITargetInfo>) -> ::windows::core::Result<()>;
    fn CreateSettingsContext(&self, flags: u32, reserved: *const ::core::ffi::c_void) -> ::windows::core::Result<ISettingsContext>;
    fn SetSettingsContext(&self, settingscontext: &::core::option::Option<ISettingsContext>) -> ::windows::core::Result<()>;
    fn ApplySettingsContext(&self, settingscontext: &::core::option::Option<ISettingsContext>, pppwzidentities: *mut *mut ::windows::core::PWSTR, pcidentities: *mut usize) -> ::windows::core::Result<()>;
    fn GetSettingsContext(&self) -> ::windows::core::Result<ISettingsContext>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISettingsEngine {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISettingsEngine_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsEngine_Impl, const OFFSET: isize>() -> ISettingsEngine_Vtbl {
        unsafe extern "system" fn GetNamespaces<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: WcmNamespaceEnumerationFlags, reserved: *const ::core::ffi::c_void, namespaces: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNamespaces(::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&reserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(namespaces, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNamespace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settingsid: *mut ::core::ffi::c_void, access: WcmNamespaceAccess, reserved: *const ::core::ffi::c_void, namespaceitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNamespace(::core::mem::transmute(&settingsid), ::core::mem::transmute_copy(&access), ::core::mem::transmute_copy(&reserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(namespaceitem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresult: i32, message: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetErrorDescription(::core::mem::transmute_copy(&hresult)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(message, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSettingsIdentity<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settingsid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateSettingsIdentity() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(settingsid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStoreStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reserved: *const ::core::ffi::c_void, status: *mut WcmUserStatus) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStoreStatus(::core::mem::transmute_copy(&reserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(status, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadStore<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LoadStore(::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn UnloadStore<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reserved: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnloadStore(::core::mem::transmute_copy(&reserved)).into()
        }
        unsafe extern "system" fn RegisterNamespace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settingsid: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, pushsettings: super::super::Foundation::BOOL, results: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RegisterNamespace(::core::mem::transmute(&settingsid), ::core::mem::transmute(&stream), ::core::mem::transmute_copy(&pushsettings)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(results, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterNamespace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settingsid: *mut ::core::ffi::c_void, removesettings: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterNamespace(::core::mem::transmute(&settingsid), ::core::mem::transmute_copy(&removesettings)).into()
        }
        unsafe extern "system" fn CreateTargetInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateTargetInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(target, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTargetInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTargetInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(target, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTargetInfo(::core::mem::transmute(&target)).into()
        }
        unsafe extern "system" fn CreateSettingsContext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: u32, reserved: *const ::core::ffi::c_void, settingscontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateSettingsContext(::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&reserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(settingscontext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSettingsContext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settingscontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSettingsContext(::core::mem::transmute(&settingscontext)).into()
        }
        unsafe extern "system" fn ApplySettingsContext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settingscontext: *mut ::core::ffi::c_void, pppwzidentities: *mut *mut ::windows::core::PWSTR, pcidentities: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ApplySettingsContext(::core::mem::transmute(&settingscontext), ::core::mem::transmute_copy(&pppwzidentities), ::core::mem::transmute_copy(&pcidentities)).into()
        }
        unsafe extern "system" fn GetSettingsContext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settingscontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSettingsContext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(settingscontext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetNamespaces: GetNamespaces::<Identity, Impl, OFFSET>,
            GetNamespace: GetNamespace::<Identity, Impl, OFFSET>,
            GetErrorDescription: GetErrorDescription::<Identity, Impl, OFFSET>,
            CreateSettingsIdentity: CreateSettingsIdentity::<Identity, Impl, OFFSET>,
            GetStoreStatus: GetStoreStatus::<Identity, Impl, OFFSET>,
            LoadStore: LoadStore::<Identity, Impl, OFFSET>,
            UnloadStore: UnloadStore::<Identity, Impl, OFFSET>,
            RegisterNamespace: RegisterNamespace::<Identity, Impl, OFFSET>,
            UnregisterNamespace: UnregisterNamespace::<Identity, Impl, OFFSET>,
            CreateTargetInfo: CreateTargetInfo::<Identity, Impl, OFFSET>,
            GetTargetInfo: GetTargetInfo::<Identity, Impl, OFFSET>,
            SetTargetInfo: SetTargetInfo::<Identity, Impl, OFFSET>,
            CreateSettingsContext: CreateSettingsContext::<Identity, Impl, OFFSET>,
            SetSettingsContext: SetSettingsContext::<Identity, Impl, OFFSET>,
            ApplySettingsContext: ApplySettingsContext::<Identity, Impl, OFFSET>,
            GetSettingsContext: GetSettingsContext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISettingsEngine as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISettingsIdentity_Impl: Sized {
    fn GetAttribute(&self, reserved: *const ::core::ffi::c_void, name: &::windows::core::PCWSTR) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetAttribute(&self, reserved: *const ::core::ffi::c_void, name: &::windows::core::PCWSTR, value: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetFlags(&self) -> ::windows::core::Result<u32>;
    fn SetFlags(&self, flags: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ISettingsIdentity {}
#[cfg(feature = "Win32_Foundation")]
impl ISettingsIdentity_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsIdentity_Impl, const OFFSET: isize>() -> ISettingsIdentity_Vtbl {
        unsafe extern "system" fn GetAttribute<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsIdentity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reserved: *const ::core::ffi::c_void, name: ::windows::core::PCWSTR, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAttribute(::core::mem::transmute_copy(&reserved), ::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttribute<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsIdentity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reserved: *const ::core::ffi::c_void, name: ::windows::core::PCWSTR, value: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAttribute(::core::mem::transmute_copy(&reserved), ::core::mem::transmute(&name), ::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn GetFlags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsIdentity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFlags() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsIdentity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFlags(::core::mem::transmute_copy(&flags)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetAttribute: GetAttribute::<Identity, Impl, OFFSET>,
            SetAttribute: SetAttribute::<Identity, Impl, OFFSET>,
            GetFlags: GetFlags::<Identity, Impl, OFFSET>,
            SetFlags: SetFlags::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISettingsIdentity as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISettingsItem_Impl: Sized {
    fn GetName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetValue(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetValue(&self, value: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GetSettingType(&self) -> ::windows::core::Result<WcmSettingType>;
    fn GetDataType(&self) -> ::windows::core::Result<WcmDataType>;
    fn GetValueRaw(&self, data: *mut *mut u8, datasize: *mut u32) -> ::windows::core::Result<()>;
    fn SetValueRaw(&self, datatype: i32, data: *const u8, datasize: u32) -> ::windows::core::Result<()>;
    fn HasChild(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn Children(&self) -> ::windows::core::Result<IItemEnumerator>;
    fn GetChild(&self, name: &::windows::core::PCWSTR) -> ::windows::core::Result<ISettingsItem>;
    fn GetSettingByPath(&self, path: &::windows::core::PCWSTR) -> ::windows::core::Result<ISettingsItem>;
    fn CreateSettingByPath(&self, path: &::windows::core::PCWSTR) -> ::windows::core::Result<ISettingsItem>;
    fn RemoveSettingByPath(&self, path: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetListKeyInformation(&self, keyname: *mut super::super::Foundation::BSTR, datatype: *mut WcmDataType) -> ::windows::core::Result<()>;
    fn CreateListElement(&self, keydata: *const super::Com::VARIANT) -> ::windows::core::Result<ISettingsItem>;
    fn RemoveListElement(&self, elementname: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn Attributes(&self) -> ::windows::core::Result<IItemEnumerator>;
    fn GetAttribute(&self, name: &::windows::core::PCWSTR) -> ::windows::core::Result<super::Com::VARIANT>;
    fn GetPath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetRestrictionFacets(&self) -> ::windows::core::Result<WcmRestrictionFacets>;
    fn GetRestriction(&self, restrictionfacet: WcmRestrictionFacets) -> ::windows::core::Result<super::Com::VARIANT>;
    fn GetKeyValue(&self) -> ::windows::core::Result<super::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISettingsItem {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISettingsItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: isize>() -> ISettingsItem_Vtbl {
        unsafe extern "system" fn GetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetValue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetValue(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetSettingType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut WcmSettingType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSettingType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(r#type, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut WcmDataType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDataType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(r#type, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValueRaw<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: *mut *mut u8, datasize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetValueRaw(::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&datasize)).into()
        }
        unsafe extern "system" fn SetValueRaw<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datatype: i32, data: *const u8, datasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetValueRaw(::core::mem::transmute_copy(&datatype), ::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&datasize)).into()
        }
        unsafe extern "system" fn HasChild<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemhaschild: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HasChild() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(itemhaschild, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Children<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, children: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Children() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(children, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChild<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, child: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetChild(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(child, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSettingByPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::windows::core::PCWSTR, setting: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSettingByPath(::core::mem::transmute(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(setting, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSettingByPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::windows::core::PCWSTR, setting: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateSettingByPath(::core::mem::transmute(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(setting, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSettingByPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveSettingByPath(::core::mem::transmute(&path)).into()
        }
        unsafe extern "system" fn GetListKeyInformation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keyname: *mut super::super::Foundation::BSTR, datatype: *mut WcmDataType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetListKeyInformation(::core::mem::transmute_copy(&keyname), ::core::mem::transmute_copy(&datatype)).into()
        }
        unsafe extern "system" fn CreateListElement<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keydata: *const super::Com::VARIANT, child: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateListElement(::core::mem::transmute_copy(&keydata)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(child, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveListElement<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, elementname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveListElement(::core::mem::transmute(&elementname)).into()
        }
        unsafe extern "system" fn Attributes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Attributes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attributes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttribute<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, value: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAttribute(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPath() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(path, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRestrictionFacets<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, restrictionfacets: *mut WcmRestrictionFacets) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRestrictionFacets() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(restrictionfacets, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRestriction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, restrictionfacet: WcmRestrictionFacets, facetdata: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRestriction(::core::mem::transmute_copy(&restrictionfacet)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(facetdata, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKeyValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetKeyValue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetValue: GetValue::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            GetSettingType: GetSettingType::<Identity, Impl, OFFSET>,
            GetDataType: GetDataType::<Identity, Impl, OFFSET>,
            GetValueRaw: GetValueRaw::<Identity, Impl, OFFSET>,
            SetValueRaw: SetValueRaw::<Identity, Impl, OFFSET>,
            HasChild: HasChild::<Identity, Impl, OFFSET>,
            Children: Children::<Identity, Impl, OFFSET>,
            GetChild: GetChild::<Identity, Impl, OFFSET>,
            GetSettingByPath: GetSettingByPath::<Identity, Impl, OFFSET>,
            CreateSettingByPath: CreateSettingByPath::<Identity, Impl, OFFSET>,
            RemoveSettingByPath: RemoveSettingByPath::<Identity, Impl, OFFSET>,
            GetListKeyInformation: GetListKeyInformation::<Identity, Impl, OFFSET>,
            CreateListElement: CreateListElement::<Identity, Impl, OFFSET>,
            RemoveListElement: RemoveListElement::<Identity, Impl, OFFSET>,
            Attributes: Attributes::<Identity, Impl, OFFSET>,
            GetAttribute: GetAttribute::<Identity, Impl, OFFSET>,
            GetPath: GetPath::<Identity, Impl, OFFSET>,
            GetRestrictionFacets: GetRestrictionFacets::<Identity, Impl, OFFSET>,
            GetRestriction: GetRestriction::<Identity, Impl, OFFSET>,
            GetKeyValue: GetKeyValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISettingsItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISettingsNamespace_Impl: Sized {
    fn GetIdentity(&self) -> ::windows::core::Result<ISettingsIdentity>;
    fn Settings(&self) -> ::windows::core::Result<IItemEnumerator>;
    fn Save(&self, pushsettings: super::super::Foundation::BOOL) -> ::windows::core::Result<ISettingsResult>;
    fn GetSettingByPath(&self, path: &::windows::core::PCWSTR) -> ::windows::core::Result<ISettingsItem>;
    fn CreateSettingByPath(&self, path: &::windows::core::PCWSTR) -> ::windows::core::Result<ISettingsItem>;
    fn RemoveSettingByPath(&self, path: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetAttribute(&self, name: &::windows::core::PCWSTR) -> ::windows::core::Result<super::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISettingsNamespace {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISettingsNamespace_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsNamespace_Impl, const OFFSET: isize>() -> ISettingsNamespace_Vtbl {
        unsafe extern "system" fn GetIdentity<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settingsid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIdentity() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(settingsid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Settings<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settings: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Settings() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(settings, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Save<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pushsettings: super::super::Foundation::BOOL, result: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Save(::core::mem::transmute_copy(&pushsettings)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSettingByPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::windows::core::PCWSTR, setting: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSettingByPath(::core::mem::transmute(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(setting, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSettingByPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::windows::core::PCWSTR, setting: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateSettingByPath(::core::mem::transmute(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(setting, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSettingByPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveSettingByPath(::core::mem::transmute(&path)).into()
        }
        unsafe extern "system" fn GetAttribute<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, value: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAttribute(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetIdentity: GetIdentity::<Identity, Impl, OFFSET>,
            Settings: Settings::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            GetSettingByPath: GetSettingByPath::<Identity, Impl, OFFSET>,
            CreateSettingByPath: CreateSettingByPath::<Identity, Impl, OFFSET>,
            RemoveSettingByPath: RemoveSettingByPath::<Identity, Impl, OFFSET>,
            GetAttribute: GetAttribute::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISettingsNamespace as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISettingsResult_Impl: Sized {
    fn GetDescription(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn GetContextDescription(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetLine(&self) -> ::windows::core::Result<u32>;
    fn GetColumn(&self) -> ::windows::core::Result<u32>;
    fn GetSource(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ISettingsResult {}
#[cfg(feature = "Win32_Foundation")]
impl ISettingsResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsResult_Impl, const OFFSET: isize>() -> ISettingsResult_Vtbl {
        unsafe extern "system" fn GetDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDescription() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(description, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorCode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrout: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetErrorCode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hrout, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContextDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetContextDescription() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(description, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLine<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwline: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLine() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dwline, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumn<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcolumn: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetColumn() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dwcolumn, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISettingsResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSource() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(file, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
            GetErrorCode: GetErrorCode::<Identity, Impl, OFFSET>,
            GetContextDescription: GetContextDescription::<Identity, Impl, OFFSET>,
            GetLine: GetLine::<Identity, Impl, OFFSET>,
            GetColumn: GetColumn::<Identity, Impl, OFFSET>,
            GetSource: GetSource::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISettingsResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITargetInfo_Impl: Sized {
    fn GetTargetMode(&self) -> ::windows::core::Result<WcmTargetMode>;
    fn SetTargetMode(&self, targetmode: WcmTargetMode) -> ::windows::core::Result<()>;
    fn GetTemporaryStoreLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetTemporaryStoreLocation(&self, temporarystorelocation: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetTargetID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetTargetID(&self, targetid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetTargetProcessorArchitecture(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetTargetProcessorArchitecture(&self, processorarchitecture: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetProperty(&self, offline: super::super::Foundation::BOOL, property: &::windows::core::PCWSTR) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetProperty(&self, offline: super::super::Foundation::BOOL, property: &::windows::core::PCWSTR, value: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetEnumerator(&self) -> ::windows::core::Result<IItemEnumerator>;
    fn ExpandTarget(&self, offline: super::super::Foundation::BOOL, location: &::windows::core::PCWSTR) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ExpandTargetPath(&self, offline: super::super::Foundation::BOOL, location: &::windows::core::PCWSTR) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetModulePath(&self, module: &::windows::core::PCWSTR, path: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn LoadModule(&self, module: &::windows::core::PCWSTR) -> ::windows::core::Result<super::super::Foundation::HINSTANCE>;
    fn SetWow64Context(&self, installermodule: &::windows::core::PCWSTR, wow64context: *const u8) -> ::windows::core::Result<()>;
    fn TranslateWow64(&self, clientarchitecture: &::windows::core::PCWSTR, value: &::windows::core::PCWSTR) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetSchemaHiveLocation(&self, pwzhivedir: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetSchemaHiveLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetSchemaHiveMountName(&self, pwzmountname: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetSchemaHiveMountName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ITargetInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ITargetInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: isize>() -> ITargetInfo_Vtbl {
        unsafe extern "system" fn GetTargetMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetmode: *mut WcmTargetMode) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTargetMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(targetmode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetmode: WcmTargetMode) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTargetMode(::core::mem::transmute_copy(&targetmode)).into()
        }
        unsafe extern "system" fn GetTemporaryStoreLocation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, temporarystorelocation: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTemporaryStoreLocation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(temporarystorelocation, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTemporaryStoreLocation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, temporarystorelocation: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTemporaryStoreLocation(::core::mem::transmute(&temporarystorelocation)).into()
        }
        unsafe extern "system" fn GetTargetID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTargetID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(targetid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTargetID(::core::mem::transmute(&targetid)).into()
        }
        unsafe extern "system" fn GetTargetProcessorArchitecture<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, processorarchitecture: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTargetProcessorArchitecture() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(processorarchitecture, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetProcessorArchitecture<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, processorarchitecture: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTargetProcessorArchitecture(::core::mem::transmute(&processorarchitecture)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offline: super::super::Foundation::BOOL, property: ::windows::core::PCWSTR, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperty(::core::mem::transmute_copy(&offline), ::core::mem::transmute(&property)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offline: super::super::Foundation::BOOL, property: ::windows::core::PCWSTR, value: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProperty(::core::mem::transmute_copy(&offline), ::core::mem::transmute(&property), ::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn GetEnumerator<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enumerator, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpandTarget<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offline: super::super::Foundation::BOOL, location: ::windows::core::PCWSTR, expandedlocation: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExpandTarget(::core::mem::transmute_copy(&offline), ::core::mem::transmute(&location)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(expandedlocation, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpandTargetPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offline: super::super::Foundation::BOOL, location: ::windows::core::PCWSTR, expandedlocation: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExpandTargetPath(::core::mem::transmute_copy(&offline), ::core::mem::transmute(&location)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(expandedlocation, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetModulePath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, module: ::windows::core::PCWSTR, path: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetModulePath(::core::mem::transmute(&module), ::core::mem::transmute(&path)).into()
        }
        unsafe extern "system" fn LoadModule<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, module: ::windows::core::PCWSTR, modulehandle: *mut super::super::Foundation::HINSTANCE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LoadModule(::core::mem::transmute(&module)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(modulehandle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWow64Context<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, installermodule: ::windows::core::PCWSTR, wow64context: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetWow64Context(::core::mem::transmute(&installermodule), ::core::mem::transmute_copy(&wow64context)).into()
        }
        unsafe extern "system" fn TranslateWow64<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientarchitecture: ::windows::core::PCWSTR, value: ::windows::core::PCWSTR, translatedvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TranslateWow64(::core::mem::transmute(&clientarchitecture), ::core::mem::transmute(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(translatedvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSchemaHiveLocation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzhivedir: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSchemaHiveLocation(::core::mem::transmute(&pwzhivedir)).into()
        }
        unsafe extern "system" fn GetSchemaHiveLocation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phivelocation: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSchemaHiveLocation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phivelocation, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSchemaHiveMountName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzmountname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSchemaHiveMountName(::core::mem::transmute(&pwzmountname)).into()
        }
        unsafe extern "system" fn GetSchemaHiveMountName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmountname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSchemaHiveMountName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmountname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetTargetMode: GetTargetMode::<Identity, Impl, OFFSET>,
            SetTargetMode: SetTargetMode::<Identity, Impl, OFFSET>,
            GetTemporaryStoreLocation: GetTemporaryStoreLocation::<Identity, Impl, OFFSET>,
            SetTemporaryStoreLocation: SetTemporaryStoreLocation::<Identity, Impl, OFFSET>,
            GetTargetID: GetTargetID::<Identity, Impl, OFFSET>,
            SetTargetID: SetTargetID::<Identity, Impl, OFFSET>,
            GetTargetProcessorArchitecture: GetTargetProcessorArchitecture::<Identity, Impl, OFFSET>,
            SetTargetProcessorArchitecture: SetTargetProcessorArchitecture::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            GetEnumerator: GetEnumerator::<Identity, Impl, OFFSET>,
            ExpandTarget: ExpandTarget::<Identity, Impl, OFFSET>,
            ExpandTargetPath: ExpandTargetPath::<Identity, Impl, OFFSET>,
            SetModulePath: SetModulePath::<Identity, Impl, OFFSET>,
            LoadModule: LoadModule::<Identity, Impl, OFFSET>,
            SetWow64Context: SetWow64Context::<Identity, Impl, OFFSET>,
            TranslateWow64: TranslateWow64::<Identity, Impl, OFFSET>,
            SetSchemaHiveLocation: SetSchemaHiveLocation::<Identity, Impl, OFFSET>,
            GetSchemaHiveLocation: GetSchemaHiveLocation::<Identity, Impl, OFFSET>,
            SetSchemaHiveMountName: SetSchemaHiveMountName::<Identity, Impl, OFFSET>,
            GetSchemaHiveMountName: GetSchemaHiveMountName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITargetInfo as ::windows::core::Interface>::IID
    }
}
