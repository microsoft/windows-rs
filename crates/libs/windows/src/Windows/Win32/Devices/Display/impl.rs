pub trait ICloneViewHelperImpl: Sized {
    fn GetConnectedIDs();
    fn GetActiveTopology();
    fn SetActiveTopology();
    fn Commit();
}
impl ::windows::core::RuntimeName for ICloneViewHelper {
    const NAME: &'static str = "Windows.Win32.Devices.Display.ICloneViewHelper";
}
impl ICloneViewHelperVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICloneViewHelperImpl, const OFFSET: isize>() -> ICloneViewHelperVtbl {
        unsafe extern "system" fn GetConnectedIDs<Impl: ICloneViewHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszadaptorname: super::super::Foundation::PWSTR, pulcount: *mut u32, pulid: *mut u32, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConnectedIDs(&*(&wszadaptorname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), pulcount, pulid, ulflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActiveTopology<Impl: ICloneViewHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszadaptorname: super::super::Foundation::PWSTR, ulsourceid: u32, pulcount: *mut u32, pultargetid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActiveTopology(&*(&wszadaptorname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ulsourceid, pulcount, pultargetid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActiveTopology<Impl: ICloneViewHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszadaptorname: super::super::Foundation::PWSTR, ulsourceid: u32, ulcount: u32, pultargetid: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetActiveTopology(&*(&wszadaptorname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ulsourceid, ulcount, pultargetid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Commit<Impl: ICloneViewHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ffinalcall: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Commit(&*(&ffinalcall as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICloneViewHelper>, ::windows::core::GetTrustLevel, GetConnectedIDs::<Impl, OFFSET>, GetActiveTopology::<Impl, OFFSET>, SetActiveTopology::<Impl, OFFSET>, Commit::<Impl, OFFSET>)
    }
}
pub trait IViewHelperImpl: Sized {
    fn GetConnectedIDs();
    fn GetActiveTopology();
    fn SetActiveTopology();
    fn Commit();
    fn SetConfiguration();
    fn GetProceedOnNewConfiguration();
}
impl ::windows::core::RuntimeName for IViewHelper {
    const NAME: &'static str = "Windows.Win32.Devices.Display.IViewHelper";
}
impl IViewHelperVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IViewHelperImpl, const OFFSET: isize>() -> IViewHelperVtbl {
        unsafe extern "system" fn GetConnectedIDs<Impl: IViewHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszadaptorname: super::super::Foundation::PWSTR, pulcount: *mut u32, pulid: *mut u32, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConnectedIDs(&*(&wszadaptorname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), pulcount, pulid, ulflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActiveTopology<Impl: IViewHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszadaptorname: super::super::Foundation::PWSTR, ulsourceid: u32, pulcount: *mut u32, pultargetid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActiveTopology(&*(&wszadaptorname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ulsourceid, pulcount, pultargetid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActiveTopology<Impl: IViewHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszadaptorname: super::super::Foundation::PWSTR, ulsourceid: u32, ulcount: u32, pultargetid: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetActiveTopology(&*(&wszadaptorname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ulsourceid, ulcount, pultargetid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Commit<Impl: IViewHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Commit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConfiguration<Impl: IViewHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistream: ::windows::core::RawPtr, pulstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetConfiguration(&*(&pistream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pulstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProceedOnNewConfiguration<Impl: IViewHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProceedOnNewConfiguration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IViewHelper>, ::windows::core::GetTrustLevel, GetConnectedIDs::<Impl, OFFSET>, GetActiveTopology::<Impl, OFFSET>, SetActiveTopology::<Impl, OFFSET>, Commit::<Impl, OFFSET>, SetConfiguration::<Impl, OFFSET>, GetProceedOnNewConfiguration::<Impl, OFFSET>)
    }
}
