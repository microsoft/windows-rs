#[cfg(feature = "Win32_Foundation")]
pub trait IEnumWIA_DEV_CAPSImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
    fn GetCount();
}
#[cfg(feature = "Win32_Foundation")]
impl IEnumWIA_DEV_CAPSVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumWIA_DEV_CAPSImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumWIA_DEV_CAPSVtbl {
        unsafe extern "system" fn Next<Impl: IEnumWIA_DEV_CAPSImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut WIA_DEV_CAP, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumWIA_DEV_CAPSImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumWIA_DEV_CAPSImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumWIA_DEV_CAPSImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppienum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCount<Impl: IEnumWIA_DEV_CAPSImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>, GetCount::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumWIA_DEV_CAPS as ::windows::core::Interface>::IID
    }
}
pub trait IEnumWIA_DEV_INFOImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
    fn GetCount();
}
impl IEnumWIA_DEV_INFOVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumWIA_DEV_INFOImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumWIA_DEV_INFOVtbl {
        unsafe extern "system" fn Next<Impl: IEnumWIA_DEV_INFOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumWIA_DEV_INFOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumWIA_DEV_INFOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumWIA_DEV_INFOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppienum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCount<Impl: IEnumWIA_DEV_INFOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>, GetCount::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumWIA_DEV_INFO as ::windows::core::Interface>::IID
    }
}
pub trait IEnumWIA_FORMAT_INFOImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
    fn GetCount();
}
impl IEnumWIA_FORMAT_INFOVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumWIA_FORMAT_INFOImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumWIA_FORMAT_INFOVtbl {
        unsafe extern "system" fn Next<Impl: IEnumWIA_FORMAT_INFOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut WIA_FORMAT_INFO, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumWIA_FORMAT_INFOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumWIA_FORMAT_INFOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumWIA_FORMAT_INFOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppienum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCount<Impl: IEnumWIA_FORMAT_INFOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>, GetCount::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumWIA_FORMAT_INFO as ::windows::core::Interface>::IID
    }
}
pub trait IEnumWiaItemImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
    fn GetCount();
}
impl IEnumWiaItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumWiaItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumWiaItemVtbl {
        unsafe extern "system" fn Next<Impl: IEnumWiaItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppiwiaitem: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumWiaItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumWiaItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumWiaItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppienum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCount<Impl: IEnumWiaItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>, GetCount::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumWiaItem as ::windows::core::Interface>::IID
    }
}
pub trait IEnumWiaItem2Impl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
    fn GetCount();
}
impl IEnumWiaItem2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumWiaItem2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumWiaItem2Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumWiaItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppiwiaitem2: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumWiaItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumWiaItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumWiaItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppienum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCount<Impl: IEnumWiaItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>, GetCount::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumWiaItem2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWiaAppErrorHandlerImpl: Sized {
    fn GetWindow();
    fn ReportStatus();
}
#[cfg(feature = "Win32_Foundation")]
impl IWiaAppErrorHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiaAppErrorHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiaAppErrorHandlerVtbl {
        unsafe extern "system" fn GetWindow<Impl: IWiaAppErrorHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReportStatus<Impl: IWiaAppErrorHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pwiaitem2: ::windows::core::RawPtr, hrstatus: ::windows::core::HRESULT, lpercentcomplete: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetWindow::<Impl, IMPL_OFFSET>, ReportStatus::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaAppErrorHandler as ::windows::core::Interface>::IID
    }
}
pub trait IWiaDataCallbackImpl: Sized {
    fn BandedDataCallback();
}
impl IWiaDataCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiaDataCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiaDataCallbackVtbl {
        unsafe extern "system" fn BandedDataCallback<Impl: IWiaDataCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmessage: i32, lstatus: i32, lpercentcomplete: i32, loffset: i32, llength: i32, lreserved: i32, lreslength: i32, pbbuffer: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, BandedDataCallback::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaDataCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IWiaDataTransferImpl: Sized {
    fn idtGetData();
    fn idtGetBandedData();
    fn idtQueryGetData();
    fn idtEnumWIA_FORMAT_INFO();
    fn idtGetExtendedTransferInfo();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IWiaDataTransferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiaDataTransferImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiaDataTransferVtbl {
        unsafe extern "system" fn idtGetData<Impl: IWiaDataTransferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmedium: *mut super::super::System::Com::STGMEDIUM, piwiadatacallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn idtGetBandedData<Impl: IWiaDataTransferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwiadatatransinfo: *mut WIA_DATA_TRANSFER_INFO, piwiadatacallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn idtQueryGetData<Impl: IWiaDataTransferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfe: *const WIA_FORMAT_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn idtEnumWIA_FORMAT_INFO<Impl: IWiaDataTransferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn idtGetExtendedTransferInfo<Impl: IWiaDataTransferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pextendedtransferinfo: *mut WIA_EXTENDED_TRANSFER_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, idtGetData::<Impl, IMPL_OFFSET>, idtGetBandedData::<Impl, IMPL_OFFSET>, idtQueryGetData::<Impl, IMPL_OFFSET>, idtEnumWIA_FORMAT_INFO::<Impl, IMPL_OFFSET>, idtGetExtendedTransferInfo::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaDataTransfer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IWiaDevMgrVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiaDevMgrImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiaDevMgrVtbl {
        unsafe extern "system" fn EnumDeviceInfo<Impl: IWiaDevMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflag: i32, ppienum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateDevice<Impl: IWiaDevMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppwiaitemroot: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SelectDeviceDlg<Impl: IWiaDevMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut super::super::Foundation::BSTR, ppitemroot: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SelectDeviceDlgID<Impl: IWiaDevMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetImageDlg<Impl: IWiaDevMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, lintent: i32, pitemroot: ::windows::core::RawPtr, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pguidformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RegisterEventCallbackProgram<Impl: IWiaDevMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, peventguid: *const ::windows::core::GUID, bstrcommandline: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstricon: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RegisterEventCallbackInterface<Impl: IWiaDevMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, peventguid: *const ::windows::core::GUID, piwiaeventcallback: ::windows::core::RawPtr, peventobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RegisterEventCallbackCLSID<Impl: IWiaDevMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, peventguid: *const ::windows::core::GUID, pclsid: *const ::windows::core::GUID, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstricon: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddDeviceDlg<Impl: IWiaDevMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            EnumDeviceInfo::<Impl, IMPL_OFFSET>,
            CreateDevice::<Impl, IMPL_OFFSET>,
            SelectDeviceDlg::<Impl, IMPL_OFFSET>,
            SelectDeviceDlgID::<Impl, IMPL_OFFSET>,
            GetImageDlg::<Impl, IMPL_OFFSET>,
            RegisterEventCallbackProgram::<Impl, IMPL_OFFSET>,
            RegisterEventCallbackInterface::<Impl, IMPL_OFFSET>,
            RegisterEventCallbackCLSID::<Impl, IMPL_OFFSET>,
            AddDeviceDlg::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaDevMgr as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IWiaDevMgr2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiaDevMgr2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiaDevMgr2Vtbl {
        unsafe extern "system" fn EnumDeviceInfo<Impl: IWiaDevMgr2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, ppienum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateDevice<Impl: IWiaDevMgr2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppwiaitem2root: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SelectDeviceDlg<Impl: IWiaDevMgr2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut super::super::Foundation::BSTR, ppitemroot: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SelectDeviceDlgID<Impl: IWiaDevMgr2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RegisterEventCallbackInterface<Impl: IWiaDevMgr2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, peventguid: *const ::windows::core::GUID, piwiaeventcallback: ::windows::core::RawPtr, peventobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RegisterEventCallbackProgram<Impl: IWiaDevMgr2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, peventguid: *const ::windows::core::GUID, bstrfullappname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrcommandlinearg: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstricon: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RegisterEventCallbackCLSID<Impl: IWiaDevMgr2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, peventguid: *const ::windows::core::GUID, pclsid: *const ::windows::core::GUID, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstricon: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetImageDlg<Impl: IWiaDevMgr2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, hwndparent: super::super::Foundation::HWND, bstrfoldername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, plnumfiles: *mut i32, ppbstrfilepaths: *mut *mut super::super::Foundation::BSTR, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            EnumDeviceInfo::<Impl, IMPL_OFFSET>,
            CreateDevice::<Impl, IMPL_OFFSET>,
            SelectDeviceDlg::<Impl, IMPL_OFFSET>,
            SelectDeviceDlgID::<Impl, IMPL_OFFSET>,
            RegisterEventCallbackInterface::<Impl, IMPL_OFFSET>,
            RegisterEventCallbackProgram::<Impl, IMPL_OFFSET>,
            RegisterEventCallbackCLSID::<Impl, IMPL_OFFSET>,
            GetImageDlg::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaDevMgr2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IWiaDrvItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiaDrvItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiaDrvItemVtbl {
        unsafe extern "system" fn GetItemFlags<Impl: IWiaDrvItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0000: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceSpecContext<Impl: IWiaDrvItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0001: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFullItemName<Impl: IWiaDrvItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0002: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetItemName<Impl: IWiaDrvItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0003: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddItemToFolder<Impl: IWiaDrvItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0004: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnlinkItemTree<Impl: IWiaDrvItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0005: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveItemFromFolder<Impl: IWiaDrvItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0006: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindItemByName<Impl: IWiaDrvItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0007: i32, __midl__iwiadrvitem0008: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, __midl__iwiadrvitem0009: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindChildItemByName<Impl: IWiaDrvItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0010: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, __midl__iwiadrvitem0011: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetParentItem<Impl: IWiaDrvItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0012: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFirstChildItem<Impl: IWiaDrvItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0013: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNextSiblingItem<Impl: IWiaDrvItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0014: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DumpItemData<Impl: IWiaDrvItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0015: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetItemFlags::<Impl, IMPL_OFFSET>,
            GetDeviceSpecContext::<Impl, IMPL_OFFSET>,
            GetFullItemName::<Impl, IMPL_OFFSET>,
            GetItemName::<Impl, IMPL_OFFSET>,
            AddItemToFolder::<Impl, IMPL_OFFSET>,
            UnlinkItemTree::<Impl, IMPL_OFFSET>,
            RemoveItemFromFolder::<Impl, IMPL_OFFSET>,
            FindItemByName::<Impl, IMPL_OFFSET>,
            FindChildItemByName::<Impl, IMPL_OFFSET>,
            GetParentItem::<Impl, IMPL_OFFSET>,
            GetFirstChildItem::<Impl, IMPL_OFFSET>,
            GetNextSiblingItem::<Impl, IMPL_OFFSET>,
            DumpItemData::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaDrvItem as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWiaErrorHandlerImpl: Sized {
    fn ReportStatus();
    fn GetStatusDescription();
}
#[cfg(feature = "Win32_Foundation")]
impl IWiaErrorHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiaErrorHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiaErrorHandlerVtbl {
        unsafe extern "system" fn ReportStatus<Impl: IWiaErrorHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, hwndparent: super::super::Foundation::HWND, pwiaitem2: ::windows::core::RawPtr, hrstatus: ::windows::core::HRESULT, lpercentcomplete: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStatusDescription<Impl: IWiaErrorHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pwiaitem2: ::windows::core::RawPtr, hrstatus: ::windows::core::HRESULT, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ReportStatus::<Impl, IMPL_OFFSET>, GetStatusDescription::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaErrorHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWiaEventCallbackImpl: Sized {
    fn ImageEventCallback();
}
#[cfg(feature = "Win32_Foundation")]
impl IWiaEventCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiaEventCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiaEventCallbackVtbl {
        unsafe extern "system" fn ImageEventCallback<Impl: IWiaEventCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventguid: *const ::windows::core::GUID, bstreventdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdevicedescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwdevicetype: u32, bstrfullitemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, puleventtype: *mut u32, ulreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ImageEventCallback::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaEventCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWiaImageFilterImpl: Sized {
    fn InitializeFilter();
    fn SetNewCallback();
    fn FilterPreviewImage();
    fn ApplyProperties();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWiaImageFilterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiaImageFilterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiaImageFilterVtbl {
        unsafe extern "system" fn InitializeFilter<Impl: IWiaImageFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwiaitem2: ::windows::core::RawPtr, pwiatransfercallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNewCallback<Impl: IWiaImageFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwiatransfercallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FilterPreviewImage<Impl: IWiaImageFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pwiachilditem2: ::windows::core::RawPtr, inputimageextents: super::super::Foundation::RECT, pinputstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ApplyProperties<Impl: IWiaImageFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwiapropertystorage: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, InitializeFilter::<Impl, IMPL_OFFSET>, SetNewCallback::<Impl, IMPL_OFFSET>, FilterPreviewImage::<Impl, IMPL_OFFSET>, ApplyProperties::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaImageFilter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IWiaItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiaItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiaItemVtbl {
        unsafe extern "system" fn GetItemType<Impl: IWiaItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitemtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AnalyzeItem<Impl: IWiaItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumChildItems<Impl: IWiaItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppienumwiaitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteItem<Impl: IWiaItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateChildItem<Impl: IWiaItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrfullitemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppiwiaitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumRegisterEventInfo<Impl: IWiaItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, peventguid: *const ::windows::core::GUID, ppienum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindItemByName<Impl: IWiaItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, bstrfullitemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppiwiaitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeviceDlg<Impl: IWiaItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, lflags: i32, lintent: i32, plitemcount: *mut i32, ppiwiaitem: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeviceCommand<Impl: IWiaItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pcmdguid: *const ::windows::core::GUID, piwiaitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRootItem<Impl: IWiaItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiwiaitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumDeviceCapabilities<Impl: IWiaItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, ppienumwia_dev_caps: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DumpItemData<Impl: IWiaItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DumpDrvItemData<Impl: IWiaItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DumpTreeItemData<Impl: IWiaItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Diagnostic<Impl: IWiaItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulsize: u32, pbuffer: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetItemType::<Impl, IMPL_OFFSET>,
            AnalyzeItem::<Impl, IMPL_OFFSET>,
            EnumChildItems::<Impl, IMPL_OFFSET>,
            DeleteItem::<Impl, IMPL_OFFSET>,
            CreateChildItem::<Impl, IMPL_OFFSET>,
            EnumRegisterEventInfo::<Impl, IMPL_OFFSET>,
            FindItemByName::<Impl, IMPL_OFFSET>,
            DeviceDlg::<Impl, IMPL_OFFSET>,
            DeviceCommand::<Impl, IMPL_OFFSET>,
            GetRootItem::<Impl, IMPL_OFFSET>,
            EnumDeviceCapabilities::<Impl, IMPL_OFFSET>,
            DumpItemData::<Impl, IMPL_OFFSET>,
            DumpDrvItemData::<Impl, IMPL_OFFSET>,
            DumpTreeItemData::<Impl, IMPL_OFFSET>,
            Diagnostic::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaItem as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IWiaItem2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiaItem2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiaItem2Vtbl {
        unsafe extern "system" fn CreateChildItem<Impl: IWiaItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, litemflags: i32, lcreationflags: i32, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppiwiaitem2: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteItem<Impl: IWiaItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumChildItems<Impl: IWiaItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcategoryguid: *const ::windows::core::GUID, ppienumwiaitem2: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindItemByName<Impl: IWiaItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, bstrfullitemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppiwiaitem2: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetItemCategory<Impl: IWiaItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitemcategoryguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetItemType<Impl: IWiaItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitemtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeviceDlg<Impl: IWiaItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, hwndparent: super::super::Foundation::HWND, bstrfoldername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, plnumfiles: *mut i32, ppbstrfilepaths: *mut *mut super::super::Foundation::BSTR, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeviceCommand<Impl: IWiaItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pcmdguid: *const ::windows::core::GUID, ppiwiaitem2: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumDeviceCapabilities<Impl: IWiaItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, ppienumwia_dev_caps: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CheckExtension<Impl: IWiaItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, riidextensioninterface: *const ::windows::core::GUID, pbextensionexists: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetExtension<Impl: IWiaItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, riidextensioninterface: *const ::windows::core::GUID, ppout: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetParentItem<Impl: IWiaItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiwiaitem2: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRootItem<Impl: IWiaItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiwiaitem2: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPreviewComponent<Impl: IWiaItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, ppwiapreview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumRegisterEventInfo<Impl: IWiaItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, peventguid: *const ::windows::core::GUID, ppienum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Diagnostic<Impl: IWiaItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulsize: u32, pbuffer: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            CreateChildItem::<Impl, IMPL_OFFSET>,
            DeleteItem::<Impl, IMPL_OFFSET>,
            EnumChildItems::<Impl, IMPL_OFFSET>,
            FindItemByName::<Impl, IMPL_OFFSET>,
            GetItemCategory::<Impl, IMPL_OFFSET>,
            GetItemType::<Impl, IMPL_OFFSET>,
            DeviceDlg::<Impl, IMPL_OFFSET>,
            DeviceCommand::<Impl, IMPL_OFFSET>,
            EnumDeviceCapabilities::<Impl, IMPL_OFFSET>,
            CheckExtension::<Impl, IMPL_OFFSET>,
            GetExtension::<Impl, IMPL_OFFSET>,
            GetParentItem::<Impl, IMPL_OFFSET>,
            GetRootItem::<Impl, IMPL_OFFSET>,
            GetPreviewComponent::<Impl, IMPL_OFFSET>,
            EnumRegisterEventInfo::<Impl, IMPL_OFFSET>,
            Diagnostic::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaItem2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWiaItemExtrasImpl: Sized {
    fn GetExtendedErrorInfo();
    fn Escape();
    fn CancelPendingIO();
}
#[cfg(feature = "Win32_Foundation")]
impl IWiaItemExtrasVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiaItemExtrasImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiaItemExtrasVtbl {
        unsafe extern "system" fn GetExtendedErrorInfo<Impl: IWiaItemExtrasImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrerrortext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Escape<Impl: IWiaItemExtrasImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwescapecode: u32, lpindata: *const u8, cbindatasize: u32, poutdata: *mut u8, dwoutdatasize: u32, pdwactualdatasize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CancelPendingIO<Impl: IWiaItemExtrasImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetExtendedErrorInfo::<Impl, IMPL_OFFSET>, Escape::<Impl, IMPL_OFFSET>, CancelPendingIO::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaItemExtras as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWiaLogImpl: Sized {
    fn InitializeLog();
    fn hResult();
    fn Log();
}
#[cfg(feature = "Win32_Foundation")]
impl IWiaLogVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiaLogImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiaLogVtbl {
        unsafe extern "system" fn InitializeLog<Impl: IWiaLogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hinstance: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn hResult<Impl: IWiaLogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Log<Impl: IWiaLogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, lresid: i32, ldetail: i32, bstrtext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, InitializeLog::<Impl, IMPL_OFFSET>, hResult::<Impl, IMPL_OFFSET>, Log::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaLog as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWiaLogExImpl: Sized {
    fn InitializeLogEx();
    fn hResult();
    fn Log();
    fn hResultEx();
    fn LogEx();
}
#[cfg(feature = "Win32_Foundation")]
impl IWiaLogExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiaLogExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiaLogExVtbl {
        unsafe extern "system" fn InitializeLogEx<Impl: IWiaLogExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hinstance: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn hResult<Impl: IWiaLogExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Log<Impl: IWiaLogExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, lresid: i32, ldetail: i32, bstrtext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn hResultEx<Impl: IWiaLogExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmethodid: i32, hresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LogEx<Impl: IWiaLogExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmethodid: i32, lflags: i32, lresid: i32, ldetail: i32, bstrtext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, InitializeLogEx::<Impl, IMPL_OFFSET>, hResult::<Impl, IMPL_OFFSET>, Log::<Impl, IMPL_OFFSET>, hResultEx::<Impl, IMPL_OFFSET>, LogEx::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaLogEx as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl IWiaMiniDrvVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiaMiniDrvImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiaMiniDrvVtbl {
        unsafe extern "system" fn drvInitializeWia<Impl: IWiaMiniDrvImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0000: *const u8, __midl__iwiaminidrv0001: i32, __midl__iwiaminidrv0002: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, __midl__iwiaminidrv0003: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, __midl__iwiaminidrv0004: *mut ::core::ffi::c_void, __midl__iwiaminidrv0005: *mut ::core::ffi::c_void, __midl__iwiaminidrv0006: *mut ::windows::core::RawPtr, __midl__iwiaminidrv0007: *mut *mut ::core::ffi::c_void, __midl__iwiaminidrv0008: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn drvAcquireItemData<Impl: IWiaMiniDrvImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0009: *const u8, __midl__iwiaminidrv0010: i32, __midl__iwiaminidrv0011: *mut MINIDRV_TRANSFER_CONTEXT, __midl__iwiaminidrv0012: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn drvInitItemProperties<Impl: IWiaMiniDrvImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0013: *const u8, __midl__iwiaminidrv0014: i32, __midl__iwiaminidrv0015: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn drvValidateItemProperties<Impl: IWiaMiniDrvImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0016: *const u8, __midl__iwiaminidrv0017: i32, __midl__iwiaminidrv0018: u32, __midl__iwiaminidrv0019: *const super::super::System::Com::StructuredStorage::PROPSPEC, __midl__iwiaminidrv0020: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn drvWriteItemProperties<Impl: IWiaMiniDrvImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0021: *const u8, __midl__iwiaminidrv0022: i32, __midl__iwiaminidrv0023: *const MINIDRV_TRANSFER_CONTEXT, __midl__iwiaminidrv0024: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn drvReadItemProperties<Impl: IWiaMiniDrvImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0025: *const u8, __midl__iwiaminidrv0026: i32, __midl__iwiaminidrv0027: u32, __midl__iwiaminidrv0028: *const super::super::System::Com::StructuredStorage::PROPSPEC, __midl__iwiaminidrv0029: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn drvLockWiaDevice<Impl: IWiaMiniDrvImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0030: *const u8, __midl__iwiaminidrv0031: i32, __midl__iwiaminidrv0032: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn drvUnLockWiaDevice<Impl: IWiaMiniDrvImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0033: *const u8, __midl__iwiaminidrv0034: i32, __midl__iwiaminidrv0035: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn drvAnalyzeItem<Impl: IWiaMiniDrvImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0036: *const u8, __midl__iwiaminidrv0037: i32, __midl__iwiaminidrv0038: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn drvGetDeviceErrorStr<Impl: IWiaMiniDrvImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0039: i32, __midl__iwiaminidrv0040: i32, __midl__iwiaminidrv0041: *mut super::super::Foundation::PWSTR, __midl__iwiaminidrv0042: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn drvDeviceCommand<Impl: IWiaMiniDrvImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0043: *const u8, __midl__iwiaminidrv0044: i32, __midl__iwiaminidrv0045: *const ::windows::core::GUID, __midl__iwiaminidrv0046: *mut ::windows::core::RawPtr, __midl__iwiaminidrv0047: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn drvGetCapabilities<Impl: IWiaMiniDrvImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0048: *const u8, __midl__iwiaminidrv0049: i32, __midl__iwiaminidrv0050: *mut i32, __midl__iwiaminidrv0051: *mut *mut WIA_DEV_CAP_DRV, __midl__iwiaminidrv0052: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn drvDeleteItem<Impl: IWiaMiniDrvImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0053: *const u8, __midl__iwiaminidrv0054: i32, __midl__iwiaminidrv0055: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn drvFreeDrvItemContext<Impl: IWiaMiniDrvImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0056: i32, __midl__iwiaminidrv0057: *const u8, __midl__iwiaminidrv0058: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn drvGetWiaFormatInfo<Impl: IWiaMiniDrvImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0059: *const u8, __midl__iwiaminidrv0060: i32, __midl__iwiaminidrv0061: *mut i32, __midl__iwiaminidrv0062: *mut *mut WIA_FORMAT_INFO, __midl__iwiaminidrv0063: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn drvNotifyPnpEvent<Impl: IWiaMiniDrvImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventguid: *const ::windows::core::GUID, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ulreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn drvUnInitializeWia<Impl: IWiaMiniDrvImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0064: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            drvInitializeWia::<Impl, IMPL_OFFSET>,
            drvAcquireItemData::<Impl, IMPL_OFFSET>,
            drvInitItemProperties::<Impl, IMPL_OFFSET>,
            drvValidateItemProperties::<Impl, IMPL_OFFSET>,
            drvWriteItemProperties::<Impl, IMPL_OFFSET>,
            drvReadItemProperties::<Impl, IMPL_OFFSET>,
            drvLockWiaDevice::<Impl, IMPL_OFFSET>,
            drvUnLockWiaDevice::<Impl, IMPL_OFFSET>,
            drvAnalyzeItem::<Impl, IMPL_OFFSET>,
            drvGetDeviceErrorStr::<Impl, IMPL_OFFSET>,
            drvDeviceCommand::<Impl, IMPL_OFFSET>,
            drvGetCapabilities::<Impl, IMPL_OFFSET>,
            drvDeleteItem::<Impl, IMPL_OFFSET>,
            drvFreeDrvItemContext::<Impl, IMPL_OFFSET>,
            drvGetWiaFormatInfo::<Impl, IMPL_OFFSET>,
            drvNotifyPnpEvent::<Impl, IMPL_OFFSET>,
            drvUnInitializeWia::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaMiniDrv as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWiaMiniDrvCallBackImpl: Sized {
    fn MiniDrvCallback();
}
#[cfg(feature = "Win32_Foundation")]
impl IWiaMiniDrvCallBackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiaMiniDrvCallBackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiaMiniDrvCallBackVtbl {
        unsafe extern "system" fn MiniDrvCallback<Impl: IWiaMiniDrvCallBackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lreason: i32, lstatus: i32, lpercentcomplete: i32, loffset: i32, llength: i32, ptranctx: *const MINIDRV_TRANSFER_CONTEXT, lreserved: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, MiniDrvCallback::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaMiniDrvCallBack as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWiaMiniDrvTransferCallbackImpl: Sized {
    fn GetNextStream();
    fn SendMessage();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWiaMiniDrvTransferCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiaMiniDrvTransferCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiaMiniDrvTransferCallbackVtbl {
        unsafe extern "system" fn GetNextStream<Impl: IWiaMiniDrvTransferCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrfullitemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppistream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SendMessage<Impl: IWiaMiniDrvTransferCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pwiatransferparams: *const WiaTransferParams) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetNextStream::<Impl, IMPL_OFFSET>, SendMessage::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaMiniDrvTransferCallback as ::windows::core::Interface>::IID
    }
}
pub trait IWiaNotifyDevMgrImpl: Sized {
    fn NewDeviceArrival();
}
impl IWiaNotifyDevMgrVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiaNotifyDevMgrImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiaNotifyDevMgrVtbl {
        unsafe extern "system" fn NewDeviceArrival<Impl: IWiaNotifyDevMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, NewDeviceArrival::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaNotifyDevMgr as ::windows::core::Interface>::IID
    }
}
pub trait IWiaPreviewImpl: Sized {
    fn GetNewPreview();
    fn UpdatePreview();
    fn DetectRegions();
    fn Clear();
}
impl IWiaPreviewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiaPreviewImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiaPreviewVtbl {
        unsafe extern "system" fn GetNewPreview<Impl: IWiaPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pwiaitem2: ::windows::core::RawPtr, pwiatransfercallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdatePreview<Impl: IWiaPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pchildwiaitem2: ::windows::core::RawPtr, pwiatransfercallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DetectRegions<Impl: IWiaPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clear<Impl: IWiaPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetNewPreview::<Impl, IMPL_OFFSET>, UpdatePreview::<Impl, IMPL_OFFSET>, DetectRegions::<Impl, IMPL_OFFSET>, Clear::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaPreview as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IWiaPropertyStorageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiaPropertyStorageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiaPropertyStorageVtbl {
        unsafe extern "system" fn ReadMultiple<Impl: IWiaPropertyStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpspec: u32, rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC, rgpropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteMultiple<Impl: IWiaPropertyStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpspec: u32, rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC, rgpropvar: *const super::super::System::Com::StructuredStorage::PROPVARIANT, propidnamefirst: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteMultiple<Impl: IWiaPropertyStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpspec: u32, rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReadPropertyNames<Impl: IWiaPropertyStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpropid: u32, rgpropid: *const u32, rglpwstrname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WritePropertyNames<Impl: IWiaPropertyStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpropid: u32, rgpropid: *const u32, rglpwstrname: *const super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeletePropertyNames<Impl: IWiaPropertyStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpropid: u32, rgpropid: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Commit<Impl: IWiaPropertyStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfcommitflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Revert<Impl: IWiaPropertyStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Enum<Impl: IWiaPropertyStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTimes<Impl: IWiaPropertyStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pctime: *const super::super::Foundation::FILETIME, patime: *const super::super::Foundation::FILETIME, pmtime: *const super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetClass<Impl: IWiaPropertyStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Stat<Impl: IWiaPropertyStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatpsstg: *mut super::super::System::Com::StructuredStorage::STATPROPSETSTG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropertyAttributes<Impl: IWiaPropertyStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpspec: u32, rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC, rgflags: *mut u32, rgpropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCount<Impl: IWiaPropertyStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulnumprops: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropertyStream<Impl: IWiaPropertyStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcompatibilityid: *mut ::windows::core::GUID, ppistream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPropertyStream<Impl: IWiaPropertyStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcompatibilityid: *mut ::windows::core::GUID, pistream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ReadMultiple::<Impl, IMPL_OFFSET>,
            WriteMultiple::<Impl, IMPL_OFFSET>,
            DeleteMultiple::<Impl, IMPL_OFFSET>,
            ReadPropertyNames::<Impl, IMPL_OFFSET>,
            WritePropertyNames::<Impl, IMPL_OFFSET>,
            DeletePropertyNames::<Impl, IMPL_OFFSET>,
            Commit::<Impl, IMPL_OFFSET>,
            Revert::<Impl, IMPL_OFFSET>,
            Enum::<Impl, IMPL_OFFSET>,
            SetTimes::<Impl, IMPL_OFFSET>,
            SetClass::<Impl, IMPL_OFFSET>,
            Stat::<Impl, IMPL_OFFSET>,
            GetPropertyAttributes::<Impl, IMPL_OFFSET>,
            GetCount::<Impl, IMPL_OFFSET>,
            GetPropertyStream::<Impl, IMPL_OFFSET>,
            SetPropertyStream::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaPropertyStorage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWiaSegmentationFilterImpl: Sized {
    fn DetectRegions();
}
#[cfg(feature = "Win32_System_Com")]
impl IWiaSegmentationFilterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiaSegmentationFilterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiaSegmentationFilterVtbl {
        unsafe extern "system" fn DetectRegions<Impl: IWiaSegmentationFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pinputstream: ::windows::core::RawPtr, pwiaitem2: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, DetectRegions::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaSegmentationFilter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWiaTransferImpl: Sized {
    fn Download();
    fn Upload();
    fn Cancel();
    fn EnumWIA_FORMAT_INFO();
}
#[cfg(feature = "Win32_System_Com")]
impl IWiaTransferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiaTransferImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiaTransferVtbl {
        unsafe extern "system" fn Download<Impl: IWiaTransferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, piwiatransfercallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Upload<Impl: IWiaTransferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, psource: ::windows::core::RawPtr, piwiatransfercallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Cancel<Impl: IWiaTransferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumWIA_FORMAT_INFO<Impl: IWiaTransferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Download::<Impl, IMPL_OFFSET>, Upload::<Impl, IMPL_OFFSET>, Cancel::<Impl, IMPL_OFFSET>, EnumWIA_FORMAT_INFO::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaTransfer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWiaTransferCallbackImpl: Sized {
    fn TransferCallback();
    fn GetNextStream();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWiaTransferCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiaTransferCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiaTransferCallbackVtbl {
        unsafe extern "system" fn TransferCallback<Impl: IWiaTransferCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pwiatransferparams: *const WiaTransferParams) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNextStream<Impl: IWiaTransferCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrfullitemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdestination: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, TransferCallback::<Impl, IMPL_OFFSET>, GetNextStream::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaTransferCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IWiaUIExtensionImpl: Sized {
    fn DeviceDialog();
    fn GetDeviceIcon();
    fn GetDeviceBitmapLogo();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl IWiaUIExtensionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiaUIExtensionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiaUIExtensionVtbl {
        unsafe extern "system" fn DeviceDialog<Impl: IWiaUIExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevicedialogdata: *const DEVICEDIALOGDATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceIcon<Impl: IWiaUIExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, phicon: *mut super::super::UI::WindowsAndMessaging::HICON, nsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceBitmapLogo<Impl: IWiaUIExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, phbitmap: *mut super::super::Graphics::Gdi::HBITMAP, nmaxwidth: u32, nmaxheight: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, DeviceDialog::<Impl, IMPL_OFFSET>, GetDeviceIcon::<Impl, IMPL_OFFSET>, GetDeviceBitmapLogo::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaUIExtension as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IWiaUIExtension2Impl: Sized {
    fn DeviceDialog();
    fn GetDeviceIcon();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IWiaUIExtension2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiaUIExtension2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiaUIExtension2Vtbl {
        unsafe extern "system" fn DeviceDialog<Impl: IWiaUIExtension2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevicedialogdata: *const DEVICEDIALOGDATA2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceIcon<Impl: IWiaUIExtension2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, phicon: *mut super::super::UI::WindowsAndMessaging::HICON, nsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, DeviceDialog::<Impl, IMPL_OFFSET>, GetDeviceIcon::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaUIExtension2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IWiaVideoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiaVideoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiaVideoVtbl {
        unsafe extern "system" fn PreviewVisible<Impl: IWiaVideoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbpreviewvisible: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPreviewVisible<Impl: IWiaVideoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bpreviewvisible: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ImagesDirectory<Impl: IWiaVideoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrimagedirectory: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetImagesDirectory<Impl: IWiaVideoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrimagedirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateVideoByWiaDevID<Impl: IWiaVideoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrwiadeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, hwndparent: super::super::Foundation::HWND, bstretchtofitparent: super::super::Foundation::BOOL, bautobeginplayback: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateVideoByDevNum<Impl: IWiaVideoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uidevicenumber: u32, hwndparent: super::super::Foundation::HWND, bstretchtofitparent: super::super::Foundation::BOOL, bautobeginplayback: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateVideoByName<Impl: IWiaVideoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfriendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, hwndparent: super::super::Foundation::HWND, bstretchtofitparent: super::super::Foundation::BOOL, bautobeginplayback: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DestroyVideo<Impl: IWiaVideoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Play<Impl: IWiaVideoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Pause<Impl: IWiaVideoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TakePicture<Impl: IWiaVideoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrnewimagefilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResizeVideo<Impl: IWiaVideoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstretchtofitparent: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentState<Impl: IWiaVideoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut WIAVIDEO_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            PreviewVisible::<Impl, IMPL_OFFSET>,
            SetPreviewVisible::<Impl, IMPL_OFFSET>,
            ImagesDirectory::<Impl, IMPL_OFFSET>,
            SetImagesDirectory::<Impl, IMPL_OFFSET>,
            CreateVideoByWiaDevID::<Impl, IMPL_OFFSET>,
            CreateVideoByDevNum::<Impl, IMPL_OFFSET>,
            CreateVideoByName::<Impl, IMPL_OFFSET>,
            DestroyVideo::<Impl, IMPL_OFFSET>,
            Play::<Impl, IMPL_OFFSET>,
            Pause::<Impl, IMPL_OFFSET>,
            TakePicture::<Impl, IMPL_OFFSET>,
            ResizeVideo::<Impl, IMPL_OFFSET>,
            GetCurrentState::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiaVideo as ::windows::core::Interface>::IID
    }
}
