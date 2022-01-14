#[cfg(feature = "Win32_Foundation")]
pub trait IProtectionPolicyManagerInterop_Impl: Sized {
    fn RequestAccessForWindowAsync(&mut self, appwindow: super::super::Foundation::HWND, sourceidentity: ::windows::core::HSTRING, targetidentity: ::windows::core::HSTRING, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetForWindow(&mut self, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, result: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IProtectionPolicyManagerInterop {
    const NAME: &'static str = "";
}
#[cfg(feature = "Win32_Foundation")]
impl IProtectionPolicyManagerInterop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProtectionPolicyManagerInterop_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProtectionPolicyManagerInterop_Vtbl {
        unsafe extern "system" fn RequestAccessForWindowAsync<Impl: IProtectionPolicyManagerInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, targetidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestAccessForWindowAsync(::core::mem::transmute_copy(&appwindow), ::core::mem::transmute_copy(&sourceidentity), ::core::mem::transmute_copy(&targetidentity), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&asyncoperation)).into()
        }
        unsafe extern "system" fn GetForWindow<Impl: IProtectionPolicyManagerInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, result: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetForWindow(::core::mem::transmute_copy(&appwindow), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&result)).into()
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
pub trait IProtectionPolicyManagerInterop2_Impl: Sized {
    fn RequestAccessForAppWithWindowAsync(&mut self, appwindow: super::super::Foundation::HWND, sourceidentity: ::windows::core::HSTRING, apppackagefamilyname: ::windows::core::HSTRING, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn RequestAccessWithAuditingInfoForWindowAsync(&mut self, appwindow: super::super::Foundation::HWND, sourceidentity: ::windows::core::HSTRING, targetidentity: ::windows::core::HSTRING, auditinfounk: ::core::option::Option<::windows::core::IUnknown>, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn RequestAccessWithMessageForWindowAsync(&mut self, appwindow: super::super::Foundation::HWND, sourceidentity: ::windows::core::HSTRING, targetidentity: ::windows::core::HSTRING, auditinfounk: ::core::option::Option<::windows::core::IUnknown>, messagefromapp: ::windows::core::HSTRING, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn RequestAccessForAppWithAuditingInfoForWindowAsync(&mut self, appwindow: super::super::Foundation::HWND, sourceidentity: ::windows::core::HSTRING, apppackagefamilyname: ::windows::core::HSTRING, auditinfounk: ::core::option::Option<::windows::core::IUnknown>, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn RequestAccessForAppWithMessageForWindowAsync(&mut self, appwindow: super::super::Foundation::HWND, sourceidentity: ::windows::core::HSTRING, apppackagefamilyname: ::windows::core::HSTRING, auditinfounk: ::core::option::Option<::windows::core::IUnknown>, messagefromapp: ::windows::core::HSTRING, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IProtectionPolicyManagerInterop2 {
    const NAME: &'static str = "";
}
#[cfg(feature = "Win32_Foundation")]
impl IProtectionPolicyManagerInterop2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProtectionPolicyManagerInterop2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProtectionPolicyManagerInterop2_Vtbl {
        unsafe extern "system" fn RequestAccessForAppWithWindowAsync<Impl: IProtectionPolicyManagerInterop2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestAccessForAppWithWindowAsync(::core::mem::transmute_copy(&appwindow), ::core::mem::transmute_copy(&sourceidentity), ::core::mem::transmute_copy(&apppackagefamilyname), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&asyncoperation)).into()
        }
        unsafe extern "system" fn RequestAccessWithAuditingInfoForWindowAsync<Impl: IProtectionPolicyManagerInterop2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, targetidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, auditinfounk: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestAccessWithAuditingInfoForWindowAsync(::core::mem::transmute_copy(&appwindow), ::core::mem::transmute_copy(&sourceidentity), ::core::mem::transmute_copy(&targetidentity), ::core::mem::transmute(&auditinfounk), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&asyncoperation)).into()
        }
        unsafe extern "system" fn RequestAccessWithMessageForWindowAsync<Impl: IProtectionPolicyManagerInterop2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, targetidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, auditinfounk: *mut ::core::ffi::c_void, messagefromapp: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestAccessWithMessageForWindowAsync(::core::mem::transmute_copy(&appwindow), ::core::mem::transmute_copy(&sourceidentity), ::core::mem::transmute_copy(&targetidentity), ::core::mem::transmute(&auditinfounk), ::core::mem::transmute_copy(&messagefromapp), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&asyncoperation)).into()
        }
        unsafe extern "system" fn RequestAccessForAppWithAuditingInfoForWindowAsync<Impl: IProtectionPolicyManagerInterop2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, auditinfounk: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestAccessForAppWithAuditingInfoForWindowAsync(::core::mem::transmute_copy(&appwindow), ::core::mem::transmute_copy(&sourceidentity), ::core::mem::transmute_copy(&apppackagefamilyname), ::core::mem::transmute(&auditinfounk), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&asyncoperation)).into()
        }
        unsafe extern "system" fn RequestAccessForAppWithMessageForWindowAsync<Impl: IProtectionPolicyManagerInterop2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, auditinfounk: *mut ::core::ffi::c_void, messagefromapp: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestAccessForAppWithMessageForWindowAsync(::core::mem::transmute_copy(&appwindow), ::core::mem::transmute_copy(&sourceidentity), ::core::mem::transmute_copy(&apppackagefamilyname), ::core::mem::transmute(&auditinfounk), ::core::mem::transmute_copy(&messagefromapp), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&asyncoperation)).into()
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
pub trait IProtectionPolicyManagerInterop3_Impl: Sized {
    fn RequestAccessWithBehaviorForWindowAsync(&mut self, appwindow: super::super::Foundation::HWND, sourceidentity: ::windows::core::HSTRING, targetidentity: ::windows::core::HSTRING, auditinfounk: ::core::option::Option<::windows::core::IUnknown>, messagefromapp: ::windows::core::HSTRING, behavior: u32, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn RequestAccessForAppWithBehaviorForWindowAsync(&mut self, appwindow: super::super::Foundation::HWND, sourceidentity: ::windows::core::HSTRING, apppackagefamilyname: ::windows::core::HSTRING, auditinfounk: ::core::option::Option<::windows::core::IUnknown>, messagefromapp: ::windows::core::HSTRING, behavior: u32, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn RequestAccessToFilesForAppForWindowAsync(&mut self, appwindow: super::super::Foundation::HWND, sourceitemlistunk: ::core::option::Option<::windows::core::IUnknown>, apppackagefamilyname: ::windows::core::HSTRING, auditinfounk: ::core::option::Option<::windows::core::IUnknown>, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn RequestAccessToFilesForAppWithMessageAndBehaviorForWindowAsync(&mut self, appwindow: super::super::Foundation::HWND, sourceitemlistunk: ::core::option::Option<::windows::core::IUnknown>, apppackagefamilyname: ::windows::core::HSTRING, auditinfounk: ::core::option::Option<::windows::core::IUnknown>, messagefromapp: ::windows::core::HSTRING, behavior: u32, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn RequestAccessToFilesForProcessForWindowAsync(&mut self, appwindow: super::super::Foundation::HWND, sourceitemlistunk: ::core::option::Option<::windows::core::IUnknown>, processid: u32, auditinfounk: ::core::option::Option<::windows::core::IUnknown>, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn RequestAccessToFilesForProcessWithMessageAndBehaviorForWindowAsync(&mut self, appwindow: super::super::Foundation::HWND, sourceitemlistunk: ::core::option::Option<::windows::core::IUnknown>, processid: u32, auditinfounk: ::core::option::Option<::windows::core::IUnknown>, messagefromapp: ::windows::core::HSTRING, behavior: u32, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IProtectionPolicyManagerInterop3 {
    const NAME: &'static str = "";
}
#[cfg(feature = "Win32_Foundation")]
impl IProtectionPolicyManagerInterop3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProtectionPolicyManagerInterop3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProtectionPolicyManagerInterop3_Vtbl {
        unsafe extern "system" fn RequestAccessWithBehaviorForWindowAsync<Impl: IProtectionPolicyManagerInterop3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, targetidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, auditinfounk: *mut ::core::ffi::c_void, messagefromapp: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, behavior: u32, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestAccessWithBehaviorForWindowAsync(::core::mem::transmute_copy(&appwindow), ::core::mem::transmute_copy(&sourceidentity), ::core::mem::transmute_copy(&targetidentity), ::core::mem::transmute(&auditinfounk), ::core::mem::transmute_copy(&messagefromapp), ::core::mem::transmute_copy(&behavior), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&asyncoperation)).into()
        }
        unsafe extern "system" fn RequestAccessForAppWithBehaviorForWindowAsync<Impl: IProtectionPolicyManagerInterop3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, auditinfounk: *mut ::core::ffi::c_void, messagefromapp: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, behavior: u32, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestAccessForAppWithBehaviorForWindowAsync(::core::mem::transmute_copy(&appwindow), ::core::mem::transmute_copy(&sourceidentity), ::core::mem::transmute_copy(&apppackagefamilyname), ::core::mem::transmute(&auditinfounk), ::core::mem::transmute_copy(&messagefromapp), ::core::mem::transmute_copy(&behavior), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&asyncoperation)).into()
        }
        unsafe extern "system" fn RequestAccessToFilesForAppForWindowAsync<Impl: IProtectionPolicyManagerInterop3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceitemlistunk: *mut ::core::ffi::c_void, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, auditinfounk: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestAccessToFilesForAppForWindowAsync(::core::mem::transmute_copy(&appwindow), ::core::mem::transmute(&sourceitemlistunk), ::core::mem::transmute_copy(&apppackagefamilyname), ::core::mem::transmute(&auditinfounk), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&asyncoperation)).into()
        }
        unsafe extern "system" fn RequestAccessToFilesForAppWithMessageAndBehaviorForWindowAsync<Impl: IProtectionPolicyManagerInterop3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceitemlistunk: *mut ::core::ffi::c_void, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, auditinfounk: *mut ::core::ffi::c_void, messagefromapp: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, behavior: u32, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestAccessToFilesForAppWithMessageAndBehaviorForWindowAsync(::core::mem::transmute_copy(&appwindow), ::core::mem::transmute(&sourceitemlistunk), ::core::mem::transmute_copy(&apppackagefamilyname), ::core::mem::transmute(&auditinfounk), ::core::mem::transmute_copy(&messagefromapp), ::core::mem::transmute_copy(&behavior), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&asyncoperation)).into()
        }
        unsafe extern "system" fn RequestAccessToFilesForProcessForWindowAsync<Impl: IProtectionPolicyManagerInterop3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceitemlistunk: *mut ::core::ffi::c_void, processid: u32, auditinfounk: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestAccessToFilesForProcessForWindowAsync(::core::mem::transmute_copy(&appwindow), ::core::mem::transmute(&sourceitemlistunk), ::core::mem::transmute_copy(&processid), ::core::mem::transmute(&auditinfounk), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&asyncoperation)).into()
        }
        unsafe extern "system" fn RequestAccessToFilesForProcessWithMessageAndBehaviorForWindowAsync<Impl: IProtectionPolicyManagerInterop3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceitemlistunk: *mut ::core::ffi::c_void, processid: u32, auditinfounk: *mut ::core::ffi::c_void, messagefromapp: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, behavior: u32, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestAccessToFilesForProcessWithMessageAndBehaviorForWindowAsync(::core::mem::transmute_copy(&appwindow), ::core::mem::transmute(&sourceitemlistunk), ::core::mem::transmute_copy(&processid), ::core::mem::transmute(&auditinfounk), ::core::mem::transmute_copy(&messagefromapp), ::core::mem::transmute_copy(&behavior), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&asyncoperation)).into()
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
