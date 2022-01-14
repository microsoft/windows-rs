pub trait IOplockBreakingHandler_Impl: Sized {
    fn OplockBreaking(&mut self) -> ::windows::core::Result<()>;
}
impl IOplockBreakingHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOplockBreakingHandler_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOplockBreakingHandler_Vtbl {
        unsafe extern "system" fn OplockBreaking<Impl: IOplockBreakingHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OplockBreaking().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OplockBreaking: OplockBreaking::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOplockBreakingHandler as ::windows::core::Interface>::IID
    }
}
pub trait IRandomAccessStreamFileAccessMode_Impl: Sized {
    fn GetMode(&mut self) -> ::windows::core::Result<u32>;
}
impl IRandomAccessStreamFileAccessMode_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRandomAccessStreamFileAccessMode_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRandomAccessStreamFileAccessMode_Vtbl {
        unsafe extern "system" fn GetMode<Impl: IRandomAccessStreamFileAccessMode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fileaccessmode: *mut u32) -> ::windows::core::HRESULT {
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
pub trait IStorageFolderHandleAccess_Impl: Sized {
    fn Create(&mut self, filename: super::super::super::Foundation::PWSTR, creationoptions: HANDLE_CREATION_OPTIONS, accessoptions: HANDLE_ACCESS_OPTIONS, sharingoptions: HANDLE_SHARING_OPTIONS, options: HANDLE_OPTIONS, oplockbreakinghandler: ::core::option::Option<IOplockBreakingHandler>) -> ::windows::core::Result<super::super::super::Foundation::HANDLE>;
}
#[cfg(feature = "Win32_Foundation")]
impl IStorageFolderHandleAccess_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFolderHandleAccess_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageFolderHandleAccess_Vtbl {
        unsafe extern "system" fn Create<Impl: IStorageFolderHandleAccess_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, creationoptions: HANDLE_CREATION_OPTIONS, accessoptions: HANDLE_ACCESS_OPTIONS, sharingoptions: HANDLE_SHARING_OPTIONS, options: HANDLE_OPTIONS, oplockbreakinghandler: ::windows::core::RawPtr, interophandle: *mut super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
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
pub trait IStorageItemHandleAccess_Impl: Sized {
    fn Create(&mut self, accessoptions: HANDLE_ACCESS_OPTIONS, sharingoptions: HANDLE_SHARING_OPTIONS, options: HANDLE_OPTIONS, oplockbreakinghandler: ::core::option::Option<IOplockBreakingHandler>) -> ::windows::core::Result<super::super::super::Foundation::HANDLE>;
}
#[cfg(feature = "Win32_Foundation")]
impl IStorageItemHandleAccess_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItemHandleAccess_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageItemHandleAccess_Vtbl {
        unsafe extern "system" fn Create<Impl: IStorageItemHandleAccess_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accessoptions: HANDLE_ACCESS_OPTIONS, sharingoptions: HANDLE_SHARING_OPTIONS, options: HANDLE_OPTIONS, oplockbreakinghandler: ::windows::core::RawPtr, interophandle: *mut super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
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
pub trait IUnbufferedFileHandleOplockCallback_Impl: Sized {
    fn OnBrokenCallback(&mut self) -> ::windows::core::Result<()>;
}
impl IUnbufferedFileHandleOplockCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUnbufferedFileHandleOplockCallback_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUnbufferedFileHandleOplockCallback_Vtbl {
        unsafe extern "system" fn OnBrokenCallback<Impl: IUnbufferedFileHandleOplockCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnBrokenCallback().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnBrokenCallback: OnBrokenCallback::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUnbufferedFileHandleOplockCallback as ::windows::core::Interface>::IID
    }
}
pub trait IUnbufferedFileHandleProvider_Impl: Sized {
    fn OpenUnbufferedFileHandle(&mut self, oplockbreakcallback: ::core::option::Option<IUnbufferedFileHandleOplockCallback>) -> ::windows::core::Result<usize>;
    fn CloseUnbufferedFileHandle(&mut self) -> ::windows::core::Result<()>;
}
impl IUnbufferedFileHandleProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUnbufferedFileHandleProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUnbufferedFileHandleProvider_Vtbl {
        unsafe extern "system" fn OpenUnbufferedFileHandle<Impl: IUnbufferedFileHandleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oplockbreakcallback: ::windows::core::RawPtr, filehandle: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenUnbufferedFileHandle(::core::mem::transmute(&oplockbreakcallback)) {
                ::core::result::Result::Ok(ok__) => {
                    *filehandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseUnbufferedFileHandle<Impl: IUnbufferedFileHandleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
