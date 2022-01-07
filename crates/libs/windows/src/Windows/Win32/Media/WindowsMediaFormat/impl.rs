pub trait IAMWMBufferPassImpl: Sized {
    fn SetNotify();
}
impl ::windows::core::RuntimeName for IAMWMBufferPass {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IAMWMBufferPass";
}
impl IAMWMBufferPassVtbl {
    pub const fn new<Impl: IAMWMBufferPassImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAMWMBufferPassVtbl {
        unsafe extern "system" fn SetNotify<Impl: IAMWMBufferPassImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetNotify(&*(&pcallback as *const <IAMWMBufferPassCallback as ::windows::core::Abi>::Abi as *const <IAMWMBufferPassCallback as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAMWMBufferPass>, base.5, SetNotify::<Impl, OFFSET>)
    }
}
pub trait IAMWMBufferPassCallbackImpl: Sized {
    fn Notify();
}
impl ::windows::core::RuntimeName for IAMWMBufferPassCallback {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IAMWMBufferPassCallback";
}
impl IAMWMBufferPassCallbackVtbl {
    pub const fn new<Impl: IAMWMBufferPassCallbackImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAMWMBufferPassCallbackVtbl {
        unsafe extern "system" fn Notify<Impl: IAMWMBufferPassCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnssbuffer3: ::windows::core::RawPtr, ppin: ::windows::core::RawPtr, prtstart: *const i64, prtend: *const i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Notify(&*(&pnssbuffer3 as *const <INSSBuffer3 as ::windows::core::Abi>::Abi as *const <INSSBuffer3 as ::windows::core::DefaultType>::DefaultType), &*(&ppin as *const <super::DirectShow::IPin as ::windows::core::Abi>::Abi as *const <super::DirectShow::IPin as ::windows::core::DefaultType>::DefaultType), prtstart, prtend) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAMWMBufferPassCallback>, base.5, Notify::<Impl, OFFSET>)
    }
}
pub trait INSNetSourceCreatorImpl: Sized {
    fn Initialize();
    fn CreateNetSource();
    fn GetNetSourceProperties();
    fn GetNetSourceSharedNamespace();
    fn GetNetSourceAdminInterface();
    fn GetNumProtocolsSupported();
    fn GetProtocolName();
    fn Shutdown();
}
impl ::windows::core::RuntimeName for INSNetSourceCreator {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.INSNetSourceCreator";
}
impl INSNetSourceCreatorVtbl {
    pub const fn new<Impl: INSNetSourceCreatorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INSNetSourceCreatorVtbl {
        unsafe extern "system" fn Initialize<Impl: INSNetSourceCreatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Initialize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateNetSource<Impl: INSNetSourceCreatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszstreamname: super::super::Foundation::PWSTR, pmonitor: *mut ::core::ffi::c_void, pdata: *const u8, pusercontext: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, qwcontext: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateNetSource(
                &*(&pszstreamname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pmonitor as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                pdata,
                &*(&pusercontext as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&pcallback as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                qwcontext,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetSourceProperties<Impl: INSNetSourceCreatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszstreamname: super::super::Foundation::PWSTR, pppropertiesnode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNetSourceProperties(&*(&pszstreamname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pppropertiesnode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetSourceSharedNamespace<Impl: INSNetSourceCreatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsharednamespace: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNetSourceSharedNamespace(::core::mem::transmute_copy(&ppsharednamespace)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetSourceAdminInterface<Impl: INSNetSourceCreatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszstreamname: super::super::Foundation::PWSTR, pval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNetSourceAdminInterface(&*(&pszstreamname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumProtocolsSupported<Impl: INSNetSourceCreatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcprotocols: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNumProtocolsSupported(::core::mem::transmute_copy(&pcprotocols)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProtocolName<Impl: INSNetSourceCreatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwprotocolnum: u32, pwszprotocolname: super::super::Foundation::PWSTR, pcchprotocolname: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetProtocolName(dwprotocolnum, ::core::mem::transmute_copy(&pwszprotocolname), pcchprotocolname) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shutdown<Impl: INSNetSourceCreatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Shutdown() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INSNetSourceCreator>, base.5, Initialize::<Impl, OFFSET>, CreateNetSource::<Impl, OFFSET>, GetNetSourceProperties::<Impl, OFFSET>, GetNetSourceSharedNamespace::<Impl, OFFSET>, GetNetSourceAdminInterface::<Impl, OFFSET>, GetNumProtocolsSupported::<Impl, OFFSET>, GetProtocolName::<Impl, OFFSET>, Shutdown::<Impl, OFFSET>)
    }
}
pub trait INSSBufferImpl: Sized {
    fn GetLength();
    fn SetLength();
    fn GetMaxLength();
    fn GetBuffer();
    fn GetBufferAndLength();
}
impl ::windows::core::RuntimeName for INSSBuffer {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.INSSBuffer";
}
impl INSSBufferVtbl {
    pub const fn new<Impl: INSSBufferImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INSSBufferVtbl {
        unsafe extern "system" fn GetLength<Impl: INSSBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLength(::core::mem::transmute_copy(&pdwlength)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLength<Impl: INSSBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwlength: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetLength(dwlength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxLength<Impl: INSSBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMaxLength(::core::mem::transmute_copy(&pdwlength)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBuffer<Impl: INSSBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdwbuffer: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBuffer(::core::mem::transmute_copy(&ppdwbuffer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBufferAndLength<Impl: INSSBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdwbuffer: *mut *mut u8, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBufferAndLength(::core::mem::transmute_copy(&ppdwbuffer), ::core::mem::transmute_copy(&pdwlength)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INSSBuffer>, base.5, GetLength::<Impl, OFFSET>, SetLength::<Impl, OFFSET>, GetMaxLength::<Impl, OFFSET>, GetBuffer::<Impl, OFFSET>, GetBufferAndLength::<Impl, OFFSET>)
    }
}
pub trait INSSBuffer2Impl: Sized + INSSBufferImpl {
    fn GetSampleProperties();
    fn SetSampleProperties();
}
impl ::windows::core::RuntimeName for INSSBuffer2 {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.INSSBuffer2";
}
impl INSSBuffer2Vtbl {
    pub const fn new<Impl: INSSBuffer2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INSSBuffer2Vtbl {
        unsafe extern "system" fn GetSampleProperties<Impl: INSSBuffer2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbproperties: u32, pbproperties: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSampleProperties(cbproperties, ::core::mem::transmute_copy(&pbproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSampleProperties<Impl: INSSBuffer2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbproperties: u32, pbproperties: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSampleProperties(cbproperties, pbproperties) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INSSBuffer2>, base.5, GetSampleProperties::<Impl, OFFSET>, SetSampleProperties::<Impl, OFFSET>)
    }
}
pub trait INSSBuffer3Impl: Sized + INSSBuffer2Impl + INSSBufferImpl {
    fn SetProperty();
    fn GetProperty();
}
impl ::windows::core::RuntimeName for INSSBuffer3 {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.INSSBuffer3";
}
impl INSSBuffer3Vtbl {
    pub const fn new<Impl: INSSBuffer3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INSSBuffer3Vtbl {
        unsafe extern "system" fn SetProperty<Impl: INSSBuffer3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidbufferproperty: ::windows::core::GUID, pvbufferproperty: *const ::core::ffi::c_void, dwbufferpropertysize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetProperty(&*(&guidbufferproperty as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&pvbufferproperty as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), dwbufferpropertysize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: INSSBuffer3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidbufferproperty: ::windows::core::GUID, pvbufferproperty: *mut ::core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetProperty(&*(&guidbufferproperty as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvbufferproperty), pdwbufferpropertysize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INSSBuffer3>, base.5, SetProperty::<Impl, OFFSET>, GetProperty::<Impl, OFFSET>)
    }
}
pub trait INSSBuffer4Impl: Sized + INSSBuffer3Impl + INSSBuffer2Impl + INSSBufferImpl {
    fn GetPropertyCount();
    fn GetPropertyByIndex();
}
impl ::windows::core::RuntimeName for INSSBuffer4 {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.INSSBuffer4";
}
impl INSSBuffer4Vtbl {
    pub const fn new<Impl: INSSBuffer4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INSSBuffer4Vtbl {
        unsafe extern "system" fn GetPropertyCount<Impl: INSSBuffer4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcbufferproperties: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPropertyCount(::core::mem::transmute_copy(&pcbufferproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyByIndex<Impl: INSSBuffer4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwbufferpropertyindex: u32, pguidbufferproperty: *mut ::windows::core::GUID, pvbufferproperty: *mut ::core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPropertyByIndex(dwbufferpropertyindex, ::core::mem::transmute_copy(&pguidbufferproperty), ::core::mem::transmute_copy(&pvbufferproperty), pdwbufferpropertysize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INSSBuffer4>, base.5, GetPropertyCount::<Impl, OFFSET>, GetPropertyByIndex::<Impl, OFFSET>)
    }
}
pub trait IWMAddressAccessImpl: Sized {
    fn GetAccessEntryCount();
    fn GetAccessEntry();
    fn AddAccessEntry();
    fn RemoveAccessEntry();
}
impl ::windows::core::RuntimeName for IWMAddressAccess {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMAddressAccess";
}
impl IWMAddressAccessVtbl {
    pub const fn new<Impl: IWMAddressAccessImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMAddressAccessVtbl {
        unsafe extern "system" fn GetAccessEntryCount<Impl: IWMAddressAccessImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, pcentries: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAccessEntryCount(aetype, ::core::mem::transmute_copy(&pcentries)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAccessEntry<Impl: IWMAddressAccessImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, dwentrynum: u32, paddraccessentry: *mut WM_ADDRESS_ACCESSENTRY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAccessEntry(aetype, dwentrynum, ::core::mem::transmute_copy(&paddraccessentry)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddAccessEntry<Impl: IWMAddressAccessImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, paddraccessentry: *const WM_ADDRESS_ACCESSENTRY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddAccessEntry(aetype, &*(&paddraccessentry as *const <WM_ADDRESS_ACCESSENTRY as ::windows::core::Abi>::Abi as *const <WM_ADDRESS_ACCESSENTRY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAccessEntry<Impl: IWMAddressAccessImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, dwentrynum: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveAccessEntry(aetype, dwentrynum) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMAddressAccess>, base.5, GetAccessEntryCount::<Impl, OFFSET>, GetAccessEntry::<Impl, OFFSET>, AddAccessEntry::<Impl, OFFSET>, RemoveAccessEntry::<Impl, OFFSET>)
    }
}
pub trait IWMAddressAccess2Impl: Sized + IWMAddressAccessImpl {
    fn GetAccessEntryEx();
    fn AddAccessEntryEx();
}
impl ::windows::core::RuntimeName for IWMAddressAccess2 {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMAddressAccess2";
}
impl IWMAddressAccess2Vtbl {
    pub const fn new<Impl: IWMAddressAccess2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMAddressAccess2Vtbl {
        unsafe extern "system" fn GetAccessEntryEx<Impl: IWMAddressAccess2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, dwentrynum: u32, pbstraddress: *mut super::super::Foundation::BSTR, pbstrmask: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAccessEntryEx(aetype, dwentrynum, ::core::mem::transmute_copy(&pbstraddress), ::core::mem::transmute_copy(&pbstrmask)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddAccessEntryEx<Impl: IWMAddressAccess2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, bstraddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrmask: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddAccessEntryEx(aetype, &*(&bstraddress as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrmask as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMAddressAccess2>, base.5, GetAccessEntryEx::<Impl, OFFSET>, AddAccessEntryEx::<Impl, OFFSET>)
    }
}
pub trait IWMAuthorizerImpl: Sized {
    fn GetCertCount();
    fn GetCert();
    fn GetSharedData();
}
impl ::windows::core::RuntimeName for IWMAuthorizer {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMAuthorizer";
}
impl IWMAuthorizerVtbl {
    pub const fn new<Impl: IWMAuthorizerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMAuthorizerVtbl {
        unsafe extern "system" fn GetCertCount<Impl: IWMAuthorizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pccerts: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCertCount(::core::mem::transmute_copy(&pccerts)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCert<Impl: IWMAuthorizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwindex: u32, ppbcertdata: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCert(dwindex, ::core::mem::transmute_copy(&ppbcertdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSharedData<Impl: IWMAuthorizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcertindex: u32, pbshareddata: *const u8, pbcert: *const u8, ppbshareddata: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSharedData(dwcertindex, pbshareddata, pbcert, ::core::mem::transmute_copy(&ppbshareddata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMAuthorizer>, base.5, GetCertCount::<Impl, OFFSET>, GetCert::<Impl, OFFSET>, GetSharedData::<Impl, OFFSET>)
    }
}
pub trait IWMBackupRestorePropsImpl: Sized {
    fn GetPropCount();
    fn GetPropByIndex();
    fn GetPropByName();
    fn SetProp();
    fn RemoveProp();
    fn RemoveAllProps();
}
impl ::windows::core::RuntimeName for IWMBackupRestoreProps {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMBackupRestoreProps";
}
impl IWMBackupRestorePropsVtbl {
    pub const fn new<Impl: IWMBackupRestorePropsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMBackupRestorePropsVtbl {
        unsafe extern "system" fn GetPropCount<Impl: IWMBackupRestorePropsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcprops: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPropCount(::core::mem::transmute_copy(&pcprops)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropByIndex<Impl: IWMBackupRestorePropsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, windex: u16, pwszname: super::super::Foundation::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPropByIndex(windex, ::core::mem::transmute_copy(&pwszname), pcchnamelen, ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), pcblength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropByName<Impl: IWMBackupRestorePropsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPropByName(&*(&pszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), pcblength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProp<Impl: IWMBackupRestorePropsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetProp(&*(&pszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), r#type, pvalue, cblength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveProp<Impl: IWMBackupRestorePropsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcwszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveProp(&*(&pcwszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAllProps<Impl: IWMBackupRestorePropsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveAllProps() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMBackupRestoreProps>, base.5, GetPropCount::<Impl, OFFSET>, GetPropByIndex::<Impl, OFFSET>, GetPropByName::<Impl, OFFSET>, SetProp::<Impl, OFFSET>, RemoveProp::<Impl, OFFSET>, RemoveAllProps::<Impl, OFFSET>)
    }
}
pub trait IWMBandwidthSharingImpl: Sized + IWMStreamListImpl {
    fn GetType();
    fn SetType();
    fn GetBandwidth();
    fn SetBandwidth();
}
impl ::windows::core::RuntimeName for IWMBandwidthSharing {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMBandwidthSharing";
}
impl IWMBandwidthSharingVtbl {
    pub const fn new<Impl: IWMBandwidthSharingImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMBandwidthSharingVtbl {
        unsafe extern "system" fn GetType<Impl: IWMBandwidthSharingImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidtype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetType(::core::mem::transmute_copy(&pguidtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Impl: IWMBandwidthSharingImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetType(&*(&guidtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBandwidth<Impl: IWMBandwidthSharingImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwbitrate: *mut u32, pmsbufferwindow: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBandwidth(::core::mem::transmute_copy(&pdwbitrate), ::core::mem::transmute_copy(&pmsbufferwindow)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBandwidth<Impl: IWMBandwidthSharingImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwbitrate: u32, msbufferwindow: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetBandwidth(dwbitrate, msbufferwindow) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMBandwidthSharing>, base.5, GetType::<Impl, OFFSET>, SetType::<Impl, OFFSET>, GetBandwidth::<Impl, OFFSET>, SetBandwidth::<Impl, OFFSET>)
    }
}
pub trait IWMClientConnectionsImpl: Sized {
    fn GetClientCount();
    fn GetClientProperties();
}
impl ::windows::core::RuntimeName for IWMClientConnections {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMClientConnections";
}
impl IWMClientConnectionsVtbl {
    pub const fn new<Impl: IWMClientConnectionsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMClientConnectionsVtbl {
        unsafe extern "system" fn GetClientCount<Impl: IWMClientConnectionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcclients: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetClientCount(::core::mem::transmute_copy(&pcclients)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClientProperties<Impl: IWMClientConnectionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwclientnum: u32, pclientproperties: *mut WM_CLIENT_PROPERTIES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetClientProperties(dwclientnum, ::core::mem::transmute_copy(&pclientproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMClientConnections>, base.5, GetClientCount::<Impl, OFFSET>, GetClientProperties::<Impl, OFFSET>)
    }
}
pub trait IWMClientConnections2Impl: Sized + IWMClientConnectionsImpl {
    fn GetClientInfo();
}
impl ::windows::core::RuntimeName for IWMClientConnections2 {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMClientConnections2";
}
impl IWMClientConnections2Vtbl {
    pub const fn new<Impl: IWMClientConnections2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMClientConnections2Vtbl {
        unsafe extern "system" fn GetClientInfo<Impl: IWMClientConnections2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwclientnum: u32, pwsznetworkaddress: super::super::Foundation::PWSTR, pcchnetworkaddress: *mut u32, pwszport: super::super::Foundation::PWSTR, pcchport: *mut u32, pwszdnsname: super::super::Foundation::PWSTR, pcchdnsname: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetClientInfo(dwclientnum, ::core::mem::transmute_copy(&pwsznetworkaddress), pcchnetworkaddress, ::core::mem::transmute_copy(&pwszport), pcchport, ::core::mem::transmute_copy(&pwszdnsname), pcchdnsname) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMClientConnections2>, base.5, GetClientInfo::<Impl, OFFSET>)
    }
}
pub trait IWMCodecAMVideoAcceleratorImpl: Sized {
    fn SetAcceleratorInterface();
    fn NegotiateConnection();
    fn SetPlayerNotify();
}
impl ::windows::core::RuntimeName for IWMCodecAMVideoAccelerator {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMCodecAMVideoAccelerator";
}
impl IWMCodecAMVideoAcceleratorVtbl {
    pub const fn new<Impl: IWMCodecAMVideoAcceleratorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMCodecAMVideoAcceleratorVtbl {
        unsafe extern "system" fn SetAcceleratorInterface<Impl: IWMCodecAMVideoAcceleratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, piamva: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAcceleratorInterface(&*(&piamva as *const <super::DirectShow::IAMVideoAccelerator as ::windows::core::Abi>::Abi as *const <super::DirectShow::IAMVideoAccelerator as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NegotiateConnection<Impl: IWMCodecAMVideoAcceleratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmediatype: *const super::DirectShow::AM_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NegotiateConnection(&*(&pmediatype as *const <super::DirectShow::AM_MEDIA_TYPE as ::windows::core::Abi>::Abi as *const <super::DirectShow::AM_MEDIA_TYPE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlayerNotify<Impl: IWMCodecAMVideoAcceleratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phook: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPlayerNotify(&*(&phook as *const <IWMPlayerTimestampHook as ::windows::core::Abi>::Abi as *const <IWMPlayerTimestampHook as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMCodecAMVideoAccelerator>, base.5, SetAcceleratorInterface::<Impl, OFFSET>, NegotiateConnection::<Impl, OFFSET>, SetPlayerNotify::<Impl, OFFSET>)
    }
}
pub trait IWMCodecInfoImpl: Sized {
    fn GetCodecInfoCount();
    fn GetCodecFormatCount();
    fn GetCodecFormat();
}
impl ::windows::core::RuntimeName for IWMCodecInfo {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMCodecInfo";
}
impl IWMCodecInfoVtbl {
    pub const fn new<Impl: IWMCodecInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMCodecInfoVtbl {
        unsafe extern "system" fn GetCodecInfoCount<Impl: IWMCodecInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, pccodecs: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCodecInfoCount(&*(&guidtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pccodecs)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCodecFormatCount<Impl: IWMCodecInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pcformat: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCodecFormatCount(&*(&guidtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwcodecindex, ::core::mem::transmute_copy(&pcformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCodecFormat<Impl: IWMCodecInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32, ppistreamconfig: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCodecFormat(&*(&guidtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwcodecindex, dwformatindex, ::core::mem::transmute_copy(&ppistreamconfig)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMCodecInfo>, base.5, GetCodecInfoCount::<Impl, OFFSET>, GetCodecFormatCount::<Impl, OFFSET>, GetCodecFormat::<Impl, OFFSET>)
    }
}
pub trait IWMCodecInfo2Impl: Sized + IWMCodecInfoImpl {
    fn GetCodecName();
    fn GetCodecFormatDesc();
}
impl ::windows::core::RuntimeName for IWMCodecInfo2 {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMCodecInfo2";
}
impl IWMCodecInfo2Vtbl {
    pub const fn new<Impl: IWMCodecInfo2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMCodecInfo2Vtbl {
        unsafe extern "system" fn GetCodecName<Impl: IWMCodecInfo2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, wszname: super::super::Foundation::PWSTR, pcchname: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCodecName(&*(&guidtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwcodecindex, ::core::mem::transmute_copy(&wszname), pcchname) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCodecFormatDesc<Impl: IWMCodecInfo2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32, ppistreamconfig: *mut ::windows::core::RawPtr, wszdesc: super::super::Foundation::PWSTR, pcchdesc: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCodecFormatDesc(&*(&guidtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwcodecindex, dwformatindex, ::core::mem::transmute_copy(&ppistreamconfig), ::core::mem::transmute_copy(&wszdesc), pcchdesc) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMCodecInfo2>, base.5, GetCodecName::<Impl, OFFSET>, GetCodecFormatDesc::<Impl, OFFSET>)
    }
}
pub trait IWMCodecInfo3Impl: Sized + IWMCodecInfo2Impl + IWMCodecInfoImpl {
    fn GetCodecFormatProp();
    fn GetCodecProp();
    fn SetCodecEnumerationSetting();
    fn GetCodecEnumerationSetting();
}
impl ::windows::core::RuntimeName for IWMCodecInfo3 {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMCodecInfo3";
}
impl IWMCodecInfo3Vtbl {
    pub const fn new<Impl: IWMCodecInfo3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMCodecInfo3Vtbl {
        unsafe extern "system" fn GetCodecFormatProp<Impl: IWMCodecInfo3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCodecFormatProp(&*(&guidtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwcodecindex, dwformatindex, &*(&pszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), pdwsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCodecProp<Impl: IWMCodecInfo3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCodecProp(&*(&guidtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwcodecindex, &*(&pszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), pdwsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCodecEnumerationSetting<Impl: IWMCodecInfo3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, dwsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetCodecEnumerationSetting(&*(&guidtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwcodecindex, &*(&pszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), r#type, pvalue, dwsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCodecEnumerationSetting<Impl: IWMCodecInfo3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCodecEnumerationSetting(&*(&guidtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwcodecindex, &*(&pszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), pdwsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMCodecInfo3>, base.5, GetCodecFormatProp::<Impl, OFFSET>, GetCodecProp::<Impl, OFFSET>, SetCodecEnumerationSetting::<Impl, OFFSET>, GetCodecEnumerationSetting::<Impl, OFFSET>)
    }
}
pub trait IWMCodecVideoAcceleratorImpl: Sized {
    fn NegotiateConnection();
    fn SetPlayerNotify();
}
impl ::windows::core::RuntimeName for IWMCodecVideoAccelerator {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMCodecVideoAccelerator";
}
impl IWMCodecVideoAcceleratorVtbl {
    pub const fn new<Impl: IWMCodecVideoAcceleratorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMCodecVideoAcceleratorVtbl {
        unsafe extern "system" fn NegotiateConnection<Impl: IWMCodecVideoAcceleratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, piamva: ::windows::core::RawPtr, pmediatype: *const super::DirectShow::AM_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NegotiateConnection(&*(&piamva as *const <super::DirectShow::IAMVideoAccelerator as ::windows::core::Abi>::Abi as *const <super::DirectShow::IAMVideoAccelerator as ::windows::core::DefaultType>::DefaultType), &*(&pmediatype as *const <super::DirectShow::AM_MEDIA_TYPE as ::windows::core::Abi>::Abi as *const <super::DirectShow::AM_MEDIA_TYPE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlayerNotify<Impl: IWMCodecVideoAcceleratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phook: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPlayerNotify(&*(&phook as *const <IWMPlayerTimestampHook as ::windows::core::Abi>::Abi as *const <IWMPlayerTimestampHook as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMCodecVideoAccelerator>, base.5, NegotiateConnection::<Impl, OFFSET>, SetPlayerNotify::<Impl, OFFSET>)
    }
}
pub trait IWMCredentialCallbackImpl: Sized {
    fn AcquireCredentials();
}
impl ::windows::core::RuntimeName for IWMCredentialCallback {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMCredentialCallback";
}
impl IWMCredentialCallbackVtbl {
    pub const fn new<Impl: IWMCredentialCallbackImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMCredentialCallbackVtbl {
        unsafe extern "system" fn AcquireCredentials<Impl: IWMCredentialCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszrealm: super::super::Foundation::PWSTR, pwszsite: super::super::Foundation::PWSTR, pwszuser: super::super::Foundation::PWSTR, cchuser: u32, pwszpassword: super::super::Foundation::PWSTR, cchpassword: u32, hrstatus: ::windows::core::HRESULT, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AcquireCredentials(
                &*(&pwszrealm as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pwszsite as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pwszuser),
                cchuser,
                ::core::mem::transmute_copy(&pwszpassword),
                cchpassword,
                hrstatus,
                ::core::mem::transmute_copy(&pdwflags),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMCredentialCallback>, base.5, AcquireCredentials::<Impl, OFFSET>)
    }
}
pub trait IWMDRMEditorImpl: Sized {
    fn GetDRMProperty();
}
impl ::windows::core::RuntimeName for IWMDRMEditor {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMDRMEditor";
}
impl IWMDRMEditorVtbl {
    pub const fn new<Impl: IWMDRMEditorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMDRMEditorVtbl {
        unsafe extern "system" fn GetDRMProperty<Impl: IWMDRMEditorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwstrname: super::super::Foundation::PWSTR, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDRMProperty(&*(&pwstrname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwtype), ::core::mem::transmute_copy(&pvalue), pcblength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMDRMEditor>, base.5, GetDRMProperty::<Impl, OFFSET>)
    }
}
pub trait IWMDRMMessageParserImpl: Sized {
    fn ParseRegistrationReqMsg();
    fn ParseLicenseRequestMsg();
}
impl ::windows::core::RuntimeName for IWMDRMMessageParser {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMDRMMessageParser";
}
impl IWMDRMMessageParserVtbl {
    pub const fn new<Impl: IWMDRMMessageParserImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMDRMMessageParserVtbl {
        unsafe extern "system" fn ParseRegistrationReqMsg<Impl: IWMDRMMessageParserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbregistrationreqmsg: *const u8, cbregistrationreqmsg: u32, ppdevicecert: *mut ::windows::core::RawPtr, pdeviceserialnumber: *mut DRM_VAL16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ParseRegistrationReqMsg(pbregistrationreqmsg, cbregistrationreqmsg, ::core::mem::transmute_copy(&ppdevicecert), ::core::mem::transmute_copy(&pdeviceserialnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ParseLicenseRequestMsg<Impl: IWMDRMMessageParserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pblicenserequestmsg: *const u8, cblicenserequestmsg: u32, ppdevicecert: *mut ::windows::core::RawPtr, pdeviceserialnumber: *mut DRM_VAL16, pbstraction: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ParseLicenseRequestMsg(pblicenserequestmsg, cblicenserequestmsg, ::core::mem::transmute_copy(&ppdevicecert), ::core::mem::transmute_copy(&pdeviceserialnumber), ::core::mem::transmute_copy(&pbstraction)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMDRMMessageParser>, base.5, ParseRegistrationReqMsg::<Impl, OFFSET>, ParseLicenseRequestMsg::<Impl, OFFSET>)
    }
}
pub trait IWMDRMReaderImpl: Sized {
    fn AcquireLicense();
    fn CancelLicenseAcquisition();
    fn Individualize();
    fn CancelIndividualization();
    fn MonitorLicenseAcquisition();
    fn CancelMonitorLicenseAcquisition();
    fn SetDRMProperty();
    fn GetDRMProperty();
}
impl ::windows::core::RuntimeName for IWMDRMReader {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMDRMReader";
}
impl IWMDRMReaderVtbl {
    pub const fn new<Impl: IWMDRMReaderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMDRMReaderVtbl {
        unsafe extern "system" fn AcquireLicense<Impl: IWMDRMReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AcquireLicense(dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelLicenseAcquisition<Impl: IWMDRMReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CancelLicenseAcquisition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Individualize<Impl: IWMDRMReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Individualize(dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelIndividualization<Impl: IWMDRMReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CancelIndividualization() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MonitorLicenseAcquisition<Impl: IWMDRMReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MonitorLicenseAcquisition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelMonitorLicenseAcquisition<Impl: IWMDRMReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CancelMonitorLicenseAcquisition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDRMProperty<Impl: IWMDRMReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwstrname: super::super::Foundation::PWSTR, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDRMProperty(&*(&pwstrname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dwtype, pvalue, cblength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDRMProperty<Impl: IWMDRMReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwstrname: super::super::Foundation::PWSTR, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDRMProperty(&*(&pwstrname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwtype), ::core::mem::transmute_copy(&pvalue), pcblength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMDRMReader>, base.5, AcquireLicense::<Impl, OFFSET>, CancelLicenseAcquisition::<Impl, OFFSET>, Individualize::<Impl, OFFSET>, CancelIndividualization::<Impl, OFFSET>, MonitorLicenseAcquisition::<Impl, OFFSET>, CancelMonitorLicenseAcquisition::<Impl, OFFSET>, SetDRMProperty::<Impl, OFFSET>, GetDRMProperty::<Impl, OFFSET>)
    }
}
pub trait IWMDRMReader2Impl: Sized + IWMDRMReaderImpl {
    fn SetEvaluateOutputLevelLicenses();
    fn GetPlayOutputLevels();
    fn GetCopyOutputLevels();
    fn TryNextLicense();
}
impl ::windows::core::RuntimeName for IWMDRMReader2 {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMDRMReader2";
}
impl IWMDRMReader2Vtbl {
    pub const fn new<Impl: IWMDRMReader2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMDRMReader2Vtbl {
        unsafe extern "system" fn SetEvaluateOutputLevelLicenses<Impl: IWMDRMReader2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fevaluate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetEvaluateOutputLevelLicenses(&*(&fevaluate as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPlayOutputLevels<Impl: IWMDRMReader2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pplayopl: *mut DRM_PLAY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPlayOutputLevels(::core::mem::transmute_copy(&pplayopl), pcblength, ::core::mem::transmute_copy(&pdwminappcompliancelevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCopyOutputLevels<Impl: IWMDRMReader2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcopyopl: *mut DRM_COPY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCopyOutputLevels(::core::mem::transmute_copy(&pcopyopl), pcblength, ::core::mem::transmute_copy(&pdwminappcompliancelevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryNextLicense<Impl: IWMDRMReader2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryNextLicense() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMDRMReader2>, base.5, SetEvaluateOutputLevelLicenses::<Impl, OFFSET>, GetPlayOutputLevels::<Impl, OFFSET>, GetCopyOutputLevels::<Impl, OFFSET>, TryNextLicense::<Impl, OFFSET>)
    }
}
pub trait IWMDRMReader3Impl: Sized + IWMDRMReader2Impl + IWMDRMReaderImpl {
    fn GetInclusionList();
}
impl ::windows::core::RuntimeName for IWMDRMReader3 {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMDRMReader3";
}
impl IWMDRMReader3Vtbl {
    pub const fn new<Impl: IWMDRMReader3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMDRMReader3Vtbl {
        unsafe extern "system" fn GetInclusionList<Impl: IWMDRMReader3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppguids: *mut *mut ::windows::core::GUID, pcguids: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInclusionList(::core::mem::transmute_copy(&ppguids), ::core::mem::transmute_copy(&pcguids)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMDRMReader3>, base.5, GetInclusionList::<Impl, OFFSET>)
    }
}
pub trait IWMDRMTranscryptionManagerImpl: Sized {
    fn CreateTranscryptor();
}
impl ::windows::core::RuntimeName for IWMDRMTranscryptionManager {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMDRMTranscryptionManager";
}
impl IWMDRMTranscryptionManagerVtbl {
    pub const fn new<Impl: IWMDRMTranscryptionManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMDRMTranscryptionManagerVtbl {
        unsafe extern "system" fn CreateTranscryptor<Impl: IWMDRMTranscryptionManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptranscryptor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateTranscryptor(::core::mem::transmute_copy(&pptranscryptor)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMDRMTranscryptionManager>, base.5, CreateTranscryptor::<Impl, OFFSET>)
    }
}
pub trait IWMDRMTranscryptorImpl: Sized {
    fn Initialize();
    fn Seek();
    fn Read();
    fn Close();
}
impl ::windows::core::RuntimeName for IWMDRMTranscryptor {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMDRMTranscryptor";
}
impl IWMDRMTranscryptorVtbl {
    pub const fn new<Impl: IWMDRMTranscryptorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMDRMTranscryptorVtbl {
        unsafe extern "system" fn Initialize<Impl: IWMDRMTranscryptorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pblicenserequestmsg: *mut u8, cblicenserequestmsg: u32, pplicenseresponsemsg: *mut ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Initialize(
                &*(&bstrfilename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                pblicenserequestmsg,
                cblicenserequestmsg,
                ::core::mem::transmute_copy(&pplicenseresponsemsg),
                &*(&pcallback as *const <IWMStatusCallback as ::windows::core::Abi>::Abi as *const <IWMStatusCallback as ::windows::core::DefaultType>::DefaultType),
                &*(&pvcontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Seek<Impl: IWMDRMTranscryptorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hnstime: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Seek(hnstime) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Read<Impl: IWMDRMTranscryptorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbdata: *const u8, pcbdata: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Read(pbdata, pcbdata) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IWMDRMTranscryptorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Close() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMDRMTranscryptor>, base.5, Initialize::<Impl, OFFSET>, Seek::<Impl, OFFSET>, Read::<Impl, OFFSET>, Close::<Impl, OFFSET>)
    }
}
pub trait IWMDRMTranscryptor2Impl: Sized + IWMDRMTranscryptorImpl {
    fn SeekEx();
    fn ZeroAdjustTimestamps();
    fn GetSeekStartTime();
    fn GetDuration();
}
impl ::windows::core::RuntimeName for IWMDRMTranscryptor2 {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMDRMTranscryptor2";
}
impl IWMDRMTranscryptor2Vtbl {
    pub const fn new<Impl: IWMDRMTranscryptor2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMDRMTranscryptor2Vtbl {
        unsafe extern "system" fn SeekEx<Impl: IWMDRMTranscryptor2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cnsstarttime: u64, cnsduration: u64, flrate: f32, fincludefileheader: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SeekEx(cnsstarttime, cnsduration, flrate, &*(&fincludefileheader as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ZeroAdjustTimestamps<Impl: IWMDRMTranscryptor2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ZeroAdjustTimestamps(&*(&fenable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSeekStartTime<Impl: IWMDRMTranscryptor2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcnstime: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSeekStartTime(::core::mem::transmute_copy(&pcnstime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDuration<Impl: IWMDRMTranscryptor2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcnsduration: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDuration(::core::mem::transmute_copy(&pcnsduration)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMDRMTranscryptor2>, base.5, SeekEx::<Impl, OFFSET>, ZeroAdjustTimestamps::<Impl, OFFSET>, GetSeekStartTime::<Impl, OFFSET>, GetDuration::<Impl, OFFSET>)
    }
}
pub trait IWMDRMWriterImpl: Sized {
    fn GenerateKeySeed();
    fn GenerateKeyID();
    fn GenerateSigningKeyPair();
    fn SetDRMAttribute();
}
impl ::windows::core::RuntimeName for IWMDRMWriter {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMDRMWriter";
}
impl IWMDRMWriterVtbl {
    pub const fn new<Impl: IWMDRMWriterImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMDRMWriterVtbl {
        unsafe extern "system" fn GenerateKeySeed<Impl: IWMDRMWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszkeyseed: super::super::Foundation::PWSTR, pcwchlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GenerateKeySeed(::core::mem::transmute_copy(&pwszkeyseed), pcwchlength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenerateKeyID<Impl: IWMDRMWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszkeyid: super::super::Foundation::PWSTR, pcwchlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GenerateKeyID(::core::mem::transmute_copy(&pwszkeyid), pcwchlength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenerateSigningKeyPair<Impl: IWMDRMWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszprivkey: super::super::Foundation::PWSTR, pcwchprivkeylength: *mut u32, pwszpubkey: super::super::Foundation::PWSTR, pcwchpubkeylength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GenerateSigningKeyPair(::core::mem::transmute_copy(&pwszprivkey), pcwchprivkeylength, ::core::mem::transmute_copy(&pwszpubkey), pcwchpubkeylength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDRMAttribute<Impl: IWMDRMWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDRMAttribute(wstreamnum, &*(&pszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), r#type, pvalue, cblength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMDRMWriter>, base.5, GenerateKeySeed::<Impl, OFFSET>, GenerateKeyID::<Impl, OFFSET>, GenerateSigningKeyPair::<Impl, OFFSET>, SetDRMAttribute::<Impl, OFFSET>)
    }
}
pub trait IWMDRMWriter2Impl: Sized + IWMDRMWriterImpl {
    fn SetWMDRMNetEncryption();
}
impl ::windows::core::RuntimeName for IWMDRMWriter2 {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMDRMWriter2";
}
impl IWMDRMWriter2Vtbl {
    pub const fn new<Impl: IWMDRMWriter2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMDRMWriter2Vtbl {
        unsafe extern "system" fn SetWMDRMNetEncryption<Impl: IWMDRMWriter2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fsamplesencrypted: super::super::Foundation::BOOL, pbkeyid: *const u8, cbkeyid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetWMDRMNetEncryption(&*(&fsamplesencrypted as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), pbkeyid, cbkeyid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMDRMWriter2>, base.5, SetWMDRMNetEncryption::<Impl, OFFSET>)
    }
}
pub trait IWMDRMWriter3Impl: Sized + IWMDRMWriter2Impl + IWMDRMWriterImpl {
    fn SetProtectStreamSamples();
}
impl ::windows::core::RuntimeName for IWMDRMWriter3 {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMDRMWriter3";
}
impl IWMDRMWriter3Vtbl {
    pub const fn new<Impl: IWMDRMWriter3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMDRMWriter3Vtbl {
        unsafe extern "system" fn SetProtectStreamSamples<Impl: IWMDRMWriter3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pimportinitstruct: *const WMDRM_IMPORT_INIT_STRUCT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetProtectStreamSamples(&*(&pimportinitstruct as *const <WMDRM_IMPORT_INIT_STRUCT as ::windows::core::Abi>::Abi as *const <WMDRM_IMPORT_INIT_STRUCT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMDRMWriter3>, base.5, SetProtectStreamSamples::<Impl, OFFSET>)
    }
}
pub trait IWMDeviceRegistrationImpl: Sized {
    fn RegisterDevice();
    fn UnregisterDevice();
    fn GetRegistrationStats();
    fn GetFirstRegisteredDevice();
    fn GetNextRegisteredDevice();
    fn GetRegisteredDeviceByID();
}
impl ::windows::core::RuntimeName for IWMDeviceRegistration {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMDeviceRegistration";
}
impl IWMDeviceRegistrationVtbl {
    pub const fn new<Impl: IWMDeviceRegistrationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMDeviceRegistrationVtbl {
        unsafe extern "system" fn RegisterDevice<Impl: IWMDeviceRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: DRM_VAL16, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RegisterDevice(dwregistertype, pbcertificate, cbcertificate, &*(&serialnumber as *const <DRM_VAL16 as ::windows::core::Abi>::Abi as *const <DRM_VAL16 as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterDevice<Impl: IWMDeviceRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: DRM_VAL16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnregisterDevice(dwregistertype, pbcertificate, cbcertificate, &*(&serialnumber as *const <DRM_VAL16 as ::windows::core::Abi>::Abi as *const <DRM_VAL16 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRegistrationStats<Impl: IWMDeviceRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwregistertype: u32, pcregistereddevices: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRegistrationStats(dwregistertype, ::core::mem::transmute_copy(&pcregistereddevices)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFirstRegisteredDevice<Impl: IWMDeviceRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwregistertype: u32, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFirstRegisteredDevice(dwregistertype, ::core::mem::transmute_copy(&ppdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNextRegisteredDevice<Impl: IWMDeviceRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNextRegisteredDevice(::core::mem::transmute_copy(&ppdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRegisteredDeviceByID<Impl: IWMDeviceRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: DRM_VAL16, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRegisteredDeviceByID(dwregistertype, pbcertificate, cbcertificate, &*(&serialnumber as *const <DRM_VAL16 as ::windows::core::Abi>::Abi as *const <DRM_VAL16 as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMDeviceRegistration>, base.5, RegisterDevice::<Impl, OFFSET>, UnregisterDevice::<Impl, OFFSET>, GetRegistrationStats::<Impl, OFFSET>, GetFirstRegisteredDevice::<Impl, OFFSET>, GetNextRegisteredDevice::<Impl, OFFSET>, GetRegisteredDeviceByID::<Impl, OFFSET>)
    }
}
pub trait IWMGetSecureChannelImpl: Sized {
    fn GetPeerSecureChannelInterface();
}
impl ::windows::core::RuntimeName for IWMGetSecureChannel {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMGetSecureChannel";
}
impl IWMGetSecureChannelVtbl {
    pub const fn new<Impl: IWMGetSecureChannelImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMGetSecureChannelVtbl {
        unsafe extern "system" fn GetPeerSecureChannelInterface<Impl: IWMGetSecureChannelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppeer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPeerSecureChannelInterface(::core::mem::transmute_copy(&pppeer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMGetSecureChannel>, base.5, GetPeerSecureChannelInterface::<Impl, OFFSET>)
    }
}
pub trait IWMHeaderInfoImpl: Sized {
    fn GetAttributeCount();
    fn GetAttributeByIndex();
    fn GetAttributeByName();
    fn SetAttribute();
    fn GetMarkerCount();
    fn GetMarker();
    fn AddMarker();
    fn RemoveMarker();
    fn GetScriptCount();
    fn GetScript();
    fn AddScript();
    fn RemoveScript();
}
impl ::windows::core::RuntimeName for IWMHeaderInfo {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMHeaderInfo";
}
impl IWMHeaderInfoVtbl {
    pub const fn new<Impl: IWMHeaderInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMHeaderInfoVtbl {
        unsafe extern "system" fn GetAttributeCount<Impl: IWMHeaderInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pcattributes: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAttributeCount(wstreamnum, ::core::mem::transmute_copy(&pcattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeByIndex<Impl: IWMHeaderInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, windex: u16, pwstreamnum: *mut u16, pwszname: super::super::Foundation::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAttributeByIndex(windex, pwstreamnum, ::core::mem::transmute_copy(&pwszname), pcchnamelen, ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), pcblength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeByName<Impl: IWMHeaderInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwstreamnum: *mut u16, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAttributeByName(pwstreamnum, &*(&pszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), pcblength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttribute<Impl: IWMHeaderInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAttribute(wstreamnum, &*(&pszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), r#type, pvalue, cblength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMarkerCount<Impl: IWMHeaderInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcmarkers: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMarkerCount(::core::mem::transmute_copy(&pcmarkers)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMarker<Impl: IWMHeaderInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, windex: u16, pwszmarkername: super::super::Foundation::PWSTR, pcchmarkernamelen: *mut u16, pcnsmarkertime: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMarker(windex, ::core::mem::transmute_copy(&pwszmarkername), pcchmarkernamelen, ::core::mem::transmute_copy(&pcnsmarkertime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddMarker<Impl: IWMHeaderInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszmarkername: super::super::Foundation::PWSTR, cnsmarkertime: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddMarker(&*(&pwszmarkername as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cnsmarkertime) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMarker<Impl: IWMHeaderInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, windex: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveMarker(windex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScriptCount<Impl: IWMHeaderInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcscripts: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetScriptCount(::core::mem::transmute_copy(&pcscripts)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScript<Impl: IWMHeaderInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, windex: u16, pwsztype: super::super::Foundation::PWSTR, pcchtypelen: *mut u16, pwszcommand: super::super::Foundation::PWSTR, pcchcommandlen: *mut u16, pcnsscripttime: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetScript(windex, ::core::mem::transmute_copy(&pwsztype), pcchtypelen, ::core::mem::transmute_copy(&pwszcommand), pcchcommandlen, ::core::mem::transmute_copy(&pcnsscripttime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddScript<Impl: IWMHeaderInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwsztype: super::super::Foundation::PWSTR, pwszcommand: super::super::Foundation::PWSTR, cnsscripttime: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddScript(&*(&pwsztype as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pwszcommand as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cnsscripttime) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveScript<Impl: IWMHeaderInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, windex: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveScript(windex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMHeaderInfo>, base.5, GetAttributeCount::<Impl, OFFSET>, GetAttributeByIndex::<Impl, OFFSET>, GetAttributeByName::<Impl, OFFSET>, SetAttribute::<Impl, OFFSET>, GetMarkerCount::<Impl, OFFSET>, GetMarker::<Impl, OFFSET>, AddMarker::<Impl, OFFSET>, RemoveMarker::<Impl, OFFSET>, GetScriptCount::<Impl, OFFSET>, GetScript::<Impl, OFFSET>, AddScript::<Impl, OFFSET>, RemoveScript::<Impl, OFFSET>)
    }
}
pub trait IWMHeaderInfo2Impl: Sized + IWMHeaderInfoImpl {
    fn GetCodecInfoCount();
    fn GetCodecInfo();
}
impl ::windows::core::RuntimeName for IWMHeaderInfo2 {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMHeaderInfo2";
}
impl IWMHeaderInfo2Vtbl {
    pub const fn new<Impl: IWMHeaderInfo2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMHeaderInfo2Vtbl {
        unsafe extern "system" fn GetCodecInfoCount<Impl: IWMHeaderInfo2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pccodecinfos: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCodecInfoCount(::core::mem::transmute_copy(&pccodecinfos)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCodecInfo<Impl: IWMHeaderInfo2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, windex: u32, pcchname: *mut u16, pwszname: super::super::Foundation::PWSTR, pcchdescription: *mut u16, pwszdescription: super::super::Foundation::PWSTR, pcodectype: *mut WMT_CODEC_INFO_TYPE, pcbcodecinfo: *mut u16, pbcodecinfo: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCodecInfo(windex, pcchname, ::core::mem::transmute_copy(&pwszname), pcchdescription, ::core::mem::transmute_copy(&pwszdescription), ::core::mem::transmute_copy(&pcodectype), pcbcodecinfo, ::core::mem::transmute_copy(&pbcodecinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMHeaderInfo2>, base.5, GetCodecInfoCount::<Impl, OFFSET>, GetCodecInfo::<Impl, OFFSET>)
    }
}
pub trait IWMHeaderInfo3Impl: Sized + IWMHeaderInfo2Impl + IWMHeaderInfoImpl {
    fn GetAttributeCountEx();
    fn GetAttributeIndices();
    fn GetAttributeByIndexEx();
    fn ModifyAttribute();
    fn AddAttribute();
    fn DeleteAttribute();
    fn AddCodecInfo();
}
impl ::windows::core::RuntimeName for IWMHeaderInfo3 {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMHeaderInfo3";
}
impl IWMHeaderInfo3Vtbl {
    pub const fn new<Impl: IWMHeaderInfo3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMHeaderInfo3Vtbl {
        unsafe extern "system" fn GetAttributeCountEx<Impl: IWMHeaderInfo3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pcattributes: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAttributeCountEx(wstreamnum, ::core::mem::transmute_copy(&pcattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeIndices<Impl: IWMHeaderInfo3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pwszname: super::super::Foundation::PWSTR, pwlangindex: *const u16, pwindices: *mut u16, pwcount: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAttributeIndices(wstreamnum, &*(&pwszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), pwlangindex, ::core::mem::transmute_copy(&pwindices), pwcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeByIndexEx<Impl: IWMHeaderInfo3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, windex: u16, pwszname: super::super::Foundation::PWSTR, pwnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pwlangindex: *mut u16, pvalue: *mut u8, pdwdatalength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAttributeByIndexEx(wstreamnum, windex, ::core::mem::transmute_copy(&pwszname), pwnamelen, ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pwlangindex), ::core::mem::transmute_copy(&pvalue), pdwdatalength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifyAttribute<Impl: IWMHeaderInfo3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, windex: u16, r#type: WMT_ATTR_DATATYPE, wlangindex: u16, pvalue: *const u8, dwlength: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ModifyAttribute(wstreamnum, windex, r#type, wlangindex, pvalue, dwlength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddAttribute<Impl: IWMHeaderInfo3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pszname: super::super::Foundation::PWSTR, pwindex: *mut u16, r#type: WMT_ATTR_DATATYPE, wlangindex: u16, pvalue: *const u8, dwlength: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddAttribute(wstreamnum, &*(&pszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pwindex), r#type, wlangindex, pvalue, dwlength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteAttribute<Impl: IWMHeaderInfo3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, windex: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteAttribute(wstreamnum, windex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddCodecInfo<Impl: IWMHeaderInfo3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, pwszdescription: super::super::Foundation::PWSTR, codectype: WMT_CODEC_INFO_TYPE, cbcodecinfo: u16, pbcodecinfo: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddCodecInfo(&*(&pwszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pwszdescription as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), codectype, cbcodecinfo, pbcodecinfo) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMHeaderInfo3>, base.5, GetAttributeCountEx::<Impl, OFFSET>, GetAttributeIndices::<Impl, OFFSET>, GetAttributeByIndexEx::<Impl, OFFSET>, ModifyAttribute::<Impl, OFFSET>, AddAttribute::<Impl, OFFSET>, DeleteAttribute::<Impl, OFFSET>, AddCodecInfo::<Impl, OFFSET>)
    }
}
pub trait IWMIStreamPropsImpl: Sized {
    fn GetProperty();
}
impl ::windows::core::RuntimeName for IWMIStreamProps {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMIStreamProps";
}
impl IWMIStreamPropsVtbl {
    pub const fn new<Impl: IWMIStreamPropsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMIStreamPropsVtbl {
        unsafe extern "system" fn GetProperty<Impl: IWMIStreamPropsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetProperty(&*(&pszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), pdwsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMIStreamProps>, base.5, GetProperty::<Impl, OFFSET>)
    }
}
pub trait IWMImageInfoImpl: Sized {
    fn GetImageCount();
    fn GetImage();
}
impl ::windows::core::RuntimeName for IWMImageInfo {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMImageInfo";
}
impl IWMImageInfoVtbl {
    pub const fn new<Impl: IWMImageInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMImageInfoVtbl {
        unsafe extern "system" fn GetImageCount<Impl: IWMImageInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcimages: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetImageCount(::core::mem::transmute_copy(&pcimages)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImage<Impl: IWMImageInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, windex: u32, pcchmimetype: *mut u16, pwszmimetype: super::super::Foundation::PWSTR, pcchdescription: *mut u16, pwszdescription: super::super::Foundation::PWSTR, pimagetype: *mut u16, pcbimagedata: *mut u32, pbimagedata: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetImage(windex, pcchmimetype, ::core::mem::transmute_copy(&pwszmimetype), pcchdescription, ::core::mem::transmute_copy(&pwszdescription), ::core::mem::transmute_copy(&pimagetype), pcbimagedata, ::core::mem::transmute_copy(&pbimagedata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMImageInfo>, base.5, GetImageCount::<Impl, OFFSET>, GetImage::<Impl, OFFSET>)
    }
}
pub trait IWMIndexerImpl: Sized {
    fn StartIndexing();
    fn Cancel();
}
impl ::windows::core::RuntimeName for IWMIndexer {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMIndexer";
}
impl IWMIndexerVtbl {
    pub const fn new<Impl: IWMIndexerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMIndexerVtbl {
        unsafe extern "system" fn StartIndexing<Impl: IWMIndexerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartIndexing(&*(&pwszurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pcallback as *const <IWMStatusCallback as ::windows::core::Abi>::Abi as *const <IWMStatusCallback as ::windows::core::DefaultType>::DefaultType), &*(&pvcontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: IWMIndexerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMIndexer>, base.5, StartIndexing::<Impl, OFFSET>, Cancel::<Impl, OFFSET>)
    }
}
pub trait IWMIndexer2Impl: Sized + IWMIndexerImpl {
    fn Configure();
}
impl ::windows::core::RuntimeName for IWMIndexer2 {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMIndexer2";
}
impl IWMIndexer2Vtbl {
    pub const fn new<Impl: IWMIndexer2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMIndexer2Vtbl {
        unsafe extern "system" fn Configure<Impl: IWMIndexer2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, nindexertype: WMT_INDEXER_TYPE, pvinterval: *const ::core::ffi::c_void, pvindextype: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Configure(wstreamnum, nindexertype, &*(&pvinterval as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), &*(&pvindextype as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMIndexer2>, base.5, Configure::<Impl, OFFSET>)
    }
}
pub trait IWMInputMediaPropsImpl: Sized + IWMMediaPropsImpl {
    fn GetConnectionName();
    fn GetGroupName();
}
impl ::windows::core::RuntimeName for IWMInputMediaProps {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMInputMediaProps";
}
impl IWMInputMediaPropsVtbl {
    pub const fn new<Impl: IWMInputMediaPropsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMInputMediaPropsVtbl {
        unsafe extern "system" fn GetConnectionName<Impl: IWMInputMediaPropsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetConnectionName(::core::mem::transmute_copy(&pwszname), pcchname) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGroupName<Impl: IWMInputMediaPropsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetGroupName(::core::mem::transmute_copy(&pwszname), pcchname) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMInputMediaProps>, base.5, GetConnectionName::<Impl, OFFSET>, GetGroupName::<Impl, OFFSET>)
    }
}
pub trait IWMLanguageListImpl: Sized {
    fn GetLanguageCount();
    fn GetLanguageDetails();
    fn AddLanguageByRFC1766String();
}
impl ::windows::core::RuntimeName for IWMLanguageList {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMLanguageList";
}
impl IWMLanguageListVtbl {
    pub const fn new<Impl: IWMLanguageListImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMLanguageListVtbl {
        unsafe extern "system" fn GetLanguageCount<Impl: IWMLanguageListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwcount: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLanguageCount(::core::mem::transmute_copy(&pwcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLanguageDetails<Impl: IWMLanguageListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, windex: u16, pwszlanguagestring: super::super::Foundation::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLanguageDetails(windex, ::core::mem::transmute_copy(&pwszlanguagestring), pcchlanguagestringlength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddLanguageByRFC1766String<Impl: IWMLanguageListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszlanguagestring: super::super::Foundation::PWSTR, pwindex: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddLanguageByRFC1766String(&*(&pwszlanguagestring as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMLanguageList>, base.5, GetLanguageCount::<Impl, OFFSET>, GetLanguageDetails::<Impl, OFFSET>, AddLanguageByRFC1766String::<Impl, OFFSET>)
    }
}
pub trait IWMLicenseBackupImpl: Sized {
    fn BackupLicenses();
    fn CancelLicenseBackup();
}
impl ::windows::core::RuntimeName for IWMLicenseBackup {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMLicenseBackup";
}
impl IWMLicenseBackupVtbl {
    pub const fn new<Impl: IWMLicenseBackupImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMLicenseBackupVtbl {
        unsafe extern "system" fn BackupLicenses<Impl: IWMLicenseBackupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BackupLicenses(dwflags, &*(&pcallback as *const <IWMStatusCallback as ::windows::core::Abi>::Abi as *const <IWMStatusCallback as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelLicenseBackup<Impl: IWMLicenseBackupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CancelLicenseBackup() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMLicenseBackup>, base.5, BackupLicenses::<Impl, OFFSET>, CancelLicenseBackup::<Impl, OFFSET>)
    }
}
pub trait IWMLicenseRestoreImpl: Sized {
    fn RestoreLicenses();
    fn CancelLicenseRestore();
}
impl ::windows::core::RuntimeName for IWMLicenseRestore {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMLicenseRestore";
}
impl IWMLicenseRestoreVtbl {
    pub const fn new<Impl: IWMLicenseRestoreImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMLicenseRestoreVtbl {
        unsafe extern "system" fn RestoreLicenses<Impl: IWMLicenseRestoreImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RestoreLicenses(dwflags, &*(&pcallback as *const <IWMStatusCallback as ::windows::core::Abi>::Abi as *const <IWMStatusCallback as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelLicenseRestore<Impl: IWMLicenseRestoreImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CancelLicenseRestore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMLicenseRestore>, base.5, RestoreLicenses::<Impl, OFFSET>, CancelLicenseRestore::<Impl, OFFSET>)
    }
}
pub trait IWMLicenseRevocationAgentImpl: Sized {
    fn GetLRBChallenge();
    fn ProcessLRB();
}
impl ::windows::core::RuntimeName for IWMLicenseRevocationAgent {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMLicenseRevocationAgent";
}
impl IWMLicenseRevocationAgentVtbl {
    pub const fn new<Impl: IWMLicenseRevocationAgentImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMLicenseRevocationAgentVtbl {
        unsafe extern "system" fn GetLRBChallenge<Impl: IWMLicenseRevocationAgentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmachineid: *const u8, dwmachineidlength: u32, pchallenge: *const u8, dwchallengelength: u32, pchallengeoutput: *mut u8, pdwchallengeoutputlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLRBChallenge(pmachineid, dwmachineidlength, pchallenge, dwchallengelength, ::core::mem::transmute_copy(&pchallengeoutput), ::core::mem::transmute_copy(&pdwchallengeoutputlength)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessLRB<Impl: IWMLicenseRevocationAgentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psignedlrb: *const u8, dwsignedlrblength: u32, psignedack: *mut u8, pdwsignedacklength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProcessLRB(psignedlrb, dwsignedlrblength, ::core::mem::transmute_copy(&psignedack), ::core::mem::transmute_copy(&pdwsignedacklength)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMLicenseRevocationAgent>, base.5, GetLRBChallenge::<Impl, OFFSET>, ProcessLRB::<Impl, OFFSET>)
    }
}
pub trait IWMMediaPropsImpl: Sized {
    fn GetType();
    fn GetMediaType();
    fn SetMediaType();
}
impl ::windows::core::RuntimeName for IWMMediaProps {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMMediaProps";
}
impl IWMMediaPropsVtbl {
    pub const fn new<Impl: IWMMediaPropsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMMediaPropsVtbl {
        unsafe extern "system" fn GetType<Impl: IWMMediaPropsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidtype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetType(::core::mem::transmute_copy(&pguidtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMediaType<Impl: IWMMediaPropsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptype: *mut WM_MEDIA_TYPE, pcbtype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMediaType(::core::mem::transmute_copy(&ptype), pcbtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMediaType<Impl: IWMMediaPropsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptype: *const WM_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetMediaType(&*(&ptype as *const <WM_MEDIA_TYPE as ::windows::core::Abi>::Abi as *const <WM_MEDIA_TYPE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMMediaProps>, base.5, GetType::<Impl, OFFSET>, GetMediaType::<Impl, OFFSET>, SetMediaType::<Impl, OFFSET>)
    }
}
pub trait IWMMetadataEditorImpl: Sized {
    fn Open();
    fn Close();
    fn Flush();
}
impl ::windows::core::RuntimeName for IWMMetadataEditor {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMMetadataEditor";
}
impl IWMMetadataEditorVtbl {
    pub const fn new<Impl: IWMMetadataEditorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMMetadataEditorVtbl {
        unsafe extern "system" fn Open<Impl: IWMMetadataEditorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Open(&*(&pwszfilename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IWMMetadataEditorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Close() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Flush<Impl: IWMMetadataEditorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Flush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMMetadataEditor>, base.5, Open::<Impl, OFFSET>, Close::<Impl, OFFSET>, Flush::<Impl, OFFSET>)
    }
}
pub trait IWMMetadataEditor2Impl: Sized + IWMMetadataEditorImpl {
    fn OpenEx();
}
impl ::windows::core::RuntimeName for IWMMetadataEditor2 {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMMetadataEditor2";
}
impl IWMMetadataEditor2Vtbl {
    pub const fn new<Impl: IWMMetadataEditor2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMMetadataEditor2Vtbl {
        unsafe extern "system" fn OpenEx<Impl: IWMMetadataEditor2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszfilename: super::super::Foundation::PWSTR, dwdesiredaccess: u32, dwsharemode: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenEx(&*(&pwszfilename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dwdesiredaccess, dwsharemode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMMetadataEditor2>, base.5, OpenEx::<Impl, OFFSET>)
    }
}
pub trait IWMMutualExclusionImpl: Sized + IWMStreamListImpl {
    fn GetType();
    fn SetType();
}
impl ::windows::core::RuntimeName for IWMMutualExclusion {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMMutualExclusion";
}
impl IWMMutualExclusionVtbl {
    pub const fn new<Impl: IWMMutualExclusionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMMutualExclusionVtbl {
        unsafe extern "system" fn GetType<Impl: IWMMutualExclusionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidtype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetType(::core::mem::transmute_copy(&pguidtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Impl: IWMMutualExclusionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetType(&*(&guidtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMMutualExclusion>, base.5, GetType::<Impl, OFFSET>, SetType::<Impl, OFFSET>)
    }
}
pub trait IWMMutualExclusion2Impl: Sized + IWMMutualExclusionImpl + IWMStreamListImpl {
    fn GetName();
    fn SetName();
    fn GetRecordCount();
    fn AddRecord();
    fn RemoveRecord();
    fn GetRecordName();
    fn SetRecordName();
    fn GetStreamsForRecord();
    fn AddStreamForRecord();
    fn RemoveStreamForRecord();
}
impl ::windows::core::RuntimeName for IWMMutualExclusion2 {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMMutualExclusion2";
}
impl IWMMutualExclusion2Vtbl {
    pub const fn new<Impl: IWMMutualExclusion2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMMutualExclusion2Vtbl {
        unsafe extern "system" fn GetName<Impl: IWMMutualExclusion2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetName(::core::mem::transmute_copy(&pwszname), pcchname) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IWMMutualExclusion2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetName(&*(&pwszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecordCount<Impl: IWMMutualExclusion2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwrecordcount: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRecordCount(::core::mem::transmute_copy(&pwrecordcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddRecord<Impl: IWMMutualExclusion2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddRecord() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRecord<Impl: IWMMutualExclusion2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wrecordnumber: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveRecord(wrecordnumber) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecordName<Impl: IWMMutualExclusion2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wrecordnumber: u16, pwszrecordname: super::super::Foundation::PWSTR, pcchrecordname: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRecordName(wrecordnumber, ::core::mem::transmute_copy(&pwszrecordname), pcchrecordname) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRecordName<Impl: IWMMutualExclusion2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wrecordnumber: u16, pwszrecordname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetRecordName(wrecordnumber, &*(&pwszrecordname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStreamsForRecord<Impl: IWMMutualExclusion2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wrecordnumber: u16, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStreamsForRecord(wrecordnumber, ::core::mem::transmute_copy(&pwstreamnumarray), pcstreams) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddStreamForRecord<Impl: IWMMutualExclusion2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wrecordnumber: u16, wstreamnumber: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddStreamForRecord(wrecordnumber, wstreamnumber) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStreamForRecord<Impl: IWMMutualExclusion2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wrecordnumber: u16, wstreamnumber: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveStreamForRecord(wrecordnumber, wstreamnumber) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMMutualExclusion2>, base.5, GetName::<Impl, OFFSET>, SetName::<Impl, OFFSET>, GetRecordCount::<Impl, OFFSET>, AddRecord::<Impl, OFFSET>, RemoveRecord::<Impl, OFFSET>, GetRecordName::<Impl, OFFSET>, SetRecordName::<Impl, OFFSET>, GetStreamsForRecord::<Impl, OFFSET>, AddStreamForRecord::<Impl, OFFSET>, RemoveStreamForRecord::<Impl, OFFSET>)
    }
}
pub trait IWMOutputMediaPropsImpl: Sized + IWMMediaPropsImpl {
    fn GetStreamGroupName();
    fn GetConnectionName();
}
impl ::windows::core::RuntimeName for IWMOutputMediaProps {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMOutputMediaProps";
}
impl IWMOutputMediaPropsVtbl {
    pub const fn new<Impl: IWMOutputMediaPropsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMOutputMediaPropsVtbl {
        unsafe extern "system" fn GetStreamGroupName<Impl: IWMOutputMediaPropsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStreamGroupName(::core::mem::transmute_copy(&pwszname), pcchname) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectionName<Impl: IWMOutputMediaPropsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetConnectionName(::core::mem::transmute_copy(&pwszname), pcchname) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMOutputMediaProps>, base.5, GetStreamGroupName::<Impl, OFFSET>, GetConnectionName::<Impl, OFFSET>)
    }
}
pub trait IWMPacketSizeImpl: Sized {
    fn GetMaxPacketSize();
    fn SetMaxPacketSize();
}
impl ::windows::core::RuntimeName for IWMPacketSize {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMPacketSize";
}
impl IWMPacketSizeVtbl {
    pub const fn new<Impl: IWMPacketSizeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMPacketSizeVtbl {
        unsafe extern "system" fn GetMaxPacketSize<Impl: IWMPacketSizeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwmaxpacketsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMaxPacketSize(::core::mem::transmute_copy(&pdwmaxpacketsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxPacketSize<Impl: IWMPacketSizeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwmaxpacketsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetMaxPacketSize(dwmaxpacketsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMPacketSize>, base.5, GetMaxPacketSize::<Impl, OFFSET>, SetMaxPacketSize::<Impl, OFFSET>)
    }
}
pub trait IWMPacketSize2Impl: Sized + IWMPacketSizeImpl {
    fn GetMinPacketSize();
    fn SetMinPacketSize();
}
impl ::windows::core::RuntimeName for IWMPacketSize2 {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMPacketSize2";
}
impl IWMPacketSize2Vtbl {
    pub const fn new<Impl: IWMPacketSize2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMPacketSize2Vtbl {
        unsafe extern "system" fn GetMinPacketSize<Impl: IWMPacketSize2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwminpacketsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMinPacketSize(::core::mem::transmute_copy(&pdwminpacketsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinPacketSize<Impl: IWMPacketSize2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwminpacketsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetMinPacketSize(dwminpacketsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMPacketSize2>, base.5, GetMinPacketSize::<Impl, OFFSET>, SetMinPacketSize::<Impl, OFFSET>)
    }
}
pub trait IWMPlayerHookImpl: Sized {
    fn PreDecode();
}
impl ::windows::core::RuntimeName for IWMPlayerHook {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMPlayerHook";
}
impl IWMPlayerHookVtbl {
    pub const fn new<Impl: IWMPlayerHookImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMPlayerHookVtbl {
        unsafe extern "system" fn PreDecode<Impl: IWMPlayerHookImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PreDecode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMPlayerHook>, base.5, PreDecode::<Impl, OFFSET>)
    }
}
pub trait IWMPlayerTimestampHookImpl: Sized {
    fn MapTimestamp();
}
impl ::windows::core::RuntimeName for IWMPlayerTimestampHook {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMPlayerTimestampHook";
}
impl IWMPlayerTimestampHookVtbl {
    pub const fn new<Impl: IWMPlayerTimestampHookImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMPlayerTimestampHookVtbl {
        unsafe extern "system" fn MapTimestamp<Impl: IWMPlayerTimestampHookImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rtin: i64, prtout: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MapTimestamp(rtin, ::core::mem::transmute_copy(&prtout)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMPlayerTimestampHook>, base.5, MapTimestamp::<Impl, OFFSET>)
    }
}
pub trait IWMProfileImpl: Sized {
    fn GetVersion();
    fn GetName();
    fn SetName();
    fn GetDescription();
    fn SetDescription();
    fn GetStreamCount();
    fn GetStream();
    fn GetStreamByNumber();
    fn RemoveStream();
    fn RemoveStreamByNumber();
    fn AddStream();
    fn ReconfigStream();
    fn CreateNewStream();
    fn GetMutualExclusionCount();
    fn GetMutualExclusion();
    fn RemoveMutualExclusion();
    fn AddMutualExclusion();
    fn CreateNewMutualExclusion();
}
impl ::windows::core::RuntimeName for IWMProfile {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMProfile";
}
impl IWMProfileVtbl {
    pub const fn new<Impl: IWMProfileImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMProfileVtbl {
        unsafe extern "system" fn GetVersion<Impl: IWMProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwversion: *mut WMT_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVersion(::core::mem::transmute_copy(&pdwversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetName<Impl: IWMProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetName(::core::mem::transmute_copy(&pwszname), pcchname) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IWMProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetName(&*(&pwszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescription<Impl: IWMProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszdescription: super::super::Foundation::PWSTR, pcchdescription: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDescription(::core::mem::transmute_copy(&pwszdescription), pcchdescription) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IWMProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszdescription: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDescription(&*(&pwszdescription as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStreamCount<Impl: IWMProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcstreams: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStreamCount(::core::mem::transmute_copy(&pcstreams)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStream<Impl: IWMProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, ppconfig: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStream(dwstreamindex, ::core::mem::transmute_copy(&ppconfig)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStreamByNumber<Impl: IWMProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, ppconfig: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStreamByNumber(wstreamnum, ::core::mem::transmute_copy(&ppconfig)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStream<Impl: IWMProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pconfig: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveStream(&*(&pconfig as *const <IWMStreamConfig as ::windows::core::Abi>::Abi as *const <IWMStreamConfig as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStreamByNumber<Impl: IWMProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveStreamByNumber(wstreamnum) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddStream<Impl: IWMProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pconfig: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddStream(&*(&pconfig as *const <IWMStreamConfig as ::windows::core::Abi>::Abi as *const <IWMStreamConfig as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReconfigStream<Impl: IWMProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pconfig: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReconfigStream(&*(&pconfig as *const <IWMStreamConfig as ::windows::core::Abi>::Abi as *const <IWMStreamConfig as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateNewStream<Impl: IWMProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidstreamtype: *const ::windows::core::GUID, ppconfig: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateNewStream(&*(&guidstreamtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppconfig)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMutualExclusionCount<Impl: IWMProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcme: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMutualExclusionCount(::core::mem::transmute_copy(&pcme)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMutualExclusion<Impl: IWMProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwmeindex: u32, ppme: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMutualExclusion(dwmeindex, ::core::mem::transmute_copy(&ppme)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMutualExclusion<Impl: IWMProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pme: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveMutualExclusion(&*(&pme as *const <IWMMutualExclusion as ::windows::core::Abi>::Abi as *const <IWMMutualExclusion as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddMutualExclusion<Impl: IWMProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pme: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddMutualExclusion(&*(&pme as *const <IWMMutualExclusion as ::windows::core::Abi>::Abi as *const <IWMMutualExclusion as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateNewMutualExclusion<Impl: IWMProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppme: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateNewMutualExclusion(::core::mem::transmute_copy(&ppme)) {
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
            ::windows::core::GetRuntimeClassName::<IWMProfile>,
            base.5,
            GetVersion::<Impl, OFFSET>,
            GetName::<Impl, OFFSET>,
            SetName::<Impl, OFFSET>,
            GetDescription::<Impl, OFFSET>,
            SetDescription::<Impl, OFFSET>,
            GetStreamCount::<Impl, OFFSET>,
            GetStream::<Impl, OFFSET>,
            GetStreamByNumber::<Impl, OFFSET>,
            RemoveStream::<Impl, OFFSET>,
            RemoveStreamByNumber::<Impl, OFFSET>,
            AddStream::<Impl, OFFSET>,
            ReconfigStream::<Impl, OFFSET>,
            CreateNewStream::<Impl, OFFSET>,
            GetMutualExclusionCount::<Impl, OFFSET>,
            GetMutualExclusion::<Impl, OFFSET>,
            RemoveMutualExclusion::<Impl, OFFSET>,
            AddMutualExclusion::<Impl, OFFSET>,
            CreateNewMutualExclusion::<Impl, OFFSET>,
        )
    }
}
pub trait IWMProfile2Impl: Sized + IWMProfileImpl {
    fn GetProfileID();
}
impl ::windows::core::RuntimeName for IWMProfile2 {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMProfile2";
}
impl IWMProfile2Vtbl {
    pub const fn new<Impl: IWMProfile2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMProfile2Vtbl {
        unsafe extern "system" fn GetProfileID<Impl: IWMProfile2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetProfileID(::core::mem::transmute_copy(&pguidid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMProfile2>, base.5, GetProfileID::<Impl, OFFSET>)
    }
}
pub trait IWMProfile3Impl: Sized + IWMProfile2Impl + IWMProfileImpl {
    fn GetStorageFormat();
    fn SetStorageFormat();
    fn GetBandwidthSharingCount();
    fn GetBandwidthSharing();
    fn RemoveBandwidthSharing();
    fn AddBandwidthSharing();
    fn CreateNewBandwidthSharing();
    fn GetStreamPrioritization();
    fn SetStreamPrioritization();
    fn RemoveStreamPrioritization();
    fn CreateNewStreamPrioritization();
    fn GetExpectedPacketCount();
}
impl ::windows::core::RuntimeName for IWMProfile3 {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMProfile3";
}
impl IWMProfile3Vtbl {
    pub const fn new<Impl: IWMProfile3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMProfile3Vtbl {
        unsafe extern "system" fn GetStorageFormat<Impl: IWMProfile3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnstorageformat: *mut WMT_STORAGE_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStorageFormat(::core::mem::transmute_copy(&pnstorageformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStorageFormat<Impl: IWMProfile3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nstorageformat: WMT_STORAGE_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetStorageFormat(nstorageformat) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBandwidthSharingCount<Impl: IWMProfile3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcbs: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBandwidthSharingCount(::core::mem::transmute_copy(&pcbs)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBandwidthSharing<Impl: IWMProfile3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwbsindex: u32, ppbs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBandwidthSharing(dwbsindex, ::core::mem::transmute_copy(&ppbs)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBandwidthSharing<Impl: IWMProfile3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbs: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveBandwidthSharing(&*(&pbs as *const <IWMBandwidthSharing as ::windows::core::Abi>::Abi as *const <IWMBandwidthSharing as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddBandwidthSharing<Impl: IWMProfile3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbs: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddBandwidthSharing(&*(&pbs as *const <IWMBandwidthSharing as ::windows::core::Abi>::Abi as *const <IWMBandwidthSharing as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateNewBandwidthSharing<Impl: IWMProfile3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppbs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateNewBandwidthSharing(::core::mem::transmute_copy(&ppbs)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStreamPrioritization<Impl: IWMProfile3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStreamPrioritization(::core::mem::transmute_copy(&ppsp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStreamPrioritization<Impl: IWMProfile3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psp: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetStreamPrioritization(&*(&psp as *const <IWMStreamPrioritization as ::windows::core::Abi>::Abi as *const <IWMStreamPrioritization as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStreamPrioritization<Impl: IWMProfile3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveStreamPrioritization() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateNewStreamPrioritization<Impl: IWMProfile3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateNewStreamPrioritization(::core::mem::transmute_copy(&ppsp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExpectedPacketCount<Impl: IWMProfile3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, msduration: u64, pcpackets: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetExpectedPacketCount(msduration, ::core::mem::transmute_copy(&pcpackets)) {
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
            ::windows::core::GetRuntimeClassName::<IWMProfile3>,
            base.5,
            GetStorageFormat::<Impl, OFFSET>,
            SetStorageFormat::<Impl, OFFSET>,
            GetBandwidthSharingCount::<Impl, OFFSET>,
            GetBandwidthSharing::<Impl, OFFSET>,
            RemoveBandwidthSharing::<Impl, OFFSET>,
            AddBandwidthSharing::<Impl, OFFSET>,
            CreateNewBandwidthSharing::<Impl, OFFSET>,
            GetStreamPrioritization::<Impl, OFFSET>,
            SetStreamPrioritization::<Impl, OFFSET>,
            RemoveStreamPrioritization::<Impl, OFFSET>,
            CreateNewStreamPrioritization::<Impl, OFFSET>,
            GetExpectedPacketCount::<Impl, OFFSET>,
        )
    }
}
pub trait IWMProfileManagerImpl: Sized {
    fn CreateEmptyProfile();
    fn LoadProfileByID();
    fn LoadProfileByData();
    fn SaveProfile();
    fn GetSystemProfileCount();
    fn LoadSystemProfile();
}
impl ::windows::core::RuntimeName for IWMProfileManager {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMProfileManager";
}
impl IWMProfileManagerVtbl {
    pub const fn new<Impl: IWMProfileManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMProfileManagerVtbl {
        unsafe extern "system" fn CreateEmptyProfile<Impl: IWMProfileManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwversion: WMT_VERSION, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateEmptyProfile(dwversion, ::core::mem::transmute_copy(&ppprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadProfileByID<Impl: IWMProfileManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidprofile: *const ::windows::core::GUID, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LoadProfileByID(&*(&guidprofile as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadProfileByData<Impl: IWMProfileManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszprofile: super::super::Foundation::PWSTR, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LoadProfileByData(&*(&pwszprofile as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveProfile<Impl: IWMProfileManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, piwmprofile: ::windows::core::RawPtr, pwszprofile: super::super::Foundation::PWSTR, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SaveProfile(&*(&piwmprofile as *const <IWMProfile as ::windows::core::Abi>::Abi as *const <IWMProfile as ::windows::core::DefaultType>::DefaultType), &*(&pwszprofile as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), pdwlength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSystemProfileCount<Impl: IWMProfileManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcprofiles: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSystemProfileCount(::core::mem::transmute_copy(&pcprofiles)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadSystemProfile<Impl: IWMProfileManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwprofileindex: u32, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LoadSystemProfile(dwprofileindex, ::core::mem::transmute_copy(&ppprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMProfileManager>, base.5, CreateEmptyProfile::<Impl, OFFSET>, LoadProfileByID::<Impl, OFFSET>, LoadProfileByData::<Impl, OFFSET>, SaveProfile::<Impl, OFFSET>, GetSystemProfileCount::<Impl, OFFSET>, LoadSystemProfile::<Impl, OFFSET>)
    }
}
pub trait IWMProfileManager2Impl: Sized + IWMProfileManagerImpl {
    fn GetSystemProfileVersion();
    fn SetSystemProfileVersion();
}
impl ::windows::core::RuntimeName for IWMProfileManager2 {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMProfileManager2";
}
impl IWMProfileManager2Vtbl {
    pub const fn new<Impl: IWMProfileManager2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMProfileManager2Vtbl {
        unsafe extern "system" fn GetSystemProfileVersion<Impl: IWMProfileManager2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwversion: *mut WMT_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSystemProfileVersion(pdwversion) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSystemProfileVersion<Impl: IWMProfileManager2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwversion: WMT_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSystemProfileVersion(dwversion) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMProfileManager2>, base.5, GetSystemProfileVersion::<Impl, OFFSET>, SetSystemProfileVersion::<Impl, OFFSET>)
    }
}
pub trait IWMProfileManagerLanguageImpl: Sized {
    fn GetUserLanguageID();
    fn SetUserLanguageID();
}
impl ::windows::core::RuntimeName for IWMProfileManagerLanguage {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMProfileManagerLanguage";
}
impl IWMProfileManagerLanguageVtbl {
    pub const fn new<Impl: IWMProfileManagerLanguageImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMProfileManagerLanguageVtbl {
        unsafe extern "system" fn GetUserLanguageID<Impl: IWMProfileManagerLanguageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wlangid: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetUserLanguageID(wlangid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUserLanguageID<Impl: IWMProfileManagerLanguageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wlangid: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetUserLanguageID(wlangid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMProfileManagerLanguage>, base.5, GetUserLanguageID::<Impl, OFFSET>, SetUserLanguageID::<Impl, OFFSET>)
    }
}
pub trait IWMPropertyVaultImpl: Sized {
    fn GetPropertyCount();
    fn GetPropertyByName();
    fn SetProperty();
    fn GetPropertyByIndex();
    fn CopyPropertiesFrom();
    fn Clear();
}
impl ::windows::core::RuntimeName for IWMPropertyVault {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMPropertyVault";
}
impl IWMPropertyVaultVtbl {
    pub const fn new<Impl: IWMPropertyVaultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMPropertyVaultVtbl {
        unsafe extern "system" fn GetPropertyCount<Impl: IWMPropertyVaultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwcount: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPropertyCount(pdwcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyByName<Impl: IWMPropertyVaultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPropertyByName(&*(&pszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), pdwsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Impl: IWMPropertyVaultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, ptype: WMT_ATTR_DATATYPE, pvalue: *const u8, dwsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetProperty(&*(&pszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ptype, pvalue, dwsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyByIndex<Impl: IWMPropertyVaultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwindex: u32, pszname: super::super::Foundation::PWSTR, pdwnamelen: *mut u32, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPropertyByIndex(dwindex, ::core::mem::transmute_copy(&pszname), pdwnamelen, ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), pdwsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyPropertiesFrom<Impl: IWMPropertyVaultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, piwmpropertyvault: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CopyPropertiesFrom(&*(&piwmpropertyvault as *const <IWMPropertyVault as ::windows::core::Abi>::Abi as *const <IWMPropertyVault as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Impl: IWMPropertyVaultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMPropertyVault>, base.5, GetPropertyCount::<Impl, OFFSET>, GetPropertyByName::<Impl, OFFSET>, SetProperty::<Impl, OFFSET>, GetPropertyByIndex::<Impl, OFFSET>, CopyPropertiesFrom::<Impl, OFFSET>, Clear::<Impl, OFFSET>)
    }
}
pub trait IWMProximityDetectionImpl: Sized {
    fn StartDetection();
}
impl ::windows::core::RuntimeName for IWMProximityDetection {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMProximityDetection";
}
impl IWMProximityDetectionVtbl {
    pub const fn new<Impl: IWMProximityDetectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMProximityDetectionVtbl {
        unsafe extern "system" fn StartDetection<Impl: IWMProximityDetectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbregistrationmsg: *const u8, cbregistrationmsg: u32, pblocaladdress: *const u8, cblocaladdress: u32, dwextraportsallowed: u32, ppregistrationresponsemsg: *mut ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartDetection(pbregistrationmsg, cbregistrationmsg, pblocaladdress, cblocaladdress, dwextraportsallowed, ::core::mem::transmute_copy(&ppregistrationresponsemsg), &*(&pcallback as *const <IWMStatusCallback as ::windows::core::Abi>::Abi as *const <IWMStatusCallback as ::windows::core::DefaultType>::DefaultType), &*(&pvcontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMProximityDetection>, base.5, StartDetection::<Impl, OFFSET>)
    }
}
pub trait IWMReaderImpl: Sized {
    fn Open();
    fn Close();
    fn GetOutputCount();
    fn GetOutputProps();
    fn SetOutputProps();
    fn GetOutputFormatCount();
    fn GetOutputFormat();
    fn Start();
    fn Stop();
    fn Pause();
    fn Resume();
}
impl ::windows::core::RuntimeName for IWMReader {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMReader";
}
impl IWMReaderVtbl {
    pub const fn new<Impl: IWMReaderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMReaderVtbl {
        unsafe extern "system" fn Open<Impl: IWMReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Open(&*(&pwszurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pcallback as *const <IWMReaderCallback as ::windows::core::Abi>::Abi as *const <IWMReaderCallback as ::windows::core::DefaultType>::DefaultType), &*(&pvcontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IWMReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Close() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputCount<Impl: IWMReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcoutputs: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOutputCount(::core::mem::transmute_copy(&pcoutputs)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputProps<Impl: IWMReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, ppoutput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOutputProps(dwoutputnum, ::core::mem::transmute_copy(&ppoutput)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutputProps<Impl: IWMReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, poutput: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetOutputProps(dwoutputnum, &*(&poutput as *const <IWMOutputMediaProps as ::windows::core::Abi>::Abi as *const <IWMOutputMediaProps as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputFormatCount<Impl: IWMReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnumber: u32, pcformats: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOutputFormatCount(dwoutputnumber, ::core::mem::transmute_copy(&pcformats)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputFormat<Impl: IWMReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnumber: u32, dwformatnumber: u32, ppprops: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOutputFormat(dwoutputnumber, dwformatnumber, ::core::mem::transmute_copy(&ppprops)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Impl: IWMReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cnsstart: u64, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Start(cnsstart, cnsduration, frate, &*(&pvcontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stop<Impl: IWMReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Stop() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pause<Impl: IWMReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Resume<Impl: IWMReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Resume() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMReader>, base.5, Open::<Impl, OFFSET>, Close::<Impl, OFFSET>, GetOutputCount::<Impl, OFFSET>, GetOutputProps::<Impl, OFFSET>, SetOutputProps::<Impl, OFFSET>, GetOutputFormatCount::<Impl, OFFSET>, GetOutputFormat::<Impl, OFFSET>, Start::<Impl, OFFSET>, Stop::<Impl, OFFSET>, Pause::<Impl, OFFSET>, Resume::<Impl, OFFSET>)
    }
}
pub trait IWMReaderAcceleratorImpl: Sized {
    fn GetCodecInterface();
    fn Notify();
}
impl ::windows::core::RuntimeName for IWMReaderAccelerator {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMReaderAccelerator";
}
impl IWMReaderAcceleratorVtbl {
    pub const fn new<Impl: IWMReaderAcceleratorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMReaderAcceleratorVtbl {
        unsafe extern "system" fn GetCodecInterface<Impl: IWMReaderAcceleratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, riid: *const ::windows::core::GUID, ppvcodecinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCodecInterface(dwoutputnum, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvcodecinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Notify<Impl: IWMReaderAcceleratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, psubtype: *const WM_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Notify(dwoutputnum, &*(&psubtype as *const <WM_MEDIA_TYPE as ::windows::core::Abi>::Abi as *const <WM_MEDIA_TYPE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMReaderAccelerator>, base.5, GetCodecInterface::<Impl, OFFSET>, Notify::<Impl, OFFSET>)
    }
}
pub trait IWMReaderAdvancedImpl: Sized {
    fn SetUserProvidedClock();
    fn GetUserProvidedClock();
    fn DeliverTime();
    fn SetManualStreamSelection();
    fn GetManualStreamSelection();
    fn SetStreamsSelected();
    fn GetStreamSelected();
    fn SetReceiveSelectionCallbacks();
    fn GetReceiveSelectionCallbacks();
    fn SetReceiveStreamSamples();
    fn GetReceiveStreamSamples();
    fn SetAllocateForOutput();
    fn GetAllocateForOutput();
    fn SetAllocateForStream();
    fn GetAllocateForStream();
    fn GetStatistics();
    fn SetClientInfo();
    fn GetMaxOutputSampleSize();
    fn GetMaxStreamSampleSize();
    fn NotifyLateDelivery();
}
impl ::windows::core::RuntimeName for IWMReaderAdvanced {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMReaderAdvanced";
}
impl IWMReaderAdvancedVtbl {
    pub const fn new<Impl: IWMReaderAdvancedImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMReaderAdvancedVtbl {
        unsafe extern "system" fn SetUserProvidedClock<Impl: IWMReaderAdvancedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fuserclock: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetUserProvidedClock(&*(&fuserclock as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUserProvidedClock<Impl: IWMReaderAdvancedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfuserclock: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetUserProvidedClock(::core::mem::transmute_copy(&pfuserclock)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeliverTime<Impl: IWMReaderAdvancedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cnstime: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeliverTime(cnstime) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetManualStreamSelection<Impl: IWMReaderAdvancedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fselection: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetManualStreamSelection(&*(&fselection as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetManualStreamSelection<Impl: IWMReaderAdvancedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfselection: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetManualStreamSelection(::core::mem::transmute_copy(&pfselection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStreamsSelected<Impl: IWMReaderAdvancedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetStreamsSelected(cstreamcount, pwstreamnumbers, pselections) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStreamSelected<Impl: IWMReaderAdvancedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pselection: *mut WMT_STREAM_SELECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStreamSelected(wstreamnum, ::core::mem::transmute_copy(&pselection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReceiveSelectionCallbacks<Impl: IWMReaderAdvancedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fgetcallbacks: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetReceiveSelectionCallbacks(&*(&fgetcallbacks as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReceiveSelectionCallbacks<Impl: IWMReaderAdvancedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfgetcallbacks: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetReceiveSelectionCallbacks(::core::mem::transmute_copy(&pfgetcallbacks)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReceiveStreamSamples<Impl: IWMReaderAdvancedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, freceivestreamsamples: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetReceiveStreamSamples(wstreamnum, &*(&freceivestreamsamples as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReceiveStreamSamples<Impl: IWMReaderAdvancedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pfreceivestreamsamples: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetReceiveStreamSamples(wstreamnum, ::core::mem::transmute_copy(&pfreceivestreamsamples)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllocateForOutput<Impl: IWMReaderAdvancedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, fallocate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAllocateForOutput(dwoutputnum, &*(&fallocate as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllocateForOutput<Impl: IWMReaderAdvancedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAllocateForOutput(dwoutputnum, ::core::mem::transmute_copy(&pfallocate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllocateForStream<Impl: IWMReaderAdvancedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, fallocate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAllocateForStream(wstreamnum, &*(&fallocate as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllocateForStream<Impl: IWMReaderAdvancedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwsreamnum: u16, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAllocateForStream(dwsreamnum, ::core::mem::transmute_copy(&pfallocate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatistics<Impl: IWMReaderAdvancedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstatistics: *mut WM_READER_STATISTICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStatistics(&*(&pstatistics as *const <WM_READER_STATISTICS as ::windows::core::Abi>::Abi as *const <WM_READER_STATISTICS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientInfo<Impl: IWMReaderAdvancedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetClientInfo(&*(&pclientinfo as *const <WM_READER_CLIENTINFO as ::windows::core::Abi>::Abi as *const <WM_READER_CLIENTINFO as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxOutputSampleSize<Impl: IWMReaderAdvancedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutput: u32, pcbmax: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMaxOutputSampleSize(dwoutput, ::core::mem::transmute_copy(&pcbmax)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxStreamSampleSize<Impl: IWMReaderAdvancedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstream: u16, pcbmax: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMaxStreamSampleSize(wstream, ::core::mem::transmute_copy(&pcbmax)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyLateDelivery<Impl: IWMReaderAdvancedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cnslateness: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NotifyLateDelivery(cnslateness) {
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
            ::windows::core::GetRuntimeClassName::<IWMReaderAdvanced>,
            base.5,
            SetUserProvidedClock::<Impl, OFFSET>,
            GetUserProvidedClock::<Impl, OFFSET>,
            DeliverTime::<Impl, OFFSET>,
            SetManualStreamSelection::<Impl, OFFSET>,
            GetManualStreamSelection::<Impl, OFFSET>,
            SetStreamsSelected::<Impl, OFFSET>,
            GetStreamSelected::<Impl, OFFSET>,
            SetReceiveSelectionCallbacks::<Impl, OFFSET>,
            GetReceiveSelectionCallbacks::<Impl, OFFSET>,
            SetReceiveStreamSamples::<Impl, OFFSET>,
            GetReceiveStreamSamples::<Impl, OFFSET>,
            SetAllocateForOutput::<Impl, OFFSET>,
            GetAllocateForOutput::<Impl, OFFSET>,
            SetAllocateForStream::<Impl, OFFSET>,
            GetAllocateForStream::<Impl, OFFSET>,
            GetStatistics::<Impl, OFFSET>,
            SetClientInfo::<Impl, OFFSET>,
            GetMaxOutputSampleSize::<Impl, OFFSET>,
            GetMaxStreamSampleSize::<Impl, OFFSET>,
            NotifyLateDelivery::<Impl, OFFSET>,
        )
    }
}
pub trait IWMReaderAdvanced2Impl: Sized + IWMReaderAdvancedImpl {
    fn SetPlayMode();
    fn GetPlayMode();
    fn GetBufferProgress();
    fn GetDownloadProgress();
    fn GetSaveAsProgress();
    fn SaveFileAs();
    fn GetProtocolName();
    fn StartAtMarker();
    fn GetOutputSetting();
    fn SetOutputSetting();
    fn Preroll();
    fn SetLogClientID();
    fn GetLogClientID();
    fn StopBuffering();
    fn OpenStream();
}
impl ::windows::core::RuntimeName for IWMReaderAdvanced2 {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMReaderAdvanced2";
}
impl IWMReaderAdvanced2Vtbl {
    pub const fn new<Impl: IWMReaderAdvanced2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMReaderAdvanced2Vtbl {
        unsafe extern "system" fn SetPlayMode<Impl: IWMReaderAdvanced2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mode: WMT_PLAY_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPlayMode(mode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPlayMode<Impl: IWMReaderAdvanced2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmode: *mut WMT_PLAY_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPlayMode(::core::mem::transmute_copy(&pmode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBufferProgress<Impl: IWMReaderAdvanced2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBufferProgress(::core::mem::transmute_copy(&pdwpercent), ::core::mem::transmute_copy(&pcnsbuffering)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDownloadProgress<Impl: IWMReaderAdvanced2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDownloadProgress(::core::mem::transmute_copy(&pdwpercent), ::core::mem::transmute_copy(&pqwbytesdownloaded), ::core::mem::transmute_copy(&pcnsdownload)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSaveAsProgress<Impl: IWMReaderAdvanced2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwpercent: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSaveAsProgress(::core::mem::transmute_copy(&pdwpercent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveFileAs<Impl: IWMReaderAdvanced2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SaveFileAs(&*(&pwszfilename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProtocolName<Impl: IWMReaderAdvanced2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, pcchprotocol: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetProtocolName(::core::mem::transmute_copy(&pwszprotocol), pcchprotocol) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartAtMarker<Impl: IWMReaderAdvanced2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartAtMarker(wmarkerindex, cnsduration, frate, &*(&pvcontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputSetting<Impl: IWMReaderAdvanced2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOutputSetting(dwoutputnum, &*(&pszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), pcblength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutputSetting<Impl: IWMReaderAdvanced2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetOutputSetting(dwoutputnum, &*(&pszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), r#type, pvalue, cblength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Preroll<Impl: IWMReaderAdvanced2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Preroll(cnsstart, cnsduration, frate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogClientID<Impl: IWMReaderAdvanced2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flogclientid: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetLogClientID(&*(&flogclientid as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLogClientID<Impl: IWMReaderAdvanced2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pflogclientid: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLogClientID(::core::mem::transmute_copy(&pflogclientid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopBuffering<Impl: IWMReaderAdvanced2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StopBuffering() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenStream<Impl: IWMReaderAdvanced2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenStream(
                &*(&pstream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                &*(&pcallback as *const <IWMReaderCallback as ::windows::core::Abi>::Abi as *const <IWMReaderCallback as ::windows::core::DefaultType>::DefaultType),
                &*(&pvcontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
            ) {
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
            ::windows::core::GetRuntimeClassName::<IWMReaderAdvanced2>,
            base.5,
            SetPlayMode::<Impl, OFFSET>,
            GetPlayMode::<Impl, OFFSET>,
            GetBufferProgress::<Impl, OFFSET>,
            GetDownloadProgress::<Impl, OFFSET>,
            GetSaveAsProgress::<Impl, OFFSET>,
            SaveFileAs::<Impl, OFFSET>,
            GetProtocolName::<Impl, OFFSET>,
            StartAtMarker::<Impl, OFFSET>,
            GetOutputSetting::<Impl, OFFSET>,
            SetOutputSetting::<Impl, OFFSET>,
            Preroll::<Impl, OFFSET>,
            SetLogClientID::<Impl, OFFSET>,
            GetLogClientID::<Impl, OFFSET>,
            StopBuffering::<Impl, OFFSET>,
            OpenStream::<Impl, OFFSET>,
        )
    }
}
pub trait IWMReaderAdvanced3Impl: Sized + IWMReaderAdvanced2Impl + IWMReaderAdvancedImpl {
    fn StopNetStreaming();
    fn StartAtPosition();
}
impl ::windows::core::RuntimeName for IWMReaderAdvanced3 {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMReaderAdvanced3";
}
impl IWMReaderAdvanced3Vtbl {
    pub const fn new<Impl: IWMReaderAdvanced3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMReaderAdvanced3Vtbl {
        unsafe extern "system" fn StopNetStreaming<Impl: IWMReaderAdvanced3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StopNetStreaming() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartAtPosition<Impl: IWMReaderAdvanced3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pvoffsetstart: *const ::core::ffi::c_void, pvduration: *const ::core::ffi::c_void, dwoffsetformat: WMT_OFFSET_FORMAT, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartAtPosition(
                wstreamnum,
                &*(&pvoffsetstart as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                &*(&pvduration as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                dwoffsetformat,
                frate,
                &*(&pvcontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMReaderAdvanced3>, base.5, StopNetStreaming::<Impl, OFFSET>, StartAtPosition::<Impl, OFFSET>)
    }
}
pub trait IWMReaderAdvanced4Impl: Sized + IWMReaderAdvanced3Impl + IWMReaderAdvanced2Impl + IWMReaderAdvancedImpl {
    fn GetLanguageCount();
    fn GetLanguage();
    fn GetMaxSpeedFactor();
    fn IsUsingFastCache();
    fn AddLogParam();
    fn SendLogParams();
    fn CanSaveFileAs();
    fn CancelSaveFileAs();
    fn GetURL();
}
impl ::windows::core::RuntimeName for IWMReaderAdvanced4 {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMReaderAdvanced4";
}
impl IWMReaderAdvanced4Vtbl {
    pub const fn new<Impl: IWMReaderAdvanced4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMReaderAdvanced4Vtbl {
        unsafe extern "system" fn GetLanguageCount<Impl: IWMReaderAdvanced4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pwlanguagecount: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLanguageCount(dwoutputnum, ::core::mem::transmute_copy(&pwlanguagecount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLanguage<Impl: IWMReaderAdvanced4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, wlanguage: u16, pwszlanguagestring: super::super::Foundation::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLanguage(dwoutputnum, wlanguage, ::core::mem::transmute_copy(&pwszlanguagestring), pcchlanguagestringlength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxSpeedFactor<Impl: IWMReaderAdvanced4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdblfactor: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMaxSpeedFactor(::core::mem::transmute_copy(&pdblfactor)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUsingFastCache<Impl: IWMReaderAdvanced4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfusingfastcache: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsUsingFastCache(::core::mem::transmute_copy(&pfusingfastcache)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddLogParam<Impl: IWMReaderAdvanced4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wsznamespace: super::super::Foundation::PWSTR, wszname: super::super::Foundation::PWSTR, wszvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddLogParam(
                &*(&wsznamespace as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wszvalue as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendLogParams<Impl: IWMReaderAdvanced4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SendLogParams() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanSaveFileAs<Impl: IWMReaderAdvanced4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfcansave: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanSaveFileAs(::core::mem::transmute_copy(&pfcansave)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelSaveFileAs<Impl: IWMReaderAdvanced4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CancelSaveFileAs() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetURL<Impl: IWMReaderAdvanced4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, pcchurl: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetURL(::core::mem::transmute_copy(&pwszurl), pcchurl) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMReaderAdvanced4>, base.5, GetLanguageCount::<Impl, OFFSET>, GetLanguage::<Impl, OFFSET>, GetMaxSpeedFactor::<Impl, OFFSET>, IsUsingFastCache::<Impl, OFFSET>, AddLogParam::<Impl, OFFSET>, SendLogParams::<Impl, OFFSET>, CanSaveFileAs::<Impl, OFFSET>, CancelSaveFileAs::<Impl, OFFSET>, GetURL::<Impl, OFFSET>)
    }
}
pub trait IWMReaderAdvanced5Impl: Sized + IWMReaderAdvanced4Impl + IWMReaderAdvanced3Impl + IWMReaderAdvanced2Impl + IWMReaderAdvancedImpl {
    fn SetPlayerHook();
}
impl ::windows::core::RuntimeName for IWMReaderAdvanced5 {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMReaderAdvanced5";
}
impl IWMReaderAdvanced5Vtbl {
    pub const fn new<Impl: IWMReaderAdvanced5Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMReaderAdvanced5Vtbl {
        unsafe extern "system" fn SetPlayerHook<Impl: IWMReaderAdvanced5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, phook: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPlayerHook(dwoutputnum, &*(&phook as *const <IWMPlayerHook as ::windows::core::Abi>::Abi as *const <IWMPlayerHook as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMReaderAdvanced5>, base.5, SetPlayerHook::<Impl, OFFSET>)
    }
}
pub trait IWMReaderAdvanced6Impl: Sized + IWMReaderAdvanced5Impl + IWMReaderAdvanced4Impl + IWMReaderAdvanced3Impl + IWMReaderAdvanced2Impl + IWMReaderAdvancedImpl {
    fn SetProtectStreamSamples();
}
impl ::windows::core::RuntimeName for IWMReaderAdvanced6 {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMReaderAdvanced6";
}
impl IWMReaderAdvanced6Vtbl {
    pub const fn new<Impl: IWMReaderAdvanced6Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMReaderAdvanced6Vtbl {
        unsafe extern "system" fn SetProtectStreamSamples<Impl: IWMReaderAdvanced6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbcertificate: *const u8, cbcertificate: u32, dwcertificatetype: u32, dwflags: u32, pbinitializationvector: *mut u8, pcbinitializationvector: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetProtectStreamSamples(pbcertificate, cbcertificate, dwcertificatetype, dwflags, ::core::mem::transmute_copy(&pbinitializationvector), pcbinitializationvector) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMReaderAdvanced6>, base.5, SetProtectStreamSamples::<Impl, OFFSET>)
    }
}
pub trait IWMReaderAllocatorExImpl: Sized {
    fn AllocateForStreamEx();
    fn AllocateForOutputEx();
}
impl ::windows::core::RuntimeName for IWMReaderAllocatorEx {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMReaderAllocatorEx";
}
impl IWMReaderAllocatorExVtbl {
    pub const fn new<Impl: IWMReaderAllocatorExImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMReaderAllocatorExVtbl {
        unsafe extern "system" fn AllocateForStreamEx<Impl: IWMReaderAllocatorExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut ::windows::core::RawPtr, dwflags: u32, cnssampletime: u64, cnssampleduration: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllocateForStreamEx(wstreamnum, cbbuffer, ::core::mem::transmute_copy(&ppbuffer), dwflags, cnssampletime, cnssampleduration, &*(&pvcontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllocateForOutputEx<Impl: IWMReaderAllocatorExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, cbbuffer: u32, ppbuffer: *mut ::windows::core::RawPtr, dwflags: u32, cnssampletime: u64, cnssampleduration: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllocateForOutputEx(dwoutputnum, cbbuffer, ::core::mem::transmute_copy(&ppbuffer), dwflags, cnssampletime, cnssampleduration, &*(&pvcontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMReaderAllocatorEx>, base.5, AllocateForStreamEx::<Impl, OFFSET>, AllocateForOutputEx::<Impl, OFFSET>)
    }
}
pub trait IWMReaderCallbackImpl: Sized + IWMStatusCallbackImpl {
    fn OnSample();
}
impl ::windows::core::RuntimeName for IWMReaderCallback {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMReaderCallback";
}
impl IWMReaderCallbackVtbl {
    pub const fn new<Impl: IWMReaderCallbackImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMReaderCallbackVtbl {
        unsafe extern "system" fn OnSample<Impl: IWMReaderCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnSample(dwoutputnum, cnssampletime, cnssampleduration, dwflags, &*(&psample as *const <INSSBuffer as ::windows::core::Abi>::Abi as *const <INSSBuffer as ::windows::core::DefaultType>::DefaultType), &*(&pvcontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMReaderCallback>, base.5, OnSample::<Impl, OFFSET>)
    }
}
pub trait IWMReaderCallbackAdvancedImpl: Sized {
    fn OnStreamSample();
    fn OnTime();
    fn OnStreamSelection();
    fn OnOutputPropsChanged();
    fn AllocateForStream();
    fn AllocateForOutput();
}
impl ::windows::core::RuntimeName for IWMReaderCallbackAdvanced {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMReaderCallbackAdvanced";
}
impl IWMReaderCallbackAdvancedVtbl {
    pub const fn new<Impl: IWMReaderCallbackAdvancedImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMReaderCallbackAdvancedVtbl {
        unsafe extern "system" fn OnStreamSample<Impl: IWMReaderCallbackAdvancedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnStreamSample(wstreamnum, cnssampletime, cnssampleduration, dwflags, &*(&psample as *const <INSSBuffer as ::windows::core::Abi>::Abi as *const <INSSBuffer as ::windows::core::DefaultType>::DefaultType), &*(&pvcontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnTime<Impl: IWMReaderCallbackAdvancedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cnscurrenttime: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnTime(cnscurrenttime, &*(&pvcontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnStreamSelection<Impl: IWMReaderCallbackAdvancedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamcount: u16, pstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnStreamSelection(wstreamcount, pstreamnumbers, pselections, &*(&pvcontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnOutputPropsChanged<Impl: IWMReaderCallbackAdvancedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pmediatype: *const WM_MEDIA_TYPE, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnOutputPropsChanged(dwoutputnum, &*(&pmediatype as *const <WM_MEDIA_TYPE as ::windows::core::Abi>::Abi as *const <WM_MEDIA_TYPE as ::windows::core::DefaultType>::DefaultType), &*(&pvcontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllocateForStream<Impl: IWMReaderCallbackAdvancedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllocateForStream(wstreamnum, cbbuffer, ::core::mem::transmute_copy(&ppbuffer), &*(&pvcontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllocateForOutput<Impl: IWMReaderCallbackAdvancedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, cbbuffer: u32, ppbuffer: *mut ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllocateForOutput(dwoutputnum, cbbuffer, ::core::mem::transmute_copy(&ppbuffer), &*(&pvcontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMReaderCallbackAdvanced>, base.5, OnStreamSample::<Impl, OFFSET>, OnTime::<Impl, OFFSET>, OnStreamSelection::<Impl, OFFSET>, OnOutputPropsChanged::<Impl, OFFSET>, AllocateForStream::<Impl, OFFSET>, AllocateForOutput::<Impl, OFFSET>)
    }
}
pub trait IWMReaderNetworkConfigImpl: Sized {
    fn GetBufferingTime();
    fn SetBufferingTime();
    fn GetUDPPortRanges();
    fn SetUDPPortRanges();
    fn GetProxySettings();
    fn SetProxySettings();
    fn GetProxyHostName();
    fn SetProxyHostName();
    fn GetProxyPort();
    fn SetProxyPort();
    fn GetProxyExceptionList();
    fn SetProxyExceptionList();
    fn GetProxyBypassForLocal();
    fn SetProxyBypassForLocal();
    fn GetForceRerunAutoProxyDetection();
    fn SetForceRerunAutoProxyDetection();
    fn GetEnableMulticast();
    fn SetEnableMulticast();
    fn GetEnableHTTP();
    fn SetEnableHTTP();
    fn GetEnableUDP();
    fn SetEnableUDP();
    fn GetEnableTCP();
    fn SetEnableTCP();
    fn ResetProtocolRollover();
    fn GetConnectionBandwidth();
    fn SetConnectionBandwidth();
    fn GetNumProtocolsSupported();
    fn GetSupportedProtocolName();
    fn AddLoggingUrl();
    fn GetLoggingUrl();
    fn GetLoggingUrlCount();
    fn ResetLoggingUrlList();
}
impl ::windows::core::RuntimeName for IWMReaderNetworkConfig {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMReaderNetworkConfig";
}
impl IWMReaderNetworkConfigVtbl {
    pub const fn new<Impl: IWMReaderNetworkConfigImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMReaderNetworkConfigVtbl {
        unsafe extern "system" fn GetBufferingTime<Impl: IWMReaderNetworkConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcnsbufferingtime: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBufferingTime(::core::mem::transmute_copy(&pcnsbufferingtime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferingTime<Impl: IWMReaderNetworkConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cnsbufferingtime: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetBufferingTime(cnsbufferingtime) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUDPPortRanges<Impl: IWMReaderNetworkConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prangearray: *mut WM_PORT_NUMBER_RANGE, pcranges: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetUDPPortRanges(::core::mem::transmute_copy(&prangearray), pcranges) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUDPPortRanges<Impl: IWMReaderNetworkConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prangearray: *const WM_PORT_NUMBER_RANGE, cranges: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetUDPPortRanges(&*(&prangearray as *const <WM_PORT_NUMBER_RANGE as ::windows::core::Abi>::Abi as *const <WM_PORT_NUMBER_RANGE as ::windows::core::DefaultType>::DefaultType), cranges) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProxySettings<Impl: IWMReaderNetworkConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, pproxysetting: *mut WMT_PROXY_SETTINGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetProxySettings(&*(&pwszprotocol as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pproxysetting)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProxySettings<Impl: IWMReaderNetworkConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, proxysetting: WMT_PROXY_SETTINGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetProxySettings(&*(&pwszprotocol as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), proxysetting) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProxyHostName<Impl: IWMReaderNetworkConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, pwszhostname: super::super::Foundation::PWSTR, pcchhostname: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetProxyHostName(&*(&pwszprotocol as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pwszhostname), pcchhostname) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProxyHostName<Impl: IWMReaderNetworkConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, pwszhostname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetProxyHostName(&*(&pwszprotocol as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pwszhostname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProxyPort<Impl: IWMReaderNetworkConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, pdwport: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetProxyPort(&*(&pwszprotocol as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwport)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProxyPort<Impl: IWMReaderNetworkConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, dwport: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetProxyPort(&*(&pwszprotocol as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dwport) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProxyExceptionList<Impl: IWMReaderNetworkConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, pwszexceptionlist: super::super::Foundation::PWSTR, pcchexceptionlist: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetProxyExceptionList(&*(&pwszprotocol as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pwszexceptionlist), pcchexceptionlist) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProxyExceptionList<Impl: IWMReaderNetworkConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, pwszexceptionlist: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetProxyExceptionList(&*(&pwszprotocol as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pwszexceptionlist as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProxyBypassForLocal<Impl: IWMReaderNetworkConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, pfbypassforlocal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetProxyBypassForLocal(&*(&pwszprotocol as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfbypassforlocal)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProxyBypassForLocal<Impl: IWMReaderNetworkConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, fbypassforlocal: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetProxyBypassForLocal(&*(&pwszprotocol as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&fbypassforlocal as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetForceRerunAutoProxyDetection<Impl: IWMReaderNetworkConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfforcererundetection: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForceRerunAutoProxyDetection(::core::mem::transmute_copy(&pfforcererundetection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForceRerunAutoProxyDetection<Impl: IWMReaderNetworkConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fforcererundetection: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetForceRerunAutoProxyDetection(&*(&fforcererundetection as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnableMulticast<Impl: IWMReaderNetworkConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfenablemulticast: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetEnableMulticast(::core::mem::transmute_copy(&pfenablemulticast)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableMulticast<Impl: IWMReaderNetworkConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fenablemulticast: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetEnableMulticast(&*(&fenablemulticast as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnableHTTP<Impl: IWMReaderNetworkConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfenablehttp: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetEnableHTTP(::core::mem::transmute_copy(&pfenablehttp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableHTTP<Impl: IWMReaderNetworkConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fenablehttp: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetEnableHTTP(&*(&fenablehttp as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnableUDP<Impl: IWMReaderNetworkConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfenableudp: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetEnableUDP(::core::mem::transmute_copy(&pfenableudp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableUDP<Impl: IWMReaderNetworkConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fenableudp: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetEnableUDP(&*(&fenableudp as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnableTCP<Impl: IWMReaderNetworkConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfenabletcp: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetEnableTCP(::core::mem::transmute_copy(&pfenabletcp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableTCP<Impl: IWMReaderNetworkConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fenabletcp: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetEnableTCP(&*(&fenabletcp as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResetProtocolRollover<Impl: IWMReaderNetworkConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ResetProtocolRollover() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectionBandwidth<Impl: IWMReaderNetworkConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwconnectionbandwidth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetConnectionBandwidth(::core::mem::transmute_copy(&pdwconnectionbandwidth)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConnectionBandwidth<Impl: IWMReaderNetworkConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwconnectionbandwidth: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetConnectionBandwidth(dwconnectionbandwidth) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumProtocolsSupported<Impl: IWMReaderNetworkConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcprotocols: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNumProtocolsSupported(::core::mem::transmute_copy(&pcprotocols)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedProtocolName<Impl: IWMReaderNetworkConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwprotocolnum: u32, pwszprotocolname: super::super::Foundation::PWSTR, pcchprotocolname: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSupportedProtocolName(dwprotocolnum, ::core::mem::transmute_copy(&pwszprotocolname), pcchprotocolname) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddLoggingUrl<Impl: IWMReaderNetworkConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddLoggingUrl(&*(&pwszurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLoggingUrl<Impl: IWMReaderNetworkConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwindex: u32, pwszurl: super::super::Foundation::PWSTR, pcchurl: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLoggingUrl(dwindex, ::core::mem::transmute_copy(&pwszurl), pcchurl) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLoggingUrlCount<Impl: IWMReaderNetworkConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwurlcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLoggingUrlCount(::core::mem::transmute_copy(&pdwurlcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResetLoggingUrlList<Impl: IWMReaderNetworkConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ResetLoggingUrlList() {
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
            ::windows::core::GetRuntimeClassName::<IWMReaderNetworkConfig>,
            base.5,
            GetBufferingTime::<Impl, OFFSET>,
            SetBufferingTime::<Impl, OFFSET>,
            GetUDPPortRanges::<Impl, OFFSET>,
            SetUDPPortRanges::<Impl, OFFSET>,
            GetProxySettings::<Impl, OFFSET>,
            SetProxySettings::<Impl, OFFSET>,
            GetProxyHostName::<Impl, OFFSET>,
            SetProxyHostName::<Impl, OFFSET>,
            GetProxyPort::<Impl, OFFSET>,
            SetProxyPort::<Impl, OFFSET>,
            GetProxyExceptionList::<Impl, OFFSET>,
            SetProxyExceptionList::<Impl, OFFSET>,
            GetProxyBypassForLocal::<Impl, OFFSET>,
            SetProxyBypassForLocal::<Impl, OFFSET>,
            GetForceRerunAutoProxyDetection::<Impl, OFFSET>,
            SetForceRerunAutoProxyDetection::<Impl, OFFSET>,
            GetEnableMulticast::<Impl, OFFSET>,
            SetEnableMulticast::<Impl, OFFSET>,
            GetEnableHTTP::<Impl, OFFSET>,
            SetEnableHTTP::<Impl, OFFSET>,
            GetEnableUDP::<Impl, OFFSET>,
            SetEnableUDP::<Impl, OFFSET>,
            GetEnableTCP::<Impl, OFFSET>,
            SetEnableTCP::<Impl, OFFSET>,
            ResetProtocolRollover::<Impl, OFFSET>,
            GetConnectionBandwidth::<Impl, OFFSET>,
            SetConnectionBandwidth::<Impl, OFFSET>,
            GetNumProtocolsSupported::<Impl, OFFSET>,
            GetSupportedProtocolName::<Impl, OFFSET>,
            AddLoggingUrl::<Impl, OFFSET>,
            GetLoggingUrl::<Impl, OFFSET>,
            GetLoggingUrlCount::<Impl, OFFSET>,
            ResetLoggingUrlList::<Impl, OFFSET>,
        )
    }
}
pub trait IWMReaderNetworkConfig2Impl: Sized + IWMReaderNetworkConfigImpl {
    fn GetEnableContentCaching();
    fn SetEnableContentCaching();
    fn GetEnableFastCache();
    fn SetEnableFastCache();
    fn GetAcceleratedStreamingDuration();
    fn SetAcceleratedStreamingDuration();
    fn GetAutoReconnectLimit();
    fn SetAutoReconnectLimit();
    fn GetEnableResends();
    fn SetEnableResends();
    fn GetEnableThinning();
    fn SetEnableThinning();
    fn GetMaxNetPacketSize();
}
impl ::windows::core::RuntimeName for IWMReaderNetworkConfig2 {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMReaderNetworkConfig2";
}
impl IWMReaderNetworkConfig2Vtbl {
    pub const fn new<Impl: IWMReaderNetworkConfig2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMReaderNetworkConfig2Vtbl {
        unsafe extern "system" fn GetEnableContentCaching<Impl: IWMReaderNetworkConfig2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfenablecontentcaching: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetEnableContentCaching(::core::mem::transmute_copy(&pfenablecontentcaching)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableContentCaching<Impl: IWMReaderNetworkConfig2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fenablecontentcaching: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetEnableContentCaching(&*(&fenablecontentcaching as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnableFastCache<Impl: IWMReaderNetworkConfig2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfenablefastcache: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetEnableFastCache(::core::mem::transmute_copy(&pfenablefastcache)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableFastCache<Impl: IWMReaderNetworkConfig2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fenablefastcache: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetEnableFastCache(&*(&fenablefastcache as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAcceleratedStreamingDuration<Impl: IWMReaderNetworkConfig2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcnsaccelduration: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAcceleratedStreamingDuration(::core::mem::transmute_copy(&pcnsaccelduration)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAcceleratedStreamingDuration<Impl: IWMReaderNetworkConfig2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cnsaccelduration: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAcceleratedStreamingDuration(cnsaccelduration) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAutoReconnectLimit<Impl: IWMReaderNetworkConfig2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwautoreconnectlimit: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAutoReconnectLimit(::core::mem::transmute_copy(&pdwautoreconnectlimit)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoReconnectLimit<Impl: IWMReaderNetworkConfig2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwautoreconnectlimit: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAutoReconnectLimit(dwautoreconnectlimit) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnableResends<Impl: IWMReaderNetworkConfig2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfenableresends: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetEnableResends(::core::mem::transmute_copy(&pfenableresends)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableResends<Impl: IWMReaderNetworkConfig2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fenableresends: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetEnableResends(&*(&fenableresends as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnableThinning<Impl: IWMReaderNetworkConfig2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfenablethinning: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetEnableThinning(::core::mem::transmute_copy(&pfenablethinning)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableThinning<Impl: IWMReaderNetworkConfig2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fenablethinning: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetEnableThinning(&*(&fenablethinning as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxNetPacketSize<Impl: IWMReaderNetworkConfig2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwmaxnetpacketsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMaxNetPacketSize(::core::mem::transmute_copy(&pdwmaxnetpacketsize)) {
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
            ::windows::core::GetRuntimeClassName::<IWMReaderNetworkConfig2>,
            base.5,
            GetEnableContentCaching::<Impl, OFFSET>,
            SetEnableContentCaching::<Impl, OFFSET>,
            GetEnableFastCache::<Impl, OFFSET>,
            SetEnableFastCache::<Impl, OFFSET>,
            GetAcceleratedStreamingDuration::<Impl, OFFSET>,
            SetAcceleratedStreamingDuration::<Impl, OFFSET>,
            GetAutoReconnectLimit::<Impl, OFFSET>,
            SetAutoReconnectLimit::<Impl, OFFSET>,
            GetEnableResends::<Impl, OFFSET>,
            SetEnableResends::<Impl, OFFSET>,
            GetEnableThinning::<Impl, OFFSET>,
            SetEnableThinning::<Impl, OFFSET>,
            GetMaxNetPacketSize::<Impl, OFFSET>,
        )
    }
}
pub trait IWMReaderPlaylistBurnImpl: Sized {
    fn InitPlaylistBurn();
    fn GetInitResults();
    fn Cancel();
    fn EndPlaylistBurn();
}
impl ::windows::core::RuntimeName for IWMReaderPlaylistBurn {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMReaderPlaylistBurn";
}
impl IWMReaderPlaylistBurnVtbl {
    pub const fn new<Impl: IWMReaderPlaylistBurnImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMReaderPlaylistBurnVtbl {
        unsafe extern "system" fn InitPlaylistBurn<Impl: IWMReaderPlaylistBurnImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cfiles: u32, ppwszfilenames: *const super::super::Foundation::PWSTR, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InitPlaylistBurn(
                cfiles,
                &*(&ppwszfilenames as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pcallback as *const <IWMStatusCallback as ::windows::core::Abi>::Abi as *const <IWMStatusCallback as ::windows::core::DefaultType>::DefaultType),
                &*(&pvcontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInitResults<Impl: IWMReaderPlaylistBurnImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cfiles: u32, phrstati: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInitResults(cfiles, ::core::mem::transmute_copy(&phrstati)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: IWMReaderPlaylistBurnImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EndPlaylistBurn<Impl: IWMReaderPlaylistBurnImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrburnresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EndPlaylistBurn(hrburnresult) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMReaderPlaylistBurn>, base.5, InitPlaylistBurn::<Impl, OFFSET>, GetInitResults::<Impl, OFFSET>, Cancel::<Impl, OFFSET>, EndPlaylistBurn::<Impl, OFFSET>)
    }
}
pub trait IWMReaderStreamClockImpl: Sized {
    fn GetTime();
    fn SetTimer();
    fn KillTimer();
}
impl ::windows::core::RuntimeName for IWMReaderStreamClock {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMReaderStreamClock";
}
impl IWMReaderStreamClockVtbl {
    pub const fn new<Impl: IWMReaderStreamClockImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMReaderStreamClockVtbl {
        unsafe extern "system" fn GetTime<Impl: IWMReaderStreamClockImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcnsnow: *const u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTime(pcnsnow) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTimer<Impl: IWMReaderStreamClockImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cnswhen: u64, pvparam: *const ::core::ffi::c_void, pdwtimerid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetTimer(cnswhen, &*(&pvparam as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwtimerid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KillTimer<Impl: IWMReaderStreamClockImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwtimerid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KillTimer(dwtimerid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMReaderStreamClock>, base.5, GetTime::<Impl, OFFSET>, SetTimer::<Impl, OFFSET>, KillTimer::<Impl, OFFSET>)
    }
}
pub trait IWMReaderTimecodeImpl: Sized {
    fn GetTimecodeRangeCount();
    fn GetTimecodeRangeBounds();
}
impl ::windows::core::RuntimeName for IWMReaderTimecode {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMReaderTimecode";
}
impl IWMReaderTimecodeVtbl {
    pub const fn new<Impl: IWMReaderTimecodeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMReaderTimecodeVtbl {
        unsafe extern "system" fn GetTimecodeRangeCount<Impl: IWMReaderTimecodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pwrangecount: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTimecodeRangeCount(wstreamnum, ::core::mem::transmute_copy(&pwrangecount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTimecodeRangeBounds<Impl: IWMReaderTimecodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, wrangenum: u16, pstarttimecode: *mut u32, pendtimecode: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTimecodeRangeBounds(wstreamnum, wrangenum, ::core::mem::transmute_copy(&pstarttimecode), ::core::mem::transmute_copy(&pendtimecode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMReaderTimecode>, base.5, GetTimecodeRangeCount::<Impl, OFFSET>, GetTimecodeRangeBounds::<Impl, OFFSET>)
    }
}
pub trait IWMReaderTypeNegotiationImpl: Sized {
    fn TryOutputProps();
}
impl ::windows::core::RuntimeName for IWMReaderTypeNegotiation {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMReaderTypeNegotiation";
}
impl IWMReaderTypeNegotiationVtbl {
    pub const fn new<Impl: IWMReaderTypeNegotiationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMReaderTypeNegotiationVtbl {
        unsafe extern "system" fn TryOutputProps<Impl: IWMReaderTypeNegotiationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, poutput: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryOutputProps(dwoutputnum, &*(&poutput as *const <IWMOutputMediaProps as ::windows::core::Abi>::Abi as *const <IWMOutputMediaProps as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMReaderTypeNegotiation>, base.5, TryOutputProps::<Impl, OFFSET>)
    }
}
pub trait IWMRegisterCallbackImpl: Sized {
    fn Advise();
    fn Unadvise();
}
impl ::windows::core::RuntimeName for IWMRegisterCallback {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMRegisterCallback";
}
impl IWMRegisterCallbackVtbl {
    pub const fn new<Impl: IWMRegisterCallbackImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMRegisterCallbackVtbl {
        unsafe extern "system" fn Advise<Impl: IWMRegisterCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Advise(&*(&pcallback as *const <IWMStatusCallback as ::windows::core::Abi>::Abi as *const <IWMStatusCallback as ::windows::core::DefaultType>::DefaultType), &*(&pvcontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unadvise<Impl: IWMRegisterCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Unadvise(&*(&pcallback as *const <IWMStatusCallback as ::windows::core::Abi>::Abi as *const <IWMStatusCallback as ::windows::core::DefaultType>::DefaultType), &*(&pvcontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMRegisterCallback>, base.5, Advise::<Impl, OFFSET>, Unadvise::<Impl, OFFSET>)
    }
}
pub trait IWMRegisteredDeviceImpl: Sized {
    fn GetDeviceSerialNumber();
    fn GetDeviceCertificate();
    fn GetDeviceType();
    fn GetAttributeCount();
    fn GetAttributeByIndex();
    fn GetAttributeByName();
    fn SetAttributeByName();
    fn Approve();
    fn IsValid();
    fn IsApproved();
    fn IsWmdrmCompliant();
    fn IsOpened();
    fn Open();
    fn Close();
}
impl ::windows::core::RuntimeName for IWMRegisteredDevice {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMRegisteredDevice";
}
impl IWMRegisteredDeviceVtbl {
    pub const fn new<Impl: IWMRegisteredDeviceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMRegisteredDeviceVtbl {
        unsafe extern "system" fn GetDeviceSerialNumber<Impl: IWMRegisteredDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pserialnumber: *mut DRM_VAL16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeviceSerialNumber(::core::mem::transmute_copy(&pserialnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceCertificate<Impl: IWMRegisteredDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcertificate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeviceCertificate(::core::mem::transmute_copy(&ppcertificate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceType<Impl: IWMRegisteredDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwtype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeviceType(::core::mem::transmute_copy(&pdwtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeCount<Impl: IWMRegisteredDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcattributes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAttributeCount(::core::mem::transmute_copy(&pcattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeByIndex<Impl: IWMRegisteredDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwindex: u32, pbstrname: *mut super::super::Foundation::BSTR, pbstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAttributeByIndex(dwindex, ::core::mem::transmute_copy(&pbstrname), ::core::mem::transmute_copy(&pbstrvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeByName<Impl: IWMRegisteredDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAttributeByName(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbstrvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttributeByName<Impl: IWMRegisteredDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAttributeByName(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrvalue as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Approve<Impl: IWMRegisteredDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fapprove: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Approve(&*(&fapprove as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsValid<Impl: IWMRegisteredDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfvalid: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsValid(::core::mem::transmute_copy(&pfvalid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsApproved<Impl: IWMRegisteredDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfapproved: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsApproved(::core::mem::transmute_copy(&pfapproved)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsWmdrmCompliant<Impl: IWMRegisteredDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfcompliant: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsWmdrmCompliant(::core::mem::transmute_copy(&pfcompliant)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOpened<Impl: IWMRegisteredDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfopened: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsOpened(::core::mem::transmute_copy(&pfopened)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Open<Impl: IWMRegisteredDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Open() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IWMRegisteredDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Close() {
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
            ::windows::core::GetRuntimeClassName::<IWMRegisteredDevice>,
            base.5,
            GetDeviceSerialNumber::<Impl, OFFSET>,
            GetDeviceCertificate::<Impl, OFFSET>,
            GetDeviceType::<Impl, OFFSET>,
            GetAttributeCount::<Impl, OFFSET>,
            GetAttributeByIndex::<Impl, OFFSET>,
            GetAttributeByName::<Impl, OFFSET>,
            SetAttributeByName::<Impl, OFFSET>,
            Approve::<Impl, OFFSET>,
            IsValid::<Impl, OFFSET>,
            IsApproved::<Impl, OFFSET>,
            IsWmdrmCompliant::<Impl, OFFSET>,
            IsOpened::<Impl, OFFSET>,
            Open::<Impl, OFFSET>,
            Close::<Impl, OFFSET>,
        )
    }
}
pub trait IWMSBufferAllocatorImpl: Sized {
    fn AllocateBuffer();
    fn AllocatePageSizeBuffer();
}
impl ::windows::core::RuntimeName for IWMSBufferAllocator {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMSBufferAllocator";
}
impl IWMSBufferAllocatorVtbl {
    pub const fn new<Impl: IWMSBufferAllocatorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMSBufferAllocatorVtbl {
        unsafe extern "system" fn AllocateBuffer<Impl: IWMSBufferAllocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwmaxbuffersize: u32, ppbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllocateBuffer(dwmaxbuffersize, ::core::mem::transmute_copy(&ppbuffer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllocatePageSizeBuffer<Impl: IWMSBufferAllocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwmaxbuffersize: u32, ppbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllocatePageSizeBuffer(dwmaxbuffersize, ::core::mem::transmute_copy(&ppbuffer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMSBufferAllocator>, base.5, AllocateBuffer::<Impl, OFFSET>, AllocatePageSizeBuffer::<Impl, OFFSET>)
    }
}
pub trait IWMSInternalAdminNetSourceImpl: Sized {
    fn Initialize();
    fn GetNetSourceCreator();
    fn SetCredentials();
    fn GetCredentials();
    fn DeleteCredentials();
    fn GetCredentialFlags();
    fn SetCredentialFlags();
    fn FindProxyForURL();
    fn RegisterProxyFailure();
    fn ShutdownProxyContext();
    fn IsUsingIE();
}
impl ::windows::core::RuntimeName for IWMSInternalAdminNetSource {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMSInternalAdminNetSource";
}
impl IWMSInternalAdminNetSourceVtbl {
    pub const fn new<Impl: IWMSInternalAdminNetSourceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMSInternalAdminNetSourceVtbl {
        unsafe extern "system" fn Initialize<Impl: IWMSInternalAdminNetSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psharednamespace: *mut ::core::ffi::c_void, pnamespacenode: *mut ::core::ffi::c_void, pnetsourcecreator: ::windows::core::RawPtr, fembeddedinserver: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Initialize(
                &*(&psharednamespace as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&pnamespacenode as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&pnetsourcecreator as *const <INSNetSourceCreator as ::windows::core::Abi>::Abi as *const <INSNetSourceCreator as ::windows::core::DefaultType>::DefaultType),
                &*(&fembeddedinserver as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetSourceCreator<Impl: IWMSInternalAdminNetSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppnetsourcecreator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNetSourceCreator(::core::mem::transmute_copy(&ppnetsourcecreator)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCredentials<Impl: IWMSInternalAdminNetSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fpersist: super::super::Foundation::BOOL, fconfirmedgood: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetCredentials(
                &*(&bstrrealm as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrpassword as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&fpersist as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&fconfirmedgood as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCredentials<Impl: IWMSInternalAdminNetSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrname: *mut super::super::Foundation::BSTR, pbstrpassword: *mut super::super::Foundation::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCredentials(&*(&bstrrealm as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbstrname), ::core::mem::transmute_copy(&pbstrpassword), ::core::mem::transmute_copy(&pfconfirmedgood)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteCredentials<Impl: IWMSInternalAdminNetSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteCredentials(&*(&bstrrealm as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCredentialFlags<Impl: IWMSInternalAdminNetSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCredentialFlags(::core::mem::transmute_copy(&lpdwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCredentialFlags<Impl: IWMSInternalAdminNetSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetCredentialFlags(dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindProxyForURL<Impl: IWMSInternalAdminNetSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrhost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut super::super::Foundation::BSTR, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindProxyForURL(
                &*(&bstrprotocol as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrhost as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pfproxyenabled),
                ::core::mem::transmute_copy(&pbstrproxyserver),
                ::core::mem::transmute_copy(&pdwproxyport),
                pdwproxycontext,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterProxyFailure<Impl: IWMSInternalAdminNetSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrparam: ::windows::core::HRESULT, dwproxycontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RegisterProxyFailure(hrparam, dwproxycontext) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShutdownProxyContext<Impl: IWMSInternalAdminNetSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwproxycontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShutdownProxyContext(dwproxycontext) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUsingIE<Impl: IWMSInternalAdminNetSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwproxycontext: u32, pfisusingie: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsUsingIE(dwproxycontext, ::core::mem::transmute_copy(&pfisusingie)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMSInternalAdminNetSource>, base.5, Initialize::<Impl, OFFSET>, GetNetSourceCreator::<Impl, OFFSET>, SetCredentials::<Impl, OFFSET>, GetCredentials::<Impl, OFFSET>, DeleteCredentials::<Impl, OFFSET>, GetCredentialFlags::<Impl, OFFSET>, SetCredentialFlags::<Impl, OFFSET>, FindProxyForURL::<Impl, OFFSET>, RegisterProxyFailure::<Impl, OFFSET>, ShutdownProxyContext::<Impl, OFFSET>, IsUsingIE::<Impl, OFFSET>)
    }
}
pub trait IWMSInternalAdminNetSource2Impl: Sized {
    fn SetCredentialsEx();
    fn GetCredentialsEx();
    fn DeleteCredentialsEx();
    fn FindProxyForURLEx();
}
impl ::windows::core::RuntimeName for IWMSInternalAdminNetSource2 {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMSInternalAdminNetSource2";
}
impl IWMSInternalAdminNetSource2Vtbl {
    pub const fn new<Impl: IWMSInternalAdminNetSource2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMSInternalAdminNetSource2Vtbl {
        unsafe extern "system" fn SetCredentialsEx<Impl: IWMSInternalAdminNetSource2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fproxy: super::super::Foundation::BOOL, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fpersist: super::super::Foundation::BOOL, fconfirmedgood: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetCredentialsEx(
                &*(&bstrrealm as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrurl as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&fproxy as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrpassword as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&fpersist as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&fconfirmedgood as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCredentialsEx<Impl: IWMSInternalAdminNetSource2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fproxy: super::super::Foundation::BOOL, pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS, pbstrname: *mut super::super::Foundation::BSTR, pbstrpassword: *mut super::super::Foundation::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCredentialsEx(
                &*(&bstrrealm as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrurl as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&fproxy as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pdwurlpolicy),
                ::core::mem::transmute_copy(&pbstrname),
                ::core::mem::transmute_copy(&pbstrpassword),
                ::core::mem::transmute_copy(&pfconfirmedgood),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteCredentialsEx<Impl: IWMSInternalAdminNetSource2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fproxy: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteCredentialsEx(
                &*(&bstrrealm as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrurl as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&fproxy as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindProxyForURLEx<Impl: IWMSInternalAdminNetSource2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrhost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut super::super::Foundation::BSTR, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindProxyForURLEx(
                &*(&bstrprotocol as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrhost as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrurl as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pfproxyenabled),
                ::core::mem::transmute_copy(&pbstrproxyserver),
                ::core::mem::transmute_copy(&pdwproxyport),
                pdwproxycontext,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMSInternalAdminNetSource2>, base.5, SetCredentialsEx::<Impl, OFFSET>, GetCredentialsEx::<Impl, OFFSET>, DeleteCredentialsEx::<Impl, OFFSET>, FindProxyForURLEx::<Impl, OFFSET>)
    }
}
pub trait IWMSInternalAdminNetSource3Impl: Sized + IWMSInternalAdminNetSource2Impl {
    fn GetNetSourceCreator2();
    fn FindProxyForURLEx2();
    fn RegisterProxyFailure2();
    fn ShutdownProxyContext2();
    fn IsUsingIE2();
    fn SetCredentialsEx2();
    fn GetCredentialsEx2();
}
impl ::windows::core::RuntimeName for IWMSInternalAdminNetSource3 {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMSInternalAdminNetSource3";
}
impl IWMSInternalAdminNetSource3Vtbl {
    pub const fn new<Impl: IWMSInternalAdminNetSource3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMSInternalAdminNetSource3Vtbl {
        unsafe extern "system" fn GetNetSourceCreator2<Impl: IWMSInternalAdminNetSource3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppnetsourcecreator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNetSourceCreator2(::core::mem::transmute_copy(&ppnetsourcecreator)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindProxyForURLEx2<Impl: IWMSInternalAdminNetSource3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrhost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut super::super::Foundation::BSTR, pdwproxyport: *mut u32, pqwproxycontext: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindProxyForURLEx2(
                &*(&bstrprotocol as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrhost as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrurl as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pfproxyenabled),
                ::core::mem::transmute_copy(&pbstrproxyserver),
                ::core::mem::transmute_copy(&pdwproxyport),
                pqwproxycontext,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterProxyFailure2<Impl: IWMSInternalAdminNetSource3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrparam: ::windows::core::HRESULT, qwproxycontext: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RegisterProxyFailure2(hrparam, qwproxycontext) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShutdownProxyContext2<Impl: IWMSInternalAdminNetSource3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, qwproxycontext: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShutdownProxyContext2(qwproxycontext) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUsingIE2<Impl: IWMSInternalAdminNetSource3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, qwproxycontext: u64, pfisusingie: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsUsingIE2(qwproxycontext, ::core::mem::transmute_copy(&pfisusingie)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCredentialsEx2<Impl: IWMSInternalAdminNetSource3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fproxy: super::super::Foundation::BOOL, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fpersist: super::super::Foundation::BOOL, fconfirmedgood: super::super::Foundation::BOOL, fcleartextauthentication: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetCredentialsEx2(
                &*(&bstrrealm as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrurl as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&fproxy as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrpassword as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&fpersist as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&fconfirmedgood as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&fcleartextauthentication as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCredentialsEx2<Impl: IWMSInternalAdminNetSource3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fproxy: super::super::Foundation::BOOL, fcleartextauthentication: super::super::Foundation::BOOL, pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS, pbstrname: *mut super::super::Foundation::BSTR, pbstrpassword: *mut super::super::Foundation::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCredentialsEx2(
                &*(&bstrrealm as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrurl as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&fproxy as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&fcleartextauthentication as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pdwurlpolicy),
                ::core::mem::transmute_copy(&pbstrname),
                ::core::mem::transmute_copy(&pbstrpassword),
                ::core::mem::transmute_copy(&pfconfirmedgood),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMSInternalAdminNetSource3>, base.5, GetNetSourceCreator2::<Impl, OFFSET>, FindProxyForURLEx2::<Impl, OFFSET>, RegisterProxyFailure2::<Impl, OFFSET>, ShutdownProxyContext2::<Impl, OFFSET>, IsUsingIE2::<Impl, OFFSET>, SetCredentialsEx2::<Impl, OFFSET>, GetCredentialsEx2::<Impl, OFFSET>)
    }
}
pub trait IWMSecureChannelImpl: Sized + IWMAuthorizerImpl {
    fn WMSC_AddCertificate();
    fn WMSC_AddSignature();
    fn WMSC_Connect();
    fn WMSC_IsConnected();
    fn WMSC_Disconnect();
    fn WMSC_GetValidCertificate();
    fn WMSC_Encrypt();
    fn WMSC_Decrypt();
    fn WMSC_Lock();
    fn WMSC_Unlock();
    fn WMSC_SetSharedData();
}
impl ::windows::core::RuntimeName for IWMSecureChannel {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMSecureChannel";
}
impl IWMSecureChannelVtbl {
    pub const fn new<Impl: IWMSecureChannelImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMSecureChannelVtbl {
        unsafe extern "system" fn WMSC_AddCertificate<Impl: IWMSecureChannelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcert: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WMSC_AddCertificate(&*(&pcert as *const <IWMAuthorizer as ::windows::core::Abi>::Abi as *const <IWMAuthorizer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WMSC_AddSignature<Impl: IWMSecureChannelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbcertsig: *const u8, cbcertsig: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WMSC_AddSignature(pbcertsig, cbcertsig) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WMSC_Connect<Impl: IWMSecureChannelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, potherside: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WMSC_Connect(&*(&potherside as *const <IWMSecureChannel as ::windows::core::Abi>::Abi as *const <IWMSecureChannel as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WMSC_IsConnected<Impl: IWMSecureChannelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfisconnected: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WMSC_IsConnected(::core::mem::transmute_copy(&pfisconnected)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WMSC_Disconnect<Impl: IWMSecureChannelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WMSC_Disconnect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WMSC_GetValidCertificate<Impl: IWMSecureChannelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppbcertificate: *mut *mut u8, pdwsignature: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WMSC_GetValidCertificate(::core::mem::transmute_copy(&ppbcertificate), ::core::mem::transmute_copy(&pdwsignature)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WMSC_Encrypt<Impl: IWMSecureChannelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbdata: *const u8, cbdata: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WMSC_Encrypt(pbdata, cbdata) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WMSC_Decrypt<Impl: IWMSecureChannelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbdata: *const u8, cbdata: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WMSC_Decrypt(pbdata, cbdata) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WMSC_Lock<Impl: IWMSecureChannelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WMSC_Lock() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WMSC_Unlock<Impl: IWMSecureChannelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WMSC_Unlock() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WMSC_SetSharedData<Impl: IWMSecureChannelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcertindex: u32, pbshareddata: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WMSC_SetSharedData(dwcertindex, pbshareddata) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMSecureChannel>, base.5, WMSC_AddCertificate::<Impl, OFFSET>, WMSC_AddSignature::<Impl, OFFSET>, WMSC_Connect::<Impl, OFFSET>, WMSC_IsConnected::<Impl, OFFSET>, WMSC_Disconnect::<Impl, OFFSET>, WMSC_GetValidCertificate::<Impl, OFFSET>, WMSC_Encrypt::<Impl, OFFSET>, WMSC_Decrypt::<Impl, OFFSET>, WMSC_Lock::<Impl, OFFSET>, WMSC_Unlock::<Impl, OFFSET>, WMSC_SetSharedData::<Impl, OFFSET>)
    }
}
pub trait IWMStatusCallbackImpl: Sized {
    fn OnStatus();
}
impl ::windows::core::RuntimeName for IWMStatusCallback {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMStatusCallback";
}
impl IWMStatusCallbackVtbl {
    pub const fn new<Impl: IWMStatusCallbackImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMStatusCallbackVtbl {
        unsafe extern "system" fn OnStatus<Impl: IWMStatusCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, status: WMT_STATUS, hr: ::windows::core::HRESULT, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnStatus(status, hr, dwtype, pvalue, &*(&pvcontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMStatusCallback>, base.5, OnStatus::<Impl, OFFSET>)
    }
}
pub trait IWMStreamConfigImpl: Sized {
    fn GetStreamType();
    fn GetStreamNumber();
    fn SetStreamNumber();
    fn GetStreamName();
    fn SetStreamName();
    fn GetConnectionName();
    fn SetConnectionName();
    fn GetBitrate();
    fn SetBitrate();
    fn GetBufferWindow();
    fn SetBufferWindow();
}
impl ::windows::core::RuntimeName for IWMStreamConfig {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMStreamConfig";
}
impl IWMStreamConfigVtbl {
    pub const fn new<Impl: IWMStreamConfigImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMStreamConfigVtbl {
        unsafe extern "system" fn GetStreamType<Impl: IWMStreamConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidstreamtype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStreamType(::core::mem::transmute_copy(&pguidstreamtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStreamNumber<Impl: IWMStreamConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwstreamnum: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStreamNumber(::core::mem::transmute_copy(&pwstreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStreamNumber<Impl: IWMStreamConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetStreamNumber(wstreamnum) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStreamName<Impl: IWMStreamConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszstreamname: super::super::Foundation::PWSTR, pcchstreamname: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStreamName(::core::mem::transmute_copy(&pwszstreamname), pcchstreamname) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStreamName<Impl: IWMStreamConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszstreamname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetStreamName(&*(&pwszstreamname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectionName<Impl: IWMStreamConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszinputname: super::super::Foundation::PWSTR, pcchinputname: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetConnectionName(::core::mem::transmute_copy(&pwszinputname), pcchinputname) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConnectionName<Impl: IWMStreamConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszinputname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetConnectionName(&*(&pwszinputname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBitrate<Impl: IWMStreamConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwbitrate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBitrate(::core::mem::transmute_copy(&pdwbitrate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBitrate<Impl: IWMStreamConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwbitrate: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetBitrate(pdwbitrate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBufferWindow<Impl: IWMStreamConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmsbufferwindow: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBufferWindow(::core::mem::transmute_copy(&pmsbufferwindow)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferWindow<Impl: IWMStreamConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, msbufferwindow: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetBufferWindow(msbufferwindow) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMStreamConfig>, base.5, GetStreamType::<Impl, OFFSET>, GetStreamNumber::<Impl, OFFSET>, SetStreamNumber::<Impl, OFFSET>, GetStreamName::<Impl, OFFSET>, SetStreamName::<Impl, OFFSET>, GetConnectionName::<Impl, OFFSET>, SetConnectionName::<Impl, OFFSET>, GetBitrate::<Impl, OFFSET>, SetBitrate::<Impl, OFFSET>, GetBufferWindow::<Impl, OFFSET>, SetBufferWindow::<Impl, OFFSET>)
    }
}
pub trait IWMStreamConfig2Impl: Sized + IWMStreamConfigImpl {
    fn GetTransportType();
    fn SetTransportType();
    fn AddDataUnitExtension();
    fn GetDataUnitExtensionCount();
    fn GetDataUnitExtension();
    fn RemoveAllDataUnitExtensions();
}
impl ::windows::core::RuntimeName for IWMStreamConfig2 {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMStreamConfig2";
}
impl IWMStreamConfig2Vtbl {
    pub const fn new<Impl: IWMStreamConfig2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMStreamConfig2Vtbl {
        unsafe extern "system" fn GetTransportType<Impl: IWMStreamConfig2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pntransporttype: *mut WMT_TRANSPORT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTransportType(::core::mem::transmute_copy(&pntransporttype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransportType<Impl: IWMStreamConfig2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ntransporttype: WMT_TRANSPORT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetTransportType(ntransporttype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddDataUnitExtension<Impl: IWMStreamConfig2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidextensionsystemid: ::windows::core::GUID, cbextensiondatasize: u16, pbextensionsysteminfo: *const u8, cbextensionsysteminfo: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddDataUnitExtension(&*(&guidextensionsystemid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), cbextensiondatasize, pbextensionsysteminfo, cbextensionsysteminfo) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataUnitExtensionCount<Impl: IWMStreamConfig2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcdataunitextensions: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDataUnitExtensionCount(::core::mem::transmute_copy(&pcdataunitextensions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataUnitExtension<Impl: IWMStreamConfig2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wdataunitextensionnumber: u16, pguidextensionsystemid: *mut ::windows::core::GUID, pcbextensiondatasize: *mut u16, pbextensionsysteminfo: *mut u8, pcbextensionsysteminfo: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDataUnitExtension(wdataunitextensionnumber, ::core::mem::transmute_copy(&pguidextensionsystemid), ::core::mem::transmute_copy(&pcbextensiondatasize), ::core::mem::transmute_copy(&pbextensionsysteminfo), pcbextensionsysteminfo) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAllDataUnitExtensions<Impl: IWMStreamConfig2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveAllDataUnitExtensions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMStreamConfig2>, base.5, GetTransportType::<Impl, OFFSET>, SetTransportType::<Impl, OFFSET>, AddDataUnitExtension::<Impl, OFFSET>, GetDataUnitExtensionCount::<Impl, OFFSET>, GetDataUnitExtension::<Impl, OFFSET>, RemoveAllDataUnitExtensions::<Impl, OFFSET>)
    }
}
pub trait IWMStreamConfig3Impl: Sized + IWMStreamConfig2Impl + IWMStreamConfigImpl {
    fn GetLanguage();
    fn SetLanguage();
}
impl ::windows::core::RuntimeName for IWMStreamConfig3 {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMStreamConfig3";
}
impl IWMStreamConfig3Vtbl {
    pub const fn new<Impl: IWMStreamConfig3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMStreamConfig3Vtbl {
        unsafe extern "system" fn GetLanguage<Impl: IWMStreamConfig3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszlanguagestring: super::super::Foundation::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLanguage(::core::mem::transmute_copy(&pwszlanguagestring), pcchlanguagestringlength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguage<Impl: IWMStreamConfig3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszlanguagestring: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetLanguage(&*(&pwszlanguagestring as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMStreamConfig3>, base.5, GetLanguage::<Impl, OFFSET>, SetLanguage::<Impl, OFFSET>)
    }
}
pub trait IWMStreamListImpl: Sized {
    fn GetStreams();
    fn AddStream();
    fn RemoveStream();
}
impl ::windows::core::RuntimeName for IWMStreamList {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMStreamList";
}
impl IWMStreamListVtbl {
    pub const fn new<Impl: IWMStreamListImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMStreamListVtbl {
        unsafe extern "system" fn GetStreams<Impl: IWMStreamListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStreams(::core::mem::transmute_copy(&pwstreamnumarray), pcstreams) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddStream<Impl: IWMStreamListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddStream(wstreamnum) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStream<Impl: IWMStreamListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveStream(wstreamnum) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMStreamList>, base.5, GetStreams::<Impl, OFFSET>, AddStream::<Impl, OFFSET>, RemoveStream::<Impl, OFFSET>)
    }
}
pub trait IWMStreamPrioritizationImpl: Sized {
    fn GetPriorityRecords();
    fn SetPriorityRecords();
}
impl ::windows::core::RuntimeName for IWMStreamPrioritization {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMStreamPrioritization";
}
impl IWMStreamPrioritizationVtbl {
    pub const fn new<Impl: IWMStreamPrioritizationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMStreamPrioritizationVtbl {
        unsafe extern "system" fn GetPriorityRecords<Impl: IWMStreamPrioritizationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, precordarray: *mut WM_STREAM_PRIORITY_RECORD, pcrecords: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPriorityRecords(::core::mem::transmute_copy(&precordarray), pcrecords) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriorityRecords<Impl: IWMStreamPrioritizationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, precordarray: *const WM_STREAM_PRIORITY_RECORD, crecords: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPriorityRecords(&*(&precordarray as *const <WM_STREAM_PRIORITY_RECORD as ::windows::core::Abi>::Abi as *const <WM_STREAM_PRIORITY_RECORD as ::windows::core::DefaultType>::DefaultType), crecords) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMStreamPrioritization>, base.5, GetPriorityRecords::<Impl, OFFSET>, SetPriorityRecords::<Impl, OFFSET>)
    }
}
pub trait IWMSyncReaderImpl: Sized {
    fn Open();
    fn Close();
    fn SetRange();
    fn SetRangeByFrame();
    fn GetNextSample();
    fn SetStreamsSelected();
    fn GetStreamSelected();
    fn SetReadStreamSamples();
    fn GetReadStreamSamples();
    fn GetOutputSetting();
    fn SetOutputSetting();
    fn GetOutputCount();
    fn GetOutputProps();
    fn SetOutputProps();
    fn GetOutputFormatCount();
    fn GetOutputFormat();
    fn GetOutputNumberForStream();
    fn GetStreamNumberForOutput();
    fn GetMaxOutputSampleSize();
    fn GetMaxStreamSampleSize();
    fn OpenStream();
}
impl ::windows::core::RuntimeName for IWMSyncReader {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMSyncReader";
}
impl IWMSyncReaderVtbl {
    pub const fn new<Impl: IWMSyncReaderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMSyncReaderVtbl {
        unsafe extern "system" fn Open<Impl: IWMSyncReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Open(&*(&pwszfilename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IWMSyncReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Close() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRange<Impl: IWMSyncReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cnsstarttime: u64, cnsduration: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetRange(cnsstarttime, cnsduration) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRangeByFrame<Impl: IWMSyncReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, qwframenumber: u64, cframestoread: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetRangeByFrame(wstreamnum, qwframenumber, cframestoread) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNextSample<Impl: IWMSyncReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, ppsample: *mut ::windows::core::RawPtr, pcnssampletime: *mut u64, pcnsduration: *mut u64, pdwflags: *mut u32, pdwoutputnum: *mut u32, pwstreamnum: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNextSample(wstreamnum, ::core::mem::transmute_copy(&ppsample), ::core::mem::transmute_copy(&pcnssampletime), ::core::mem::transmute_copy(&pcnsduration), ::core::mem::transmute_copy(&pdwflags), ::core::mem::transmute_copy(&pdwoutputnum), ::core::mem::transmute_copy(&pwstreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStreamsSelected<Impl: IWMSyncReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetStreamsSelected(cstreamcount, pwstreamnumbers, pselections) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStreamSelected<Impl: IWMSyncReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pselection: *mut WMT_STREAM_SELECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStreamSelected(wstreamnum, ::core::mem::transmute_copy(&pselection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReadStreamSamples<Impl: IWMSyncReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, fcompressed: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetReadStreamSamples(wstreamnum, &*(&fcompressed as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReadStreamSamples<Impl: IWMSyncReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pfcompressed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetReadStreamSamples(wstreamnum, ::core::mem::transmute_copy(&pfcompressed)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputSetting<Impl: IWMSyncReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOutputSetting(dwoutputnum, &*(&pszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), pcblength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutputSetting<Impl: IWMSyncReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetOutputSetting(dwoutputnum, &*(&pszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), r#type, pvalue, cblength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputCount<Impl: IWMSyncReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcoutputs: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOutputCount(::core::mem::transmute_copy(&pcoutputs)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputProps<Impl: IWMSyncReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, ppoutput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOutputProps(dwoutputnum, ::core::mem::transmute_copy(&ppoutput)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutputProps<Impl: IWMSyncReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, poutput: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetOutputProps(dwoutputnum, &*(&poutput as *const <IWMOutputMediaProps as ::windows::core::Abi>::Abi as *const <IWMOutputMediaProps as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputFormatCount<Impl: IWMSyncReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pcformats: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOutputFormatCount(dwoutputnum, ::core::mem::transmute_copy(&pcformats)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputFormat<Impl: IWMSyncReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, dwformatnum: u32, ppprops: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOutputFormat(dwoutputnum, dwformatnum, ::core::mem::transmute_copy(&ppprops)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputNumberForStream<Impl: IWMSyncReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pdwoutputnum: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOutputNumberForStream(wstreamnum, ::core::mem::transmute_copy(&pdwoutputnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStreamNumberForOutput<Impl: IWMSyncReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pwstreamnum: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStreamNumberForOutput(dwoutputnum, ::core::mem::transmute_copy(&pwstreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxOutputSampleSize<Impl: IWMSyncReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutput: u32, pcbmax: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMaxOutputSampleSize(dwoutput, ::core::mem::transmute_copy(&pcbmax)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxStreamSampleSize<Impl: IWMSyncReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstream: u16, pcbmax: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMaxStreamSampleSize(wstream, ::core::mem::transmute_copy(&pcbmax)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenStream<Impl: IWMSyncReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenStream(&*(&pstream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IWMSyncReader>,
            base.5,
            Open::<Impl, OFFSET>,
            Close::<Impl, OFFSET>,
            SetRange::<Impl, OFFSET>,
            SetRangeByFrame::<Impl, OFFSET>,
            GetNextSample::<Impl, OFFSET>,
            SetStreamsSelected::<Impl, OFFSET>,
            GetStreamSelected::<Impl, OFFSET>,
            SetReadStreamSamples::<Impl, OFFSET>,
            GetReadStreamSamples::<Impl, OFFSET>,
            GetOutputSetting::<Impl, OFFSET>,
            SetOutputSetting::<Impl, OFFSET>,
            GetOutputCount::<Impl, OFFSET>,
            GetOutputProps::<Impl, OFFSET>,
            SetOutputProps::<Impl, OFFSET>,
            GetOutputFormatCount::<Impl, OFFSET>,
            GetOutputFormat::<Impl, OFFSET>,
            GetOutputNumberForStream::<Impl, OFFSET>,
            GetStreamNumberForOutput::<Impl, OFFSET>,
            GetMaxOutputSampleSize::<Impl, OFFSET>,
            GetMaxStreamSampleSize::<Impl, OFFSET>,
            OpenStream::<Impl, OFFSET>,
        )
    }
}
pub trait IWMSyncReader2Impl: Sized + IWMSyncReaderImpl {
    fn SetRangeByTimecode();
    fn SetRangeByFrameEx();
    fn SetAllocateForOutput();
    fn GetAllocateForOutput();
    fn SetAllocateForStream();
    fn GetAllocateForStream();
}
impl ::windows::core::RuntimeName for IWMSyncReader2 {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMSyncReader2";
}
impl IWMSyncReader2Vtbl {
    pub const fn new<Impl: IWMSyncReader2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMSyncReader2Vtbl {
        unsafe extern "system" fn SetRangeByTimecode<Impl: IWMSyncReader2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pstart: *const WMT_TIMECODE_EXTENSION_DATA, pend: *const WMT_TIMECODE_EXTENSION_DATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetRangeByTimecode(wstreamnum, &*(&pstart as *const <WMT_TIMECODE_EXTENSION_DATA as ::windows::core::Abi>::Abi as *const <WMT_TIMECODE_EXTENSION_DATA as ::windows::core::DefaultType>::DefaultType), &*(&pend as *const <WMT_TIMECODE_EXTENSION_DATA as ::windows::core::Abi>::Abi as *const <WMT_TIMECODE_EXTENSION_DATA as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRangeByFrameEx<Impl: IWMSyncReader2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, qwframenumber: u64, cframestoread: i64, pcnsstarttime: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetRangeByFrameEx(wstreamnum, qwframenumber, cframestoread, ::core::mem::transmute_copy(&pcnsstarttime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllocateForOutput<Impl: IWMSyncReader2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pallocator: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAllocateForOutput(dwoutputnum, &*(&pallocator as *const <IWMReaderAllocatorEx as ::windows::core::Abi>::Abi as *const <IWMReaderAllocatorEx as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllocateForOutput<Impl: IWMSyncReader2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, ppallocator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAllocateForOutput(dwoutputnum, ::core::mem::transmute_copy(&ppallocator)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllocateForStream<Impl: IWMSyncReader2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pallocator: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAllocateForStream(wstreamnum, &*(&pallocator as *const <IWMReaderAllocatorEx as ::windows::core::Abi>::Abi as *const <IWMReaderAllocatorEx as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllocateForStream<Impl: IWMSyncReader2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwsreamnum: u16, ppallocator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAllocateForStream(dwsreamnum, ::core::mem::transmute_copy(&ppallocator)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMSyncReader2>, base.5, SetRangeByTimecode::<Impl, OFFSET>, SetRangeByFrameEx::<Impl, OFFSET>, SetAllocateForOutput::<Impl, OFFSET>, GetAllocateForOutput::<Impl, OFFSET>, SetAllocateForStream::<Impl, OFFSET>, GetAllocateForStream::<Impl, OFFSET>)
    }
}
pub trait IWMVideoMediaPropsImpl: Sized + IWMMediaPropsImpl {
    fn GetMaxKeyFrameSpacing();
    fn SetMaxKeyFrameSpacing();
    fn GetQuality();
    fn SetQuality();
}
impl ::windows::core::RuntimeName for IWMVideoMediaProps {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMVideoMediaProps";
}
impl IWMVideoMediaPropsVtbl {
    pub const fn new<Impl: IWMVideoMediaPropsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMVideoMediaPropsVtbl {
        unsafe extern "system" fn GetMaxKeyFrameSpacing<Impl: IWMVideoMediaPropsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plltime: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMaxKeyFrameSpacing(::core::mem::transmute_copy(&plltime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxKeyFrameSpacing<Impl: IWMVideoMediaPropsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lltime: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetMaxKeyFrameSpacing(lltime) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetQuality<Impl: IWMVideoMediaPropsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwquality: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetQuality(::core::mem::transmute_copy(&pdwquality)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuality<Impl: IWMVideoMediaPropsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwquality: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetQuality(dwquality) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMVideoMediaProps>, base.5, GetMaxKeyFrameSpacing::<Impl, OFFSET>, SetMaxKeyFrameSpacing::<Impl, OFFSET>, GetQuality::<Impl, OFFSET>, SetQuality::<Impl, OFFSET>)
    }
}
pub trait IWMWatermarkInfoImpl: Sized {
    fn GetWatermarkEntryCount();
    fn GetWatermarkEntry();
}
impl ::windows::core::RuntimeName for IWMWatermarkInfo {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMWatermarkInfo";
}
impl IWMWatermarkInfoVtbl {
    pub const fn new<Impl: IWMWatermarkInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMWatermarkInfoVtbl {
        unsafe extern "system" fn GetWatermarkEntryCount<Impl: IWMWatermarkInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wmettype: WMT_WATERMARK_ENTRY_TYPE, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetWatermarkEntryCount(wmettype, ::core::mem::transmute_copy(&pdwcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWatermarkEntry<Impl: IWMWatermarkInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wmettype: WMT_WATERMARK_ENTRY_TYPE, dwentrynum: u32, pentry: *mut WMT_WATERMARK_ENTRY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetWatermarkEntry(wmettype, dwentrynum, ::core::mem::transmute_copy(&pentry)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMWatermarkInfo>, base.5, GetWatermarkEntryCount::<Impl, OFFSET>, GetWatermarkEntry::<Impl, OFFSET>)
    }
}
pub trait IWMWriterImpl: Sized {
    fn SetProfileByID();
    fn SetProfile();
    fn SetOutputFilename();
    fn GetInputCount();
    fn GetInputProps();
    fn SetInputProps();
    fn GetInputFormatCount();
    fn GetInputFormat();
    fn BeginWriting();
    fn EndWriting();
    fn AllocateSample();
    fn WriteSample();
    fn Flush();
}
impl ::windows::core::RuntimeName for IWMWriter {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMWriter";
}
impl IWMWriterVtbl {
    pub const fn new<Impl: IWMWriterImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMWriterVtbl {
        unsafe extern "system" fn SetProfileByID<Impl: IWMWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidprofile: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetProfileByID(&*(&guidprofile as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProfile<Impl: IWMWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprofile: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetProfile(&*(&pprofile as *const <IWMProfile as ::windows::core::Abi>::Abi as *const <IWMProfile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutputFilename<Impl: IWMWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetOutputFilename(&*(&pwszfilename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputCount<Impl: IWMWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcinputs: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInputCount(::core::mem::transmute_copy(&pcinputs)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputProps<Impl: IWMWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, ppinput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInputProps(dwinputnum, ::core::mem::transmute_copy(&ppinput)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInputProps<Impl: IWMWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, pinput: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetInputProps(dwinputnum, &*(&pinput as *const <IWMInputMediaProps as ::windows::core::Abi>::Abi as *const <IWMInputMediaProps as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputFormatCount<Impl: IWMWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwinputnumber: u32, pcformats: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInputFormatCount(dwinputnumber, ::core::mem::transmute_copy(&pcformats)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputFormat<Impl: IWMWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwinputnumber: u32, dwformatnumber: u32, pprops: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInputFormat(dwinputnumber, dwformatnumber, ::core::mem::transmute_copy(&pprops)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginWriting<Impl: IWMWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BeginWriting() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndWriting<Impl: IWMWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EndWriting() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllocateSample<Impl: IWMWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwsamplesize: u32, ppsample: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllocateSample(dwsamplesize, ::core::mem::transmute_copy(&ppsample)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteSample<Impl: IWMWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, cnssampletime: u64, dwflags: u32, psample: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WriteSample(dwinputnum, cnssampletime, dwflags, &*(&psample as *const <INSSBuffer as ::windows::core::Abi>::Abi as *const <INSSBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Flush<Impl: IWMWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Flush() {
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
            ::windows::core::GetRuntimeClassName::<IWMWriter>,
            base.5,
            SetProfileByID::<Impl, OFFSET>,
            SetProfile::<Impl, OFFSET>,
            SetOutputFilename::<Impl, OFFSET>,
            GetInputCount::<Impl, OFFSET>,
            GetInputProps::<Impl, OFFSET>,
            SetInputProps::<Impl, OFFSET>,
            GetInputFormatCount::<Impl, OFFSET>,
            GetInputFormat::<Impl, OFFSET>,
            BeginWriting::<Impl, OFFSET>,
            EndWriting::<Impl, OFFSET>,
            AllocateSample::<Impl, OFFSET>,
            WriteSample::<Impl, OFFSET>,
            Flush::<Impl, OFFSET>,
        )
    }
}
pub trait IWMWriterAdvancedImpl: Sized {
    fn GetSinkCount();
    fn GetSink();
    fn AddSink();
    fn RemoveSink();
    fn WriteStreamSample();
    fn SetLiveSource();
    fn IsRealTime();
    fn GetWriterTime();
    fn GetStatistics();
    fn SetSyncTolerance();
    fn GetSyncTolerance();
}
impl ::windows::core::RuntimeName for IWMWriterAdvanced {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMWriterAdvanced";
}
impl IWMWriterAdvancedVtbl {
    pub const fn new<Impl: IWMWriterAdvancedImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMWriterAdvancedVtbl {
        unsafe extern "system" fn GetSinkCount<Impl: IWMWriterAdvancedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcsinks: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSinkCount(::core::mem::transmute_copy(&pcsinks)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSink<Impl: IWMWriterAdvancedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwsinknum: u32, ppsink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSink(dwsinknum, ::core::mem::transmute_copy(&ppsink)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddSink<Impl: IWMWriterAdvancedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddSink(&*(&psink as *const <IWMWriterSink as ::windows::core::Abi>::Abi as *const <IWMWriterSink as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSink<Impl: IWMWriterAdvancedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveSink(&*(&psink as *const <IWMWriterSink as ::windows::core::Abi>::Abi as *const <IWMWriterSink as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteStreamSample<Impl: IWMWriterAdvancedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, cnssampletime: u64, mssamplesendtime: u32, cnssampleduration: u64, dwflags: u32, psample: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WriteStreamSample(wstreamnum, cnssampletime, mssamplesendtime, cnssampleduration, dwflags, &*(&psample as *const <INSSBuffer as ::windows::core::Abi>::Abi as *const <INSSBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLiveSource<Impl: IWMWriterAdvancedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fislivesource: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetLiveSource(&*(&fislivesource as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRealTime<Impl: IWMWriterAdvancedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfrealtime: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsRealTime(::core::mem::transmute_copy(&pfrealtime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWriterTime<Impl: IWMWriterAdvancedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcnscurrenttime: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetWriterTime(::core::mem::transmute_copy(&pcnscurrenttime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatistics<Impl: IWMWriterAdvancedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pstats: *mut WM_WRITER_STATISTICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStatistics(wstreamnum, ::core::mem::transmute_copy(&pstats)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSyncTolerance<Impl: IWMWriterAdvancedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mswindow: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSyncTolerance(mswindow) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSyncTolerance<Impl: IWMWriterAdvancedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmswindow: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSyncTolerance(::core::mem::transmute_copy(&pmswindow)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMWriterAdvanced>, base.5, GetSinkCount::<Impl, OFFSET>, GetSink::<Impl, OFFSET>, AddSink::<Impl, OFFSET>, RemoveSink::<Impl, OFFSET>, WriteStreamSample::<Impl, OFFSET>, SetLiveSource::<Impl, OFFSET>, IsRealTime::<Impl, OFFSET>, GetWriterTime::<Impl, OFFSET>, GetStatistics::<Impl, OFFSET>, SetSyncTolerance::<Impl, OFFSET>, GetSyncTolerance::<Impl, OFFSET>)
    }
}
pub trait IWMWriterAdvanced2Impl: Sized + IWMWriterAdvancedImpl {
    fn GetInputSetting();
    fn SetInputSetting();
}
impl ::windows::core::RuntimeName for IWMWriterAdvanced2 {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMWriterAdvanced2";
}
impl IWMWriterAdvanced2Vtbl {
    pub const fn new<Impl: IWMWriterAdvanced2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMWriterAdvanced2Vtbl {
        unsafe extern "system" fn GetInputSetting<Impl: IWMWriterAdvanced2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInputSetting(dwinputnum, &*(&pszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), pcblength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInputSetting<Impl: IWMWriterAdvanced2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetInputSetting(dwinputnum, &*(&pszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), r#type, pvalue, cblength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMWriterAdvanced2>, base.5, GetInputSetting::<Impl, OFFSET>, SetInputSetting::<Impl, OFFSET>)
    }
}
pub trait IWMWriterAdvanced3Impl: Sized + IWMWriterAdvanced2Impl + IWMWriterAdvancedImpl {
    fn GetStatisticsEx();
    fn SetNonBlocking();
}
impl ::windows::core::RuntimeName for IWMWriterAdvanced3 {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMWriterAdvanced3";
}
impl IWMWriterAdvanced3Vtbl {
    pub const fn new<Impl: IWMWriterAdvanced3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMWriterAdvanced3Vtbl {
        unsafe extern "system" fn GetStatisticsEx<Impl: IWMWriterAdvanced3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pstats: *mut WM_WRITER_STATISTICS_EX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStatisticsEx(wstreamnum, ::core::mem::transmute_copy(&pstats)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNonBlocking<Impl: IWMWriterAdvanced3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetNonBlocking() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMWriterAdvanced3>, base.5, GetStatisticsEx::<Impl, OFFSET>, SetNonBlocking::<Impl, OFFSET>)
    }
}
pub trait IWMWriterFileSinkImpl: Sized + IWMWriterSinkImpl {
    fn Open();
}
impl ::windows::core::RuntimeName for IWMWriterFileSink {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMWriterFileSink";
}
impl IWMWriterFileSinkVtbl {
    pub const fn new<Impl: IWMWriterFileSinkImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMWriterFileSinkVtbl {
        unsafe extern "system" fn Open<Impl: IWMWriterFileSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Open(&*(&pwszfilename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMWriterFileSink>, base.5, Open::<Impl, OFFSET>)
    }
}
pub trait IWMWriterFileSink2Impl: Sized + IWMWriterFileSinkImpl + IWMWriterSinkImpl {
    fn Start();
    fn Stop();
    fn IsStopped();
    fn GetFileDuration();
    fn GetFileSize();
    fn Close();
    fn IsClosed();
}
impl ::windows::core::RuntimeName for IWMWriterFileSink2 {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMWriterFileSink2";
}
impl IWMWriterFileSink2Vtbl {
    pub const fn new<Impl: IWMWriterFileSink2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMWriterFileSink2Vtbl {
        unsafe extern "system" fn Start<Impl: IWMWriterFileSink2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cnsstarttime: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Start(cnsstarttime) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stop<Impl: IWMWriterFileSink2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cnsstoptime: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Stop(cnsstoptime) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStopped<Impl: IWMWriterFileSink2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfstopped: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsStopped(::core::mem::transmute_copy(&pfstopped)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileDuration<Impl: IWMWriterFileSink2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcnsduration: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFileDuration(::core::mem::transmute_copy(&pcnsduration)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileSize<Impl: IWMWriterFileSink2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcbfile: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFileSize(::core::mem::transmute_copy(&pcbfile)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IWMWriterFileSink2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Close() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsClosed<Impl: IWMWriterFileSink2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfclosed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsClosed(::core::mem::transmute_copy(&pfclosed)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMWriterFileSink2>, base.5, Start::<Impl, OFFSET>, Stop::<Impl, OFFSET>, IsStopped::<Impl, OFFSET>, GetFileDuration::<Impl, OFFSET>, GetFileSize::<Impl, OFFSET>, Close::<Impl, OFFSET>, IsClosed::<Impl, OFFSET>)
    }
}
pub trait IWMWriterFileSink3Impl: Sized + IWMWriterFileSink2Impl + IWMWriterFileSinkImpl + IWMWriterSinkImpl {
    fn SetAutoIndexing();
    fn GetAutoIndexing();
    fn SetControlStream();
    fn GetMode();
    fn OnDataUnitEx();
    fn SetUnbufferedIO();
    fn GetUnbufferedIO();
    fn CompleteOperations();
}
impl ::windows::core::RuntimeName for IWMWriterFileSink3 {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMWriterFileSink3";
}
impl IWMWriterFileSink3Vtbl {
    pub const fn new<Impl: IWMWriterFileSink3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMWriterFileSink3Vtbl {
        unsafe extern "system" fn SetAutoIndexing<Impl: IWMWriterFileSink3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fdoautoindexing: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAutoIndexing(&*(&fdoautoindexing as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAutoIndexing<Impl: IWMWriterFileSink3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfautoindexing: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAutoIndexing(::core::mem::transmute_copy(&pfautoindexing)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetControlStream<Impl: IWMWriterFileSink3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, fshouldcontrolstartandstop: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetControlStream(wstreamnumber, &*(&fshouldcontrolstartandstop as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMode<Impl: IWMWriterFileSink3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwfilesinkmode: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMode(::core::mem::transmute_copy(&pdwfilesinkmode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnDataUnitEx<Impl: IWMWriterFileSink3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfilesinkdataunit: *const WMT_FILESINK_DATA_UNIT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnDataUnitEx(&*(&pfilesinkdataunit as *const <WMT_FILESINK_DATA_UNIT as ::windows::core::Abi>::Abi as *const <WMT_FILESINK_DATA_UNIT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnbufferedIO<Impl: IWMWriterFileSink3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, funbufferedio: super::super::Foundation::BOOL, frestrictmemusage: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetUnbufferedIO(&*(&funbufferedio as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), &*(&frestrictmemusage as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUnbufferedIO<Impl: IWMWriterFileSink3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfunbufferedio: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetUnbufferedIO(::core::mem::transmute_copy(&pfunbufferedio)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompleteOperations<Impl: IWMWriterFileSink3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CompleteOperations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMWriterFileSink3>, base.5, SetAutoIndexing::<Impl, OFFSET>, GetAutoIndexing::<Impl, OFFSET>, SetControlStream::<Impl, OFFSET>, GetMode::<Impl, OFFSET>, OnDataUnitEx::<Impl, OFFSET>, SetUnbufferedIO::<Impl, OFFSET>, GetUnbufferedIO::<Impl, OFFSET>, CompleteOperations::<Impl, OFFSET>)
    }
}
pub trait IWMWriterNetworkSinkImpl: Sized + IWMWriterSinkImpl {
    fn SetMaximumClients();
    fn GetMaximumClients();
    fn SetNetworkProtocol();
    fn GetNetworkProtocol();
    fn GetHostURL();
    fn Open();
    fn Disconnect();
    fn Close();
}
impl ::windows::core::RuntimeName for IWMWriterNetworkSink {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMWriterNetworkSink";
}
impl IWMWriterNetworkSinkVtbl {
    pub const fn new<Impl: IWMWriterNetworkSinkImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMWriterNetworkSinkVtbl {
        unsafe extern "system" fn SetMaximumClients<Impl: IWMWriterNetworkSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwmaxclients: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetMaximumClients(dwmaxclients) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaximumClients<Impl: IWMWriterNetworkSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwmaxclients: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMaximumClients(::core::mem::transmute_copy(&pdwmaxclients)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetworkProtocol<Impl: IWMWriterNetworkSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, protocol: WMT_NET_PROTOCOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetNetworkProtocol(protocol) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetworkProtocol<Impl: IWMWriterNetworkSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprotocol: *mut WMT_NET_PROTOCOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNetworkProtocol(::core::mem::transmute_copy(&pprotocol)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHostURL<Impl: IWMWriterNetworkSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, pcchurl: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetHostURL(::core::mem::transmute_copy(&pwszurl), pcchurl) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Open<Impl: IWMWriterNetworkSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwportnum: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Open(pdwportnum) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disconnect<Impl: IWMWriterNetworkSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Disconnect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IWMWriterNetworkSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Close() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMWriterNetworkSink>, base.5, SetMaximumClients::<Impl, OFFSET>, GetMaximumClients::<Impl, OFFSET>, SetNetworkProtocol::<Impl, OFFSET>, GetNetworkProtocol::<Impl, OFFSET>, GetHostURL::<Impl, OFFSET>, Open::<Impl, OFFSET>, Disconnect::<Impl, OFFSET>, Close::<Impl, OFFSET>)
    }
}
pub trait IWMWriterPostViewImpl: Sized {
    fn SetPostViewCallback();
    fn SetReceivePostViewSamples();
    fn GetReceivePostViewSamples();
    fn GetPostViewProps();
    fn SetPostViewProps();
    fn GetPostViewFormatCount();
    fn GetPostViewFormat();
    fn SetAllocateForPostView();
    fn GetAllocateForPostView();
}
impl ::windows::core::RuntimeName for IWMWriterPostView {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMWriterPostView";
}
impl IWMWriterPostViewVtbl {
    pub const fn new<Impl: IWMWriterPostViewImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMWriterPostViewVtbl {
        unsafe extern "system" fn SetPostViewCallback<Impl: IWMWriterPostViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr, pvcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPostViewCallback(&*(&pcallback as *const <IWMWriterPostViewCallback as ::windows::core::Abi>::Abi as *const <IWMWriterPostViewCallback as ::windows::core::DefaultType>::DefaultType), &*(&pvcontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReceivePostViewSamples<Impl: IWMWriterPostViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, freceivepostviewsamples: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetReceivePostViewSamples(wstreamnum, &*(&freceivepostviewsamples as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReceivePostViewSamples<Impl: IWMWriterPostViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pfreceivepostviewsamples: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetReceivePostViewSamples(wstreamnum, ::core::mem::transmute_copy(&pfreceivepostviewsamples)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPostViewProps<Impl: IWMWriterPostViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, ppoutput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPostViewProps(wstreamnumber, ::core::mem::transmute_copy(&ppoutput)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPostViewProps<Impl: IWMWriterPostViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, poutput: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPostViewProps(wstreamnumber, &*(&poutput as *const <IWMMediaProps as ::windows::core::Abi>::Abi as *const <IWMMediaProps as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPostViewFormatCount<Impl: IWMWriterPostViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, pcformats: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPostViewFormatCount(wstreamnumber, ::core::mem::transmute_copy(&pcformats)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPostViewFormat<Impl: IWMWriterPostViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, dwformatnumber: u32, ppprops: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPostViewFormat(wstreamnumber, dwformatnumber, ::core::mem::transmute_copy(&ppprops)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllocateForPostView<Impl: IWMWriterPostViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, fallocate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAllocateForPostView(wstreamnumber, &*(&fallocate as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllocateForPostView<Impl: IWMWriterPostViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAllocateForPostView(wstreamnumber, ::core::mem::transmute_copy(&pfallocate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMWriterPostView>, base.5, SetPostViewCallback::<Impl, OFFSET>, SetReceivePostViewSamples::<Impl, OFFSET>, GetReceivePostViewSamples::<Impl, OFFSET>, GetPostViewProps::<Impl, OFFSET>, SetPostViewProps::<Impl, OFFSET>, GetPostViewFormatCount::<Impl, OFFSET>, GetPostViewFormat::<Impl, OFFSET>, SetAllocateForPostView::<Impl, OFFSET>, GetAllocateForPostView::<Impl, OFFSET>)
    }
}
pub trait IWMWriterPostViewCallbackImpl: Sized + IWMStatusCallbackImpl {
    fn OnPostViewSample();
    fn AllocateForPostView();
}
impl ::windows::core::RuntimeName for IWMWriterPostViewCallback {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMWriterPostViewCallback";
}
impl IWMWriterPostViewCallbackVtbl {
    pub const fn new<Impl: IWMWriterPostViewCallbackImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMWriterPostViewCallbackVtbl {
        unsafe extern "system" fn OnPostViewSample<Impl: IWMWriterPostViewCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnPostViewSample(wstreamnumber, cnssampletime, cnssampleduration, dwflags, &*(&psample as *const <INSSBuffer as ::windows::core::Abi>::Abi as *const <INSSBuffer as ::windows::core::DefaultType>::DefaultType), &*(&pvcontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllocateForPostView<Impl: IWMWriterPostViewCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllocateForPostView(wstreamnum, cbbuffer, ::core::mem::transmute_copy(&ppbuffer), &*(&pvcontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMWriterPostViewCallback>, base.5, OnPostViewSample::<Impl, OFFSET>, AllocateForPostView::<Impl, OFFSET>)
    }
}
pub trait IWMWriterPreprocessImpl: Sized {
    fn GetMaxPreprocessingPasses();
    fn SetNumPreprocessingPasses();
    fn BeginPreprocessingPass();
    fn PreprocessSample();
    fn EndPreprocessingPass();
}
impl ::windows::core::RuntimeName for IWMWriterPreprocess {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMWriterPreprocess";
}
impl IWMWriterPreprocessVtbl {
    pub const fn new<Impl: IWMWriterPreprocessImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMWriterPreprocessVtbl {
        unsafe extern "system" fn GetMaxPreprocessingPasses<Impl: IWMWriterPreprocessImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, dwflags: u32, pdwmaxnumpasses: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMaxPreprocessingPasses(dwinputnum, dwflags, ::core::mem::transmute_copy(&pdwmaxnumpasses)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNumPreprocessingPasses<Impl: IWMWriterPreprocessImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, dwflags: u32, dwnumpasses: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetNumPreprocessingPasses(dwinputnum, dwflags, dwnumpasses) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginPreprocessingPass<Impl: IWMWriterPreprocessImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BeginPreprocessingPass(dwinputnum, dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreprocessSample<Impl: IWMWriterPreprocessImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, cnssampletime: u64, dwflags: u32, psample: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PreprocessSample(dwinputnum, cnssampletime, dwflags, &*(&psample as *const <INSSBuffer as ::windows::core::Abi>::Abi as *const <INSSBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndPreprocessingPass<Impl: IWMWriterPreprocessImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EndPreprocessingPass(dwinputnum, dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMWriterPreprocess>, base.5, GetMaxPreprocessingPasses::<Impl, OFFSET>, SetNumPreprocessingPasses::<Impl, OFFSET>, BeginPreprocessingPass::<Impl, OFFSET>, PreprocessSample::<Impl, OFFSET>, EndPreprocessingPass::<Impl, OFFSET>)
    }
}
pub trait IWMWriterPushSinkImpl: Sized + IWMWriterSinkImpl {
    fn Connect();
    fn Disconnect();
    fn EndSession();
}
impl ::windows::core::RuntimeName for IWMWriterPushSink {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMWriterPushSink";
}
impl IWMWriterPushSinkVtbl {
    pub const fn new<Impl: IWMWriterPushSinkImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMWriterPushSinkVtbl {
        unsafe extern "system" fn Connect<Impl: IWMWriterPushSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, pwsztemplateurl: super::super::Foundation::PWSTR, fautodestroy: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Connect(
                &*(&pwszurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pwsztemplateurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&fautodestroy as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disconnect<Impl: IWMWriterPushSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Disconnect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndSession<Impl: IWMWriterPushSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EndSession() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMWriterPushSink>, base.5, Connect::<Impl, OFFSET>, Disconnect::<Impl, OFFSET>, EndSession::<Impl, OFFSET>)
    }
}
pub trait IWMWriterSinkImpl: Sized {
    fn OnHeader();
    fn IsRealTime();
    fn AllocateDataUnit();
    fn OnDataUnit();
    fn OnEndWriting();
}
impl ::windows::core::RuntimeName for IWMWriterSink {
    const NAME: &'static str = "Windows.Win32.Media.WindowsMediaFormat.IWMWriterSink";
}
impl IWMWriterSinkVtbl {
    pub const fn new<Impl: IWMWriterSinkImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMWriterSinkVtbl {
        unsafe extern "system" fn OnHeader<Impl: IWMWriterSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pheader: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnHeader(&*(&pheader as *const <INSSBuffer as ::windows::core::Abi>::Abi as *const <INSSBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRealTime<Impl: IWMWriterSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfrealtime: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsRealTime(::core::mem::transmute_copy(&pfrealtime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllocateDataUnit<Impl: IWMWriterSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbdataunit: u32, ppdataunit: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllocateDataUnit(cbdataunit, ::core::mem::transmute_copy(&ppdataunit)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnDataUnit<Impl: IWMWriterSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdataunit: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnDataUnit(&*(&pdataunit as *const <INSSBuffer as ::windows::core::Abi>::Abi as *const <INSSBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnEndWriting<Impl: IWMWriterSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnEndWriting() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMWriterSink>, base.5, OnHeader::<Impl, OFFSET>, IsRealTime::<Impl, OFFSET>, AllocateDataUnit::<Impl, OFFSET>, OnDataUnit::<Impl, OFFSET>, OnEndWriting::<Impl, OFFSET>)
    }
}
