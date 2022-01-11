pub trait IOplockBreakingHandlerImpl: Sized {
    fn OplockBreaking();
}
impl IOplockBreakingHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOplockBreakingHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOplockBreakingHandlerVtbl {
        unsafe extern "system" fn OplockBreaking<Impl: IOplockBreakingHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OplockBreaking::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOplockBreakingHandler as ::windows::core::Interface>::IID
    }
}
pub trait IRandomAccessStreamFileAccessModeImpl: Sized {
    fn GetMode();
}
impl IRandomAccessStreamFileAccessModeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRandomAccessStreamFileAccessModeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRandomAccessStreamFileAccessModeVtbl {
        unsafe extern "system" fn GetMode<Impl: IRandomAccessStreamFileAccessModeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fileaccessmode: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetMode::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRandomAccessStreamFileAccessMode as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IStorageFolderHandleAccessImpl: Sized {
    fn Create();
}
#[cfg(feature = "Win32_Foundation")]
impl IStorageFolderHandleAccessVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFolderHandleAccessImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageFolderHandleAccessVtbl {
        unsafe extern "system" fn Create<Impl: IStorageFolderHandleAccessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, creationoptions: HANDLE_CREATION_OPTIONS, accessoptions: HANDLE_ACCESS_OPTIONS, sharingoptions: HANDLE_SHARING_OPTIONS, options: HANDLE_OPTIONS, oplockbreakinghandler: ::windows::core::RawPtr, interophandle: *mut super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Create::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageFolderHandleAccess as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IStorageItemHandleAccessImpl: Sized {
    fn Create();
}
#[cfg(feature = "Win32_Foundation")]
impl IStorageItemHandleAccessVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItemHandleAccessImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageItemHandleAccessVtbl {
        unsafe extern "system" fn Create<Impl: IStorageItemHandleAccessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accessoptions: HANDLE_ACCESS_OPTIONS, sharingoptions: HANDLE_SHARING_OPTIONS, options: HANDLE_OPTIONS, oplockbreakinghandler: ::windows::core::RawPtr, interophandle: *mut super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Create::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageItemHandleAccess as ::windows::core::Interface>::IID
    }
}
pub trait IUnbufferedFileHandleOplockCallbackImpl: Sized {
    fn OnBrokenCallback();
}
impl IUnbufferedFileHandleOplockCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUnbufferedFileHandleOplockCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUnbufferedFileHandleOplockCallbackVtbl {
        unsafe extern "system" fn OnBrokenCallback<Impl: IUnbufferedFileHandleOplockCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnBrokenCallback::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUnbufferedFileHandleOplockCallback as ::windows::core::Interface>::IID
    }
}
pub trait IUnbufferedFileHandleProviderImpl: Sized {
    fn OpenUnbufferedFileHandle();
    fn CloseUnbufferedFileHandle();
}
impl IUnbufferedFileHandleProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUnbufferedFileHandleProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUnbufferedFileHandleProviderVtbl {
        unsafe extern "system" fn OpenUnbufferedFileHandle<Impl: IUnbufferedFileHandleProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oplockbreakcallback: ::windows::core::RawPtr, filehandle: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CloseUnbufferedFileHandle<Impl: IUnbufferedFileHandleProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OpenUnbufferedFileHandle::<Impl, IMPL_OFFSET>, CloseUnbufferedFileHandle::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUnbufferedFileHandleProvider as ::windows::core::Interface>::IID
    }
}
