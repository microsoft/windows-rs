#[cfg(feature = "Win32_Foundation")]
pub trait IProtectionPolicyManagerInteropImpl: Sized {
    fn RequestAccessForWindowAsync();
    fn GetForWindow();
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IProtectionPolicyManagerInterop {
    const NAME: &'static str = "";
}
#[cfg(feature = "Win32_Foundation")]
impl IProtectionPolicyManagerInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProtectionPolicyManagerInteropImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProtectionPolicyManagerInteropVtbl {
        unsafe extern "system" fn RequestAccessForWindowAsync<Impl: IProtectionPolicyManagerInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, targetidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetForWindow<Impl: IProtectionPolicyManagerInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, result: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IProtectionPolicyManagerInterop, BASE_OFFSET>(),
            RequestAccessForWindowAsync: RequestAccessForWindowAsync::<Impl, IMPL_OFFSET>,
            GetForWindow: GetForWindow::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProtectionPolicyManagerInterop as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IProtectionPolicyManagerInterop2Impl: Sized {
    fn RequestAccessForAppWithWindowAsync();
    fn RequestAccessWithAuditingInfoForWindowAsync();
    fn RequestAccessWithMessageForWindowAsync();
    fn RequestAccessForAppWithAuditingInfoForWindowAsync();
    fn RequestAccessForAppWithMessageForWindowAsync();
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IProtectionPolicyManagerInterop2 {
    const NAME: &'static str = "";
}
#[cfg(feature = "Win32_Foundation")]
impl IProtectionPolicyManagerInterop2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProtectionPolicyManagerInterop2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProtectionPolicyManagerInterop2Vtbl {
        unsafe extern "system" fn RequestAccessForAppWithWindowAsync<Impl: IProtectionPolicyManagerInterop2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequestAccessWithAuditingInfoForWindowAsync<Impl: IProtectionPolicyManagerInterop2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, targetidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, auditinfounk: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequestAccessWithMessageForWindowAsync<Impl: IProtectionPolicyManagerInterop2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, targetidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, auditinfounk: *mut ::core::ffi::c_void, messagefromapp: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequestAccessForAppWithAuditingInfoForWindowAsync<Impl: IProtectionPolicyManagerInterop2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, auditinfounk: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequestAccessForAppWithMessageForWindowAsync<Impl: IProtectionPolicyManagerInterop2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, auditinfounk: *mut ::core::ffi::c_void, messagefromapp: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IProtectionPolicyManagerInterop2, BASE_OFFSET>(),
            RequestAccessForAppWithWindowAsync: RequestAccessForAppWithWindowAsync::<Impl, IMPL_OFFSET>,
            RequestAccessWithAuditingInfoForWindowAsync: RequestAccessWithAuditingInfoForWindowAsync::<Impl, IMPL_OFFSET>,
            RequestAccessWithMessageForWindowAsync: RequestAccessWithMessageForWindowAsync::<Impl, IMPL_OFFSET>,
            RequestAccessForAppWithAuditingInfoForWindowAsync: RequestAccessForAppWithAuditingInfoForWindowAsync::<Impl, IMPL_OFFSET>,
            RequestAccessForAppWithMessageForWindowAsync: RequestAccessForAppWithMessageForWindowAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProtectionPolicyManagerInterop2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IProtectionPolicyManagerInterop3Impl: Sized {
    fn RequestAccessWithBehaviorForWindowAsync();
    fn RequestAccessForAppWithBehaviorForWindowAsync();
    fn RequestAccessToFilesForAppForWindowAsync();
    fn RequestAccessToFilesForAppWithMessageAndBehaviorForWindowAsync();
    fn RequestAccessToFilesForProcessForWindowAsync();
    fn RequestAccessToFilesForProcessWithMessageAndBehaviorForWindowAsync();
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IProtectionPolicyManagerInterop3 {
    const NAME: &'static str = "";
}
#[cfg(feature = "Win32_Foundation")]
impl IProtectionPolicyManagerInterop3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProtectionPolicyManagerInterop3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProtectionPolicyManagerInterop3Vtbl {
        unsafe extern "system" fn RequestAccessWithBehaviorForWindowAsync<Impl: IProtectionPolicyManagerInterop3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, targetidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, auditinfounk: *mut ::core::ffi::c_void, messagefromapp: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, behavior: u32, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequestAccessForAppWithBehaviorForWindowAsync<Impl: IProtectionPolicyManagerInterop3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, auditinfounk: *mut ::core::ffi::c_void, messagefromapp: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, behavior: u32, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequestAccessToFilesForAppForWindowAsync<Impl: IProtectionPolicyManagerInterop3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceitemlistunk: *mut ::core::ffi::c_void, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, auditinfounk: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequestAccessToFilesForAppWithMessageAndBehaviorForWindowAsync<Impl: IProtectionPolicyManagerInterop3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceitemlistunk: *mut ::core::ffi::c_void, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, auditinfounk: *mut ::core::ffi::c_void, messagefromapp: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, behavior: u32, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequestAccessToFilesForProcessForWindowAsync<Impl: IProtectionPolicyManagerInterop3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceitemlistunk: *mut ::core::ffi::c_void, processid: u32, auditinfounk: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequestAccessToFilesForProcessWithMessageAndBehaviorForWindowAsync<Impl: IProtectionPolicyManagerInterop3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceitemlistunk: *mut ::core::ffi::c_void, processid: u32, auditinfounk: *mut ::core::ffi::c_void, messagefromapp: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, behavior: u32, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IProtectionPolicyManagerInterop3, BASE_OFFSET>(),
            RequestAccessWithBehaviorForWindowAsync: RequestAccessWithBehaviorForWindowAsync::<Impl, IMPL_OFFSET>,
            RequestAccessForAppWithBehaviorForWindowAsync: RequestAccessForAppWithBehaviorForWindowAsync::<Impl, IMPL_OFFSET>,
            RequestAccessToFilesForAppForWindowAsync: RequestAccessToFilesForAppForWindowAsync::<Impl, IMPL_OFFSET>,
            RequestAccessToFilesForAppWithMessageAndBehaviorForWindowAsync: RequestAccessToFilesForAppWithMessageAndBehaviorForWindowAsync::<Impl, IMPL_OFFSET>,
            RequestAccessToFilesForProcessForWindowAsync: RequestAccessToFilesForProcessForWindowAsync::<Impl, IMPL_OFFSET>,
            RequestAccessToFilesForProcessWithMessageAndBehaviorForWindowAsync: RequestAccessToFilesForProcessWithMessageAndBehaviorForWindowAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProtectionPolicyManagerInterop3 as ::windows::core::Interface>::IID
    }
}
