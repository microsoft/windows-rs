pub trait IEnumWIA_DEV_CAPSImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
    fn GetCount();
}
impl ::windows::core::RuntimeName for IEnumWIA_DEV_CAPS {
    const NAME: &'static str = "Windows.Win32.Devices.ImageAcquisition.IEnumWIA_DEV_CAPS";
}
impl IEnumWIA_DEV_CAPSVtbl {
    pub const fn new<Impl: IEnumWIA_DEV_CAPSImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumWIA_DEV_CAPSVtbl {
        unsafe extern "system" fn Next<Impl: IEnumWIA_DEV_CAPSImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut WIA_DEV_CAP, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(celt, &*(&rgelt as *const <WIA_DEV_CAP as ::windows::core::Abi>::Abi as *const <WIA_DEV_CAP as ::windows::core::DefaultType>::DefaultType), pceltfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumWIA_DEV_CAPSImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumWIA_DEV_CAPSImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumWIA_DEV_CAPSImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppienum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppienum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Impl: IEnumWIA_DEV_CAPSImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&pcelt)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumWIA_DEV_CAPS>, base.5, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>, GetCount::<Impl, OFFSET>)
    }
}
pub trait IEnumWIA_DEV_INFOImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
    fn GetCount();
}
impl ::windows::core::RuntimeName for IEnumWIA_DEV_INFO {
    const NAME: &'static str = "Windows.Win32.Devices.ImageAcquisition.IEnumWIA_DEV_INFO";
}
impl IEnumWIA_DEV_INFOVtbl {
    pub const fn new<Impl: IEnumWIA_DEV_INFOImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumWIA_DEV_INFOVtbl {
        unsafe extern "system" fn Next<Impl: IEnumWIA_DEV_INFOImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&rgelt), pceltfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumWIA_DEV_INFOImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumWIA_DEV_INFOImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumWIA_DEV_INFOImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppienum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppienum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Impl: IEnumWIA_DEV_INFOImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&celt)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumWIA_DEV_INFO>, base.5, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>, GetCount::<Impl, OFFSET>)
    }
}
pub trait IEnumWIA_FORMAT_INFOImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
    fn GetCount();
}
impl ::windows::core::RuntimeName for IEnumWIA_FORMAT_INFO {
    const NAME: &'static str = "Windows.Win32.Devices.ImageAcquisition.IEnumWIA_FORMAT_INFO";
}
impl IEnumWIA_FORMAT_INFOVtbl {
    pub const fn new<Impl: IEnumWIA_FORMAT_INFOImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumWIA_FORMAT_INFOVtbl {
        unsafe extern "system" fn Next<Impl: IEnumWIA_FORMAT_INFOImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut WIA_FORMAT_INFO, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(celt, &*(&rgelt as *const <WIA_FORMAT_INFO as ::windows::core::Abi>::Abi as *const <WIA_FORMAT_INFO as ::windows::core::DefaultType>::DefaultType), pceltfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumWIA_FORMAT_INFOImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumWIA_FORMAT_INFOImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumWIA_FORMAT_INFOImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppienum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppienum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Impl: IEnumWIA_FORMAT_INFOImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&pcelt)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumWIA_FORMAT_INFO>, base.5, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>, GetCount::<Impl, OFFSET>)
    }
}
pub trait IEnumWiaItemImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
    fn GetCount();
}
impl ::windows::core::RuntimeName for IEnumWiaItem {
    const NAME: &'static str = "Windows.Win32.Devices.ImageAcquisition.IEnumWiaItem";
}
impl IEnumWiaItemVtbl {
    pub const fn new<Impl: IEnumWiaItemImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumWiaItemVtbl {
        unsafe extern "system" fn Next<Impl: IEnumWiaItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppiwiaitem: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&ppiwiaitem), pceltfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumWiaItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumWiaItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumWiaItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppienum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppienum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Impl: IEnumWiaItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&celt)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumWiaItem>, base.5, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>, GetCount::<Impl, OFFSET>)
    }
}
pub trait IEnumWiaItem2Impl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
    fn GetCount();
}
impl ::windows::core::RuntimeName for IEnumWiaItem2 {
    const NAME: &'static str = "Windows.Win32.Devices.ImageAcquisition.IEnumWiaItem2";
}
impl IEnumWiaItem2Vtbl {
    pub const fn new<Impl: IEnumWiaItem2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumWiaItem2Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumWiaItem2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppiwiaitem2: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&ppiwiaitem2), pceltfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumWiaItem2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumWiaItem2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumWiaItem2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppienum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppienum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Impl: IEnumWiaItem2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&celt)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumWiaItem2>, base.5, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>, GetCount::<Impl, OFFSET>)
    }
}
pub trait IWiaAppErrorHandlerImpl: Sized {
    fn GetWindow();
    fn ReportStatus();
}
impl ::windows::core::RuntimeName for IWiaAppErrorHandler {
    const NAME: &'static str = "Windows.Win32.Devices.ImageAcquisition.IWiaAppErrorHandler";
}
impl IWiaAppErrorHandlerVtbl {
    pub const fn new<Impl: IWiaAppErrorHandlerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWiaAppErrorHandlerVtbl {
        unsafe extern "system" fn GetWindow<Impl: IWiaAppErrorHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetWindow(::core::mem::transmute_copy(&phwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportStatus<Impl: IWiaAppErrorHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, pwiaitem2: ::windows::core::RawPtr, hrstatus: ::windows::core::HRESULT, lpercentcomplete: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReportStatus(lflags, &*(&pwiaitem2 as *const <IWiaItem2 as ::windows::core::Abi>::Abi as *const <IWiaItem2 as ::windows::core::DefaultType>::DefaultType), hrstatus, lpercentcomplete) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWiaAppErrorHandler>, base.5, GetWindow::<Impl, OFFSET>, ReportStatus::<Impl, OFFSET>)
    }
}
pub trait IWiaDataCallbackImpl: Sized {
    fn BandedDataCallback();
}
impl ::windows::core::RuntimeName for IWiaDataCallback {
    const NAME: &'static str = "Windows.Win32.Devices.ImageAcquisition.IWiaDataCallback";
}
impl IWiaDataCallbackVtbl {
    pub const fn new<Impl: IWiaDataCallbackImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWiaDataCallbackVtbl {
        unsafe extern "system" fn BandedDataCallback<Impl: IWiaDataCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lmessage: i32, lstatus: i32, lpercentcomplete: i32, loffset: i32, llength: i32, lreserved: i32, lreslength: i32, pbbuffer: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BandedDataCallback(lmessage, lstatus, lpercentcomplete, loffset, llength, lreserved, lreslength, pbbuffer) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWiaDataCallback>, base.5, BandedDataCallback::<Impl, OFFSET>)
    }
}
pub trait IWiaDataTransferImpl: Sized {
    fn idtGetData();
    fn idtGetBandedData();
    fn idtQueryGetData();
    fn idtEnumWIA_FORMAT_INFO();
    fn idtGetExtendedTransferInfo();
}
impl ::windows::core::RuntimeName for IWiaDataTransfer {
    const NAME: &'static str = "Windows.Win32.Devices.ImageAcquisition.IWiaDataTransfer";
}
impl IWiaDataTransferVtbl {
    pub const fn new<Impl: IWiaDataTransferImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWiaDataTransferVtbl {
        unsafe extern "system" fn idtGetData<Impl: IWiaDataTransferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmedium: *mut super::super::System::Com::STGMEDIUM, piwiadatacallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).idtGetData(&*(&pmedium as *const <super::super::System::Com::STGMEDIUM as ::windows::core::Abi>::Abi as *const <super::super::System::Com::STGMEDIUM as ::windows::core::DefaultType>::DefaultType), &*(&piwiadatacallback as *const <IWiaDataCallback as ::windows::core::Abi>::Abi as *const <IWiaDataCallback as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn idtGetBandedData<Impl: IWiaDataTransferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwiadatatransinfo: *mut WIA_DATA_TRANSFER_INFO, piwiadatacallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).idtGetBandedData(&*(&pwiadatatransinfo as *const <WIA_DATA_TRANSFER_INFO as ::windows::core::Abi>::Abi as *const <WIA_DATA_TRANSFER_INFO as ::windows::core::DefaultType>::DefaultType), &*(&piwiadatacallback as *const <IWiaDataCallback as ::windows::core::Abi>::Abi as *const <IWiaDataCallback as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn idtQueryGetData<Impl: IWiaDataTransferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfe: *const WIA_FORMAT_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).idtQueryGetData(&*(&pfe as *const <WIA_FORMAT_INFO as ::windows::core::Abi>::Abi as *const <WIA_FORMAT_INFO as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn idtEnumWIA_FORMAT_INFO<Impl: IWiaDataTransferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).idtEnumWIA_FORMAT_INFO(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn idtGetExtendedTransferInfo<Impl: IWiaDataTransferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pextendedtransferinfo: *mut WIA_EXTENDED_TRANSFER_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).idtGetExtendedTransferInfo(::core::mem::transmute_copy(&pextendedtransferinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWiaDataTransfer>, base.5, idtGetData::<Impl, OFFSET>, idtGetBandedData::<Impl, OFFSET>, idtQueryGetData::<Impl, OFFSET>, idtEnumWIA_FORMAT_INFO::<Impl, OFFSET>, idtGetExtendedTransferInfo::<Impl, OFFSET>)
    }
}
pub trait IWiaDevMgrImpl: Sized {
    fn EnumDeviceInfo();
    fn CreateDevice();
    fn SelectDeviceDlg();
    fn SelectDeviceDlgID();
    fn GetImageDlg();
    fn RegisterEventCallbackProgram();
    fn RegisterEventCallbackInterface();
    fn RegisterEventCallbackCLSID();
    fn AddDeviceDlg();
}
impl ::windows::core::RuntimeName for IWiaDevMgr {
    const NAME: &'static str = "Windows.Win32.Devices.ImageAcquisition.IWiaDevMgr";
}
impl IWiaDevMgrVtbl {
    pub const fn new<Impl: IWiaDevMgrImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWiaDevMgrVtbl {
        unsafe extern "system" fn EnumDeviceInfo<Impl: IWiaDevMgrImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflag: i32, ppienum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumDeviceInfo(lflag, ::core::mem::transmute_copy(&ppienum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDevice<Impl: IWiaDevMgrImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppwiaitemroot: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDevice(&*(&bstrdeviceid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppwiaitemroot)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectDeviceDlg<Impl: IWiaDevMgrImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut super::super::Foundation::BSTR, ppitemroot: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectDeviceDlg(&*(&hwndparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), ldevicetype, lflags, &*(&pbstrdeviceid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppitemroot)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectDeviceDlgID<Impl: IWiaDevMgrImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectDeviceDlgID(&*(&hwndparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), ldevicetype, lflags, &*(&pbstrdeviceid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImageDlg<Impl: IWiaDevMgrImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, lintent: i32, pitemroot: ::windows::core::RawPtr, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pguidformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetImageDlg(
                &*(&hwndparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                ldevicetype,
                lflags,
                lintent,
                &*(&pitemroot as *const <IWiaItem as ::windows::core::Abi>::Abi as *const <IWiaItem as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrfilename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pguidformat as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterEventCallbackProgram<Impl: IWiaDevMgrImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, peventguid: *const ::windows::core::GUID, bstrcommandline: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstricon: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RegisterEventCallbackProgram(
                lflags,
                &*(&bstrdeviceid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&peventguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrcommandline as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrdescription as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstricon as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterEventCallbackInterface<Impl: IWiaDevMgrImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, peventguid: *const ::windows::core::GUID, piwiaeventcallback: ::windows::core::RawPtr, peventobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RegisterEventCallbackInterface(
                lflags,
                &*(&bstrdeviceid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&peventguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&piwiaeventcallback as *const <IWiaEventCallback as ::windows::core::Abi>::Abi as *const <IWiaEventCallback as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&peventobject),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterEventCallbackCLSID<Impl: IWiaDevMgrImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, peventguid: *const ::windows::core::GUID, pclsid: *const ::windows::core::GUID, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstricon: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RegisterEventCallbackCLSID(
                lflags,
                &*(&bstrdeviceid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&peventguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&pclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrdescription as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstricon as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddDeviceDlg<Impl: IWiaDevMgrImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddDeviceDlg(&*(&hwndparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), lflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWiaDevMgr>, base.5, EnumDeviceInfo::<Impl, OFFSET>, CreateDevice::<Impl, OFFSET>, SelectDeviceDlg::<Impl, OFFSET>, SelectDeviceDlgID::<Impl, OFFSET>, GetImageDlg::<Impl, OFFSET>, RegisterEventCallbackProgram::<Impl, OFFSET>, RegisterEventCallbackInterface::<Impl, OFFSET>, RegisterEventCallbackCLSID::<Impl, OFFSET>, AddDeviceDlg::<Impl, OFFSET>)
    }
}
pub trait IWiaDevMgr2Impl: Sized {
    fn EnumDeviceInfo();
    fn CreateDevice();
    fn SelectDeviceDlg();
    fn SelectDeviceDlgID();
    fn RegisterEventCallbackInterface();
    fn RegisterEventCallbackProgram();
    fn RegisterEventCallbackCLSID();
    fn GetImageDlg();
}
impl ::windows::core::RuntimeName for IWiaDevMgr2 {
    const NAME: &'static str = "Windows.Win32.Devices.ImageAcquisition.IWiaDevMgr2";
}
impl IWiaDevMgr2Vtbl {
    pub const fn new<Impl: IWiaDevMgr2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWiaDevMgr2Vtbl {
        unsafe extern "system" fn EnumDeviceInfo<Impl: IWiaDevMgr2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, ppienum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumDeviceInfo(lflags, ::core::mem::transmute_copy(&ppienum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDevice<Impl: IWiaDevMgr2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppwiaitem2root: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDevice(lflags, &*(&bstrdeviceid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppwiaitem2root)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectDeviceDlg<Impl: IWiaDevMgr2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut super::super::Foundation::BSTR, ppitemroot: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectDeviceDlg(&*(&hwndparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), ldevicetype, lflags, &*(&pbstrdeviceid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppitemroot)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectDeviceDlgID<Impl: IWiaDevMgr2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectDeviceDlgID(&*(&hwndparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), ldevicetype, lflags, &*(&pbstrdeviceid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterEventCallbackInterface<Impl: IWiaDevMgr2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, peventguid: *const ::windows::core::GUID, piwiaeventcallback: ::windows::core::RawPtr, peventobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RegisterEventCallbackInterface(
                lflags,
                &*(&bstrdeviceid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&peventguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&piwiaeventcallback as *const <IWiaEventCallback as ::windows::core::Abi>::Abi as *const <IWiaEventCallback as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&peventobject),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterEventCallbackProgram<Impl: IWiaDevMgr2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, peventguid: *const ::windows::core::GUID, bstrfullappname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrcommandlinearg: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstricon: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RegisterEventCallbackProgram(
                lflags,
                &*(&bstrdeviceid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&peventguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrfullappname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrcommandlinearg as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrdescription as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstricon as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterEventCallbackCLSID<Impl: IWiaDevMgr2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, peventguid: *const ::windows::core::GUID, pclsid: *const ::windows::core::GUID, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstricon: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RegisterEventCallbackCLSID(
                lflags,
                &*(&bstrdeviceid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&peventguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&pclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrdescription as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstricon as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImageDlg<Impl: IWiaDevMgr2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, hwndparent: super::super::Foundation::HWND, bstrfoldername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, plnumfiles: *mut i32, ppbstrfilepaths: *mut *mut super::super::Foundation::BSTR, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetImageDlg(
                lflags,
                &*(&bstrdeviceid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&hwndparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrfoldername as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrfilename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                plnumfiles,
                &*(&ppbstrfilepaths as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppitem),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWiaDevMgr2>, base.5, EnumDeviceInfo::<Impl, OFFSET>, CreateDevice::<Impl, OFFSET>, SelectDeviceDlg::<Impl, OFFSET>, SelectDeviceDlgID::<Impl, OFFSET>, RegisterEventCallbackInterface::<Impl, OFFSET>, RegisterEventCallbackProgram::<Impl, OFFSET>, RegisterEventCallbackCLSID::<Impl, OFFSET>, GetImageDlg::<Impl, OFFSET>)
    }
}
pub trait IWiaDrvItemImpl: Sized {
    fn GetItemFlags();
    fn GetDeviceSpecContext();
    fn GetFullItemName();
    fn GetItemName();
    fn AddItemToFolder();
    fn UnlinkItemTree();
    fn RemoveItemFromFolder();
    fn FindItemByName();
    fn FindChildItemByName();
    fn GetParentItem();
    fn GetFirstChildItem();
    fn GetNextSiblingItem();
    fn DumpItemData();
}
impl ::windows::core::RuntimeName for IWiaDrvItem {
    const NAME: &'static str = "Windows.Win32.Devices.ImageAcquisition.IWiaDrvItem";
}
impl IWiaDrvItemVtbl {
    pub const fn new<Impl: IWiaDrvItemImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWiaDrvItemVtbl {
        unsafe extern "system" fn GetItemFlags<Impl: IWiaDrvItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0000: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetItemFlags(::core::mem::transmute_copy(&__midl__iwiadrvitem0000)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSpecContext<Impl: IWiaDrvItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0001: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeviceSpecContext(::core::mem::transmute_copy(&__midl__iwiadrvitem0001)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFullItemName<Impl: IWiaDrvItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0002: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFullItemName(::core::mem::transmute_copy(&__midl__iwiadrvitem0002)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemName<Impl: IWiaDrvItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0003: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetItemName(::core::mem::transmute_copy(&__midl__iwiadrvitem0003)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddItemToFolder<Impl: IWiaDrvItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0004: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddItemToFolder(&*(&__midl__iwiadrvitem0004 as *const <IWiaDrvItem as ::windows::core::Abi>::Abi as *const <IWiaDrvItem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnlinkItemTree<Impl: IWiaDrvItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0005: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnlinkItemTree(__midl__iwiadrvitem0005) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveItemFromFolder<Impl: IWiaDrvItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0006: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveItemFromFolder(__midl__iwiadrvitem0006) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindItemByName<Impl: IWiaDrvItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0007: i32, __midl__iwiadrvitem0008: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, __midl__iwiadrvitem0009: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindItemByName(__midl__iwiadrvitem0007, &*(&__midl__iwiadrvitem0008 as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&__midl__iwiadrvitem0009)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindChildItemByName<Impl: IWiaDrvItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0010: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, __midl__iwiadrvitem0011: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindChildItemByName(&*(&__midl__iwiadrvitem0010 as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&__midl__iwiadrvitem0011)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParentItem<Impl: IWiaDrvItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0012: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetParentItem(::core::mem::transmute_copy(&__midl__iwiadrvitem0012)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFirstChildItem<Impl: IWiaDrvItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0013: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFirstChildItem(::core::mem::transmute_copy(&__midl__iwiadrvitem0013)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNextSiblingItem<Impl: IWiaDrvItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0014: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNextSiblingItem(::core::mem::transmute_copy(&__midl__iwiadrvitem0014)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DumpItemData<Impl: IWiaDrvItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0015: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DumpItemData(::core::mem::transmute_copy(&__midl__iwiadrvitem0015)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IWiaDrvItem>,
            base.5,
            GetItemFlags::<Impl, OFFSET>,
            GetDeviceSpecContext::<Impl, OFFSET>,
            GetFullItemName::<Impl, OFFSET>,
            GetItemName::<Impl, OFFSET>,
            AddItemToFolder::<Impl, OFFSET>,
            UnlinkItemTree::<Impl, OFFSET>,
            RemoveItemFromFolder::<Impl, OFFSET>,
            FindItemByName::<Impl, OFFSET>,
            FindChildItemByName::<Impl, OFFSET>,
            GetParentItem::<Impl, OFFSET>,
            GetFirstChildItem::<Impl, OFFSET>,
            GetNextSiblingItem::<Impl, OFFSET>,
            DumpItemData::<Impl, OFFSET>,
        )
    }
}
pub trait IWiaErrorHandlerImpl: Sized {
    fn ReportStatus();
    fn GetStatusDescription();
}
impl ::windows::core::RuntimeName for IWiaErrorHandler {
    const NAME: &'static str = "Windows.Win32.Devices.ImageAcquisition.IWiaErrorHandler";
}
impl IWiaErrorHandlerVtbl {
    pub const fn new<Impl: IWiaErrorHandlerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWiaErrorHandlerVtbl {
        unsafe extern "system" fn ReportStatus<Impl: IWiaErrorHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, hwndparent: super::super::Foundation::HWND, pwiaitem2: ::windows::core::RawPtr, hrstatus: ::windows::core::HRESULT, lpercentcomplete: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReportStatus(lflags, &*(&hwndparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&pwiaitem2 as *const <IWiaItem2 as ::windows::core::Abi>::Abi as *const <IWiaItem2 as ::windows::core::DefaultType>::DefaultType), hrstatus, lpercentcomplete) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatusDescription<Impl: IWiaErrorHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, pwiaitem2: ::windows::core::RawPtr, hrstatus: ::windows::core::HRESULT, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStatusDescription(lflags, &*(&pwiaitem2 as *const <IWiaItem2 as ::windows::core::Abi>::Abi as *const <IWiaItem2 as ::windows::core::DefaultType>::DefaultType), hrstatus, ::core::mem::transmute_copy(&pbstrdescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWiaErrorHandler>, base.5, ReportStatus::<Impl, OFFSET>, GetStatusDescription::<Impl, OFFSET>)
    }
}
pub trait IWiaEventCallbackImpl: Sized {
    fn ImageEventCallback();
}
impl ::windows::core::RuntimeName for IWiaEventCallback {
    const NAME: &'static str = "Windows.Win32.Devices.ImageAcquisition.IWiaEventCallback";
}
impl IWiaEventCallbackVtbl {
    pub const fn new<Impl: IWiaEventCallbackImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWiaEventCallbackVtbl {
        unsafe extern "system" fn ImageEventCallback<Impl: IWiaEventCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, peventguid: *const ::windows::core::GUID, bstreventdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdevicedescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwdevicetype: u32, bstrfullitemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, puleventtype: *mut u32, ulreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ImageEventCallback(
                &*(&peventguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&bstreventdescription as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrdeviceid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrdevicedescription as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                dwdevicetype,
                &*(&bstrfullitemname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                puleventtype,
                ulreserved,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWiaEventCallback>, base.5, ImageEventCallback::<Impl, OFFSET>)
    }
}
pub trait IWiaImageFilterImpl: Sized {
    fn InitializeFilter();
    fn SetNewCallback();
    fn FilterPreviewImage();
    fn ApplyProperties();
}
impl ::windows::core::RuntimeName for IWiaImageFilter {
    const NAME: &'static str = "Windows.Win32.Devices.ImageAcquisition.IWiaImageFilter";
}
impl IWiaImageFilterVtbl {
    pub const fn new<Impl: IWiaImageFilterImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWiaImageFilterVtbl {
        unsafe extern "system" fn InitializeFilter<Impl: IWiaImageFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwiaitem2: ::windows::core::RawPtr, pwiatransfercallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InitializeFilter(&*(&pwiaitem2 as *const <IWiaItem2 as ::windows::core::Abi>::Abi as *const <IWiaItem2 as ::windows::core::DefaultType>::DefaultType), &*(&pwiatransfercallback as *const <IWiaTransferCallback as ::windows::core::Abi>::Abi as *const <IWiaTransferCallback as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNewCallback<Impl: IWiaImageFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwiatransfercallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetNewCallback(&*(&pwiatransfercallback as *const <IWiaTransferCallback as ::windows::core::Abi>::Abi as *const <IWiaTransferCallback as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FilterPreviewImage<Impl: IWiaImageFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, pwiachilditem2: ::windows::core::RawPtr, inputimageextents: super::super::Foundation::RECT, pinputstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FilterPreviewImage(
                lflags,
                &*(&pwiachilditem2 as *const <IWiaItem2 as ::windows::core::Abi>::Abi as *const <IWiaItem2 as ::windows::core::DefaultType>::DefaultType),
                &*(&inputimageextents as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                &*(&pinputstream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplyProperties<Impl: IWiaImageFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwiapropertystorage: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ApplyProperties(&*(&pwiapropertystorage as *const <IWiaPropertyStorage as ::windows::core::Abi>::Abi as *const <IWiaPropertyStorage as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWiaImageFilter>, base.5, InitializeFilter::<Impl, OFFSET>, SetNewCallback::<Impl, OFFSET>, FilterPreviewImage::<Impl, OFFSET>, ApplyProperties::<Impl, OFFSET>)
    }
}
pub trait IWiaItemImpl: Sized {
    fn GetItemType();
    fn AnalyzeItem();
    fn EnumChildItems();
    fn DeleteItem();
    fn CreateChildItem();
    fn EnumRegisterEventInfo();
    fn FindItemByName();
    fn DeviceDlg();
    fn DeviceCommand();
    fn GetRootItem();
    fn EnumDeviceCapabilities();
    fn DumpItemData();
    fn DumpDrvItemData();
    fn DumpTreeItemData();
    fn Diagnostic();
}
impl ::windows::core::RuntimeName for IWiaItem {
    const NAME: &'static str = "Windows.Win32.Devices.ImageAcquisition.IWiaItem";
}
impl IWiaItemVtbl {
    pub const fn new<Impl: IWiaItemImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWiaItemVtbl {
        unsafe extern "system" fn GetItemType<Impl: IWiaItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pitemtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetItemType(::core::mem::transmute_copy(&pitemtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AnalyzeItem<Impl: IWiaItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AnalyzeItem(lflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumChildItems<Impl: IWiaItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppienumwiaitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumChildItems(::core::mem::transmute_copy(&ppienumwiaitem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteItem<Impl: IWiaItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteItem(lflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateChildItem<Impl: IWiaItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrfullitemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppiwiaitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateChildItem(lflags, &*(&bstritemname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrfullitemname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppiwiaitem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumRegisterEventInfo<Impl: IWiaItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, peventguid: *const ::windows::core::GUID, ppienum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumRegisterEventInfo(lflags, &*(&peventguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppienum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindItemByName<Impl: IWiaItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, bstrfullitemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppiwiaitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindItemByName(lflags, &*(&bstrfullitemname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppiwiaitem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceDlg<Impl: IWiaItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, lflags: i32, lintent: i32, plitemcount: *mut i32, ppiwiaitem: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceDlg(&*(&hwndparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), lflags, lintent, plitemcount, &*(&ppiwiaitem as *const <IWiaItem as ::windows::core::Abi>::Abi as *const <IWiaItem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceCommand<Impl: IWiaItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, pcmdguid: *const ::windows::core::GUID, piwiaitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceCommand(lflags, &*(&pcmdguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&piwiaitem as *const <IWiaItem as ::windows::core::Abi>::Abi as *const <IWiaItem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRootItem<Impl: IWiaItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppiwiaitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRootItem(::core::mem::transmute_copy(&ppiwiaitem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDeviceCapabilities<Impl: IWiaItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, ppienumwia_dev_caps: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumDeviceCapabilities(lflags, ::core::mem::transmute_copy(&ppienumwia_dev_caps)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DumpItemData<Impl: IWiaItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DumpItemData(::core::mem::transmute_copy(&bstrdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DumpDrvItemData<Impl: IWiaItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DumpDrvItemData(::core::mem::transmute_copy(&bstrdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DumpTreeItemData<Impl: IWiaItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DumpTreeItemData(::core::mem::transmute_copy(&bstrdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Diagnostic<Impl: IWiaItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulsize: u32, pbuffer: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Diagnostic(ulsize, pbuffer) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IWiaItem>,
            base.5,
            GetItemType::<Impl, OFFSET>,
            AnalyzeItem::<Impl, OFFSET>,
            EnumChildItems::<Impl, OFFSET>,
            DeleteItem::<Impl, OFFSET>,
            CreateChildItem::<Impl, OFFSET>,
            EnumRegisterEventInfo::<Impl, OFFSET>,
            FindItemByName::<Impl, OFFSET>,
            DeviceDlg::<Impl, OFFSET>,
            DeviceCommand::<Impl, OFFSET>,
            GetRootItem::<Impl, OFFSET>,
            EnumDeviceCapabilities::<Impl, OFFSET>,
            DumpItemData::<Impl, OFFSET>,
            DumpDrvItemData::<Impl, OFFSET>,
            DumpTreeItemData::<Impl, OFFSET>,
            Diagnostic::<Impl, OFFSET>,
        )
    }
}
pub trait IWiaItem2Impl: Sized {
    fn CreateChildItem();
    fn DeleteItem();
    fn EnumChildItems();
    fn FindItemByName();
    fn GetItemCategory();
    fn GetItemType();
    fn DeviceDlg();
    fn DeviceCommand();
    fn EnumDeviceCapabilities();
    fn CheckExtension();
    fn GetExtension();
    fn GetParentItem();
    fn GetRootItem();
    fn GetPreviewComponent();
    fn EnumRegisterEventInfo();
    fn Diagnostic();
}
impl ::windows::core::RuntimeName for IWiaItem2 {
    const NAME: &'static str = "Windows.Win32.Devices.ImageAcquisition.IWiaItem2";
}
impl IWiaItem2Vtbl {
    pub const fn new<Impl: IWiaItem2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWiaItem2Vtbl {
        unsafe extern "system" fn CreateChildItem<Impl: IWiaItem2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, litemflags: i32, lcreationflags: i32, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppiwiaitem2: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateChildItem(litemflags, lcreationflags, &*(&bstritemname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppiwiaitem2)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteItem<Impl: IWiaItem2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteItem(lflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumChildItems<Impl: IWiaItem2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcategoryguid: *const ::windows::core::GUID, ppienumwiaitem2: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumChildItems(&*(&pcategoryguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppienumwiaitem2)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindItemByName<Impl: IWiaItem2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, bstrfullitemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppiwiaitem2: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindItemByName(lflags, &*(&bstrfullitemname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppiwiaitem2)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemCategory<Impl: IWiaItem2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pitemcategoryguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetItemCategory(::core::mem::transmute_copy(&pitemcategoryguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemType<Impl: IWiaItem2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pitemtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetItemType(::core::mem::transmute_copy(&pitemtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceDlg<Impl: IWiaItem2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, hwndparent: super::super::Foundation::HWND, bstrfoldername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, plnumfiles: *mut i32, ppbstrfilepaths: *mut *mut super::super::Foundation::BSTR, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceDlg(
                lflags,
                &*(&hwndparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrfoldername as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrfilename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&plnumfiles),
                ::core::mem::transmute_copy(&ppbstrfilepaths),
                ::core::mem::transmute_copy(&ppitem),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceCommand<Impl: IWiaItem2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, pcmdguid: *const ::windows::core::GUID, ppiwiaitem2: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceCommand(lflags, &*(&pcmdguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&ppiwiaitem2 as *const <IWiaItem2 as ::windows::core::Abi>::Abi as *const <IWiaItem2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDeviceCapabilities<Impl: IWiaItem2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, ppienumwia_dev_caps: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumDeviceCapabilities(lflags, ::core::mem::transmute_copy(&ppienumwia_dev_caps)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckExtension<Impl: IWiaItem2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, riidextensioninterface: *const ::windows::core::GUID, pbextensionexists: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckExtension(
                lflags,
                &*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&riidextensioninterface as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&pbextensionexists as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExtension<Impl: IWiaItem2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, riidextensioninterface: *const ::windows::core::GUID, ppout: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetExtension(
                lflags,
                &*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&riidextensioninterface as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&ppout as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParentItem<Impl: IWiaItem2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppiwiaitem2: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetParentItem(::core::mem::transmute_copy(&ppiwiaitem2)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRootItem<Impl: IWiaItem2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppiwiaitem2: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRootItem(::core::mem::transmute_copy(&ppiwiaitem2)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreviewComponent<Impl: IWiaItem2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, ppwiapreview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPreviewComponent(lflags, ::core::mem::transmute_copy(&ppwiapreview)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumRegisterEventInfo<Impl: IWiaItem2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, peventguid: *const ::windows::core::GUID, ppienum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumRegisterEventInfo(lflags, &*(&peventguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppienum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Diagnostic<Impl: IWiaItem2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulsize: u32, pbuffer: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Diagnostic(ulsize, pbuffer) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IWiaItem2>,
            base.5,
            CreateChildItem::<Impl, OFFSET>,
            DeleteItem::<Impl, OFFSET>,
            EnumChildItems::<Impl, OFFSET>,
            FindItemByName::<Impl, OFFSET>,
            GetItemCategory::<Impl, OFFSET>,
            GetItemType::<Impl, OFFSET>,
            DeviceDlg::<Impl, OFFSET>,
            DeviceCommand::<Impl, OFFSET>,
            EnumDeviceCapabilities::<Impl, OFFSET>,
            CheckExtension::<Impl, OFFSET>,
            GetExtension::<Impl, OFFSET>,
            GetParentItem::<Impl, OFFSET>,
            GetRootItem::<Impl, OFFSET>,
            GetPreviewComponent::<Impl, OFFSET>,
            EnumRegisterEventInfo::<Impl, OFFSET>,
            Diagnostic::<Impl, OFFSET>,
        )
    }
}
pub trait IWiaItemExtrasImpl: Sized {
    fn GetExtendedErrorInfo();
    fn Escape();
    fn CancelPendingIO();
}
impl ::windows::core::RuntimeName for IWiaItemExtras {
    const NAME: &'static str = "Windows.Win32.Devices.ImageAcquisition.IWiaItemExtras";
}
impl IWiaItemExtrasVtbl {
    pub const fn new<Impl: IWiaItemExtrasImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWiaItemExtrasVtbl {
        unsafe extern "system" fn GetExtendedErrorInfo<Impl: IWiaItemExtrasImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrerrortext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetExtendedErrorInfo(::core::mem::transmute_copy(&bstrerrortext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Escape<Impl: IWiaItemExtrasImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwescapecode: u32, lpindata: *const u8, cbindatasize: u32, poutdata: *mut u8, dwoutdatasize: u32, pdwactualdatasize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Escape(dwescapecode, lpindata, cbindatasize, ::core::mem::transmute_copy(&poutdata), dwoutdatasize, ::core::mem::transmute_copy(&pdwactualdatasize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelPendingIO<Impl: IWiaItemExtrasImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CancelPendingIO() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWiaItemExtras>, base.5, GetExtendedErrorInfo::<Impl, OFFSET>, Escape::<Impl, OFFSET>, CancelPendingIO::<Impl, OFFSET>)
    }
}
pub trait IWiaLogImpl: Sized {
    fn InitializeLog();
    fn hResult();
    fn Log();
}
impl ::windows::core::RuntimeName for IWiaLog {
    const NAME: &'static str = "Windows.Win32.Devices.ImageAcquisition.IWiaLog";
}
impl IWiaLogVtbl {
    pub const fn new<Impl: IWiaLogImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWiaLogVtbl {
        unsafe extern "system" fn InitializeLog<Impl: IWiaLogImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hinstance: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InitializeLog(hinstance) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn hResult<Impl: IWiaLogImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).hResult(hresult) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Log<Impl: IWiaLogImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, lresid: i32, ldetail: i32, bstrtext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Log(lflags, lresid, ldetail, &*(&bstrtext as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWiaLog>, base.5, InitializeLog::<Impl, OFFSET>, hResult::<Impl, OFFSET>, Log::<Impl, OFFSET>)
    }
}
pub trait IWiaLogExImpl: Sized {
    fn InitializeLogEx();
    fn hResult();
    fn Log();
    fn hResultEx();
    fn LogEx();
}
impl ::windows::core::RuntimeName for IWiaLogEx {
    const NAME: &'static str = "Windows.Win32.Devices.ImageAcquisition.IWiaLogEx";
}
impl IWiaLogExVtbl {
    pub const fn new<Impl: IWiaLogExImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWiaLogExVtbl {
        unsafe extern "system" fn InitializeLogEx<Impl: IWiaLogExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hinstance: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InitializeLogEx(hinstance) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn hResult<Impl: IWiaLogExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).hResult(hresult) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Log<Impl: IWiaLogExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, lresid: i32, ldetail: i32, bstrtext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Log(lflags, lresid, ldetail, &*(&bstrtext as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn hResultEx<Impl: IWiaLogExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lmethodid: i32, hresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).hResultEx(lmethodid, hresult) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LogEx<Impl: IWiaLogExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lmethodid: i32, lflags: i32, lresid: i32, ldetail: i32, bstrtext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LogEx(lmethodid, lflags, lresid, ldetail, &*(&bstrtext as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWiaLogEx>, base.5, InitializeLogEx::<Impl, OFFSET>, hResult::<Impl, OFFSET>, Log::<Impl, OFFSET>, hResultEx::<Impl, OFFSET>, LogEx::<Impl, OFFSET>)
    }
}
pub trait IWiaMiniDrvImpl: Sized {
    fn drvInitializeWia();
    fn drvAcquireItemData();
    fn drvInitItemProperties();
    fn drvValidateItemProperties();
    fn drvWriteItemProperties();
    fn drvReadItemProperties();
    fn drvLockWiaDevice();
    fn drvUnLockWiaDevice();
    fn drvAnalyzeItem();
    fn drvGetDeviceErrorStr();
    fn drvDeviceCommand();
    fn drvGetCapabilities();
    fn drvDeleteItem();
    fn drvFreeDrvItemContext();
    fn drvGetWiaFormatInfo();
    fn drvNotifyPnpEvent();
    fn drvUnInitializeWia();
}
impl ::windows::core::RuntimeName for IWiaMiniDrv {
    const NAME: &'static str = "Windows.Win32.Devices.ImageAcquisition.IWiaMiniDrv";
}
impl IWiaMiniDrvVtbl {
    pub const fn new<Impl: IWiaMiniDrvImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWiaMiniDrvVtbl {
        unsafe extern "system" fn drvInitializeWia<Impl: IWiaMiniDrvImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0000: *const u8, __midl__iwiaminidrv0001: i32, __midl__iwiaminidrv0002: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, __midl__iwiaminidrv0003: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, __midl__iwiaminidrv0004: *mut ::core::ffi::c_void, __midl__iwiaminidrv0005: *mut ::core::ffi::c_void, __midl__iwiaminidrv0006: *mut ::windows::core::RawPtr, __midl__iwiaminidrv0007: *mut *mut ::core::ffi::c_void, __midl__iwiaminidrv0008: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).drvInitializeWia(
                __midl__iwiaminidrv0000,
                __midl__iwiaminidrv0001,
                &*(&__midl__iwiaminidrv0002 as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&__midl__iwiaminidrv0003 as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&__midl__iwiaminidrv0004 as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&__midl__iwiaminidrv0005 as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&__midl__iwiaminidrv0006),
                ::core::mem::transmute_copy(&__midl__iwiaminidrv0007),
                ::core::mem::transmute_copy(&__midl__iwiaminidrv0008),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn drvAcquireItemData<Impl: IWiaMiniDrvImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0009: *const u8, __midl__iwiaminidrv0010: i32, __midl__iwiaminidrv0011: *mut MINIDRV_TRANSFER_CONTEXT, __midl__iwiaminidrv0012: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).drvAcquireItemData(__midl__iwiaminidrv0009, __midl__iwiaminidrv0010, &*(&__midl__iwiaminidrv0011 as *const <MINIDRV_TRANSFER_CONTEXT as ::windows::core::Abi>::Abi as *const <MINIDRV_TRANSFER_CONTEXT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&__midl__iwiaminidrv0012)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn drvInitItemProperties<Impl: IWiaMiniDrvImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0013: *const u8, __midl__iwiaminidrv0014: i32, __midl__iwiaminidrv0015: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).drvInitItemProperties(__midl__iwiaminidrv0013, __midl__iwiaminidrv0014, ::core::mem::transmute_copy(&__midl__iwiaminidrv0015)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn drvValidateItemProperties<Impl: IWiaMiniDrvImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0016: *const u8, __midl__iwiaminidrv0017: i32, __midl__iwiaminidrv0018: u32, __midl__iwiaminidrv0019: *const super::super::System::Com::StructuredStorage::PROPSPEC, __midl__iwiaminidrv0020: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).drvValidateItemProperties(__midl__iwiaminidrv0016, __midl__iwiaminidrv0017, __midl__iwiaminidrv0018, &*(&__midl__iwiaminidrv0019 as *const <super::super::System::Com::StructuredStorage::PROPSPEC as ::windows::core::Abi>::Abi as *const <super::super::System::Com::StructuredStorage::PROPSPEC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&__midl__iwiaminidrv0020)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn drvWriteItemProperties<Impl: IWiaMiniDrvImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0021: *const u8, __midl__iwiaminidrv0022: i32, __midl__iwiaminidrv0023: *const MINIDRV_TRANSFER_CONTEXT, __midl__iwiaminidrv0024: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).drvWriteItemProperties(__midl__iwiaminidrv0021, __midl__iwiaminidrv0022, &*(&__midl__iwiaminidrv0023 as *const <MINIDRV_TRANSFER_CONTEXT as ::windows::core::Abi>::Abi as *const <MINIDRV_TRANSFER_CONTEXT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&__midl__iwiaminidrv0024)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn drvReadItemProperties<Impl: IWiaMiniDrvImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0025: *const u8, __midl__iwiaminidrv0026: i32, __midl__iwiaminidrv0027: u32, __midl__iwiaminidrv0028: *const super::super::System::Com::StructuredStorage::PROPSPEC, __midl__iwiaminidrv0029: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).drvReadItemProperties(__midl__iwiaminidrv0025, __midl__iwiaminidrv0026, __midl__iwiaminidrv0027, &*(&__midl__iwiaminidrv0028 as *const <super::super::System::Com::StructuredStorage::PROPSPEC as ::windows::core::Abi>::Abi as *const <super::super::System::Com::StructuredStorage::PROPSPEC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&__midl__iwiaminidrv0029)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn drvLockWiaDevice<Impl: IWiaMiniDrvImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0030: *const u8, __midl__iwiaminidrv0031: i32, __midl__iwiaminidrv0032: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).drvLockWiaDevice(__midl__iwiaminidrv0030, __midl__iwiaminidrv0031, ::core::mem::transmute_copy(&__midl__iwiaminidrv0032)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn drvUnLockWiaDevice<Impl: IWiaMiniDrvImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0033: *const u8, __midl__iwiaminidrv0034: i32, __midl__iwiaminidrv0035: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).drvUnLockWiaDevice(__midl__iwiaminidrv0033, __midl__iwiaminidrv0034, ::core::mem::transmute_copy(&__midl__iwiaminidrv0035)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn drvAnalyzeItem<Impl: IWiaMiniDrvImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0036: *const u8, __midl__iwiaminidrv0037: i32, __midl__iwiaminidrv0038: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).drvAnalyzeItem(__midl__iwiaminidrv0036, __midl__iwiaminidrv0037, __midl__iwiaminidrv0038) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn drvGetDeviceErrorStr<Impl: IWiaMiniDrvImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0039: i32, __midl__iwiaminidrv0040: i32, __midl__iwiaminidrv0041: *mut super::super::Foundation::PWSTR, __midl__iwiaminidrv0042: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).drvGetDeviceErrorStr(__midl__iwiaminidrv0039, __midl__iwiaminidrv0040, ::core::mem::transmute_copy(&__midl__iwiaminidrv0041), ::core::mem::transmute_copy(&__midl__iwiaminidrv0042)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn drvDeviceCommand<Impl: IWiaMiniDrvImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0043: *const u8, __midl__iwiaminidrv0044: i32, __midl__iwiaminidrv0045: *const ::windows::core::GUID, __midl__iwiaminidrv0046: *mut ::windows::core::RawPtr, __midl__iwiaminidrv0047: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).drvDeviceCommand(__midl__iwiaminidrv0043, __midl__iwiaminidrv0044, &*(&__midl__iwiaminidrv0045 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&__midl__iwiaminidrv0046), ::core::mem::transmute_copy(&__midl__iwiaminidrv0047)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn drvGetCapabilities<Impl: IWiaMiniDrvImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0048: *const u8, __midl__iwiaminidrv0049: i32, __midl__iwiaminidrv0050: *mut i32, __midl__iwiaminidrv0051: *mut *mut WIA_DEV_CAP_DRV, __midl__iwiaminidrv0052: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).drvGetCapabilities(__midl__iwiaminidrv0048, __midl__iwiaminidrv0049, ::core::mem::transmute_copy(&__midl__iwiaminidrv0050), ::core::mem::transmute_copy(&__midl__iwiaminidrv0051), ::core::mem::transmute_copy(&__midl__iwiaminidrv0052)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn drvDeleteItem<Impl: IWiaMiniDrvImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0053: *const u8, __midl__iwiaminidrv0054: i32, __midl__iwiaminidrv0055: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).drvDeleteItem(__midl__iwiaminidrv0053, __midl__iwiaminidrv0054, ::core::mem::transmute_copy(&__midl__iwiaminidrv0055)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn drvFreeDrvItemContext<Impl: IWiaMiniDrvImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0056: i32, __midl__iwiaminidrv0057: *const u8, __midl__iwiaminidrv0058: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).drvFreeDrvItemContext(__midl__iwiaminidrv0056, __midl__iwiaminidrv0057, ::core::mem::transmute_copy(&__midl__iwiaminidrv0058)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn drvGetWiaFormatInfo<Impl: IWiaMiniDrvImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0059: *const u8, __midl__iwiaminidrv0060: i32, __midl__iwiaminidrv0061: *mut i32, __midl__iwiaminidrv0062: *mut *mut WIA_FORMAT_INFO, __midl__iwiaminidrv0063: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).drvGetWiaFormatInfo(__midl__iwiaminidrv0059, __midl__iwiaminidrv0060, ::core::mem::transmute_copy(&__midl__iwiaminidrv0061), ::core::mem::transmute_copy(&__midl__iwiaminidrv0062), ::core::mem::transmute_copy(&__midl__iwiaminidrv0063)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn drvNotifyPnpEvent<Impl: IWiaMiniDrvImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, peventguid: *const ::windows::core::GUID, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ulreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).drvNotifyPnpEvent(&*(&peventguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&bstrdeviceid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ulreserved) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn drvUnInitializeWia<Impl: IWiaMiniDrvImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0064: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).drvUnInitializeWia(__midl__iwiaminidrv0064) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IWiaMiniDrv>,
            base.5,
            drvInitializeWia::<Impl, OFFSET>,
            drvAcquireItemData::<Impl, OFFSET>,
            drvInitItemProperties::<Impl, OFFSET>,
            drvValidateItemProperties::<Impl, OFFSET>,
            drvWriteItemProperties::<Impl, OFFSET>,
            drvReadItemProperties::<Impl, OFFSET>,
            drvLockWiaDevice::<Impl, OFFSET>,
            drvUnLockWiaDevice::<Impl, OFFSET>,
            drvAnalyzeItem::<Impl, OFFSET>,
            drvGetDeviceErrorStr::<Impl, OFFSET>,
            drvDeviceCommand::<Impl, OFFSET>,
            drvGetCapabilities::<Impl, OFFSET>,
            drvDeleteItem::<Impl, OFFSET>,
            drvFreeDrvItemContext::<Impl, OFFSET>,
            drvGetWiaFormatInfo::<Impl, OFFSET>,
            drvNotifyPnpEvent::<Impl, OFFSET>,
            drvUnInitializeWia::<Impl, OFFSET>,
        )
    }
}
pub trait IWiaMiniDrvCallBackImpl: Sized {
    fn MiniDrvCallback();
}
impl ::windows::core::RuntimeName for IWiaMiniDrvCallBack {
    const NAME: &'static str = "Windows.Win32.Devices.ImageAcquisition.IWiaMiniDrvCallBack";
}
impl IWiaMiniDrvCallBackVtbl {
    pub const fn new<Impl: IWiaMiniDrvCallBackImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWiaMiniDrvCallBackVtbl {
        unsafe extern "system" fn MiniDrvCallback<Impl: IWiaMiniDrvCallBackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lreason: i32, lstatus: i32, lpercentcomplete: i32, loffset: i32, llength: i32, ptranctx: *const MINIDRV_TRANSFER_CONTEXT, lreserved: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MiniDrvCallback(lreason, lstatus, lpercentcomplete, loffset, llength, &*(&ptranctx as *const <MINIDRV_TRANSFER_CONTEXT as ::windows::core::Abi>::Abi as *const <MINIDRV_TRANSFER_CONTEXT as ::windows::core::DefaultType>::DefaultType), lreserved) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWiaMiniDrvCallBack>, base.5, MiniDrvCallback::<Impl, OFFSET>)
    }
}
pub trait IWiaMiniDrvTransferCallbackImpl: Sized {
    fn GetNextStream();
    fn SendMessage();
}
impl ::windows::core::RuntimeName for IWiaMiniDrvTransferCallback {
    const NAME: &'static str = "Windows.Win32.Devices.ImageAcquisition.IWiaMiniDrvTransferCallback";
}
impl IWiaMiniDrvTransferCallbackVtbl {
    pub const fn new<Impl: IWiaMiniDrvTransferCallbackImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWiaMiniDrvTransferCallbackVtbl {
        unsafe extern "system" fn GetNextStream<Impl: IWiaMiniDrvTransferCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrfullitemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppistream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNextStream(lflags, &*(&bstritemname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrfullitemname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppistream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendMessage<Impl: IWiaMiniDrvTransferCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, pwiatransferparams: *const WiaTransferParams) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SendMessage(lflags, &*(&pwiatransferparams as *const <WiaTransferParams as ::windows::core::Abi>::Abi as *const <WiaTransferParams as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWiaMiniDrvTransferCallback>, base.5, GetNextStream::<Impl, OFFSET>, SendMessage::<Impl, OFFSET>)
    }
}
pub trait IWiaNotifyDevMgrImpl: Sized {
    fn NewDeviceArrival();
}
impl ::windows::core::RuntimeName for IWiaNotifyDevMgr {
    const NAME: &'static str = "Windows.Win32.Devices.ImageAcquisition.IWiaNotifyDevMgr";
}
impl IWiaNotifyDevMgrVtbl {
    pub const fn new<Impl: IWiaNotifyDevMgrImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWiaNotifyDevMgrVtbl {
        unsafe extern "system" fn NewDeviceArrival<Impl: IWiaNotifyDevMgrImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NewDeviceArrival() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWiaNotifyDevMgr>, base.5, NewDeviceArrival::<Impl, OFFSET>)
    }
}
pub trait IWiaPreviewImpl: Sized {
    fn GetNewPreview();
    fn UpdatePreview();
    fn DetectRegions();
    fn Clear();
}
impl ::windows::core::RuntimeName for IWiaPreview {
    const NAME: &'static str = "Windows.Win32.Devices.ImageAcquisition.IWiaPreview";
}
impl IWiaPreviewVtbl {
    pub const fn new<Impl: IWiaPreviewImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWiaPreviewVtbl {
        unsafe extern "system" fn GetNewPreview<Impl: IWiaPreviewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, pwiaitem2: ::windows::core::RawPtr, pwiatransfercallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNewPreview(lflags, &*(&pwiaitem2 as *const <IWiaItem2 as ::windows::core::Abi>::Abi as *const <IWiaItem2 as ::windows::core::DefaultType>::DefaultType), &*(&pwiatransfercallback as *const <IWiaTransferCallback as ::windows::core::Abi>::Abi as *const <IWiaTransferCallback as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdatePreview<Impl: IWiaPreviewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, pchildwiaitem2: ::windows::core::RawPtr, pwiatransfercallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdatePreview(lflags, &*(&pchildwiaitem2 as *const <IWiaItem2 as ::windows::core::Abi>::Abi as *const <IWiaItem2 as ::windows::core::DefaultType>::DefaultType), &*(&pwiatransfercallback as *const <IWiaTransferCallback as ::windows::core::Abi>::Abi as *const <IWiaTransferCallback as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DetectRegions<Impl: IWiaPreviewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DetectRegions(lflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Impl: IWiaPreviewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clear() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWiaPreview>, base.5, GetNewPreview::<Impl, OFFSET>, UpdatePreview::<Impl, OFFSET>, DetectRegions::<Impl, OFFSET>, Clear::<Impl, OFFSET>)
    }
}
pub trait IWiaPropertyStorageImpl: Sized {
    fn ReadMultiple();
    fn WriteMultiple();
    fn DeleteMultiple();
    fn ReadPropertyNames();
    fn WritePropertyNames();
    fn DeletePropertyNames();
    fn Commit();
    fn Revert();
    fn Enum();
    fn SetTimes();
    fn SetClass();
    fn Stat();
    fn GetPropertyAttributes();
    fn GetCount();
    fn GetPropertyStream();
    fn SetPropertyStream();
}
impl ::windows::core::RuntimeName for IWiaPropertyStorage {
    const NAME: &'static str = "Windows.Win32.Devices.ImageAcquisition.IWiaPropertyStorage";
}
impl IWiaPropertyStorageVtbl {
    pub const fn new<Impl: IWiaPropertyStorageImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWiaPropertyStorageVtbl {
        unsafe extern "system" fn ReadMultiple<Impl: IWiaPropertyStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cpspec: u32, rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC, rgpropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReadMultiple(cpspec, &*(&rgpspec as *const <super::super::System::Com::StructuredStorage::PROPSPEC as ::windows::core::Abi>::Abi as *const <super::super::System::Com::StructuredStorage::PROPSPEC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&rgpropvar)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteMultiple<Impl: IWiaPropertyStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cpspec: u32, rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC, rgpropvar: *const super::super::System::Com::StructuredStorage::PROPVARIANT, propidnamefirst: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WriteMultiple(cpspec, &*(&rgpspec as *const <super::super::System::Com::StructuredStorage::PROPSPEC as ::windows::core::Abi>::Abi as *const <super::super::System::Com::StructuredStorage::PROPSPEC as ::windows::core::DefaultType>::DefaultType), &*(&rgpropvar as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType), propidnamefirst) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteMultiple<Impl: IWiaPropertyStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cpspec: u32, rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteMultiple(cpspec, &*(&rgpspec as *const <super::super::System::Com::StructuredStorage::PROPSPEC as ::windows::core::Abi>::Abi as *const <super::super::System::Com::StructuredStorage::PROPSPEC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadPropertyNames<Impl: IWiaPropertyStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cpropid: u32, rgpropid: *const u32, rglpwstrname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReadPropertyNames(cpropid, rgpropid, ::core::mem::transmute_copy(&rglpwstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WritePropertyNames<Impl: IWiaPropertyStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cpropid: u32, rgpropid: *const u32, rglpwstrname: *const super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WritePropertyNames(cpropid, rgpropid, &*(&rglpwstrname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeletePropertyNames<Impl: IWiaPropertyStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cpropid: u32, rgpropid: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeletePropertyNames(cpropid, rgpropid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Commit<Impl: IWiaPropertyStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, grfcommitflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Commit(grfcommitflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Revert<Impl: IWiaPropertyStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Revert() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enum<Impl: IWiaPropertyStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Enum(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTimes<Impl: IWiaPropertyStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pctime: *const super::super::Foundation::FILETIME, patime: *const super::super::Foundation::FILETIME, pmtime: *const super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetTimes(
                &*(&pctime as *const <super::super::Foundation::FILETIME as ::windows::core::Abi>::Abi as *const <super::super::Foundation::FILETIME as ::windows::core::DefaultType>::DefaultType),
                &*(&patime as *const <super::super::Foundation::FILETIME as ::windows::core::Abi>::Abi as *const <super::super::Foundation::FILETIME as ::windows::core::DefaultType>::DefaultType),
                &*(&pmtime as *const <super::super::Foundation::FILETIME as ::windows::core::Abi>::Abi as *const <super::super::Foundation::FILETIME as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClass<Impl: IWiaPropertyStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetClass(&*(&clsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stat<Impl: IWiaPropertyStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstatpsstg: *mut super::super::System::Com::StructuredStorage::STATPROPSETSTG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Stat(::core::mem::transmute_copy(&pstatpsstg)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyAttributes<Impl: IWiaPropertyStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cpspec: u32, rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC, rgflags: *mut u32, rgpropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPropertyAttributes(cpspec, &*(&rgpspec as *const <super::super::System::Com::StructuredStorage::PROPSPEC as ::windows::core::Abi>::Abi as *const <super::super::System::Com::StructuredStorage::PROPSPEC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&rgflags), ::core::mem::transmute_copy(&rgpropvar)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Impl: IWiaPropertyStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulnumprops: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&pulnumprops)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyStream<Impl: IWiaPropertyStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcompatibilityid: *mut ::windows::core::GUID, ppistream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPropertyStream(::core::mem::transmute_copy(&pcompatibilityid), ::core::mem::transmute_copy(&ppistream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPropertyStream<Impl: IWiaPropertyStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcompatibilityid: *mut ::windows::core::GUID, pistream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPropertyStream(&*(&pcompatibilityid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&pistream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IWiaPropertyStorage>,
            base.5,
            ReadMultiple::<Impl, OFFSET>,
            WriteMultiple::<Impl, OFFSET>,
            DeleteMultiple::<Impl, OFFSET>,
            ReadPropertyNames::<Impl, OFFSET>,
            WritePropertyNames::<Impl, OFFSET>,
            DeletePropertyNames::<Impl, OFFSET>,
            Commit::<Impl, OFFSET>,
            Revert::<Impl, OFFSET>,
            Enum::<Impl, OFFSET>,
            SetTimes::<Impl, OFFSET>,
            SetClass::<Impl, OFFSET>,
            Stat::<Impl, OFFSET>,
            GetPropertyAttributes::<Impl, OFFSET>,
            GetCount::<Impl, OFFSET>,
            GetPropertyStream::<Impl, OFFSET>,
            SetPropertyStream::<Impl, OFFSET>,
        )
    }
}
pub trait IWiaSegmentationFilterImpl: Sized {
    fn DetectRegions();
}
impl ::windows::core::RuntimeName for IWiaSegmentationFilter {
    const NAME: &'static str = "Windows.Win32.Devices.ImageAcquisition.IWiaSegmentationFilter";
}
impl IWiaSegmentationFilterVtbl {
    pub const fn new<Impl: IWiaSegmentationFilterImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWiaSegmentationFilterVtbl {
        unsafe extern "system" fn DetectRegions<Impl: IWiaSegmentationFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, pinputstream: ::windows::core::RawPtr, pwiaitem2: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DetectRegions(lflags, &*(&pinputstream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), &*(&pwiaitem2 as *const <IWiaItem2 as ::windows::core::Abi>::Abi as *const <IWiaItem2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWiaSegmentationFilter>, base.5, DetectRegions::<Impl, OFFSET>)
    }
}
pub trait IWiaTransferImpl: Sized {
    fn Download();
    fn Upload();
    fn Cancel();
    fn EnumWIA_FORMAT_INFO();
}
impl ::windows::core::RuntimeName for IWiaTransfer {
    const NAME: &'static str = "Windows.Win32.Devices.ImageAcquisition.IWiaTransfer";
}
impl IWiaTransferVtbl {
    pub const fn new<Impl: IWiaTransferImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWiaTransferVtbl {
        unsafe extern "system" fn Download<Impl: IWiaTransferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, piwiatransfercallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Download(lflags, &*(&piwiatransfercallback as *const <IWiaTransferCallback as ::windows::core::Abi>::Abi as *const <IWiaTransferCallback as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Upload<Impl: IWiaTransferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, psource: ::windows::core::RawPtr, piwiatransfercallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Upload(lflags, &*(&psource as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), &*(&piwiatransfercallback as *const <IWiaTransferCallback as ::windows::core::Abi>::Abi as *const <IWiaTransferCallback as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: IWiaTransferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Cancel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumWIA_FORMAT_INFO<Impl: IWiaTransferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumWIA_FORMAT_INFO(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWiaTransfer>, base.5, Download::<Impl, OFFSET>, Upload::<Impl, OFFSET>, Cancel::<Impl, OFFSET>, EnumWIA_FORMAT_INFO::<Impl, OFFSET>)
    }
}
pub trait IWiaTransferCallbackImpl: Sized {
    fn TransferCallback();
    fn GetNextStream();
}
impl ::windows::core::RuntimeName for IWiaTransferCallback {
    const NAME: &'static str = "Windows.Win32.Devices.ImageAcquisition.IWiaTransferCallback";
}
impl IWiaTransferCallbackVtbl {
    pub const fn new<Impl: IWiaTransferCallbackImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWiaTransferCallbackVtbl {
        unsafe extern "system" fn TransferCallback<Impl: IWiaTransferCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, pwiatransferparams: *const WiaTransferParams) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TransferCallback(lflags, &*(&pwiatransferparams as *const <WiaTransferParams as ::windows::core::Abi>::Abi as *const <WiaTransferParams as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNextStream<Impl: IWiaTransferCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrfullitemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdestination: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNextStream(lflags, &*(&bstritemname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrfullitemname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppdestination)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWiaTransferCallback>, base.5, TransferCallback::<Impl, OFFSET>, GetNextStream::<Impl, OFFSET>)
    }
}
pub trait IWiaUIExtensionImpl: Sized {
    fn DeviceDialog();
    fn GetDeviceIcon();
    fn GetDeviceBitmapLogo();
}
impl ::windows::core::RuntimeName for IWiaUIExtension {
    const NAME: &'static str = "Windows.Win32.Devices.ImageAcquisition.IWiaUIExtension";
}
impl IWiaUIExtensionVtbl {
    pub const fn new<Impl: IWiaUIExtensionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWiaUIExtensionVtbl {
        unsafe extern "system" fn DeviceDialog<Impl: IWiaUIExtensionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevicedialogdata: *const DEVICEDIALOGDATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceDialog(&*(&pdevicedialogdata as *const <DEVICEDIALOGDATA as ::windows::core::Abi>::Abi as *const <DEVICEDIALOGDATA as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceIcon<Impl: IWiaUIExtensionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, phicon: *mut super::super::UI::WindowsAndMessaging::HICON, nsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeviceIcon(&*(&bstrdeviceid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&phicon), nsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceBitmapLogo<Impl: IWiaUIExtensionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, phbitmap: *mut super::super::Graphics::Gdi::HBITMAP, nmaxwidth: u32, nmaxheight: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeviceBitmapLogo(&*(&bstrdeviceid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&phbitmap), nmaxwidth, nmaxheight) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWiaUIExtension>, base.5, DeviceDialog::<Impl, OFFSET>, GetDeviceIcon::<Impl, OFFSET>, GetDeviceBitmapLogo::<Impl, OFFSET>)
    }
}
pub trait IWiaUIExtension2Impl: Sized {
    fn DeviceDialog();
    fn GetDeviceIcon();
}
impl ::windows::core::RuntimeName for IWiaUIExtension2 {
    const NAME: &'static str = "Windows.Win32.Devices.ImageAcquisition.IWiaUIExtension2";
}
impl IWiaUIExtension2Vtbl {
    pub const fn new<Impl: IWiaUIExtension2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWiaUIExtension2Vtbl {
        unsafe extern "system" fn DeviceDialog<Impl: IWiaUIExtension2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevicedialogdata: *const DEVICEDIALOGDATA2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceDialog(&*(&pdevicedialogdata as *const <DEVICEDIALOGDATA2 as ::windows::core::Abi>::Abi as *const <DEVICEDIALOGDATA2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceIcon<Impl: IWiaUIExtension2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, phicon: *mut super::super::UI::WindowsAndMessaging::HICON, nsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeviceIcon(&*(&bstrdeviceid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&phicon), nsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWiaUIExtension2>, base.5, DeviceDialog::<Impl, OFFSET>, GetDeviceIcon::<Impl, OFFSET>)
    }
}
pub trait IWiaVideoImpl: Sized {
    fn PreviewVisible();
    fn SetPreviewVisible();
    fn ImagesDirectory();
    fn SetImagesDirectory();
    fn CreateVideoByWiaDevID();
    fn CreateVideoByDevNum();
    fn CreateVideoByName();
    fn DestroyVideo();
    fn Play();
    fn Pause();
    fn TakePicture();
    fn ResizeVideo();
    fn GetCurrentState();
}
impl ::windows::core::RuntimeName for IWiaVideo {
    const NAME: &'static str = "Windows.Win32.Devices.ImageAcquisition.IWiaVideo";
}
impl IWiaVideoVtbl {
    pub const fn new<Impl: IWiaVideoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWiaVideoVtbl {
        unsafe extern "system" fn PreviewVisible<Impl: IWiaVideoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbpreviewvisible: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PreviewVisible(::core::mem::transmute_copy(&pbpreviewvisible)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreviewVisible<Impl: IWiaVideoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bpreviewvisible: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPreviewVisible(&*(&bpreviewvisible as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImagesDirectory<Impl: IWiaVideoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrimagedirectory: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ImagesDirectory(::core::mem::transmute_copy(&pbstrimagedirectory)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetImagesDirectory<Impl: IWiaVideoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrimagedirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetImagesDirectory(&*(&bstrimagedirectory as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVideoByWiaDevID<Impl: IWiaVideoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrwiadeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, hwndparent: super::super::Foundation::HWND, bstretchtofitparent: super::super::Foundation::BOOL, bautobeginplayback: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateVideoByWiaDevID(
                &*(&bstrwiadeviceid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&hwndparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                &*(&bstretchtofitparent as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&bautobeginplayback as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVideoByDevNum<Impl: IWiaVideoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uidevicenumber: u32, hwndparent: super::super::Foundation::HWND, bstretchtofitparent: super::super::Foundation::BOOL, bautobeginplayback: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateVideoByDevNum(
                uidevicenumber,
                &*(&hwndparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                &*(&bstretchtofitparent as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&bautobeginplayback as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVideoByName<Impl: IWiaVideoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrfriendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, hwndparent: super::super::Foundation::HWND, bstretchtofitparent: super::super::Foundation::BOOL, bautobeginplayback: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateVideoByName(
                &*(&bstrfriendlyname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&hwndparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                &*(&bstretchtofitparent as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&bautobeginplayback as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestroyVideo<Impl: IWiaVideoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DestroyVideo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Play<Impl: IWiaVideoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Play() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pause<Impl: IWiaVideoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Pause() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TakePicture<Impl: IWiaVideoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrnewimagefilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TakePicture(::core::mem::transmute_copy(&pbstrnewimagefilename)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResizeVideo<Impl: IWiaVideoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstretchtofitparent: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ResizeVideo(&*(&bstretchtofitparent as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentState<Impl: IWiaVideoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstate: *mut WIAVIDEO_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCurrentState(::core::mem::transmute_copy(&pstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IWiaVideo>,
            base.5,
            PreviewVisible::<Impl, OFFSET>,
            SetPreviewVisible::<Impl, OFFSET>,
            ImagesDirectory::<Impl, OFFSET>,
            SetImagesDirectory::<Impl, OFFSET>,
            CreateVideoByWiaDevID::<Impl, OFFSET>,
            CreateVideoByDevNum::<Impl, OFFSET>,
            CreateVideoByName::<Impl, OFFSET>,
            DestroyVideo::<Impl, OFFSET>,
            Play::<Impl, OFFSET>,
            Pause::<Impl, OFFSET>,
            TakePicture::<Impl, OFFSET>,
            ResizeVideo::<Impl, OFFSET>,
            GetCurrentState::<Impl, OFFSET>,
        )
    }
}
