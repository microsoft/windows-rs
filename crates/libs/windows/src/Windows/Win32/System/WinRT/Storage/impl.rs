pub trait IOplockBreakingHandler_Impl: Sized {
    fn OplockBreaking(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IOplockBreakingHandler {}
impl IOplockBreakingHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOplockBreakingHandler_Impl, const OFFSET: isize>() -> IOplockBreakingHandler_Vtbl {
        unsafe extern "system" fn OplockBreaking<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOplockBreakingHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OplockBreaking().into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OplockBreaking: OplockBreaking::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOplockBreakingHandler as ::windows::core::Interface>::IID
    }
}
pub trait IRandomAccessStreamFileAccessMode_Impl: Sized {
    fn GetMode(&self) -> ::windows::core::Result<u32>;
}
impl ::windows::core::RuntimeName for IRandomAccessStreamFileAccessMode {}
impl IRandomAccessStreamFileAccessMode_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRandomAccessStreamFileAccessMode_Impl, const OFFSET: isize>() -> IRandomAccessStreamFileAccessMode_Vtbl {
        unsafe extern "system" fn GetMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRandomAccessStreamFileAccessMode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fileaccessmode: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fileaccessmode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetMode: GetMode::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRandomAccessStreamFileAccessMode as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IStorageFolderHandleAccess_Impl: Sized {
    fn Create(&self, filename: &::windows::core::PCWSTR, creationoptions: HANDLE_CREATION_OPTIONS, accessoptions: HANDLE_ACCESS_OPTIONS, sharingoptions: HANDLE_SHARING_OPTIONS, options: HANDLE_OPTIONS, oplockbreakinghandler: &::core::option::Option<IOplockBreakingHandler>) -> ::windows::core::Result<super::super::super::Foundation::HANDLE>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IStorageFolderHandleAccess {}
#[cfg(feature = "Win32_Foundation")]
impl IStorageFolderHandleAccess_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStorageFolderHandleAccess_Impl, const OFFSET: isize>() -> IStorageFolderHandleAccess_Vtbl {
        unsafe extern "system" fn Create<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStorageFolderHandleAccess_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, creationoptions: HANDLE_CREATION_OPTIONS, accessoptions: HANDLE_ACCESS_OPTIONS, sharingoptions: HANDLE_SHARING_OPTIONS, options: HANDLE_OPTIONS, oplockbreakinghandler: *mut ::core::ffi::c_void, interophandle: *mut super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Create(::core::mem::transmute(&filename), ::core::mem::transmute_copy(&creationoptions), ::core::mem::transmute_copy(&accessoptions), ::core::mem::transmute_copy(&sharingoptions), ::core::mem::transmute_copy(&options), ::core::mem::transmute(&oplockbreakinghandler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(interophandle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Create: Create::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageFolderHandleAccess as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IStorageItemHandleAccess_Impl: Sized {
    fn Create(&self, accessoptions: HANDLE_ACCESS_OPTIONS, sharingoptions: HANDLE_SHARING_OPTIONS, options: HANDLE_OPTIONS, oplockbreakinghandler: &::core::option::Option<IOplockBreakingHandler>) -> ::windows::core::Result<super::super::super::Foundation::HANDLE>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IStorageItemHandleAccess {}
#[cfg(feature = "Win32_Foundation")]
impl IStorageItemHandleAccess_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemHandleAccess_Impl, const OFFSET: isize>() -> IStorageItemHandleAccess_Vtbl {
        unsafe extern "system" fn Create<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemHandleAccess_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accessoptions: HANDLE_ACCESS_OPTIONS, sharingoptions: HANDLE_SHARING_OPTIONS, options: HANDLE_OPTIONS, oplockbreakinghandler: *mut ::core::ffi::c_void, interophandle: *mut super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Create(::core::mem::transmute_copy(&accessoptions), ::core::mem::transmute_copy(&sharingoptions), ::core::mem::transmute_copy(&options), ::core::mem::transmute(&oplockbreakinghandler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(interophandle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Create: Create::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageItemHandleAccess as ::windows::core::Interface>::IID
    }
}
pub trait IUnbufferedFileHandleOplockCallback_Impl: Sized {
    fn OnBrokenCallback(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IUnbufferedFileHandleOplockCallback {}
impl IUnbufferedFileHandleOplockCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUnbufferedFileHandleOplockCallback_Impl, const OFFSET: isize>() -> IUnbufferedFileHandleOplockCallback_Vtbl {
        unsafe extern "system" fn OnBrokenCallback<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUnbufferedFileHandleOplockCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnBrokenCallback().into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnBrokenCallback: OnBrokenCallback::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUnbufferedFileHandleOplockCallback as ::windows::core::Interface>::IID
    }
}
pub trait IUnbufferedFileHandleProvider_Impl: Sized {
    fn OpenUnbufferedFileHandle(&self, oplockbreakcallback: &::core::option::Option<IUnbufferedFileHandleOplockCallback>) -> ::windows::core::Result<usize>;
    fn CloseUnbufferedFileHandle(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IUnbufferedFileHandleProvider {}
impl IUnbufferedFileHandleProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUnbufferedFileHandleProvider_Impl, const OFFSET: isize>() -> IUnbufferedFileHandleProvider_Vtbl {
        unsafe extern "system" fn OpenUnbufferedFileHandle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUnbufferedFileHandleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oplockbreakcallback: *mut ::core::ffi::c_void, filehandle: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenUnbufferedFileHandle(::core::mem::transmute(&oplockbreakcallback)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filehandle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseUnbufferedFileHandle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUnbufferedFileHandleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CloseUnbufferedFileHandle().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OpenUnbufferedFileHandle: OpenUnbufferedFileHandle::<Identity, Impl, OFFSET>,
            CloseUnbufferedFileHandle: CloseUnbufferedFileHandle::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUnbufferedFileHandleProvider as ::windows::core::Interface>::IID
    }
}
