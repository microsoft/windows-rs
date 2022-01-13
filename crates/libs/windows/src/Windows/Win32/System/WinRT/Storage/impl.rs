pub trait IOplockBreakingHandlerImpl: Sized {
    fn OplockBreaking(&mut self) -> ::windows::core::Result<()>;
}
impl IOplockBreakingHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOplockBreakingHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOplockBreakingHandlerVtbl {
        unsafe extern "system" fn OplockBreaking<Impl: IOplockBreakingHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OplockBreaking().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OplockBreaking: OplockBreaking::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOplockBreakingHandler as ::windows::core::Interface>::IID
    }
}
pub trait IRandomAccessStreamFileAccessModeImpl: Sized {
    fn GetMode(&mut self) -> ::windows::core::Result<u32>;
}
impl IRandomAccessStreamFileAccessModeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRandomAccessStreamFileAccessModeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRandomAccessStreamFileAccessModeVtbl {
        unsafe extern "system" fn GetMode<Impl: IRandomAccessStreamFileAccessModeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fileaccessmode: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMode() {
                ::core::result::Result::Ok(ok__) => {
                    *fileaccessmode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetMode: GetMode::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRandomAccessStreamFileAccessMode as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IStorageFolderHandleAccessImpl: Sized {
    fn Create(&mut self, filename: super::super::super::Foundation::PWSTR, creationoptions: HANDLE_CREATION_OPTIONS, accessoptions: HANDLE_ACCESS_OPTIONS, sharingoptions: HANDLE_SHARING_OPTIONS, options: HANDLE_OPTIONS, oplockbreakinghandler: ::core::option::Option<IOplockBreakingHandler>) -> ::windows::core::Result<super::super::super::Foundation::HANDLE>;
}
#[cfg(feature = "Win32_Foundation")]
impl IStorageFolderHandleAccessVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFolderHandleAccessImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageFolderHandleAccessVtbl {
        unsafe extern "system" fn Create<Impl: IStorageFolderHandleAccessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, creationoptions: HANDLE_CREATION_OPTIONS, accessoptions: HANDLE_ACCESS_OPTIONS, sharingoptions: HANDLE_SHARING_OPTIONS, options: HANDLE_OPTIONS, oplockbreakinghandler: ::windows::core::RawPtr, interophandle: *mut super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(::core::mem::transmute_copy(&filename), ::core::mem::transmute_copy(&creationoptions), ::core::mem::transmute_copy(&accessoptions), ::core::mem::transmute_copy(&sharingoptions), ::core::mem::transmute_copy(&options), ::core::mem::transmute(&oplockbreakinghandler)) {
                ::core::result::Result::Ok(ok__) => {
                    *interophandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageFolderHandleAccess as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IStorageItemHandleAccessImpl: Sized {
    fn Create(&mut self, accessoptions: HANDLE_ACCESS_OPTIONS, sharingoptions: HANDLE_SHARING_OPTIONS, options: HANDLE_OPTIONS, oplockbreakinghandler: ::core::option::Option<IOplockBreakingHandler>) -> ::windows::core::Result<super::super::super::Foundation::HANDLE>;
}
#[cfg(feature = "Win32_Foundation")]
impl IStorageItemHandleAccessVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItemHandleAccessImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageItemHandleAccessVtbl {
        unsafe extern "system" fn Create<Impl: IStorageItemHandleAccessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accessoptions: HANDLE_ACCESS_OPTIONS, sharingoptions: HANDLE_SHARING_OPTIONS, options: HANDLE_OPTIONS, oplockbreakinghandler: ::windows::core::RawPtr, interophandle: *mut super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(::core::mem::transmute_copy(&accessoptions), ::core::mem::transmute_copy(&sharingoptions), ::core::mem::transmute_copy(&options), ::core::mem::transmute(&oplockbreakinghandler)) {
                ::core::result::Result::Ok(ok__) => {
                    *interophandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageItemHandleAccess as ::windows::core::Interface>::IID
    }
}
pub trait IUnbufferedFileHandleOplockCallbackImpl: Sized {
    fn OnBrokenCallback(&mut self) -> ::windows::core::Result<()>;
}
impl IUnbufferedFileHandleOplockCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUnbufferedFileHandleOplockCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUnbufferedFileHandleOplockCallbackVtbl {
        unsafe extern "system" fn OnBrokenCallback<Impl: IUnbufferedFileHandleOplockCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnBrokenCallback().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnBrokenCallback: OnBrokenCallback::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUnbufferedFileHandleOplockCallback as ::windows::core::Interface>::IID
    }
}
pub trait IUnbufferedFileHandleProviderImpl: Sized {
    fn OpenUnbufferedFileHandle(&mut self, oplockbreakcallback: ::core::option::Option<IUnbufferedFileHandleOplockCallback>) -> ::windows::core::Result<usize>;
    fn CloseUnbufferedFileHandle(&mut self) -> ::windows::core::Result<()>;
}
impl IUnbufferedFileHandleProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUnbufferedFileHandleProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUnbufferedFileHandleProviderVtbl {
        unsafe extern "system" fn OpenUnbufferedFileHandle<Impl: IUnbufferedFileHandleProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oplockbreakcallback: ::windows::core::RawPtr, filehandle: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenUnbufferedFileHandle(::core::mem::transmute(&oplockbreakcallback)) {
                ::core::result::Result::Ok(ok__) => {
                    *filehandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseUnbufferedFileHandle<Impl: IUnbufferedFileHandleProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CloseUnbufferedFileHandle().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OpenUnbufferedFileHandle: OpenUnbufferedFileHandle::<Impl, IMPL_OFFSET>,
            CloseUnbufferedFileHandle: CloseUnbufferedFileHandle::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUnbufferedFileHandleProvider as ::windows::core::Interface>::IID
    }
}
