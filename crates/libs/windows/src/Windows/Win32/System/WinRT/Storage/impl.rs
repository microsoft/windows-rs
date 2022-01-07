pub trait IOplockBreakingHandlerImpl: Sized {
    fn OplockBreaking();
}
impl ::windows::core::RuntimeName for IOplockBreakingHandler {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Storage.IOplockBreakingHandler";
}
impl IOplockBreakingHandlerVtbl {
    pub const fn new<Impl: IOplockBreakingHandlerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IOplockBreakingHandlerVtbl {
        unsafe extern "system" fn OplockBreaking<Impl: IOplockBreakingHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OplockBreaking() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IOplockBreakingHandler>, base.5, OplockBreaking::<Impl, OFFSET>)
    }
}
pub trait IRandomAccessStreamFileAccessModeImpl: Sized {
    fn GetMode();
}
impl ::windows::core::RuntimeName for IRandomAccessStreamFileAccessMode {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Storage.IRandomAccessStreamFileAccessMode";
}
impl IRandomAccessStreamFileAccessModeVtbl {
    pub const fn new<Impl: IRandomAccessStreamFileAccessModeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRandomAccessStreamFileAccessModeVtbl {
        unsafe extern "system" fn GetMode<Impl: IRandomAccessStreamFileAccessModeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fileaccessmode: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMode(::core::mem::transmute_copy(&fileaccessmode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRandomAccessStreamFileAccessMode>, base.5, GetMode::<Impl, OFFSET>)
    }
}
pub trait IStorageFolderHandleAccessImpl: Sized {
    fn Create();
}
impl ::windows::core::RuntimeName for IStorageFolderHandleAccess {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Storage.IStorageFolderHandleAccess";
}
impl IStorageFolderHandleAccessVtbl {
    pub const fn new<Impl: IStorageFolderHandleAccessImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IStorageFolderHandleAccessVtbl {
        unsafe extern "system" fn Create<Impl: IStorageFolderHandleAccessImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: super::super::super::Foundation::PWSTR, creationoptions: HANDLE_CREATION_OPTIONS, accessoptions: HANDLE_ACCESS_OPTIONS, sharingoptions: HANDLE_SHARING_OPTIONS, options: HANDLE_OPTIONS, oplockbreakinghandler: ::windows::core::RawPtr, interophandle: *mut super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&filename as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), creationoptions, accessoptions, sharingoptions, options, &*(&oplockbreakinghandler as *const <IOplockBreakingHandler as ::windows::core::Abi>::Abi as *const <IOplockBreakingHandler as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&interophandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IStorageFolderHandleAccess>, base.5, Create::<Impl, OFFSET>)
    }
}
pub trait IStorageItemHandleAccessImpl: Sized {
    fn Create();
}
impl ::windows::core::RuntimeName for IStorageItemHandleAccess {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Storage.IStorageItemHandleAccess";
}
impl IStorageItemHandleAccessVtbl {
    pub const fn new<Impl: IStorageItemHandleAccessImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IStorageItemHandleAccessVtbl {
        unsafe extern "system" fn Create<Impl: IStorageItemHandleAccessImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, accessoptions: HANDLE_ACCESS_OPTIONS, sharingoptions: HANDLE_SHARING_OPTIONS, options: HANDLE_OPTIONS, oplockbreakinghandler: ::windows::core::RawPtr, interophandle: *mut super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(accessoptions, sharingoptions, options, &*(&oplockbreakinghandler as *const <IOplockBreakingHandler as ::windows::core::Abi>::Abi as *const <IOplockBreakingHandler as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&interophandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IStorageItemHandleAccess>, base.5, Create::<Impl, OFFSET>)
    }
}
pub trait IUnbufferedFileHandleOplockCallbackImpl: Sized {
    fn OnBrokenCallback();
}
impl ::windows::core::RuntimeName for IUnbufferedFileHandleOplockCallback {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Storage.IUnbufferedFileHandleOplockCallback";
}
impl IUnbufferedFileHandleOplockCallbackVtbl {
    pub const fn new<Impl: IUnbufferedFileHandleOplockCallbackImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUnbufferedFileHandleOplockCallbackVtbl {
        unsafe extern "system" fn OnBrokenCallback<Impl: IUnbufferedFileHandleOplockCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnBrokenCallback() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUnbufferedFileHandleOplockCallback>, base.5, OnBrokenCallback::<Impl, OFFSET>)
    }
}
pub trait IUnbufferedFileHandleProviderImpl: Sized {
    fn OpenUnbufferedFileHandle();
    fn CloseUnbufferedFileHandle();
}
impl ::windows::core::RuntimeName for IUnbufferedFileHandleProvider {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Storage.IUnbufferedFileHandleProvider";
}
impl IUnbufferedFileHandleProviderVtbl {
    pub const fn new<Impl: IUnbufferedFileHandleProviderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUnbufferedFileHandleProviderVtbl {
        unsafe extern "system" fn OpenUnbufferedFileHandle<Impl: IUnbufferedFileHandleProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, oplockbreakcallback: ::windows::core::RawPtr, filehandle: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenUnbufferedFileHandle(&*(&oplockbreakcallback as *const <IUnbufferedFileHandleOplockCallback as ::windows::core::Abi>::Abi as *const <IUnbufferedFileHandleOplockCallback as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&filehandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseUnbufferedFileHandle<Impl: IUnbufferedFileHandleProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CloseUnbufferedFileHandle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUnbufferedFileHandleProvider>, base.5, OpenUnbufferedFileHandle::<Impl, OFFSET>, CloseUnbufferedFileHandle::<Impl, OFFSET>)
    }
}
