pub trait IAccClientDocMgrImpl: Sized {
    fn GetDocuments();
    fn LookupByHWND();
    fn LookupByPoint();
    fn GetFocused();
}
impl ::windows::core::RuntimeName for IAccClientDocMgr {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.IAccClientDocMgr";
}
impl IAccClientDocMgrVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccClientDocMgrImpl, const OFFSET: isize>() -> IAccClientDocMgrVtbl {
        unsafe extern "system" fn GetDocuments<Impl: IAccClientDocMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumunknown: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocuments(::core::mem::transmute_copy(&enumunknown)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupByHWND<Impl: IAccClientDocMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LookupByHWND(&*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupByPoint<Impl: IAccClientDocMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pt: super::super::Foundation::POINT, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LookupByPoint(&*(&pt as *const <super::super::Foundation::POINT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::POINT as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFocused<Impl: IAccClientDocMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFocused(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAccClientDocMgr>, ::windows::core::GetTrustLevel, GetDocuments::<Impl, OFFSET>, LookupByHWND::<Impl, OFFSET>, LookupByPoint::<Impl, OFFSET>, GetFocused::<Impl, OFFSET>)
    }
}
pub trait IAccDictionaryImpl: Sized {
    fn GetLocalizedString();
    fn GetParentTerm();
    fn GetMnemonicString();
    fn LookupMnemonicTerm();
    fn ConvertValueToString();
}
impl ::windows::core::RuntimeName for IAccDictionary {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.IAccDictionary";
}
impl IAccDictionaryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccDictionaryImpl, const OFFSET: isize>() -> IAccDictionaryVtbl {
        unsafe extern "system" fn GetLocalizedString<Impl: IAccDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, term: *const ::windows::core::GUID, lcid: u32, presult: *mut super::super::Foundation::BSTR, plcid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocalizedString(&*(&term as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), lcid, ::core::mem::transmute_copy(&presult), ::core::mem::transmute_copy(&plcid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParentTerm<Impl: IAccDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, term: *const ::windows::core::GUID, pparentterm: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetParentTerm(&*(&term as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pparentterm)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMnemonicString<Impl: IAccDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, term: *const ::windows::core::GUID, presult: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMnemonicString(&*(&term as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&presult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupMnemonicTerm<Impl: IAccDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmnemonic: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pterm: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LookupMnemonicTerm(&*(&bstrmnemonic as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pterm)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertValueToString<Impl: IAccDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, term: *const ::windows::core::GUID, lcid: u32, varvalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pbstrresult: *mut super::super::Foundation::BSTR, plcid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertValueToString(&*(&term as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), lcid, &*(&varvalue as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbstrresult), ::core::mem::transmute_copy(&plcid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAccDictionary>, ::windows::core::GetTrustLevel, GetLocalizedString::<Impl, OFFSET>, GetParentTerm::<Impl, OFFSET>, GetMnemonicString::<Impl, OFFSET>, LookupMnemonicTerm::<Impl, OFFSET>, ConvertValueToString::<Impl, OFFSET>)
    }
}
pub trait IAccServerDocMgrImpl: Sized {
    fn NewDocument();
    fn RevokeDocument();
    fn OnDocumentFocus();
}
impl ::windows::core::RuntimeName for IAccServerDocMgr {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.IAccServerDocMgr";
}
impl IAccServerDocMgrVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccServerDocMgrImpl, const OFFSET: isize>() -> IAccServerDocMgrVtbl {
        unsafe extern "system" fn NewDocument<Impl: IAccServerDocMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NewDocument(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&punk as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RevokeDocument<Impl: IAccServerDocMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RevokeDocument(&*(&punk as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnDocumentFocus<Impl: IAccServerDocMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnDocumentFocus(&*(&punk as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAccServerDocMgr>, ::windows::core::GetTrustLevel, NewDocument::<Impl, OFFSET>, RevokeDocument::<Impl, OFFSET>, OnDocumentFocus::<Impl, OFFSET>)
    }
}
pub trait IAccStoreImpl: Sized {
    fn Register();
    fn Unregister();
    fn GetDocuments();
    fn LookupByHWND();
    fn LookupByPoint();
    fn OnDocumentFocus();
    fn GetFocused();
}
impl ::windows::core::RuntimeName for IAccStore {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.IAccStore";
}
impl IAccStoreVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccStoreImpl, const OFFSET: isize>() -> IAccStoreVtbl {
        unsafe extern "system" fn Register<Impl: IAccStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Register(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&punk as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unregister<Impl: IAccStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unregister(&*(&punk as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocuments<Impl: IAccStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumunknown: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocuments(::core::mem::transmute_copy(&enumunknown)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupByHWND<Impl: IAccStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LookupByHWND(&*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupByPoint<Impl: IAccStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pt: super::super::Foundation::POINT, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LookupByPoint(&*(&pt as *const <super::super::Foundation::POINT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::POINT as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnDocumentFocus<Impl: IAccStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnDocumentFocus(&*(&punk as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFocused<Impl: IAccStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFocused(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAccStore>, ::windows::core::GetTrustLevel, Register::<Impl, OFFSET>, Unregister::<Impl, OFFSET>, GetDocuments::<Impl, OFFSET>, LookupByHWND::<Impl, OFFSET>, LookupByPoint::<Impl, OFFSET>, OnDocumentFocus::<Impl, OFFSET>, GetFocused::<Impl, OFFSET>)
    }
}
pub trait IAnchorImpl: Sized {
    fn SetGravity();
    fn GetGravity();
    fn IsEqual();
    fn Compare();
    fn Shift();
    fn ShiftTo();
    fn ShiftRegion();
    fn SetChangeHistoryMask();
    fn GetChangeHistory();
    fn ClearChangeHistory();
    fn Clone();
}
impl ::windows::core::RuntimeName for IAnchor {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.IAnchor";
}
impl IAnchorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAnchorImpl, const OFFSET: isize>() -> IAnchorVtbl {
        unsafe extern "system" fn SetGravity<Impl: IAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gravity: TsGravity) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetGravity(gravity) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGravity<Impl: IAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgravity: *mut TsGravity) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGravity(::core::mem::transmute_copy(&pgravity)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Impl: IAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pawith: ::windows::core::RawPtr, pfequal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEqual(&*(&pawith as *const <IAnchor as ::windows::core::Abi>::Abi as *const <IAnchor as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfequal)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Compare<Impl: IAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pawith: ::windows::core::RawPtr, plresult: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Compare(&*(&pawith as *const <IAnchor as ::windows::core::Abi>::Abi as *const <IAnchor as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&plresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shift<Impl: IAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, cchreq: i32, pcch: *mut i32, pahaltanchor: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Shift(dwflags, cchreq, ::core::mem::transmute_copy(&pcch), &*(&pahaltanchor as *const <IAnchor as ::windows::core::Abi>::Abi as *const <IAnchor as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShiftTo<Impl: IAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pasite: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShiftTo(&*(&pasite as *const <IAnchor as ::windows::core::Abi>::Abi as *const <IAnchor as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShiftRegion<Impl: IAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, dir: TsShiftDir, pfnoregion: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShiftRegion(dwflags, dir, ::core::mem::transmute_copy(&pfnoregion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChangeHistoryMask<Impl: IAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetChangeHistoryMask(dwmask) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChangeHistory<Impl: IAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwhistory: *mut ANCHOR_CHANGE_HISTORY_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChangeHistory(::core::mem::transmute_copy(&pdwhistory)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearChangeHistory<Impl: IAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClearChangeHistory() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaclone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppaclone)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAnchor>,
            ::windows::core::GetTrustLevel,
            SetGravity::<Impl, OFFSET>,
            GetGravity::<Impl, OFFSET>,
            IsEqual::<Impl, OFFSET>,
            Compare::<Impl, OFFSET>,
            Shift::<Impl, OFFSET>,
            ShiftTo::<Impl, OFFSET>,
            ShiftRegion::<Impl, OFFSET>,
            SetChangeHistoryMask::<Impl, OFFSET>,
            GetChangeHistory::<Impl, OFFSET>,
            ClearChangeHistory::<Impl, OFFSET>,
            Clone::<Impl, OFFSET>,
        )
    }
}
pub trait IClonableWrapperImpl: Sized {
    fn CloneNewWrapper();
}
impl ::windows::core::RuntimeName for IClonableWrapper {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.IClonableWrapper";
}
impl IClonableWrapperVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClonableWrapperImpl, const OFFSET: isize>() -> IClonableWrapperVtbl {
        unsafe extern "system" fn CloneNewWrapper<Impl: IClonableWrapperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CloneNewWrapper(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IClonableWrapper>, ::windows::core::GetTrustLevel, CloneNewWrapper::<Impl, OFFSET>)
    }
}
pub trait ICoCreateLocallyImpl: Sized {
    fn CoCreateLocally();
}
impl ::windows::core::RuntimeName for ICoCreateLocally {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ICoCreateLocally";
}
impl ICoCreateLocallyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoCreateLocallyImpl, const OFFSET: isize>() -> ICoCreateLocallyVtbl {
        unsafe extern "system" fn CoCreateLocally<Impl: ICoCreateLocallyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, dwclscontext: u32, riid: *const ::windows::core::GUID, punk: *mut *mut ::core::ffi::c_void, riidparam: *const ::windows::core::GUID, punkparam: *mut ::core::ffi::c_void, varparam: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CoCreateLocally(
                &*(&rclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                dwclscontext,
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&punk),
                &*(&riidparam as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&punkparam as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&varparam as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICoCreateLocally>, ::windows::core::GetTrustLevel, CoCreateLocally::<Impl, OFFSET>)
    }
}
pub trait ICoCreatedLocallyImpl: Sized {
    fn LocalInit();
}
impl ::windows::core::RuntimeName for ICoCreatedLocally {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ICoCreatedLocally";
}
impl ICoCreatedLocallyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoCreatedLocallyImpl, const OFFSET: isize>() -> ICoCreatedLocallyVtbl {
        unsafe extern "system" fn LocalInit<Impl: ICoCreatedLocallyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punklocalobject: *mut ::core::ffi::c_void, riidparam: *const ::windows::core::GUID, punkparam: *mut ::core::ffi::c_void, varparam: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalInit(
                &*(&punklocalobject as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&riidparam as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&punkparam as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&varparam as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICoCreatedLocally>, ::windows::core::GetTrustLevel, LocalInit::<Impl, OFFSET>)
    }
}
pub trait IDocWrapImpl: Sized {
    fn SetDoc();
    fn GetWrappedDoc();
}
impl ::windows::core::RuntimeName for IDocWrap {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.IDocWrap";
}
impl IDocWrapVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDocWrapImpl, const OFFSET: isize>() -> IDocWrapVtbl {
        unsafe extern "system" fn SetDoc<Impl: IDocWrapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDoc(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&punk as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWrappedDoc<Impl: IDocWrapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWrappedDoc(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDocWrap>, ::windows::core::GetTrustLevel, SetDoc::<Impl, OFFSET>, GetWrappedDoc::<Impl, OFFSET>)
    }
}
pub trait IEnumITfCompositionViewImpl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
impl ::windows::core::RuntimeName for IEnumITfCompositionView {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.IEnumITfCompositionView";
}
impl IEnumITfCompositionViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumITfCompositionViewImpl, const OFFSET: isize>() -> IEnumITfCompositionViewVtbl {
        unsafe extern "system" fn Clone<Impl: IEnumITfCompositionViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumITfCompositionViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, rgcompositionview: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(ulcount, ::core::mem::transmute_copy(&rgcompositionview), ::core::mem::transmute_copy(&pcfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumITfCompositionViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumITfCompositionViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(ulcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumITfCompositionView>, ::windows::core::GetTrustLevel, Clone::<Impl, OFFSET>, Next::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Skip::<Impl, OFFSET>)
    }
}
pub trait IEnumSpeechCommandsImpl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
impl ::windows::core::RuntimeName for IEnumSpeechCommands {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.IEnumSpeechCommands";
}
impl IEnumSpeechCommandsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSpeechCommandsImpl, const OFFSET: isize>() -> IEnumSpeechCommandsVtbl {
        unsafe extern "system" fn Clone<Impl: IEnumSpeechCommandsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumSpeechCommandsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, pspcmds: *mut *mut u16, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(ulcount, ::core::mem::transmute_copy(&pspcmds), ::core::mem::transmute_copy(&pcfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumSpeechCommandsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumSpeechCommandsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(ulcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumSpeechCommands>, ::windows::core::GetTrustLevel, Clone::<Impl, OFFSET>, Next::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Skip::<Impl, OFFSET>)
    }
}
pub trait IEnumTfCandidatesImpl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
impl ::windows::core::RuntimeName for IEnumTfCandidates {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.IEnumTfCandidates";
}
impl IEnumTfCandidatesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTfCandidatesImpl, const OFFSET: isize>() -> IEnumTfCandidatesVtbl {
        unsafe extern "system" fn Clone<Impl: IEnumTfCandidatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumTfCandidatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, ppcand: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(ulcount, ::core::mem::transmute_copy(&ppcand), ::core::mem::transmute_copy(&pcfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumTfCandidatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumTfCandidatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(ulcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumTfCandidates>, ::windows::core::GetTrustLevel, Clone::<Impl, OFFSET>, Next::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Skip::<Impl, OFFSET>)
    }
}
pub trait IEnumTfContextViewsImpl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
impl ::windows::core::RuntimeName for IEnumTfContextViews {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.IEnumTfContextViews";
}
impl IEnumTfContextViewsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTfContextViewsImpl, const OFFSET: isize>() -> IEnumTfContextViewsVtbl {
        unsafe extern "system" fn Clone<Impl: IEnumTfContextViewsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumTfContextViewsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, rgviews: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(ulcount, ::core::mem::transmute_copy(&rgviews), ::core::mem::transmute_copy(&pcfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumTfContextViewsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumTfContextViewsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(ulcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumTfContextViews>, ::windows::core::GetTrustLevel, Clone::<Impl, OFFSET>, Next::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Skip::<Impl, OFFSET>)
    }
}
pub trait IEnumTfContextsImpl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
impl ::windows::core::RuntimeName for IEnumTfContexts {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.IEnumTfContexts";
}
impl IEnumTfContextsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTfContextsImpl, const OFFSET: isize>() -> IEnumTfContextsVtbl {
        unsafe extern "system" fn Clone<Impl: IEnumTfContextsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumTfContextsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, rgcontext: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(ulcount, ::core::mem::transmute_copy(&rgcontext), ::core::mem::transmute_copy(&pcfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumTfContextsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumTfContextsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(ulcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumTfContexts>, ::windows::core::GetTrustLevel, Clone::<Impl, OFFSET>, Next::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Skip::<Impl, OFFSET>)
    }
}
pub trait IEnumTfDisplayAttributeInfoImpl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
impl ::windows::core::RuntimeName for IEnumTfDisplayAttributeInfo {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.IEnumTfDisplayAttributeInfo";
}
impl IEnumTfDisplayAttributeInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTfDisplayAttributeInfoImpl, const OFFSET: isize>() -> IEnumTfDisplayAttributeInfoVtbl {
        unsafe extern "system" fn Clone<Impl: IEnumTfDisplayAttributeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumTfDisplayAttributeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, rginfo: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(ulcount, ::core::mem::transmute_copy(&rginfo), ::core::mem::transmute_copy(&pcfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumTfDisplayAttributeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumTfDisplayAttributeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(ulcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumTfDisplayAttributeInfo>, ::windows::core::GetTrustLevel, Clone::<Impl, OFFSET>, Next::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Skip::<Impl, OFFSET>)
    }
}
pub trait IEnumTfDocumentMgrsImpl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
impl ::windows::core::RuntimeName for IEnumTfDocumentMgrs {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.IEnumTfDocumentMgrs";
}
impl IEnumTfDocumentMgrsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTfDocumentMgrsImpl, const OFFSET: isize>() -> IEnumTfDocumentMgrsVtbl {
        unsafe extern "system" fn Clone<Impl: IEnumTfDocumentMgrsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumTfDocumentMgrsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, rgdocumentmgr: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(ulcount, ::core::mem::transmute_copy(&rgdocumentmgr), ::core::mem::transmute_copy(&pcfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumTfDocumentMgrsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumTfDocumentMgrsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(ulcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumTfDocumentMgrs>, ::windows::core::GetTrustLevel, Clone::<Impl, OFFSET>, Next::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Skip::<Impl, OFFSET>)
    }
}
pub trait IEnumTfFunctionProvidersImpl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
impl ::windows::core::RuntimeName for IEnumTfFunctionProviders {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.IEnumTfFunctionProviders";
}
impl IEnumTfFunctionProvidersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTfFunctionProvidersImpl, const OFFSET: isize>() -> IEnumTfFunctionProvidersVtbl {
        unsafe extern "system" fn Clone<Impl: IEnumTfFunctionProvidersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumTfFunctionProvidersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, ppcmdobj: *mut ::windows::core::RawPtr, pcfetch: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(ulcount, ::core::mem::transmute_copy(&ppcmdobj), ::core::mem::transmute_copy(&pcfetch)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumTfFunctionProvidersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumTfFunctionProvidersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(ulcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumTfFunctionProviders>, ::windows::core::GetTrustLevel, Clone::<Impl, OFFSET>, Next::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Skip::<Impl, OFFSET>)
    }
}
pub trait IEnumTfInputProcessorProfilesImpl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
impl ::windows::core::RuntimeName for IEnumTfInputProcessorProfiles {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.IEnumTfInputProcessorProfiles";
}
impl IEnumTfInputProcessorProfilesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTfInputProcessorProfilesImpl, const OFFSET: isize>() -> IEnumTfInputProcessorProfilesVtbl {
        unsafe extern "system" fn Clone<Impl: IEnumTfInputProcessorProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumTfInputProcessorProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, pprofile: *mut TF_INPUTPROCESSORPROFILE, pcfetch: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(ulcount, ::core::mem::transmute_copy(&pprofile), ::core::mem::transmute_copy(&pcfetch)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumTfInputProcessorProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumTfInputProcessorProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(ulcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumTfInputProcessorProfiles>, ::windows::core::GetTrustLevel, Clone::<Impl, OFFSET>, Next::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Skip::<Impl, OFFSET>)
    }
}
pub trait IEnumTfLangBarItemsImpl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
impl ::windows::core::RuntimeName for IEnumTfLangBarItems {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.IEnumTfLangBarItems";
}
impl IEnumTfLangBarItemsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTfLangBarItemsImpl, const OFFSET: isize>() -> IEnumTfLangBarItemsVtbl {
        unsafe extern "system" fn Clone<Impl: IEnumTfLangBarItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumTfLangBarItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, ppitem: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(ulcount, ::core::mem::transmute_copy(&ppitem), pcfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumTfLangBarItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumTfLangBarItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(ulcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumTfLangBarItems>, ::windows::core::GetTrustLevel, Clone::<Impl, OFFSET>, Next::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Skip::<Impl, OFFSET>)
    }
}
pub trait IEnumTfLanguageProfilesImpl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
impl ::windows::core::RuntimeName for IEnumTfLanguageProfiles {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.IEnumTfLanguageProfiles";
}
impl IEnumTfLanguageProfilesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTfLanguageProfilesImpl, const OFFSET: isize>() -> IEnumTfLanguageProfilesVtbl {
        unsafe extern "system" fn Clone<Impl: IEnumTfLanguageProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumTfLanguageProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, pprofile: *mut TF_LANGUAGEPROFILE, pcfetch: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(ulcount, ::core::mem::transmute_copy(&pprofile), ::core::mem::transmute_copy(&pcfetch)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumTfLanguageProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumTfLanguageProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(ulcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumTfLanguageProfiles>, ::windows::core::GetTrustLevel, Clone::<Impl, OFFSET>, Next::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Skip::<Impl, OFFSET>)
    }
}
pub trait IEnumTfLatticeElementsImpl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
impl ::windows::core::RuntimeName for IEnumTfLatticeElements {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.IEnumTfLatticeElements";
}
impl IEnumTfLatticeElementsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTfLatticeElementsImpl, const OFFSET: isize>() -> IEnumTfLatticeElementsVtbl {
        unsafe extern "system" fn Clone<Impl: IEnumTfLatticeElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumTfLatticeElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, rgselements: *mut TF_LMLATTELEMENT, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(ulcount, ::core::mem::transmute_copy(&rgselements), ::core::mem::transmute_copy(&pcfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumTfLatticeElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumTfLatticeElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(ulcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumTfLatticeElements>, ::windows::core::GetTrustLevel, Clone::<Impl, OFFSET>, Next::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Skip::<Impl, OFFSET>)
    }
}
pub trait IEnumTfPropertiesImpl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
impl ::windows::core::RuntimeName for IEnumTfProperties {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.IEnumTfProperties";
}
impl IEnumTfPropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTfPropertiesImpl, const OFFSET: isize>() -> IEnumTfPropertiesVtbl {
        unsafe extern "system" fn Clone<Impl: IEnumTfPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumTfPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, ppprop: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(ulcount, ::core::mem::transmute_copy(&ppprop), ::core::mem::transmute_copy(&pcfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumTfPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumTfPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(ulcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumTfProperties>, ::windows::core::GetTrustLevel, Clone::<Impl, OFFSET>, Next::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Skip::<Impl, OFFSET>)
    }
}
pub trait IEnumTfPropertyValueImpl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
impl ::windows::core::RuntimeName for IEnumTfPropertyValue {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.IEnumTfPropertyValue";
}
impl IEnumTfPropertyValueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTfPropertyValueImpl, const OFFSET: isize>() -> IEnumTfPropertyValueVtbl {
        unsafe extern "system" fn Clone<Impl: IEnumTfPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumTfPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, rgvalues: *mut TF_PROPERTYVAL, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(ulcount, ::core::mem::transmute_copy(&rgvalues), ::core::mem::transmute_copy(&pcfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumTfPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumTfPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(ulcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumTfPropertyValue>, ::windows::core::GetTrustLevel, Clone::<Impl, OFFSET>, Next::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Skip::<Impl, OFFSET>)
    }
}
pub trait IEnumTfRangesImpl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
impl ::windows::core::RuntimeName for IEnumTfRanges {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.IEnumTfRanges";
}
impl IEnumTfRangesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTfRangesImpl, const OFFSET: isize>() -> IEnumTfRangesVtbl {
        unsafe extern "system" fn Clone<Impl: IEnumTfRangesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumTfRangesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, pprange: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(ulcount, ::core::mem::transmute_copy(&pprange), ::core::mem::transmute_copy(&pcfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumTfRangesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumTfRangesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(ulcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumTfRanges>, ::windows::core::GetTrustLevel, Clone::<Impl, OFFSET>, Next::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Skip::<Impl, OFFSET>)
    }
}
pub trait IEnumTfUIElementsImpl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
impl ::windows::core::RuntimeName for IEnumTfUIElements {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.IEnumTfUIElements";
}
impl IEnumTfUIElementsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTfUIElementsImpl, const OFFSET: isize>() -> IEnumTfUIElementsVtbl {
        unsafe extern "system" fn Clone<Impl: IEnumTfUIElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumTfUIElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, ppelement: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(ulcount, ::core::mem::transmute_copy(&ppelement), ::core::mem::transmute_copy(&pcfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumTfUIElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumTfUIElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(ulcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumTfUIElements>, ::windows::core::GetTrustLevel, Clone::<Impl, OFFSET>, Next::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Skip::<Impl, OFFSET>)
    }
}
pub trait IInternalDocWrapImpl: Sized {
    fn NotifyRevoke();
}
impl ::windows::core::RuntimeName for IInternalDocWrap {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.IInternalDocWrap";
}
impl IInternalDocWrapVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternalDocWrapImpl, const OFFSET: isize>() -> IInternalDocWrapVtbl {
        unsafe extern "system" fn NotifyRevoke<Impl: IInternalDocWrapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotifyRevoke() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInternalDocWrap>, ::windows::core::GetTrustLevel, NotifyRevoke::<Impl, OFFSET>)
    }
}
pub trait ISpeechCommandProviderImpl: Sized {
    fn EnumSpeechCommands();
    fn ProcessCommand();
}
impl ::windows::core::RuntimeName for ISpeechCommandProvider {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ISpeechCommandProvider";
}
impl ISpeechCommandProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechCommandProviderImpl, const OFFSET: isize>() -> ISpeechCommandProviderVtbl {
        unsafe extern "system" fn EnumSpeechCommands<Impl: ISpeechCommandProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumSpeechCommands(langid, ::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessCommand<Impl: ISpeechCommandProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcommand: super::super::Foundation::PWSTR, cch: u32, langid: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessCommand(&*(&pszcommand as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cch, langid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpeechCommandProvider>, ::windows::core::GetTrustLevel, EnumSpeechCommands::<Impl, OFFSET>, ProcessCommand::<Impl, OFFSET>)
    }
}
pub trait ITextStoreACPImpl: Sized {
    fn AdviseSink();
    fn UnadviseSink();
    fn RequestLock();
    fn GetStatus();
    fn QueryInsert();
    fn GetSelection();
    fn SetSelection();
    fn GetText();
    fn SetText();
    fn GetFormattedText();
    fn GetEmbedded();
    fn QueryInsertEmbedded();
    fn InsertEmbedded();
    fn InsertTextAtSelection();
    fn InsertEmbeddedAtSelection();
    fn RequestSupportedAttrs();
    fn RequestAttrsAtPosition();
    fn RequestAttrsTransitioningAtPosition();
    fn FindNextAttrTransition();
    fn RetrieveRequestedAttrs();
    fn GetEndACP();
    fn GetActiveView();
    fn GetACPFromPoint();
    fn GetTextExt();
    fn GetScreenExt();
    fn GetWnd();
}
impl ::windows::core::RuntimeName for ITextStoreACP {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITextStoreACP";
}
impl ITextStoreACPVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextStoreACPImpl, const OFFSET: isize>() -> ITextStoreACPVtbl {
        unsafe extern "system" fn AdviseSink<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void, dwmask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdviseSink(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&punk as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), dwmask) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnadviseSink<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnadviseSink(&*(&punk as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestLock<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlockflags: u32, phrsession: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestLock(dwlockflags, ::core::mem::transmute_copy(&phrsession)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdcs: *mut TS_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus(::core::mem::transmute_copy(&pdcs)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryInsert<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpteststart: i32, acptestend: i32, cch: u32, pacpresultstart: *mut i32, pacpresultend: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryInsert(acpteststart, acptestend, cch, ::core::mem::transmute_copy(&pacpresultstart), ::core::mem::transmute_copy(&pacpresultend)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelection<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ACP, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSelection(ulindex, ulcount, ::core::mem::transmute_copy(&pselection), ::core::mem::transmute_copy(&pcfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelection<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, pselection: *const TS_SELECTION_ACP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSelection(ulcount, &*(&pselection as *const <TS_SELECTION_ACP as ::windows::core::Abi>::Abi as *const <TS_SELECTION_ACP as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetText<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, pchplain: super::super::Foundation::PWSTR, cchplainreq: u32, pcchplainret: *mut u32, prgruninfo: *mut TS_RUNINFO, cruninforeq: u32, pcruninforet: *mut u32, pacpnext: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetText(acpstart, acpend, ::core::mem::transmute_copy(&pchplain), cchplainreq, ::core::mem::transmute_copy(&pcchplainret), ::core::mem::transmute_copy(&prgruninfo), cruninforeq, ::core::mem::transmute_copy(&pcruninforet), ::core::mem::transmute_copy(&pacpnext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetText<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, acpstart: i32, acpend: i32, pchtext: super::super::Foundation::PWSTR, cch: u32, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetText(dwflags, acpstart, acpend, &*(&pchtext as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cch, ::core::mem::transmute_copy(&pchange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormattedText<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, ppdataobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFormattedText(acpstart, acpend, ::core::mem::transmute_copy(&ppdataobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEmbedded<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acppos: i32, rguidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEmbedded(acppos, &*(&rguidservice as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryInsertEmbedded<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidservice: *const ::windows::core::GUID, pformatetc: *const super::super::System::Com::FORMATETC, pfinsertable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryInsertEmbedded(&*(&pguidservice as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&pformatetc as *const <super::super::System::Com::FORMATETC as ::windows::core::Abi>::Abi as *const <super::super::System::Com::FORMATETC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfinsertable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertEmbedded<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, acpstart: i32, acpend: i32, pdataobject: ::windows::core::RawPtr, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InsertEmbedded(dwflags, acpstart, acpend, &*(&pdataobject as *const <super::super::System::Com::IDataObject as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IDataObject as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pchange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertTextAtSelection<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pchtext: super::super::Foundation::PWSTR, cch: u32, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InsertTextAtSelection(dwflags, &*(&pchtext as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cch, ::core::mem::transmute_copy(&pacpstart), ::core::mem::transmute_copy(&pacpend), ::core::mem::transmute_copy(&pchange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertEmbeddedAtSelection<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pdataobject: ::windows::core::RawPtr, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InsertEmbeddedAtSelection(dwflags, &*(&pdataobject as *const <super::super::System::Com::IDataObject as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IDataObject as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pacpstart), ::core::mem::transmute_copy(&pacpend), ::core::mem::transmute_copy(&pchange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestSupportedAttrs<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestSupportedAttrs(dwflags, cfilterattrs, &*(&pafilterattrs as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAttrsAtPosition<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAttrsAtPosition(acppos, cfilterattrs, &*(&pafilterattrs as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAttrsTransitioningAtPosition<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAttrsTransitioningAtPosition(acppos, cfilterattrs, &*(&pafilterattrs as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindNextAttrTransition<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpstart: i32, acphalt: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32, pacpnext: *mut i32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindNextAttrTransition(acpstart, acphalt, cfilterattrs, &*(&pafilterattrs as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwflags, ::core::mem::transmute_copy(&pacpnext), ::core::mem::transmute_copy(&pffound), ::core::mem::transmute_copy(&plfoundoffset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetrieveRequestedAttrs<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetrieveRequestedAttrs(ulcount, ::core::mem::transmute_copy(&paattrvals), ::core::mem::transmute_copy(&pcfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEndACP<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pacp: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEndACP(::core::mem::transmute_copy(&pacp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActiveView<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvcview: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActiveView(::core::mem::transmute_copy(&pvcview)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetACPFromPoint<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32, pacp: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetACPFromPoint(vcview, &*(&ptscreen as *const <super::super::Foundation::POINT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::POINT as ::windows::core::DefaultType>::DefaultType), dwflags, ::core::mem::transmute_copy(&pacp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTextExt<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcview: u32, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTextExt(vcview, acpstart, acpend, ::core::mem::transmute_copy(&prc), ::core::mem::transmute_copy(&pfclipped)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScreenExt<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcview: u32, prc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetScreenExt(vcview, ::core::mem::transmute_copy(&prc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWnd<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcview: u32, phwnd: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWnd(vcview, ::core::mem::transmute_copy(&phwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITextStoreACP>,
            ::windows::core::GetTrustLevel,
            AdviseSink::<Impl, OFFSET>,
            UnadviseSink::<Impl, OFFSET>,
            RequestLock::<Impl, OFFSET>,
            GetStatus::<Impl, OFFSET>,
            QueryInsert::<Impl, OFFSET>,
            GetSelection::<Impl, OFFSET>,
            SetSelection::<Impl, OFFSET>,
            GetText::<Impl, OFFSET>,
            SetText::<Impl, OFFSET>,
            GetFormattedText::<Impl, OFFSET>,
            GetEmbedded::<Impl, OFFSET>,
            QueryInsertEmbedded::<Impl, OFFSET>,
            InsertEmbedded::<Impl, OFFSET>,
            InsertTextAtSelection::<Impl, OFFSET>,
            InsertEmbeddedAtSelection::<Impl, OFFSET>,
            RequestSupportedAttrs::<Impl, OFFSET>,
            RequestAttrsAtPosition::<Impl, OFFSET>,
            RequestAttrsTransitioningAtPosition::<Impl, OFFSET>,
            FindNextAttrTransition::<Impl, OFFSET>,
            RetrieveRequestedAttrs::<Impl, OFFSET>,
            GetEndACP::<Impl, OFFSET>,
            GetActiveView::<Impl, OFFSET>,
            GetACPFromPoint::<Impl, OFFSET>,
            GetTextExt::<Impl, OFFSET>,
            GetScreenExt::<Impl, OFFSET>,
            GetWnd::<Impl, OFFSET>,
        )
    }
}
pub trait ITextStoreACP2Impl: Sized {
    fn AdviseSink();
    fn UnadviseSink();
    fn RequestLock();
    fn GetStatus();
    fn QueryInsert();
    fn GetSelection();
    fn SetSelection();
    fn GetText();
    fn SetText();
    fn GetFormattedText();
    fn GetEmbedded();
    fn QueryInsertEmbedded();
    fn InsertEmbedded();
    fn InsertTextAtSelection();
    fn InsertEmbeddedAtSelection();
    fn RequestSupportedAttrs();
    fn RequestAttrsAtPosition();
    fn RequestAttrsTransitioningAtPosition();
    fn FindNextAttrTransition();
    fn RetrieveRequestedAttrs();
    fn GetEndACP();
    fn GetActiveView();
    fn GetACPFromPoint();
    fn GetTextExt();
    fn GetScreenExt();
}
impl ::windows::core::RuntimeName for ITextStoreACP2 {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITextStoreACP2";
}
impl ITextStoreACP2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextStoreACP2Impl, const OFFSET: isize>() -> ITextStoreACP2Vtbl {
        unsafe extern "system" fn AdviseSink<Impl: ITextStoreACP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void, dwmask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdviseSink(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&punk as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), dwmask) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnadviseSink<Impl: ITextStoreACP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnadviseSink(&*(&punk as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestLock<Impl: ITextStoreACP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlockflags: u32, phrsession: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestLock(dwlockflags, ::core::mem::transmute_copy(&phrsession)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: ITextStoreACP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdcs: *mut TS_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus(::core::mem::transmute_copy(&pdcs)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryInsert<Impl: ITextStoreACP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpteststart: i32, acptestend: i32, cch: u32, pacpresultstart: *mut i32, pacpresultend: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryInsert(acpteststart, acptestend, cch, ::core::mem::transmute_copy(&pacpresultstart), ::core::mem::transmute_copy(&pacpresultend)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelection<Impl: ITextStoreACP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ACP, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSelection(ulindex, ulcount, ::core::mem::transmute_copy(&pselection), ::core::mem::transmute_copy(&pcfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelection<Impl: ITextStoreACP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, pselection: *const TS_SELECTION_ACP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSelection(ulcount, &*(&pselection as *const <TS_SELECTION_ACP as ::windows::core::Abi>::Abi as *const <TS_SELECTION_ACP as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetText<Impl: ITextStoreACP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, pchplain: super::super::Foundation::PWSTR, cchplainreq: u32, pcchplainret: *mut u32, prgruninfo: *mut TS_RUNINFO, cruninforeq: u32, pcruninforet: *mut u32, pacpnext: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetText(acpstart, acpend, ::core::mem::transmute_copy(&pchplain), cchplainreq, ::core::mem::transmute_copy(&pcchplainret), ::core::mem::transmute_copy(&prgruninfo), cruninforeq, ::core::mem::transmute_copy(&pcruninforet), ::core::mem::transmute_copy(&pacpnext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetText<Impl: ITextStoreACP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, acpstart: i32, acpend: i32, pchtext: super::super::Foundation::PWSTR, cch: u32, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetText(dwflags, acpstart, acpend, &*(&pchtext as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cch, ::core::mem::transmute_copy(&pchange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormattedText<Impl: ITextStoreACP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, ppdataobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFormattedText(acpstart, acpend, ::core::mem::transmute_copy(&ppdataobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEmbedded<Impl: ITextStoreACP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acppos: i32, rguidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEmbedded(acppos, &*(&rguidservice as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryInsertEmbedded<Impl: ITextStoreACP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidservice: *const ::windows::core::GUID, pformatetc: *const super::super::System::Com::FORMATETC, pfinsertable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryInsertEmbedded(&*(&pguidservice as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&pformatetc as *const <super::super::System::Com::FORMATETC as ::windows::core::Abi>::Abi as *const <super::super::System::Com::FORMATETC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfinsertable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertEmbedded<Impl: ITextStoreACP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, acpstart: i32, acpend: i32, pdataobject: ::windows::core::RawPtr, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InsertEmbedded(dwflags, acpstart, acpend, &*(&pdataobject as *const <super::super::System::Com::IDataObject as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IDataObject as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pchange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertTextAtSelection<Impl: ITextStoreACP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pchtext: super::super::Foundation::PWSTR, cch: u32, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InsertTextAtSelection(dwflags, &*(&pchtext as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cch, ::core::mem::transmute_copy(&pacpstart), ::core::mem::transmute_copy(&pacpend), ::core::mem::transmute_copy(&pchange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertEmbeddedAtSelection<Impl: ITextStoreACP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pdataobject: ::windows::core::RawPtr, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InsertEmbeddedAtSelection(dwflags, &*(&pdataobject as *const <super::super::System::Com::IDataObject as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IDataObject as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pacpstart), ::core::mem::transmute_copy(&pacpend), ::core::mem::transmute_copy(&pchange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestSupportedAttrs<Impl: ITextStoreACP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestSupportedAttrs(dwflags, cfilterattrs, &*(&pafilterattrs as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAttrsAtPosition<Impl: ITextStoreACP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAttrsAtPosition(acppos, cfilterattrs, &*(&pafilterattrs as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAttrsTransitioningAtPosition<Impl: ITextStoreACP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAttrsTransitioningAtPosition(acppos, cfilterattrs, &*(&pafilterattrs as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindNextAttrTransition<Impl: ITextStoreACP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpstart: i32, acphalt: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32, pacpnext: *mut i32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindNextAttrTransition(acpstart, acphalt, cfilterattrs, &*(&pafilterattrs as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwflags, ::core::mem::transmute_copy(&pacpnext), ::core::mem::transmute_copy(&pffound), ::core::mem::transmute_copy(&plfoundoffset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetrieveRequestedAttrs<Impl: ITextStoreACP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetrieveRequestedAttrs(ulcount, ::core::mem::transmute_copy(&paattrvals), ::core::mem::transmute_copy(&pcfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEndACP<Impl: ITextStoreACP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pacp: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEndACP(::core::mem::transmute_copy(&pacp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActiveView<Impl: ITextStoreACP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvcview: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActiveView(::core::mem::transmute_copy(&pvcview)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetACPFromPoint<Impl: ITextStoreACP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32, pacp: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetACPFromPoint(vcview, &*(&ptscreen as *const <super::super::Foundation::POINT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::POINT as ::windows::core::DefaultType>::DefaultType), dwflags, ::core::mem::transmute_copy(&pacp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTextExt<Impl: ITextStoreACP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcview: u32, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTextExt(vcview, acpstart, acpend, ::core::mem::transmute_copy(&prc), ::core::mem::transmute_copy(&pfclipped)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScreenExt<Impl: ITextStoreACP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcview: u32, prc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetScreenExt(vcview, ::core::mem::transmute_copy(&prc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITextStoreACP2>,
            ::windows::core::GetTrustLevel,
            AdviseSink::<Impl, OFFSET>,
            UnadviseSink::<Impl, OFFSET>,
            RequestLock::<Impl, OFFSET>,
            GetStatus::<Impl, OFFSET>,
            QueryInsert::<Impl, OFFSET>,
            GetSelection::<Impl, OFFSET>,
            SetSelection::<Impl, OFFSET>,
            GetText::<Impl, OFFSET>,
            SetText::<Impl, OFFSET>,
            GetFormattedText::<Impl, OFFSET>,
            GetEmbedded::<Impl, OFFSET>,
            QueryInsertEmbedded::<Impl, OFFSET>,
            InsertEmbedded::<Impl, OFFSET>,
            InsertTextAtSelection::<Impl, OFFSET>,
            InsertEmbeddedAtSelection::<Impl, OFFSET>,
            RequestSupportedAttrs::<Impl, OFFSET>,
            RequestAttrsAtPosition::<Impl, OFFSET>,
            RequestAttrsTransitioningAtPosition::<Impl, OFFSET>,
            FindNextAttrTransition::<Impl, OFFSET>,
            RetrieveRequestedAttrs::<Impl, OFFSET>,
            GetEndACP::<Impl, OFFSET>,
            GetActiveView::<Impl, OFFSET>,
            GetACPFromPoint::<Impl, OFFSET>,
            GetTextExt::<Impl, OFFSET>,
            GetScreenExt::<Impl, OFFSET>,
        )
    }
}
pub trait ITextStoreACPExImpl: Sized {
    fn ScrollToRect();
}
impl ::windows::core::RuntimeName for ITextStoreACPEx {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITextStoreACPEx";
}
impl ITextStoreACPExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextStoreACPExImpl, const OFFSET: isize>() -> ITextStoreACPExVtbl {
        unsafe extern "system" fn ScrollToRect<Impl: ITextStoreACPExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, rc: super::super::Foundation::RECT, dwposition: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScrollToRect(acpstart, acpend, &*(&rc as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType), dwposition) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextStoreACPEx>, ::windows::core::GetTrustLevel, ScrollToRect::<Impl, OFFSET>)
    }
}
pub trait ITextStoreACPServicesImpl: Sized {
    fn Serialize();
    fn Unserialize();
    fn ForceLoadProperty();
    fn CreateRange();
}
impl ::windows::core::RuntimeName for ITextStoreACPServices {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITextStoreACPServices";
}
impl ITextStoreACPServicesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextStoreACPServicesImpl, const OFFSET: isize>() -> ITextStoreACPServicesVtbl {
        unsafe extern "system" fn Serialize<Impl: ITextStoreACPServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprop: ::windows::core::RawPtr, prange: ::windows::core::RawPtr, phdr: *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Serialize(&*(&pprop as *const <ITfProperty as ::windows::core::Abi>::Abi as *const <ITfProperty as ::windows::core::DefaultType>::DefaultType), &*(&prange as *const <ITfRange as ::windows::core::Abi>::Abi as *const <ITfRange as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&phdr), &*(&pstream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unserialize<Impl: ITextStoreACPServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprop: ::windows::core::RawPtr, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: ::windows::core::RawPtr, ploader: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unserialize(
                &*(&pprop as *const <ITfProperty as ::windows::core::Abi>::Abi as *const <ITfProperty as ::windows::core::DefaultType>::DefaultType),
                &*(&phdr as *const <TF_PERSISTENT_PROPERTY_HEADER_ACP as ::windows::core::Abi>::Abi as *const <TF_PERSISTENT_PROPERTY_HEADER_ACP as ::windows::core::DefaultType>::DefaultType),
                &*(&pstream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                &*(&ploader as *const <ITfPersistentPropertyLoaderACP as ::windows::core::Abi>::Abi as *const <ITfPersistentPropertyLoaderACP as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ForceLoadProperty<Impl: ITextStoreACPServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprop: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForceLoadProperty(&*(&pprop as *const <ITfProperty as ::windows::core::Abi>::Abi as *const <ITfProperty as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRange<Impl: ITextStoreACPServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRange(acpstart, acpend, ::core::mem::transmute_copy(&pprange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextStoreACPServices>, ::windows::core::GetTrustLevel, Serialize::<Impl, OFFSET>, Unserialize::<Impl, OFFSET>, ForceLoadProperty::<Impl, OFFSET>, CreateRange::<Impl, OFFSET>)
    }
}
pub trait ITextStoreACPSinkImpl: Sized {
    fn OnTextChange();
    fn OnSelectionChange();
    fn OnLayoutChange();
    fn OnStatusChange();
    fn OnAttrsChange();
    fn OnLockGranted();
    fn OnStartEditTransaction();
    fn OnEndEditTransaction();
}
impl ::windows::core::RuntimeName for ITextStoreACPSink {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITextStoreACPSink";
}
impl ITextStoreACPSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextStoreACPSinkImpl, const OFFSET: isize>() -> ITextStoreACPSinkVtbl {
        unsafe extern "system" fn OnTextChange<Impl: ITextStoreACPSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: TEXT_STORE_TEXT_CHANGE_FLAGS, pchange: *const TS_TEXTCHANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnTextChange(dwflags, &*(&pchange as *const <TS_TEXTCHANGE as ::windows::core::Abi>::Abi as *const <TS_TEXTCHANGE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnSelectionChange<Impl: ITextStoreACPSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnSelectionChange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnLayoutChange<Impl: ITextStoreACPSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcode: TsLayoutCode, vcview: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnLayoutChange(lcode, vcview) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnStatusChange<Impl: ITextStoreACPSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnStatusChange(dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnAttrsChange<Impl: ITextStoreACPSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, cattrs: u32, paattrs: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnAttrsChange(acpstart, acpend, cattrs, &*(&paattrs as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnLockGranted<Impl: ITextStoreACPSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnLockGranted(dwlockflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnStartEditTransaction<Impl: ITextStoreACPSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnStartEditTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnEndEditTransaction<Impl: ITextStoreACPSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnEndEditTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITextStoreACPSink>,
            ::windows::core::GetTrustLevel,
            OnTextChange::<Impl, OFFSET>,
            OnSelectionChange::<Impl, OFFSET>,
            OnLayoutChange::<Impl, OFFSET>,
            OnStatusChange::<Impl, OFFSET>,
            OnAttrsChange::<Impl, OFFSET>,
            OnLockGranted::<Impl, OFFSET>,
            OnStartEditTransaction::<Impl, OFFSET>,
            OnEndEditTransaction::<Impl, OFFSET>,
        )
    }
}
pub trait ITextStoreACPSinkExImpl: Sized + ITextStoreACPSinkImpl {
    fn OnDisconnect();
}
impl ::windows::core::RuntimeName for ITextStoreACPSinkEx {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITextStoreACPSinkEx";
}
impl ITextStoreACPSinkExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextStoreACPSinkExImpl, const OFFSET: isize>() -> ITextStoreACPSinkExVtbl {
        unsafe extern "system" fn OnDisconnect<Impl: ITextStoreACPSinkExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnDisconnect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextStoreACPSinkEx>, ::windows::core::GetTrustLevel, OnDisconnect::<Impl, OFFSET>)
    }
}
pub trait ITextStoreAnchorImpl: Sized {
    fn AdviseSink();
    fn UnadviseSink();
    fn RequestLock();
    fn GetStatus();
    fn QueryInsert();
    fn GetSelection();
    fn SetSelection();
    fn GetText();
    fn SetText();
    fn GetFormattedText();
    fn GetEmbedded();
    fn InsertEmbedded();
    fn RequestSupportedAttrs();
    fn RequestAttrsAtPosition();
    fn RequestAttrsTransitioningAtPosition();
    fn FindNextAttrTransition();
    fn RetrieveRequestedAttrs();
    fn GetStart();
    fn GetEnd();
    fn GetActiveView();
    fn GetAnchorFromPoint();
    fn GetTextExt();
    fn GetScreenExt();
    fn GetWnd();
    fn QueryInsertEmbedded();
    fn InsertTextAtSelection();
    fn InsertEmbeddedAtSelection();
}
impl ::windows::core::RuntimeName for ITextStoreAnchor {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITextStoreAnchor";
}
impl ITextStoreAnchorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextStoreAnchorImpl, const OFFSET: isize>() -> ITextStoreAnchorVtbl {
        unsafe extern "system" fn AdviseSink<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void, dwmask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdviseSink(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&punk as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), dwmask) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnadviseSink<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnadviseSink(&*(&punk as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestLock<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlockflags: u32, phrsession: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestLock(dwlockflags, ::core::mem::transmute_copy(&phrsession)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdcs: *mut TS_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus(::core::mem::transmute_copy(&pdcs)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryInsert<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pateststart: ::windows::core::RawPtr, patestend: ::windows::core::RawPtr, cch: u32, pparesultstart: *mut ::windows::core::RawPtr, pparesultend: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryInsert(&*(&pateststart as *const <IAnchor as ::windows::core::Abi>::Abi as *const <IAnchor as ::windows::core::DefaultType>::DefaultType), &*(&patestend as *const <IAnchor as ::windows::core::Abi>::Abi as *const <IAnchor as ::windows::core::DefaultType>::DefaultType), cch, ::core::mem::transmute_copy(&pparesultstart), ::core::mem::transmute_copy(&pparesultend)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelection<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ANCHOR, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSelection(ulindex, ulcount, ::core::mem::transmute_copy(&pselection), ::core::mem::transmute_copy(&pcfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelection<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, pselection: *const TS_SELECTION_ANCHOR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSelection(ulcount, &*(&pselection as *const <TS_SELECTION_ANCHOR as ::windows::core::Abi>::Abi as *const <TS_SELECTION_ANCHOR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetText<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pastart: ::windows::core::RawPtr, paend: ::windows::core::RawPtr, pchtext: super::super::Foundation::PWSTR, cchreq: u32, pcch: *mut u32, fupdateanchor: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetText(
                dwflags,
                &*(&pastart as *const <IAnchor as ::windows::core::Abi>::Abi as *const <IAnchor as ::windows::core::DefaultType>::DefaultType),
                &*(&paend as *const <IAnchor as ::windows::core::Abi>::Abi as *const <IAnchor as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pchtext),
                cchreq,
                ::core::mem::transmute_copy(&pcch),
                &*(&fupdateanchor as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetText<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pastart: ::windows::core::RawPtr, paend: ::windows::core::RawPtr, pchtext: super::super::Foundation::PWSTR, cch: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetText(dwflags, &*(&pastart as *const <IAnchor as ::windows::core::Abi>::Abi as *const <IAnchor as ::windows::core::DefaultType>::DefaultType), &*(&paend as *const <IAnchor as ::windows::core::Abi>::Abi as *const <IAnchor as ::windows::core::DefaultType>::DefaultType), &*(&pchtext as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cch) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormattedText<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pastart: ::windows::core::RawPtr, paend: ::windows::core::RawPtr, ppdataobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFormattedText(&*(&pastart as *const <IAnchor as ::windows::core::Abi>::Abi as *const <IAnchor as ::windows::core::DefaultType>::DefaultType), &*(&paend as *const <IAnchor as ::windows::core::Abi>::Abi as *const <IAnchor as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppdataobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEmbedded<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, papos: ::windows::core::RawPtr, rguidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEmbedded(
                dwflags,
                &*(&papos as *const <IAnchor as ::windows::core::Abi>::Abi as *const <IAnchor as ::windows::core::DefaultType>::DefaultType),
                &*(&rguidservice as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppunk),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertEmbedded<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pastart: ::windows::core::RawPtr, paend: ::windows::core::RawPtr, pdataobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InsertEmbedded(dwflags, &*(&pastart as *const <IAnchor as ::windows::core::Abi>::Abi as *const <IAnchor as ::windows::core::DefaultType>::DefaultType), &*(&paend as *const <IAnchor as ::windows::core::Abi>::Abi as *const <IAnchor as ::windows::core::DefaultType>::DefaultType), &*(&pdataobject as *const <super::super::System::Com::IDataObject as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IDataObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestSupportedAttrs<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestSupportedAttrs(dwflags, cfilterattrs, &*(&pafilterattrs as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAttrsAtPosition<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, papos: ::windows::core::RawPtr, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAttrsAtPosition(&*(&papos as *const <IAnchor as ::windows::core::Abi>::Abi as *const <IAnchor as ::windows::core::DefaultType>::DefaultType), cfilterattrs, &*(&pafilterattrs as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAttrsTransitioningAtPosition<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, papos: ::windows::core::RawPtr, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAttrsTransitioningAtPosition(&*(&papos as *const <IAnchor as ::windows::core::Abi>::Abi as *const <IAnchor as ::windows::core::DefaultType>::DefaultType), cfilterattrs, &*(&pafilterattrs as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindNextAttrTransition<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pastart: ::windows::core::RawPtr, pahalt: ::windows::core::RawPtr, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindNextAttrTransition(
                &*(&pastart as *const <IAnchor as ::windows::core::Abi>::Abi as *const <IAnchor as ::windows::core::DefaultType>::DefaultType),
                &*(&pahalt as *const <IAnchor as ::windows::core::Abi>::Abi as *const <IAnchor as ::windows::core::DefaultType>::DefaultType),
                cfilterattrs,
                &*(&pafilterattrs as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                dwflags,
                ::core::mem::transmute_copy(&pffound),
                ::core::mem::transmute_copy(&plfoundoffset),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetrieveRequestedAttrs<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetrieveRequestedAttrs(ulcount, ::core::mem::transmute_copy(&paattrvals), ::core::mem::transmute_copy(&pcfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStart<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppastart: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStart(::core::mem::transmute_copy(&ppastart)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnd<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaend: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnd(::core::mem::transmute_copy(&ppaend)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActiveView<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvcview: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActiveView(::core::mem::transmute_copy(&pvcview)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAnchorFromPoint<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32, ppasite: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAnchorFromPoint(vcview, &*(&ptscreen as *const <super::super::Foundation::POINT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::POINT as ::windows::core::DefaultType>::DefaultType), dwflags, ::core::mem::transmute_copy(&ppasite)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTextExt<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcview: u32, pastart: ::windows::core::RawPtr, paend: ::windows::core::RawPtr, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTextExt(vcview, &*(&pastart as *const <IAnchor as ::windows::core::Abi>::Abi as *const <IAnchor as ::windows::core::DefaultType>::DefaultType), &*(&paend as *const <IAnchor as ::windows::core::Abi>::Abi as *const <IAnchor as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&prc), ::core::mem::transmute_copy(&pfclipped)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScreenExt<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcview: u32, prc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetScreenExt(vcview, ::core::mem::transmute_copy(&prc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWnd<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcview: u32, phwnd: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWnd(vcview, ::core::mem::transmute_copy(&phwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryInsertEmbedded<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidservice: *const ::windows::core::GUID, pformatetc: *const super::super::System::Com::FORMATETC, pfinsertable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryInsertEmbedded(&*(&pguidservice as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&pformatetc as *const <super::super::System::Com::FORMATETC as ::windows::core::Abi>::Abi as *const <super::super::System::Com::FORMATETC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfinsertable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertTextAtSelection<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pchtext: super::super::Foundation::PWSTR, cch: u32, ppastart: *mut ::windows::core::RawPtr, ppaend: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InsertTextAtSelection(dwflags, &*(&pchtext as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cch, ::core::mem::transmute_copy(&ppastart), ::core::mem::transmute_copy(&ppaend)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertEmbeddedAtSelection<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pdataobject: ::windows::core::RawPtr, ppastart: *mut ::windows::core::RawPtr, ppaend: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InsertEmbeddedAtSelection(dwflags, &*(&pdataobject as *const <super::super::System::Com::IDataObject as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IDataObject as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppastart), ::core::mem::transmute_copy(&ppaend)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITextStoreAnchor>,
            ::windows::core::GetTrustLevel,
            AdviseSink::<Impl, OFFSET>,
            UnadviseSink::<Impl, OFFSET>,
            RequestLock::<Impl, OFFSET>,
            GetStatus::<Impl, OFFSET>,
            QueryInsert::<Impl, OFFSET>,
            GetSelection::<Impl, OFFSET>,
            SetSelection::<Impl, OFFSET>,
            GetText::<Impl, OFFSET>,
            SetText::<Impl, OFFSET>,
            GetFormattedText::<Impl, OFFSET>,
            GetEmbedded::<Impl, OFFSET>,
            InsertEmbedded::<Impl, OFFSET>,
            RequestSupportedAttrs::<Impl, OFFSET>,
            RequestAttrsAtPosition::<Impl, OFFSET>,
            RequestAttrsTransitioningAtPosition::<Impl, OFFSET>,
            FindNextAttrTransition::<Impl, OFFSET>,
            RetrieveRequestedAttrs::<Impl, OFFSET>,
            GetStart::<Impl, OFFSET>,
            GetEnd::<Impl, OFFSET>,
            GetActiveView::<Impl, OFFSET>,
            GetAnchorFromPoint::<Impl, OFFSET>,
            GetTextExt::<Impl, OFFSET>,
            GetScreenExt::<Impl, OFFSET>,
            GetWnd::<Impl, OFFSET>,
            QueryInsertEmbedded::<Impl, OFFSET>,
            InsertTextAtSelection::<Impl, OFFSET>,
            InsertEmbeddedAtSelection::<Impl, OFFSET>,
        )
    }
}
pub trait ITextStoreAnchorExImpl: Sized {
    fn ScrollToRect();
}
impl ::windows::core::RuntimeName for ITextStoreAnchorEx {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITextStoreAnchorEx";
}
impl ITextStoreAnchorExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextStoreAnchorExImpl, const OFFSET: isize>() -> ITextStoreAnchorExVtbl {
        unsafe extern "system" fn ScrollToRect<Impl: ITextStoreAnchorExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstart: ::windows::core::RawPtr, pend: ::windows::core::RawPtr, rc: super::super::Foundation::RECT, dwposition: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScrollToRect(&*(&pstart as *const <IAnchor as ::windows::core::Abi>::Abi as *const <IAnchor as ::windows::core::DefaultType>::DefaultType), &*(&pend as *const <IAnchor as ::windows::core::Abi>::Abi as *const <IAnchor as ::windows::core::DefaultType>::DefaultType), &*(&rc as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType), dwposition) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextStoreAnchorEx>, ::windows::core::GetTrustLevel, ScrollToRect::<Impl, OFFSET>)
    }
}
pub trait ITextStoreAnchorSinkImpl: Sized {
    fn OnTextChange();
    fn OnSelectionChange();
    fn OnLayoutChange();
    fn OnStatusChange();
    fn OnAttrsChange();
    fn OnLockGranted();
    fn OnStartEditTransaction();
    fn OnEndEditTransaction();
}
impl ::windows::core::RuntimeName for ITextStoreAnchorSink {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITextStoreAnchorSink";
}
impl ITextStoreAnchorSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextStoreAnchorSinkImpl, const OFFSET: isize>() -> ITextStoreAnchorSinkVtbl {
        unsafe extern "system" fn OnTextChange<Impl: ITextStoreAnchorSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: TEXT_STORE_CHANGE_FLAGS, pastart: ::windows::core::RawPtr, paend: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnTextChange(dwflags, &*(&pastart as *const <IAnchor as ::windows::core::Abi>::Abi as *const <IAnchor as ::windows::core::DefaultType>::DefaultType), &*(&paend as *const <IAnchor as ::windows::core::Abi>::Abi as *const <IAnchor as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnSelectionChange<Impl: ITextStoreAnchorSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnSelectionChange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnLayoutChange<Impl: ITextStoreAnchorSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcode: TsLayoutCode, vcview: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnLayoutChange(lcode, vcview) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnStatusChange<Impl: ITextStoreAnchorSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnStatusChange(dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnAttrsChange<Impl: ITextStoreAnchorSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pastart: ::windows::core::RawPtr, paend: ::windows::core::RawPtr, cattrs: u32, paattrs: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnAttrsChange(&*(&pastart as *const <IAnchor as ::windows::core::Abi>::Abi as *const <IAnchor as ::windows::core::DefaultType>::DefaultType), &*(&paend as *const <IAnchor as ::windows::core::Abi>::Abi as *const <IAnchor as ::windows::core::DefaultType>::DefaultType), cattrs, &*(&paattrs as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnLockGranted<Impl: ITextStoreAnchorSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnLockGranted(dwlockflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnStartEditTransaction<Impl: ITextStoreAnchorSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnStartEditTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnEndEditTransaction<Impl: ITextStoreAnchorSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnEndEditTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITextStoreAnchorSink>,
            ::windows::core::GetTrustLevel,
            OnTextChange::<Impl, OFFSET>,
            OnSelectionChange::<Impl, OFFSET>,
            OnLayoutChange::<Impl, OFFSET>,
            OnStatusChange::<Impl, OFFSET>,
            OnAttrsChange::<Impl, OFFSET>,
            OnLockGranted::<Impl, OFFSET>,
            OnStartEditTransaction::<Impl, OFFSET>,
            OnEndEditTransaction::<Impl, OFFSET>,
        )
    }
}
pub trait ITextStoreSinkAnchorExImpl: Sized + ITextStoreAnchorSinkImpl {
    fn OnDisconnect();
}
impl ::windows::core::RuntimeName for ITextStoreSinkAnchorEx {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITextStoreSinkAnchorEx";
}
impl ITextStoreSinkAnchorExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextStoreSinkAnchorExImpl, const OFFSET: isize>() -> ITextStoreSinkAnchorExVtbl {
        unsafe extern "system" fn OnDisconnect<Impl: ITextStoreSinkAnchorExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnDisconnect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextStoreSinkAnchorEx>, ::windows::core::GetTrustLevel, OnDisconnect::<Impl, OFFSET>)
    }
}
pub trait ITfActiveLanguageProfileNotifySinkImpl: Sized {
    fn OnActivated();
}
impl ::windows::core::RuntimeName for ITfActiveLanguageProfileNotifySink {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfActiveLanguageProfileNotifySink";
}
impl ITfActiveLanguageProfileNotifySinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfActiveLanguageProfileNotifySinkImpl, const OFFSET: isize>() -> ITfActiveLanguageProfileNotifySinkVtbl {
        unsafe extern "system" fn OnActivated<Impl: ITfActiveLanguageProfileNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, guidprofile: *const ::windows::core::GUID, factivated: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnActivated(
                &*(&clsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&guidprofile as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&factivated as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfActiveLanguageProfileNotifySink>, ::windows::core::GetTrustLevel, OnActivated::<Impl, OFFSET>)
    }
}
pub trait ITfCandidateListImpl: Sized {
    fn EnumCandidates();
    fn GetCandidate();
    fn GetCandidateNum();
    fn SetResult();
}
impl ::windows::core::RuntimeName for ITfCandidateList {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfCandidateList";
}
impl ITfCandidateListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfCandidateListImpl, const OFFSET: isize>() -> ITfCandidateListVtbl {
        unsafe extern "system" fn EnumCandidates<Impl: ITfCandidateListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumCandidates(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCandidate<Impl: ITfCandidateListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, ppcand: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCandidate(nindex, ::core::mem::transmute_copy(&ppcand)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCandidateNum<Impl: ITfCandidateListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pncnt: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCandidateNum(::core::mem::transmute_copy(&pncnt)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetResult<Impl: ITfCandidateListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, imcr: TfCandidateResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetResult(nindex, imcr) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfCandidateList>, ::windows::core::GetTrustLevel, EnumCandidates::<Impl, OFFSET>, GetCandidate::<Impl, OFFSET>, GetCandidateNum::<Impl, OFFSET>, SetResult::<Impl, OFFSET>)
    }
}
pub trait ITfCandidateListUIElementImpl: Sized + ITfUIElementImpl {
    fn GetUpdatedFlags();
    fn GetDocumentMgr();
    fn GetCount();
    fn GetSelection();
    fn GetString();
    fn GetPageIndex();
    fn SetPageIndex();
    fn GetCurrentPage();
}
impl ::windows::core::RuntimeName for ITfCandidateListUIElement {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfCandidateListUIElement";
}
impl ITfCandidateListUIElementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfCandidateListUIElementImpl, const OFFSET: isize>() -> ITfCandidateListUIElementVtbl {
        unsafe extern "system" fn GetUpdatedFlags<Impl: ITfCandidateListUIElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUpdatedFlags(::core::mem::transmute_copy(&pdwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocumentMgr<Impl: ITfCandidateListUIElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdim: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocumentMgr(::core::mem::transmute_copy(&ppdim)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Impl: ITfCandidateListUIElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&pucount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelection<Impl: ITfCandidateListUIElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSelection(::core::mem::transmute_copy(&puindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetString<Impl: ITfCandidateListUIElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, pstr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetString(uindex, ::core::mem::transmute_copy(&pstr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPageIndex<Impl: ITfCandidateListUIElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pindex: *mut u32, usize: u32, pupagecnt: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPageIndex(::core::mem::transmute_copy(&pindex), usize, ::core::mem::transmute_copy(&pupagecnt)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPageIndex<Impl: ITfCandidateListUIElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pindex: *const u32, upagecnt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPageIndex(pindex, upagecnt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentPage<Impl: ITfCandidateListUIElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pupage: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentPage(::core::mem::transmute_copy(&pupage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITfCandidateListUIElement>,
            ::windows::core::GetTrustLevel,
            GetUpdatedFlags::<Impl, OFFSET>,
            GetDocumentMgr::<Impl, OFFSET>,
            GetCount::<Impl, OFFSET>,
            GetSelection::<Impl, OFFSET>,
            GetString::<Impl, OFFSET>,
            GetPageIndex::<Impl, OFFSET>,
            SetPageIndex::<Impl, OFFSET>,
            GetCurrentPage::<Impl, OFFSET>,
        )
    }
}
pub trait ITfCandidateListUIElementBehaviorImpl: Sized + ITfCandidateListUIElementImpl + ITfUIElementImpl {
    fn SetSelection();
    fn Finalize();
    fn Abort();
}
impl ::windows::core::RuntimeName for ITfCandidateListUIElementBehavior {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfCandidateListUIElementBehavior";
}
impl ITfCandidateListUIElementBehaviorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfCandidateListUIElementBehaviorImpl, const OFFSET: isize>() -> ITfCandidateListUIElementBehaviorVtbl {
        unsafe extern "system" fn SetSelection<Impl: ITfCandidateListUIElementBehaviorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSelection(nindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finalize<Impl: ITfCandidateListUIElementBehaviorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Finalize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Abort<Impl: ITfCandidateListUIElementBehaviorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Abort() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfCandidateListUIElementBehavior>, ::windows::core::GetTrustLevel, SetSelection::<Impl, OFFSET>, Finalize::<Impl, OFFSET>, Abort::<Impl, OFFSET>)
    }
}
pub trait ITfCandidateStringImpl: Sized {
    fn GetString();
    fn GetIndex();
}
impl ::windows::core::RuntimeName for ITfCandidateString {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfCandidateString";
}
impl ITfCandidateStringVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfCandidateStringImpl, const OFFSET: isize>() -> ITfCandidateStringVtbl {
        unsafe extern "system" fn GetString<Impl: ITfCandidateStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetString(::core::mem::transmute_copy(&pbstr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIndex<Impl: ITfCandidateStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIndex(::core::mem::transmute_copy(&pnindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfCandidateString>, ::windows::core::GetTrustLevel, GetString::<Impl, OFFSET>, GetIndex::<Impl, OFFSET>)
    }
}
pub trait ITfCategoryMgrImpl: Sized {
    fn RegisterCategory();
    fn UnregisterCategory();
    fn EnumCategoriesInItem();
    fn EnumItemsInCategory();
    fn FindClosestCategory();
    fn RegisterGUIDDescription();
    fn UnregisterGUIDDescription();
    fn GetGUIDDescription();
    fn RegisterGUIDDWORD();
    fn UnregisterGUIDDWORD();
    fn GetGUIDDWORD();
    fn RegisterGUID();
    fn GetGUID();
    fn IsEqualTfGuidAtom();
}
impl ::windows::core::RuntimeName for ITfCategoryMgr {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfCategoryMgr";
}
impl ITfCategoryMgrVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfCategoryMgrImpl, const OFFSET: isize>() -> ITfCategoryMgrVtbl {
        unsafe extern "system" fn RegisterCategory<Impl: ITfCategoryMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, rcatid: *const ::windows::core::GUID, rguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterCategory(&*(&rclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&rcatid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&rguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterCategory<Impl: ITfCategoryMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, rcatid: *const ::windows::core::GUID, rguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnregisterCategory(&*(&rclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&rcatid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&rguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumCategoriesInItem<Impl: ITfCategoryMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumCategoriesInItem(&*(&rguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumItemsInCategory<Impl: ITfCategoryMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rcatid: *const ::windows::core::GUID, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumItemsInCategory(&*(&rcatid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindClosestCategory<Impl: ITfCategoryMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, pcatid: *mut ::windows::core::GUID, ppcatidlist: *const *const ::windows::core::GUID, ulcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindClosestCategory(&*(&rguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcatid), &*(&ppcatidlist as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ulcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterGUIDDescription<Impl: ITfCategoryMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, rguid: *const ::windows::core::GUID, pchdesc: super::super::Foundation::PWSTR, cch: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterGUIDDescription(
                &*(&rclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&rguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&pchdesc as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cch,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterGUIDDescription<Impl: ITfCategoryMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, rguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnregisterGUIDDescription(&*(&rclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&rguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGUIDDescription<Impl: ITfCategoryMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, pbstrdesc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGUIDDescription(&*(&rguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbstrdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterGUIDDWORD<Impl: ITfCategoryMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, rguid: *const ::windows::core::GUID, dw: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterGUIDDWORD(&*(&rclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&rguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dw) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterGUIDDWORD<Impl: ITfCategoryMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, rguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnregisterGUIDDWORD(&*(&rclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&rguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGUIDDWORD<Impl: ITfCategoryMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, pdw: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGUIDDWORD(&*(&rguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdw)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterGUID<Impl: ITfCategoryMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, pguidatom: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterGUID(&*(&rguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pguidatom)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGUID<Impl: ITfCategoryMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidatom: u32, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGUID(guidatom, ::core::mem::transmute_copy(&pguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqualTfGuidAtom<Impl: ITfCategoryMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidatom: u32, rguid: *const ::windows::core::GUID, pfequal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEqualTfGuidAtom(guidatom, &*(&rguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfequal)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITfCategoryMgr>,
            ::windows::core::GetTrustLevel,
            RegisterCategory::<Impl, OFFSET>,
            UnregisterCategory::<Impl, OFFSET>,
            EnumCategoriesInItem::<Impl, OFFSET>,
            EnumItemsInCategory::<Impl, OFFSET>,
            FindClosestCategory::<Impl, OFFSET>,
            RegisterGUIDDescription::<Impl, OFFSET>,
            UnregisterGUIDDescription::<Impl, OFFSET>,
            GetGUIDDescription::<Impl, OFFSET>,
            RegisterGUIDDWORD::<Impl, OFFSET>,
            UnregisterGUIDDWORD::<Impl, OFFSET>,
            GetGUIDDWORD::<Impl, OFFSET>,
            RegisterGUID::<Impl, OFFSET>,
            GetGUID::<Impl, OFFSET>,
            IsEqualTfGuidAtom::<Impl, OFFSET>,
        )
    }
}
pub trait ITfCleanupContextDurationSinkImpl: Sized {
    fn OnStartCleanupContext();
    fn OnEndCleanupContext();
}
impl ::windows::core::RuntimeName for ITfCleanupContextDurationSink {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfCleanupContextDurationSink";
}
impl ITfCleanupContextDurationSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfCleanupContextDurationSinkImpl, const OFFSET: isize>() -> ITfCleanupContextDurationSinkVtbl {
        unsafe extern "system" fn OnStartCleanupContext<Impl: ITfCleanupContextDurationSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnStartCleanupContext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnEndCleanupContext<Impl: ITfCleanupContextDurationSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnEndCleanupContext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfCleanupContextDurationSink>, ::windows::core::GetTrustLevel, OnStartCleanupContext::<Impl, OFFSET>, OnEndCleanupContext::<Impl, OFFSET>)
    }
}
pub trait ITfCleanupContextSinkImpl: Sized {
    fn OnCleanupContext();
}
impl ::windows::core::RuntimeName for ITfCleanupContextSink {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfCleanupContextSink";
}
impl ITfCleanupContextSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfCleanupContextSinkImpl, const OFFSET: isize>() -> ITfCleanupContextSinkVtbl {
        unsafe extern "system" fn OnCleanupContext<Impl: ITfCleanupContextSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ecwrite: u32, pic: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnCleanupContext(ecwrite, &*(&pic as *const <ITfContext as ::windows::core::Abi>::Abi as *const <ITfContext as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfCleanupContextSink>, ::windows::core::GetTrustLevel, OnCleanupContext::<Impl, OFFSET>)
    }
}
pub trait ITfClientIdImpl: Sized {
    fn GetClientId();
}
impl ::windows::core::RuntimeName for ITfClientId {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfClientId";
}
impl ITfClientIdVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfClientIdImpl, const OFFSET: isize>() -> ITfClientIdVtbl {
        unsafe extern "system" fn GetClientId<Impl: ITfClientIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, ptid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClientId(&*(&rclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ptid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfClientId>, ::windows::core::GetTrustLevel, GetClientId::<Impl, OFFSET>)
    }
}
pub trait ITfCompartmentImpl: Sized {
    fn SetValue();
    fn GetValue();
}
impl ::windows::core::RuntimeName for ITfCompartment {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfCompartment";
}
impl ITfCompartmentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfCompartmentImpl, const OFFSET: isize>() -> ITfCompartmentVtbl {
        unsafe extern "system" fn SetValue<Impl: ITfCompartmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tid: u32, pvarvalue: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetValue(tid, &*(&pvarvalue as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Impl: ITfCompartmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetValue(::core::mem::transmute_copy(&pvarvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfCompartment>, ::windows::core::GetTrustLevel, SetValue::<Impl, OFFSET>, GetValue::<Impl, OFFSET>)
    }
}
pub trait ITfCompartmentEventSinkImpl: Sized {
    fn OnChange();
}
impl ::windows::core::RuntimeName for ITfCompartmentEventSink {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfCompartmentEventSink";
}
impl ITfCompartmentEventSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfCompartmentEventSinkImpl, const OFFSET: isize>() -> ITfCompartmentEventSinkVtbl {
        unsafe extern "system" fn OnChange<Impl: ITfCompartmentEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnChange(&*(&rguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfCompartmentEventSink>, ::windows::core::GetTrustLevel, OnChange::<Impl, OFFSET>)
    }
}
pub trait ITfCompartmentMgrImpl: Sized {
    fn GetCompartment();
    fn ClearCompartment();
    fn EnumCompartments();
}
impl ::windows::core::RuntimeName for ITfCompartmentMgr {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfCompartmentMgr";
}
impl ITfCompartmentMgrVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfCompartmentMgrImpl, const OFFSET: isize>() -> ITfCompartmentMgrVtbl {
        unsafe extern "system" fn GetCompartment<Impl: ITfCompartmentMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, ppcomp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCompartment(&*(&rguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppcomp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearCompartment<Impl: ITfCompartmentMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tid: u32, rguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClearCompartment(tid, &*(&rguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumCompartments<Impl: ITfCompartmentMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumCompartments(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfCompartmentMgr>, ::windows::core::GetTrustLevel, GetCompartment::<Impl, OFFSET>, ClearCompartment::<Impl, OFFSET>, EnumCompartments::<Impl, OFFSET>)
    }
}
pub trait ITfCompositionImpl: Sized {
    fn GetRange();
    fn ShiftStart();
    fn ShiftEnd();
    fn EndComposition();
}
impl ::windows::core::RuntimeName for ITfComposition {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfComposition";
}
impl ITfCompositionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfCompositionImpl, const OFFSET: isize>() -> ITfCompositionVtbl {
        unsafe extern "system" fn GetRange<Impl: ITfCompositionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRange(::core::mem::transmute_copy(&pprange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShiftStart<Impl: ITfCompositionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ecwrite: u32, pnewstart: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShiftStart(ecwrite, &*(&pnewstart as *const <ITfRange as ::windows::core::Abi>::Abi as *const <ITfRange as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShiftEnd<Impl: ITfCompositionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ecwrite: u32, pnewend: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShiftEnd(ecwrite, &*(&pnewend as *const <ITfRange as ::windows::core::Abi>::Abi as *const <ITfRange as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndComposition<Impl: ITfCompositionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ecwrite: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndComposition(ecwrite) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfComposition>, ::windows::core::GetTrustLevel, GetRange::<Impl, OFFSET>, ShiftStart::<Impl, OFFSET>, ShiftEnd::<Impl, OFFSET>, EndComposition::<Impl, OFFSET>)
    }
}
pub trait ITfCompositionSinkImpl: Sized {
    fn OnCompositionTerminated();
}
impl ::windows::core::RuntimeName for ITfCompositionSink {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfCompositionSink";
}
impl ITfCompositionSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfCompositionSinkImpl, const OFFSET: isize>() -> ITfCompositionSinkVtbl {
        unsafe extern "system" fn OnCompositionTerminated<Impl: ITfCompositionSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ecwrite: u32, pcomposition: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnCompositionTerminated(ecwrite, &*(&pcomposition as *const <ITfComposition as ::windows::core::Abi>::Abi as *const <ITfComposition as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfCompositionSink>, ::windows::core::GetTrustLevel, OnCompositionTerminated::<Impl, OFFSET>)
    }
}
pub trait ITfCompositionViewImpl: Sized {
    fn GetOwnerClsid();
    fn GetRange();
}
impl ::windows::core::RuntimeName for ITfCompositionView {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfCompositionView";
}
impl ITfCompositionViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfCompositionViewImpl, const OFFSET: isize>() -> ITfCompositionViewVtbl {
        unsafe extern "system" fn GetOwnerClsid<Impl: ITfCompositionViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOwnerClsid(::core::mem::transmute_copy(&pclsid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRange<Impl: ITfCompositionViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRange(::core::mem::transmute_copy(&pprange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfCompositionView>, ::windows::core::GetTrustLevel, GetOwnerClsid::<Impl, OFFSET>, GetRange::<Impl, OFFSET>)
    }
}
pub trait ITfConfigureSystemKeystrokeFeedImpl: Sized {
    fn DisableSystemKeystrokeFeed();
    fn EnableSystemKeystrokeFeed();
}
impl ::windows::core::RuntimeName for ITfConfigureSystemKeystrokeFeed {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfConfigureSystemKeystrokeFeed";
}
impl ITfConfigureSystemKeystrokeFeedVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfConfigureSystemKeystrokeFeedImpl, const OFFSET: isize>() -> ITfConfigureSystemKeystrokeFeedVtbl {
        unsafe extern "system" fn DisableSystemKeystrokeFeed<Impl: ITfConfigureSystemKeystrokeFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisableSystemKeystrokeFeed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableSystemKeystrokeFeed<Impl: ITfConfigureSystemKeystrokeFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableSystemKeystrokeFeed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfConfigureSystemKeystrokeFeed>, ::windows::core::GetTrustLevel, DisableSystemKeystrokeFeed::<Impl, OFFSET>, EnableSystemKeystrokeFeed::<Impl, OFFSET>)
    }
}
pub trait ITfContextImpl: Sized {
    fn RequestEditSession();
    fn InWriteSession();
    fn GetSelection();
    fn SetSelection();
    fn GetStart();
    fn GetEnd();
    fn GetActiveView();
    fn EnumViews();
    fn GetStatus();
    fn GetProperty();
    fn GetAppProperty();
    fn TrackProperties();
    fn EnumProperties();
    fn GetDocumentMgr();
    fn CreateRangeBackup();
}
impl ::windows::core::RuntimeName for ITfContext {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfContext";
}
impl ITfContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfContextImpl, const OFFSET: isize>() -> ITfContextVtbl {
        unsafe extern "system" fn RequestEditSession<Impl: ITfContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tid: u32, pes: ::windows::core::RawPtr, dwflags: TF_CONTEXT_EDIT_CONTEXT_FLAGS, phrsession: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestEditSession(tid, &*(&pes as *const <ITfEditSession as ::windows::core::Abi>::Abi as *const <ITfEditSession as ::windows::core::DefaultType>::DefaultType), dwflags, ::core::mem::transmute_copy(&phrsession)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InWriteSession<Impl: ITfContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tid: u32, pfwritesession: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InWriteSession(tid, ::core::mem::transmute_copy(&pfwritesession)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelection<Impl: ITfContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, ulindex: u32, ulcount: u32, pselection: *mut TF_SELECTION, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSelection(ec, ulindex, ulcount, ::core::mem::transmute_copy(&pselection), ::core::mem::transmute_copy(&pcfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelection<Impl: ITfContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, ulcount: u32, pselection: *const TF_SELECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSelection(ec, ulcount, &*(&pselection as *const <TF_SELECTION as ::windows::core::Abi>::Abi as *const <TF_SELECTION as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStart<Impl: ITfContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, ppstart: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStart(ec, ::core::mem::transmute_copy(&ppstart)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnd<Impl: ITfContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, ppend: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnd(ec, ::core::mem::transmute_copy(&ppend)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActiveView<Impl: ITfContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActiveView(::core::mem::transmute_copy(&ppview)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumViews<Impl: ITfContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumViews(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: ITfContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdcs: *mut TS_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus(::core::mem::transmute_copy(&pdcs)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: ITfContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidprop: *const ::windows::core::GUID, ppprop: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(&*(&guidprop as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAppProperty<Impl: ITfContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidprop: *const ::windows::core::GUID, ppprop: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAppProperty(&*(&guidprop as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrackProperties<Impl: ITfContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prgprop: *const *const ::windows::core::GUID, cprop: u32, prgappprop: *const *const ::windows::core::GUID, cappprop: u32, ppproperty: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrackProperties(&*(&prgprop as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), cprop, &*(&prgappprop as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), cappprop, ::core::mem::transmute_copy(&ppproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumProperties<Impl: ITfContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumProperties(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocumentMgr<Impl: ITfContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocumentMgr(::core::mem::transmute_copy(&ppdm)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRangeBackup<Impl: ITfContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, prange: ::windows::core::RawPtr, ppbackup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRangeBackup(ec, &*(&prange as *const <ITfRange as ::windows::core::Abi>::Abi as *const <ITfRange as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppbackup)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITfContext>,
            ::windows::core::GetTrustLevel,
            RequestEditSession::<Impl, OFFSET>,
            InWriteSession::<Impl, OFFSET>,
            GetSelection::<Impl, OFFSET>,
            SetSelection::<Impl, OFFSET>,
            GetStart::<Impl, OFFSET>,
            GetEnd::<Impl, OFFSET>,
            GetActiveView::<Impl, OFFSET>,
            EnumViews::<Impl, OFFSET>,
            GetStatus::<Impl, OFFSET>,
            GetProperty::<Impl, OFFSET>,
            GetAppProperty::<Impl, OFFSET>,
            TrackProperties::<Impl, OFFSET>,
            EnumProperties::<Impl, OFFSET>,
            GetDocumentMgr::<Impl, OFFSET>,
            CreateRangeBackup::<Impl, OFFSET>,
        )
    }
}
pub trait ITfContextCompositionImpl: Sized {
    fn StartComposition();
    fn EnumCompositions();
    fn FindComposition();
    fn TakeOwnership();
}
impl ::windows::core::RuntimeName for ITfContextComposition {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfContextComposition";
}
impl ITfContextCompositionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfContextCompositionImpl, const OFFSET: isize>() -> ITfContextCompositionVtbl {
        unsafe extern "system" fn StartComposition<Impl: ITfContextCompositionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ecwrite: u32, pcompositionrange: ::windows::core::RawPtr, psink: ::windows::core::RawPtr, ppcomposition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartComposition(ecwrite, &*(&pcompositionrange as *const <ITfRange as ::windows::core::Abi>::Abi as *const <ITfRange as ::windows::core::DefaultType>::DefaultType), &*(&psink as *const <ITfCompositionSink as ::windows::core::Abi>::Abi as *const <ITfCompositionSink as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppcomposition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumCompositions<Impl: ITfContextCompositionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumCompositions(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindComposition<Impl: ITfContextCompositionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ecread: u32, ptestrange: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindComposition(ecread, &*(&ptestrange as *const <ITfRange as ::windows::core::Abi>::Abi as *const <ITfRange as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TakeOwnership<Impl: ITfContextCompositionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ecwrite: u32, pcomposition: ::windows::core::RawPtr, psink: ::windows::core::RawPtr, ppcomposition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TakeOwnership(ecwrite, &*(&pcomposition as *const <ITfCompositionView as ::windows::core::Abi>::Abi as *const <ITfCompositionView as ::windows::core::DefaultType>::DefaultType), &*(&psink as *const <ITfCompositionSink as ::windows::core::Abi>::Abi as *const <ITfCompositionSink as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppcomposition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfContextComposition>, ::windows::core::GetTrustLevel, StartComposition::<Impl, OFFSET>, EnumCompositions::<Impl, OFFSET>, FindComposition::<Impl, OFFSET>, TakeOwnership::<Impl, OFFSET>)
    }
}
pub trait ITfContextKeyEventSinkImpl: Sized {
    fn OnKeyDown();
    fn OnKeyUp();
    fn OnTestKeyDown();
    fn OnTestKeyUp();
}
impl ::windows::core::RuntimeName for ITfContextKeyEventSink {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfContextKeyEventSink";
}
impl ITfContextKeyEventSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfContextKeyEventSinkImpl, const OFFSET: isize>() -> ITfContextKeyEventSinkVtbl {
        unsafe extern "system" fn OnKeyDown<Impl: ITfContextKeyEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnKeyDown(&*(&wparam as *const <super::super::Foundation::WPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::WPARAM as ::windows::core::DefaultType>::DefaultType), &*(&lparam as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfeaten)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnKeyUp<Impl: ITfContextKeyEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnKeyUp(&*(&wparam as *const <super::super::Foundation::WPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::WPARAM as ::windows::core::DefaultType>::DefaultType), &*(&lparam as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfeaten)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnTestKeyDown<Impl: ITfContextKeyEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnTestKeyDown(&*(&wparam as *const <super::super::Foundation::WPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::WPARAM as ::windows::core::DefaultType>::DefaultType), &*(&lparam as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfeaten)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnTestKeyUp<Impl: ITfContextKeyEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnTestKeyUp(&*(&wparam as *const <super::super::Foundation::WPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::WPARAM as ::windows::core::DefaultType>::DefaultType), &*(&lparam as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfeaten)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfContextKeyEventSink>, ::windows::core::GetTrustLevel, OnKeyDown::<Impl, OFFSET>, OnKeyUp::<Impl, OFFSET>, OnTestKeyDown::<Impl, OFFSET>, OnTestKeyUp::<Impl, OFFSET>)
    }
}
pub trait ITfContextOwnerImpl: Sized {
    fn GetACPFromPoint();
    fn GetTextExt();
    fn GetScreenExt();
    fn GetStatus();
    fn GetWnd();
    fn GetAttribute();
}
impl ::windows::core::RuntimeName for ITfContextOwner {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfContextOwner";
}
impl ITfContextOwnerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfContextOwnerImpl, const OFFSET: isize>() -> ITfContextOwnerVtbl {
        unsafe extern "system" fn GetACPFromPoint<Impl: ITfContextOwnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptscreen: *const super::super::Foundation::POINT, dwflags: u32, pacp: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetACPFromPoint(&*(&ptscreen as *const <super::super::Foundation::POINT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::POINT as ::windows::core::DefaultType>::DefaultType), dwflags, ::core::mem::transmute_copy(&pacp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTextExt<Impl: ITfContextOwnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTextExt(acpstart, acpend, ::core::mem::transmute_copy(&prc), ::core::mem::transmute_copy(&pfclipped)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScreenExt<Impl: ITfContextOwnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetScreenExt(::core::mem::transmute_copy(&prc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: ITfContextOwnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdcs: *mut TS_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus(::core::mem::transmute_copy(&pdcs)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWnd<Impl: ITfContextOwnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWnd(::core::mem::transmute_copy(&phwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttribute<Impl: ITfContextOwnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidattribute: *const ::windows::core::GUID, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAttribute(&*(&rguidattribute as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvarvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfContextOwner>, ::windows::core::GetTrustLevel, GetACPFromPoint::<Impl, OFFSET>, GetTextExt::<Impl, OFFSET>, GetScreenExt::<Impl, OFFSET>, GetStatus::<Impl, OFFSET>, GetWnd::<Impl, OFFSET>, GetAttribute::<Impl, OFFSET>)
    }
}
pub trait ITfContextOwnerCompositionServicesImpl: Sized + ITfContextCompositionImpl {
    fn TerminateComposition();
}
impl ::windows::core::RuntimeName for ITfContextOwnerCompositionServices {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfContextOwnerCompositionServices";
}
impl ITfContextOwnerCompositionServicesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfContextOwnerCompositionServicesImpl, const OFFSET: isize>() -> ITfContextOwnerCompositionServicesVtbl {
        unsafe extern "system" fn TerminateComposition<Impl: ITfContextOwnerCompositionServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcomposition: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TerminateComposition(&*(&pcomposition as *const <ITfCompositionView as ::windows::core::Abi>::Abi as *const <ITfCompositionView as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfContextOwnerCompositionServices>, ::windows::core::GetTrustLevel, TerminateComposition::<Impl, OFFSET>)
    }
}
pub trait ITfContextOwnerCompositionSinkImpl: Sized {
    fn OnStartComposition();
    fn OnUpdateComposition();
    fn OnEndComposition();
}
impl ::windows::core::RuntimeName for ITfContextOwnerCompositionSink {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfContextOwnerCompositionSink";
}
impl ITfContextOwnerCompositionSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfContextOwnerCompositionSinkImpl, const OFFSET: isize>() -> ITfContextOwnerCompositionSinkVtbl {
        unsafe extern "system" fn OnStartComposition<Impl: ITfContextOwnerCompositionSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcomposition: ::windows::core::RawPtr, pfok: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnStartComposition(&*(&pcomposition as *const <ITfCompositionView as ::windows::core::Abi>::Abi as *const <ITfCompositionView as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfok)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnUpdateComposition<Impl: ITfContextOwnerCompositionSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcomposition: ::windows::core::RawPtr, prangenew: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnUpdateComposition(&*(&pcomposition as *const <ITfCompositionView as ::windows::core::Abi>::Abi as *const <ITfCompositionView as ::windows::core::DefaultType>::DefaultType), &*(&prangenew as *const <ITfRange as ::windows::core::Abi>::Abi as *const <ITfRange as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnEndComposition<Impl: ITfContextOwnerCompositionSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcomposition: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnEndComposition(&*(&pcomposition as *const <ITfCompositionView as ::windows::core::Abi>::Abi as *const <ITfCompositionView as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfContextOwnerCompositionSink>, ::windows::core::GetTrustLevel, OnStartComposition::<Impl, OFFSET>, OnUpdateComposition::<Impl, OFFSET>, OnEndComposition::<Impl, OFFSET>)
    }
}
pub trait ITfContextOwnerServicesImpl: Sized {
    fn OnLayoutChange();
    fn OnStatusChange();
    fn OnAttributeChange();
    fn Serialize();
    fn Unserialize();
    fn ForceLoadProperty();
    fn CreateRange();
}
impl ::windows::core::RuntimeName for ITfContextOwnerServices {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfContextOwnerServices";
}
impl ITfContextOwnerServicesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfContextOwnerServicesImpl, const OFFSET: isize>() -> ITfContextOwnerServicesVtbl {
        unsafe extern "system" fn OnLayoutChange<Impl: ITfContextOwnerServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnLayoutChange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnStatusChange<Impl: ITfContextOwnerServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnStatusChange(dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnAttributeChange<Impl: ITfContextOwnerServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidattribute: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnAttributeChange(&*(&rguidattribute as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Serialize<Impl: ITfContextOwnerServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprop: ::windows::core::RawPtr, prange: ::windows::core::RawPtr, phdr: *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Serialize(&*(&pprop as *const <ITfProperty as ::windows::core::Abi>::Abi as *const <ITfProperty as ::windows::core::DefaultType>::DefaultType), &*(&prange as *const <ITfRange as ::windows::core::Abi>::Abi as *const <ITfRange as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&phdr), &*(&pstream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unserialize<Impl: ITfContextOwnerServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprop: ::windows::core::RawPtr, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: ::windows::core::RawPtr, ploader: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unserialize(
                &*(&pprop as *const <ITfProperty as ::windows::core::Abi>::Abi as *const <ITfProperty as ::windows::core::DefaultType>::DefaultType),
                &*(&phdr as *const <TF_PERSISTENT_PROPERTY_HEADER_ACP as ::windows::core::Abi>::Abi as *const <TF_PERSISTENT_PROPERTY_HEADER_ACP as ::windows::core::DefaultType>::DefaultType),
                &*(&pstream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                &*(&ploader as *const <ITfPersistentPropertyLoaderACP as ::windows::core::Abi>::Abi as *const <ITfPersistentPropertyLoaderACP as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ForceLoadProperty<Impl: ITfContextOwnerServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprop: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForceLoadProperty(&*(&pprop as *const <ITfProperty as ::windows::core::Abi>::Abi as *const <ITfProperty as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRange<Impl: ITfContextOwnerServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRange(acpstart, acpend, ::core::mem::transmute_copy(&pprange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITfContextOwnerServices>,
            ::windows::core::GetTrustLevel,
            OnLayoutChange::<Impl, OFFSET>,
            OnStatusChange::<Impl, OFFSET>,
            OnAttributeChange::<Impl, OFFSET>,
            Serialize::<Impl, OFFSET>,
            Unserialize::<Impl, OFFSET>,
            ForceLoadProperty::<Impl, OFFSET>,
            CreateRange::<Impl, OFFSET>,
        )
    }
}
pub trait ITfContextViewImpl: Sized {
    fn GetRangeFromPoint();
    fn GetTextExt();
    fn GetScreenExt();
    fn GetWnd();
}
impl ::windows::core::RuntimeName for ITfContextView {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfContextView";
}
impl ITfContextViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfContextViewImpl, const OFFSET: isize>() -> ITfContextViewVtbl {
        unsafe extern "system" fn GetRangeFromPoint<Impl: ITfContextViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, ppt: *const super::super::Foundation::POINT, dwflags: u32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRangeFromPoint(ec, &*(&ppt as *const <super::super::Foundation::POINT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::POINT as ::windows::core::DefaultType>::DefaultType), dwflags, ::core::mem::transmute_copy(&pprange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTextExt<Impl: ITfContextViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, prange: ::windows::core::RawPtr, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTextExt(ec, &*(&prange as *const <ITfRange as ::windows::core::Abi>::Abi as *const <ITfRange as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&prc), ::core::mem::transmute_copy(&pfclipped)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScreenExt<Impl: ITfContextViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetScreenExt(::core::mem::transmute_copy(&prc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWnd<Impl: ITfContextViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWnd(::core::mem::transmute_copy(&phwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfContextView>, ::windows::core::GetTrustLevel, GetRangeFromPoint::<Impl, OFFSET>, GetTextExt::<Impl, OFFSET>, GetScreenExt::<Impl, OFFSET>, GetWnd::<Impl, OFFSET>)
    }
}
pub trait ITfCreatePropertyStoreImpl: Sized {
    fn IsStoreSerializable();
    fn CreatePropertyStore();
}
impl ::windows::core::RuntimeName for ITfCreatePropertyStore {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfCreatePropertyStore";
}
impl ITfCreatePropertyStoreVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfCreatePropertyStoreImpl, const OFFSET: isize>() -> ITfCreatePropertyStoreVtbl {
        unsafe extern "system" fn IsStoreSerializable<Impl: ITfCreatePropertyStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidprop: *const ::windows::core::GUID, prange: ::windows::core::RawPtr, ppropstore: ::windows::core::RawPtr, pfserializable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStoreSerializable(&*(&guidprop as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&prange as *const <ITfRange as ::windows::core::Abi>::Abi as *const <ITfRange as ::windows::core::DefaultType>::DefaultType), &*(&ppropstore as *const <ITfPropertyStore as ::windows::core::Abi>::Abi as *const <ITfPropertyStore as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfserializable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePropertyStore<Impl: ITfCreatePropertyStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidprop: *const ::windows::core::GUID, prange: ::windows::core::RawPtr, cb: u32, pstream: ::windows::core::RawPtr, ppstore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePropertyStore(
                &*(&guidprop as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&prange as *const <ITfRange as ::windows::core::Abi>::Abi as *const <ITfRange as ::windows::core::DefaultType>::DefaultType),
                cb,
                &*(&pstream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppstore),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfCreatePropertyStore>, ::windows::core::GetTrustLevel, IsStoreSerializable::<Impl, OFFSET>, CreatePropertyStore::<Impl, OFFSET>)
    }
}
pub trait ITfDisplayAttributeInfoImpl: Sized {
    fn GetGUID();
    fn GetDescription();
    fn GetAttributeInfo();
    fn SetAttributeInfo();
    fn Reset();
}
impl ::windows::core::RuntimeName for ITfDisplayAttributeInfo {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfDisplayAttributeInfo";
}
impl ITfDisplayAttributeInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfDisplayAttributeInfoImpl, const OFFSET: isize>() -> ITfDisplayAttributeInfoVtbl {
        unsafe extern "system" fn GetGUID<Impl: ITfDisplayAttributeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGUID(::core::mem::transmute_copy(&pguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescription<Impl: ITfDisplayAttributeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdesc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDescription(::core::mem::transmute_copy(&pbstrdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeInfo<Impl: ITfDisplayAttributeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pda: *mut TF_DISPLAYATTRIBUTE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAttributeInfo(::core::mem::transmute_copy(&pda)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttributeInfo<Impl: ITfDisplayAttributeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pda: *const TF_DISPLAYATTRIBUTE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAttributeInfo(&*(&pda as *const <TF_DISPLAYATTRIBUTE as ::windows::core::Abi>::Abi as *const <TF_DISPLAYATTRIBUTE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: ITfDisplayAttributeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfDisplayAttributeInfo>, ::windows::core::GetTrustLevel, GetGUID::<Impl, OFFSET>, GetDescription::<Impl, OFFSET>, GetAttributeInfo::<Impl, OFFSET>, SetAttributeInfo::<Impl, OFFSET>, Reset::<Impl, OFFSET>)
    }
}
pub trait ITfDisplayAttributeMgrImpl: Sized {
    fn OnUpdateInfo();
    fn EnumDisplayAttributeInfo();
    fn GetDisplayAttributeInfo();
}
impl ::windows::core::RuntimeName for ITfDisplayAttributeMgr {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfDisplayAttributeMgr";
}
impl ITfDisplayAttributeMgrVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfDisplayAttributeMgrImpl, const OFFSET: isize>() -> ITfDisplayAttributeMgrVtbl {
        unsafe extern "system" fn OnUpdateInfo<Impl: ITfDisplayAttributeMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnUpdateInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDisplayAttributeInfo<Impl: ITfDisplayAttributeMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumDisplayAttributeInfo(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayAttributeInfo<Impl: ITfDisplayAttributeMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, ppinfo: *mut ::windows::core::RawPtr, pclsidowner: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDisplayAttributeInfo(&*(&guid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppinfo), ::core::mem::transmute_copy(&pclsidowner)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfDisplayAttributeMgr>, ::windows::core::GetTrustLevel, OnUpdateInfo::<Impl, OFFSET>, EnumDisplayAttributeInfo::<Impl, OFFSET>, GetDisplayAttributeInfo::<Impl, OFFSET>)
    }
}
pub trait ITfDisplayAttributeNotifySinkImpl: Sized {
    fn OnUpdateInfo();
}
impl ::windows::core::RuntimeName for ITfDisplayAttributeNotifySink {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfDisplayAttributeNotifySink";
}
impl ITfDisplayAttributeNotifySinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfDisplayAttributeNotifySinkImpl, const OFFSET: isize>() -> ITfDisplayAttributeNotifySinkVtbl {
        unsafe extern "system" fn OnUpdateInfo<Impl: ITfDisplayAttributeNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnUpdateInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfDisplayAttributeNotifySink>, ::windows::core::GetTrustLevel, OnUpdateInfo::<Impl, OFFSET>)
    }
}
pub trait ITfDisplayAttributeProviderImpl: Sized {
    fn EnumDisplayAttributeInfo();
    fn GetDisplayAttributeInfo();
}
impl ::windows::core::RuntimeName for ITfDisplayAttributeProvider {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfDisplayAttributeProvider";
}
impl ITfDisplayAttributeProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfDisplayAttributeProviderImpl, const OFFSET: isize>() -> ITfDisplayAttributeProviderVtbl {
        unsafe extern "system" fn EnumDisplayAttributeInfo<Impl: ITfDisplayAttributeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumDisplayAttributeInfo(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayAttributeInfo<Impl: ITfDisplayAttributeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, ppinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDisplayAttributeInfo(&*(&guid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfDisplayAttributeProvider>, ::windows::core::GetTrustLevel, EnumDisplayAttributeInfo::<Impl, OFFSET>, GetDisplayAttributeInfo::<Impl, OFFSET>)
    }
}
pub trait ITfDocumentMgrImpl: Sized {
    fn CreateContext();
    fn Push();
    fn Pop();
    fn GetTop();
    fn GetBase();
    fn EnumContexts();
}
impl ::windows::core::RuntimeName for ITfDocumentMgr {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfDocumentMgr";
}
impl ITfDocumentMgrVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfDocumentMgrImpl, const OFFSET: isize>() -> ITfDocumentMgrVtbl {
        unsafe extern "system" fn CreateContext<Impl: ITfDocumentMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tidowner: u32, dwflags: u32, punk: *mut ::core::ffi::c_void, ppic: *mut ::windows::core::RawPtr, pectextstore: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateContext(tidowner, dwflags, &*(&punk as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppic), ::core::mem::transmute_copy(&pectextstore)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Push<Impl: ITfDocumentMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Push(&*(&pic as *const <ITfContext as ::windows::core::Abi>::Abi as *const <ITfContext as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pop<Impl: ITfDocumentMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pop(dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTop<Impl: ITfDocumentMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppic: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTop(::core::mem::transmute_copy(&ppic)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBase<Impl: ITfDocumentMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppic: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBase(::core::mem::transmute_copy(&ppic)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumContexts<Impl: ITfDocumentMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumContexts(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfDocumentMgr>, ::windows::core::GetTrustLevel, CreateContext::<Impl, OFFSET>, Push::<Impl, OFFSET>, Pop::<Impl, OFFSET>, GetTop::<Impl, OFFSET>, GetBase::<Impl, OFFSET>, EnumContexts::<Impl, OFFSET>)
    }
}
pub trait ITfEditRecordImpl: Sized {
    fn GetSelectionStatus();
    fn GetTextAndPropertyUpdates();
}
impl ::windows::core::RuntimeName for ITfEditRecord {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfEditRecord";
}
impl ITfEditRecordVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfEditRecordImpl, const OFFSET: isize>() -> ITfEditRecordVtbl {
        unsafe extern "system" fn GetSelectionStatus<Impl: ITfEditRecordImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfchanged: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSelectionStatus(::core::mem::transmute_copy(&pfchanged)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTextAndPropertyUpdates<Impl: ITfEditRecordImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: GET_TEXT_AND_PROPERTY_UPDATES_FLAGS, prgproperties: *const *const ::windows::core::GUID, cproperties: u32, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTextAndPropertyUpdates(dwflags, &*(&prgproperties as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), cproperties, ::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfEditRecord>, ::windows::core::GetTrustLevel, GetSelectionStatus::<Impl, OFFSET>, GetTextAndPropertyUpdates::<Impl, OFFSET>)
    }
}
pub trait ITfEditSessionImpl: Sized {
    fn DoEditSession();
}
impl ::windows::core::RuntimeName for ITfEditSession {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfEditSession";
}
impl ITfEditSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfEditSessionImpl, const OFFSET: isize>() -> ITfEditSessionVtbl {
        unsafe extern "system" fn DoEditSession<Impl: ITfEditSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DoEditSession(ec) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfEditSession>, ::windows::core::GetTrustLevel, DoEditSession::<Impl, OFFSET>)
    }
}
pub trait ITfEditTransactionSinkImpl: Sized {
    fn OnStartEditTransaction();
    fn OnEndEditTransaction();
}
impl ::windows::core::RuntimeName for ITfEditTransactionSink {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfEditTransactionSink";
}
impl ITfEditTransactionSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfEditTransactionSinkImpl, const OFFSET: isize>() -> ITfEditTransactionSinkVtbl {
        unsafe extern "system" fn OnStartEditTransaction<Impl: ITfEditTransactionSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnStartEditTransaction(&*(&pic as *const <ITfContext as ::windows::core::Abi>::Abi as *const <ITfContext as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnEndEditTransaction<Impl: ITfEditTransactionSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnEndEditTransaction(&*(&pic as *const <ITfContext as ::windows::core::Abi>::Abi as *const <ITfContext as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfEditTransactionSink>, ::windows::core::GetTrustLevel, OnStartEditTransaction::<Impl, OFFSET>, OnEndEditTransaction::<Impl, OFFSET>)
    }
}
pub trait ITfFnAdviseTextImpl: Sized + ITfFunctionImpl {
    fn OnTextUpdate();
    fn OnLatticeUpdate();
}
impl ::windows::core::RuntimeName for ITfFnAdviseText {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfFnAdviseText";
}
impl ITfFnAdviseTextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFnAdviseTextImpl, const OFFSET: isize>() -> ITfFnAdviseTextVtbl {
        unsafe extern "system" fn OnTextUpdate<Impl: ITfFnAdviseTextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, pchtext: super::super::Foundation::PWSTR, cch: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnTextUpdate(&*(&prange as *const <ITfRange as ::windows::core::Abi>::Abi as *const <ITfRange as ::windows::core::DefaultType>::DefaultType), &*(&pchtext as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cch) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnLatticeUpdate<Impl: ITfFnAdviseTextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, plattice: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnLatticeUpdate(&*(&prange as *const <ITfRange as ::windows::core::Abi>::Abi as *const <ITfRange as ::windows::core::DefaultType>::DefaultType), &*(&plattice as *const <ITfLMLattice as ::windows::core::Abi>::Abi as *const <ITfLMLattice as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfFnAdviseText>, ::windows::core::GetTrustLevel, OnTextUpdate::<Impl, OFFSET>, OnLatticeUpdate::<Impl, OFFSET>)
    }
}
pub trait ITfFnBalloonImpl: Sized {
    fn UpdateBalloon();
}
impl ::windows::core::RuntimeName for ITfFnBalloon {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfFnBalloon";
}
impl ITfFnBalloonVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFnBalloonImpl, const OFFSET: isize>() -> ITfFnBalloonVtbl {
        unsafe extern "system" fn UpdateBalloon<Impl: ITfFnBalloonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, style: TfLBBalloonStyle, pch: super::super::Foundation::PWSTR, cch: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateBalloon(style, &*(&pch as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cch) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfFnBalloon>, ::windows::core::GetTrustLevel, UpdateBalloon::<Impl, OFFSET>)
    }
}
pub trait ITfFnConfigureImpl: Sized + ITfFunctionImpl {
    fn Show();
}
impl ::windows::core::RuntimeName for ITfFnConfigure {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfFnConfigure";
}
impl ITfFnConfigureVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFnConfigureImpl, const OFFSET: isize>() -> ITfFnConfigureVtbl {
        unsafe extern "system" fn Show<Impl: ITfFnConfigureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Show(&*(&hwndparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), langid, &*(&rguidprofile as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfFnConfigure>, ::windows::core::GetTrustLevel, Show::<Impl, OFFSET>)
    }
}
pub trait ITfFnConfigureRegisterEudcImpl: Sized + ITfFunctionImpl {
    fn Show();
}
impl ::windows::core::RuntimeName for ITfFnConfigureRegisterEudc {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfFnConfigureRegisterEudc";
}
impl ITfFnConfigureRegisterEudcVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFnConfigureRegisterEudcImpl, const OFFSET: isize>() -> ITfFnConfigureRegisterEudcVtbl {
        unsafe extern "system" fn Show<Impl: ITfFnConfigureRegisterEudcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const ::windows::core::GUID, bstrregistered: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Show(
                &*(&hwndparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                langid,
                &*(&rguidprofile as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrregistered as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfFnConfigureRegisterEudc>, ::windows::core::GetTrustLevel, Show::<Impl, OFFSET>)
    }
}
pub trait ITfFnConfigureRegisterWordImpl: Sized + ITfFunctionImpl {
    fn Show();
}
impl ::windows::core::RuntimeName for ITfFnConfigureRegisterWord {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfFnConfigureRegisterWord";
}
impl ITfFnConfigureRegisterWordVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFnConfigureRegisterWordImpl, const OFFSET: isize>() -> ITfFnConfigureRegisterWordVtbl {
        unsafe extern "system" fn Show<Impl: ITfFnConfigureRegisterWordImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const ::windows::core::GUID, bstrregistered: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Show(
                &*(&hwndparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                langid,
                &*(&rguidprofile as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrregistered as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfFnConfigureRegisterWord>, ::windows::core::GetTrustLevel, Show::<Impl, OFFSET>)
    }
}
pub trait ITfFnCustomSpeechCommandImpl: Sized + ITfFunctionImpl {
    fn SetSpeechCommandProvider();
}
impl ::windows::core::RuntimeName for ITfFnCustomSpeechCommand {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfFnCustomSpeechCommand";
}
impl ITfFnCustomSpeechCommandVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFnCustomSpeechCommandImpl, const OFFSET: isize>() -> ITfFnCustomSpeechCommandVtbl {
        unsafe extern "system" fn SetSpeechCommandProvider<Impl: ITfFnCustomSpeechCommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pspcmdprovider: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSpeechCommandProvider(&*(&pspcmdprovider as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfFnCustomSpeechCommand>, ::windows::core::GetTrustLevel, SetSpeechCommandProvider::<Impl, OFFSET>)
    }
}
pub trait ITfFnGetLinguisticAlternatesImpl: Sized + ITfFunctionImpl {
    fn GetAlternates();
}
impl ::windows::core::RuntimeName for ITfFnGetLinguisticAlternates {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfFnGetLinguisticAlternates";
}
impl ITfFnGetLinguisticAlternatesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFnGetLinguisticAlternatesImpl, const OFFSET: isize>() -> ITfFnGetLinguisticAlternatesVtbl {
        unsafe extern "system" fn GetAlternates<Impl: ITfFnGetLinguisticAlternatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, ppcandidatelist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAlternates(&*(&prange as *const <ITfRange as ::windows::core::Abi>::Abi as *const <ITfRange as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppcandidatelist)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfFnGetLinguisticAlternates>, ::windows::core::GetTrustLevel, GetAlternates::<Impl, OFFSET>)
    }
}
pub trait ITfFnGetPreferredTouchKeyboardLayoutImpl: Sized + ITfFunctionImpl {
    fn GetLayout();
}
impl ::windows::core::RuntimeName for ITfFnGetPreferredTouchKeyboardLayout {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfFnGetPreferredTouchKeyboardLayout";
}
impl ITfFnGetPreferredTouchKeyboardLayoutVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFnGetPreferredTouchKeyboardLayoutImpl, const OFFSET: isize>() -> ITfFnGetPreferredTouchKeyboardLayoutVtbl {
        unsafe extern "system" fn GetLayout<Impl: ITfFnGetPreferredTouchKeyboardLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptkblayouttype: *mut TKBLayoutType, pwpreferredlayoutid: *const u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLayout(::core::mem::transmute_copy(&ptkblayouttype), pwpreferredlayoutid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfFnGetPreferredTouchKeyboardLayout>, ::windows::core::GetTrustLevel, GetLayout::<Impl, OFFSET>)
    }
}
pub trait ITfFnGetSAPIObjectImpl: Sized + ITfFunctionImpl {
    fn Get();
}
impl ::windows::core::RuntimeName for ITfFnGetSAPIObject {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfFnGetSAPIObject";
}
impl ITfFnGetSAPIObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFnGetSAPIObjectImpl, const OFFSET: isize>() -> ITfFnGetSAPIObjectVtbl {
        unsafe extern "system" fn Get<Impl: ITfFnGetSAPIObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sobj: TfSapiObject, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Get(sobj, ::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfFnGetSAPIObject>, ::windows::core::GetTrustLevel, Get::<Impl, OFFSET>)
    }
}
pub trait ITfFnLMInternalImpl: Sized + ITfFnLMProcessorImpl + ITfFunctionImpl {
    fn ProcessLattice();
}
impl ::windows::core::RuntimeName for ITfFnLMInternal {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfFnLMInternal";
}
impl ITfFnLMInternalVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFnLMInternalImpl, const OFFSET: isize>() -> ITfFnLMInternalVtbl {
        unsafe extern "system" fn ProcessLattice<Impl: ITfFnLMInternalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessLattice(&*(&prange as *const <ITfRange as ::windows::core::Abi>::Abi as *const <ITfRange as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfFnLMInternal>, ::windows::core::GetTrustLevel, ProcessLattice::<Impl, OFFSET>)
    }
}
pub trait ITfFnLMProcessorImpl: Sized + ITfFunctionImpl {
    fn QueryRange();
    fn QueryLangID();
    fn GetReconversion();
    fn Reconvert();
    fn QueryKey();
    fn InvokeKey();
    fn InvokeFunc();
}
impl ::windows::core::RuntimeName for ITfFnLMProcessor {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfFnLMProcessor";
}
impl ITfFnLMProcessorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFnLMProcessorImpl, const OFFSET: isize>() -> ITfFnLMProcessorVtbl {
        unsafe extern "system" fn QueryRange<Impl: ITfFnLMProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, ppnewrange: *mut ::windows::core::RawPtr, pfaccepted: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryRange(&*(&prange as *const <ITfRange as ::windows::core::Abi>::Abi as *const <ITfRange as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppnewrange), ::core::mem::transmute_copy(&pfaccepted)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryLangID<Impl: ITfFnLMProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16, pfaccepted: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryLangID(langid, ::core::mem::transmute_copy(&pfaccepted)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReconversion<Impl: ITfFnLMProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, ppcandlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetReconversion(&*(&prange as *const <ITfRange as ::windows::core::Abi>::Abi as *const <ITfRange as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppcandlist)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reconvert<Impl: ITfFnLMProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reconvert(&*(&prange as *const <ITfRange as ::windows::core::Abi>::Abi as *const <ITfRange as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryKey<Impl: ITfFnLMProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fup: super::super::Foundation::BOOL, vkey: super::super::Foundation::WPARAM, lparamkeydata: super::super::Foundation::LPARAM, pfinterested: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryKey(
                &*(&fup as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&vkey as *const <super::super::Foundation::WPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::WPARAM as ::windows::core::DefaultType>::DefaultType),
                &*(&lparamkeydata as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pfinterested),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvokeKey<Impl: ITfFnLMProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fup: super::super::Foundation::BOOL, vkey: super::super::Foundation::WPARAM, lparamkeydata: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InvokeKey(
                &*(&fup as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&vkey as *const <super::super::Foundation::WPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::WPARAM as ::windows::core::DefaultType>::DefaultType),
                &*(&lparamkeydata as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvokeFunc<Impl: ITfFnLMProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr, refguidfunc: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InvokeFunc(&*(&pic as *const <ITfContext as ::windows::core::Abi>::Abi as *const <ITfContext as ::windows::core::DefaultType>::DefaultType), &*(&refguidfunc as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfFnLMProcessor>, ::windows::core::GetTrustLevel, QueryRange::<Impl, OFFSET>, QueryLangID::<Impl, OFFSET>, GetReconversion::<Impl, OFFSET>, Reconvert::<Impl, OFFSET>, QueryKey::<Impl, OFFSET>, InvokeKey::<Impl, OFFSET>, InvokeFunc::<Impl, OFFSET>)
    }
}
pub trait ITfFnLangProfileUtilImpl: Sized + ITfFunctionImpl {
    fn RegisterActiveProfiles();
    fn IsProfileAvailableForLang();
}
impl ::windows::core::RuntimeName for ITfFnLangProfileUtil {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfFnLangProfileUtil";
}
impl ITfFnLangProfileUtilVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFnLangProfileUtilImpl, const OFFSET: isize>() -> ITfFnLangProfileUtilVtbl {
        unsafe extern "system" fn RegisterActiveProfiles<Impl: ITfFnLangProfileUtilImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterActiveProfiles() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsProfileAvailableForLang<Impl: ITfFnLangProfileUtilImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16, pfavailable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsProfileAvailableForLang(langid, ::core::mem::transmute_copy(&pfavailable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfFnLangProfileUtil>, ::windows::core::GetTrustLevel, RegisterActiveProfiles::<Impl, OFFSET>, IsProfileAvailableForLang::<Impl, OFFSET>)
    }
}
pub trait ITfFnPlayBackImpl: Sized + ITfFunctionImpl {
    fn QueryRange();
    fn Play();
}
impl ::windows::core::RuntimeName for ITfFnPlayBack {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfFnPlayBack";
}
impl ITfFnPlayBackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFnPlayBackImpl, const OFFSET: isize>() -> ITfFnPlayBackVtbl {
        unsafe extern "system" fn QueryRange<Impl: ITfFnPlayBackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, ppnewrange: *mut ::windows::core::RawPtr, pfplayable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryRange(&*(&prange as *const <ITfRange as ::windows::core::Abi>::Abi as *const <ITfRange as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppnewrange), ::core::mem::transmute_copy(&pfplayable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Play<Impl: ITfFnPlayBackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Play(&*(&prange as *const <ITfRange as ::windows::core::Abi>::Abi as *const <ITfRange as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfFnPlayBack>, ::windows::core::GetTrustLevel, QueryRange::<Impl, OFFSET>, Play::<Impl, OFFSET>)
    }
}
pub trait ITfFnPropertyUIStatusImpl: Sized + ITfFunctionImpl {
    fn GetStatus();
    fn SetStatus();
}
impl ::windows::core::RuntimeName for ITfFnPropertyUIStatus {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfFnPropertyUIStatus";
}
impl ITfFnPropertyUIStatusVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFnPropertyUIStatusImpl, const OFFSET: isize>() -> ITfFnPropertyUIStatusVtbl {
        unsafe extern "system" fn GetStatus<Impl: ITfFnPropertyUIStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, refguidprop: *const ::windows::core::GUID, pdw: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus(&*(&refguidprop as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdw)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStatus<Impl: ITfFnPropertyUIStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, refguidprop: *const ::windows::core::GUID, dw: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetStatus(&*(&refguidprop as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dw) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfFnPropertyUIStatus>, ::windows::core::GetTrustLevel, GetStatus::<Impl, OFFSET>, SetStatus::<Impl, OFFSET>)
    }
}
pub trait ITfFnReconversionImpl: Sized + ITfFunctionImpl {
    fn QueryRange();
    fn GetReconversion();
    fn Reconvert();
}
impl ::windows::core::RuntimeName for ITfFnReconversion {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfFnReconversion";
}
impl ITfFnReconversionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFnReconversionImpl, const OFFSET: isize>() -> ITfFnReconversionVtbl {
        unsafe extern "system" fn QueryRange<Impl: ITfFnReconversionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, ppnewrange: *mut ::windows::core::RawPtr, pfconvertable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryRange(&*(&prange as *const <ITfRange as ::windows::core::Abi>::Abi as *const <ITfRange as ::windows::core::DefaultType>::DefaultType), &*(&ppnewrange as *const <ITfRange as ::windows::core::Abi>::Abi as *const <ITfRange as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfconvertable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReconversion<Impl: ITfFnReconversionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, ppcandlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetReconversion(&*(&prange as *const <ITfRange as ::windows::core::Abi>::Abi as *const <ITfRange as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppcandlist)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reconvert<Impl: ITfFnReconversionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reconvert(&*(&prange as *const <ITfRange as ::windows::core::Abi>::Abi as *const <ITfRange as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfFnReconversion>, ::windows::core::GetTrustLevel, QueryRange::<Impl, OFFSET>, GetReconversion::<Impl, OFFSET>, Reconvert::<Impl, OFFSET>)
    }
}
pub trait ITfFnSearchCandidateProviderImpl: Sized + ITfFunctionImpl {
    fn GetSearchCandidates();
    fn SetResult();
}
impl ::windows::core::RuntimeName for ITfFnSearchCandidateProvider {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfFnSearchCandidateProvider";
}
impl ITfFnSearchCandidateProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFnSearchCandidateProviderImpl, const OFFSET: isize>() -> ITfFnSearchCandidateProviderVtbl {
        unsafe extern "system" fn GetSearchCandidates<Impl: ITfFnSearchCandidateProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrapplicationid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pplist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSearchCandidates(&*(&bstrquery as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrapplicationid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pplist)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetResult<Impl: ITfFnSearchCandidateProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrapplicationid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrresult: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetResult(
                &*(&bstrquery as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrapplicationid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrresult as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfFnSearchCandidateProvider>, ::windows::core::GetTrustLevel, GetSearchCandidates::<Impl, OFFSET>, SetResult::<Impl, OFFSET>)
    }
}
pub trait ITfFnShowHelpImpl: Sized + ITfFunctionImpl {
    fn Show();
}
impl ::windows::core::RuntimeName for ITfFnShowHelp {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfFnShowHelp";
}
impl ITfFnShowHelpVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFnShowHelpImpl, const OFFSET: isize>() -> ITfFnShowHelpVtbl {
        unsafe extern "system" fn Show<Impl: ITfFnShowHelpImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Show(&*(&hwndparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfFnShowHelp>, ::windows::core::GetTrustLevel, Show::<Impl, OFFSET>)
    }
}
pub trait ITfFunctionImpl: Sized {
    fn GetDisplayName();
}
impl ::windows::core::RuntimeName for ITfFunction {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfFunction";
}
impl ITfFunctionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFunctionImpl, const OFFSET: isize>() -> ITfFunctionVtbl {
        unsafe extern "system" fn GetDisplayName<Impl: ITfFunctionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDisplayName(::core::mem::transmute_copy(&pbstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfFunction>, ::windows::core::GetTrustLevel, GetDisplayName::<Impl, OFFSET>)
    }
}
pub trait ITfFunctionProviderImpl: Sized {
    fn GetType();
    fn GetDescription();
    fn GetFunction();
}
impl ::windows::core::RuntimeName for ITfFunctionProvider {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfFunctionProvider";
}
impl ITfFunctionProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFunctionProviderImpl, const OFFSET: isize>() -> ITfFunctionProviderVtbl {
        unsafe extern "system" fn GetType<Impl: ITfFunctionProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetType(::core::mem::transmute_copy(&pguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescription<Impl: ITfFunctionProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdesc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDescription(::core::mem::transmute_copy(&pbstrdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunction<Impl: ITfFunctionProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFunction(&*(&rguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfFunctionProvider>, ::windows::core::GetTrustLevel, GetType::<Impl, OFFSET>, GetDescription::<Impl, OFFSET>, GetFunction::<Impl, OFFSET>)
    }
}
pub trait ITfInputProcessorProfileActivationSinkImpl: Sized {
    fn OnActivated();
}
impl ::windows::core::RuntimeName for ITfInputProcessorProfileActivationSink {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfInputProcessorProfileActivationSink";
}
impl ITfInputProcessorProfileActivationSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfInputProcessorProfileActivationSinkImpl, const OFFSET: isize>() -> ITfInputProcessorProfileActivationSinkVtbl {
        unsafe extern "system" fn OnActivated<Impl: ITfInputProcessorProfileActivationSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwprofiletype: u32, langid: u16, clsid: *const ::windows::core::GUID, catid: *const ::windows::core::GUID, guidprofile: *const ::windows::core::GUID, hkl: HKL, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnActivated(
                dwprofiletype,
                langid,
                &*(&clsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&catid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&guidprofile as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&hkl as *const <HKL as ::windows::core::Abi>::Abi as *const <HKL as ::windows::core::DefaultType>::DefaultType),
                dwflags,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfInputProcessorProfileActivationSink>, ::windows::core::GetTrustLevel, OnActivated::<Impl, OFFSET>)
    }
}
pub trait ITfInputProcessorProfileMgrImpl: Sized {
    fn ActivateProfile();
    fn DeactivateProfile();
    fn GetProfile();
    fn EnumProfiles();
    fn ReleaseInputProcessor();
    fn RegisterProfile();
    fn UnregisterProfile();
    fn GetActiveProfile();
}
impl ::windows::core::RuntimeName for ITfInputProcessorProfileMgr {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfInputProcessorProfileMgr";
}
impl ITfInputProcessorProfileMgrVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfInputProcessorProfileMgrImpl, const OFFSET: isize>() -> ITfInputProcessorProfileMgrVtbl {
        unsafe extern "system" fn ActivateProfile<Impl: ITfInputProcessorProfileMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwprofiletype: u32, langid: u16, clsid: *const ::windows::core::GUID, guidprofile: *const ::windows::core::GUID, hkl: HKL, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivateProfile(dwprofiletype, langid, &*(&clsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&guidprofile as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&hkl as *const <HKL as ::windows::core::Abi>::Abi as *const <HKL as ::windows::core::DefaultType>::DefaultType), dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeactivateProfile<Impl: ITfInputProcessorProfileMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwprofiletype: u32, langid: u16, clsid: *const ::windows::core::GUID, guidprofile: *const ::windows::core::GUID, hkl: HKL, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeactivateProfile(dwprofiletype, langid, &*(&clsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&guidprofile as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&hkl as *const <HKL as ::windows::core::Abi>::Abi as *const <HKL as ::windows::core::DefaultType>::DefaultType), dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProfile<Impl: ITfInputProcessorProfileMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwprofiletype: u32, langid: u16, clsid: *const ::windows::core::GUID, guidprofile: *const ::windows::core::GUID, hkl: HKL, pprofile: *mut TF_INPUTPROCESSORPROFILE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProfile(
                dwprofiletype,
                langid,
                &*(&clsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&guidprofile as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&hkl as *const <HKL as ::windows::core::Abi>::Abi as *const <HKL as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pprofile),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumProfiles<Impl: ITfInputProcessorProfileMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumProfiles(langid, ::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseInputProcessor<Impl: ITfInputProcessorProfileMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReleaseInputProcessor(&*(&rclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterProfile<Impl: ITfInputProcessorProfileMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, pchdesc: super::super::Foundation::PWSTR, cchdesc: u32, pchiconfile: super::super::Foundation::PWSTR, cchfile: u32, uiconindex: u32, hklsubstitute: HKL, dwpreferredlayout: u32, benabledbydefault: super::super::Foundation::BOOL, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterProfile(
                &*(&rclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                langid,
                &*(&guidprofile as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&pchdesc as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cchdesc,
                &*(&pchiconfile as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cchfile,
                uiconindex,
                &*(&hklsubstitute as *const <HKL as ::windows::core::Abi>::Abi as *const <HKL as ::windows::core::DefaultType>::DefaultType),
                dwpreferredlayout,
                &*(&benabledbydefault as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                dwflags,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterProfile<Impl: ITfInputProcessorProfileMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnregisterProfile(&*(&rclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), langid, &*(&guidprofile as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActiveProfile<Impl: ITfInputProcessorProfileMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, catid: *const ::windows::core::GUID, pprofile: *mut TF_INPUTPROCESSORPROFILE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActiveProfile(&*(&catid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITfInputProcessorProfileMgr>,
            ::windows::core::GetTrustLevel,
            ActivateProfile::<Impl, OFFSET>,
            DeactivateProfile::<Impl, OFFSET>,
            GetProfile::<Impl, OFFSET>,
            EnumProfiles::<Impl, OFFSET>,
            ReleaseInputProcessor::<Impl, OFFSET>,
            RegisterProfile::<Impl, OFFSET>,
            UnregisterProfile::<Impl, OFFSET>,
            GetActiveProfile::<Impl, OFFSET>,
        )
    }
}
pub trait ITfInputProcessorProfileSubstituteLayoutImpl: Sized {
    fn GetSubstituteKeyboardLayout();
}
impl ::windows::core::RuntimeName for ITfInputProcessorProfileSubstituteLayout {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfInputProcessorProfileSubstituteLayout";
}
impl ITfInputProcessorProfileSubstituteLayoutVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfInputProcessorProfileSubstituteLayoutImpl, const OFFSET: isize>() -> ITfInputProcessorProfileSubstituteLayoutVtbl {
        unsafe extern "system" fn GetSubstituteKeyboardLayout<Impl: ITfInputProcessorProfileSubstituteLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, phkl: *mut HKL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSubstituteKeyboardLayout(&*(&rclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), langid, &*(&guidprofile as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&phkl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfInputProcessorProfileSubstituteLayout>, ::windows::core::GetTrustLevel, GetSubstituteKeyboardLayout::<Impl, OFFSET>)
    }
}
pub trait ITfInputProcessorProfilesImpl: Sized {
    fn Register();
    fn Unregister();
    fn AddLanguageProfile();
    fn RemoveLanguageProfile();
    fn EnumInputProcessorInfo();
    fn GetDefaultLanguageProfile();
    fn SetDefaultLanguageProfile();
    fn ActivateLanguageProfile();
    fn GetActiveLanguageProfile();
    fn GetLanguageProfileDescription();
    fn GetCurrentLanguage();
    fn ChangeCurrentLanguage();
    fn GetLanguageList();
    fn EnumLanguageProfiles();
    fn EnableLanguageProfile();
    fn IsEnabledLanguageProfile();
    fn EnableLanguageProfileByDefault();
    fn SubstituteKeyboardLayout();
}
impl ::windows::core::RuntimeName for ITfInputProcessorProfiles {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfInputProcessorProfiles";
}
impl ITfInputProcessorProfilesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfInputProcessorProfilesImpl, const OFFSET: isize>() -> ITfInputProcessorProfilesVtbl {
        unsafe extern "system" fn Register<Impl: ITfInputProcessorProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Register(&*(&rclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unregister<Impl: ITfInputProcessorProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unregister(&*(&rclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddLanguageProfile<Impl: ITfInputProcessorProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, pchdesc: super::super::Foundation::PWSTR, cchdesc: u32, pchiconfile: super::super::Foundation::PWSTR, cchfile: u32, uiconindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddLanguageProfile(
                &*(&rclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                langid,
                &*(&guidprofile as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&pchdesc as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cchdesc,
                &*(&pchiconfile as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cchfile,
                uiconindex,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLanguageProfile<Impl: ITfInputProcessorProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveLanguageProfile(&*(&rclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), langid, &*(&guidprofile as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumInputProcessorInfo<Impl: ITfInputProcessorProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumInputProcessorInfo(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultLanguageProfile<Impl: ITfInputProcessorProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16, catid: *const ::windows::core::GUID, pclsid: *mut ::windows::core::GUID, pguidprofile: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultLanguageProfile(langid, &*(&catid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pclsid), ::core::mem::transmute_copy(&pguidprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultLanguageProfile<Impl: ITfInputProcessorProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16, rclsid: *const ::windows::core::GUID, guidprofiles: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDefaultLanguageProfile(langid, &*(&rclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&guidprofiles as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivateLanguageProfile<Impl: ITfInputProcessorProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofiles: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivateLanguageProfile(&*(&rclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), langid, &*(&guidprofiles as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActiveLanguageProfile<Impl: ITfInputProcessorProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, plangid: *mut u16, pguidprofile: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActiveLanguageProfile(&*(&rclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&plangid), ::core::mem::transmute_copy(&pguidprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLanguageProfileDescription<Impl: ITfInputProcessorProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, pbstrprofile: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLanguageProfileDescription(&*(&rclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), langid, &*(&guidprofile as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbstrprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentLanguage<Impl: ITfInputProcessorProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plangid: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentLanguage(::core::mem::transmute_copy(&plangid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChangeCurrentLanguage<Impl: ITfInputProcessorProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChangeCurrentLanguage(langid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLanguageList<Impl: ITfInputProcessorProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplangid: *mut *mut u16, pulcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLanguageList(::core::mem::transmute_copy(&pplangid), ::core::mem::transmute_copy(&pulcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumLanguageProfiles<Impl: ITfInputProcessorProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumLanguageProfiles(langid, ::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableLanguageProfile<Impl: ITfInputProcessorProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableLanguageProfile(
                &*(&rclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                langid,
                &*(&guidprofile as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&fenable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEnabledLanguageProfile<Impl: ITfInputProcessorProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, pfenable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEnabledLanguageProfile(&*(&rclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), langid, &*(&guidprofile as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfenable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableLanguageProfileByDefault<Impl: ITfInputProcessorProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableLanguageProfileByDefault(
                &*(&rclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                langid,
                &*(&guidprofile as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&fenable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubstituteKeyboardLayout<Impl: ITfInputProcessorProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, hkl: HKL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubstituteKeyboardLayout(&*(&rclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), langid, &*(&guidprofile as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&hkl as *const <HKL as ::windows::core::Abi>::Abi as *const <HKL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITfInputProcessorProfiles>,
            ::windows::core::GetTrustLevel,
            Register::<Impl, OFFSET>,
            Unregister::<Impl, OFFSET>,
            AddLanguageProfile::<Impl, OFFSET>,
            RemoveLanguageProfile::<Impl, OFFSET>,
            EnumInputProcessorInfo::<Impl, OFFSET>,
            GetDefaultLanguageProfile::<Impl, OFFSET>,
            SetDefaultLanguageProfile::<Impl, OFFSET>,
            ActivateLanguageProfile::<Impl, OFFSET>,
            GetActiveLanguageProfile::<Impl, OFFSET>,
            GetLanguageProfileDescription::<Impl, OFFSET>,
            GetCurrentLanguage::<Impl, OFFSET>,
            ChangeCurrentLanguage::<Impl, OFFSET>,
            GetLanguageList::<Impl, OFFSET>,
            EnumLanguageProfiles::<Impl, OFFSET>,
            EnableLanguageProfile::<Impl, OFFSET>,
            IsEnabledLanguageProfile::<Impl, OFFSET>,
            EnableLanguageProfileByDefault::<Impl, OFFSET>,
            SubstituteKeyboardLayout::<Impl, OFFSET>,
        )
    }
}
pub trait ITfInputProcessorProfilesExImpl: Sized + ITfInputProcessorProfilesImpl {
    fn SetLanguageProfileDisplayName();
}
impl ::windows::core::RuntimeName for ITfInputProcessorProfilesEx {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfInputProcessorProfilesEx";
}
impl ITfInputProcessorProfilesExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfInputProcessorProfilesExImpl, const OFFSET: isize>() -> ITfInputProcessorProfilesExVtbl {
        unsafe extern "system" fn SetLanguageProfileDisplayName<Impl: ITfInputProcessorProfilesExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, pchfile: super::super::Foundation::PWSTR, cchfile: u32, uresid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLanguageProfileDisplayName(
                &*(&rclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                langid,
                &*(&guidprofile as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&pchfile as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cchfile,
                uresid,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfInputProcessorProfilesEx>, ::windows::core::GetTrustLevel, SetLanguageProfileDisplayName::<Impl, OFFSET>)
    }
}
pub trait ITfInputScopeImpl: Sized {
    fn GetInputScopes();
    fn GetPhrase();
    fn GetRegularExpression();
    fn GetSRGS();
    fn GetXML();
}
impl ::windows::core::RuntimeName for ITfInputScope {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfInputScope";
}
impl ITfInputScopeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfInputScopeImpl, const OFFSET: isize>() -> ITfInputScopeVtbl {
        unsafe extern "system" fn GetInputScopes<Impl: ITfInputScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprginputscopes: *mut *mut InputScope, pccount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputScopes(::core::mem::transmute_copy(&pprginputscopes), ::core::mem::transmute_copy(&pccount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPhrase<Impl: ITfInputScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbstrphrases: *mut *mut super::super::Foundation::BSTR, pccount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPhrase(::core::mem::transmute_copy(&ppbstrphrases), ::core::mem::transmute_copy(&pccount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRegularExpression<Impl: ITfInputScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrregexp: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRegularExpression(::core::mem::transmute_copy(&pbstrregexp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSRGS<Impl: ITfInputScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsrgs: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSRGS(::core::mem::transmute_copy(&pbstrsrgs)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXML<Impl: ITfInputScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrxml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetXML(::core::mem::transmute_copy(&pbstrxml)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfInputScope>, ::windows::core::GetTrustLevel, GetInputScopes::<Impl, OFFSET>, GetPhrase::<Impl, OFFSET>, GetRegularExpression::<Impl, OFFSET>, GetSRGS::<Impl, OFFSET>, GetXML::<Impl, OFFSET>)
    }
}
pub trait ITfInputScope2Impl: Sized + ITfInputScopeImpl {
    fn EnumWordList();
}
impl ::windows::core::RuntimeName for ITfInputScope2 {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfInputScope2";
}
impl ITfInputScope2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfInputScope2Impl, const OFFSET: isize>() -> ITfInputScope2Vtbl {
        unsafe extern "system" fn EnumWordList<Impl: ITfInputScope2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumstring: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumWordList(::core::mem::transmute_copy(&ppenumstring)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfInputScope2>, ::windows::core::GetTrustLevel, EnumWordList::<Impl, OFFSET>)
    }
}
pub trait ITfInsertAtSelectionImpl: Sized {
    fn InsertTextAtSelection();
    fn InsertEmbeddedAtSelection();
}
impl ::windows::core::RuntimeName for ITfInsertAtSelection {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfInsertAtSelection";
}
impl ITfInsertAtSelectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfInsertAtSelectionImpl, const OFFSET: isize>() -> ITfInsertAtSelectionVtbl {
        unsafe extern "system" fn InsertTextAtSelection<Impl: ITfInsertAtSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, dwflags: INSERT_TEXT_AT_SELECTION_FLAGS, pchtext: super::super::Foundation::PWSTR, cch: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InsertTextAtSelection(ec, dwflags, &*(&pchtext as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cch, ::core::mem::transmute_copy(&pprange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertEmbeddedAtSelection<Impl: ITfInsertAtSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, dwflags: u32, pdataobject: ::windows::core::RawPtr, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InsertEmbeddedAtSelection(ec, dwflags, &*(&pdataobject as *const <super::super::System::Com::IDataObject as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IDataObject as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pprange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfInsertAtSelection>, ::windows::core::GetTrustLevel, InsertTextAtSelection::<Impl, OFFSET>, InsertEmbeddedAtSelection::<Impl, OFFSET>)
    }
}
pub trait ITfIntegratableCandidateListUIElementImpl: Sized {
    fn SetIntegrationStyle();
    fn GetSelectionStyle();
    fn OnKeyDown();
    fn ShowCandidateNumbers();
    fn FinalizeExactCompositionString();
}
impl ::windows::core::RuntimeName for ITfIntegratableCandidateListUIElement {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfIntegratableCandidateListUIElement";
}
impl ITfIntegratableCandidateListUIElementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfIntegratableCandidateListUIElementImpl, const OFFSET: isize>() -> ITfIntegratableCandidateListUIElementVtbl {
        unsafe extern "system" fn SetIntegrationStyle<Impl: ITfIntegratableCandidateListUIElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidintegrationstyle: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetIntegrationStyle(&*(&guidintegrationstyle as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelectionStyle<Impl: ITfIntegratableCandidateListUIElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptfselectionstyle: *mut TfIntegratableCandidateListSelectionStyle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSelectionStyle(::core::mem::transmute_copy(&ptfselectionstyle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnKeyDown<Impl: ITfIntegratableCandidateListUIElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnKeyDown(&*(&wparam as *const <super::super::Foundation::WPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::WPARAM as ::windows::core::DefaultType>::DefaultType), &*(&lparam as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfeaten)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowCandidateNumbers<Impl: ITfIntegratableCandidateListUIElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfshow: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowCandidateNumbers(::core::mem::transmute_copy(&pfshow)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FinalizeExactCompositionString<Impl: ITfIntegratableCandidateListUIElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FinalizeExactCompositionString() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfIntegratableCandidateListUIElement>, ::windows::core::GetTrustLevel, SetIntegrationStyle::<Impl, OFFSET>, GetSelectionStyle::<Impl, OFFSET>, OnKeyDown::<Impl, OFFSET>, ShowCandidateNumbers::<Impl, OFFSET>, FinalizeExactCompositionString::<Impl, OFFSET>)
    }
}
pub trait ITfKeyEventSinkImpl: Sized {
    fn OnSetFocus();
    fn OnTestKeyDown();
    fn OnTestKeyUp();
    fn OnKeyDown();
    fn OnKeyUp();
    fn OnPreservedKey();
}
impl ::windows::core::RuntimeName for ITfKeyEventSink {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfKeyEventSink";
}
impl ITfKeyEventSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfKeyEventSinkImpl, const OFFSET: isize>() -> ITfKeyEventSinkVtbl {
        unsafe extern "system" fn OnSetFocus<Impl: ITfKeyEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fforeground: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnSetFocus(&*(&fforeground as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnTestKeyDown<Impl: ITfKeyEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnTestKeyDown(
                &*(&pic as *const <ITfContext as ::windows::core::Abi>::Abi as *const <ITfContext as ::windows::core::DefaultType>::DefaultType),
                &*(&wparam as *const <super::super::Foundation::WPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::WPARAM as ::windows::core::DefaultType>::DefaultType),
                &*(&lparam as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pfeaten),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnTestKeyUp<Impl: ITfKeyEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnTestKeyUp(
                &*(&pic as *const <ITfContext as ::windows::core::Abi>::Abi as *const <ITfContext as ::windows::core::DefaultType>::DefaultType),
                &*(&wparam as *const <super::super::Foundation::WPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::WPARAM as ::windows::core::DefaultType>::DefaultType),
                &*(&lparam as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pfeaten),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnKeyDown<Impl: ITfKeyEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnKeyDown(
                &*(&pic as *const <ITfContext as ::windows::core::Abi>::Abi as *const <ITfContext as ::windows::core::DefaultType>::DefaultType),
                &*(&wparam as *const <super::super::Foundation::WPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::WPARAM as ::windows::core::DefaultType>::DefaultType),
                &*(&lparam as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pfeaten),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnKeyUp<Impl: ITfKeyEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnKeyUp(
                &*(&pic as *const <ITfContext as ::windows::core::Abi>::Abi as *const <ITfContext as ::windows::core::DefaultType>::DefaultType),
                &*(&wparam as *const <super::super::Foundation::WPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::WPARAM as ::windows::core::DefaultType>::DefaultType),
                &*(&lparam as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pfeaten),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnPreservedKey<Impl: ITfKeyEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr, rguid: *const ::windows::core::GUID, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnPreservedKey(&*(&pic as *const <ITfContext as ::windows::core::Abi>::Abi as *const <ITfContext as ::windows::core::DefaultType>::DefaultType), &*(&rguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfeaten)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfKeyEventSink>, ::windows::core::GetTrustLevel, OnSetFocus::<Impl, OFFSET>, OnTestKeyDown::<Impl, OFFSET>, OnTestKeyUp::<Impl, OFFSET>, OnKeyDown::<Impl, OFFSET>, OnKeyUp::<Impl, OFFSET>, OnPreservedKey::<Impl, OFFSET>)
    }
}
pub trait ITfKeyTraceEventSinkImpl: Sized {
    fn OnKeyTraceDown();
    fn OnKeyTraceUp();
}
impl ::windows::core::RuntimeName for ITfKeyTraceEventSink {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfKeyTraceEventSink";
}
impl ITfKeyTraceEventSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfKeyTraceEventSinkImpl, const OFFSET: isize>() -> ITfKeyTraceEventSinkVtbl {
        unsafe extern "system" fn OnKeyTraceDown<Impl: ITfKeyTraceEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnKeyTraceDown(&*(&wparam as *const <super::super::Foundation::WPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::WPARAM as ::windows::core::DefaultType>::DefaultType), &*(&lparam as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnKeyTraceUp<Impl: ITfKeyTraceEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnKeyTraceUp(&*(&wparam as *const <super::super::Foundation::WPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::WPARAM as ::windows::core::DefaultType>::DefaultType), &*(&lparam as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfKeyTraceEventSink>, ::windows::core::GetTrustLevel, OnKeyTraceDown::<Impl, OFFSET>, OnKeyTraceUp::<Impl, OFFSET>)
    }
}
pub trait ITfKeystrokeMgrImpl: Sized {
    fn AdviseKeyEventSink();
    fn UnadviseKeyEventSink();
    fn GetForeground();
    fn TestKeyDown();
    fn TestKeyUp();
    fn KeyDown();
    fn KeyUp();
    fn GetPreservedKey();
    fn IsPreservedKey();
    fn PreserveKey();
    fn UnpreserveKey();
    fn SetPreservedKeyDescription();
    fn GetPreservedKeyDescription();
    fn SimulatePreservedKey();
}
impl ::windows::core::RuntimeName for ITfKeystrokeMgr {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfKeystrokeMgr";
}
impl ITfKeystrokeMgrVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfKeystrokeMgrImpl, const OFFSET: isize>() -> ITfKeystrokeMgrVtbl {
        unsafe extern "system" fn AdviseKeyEventSink<Impl: ITfKeystrokeMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tid: u32, psink: ::windows::core::RawPtr, fforeground: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdviseKeyEventSink(tid, &*(&psink as *const <ITfKeyEventSink as ::windows::core::Abi>::Abi as *const <ITfKeyEventSink as ::windows::core::DefaultType>::DefaultType), &*(&fforeground as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnadviseKeyEventSink<Impl: ITfKeystrokeMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnadviseKeyEventSink(tid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetForeground<Impl: ITfKeystrokeMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForeground(::core::mem::transmute_copy(&pclsid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TestKeyDown<Impl: ITfKeystrokeMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TestKeyDown(&*(&wparam as *const <super::super::Foundation::WPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::WPARAM as ::windows::core::DefaultType>::DefaultType), &*(&lparam as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfeaten)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TestKeyUp<Impl: ITfKeystrokeMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TestKeyUp(&*(&wparam as *const <super::super::Foundation::WPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::WPARAM as ::windows::core::DefaultType>::DefaultType), &*(&lparam as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfeaten)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyDown<Impl: ITfKeystrokeMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyDown(&*(&wparam as *const <super::super::Foundation::WPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::WPARAM as ::windows::core::DefaultType>::DefaultType), &*(&lparam as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfeaten)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyUp<Impl: ITfKeystrokeMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyUp(&*(&wparam as *const <super::super::Foundation::WPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::WPARAM as ::windows::core::DefaultType>::DefaultType), &*(&lparam as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfeaten)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreservedKey<Impl: ITfKeystrokeMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr, pprekey: *const TF_PRESERVEDKEY, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPreservedKey(&*(&pic as *const <ITfContext as ::windows::core::Abi>::Abi as *const <ITfContext as ::windows::core::DefaultType>::DefaultType), &*(&pprekey as *const <TF_PRESERVEDKEY as ::windows::core::Abi>::Abi as *const <TF_PRESERVEDKEY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPreservedKey<Impl: ITfKeystrokeMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, pprekey: *const TF_PRESERVEDKEY, pfregistered: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPreservedKey(&*(&rguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&pprekey as *const <TF_PRESERVEDKEY as ::windows::core::Abi>::Abi as *const <TF_PRESERVEDKEY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfregistered)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreserveKey<Impl: ITfKeystrokeMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tid: u32, rguid: *const ::windows::core::GUID, prekey: *const TF_PRESERVEDKEY, pchdesc: super::super::Foundation::PWSTR, cchdesc: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreserveKey(
                tid,
                &*(&rguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&prekey as *const <TF_PRESERVEDKEY as ::windows::core::Abi>::Abi as *const <TF_PRESERVEDKEY as ::windows::core::DefaultType>::DefaultType),
                &*(&pchdesc as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cchdesc,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnpreserveKey<Impl: ITfKeystrokeMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, pprekey: *const TF_PRESERVEDKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnpreserveKey(&*(&rguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&pprekey as *const <TF_PRESERVEDKEY as ::windows::core::Abi>::Abi as *const <TF_PRESERVEDKEY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreservedKeyDescription<Impl: ITfKeystrokeMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, pchdesc: super::super::Foundation::PWSTR, cchdesc: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPreservedKeyDescription(&*(&rguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&pchdesc as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cchdesc) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreservedKeyDescription<Impl: ITfKeystrokeMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, pbstrdesc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPreservedKeyDescription(&*(&rguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbstrdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SimulatePreservedKey<Impl: ITfKeystrokeMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr, rguid: *const ::windows::core::GUID, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SimulatePreservedKey(&*(&pic as *const <ITfContext as ::windows::core::Abi>::Abi as *const <ITfContext as ::windows::core::DefaultType>::DefaultType), &*(&rguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfeaten)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITfKeystrokeMgr>,
            ::windows::core::GetTrustLevel,
            AdviseKeyEventSink::<Impl, OFFSET>,
            UnadviseKeyEventSink::<Impl, OFFSET>,
            GetForeground::<Impl, OFFSET>,
            TestKeyDown::<Impl, OFFSET>,
            TestKeyUp::<Impl, OFFSET>,
            KeyDown::<Impl, OFFSET>,
            KeyUp::<Impl, OFFSET>,
            GetPreservedKey::<Impl, OFFSET>,
            IsPreservedKey::<Impl, OFFSET>,
            PreserveKey::<Impl, OFFSET>,
            UnpreserveKey::<Impl, OFFSET>,
            SetPreservedKeyDescription::<Impl, OFFSET>,
            GetPreservedKeyDescription::<Impl, OFFSET>,
            SimulatePreservedKey::<Impl, OFFSET>,
        )
    }
}
pub trait ITfLMLatticeImpl: Sized {
    fn QueryType();
    fn EnumLatticeElements();
}
impl ::windows::core::RuntimeName for ITfLMLattice {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfLMLattice";
}
impl ITfLMLatticeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfLMLatticeImpl, const OFFSET: isize>() -> ITfLMLatticeVtbl {
        unsafe extern "system" fn QueryType<Impl: ITfLMLatticeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidtype: *const ::windows::core::GUID, pfsupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryType(&*(&rguidtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfsupported)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumLatticeElements<Impl: ITfLMLatticeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwframestart: u32, rguidtype: *const ::windows::core::GUID, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumLatticeElements(dwframestart, &*(&rguidtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfLMLattice>, ::windows::core::GetTrustLevel, QueryType::<Impl, OFFSET>, EnumLatticeElements::<Impl, OFFSET>)
    }
}
pub trait ITfLangBarEventSinkImpl: Sized {
    fn OnSetFocus();
    fn OnThreadTerminate();
    fn OnThreadItemChange();
    fn OnModalInput();
    fn ShowFloating();
    fn GetItemFloatingRect();
}
impl ::windows::core::RuntimeName for ITfLangBarEventSink {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfLangBarEventSink";
}
impl ITfLangBarEventSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfLangBarEventSinkImpl, const OFFSET: isize>() -> ITfLangBarEventSinkVtbl {
        unsafe extern "system" fn OnSetFocus<Impl: ITfLangBarEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwthreadid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnSetFocus(dwthreadid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnThreadTerminate<Impl: ITfLangBarEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwthreadid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnThreadTerminate(dwthreadid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnThreadItemChange<Impl: ITfLangBarEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwthreadid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnThreadItemChange(dwthreadid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnModalInput<Impl: ITfLangBarEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwthreadid: u32, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnModalInput(dwthreadid, umsg, &*(&wparam as *const <super::super::Foundation::WPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::WPARAM as ::windows::core::DefaultType>::DefaultType), &*(&lparam as *const <super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowFloating<Impl: ITfLangBarEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowFloating(dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemFloatingRect<Impl: ITfLangBarEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwthreadid: u32, rguid: *const ::windows::core::GUID, prc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemFloatingRect(dwthreadid, &*(&rguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&prc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfLangBarEventSink>, ::windows::core::GetTrustLevel, OnSetFocus::<Impl, OFFSET>, OnThreadTerminate::<Impl, OFFSET>, OnThreadItemChange::<Impl, OFFSET>, OnModalInput::<Impl, OFFSET>, ShowFloating::<Impl, OFFSET>, GetItemFloatingRect::<Impl, OFFSET>)
    }
}
pub trait ITfLangBarItemImpl: Sized {
    fn GetInfo();
    fn GetStatus();
    fn Show();
    fn GetTooltipString();
}
impl ::windows::core::RuntimeName for ITfLangBarItem {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfLangBarItem";
}
impl ITfLangBarItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfLangBarItemImpl, const OFFSET: isize>() -> ITfLangBarItemVtbl {
        unsafe extern "system" fn GetInfo<Impl: ITfLangBarItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *mut TF_LANGBARITEMINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInfo(::core::mem::transmute_copy(&pinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: ITfLangBarItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus(::core::mem::transmute_copy(&pdwstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Show<Impl: ITfLangBarItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Show(&*(&fshow as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTooltipString<Impl: ITfLangBarItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtooltip: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTooltipString(::core::mem::transmute_copy(&pbstrtooltip)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfLangBarItem>, ::windows::core::GetTrustLevel, GetInfo::<Impl, OFFSET>, GetStatus::<Impl, OFFSET>, Show::<Impl, OFFSET>, GetTooltipString::<Impl, OFFSET>)
    }
}
pub trait ITfLangBarItemBalloonImpl: Sized + ITfLangBarItemImpl {
    fn OnClick();
    fn GetPreferredSize();
    fn GetBalloonInfo();
}
impl ::windows::core::RuntimeName for ITfLangBarItemBalloon {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfLangBarItemBalloon";
}
impl ITfLangBarItemBalloonVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfLangBarItemBalloonImpl, const OFFSET: isize>() -> ITfLangBarItemBalloonVtbl {
        unsafe extern "system" fn OnClick<Impl: ITfLangBarItemBalloonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnClick(click, &*(&pt as *const <super::super::Foundation::POINT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::POINT as ::windows::core::DefaultType>::DefaultType), &*(&prcarea as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreferredSize<Impl: ITfLangBarItemBalloonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdefault: *const super::super::Foundation::SIZE, psz: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPreferredSize(&*(&pszdefault as *const <super::super::Foundation::SIZE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::SIZE as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&psz)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBalloonInfo<Impl: ITfLangBarItemBalloonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *mut TF_LBBALLOONINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBalloonInfo(::core::mem::transmute_copy(&pinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfLangBarItemBalloon>, ::windows::core::GetTrustLevel, OnClick::<Impl, OFFSET>, GetPreferredSize::<Impl, OFFSET>, GetBalloonInfo::<Impl, OFFSET>)
    }
}
pub trait ITfLangBarItemBitmapImpl: Sized + ITfLangBarItemImpl {
    fn OnClick();
    fn GetPreferredSize();
    fn DrawBitmap();
}
impl ::windows::core::RuntimeName for ITfLangBarItemBitmap {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfLangBarItemBitmap";
}
impl ITfLangBarItemBitmapVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfLangBarItemBitmapImpl, const OFFSET: isize>() -> ITfLangBarItemBitmapVtbl {
        unsafe extern "system" fn OnClick<Impl: ITfLangBarItemBitmapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnClick(click, &*(&pt as *const <super::super::Foundation::POINT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::POINT as ::windows::core::DefaultType>::DefaultType), &*(&prcarea as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreferredSize<Impl: ITfLangBarItemBitmapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdefault: *const super::super::Foundation::SIZE, psz: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPreferredSize(&*(&pszdefault as *const <super::super::Foundation::SIZE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::SIZE as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&psz)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawBitmap<Impl: ITfLangBarItemBitmapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmwidth: i32, bmheight: i32, dwflags: u32, phbmp: *mut super::super::Graphics::Gdi::HBITMAP, phbmpmask: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DrawBitmap(bmwidth, bmheight, dwflags, ::core::mem::transmute_copy(&phbmp), ::core::mem::transmute_copy(&phbmpmask)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfLangBarItemBitmap>, ::windows::core::GetTrustLevel, OnClick::<Impl, OFFSET>, GetPreferredSize::<Impl, OFFSET>, DrawBitmap::<Impl, OFFSET>)
    }
}
pub trait ITfLangBarItemBitmapButtonImpl: Sized + ITfLangBarItemImpl {
    fn OnClick();
    fn InitMenu();
    fn OnMenuSelect();
    fn GetPreferredSize();
    fn DrawBitmap();
    fn GetText();
}
impl ::windows::core::RuntimeName for ITfLangBarItemBitmapButton {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfLangBarItemBitmapButton";
}
impl ITfLangBarItemBitmapButtonVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfLangBarItemBitmapButtonImpl, const OFFSET: isize>() -> ITfLangBarItemBitmapButtonVtbl {
        unsafe extern "system" fn OnClick<Impl: ITfLangBarItemBitmapButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnClick(click, &*(&pt as *const <super::super::Foundation::POINT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::POINT as ::windows::core::DefaultType>::DefaultType), &*(&prcarea as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitMenu<Impl: ITfLangBarItemBitmapButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmenu: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitMenu(&*(&pmenu as *const <ITfMenu as ::windows::core::Abi>::Abi as *const <ITfMenu as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnMenuSelect<Impl: ITfLangBarItemBitmapButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnMenuSelect(wid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreferredSize<Impl: ITfLangBarItemBitmapButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdefault: *const super::super::Foundation::SIZE, psz: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPreferredSize(&*(&pszdefault as *const <super::super::Foundation::SIZE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::SIZE as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&psz)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawBitmap<Impl: ITfLangBarItemBitmapButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmwidth: i32, bmheight: i32, dwflags: u32, phbmp: *mut super::super::Graphics::Gdi::HBITMAP, phbmpmask: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DrawBitmap(bmwidth, bmheight, dwflags, ::core::mem::transmute_copy(&phbmp), ::core::mem::transmute_copy(&phbmpmask)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetText<Impl: ITfLangBarItemBitmapButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetText(::core::mem::transmute_copy(&pbstrtext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfLangBarItemBitmapButton>, ::windows::core::GetTrustLevel, OnClick::<Impl, OFFSET>, InitMenu::<Impl, OFFSET>, OnMenuSelect::<Impl, OFFSET>, GetPreferredSize::<Impl, OFFSET>, DrawBitmap::<Impl, OFFSET>, GetText::<Impl, OFFSET>)
    }
}
pub trait ITfLangBarItemButtonImpl: Sized + ITfLangBarItemImpl {
    fn OnClick();
    fn InitMenu();
    fn OnMenuSelect();
    fn GetIcon();
    fn GetText();
}
impl ::windows::core::RuntimeName for ITfLangBarItemButton {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfLangBarItemButton";
}
impl ITfLangBarItemButtonVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfLangBarItemButtonImpl, const OFFSET: isize>() -> ITfLangBarItemButtonVtbl {
        unsafe extern "system" fn OnClick<Impl: ITfLangBarItemButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnClick(click, &*(&pt as *const <super::super::Foundation::POINT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::POINT as ::windows::core::DefaultType>::DefaultType), &*(&prcarea as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitMenu<Impl: ITfLangBarItemButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmenu: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitMenu(&*(&pmenu as *const <ITfMenu as ::windows::core::Abi>::Abi as *const <ITfMenu as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnMenuSelect<Impl: ITfLangBarItemButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnMenuSelect(wid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIcon<Impl: ITfLangBarItemButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phicon: *mut super::WindowsAndMessaging::HICON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIcon(::core::mem::transmute_copy(&phicon)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetText<Impl: ITfLangBarItemButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetText(::core::mem::transmute_copy(&pbstrtext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfLangBarItemButton>, ::windows::core::GetTrustLevel, OnClick::<Impl, OFFSET>, InitMenu::<Impl, OFFSET>, OnMenuSelect::<Impl, OFFSET>, GetIcon::<Impl, OFFSET>, GetText::<Impl, OFFSET>)
    }
}
pub trait ITfLangBarItemMgrImpl: Sized {
    fn EnumItems();
    fn GetItem();
    fn AddItem();
    fn RemoveItem();
    fn AdviseItemSink();
    fn UnadviseItemSink();
    fn GetItemFloatingRect();
    fn GetItemsStatus();
    fn GetItemNum();
    fn GetItems();
    fn AdviseItemsSink();
    fn UnadviseItemsSink();
}
impl ::windows::core::RuntimeName for ITfLangBarItemMgr {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfLangBarItemMgr";
}
impl ITfLangBarItemMgrVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfLangBarItemMgrImpl, const OFFSET: isize>() -> ITfLangBarItemMgrVtbl {
        unsafe extern "system" fn EnumItems<Impl: ITfLangBarItemMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumItems(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItem<Impl: ITfLangBarItemMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItem(&*(&rguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppitem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddItem<Impl: ITfLangBarItemMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddItem(&*(&punk as *const <ITfLangBarItem as ::windows::core::Abi>::Abi as *const <ITfLangBarItem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveItem<Impl: ITfLangBarItemMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveItem(&*(&punk as *const <ITfLangBarItem as ::windows::core::Abi>::Abi as *const <ITfLangBarItem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdviseItemSink<Impl: ITfLangBarItemMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: ::windows::core::RawPtr, pdwcookie: *mut u32, rguiditem: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdviseItemSink(&*(&punk as *const <ITfLangBarItemSink as ::windows::core::Abi>::Abi as *const <ITfLangBarItemSink as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwcookie), &*(&rguiditem as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnadviseItemSink<Impl: ITfLangBarItemMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnadviseItemSink(dwcookie) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemFloatingRect<Impl: ITfLangBarItemMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwthreadid: u32, rguid: *const ::windows::core::GUID, prc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemFloatingRect(dwthreadid, &*(&rguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&prc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemsStatus<Impl: ITfLangBarItemMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, prgguid: *const ::windows::core::GUID, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemsStatus(ulcount, &*(&prgguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemNum<Impl: ITfLangBarItemMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemNum(::core::mem::transmute_copy(&pulcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItems<Impl: ITfLangBarItemMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, ppitem: *mut ::windows::core::RawPtr, pinfo: *mut TF_LANGBARITEMINFO, pdwstatus: *mut u32, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItems(ulcount, ::core::mem::transmute_copy(&ppitem), ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&pdwstatus), pcfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdviseItemsSink<Impl: ITfLangBarItemMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, ppunk: *const ::windows::core::RawPtr, pguiditem: *const ::windows::core::GUID, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdviseItemsSink(ulcount, &*(&ppunk as *const <ITfLangBarItemSink as ::windows::core::Abi>::Abi as *const <ITfLangBarItemSink as ::windows::core::DefaultType>::DefaultType), &*(&pguiditem as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwcookie)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnadviseItemsSink<Impl: ITfLangBarItemMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, pdwcookie: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnadviseItemsSink(ulcount, pdwcookie) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITfLangBarItemMgr>,
            ::windows::core::GetTrustLevel,
            EnumItems::<Impl, OFFSET>,
            GetItem::<Impl, OFFSET>,
            AddItem::<Impl, OFFSET>,
            RemoveItem::<Impl, OFFSET>,
            AdviseItemSink::<Impl, OFFSET>,
            UnadviseItemSink::<Impl, OFFSET>,
            GetItemFloatingRect::<Impl, OFFSET>,
            GetItemsStatus::<Impl, OFFSET>,
            GetItemNum::<Impl, OFFSET>,
            GetItems::<Impl, OFFSET>,
            AdviseItemsSink::<Impl, OFFSET>,
            UnadviseItemsSink::<Impl, OFFSET>,
        )
    }
}
pub trait ITfLangBarItemSinkImpl: Sized {
    fn OnUpdate();
}
impl ::windows::core::RuntimeName for ITfLangBarItemSink {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfLangBarItemSink";
}
impl ITfLangBarItemSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfLangBarItemSinkImpl, const OFFSET: isize>() -> ITfLangBarItemSinkVtbl {
        unsafe extern "system" fn OnUpdate<Impl: ITfLangBarItemSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnUpdate(dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfLangBarItemSink>, ::windows::core::GetTrustLevel, OnUpdate::<Impl, OFFSET>)
    }
}
pub trait ITfLangBarMgrImpl: Sized {
    fn AdviseEventSink();
    fn UnadviseEventSink();
    fn GetThreadMarshalInterface();
    fn GetThreadLangBarItemMgr();
    fn GetInputProcessorProfiles();
    fn RestoreLastFocus();
    fn SetModalInput();
    fn ShowFloating();
    fn GetShowFloatingStatus();
}
impl ::windows::core::RuntimeName for ITfLangBarMgr {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfLangBarMgr";
}
impl ITfLangBarMgrVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfLangBarMgrImpl, const OFFSET: isize>() -> ITfLangBarMgrVtbl {
        unsafe extern "system" fn AdviseEventSink<Impl: ITfLangBarMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psink: ::windows::core::RawPtr, hwnd: super::super::Foundation::HWND, dwflags: u32, pdwcookie: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdviseEventSink(&*(&psink as *const <ITfLangBarEventSink as ::windows::core::Abi>::Abi as *const <ITfLangBarEventSink as ::windows::core::DefaultType>::DefaultType), &*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), dwflags, pdwcookie) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnadviseEventSink<Impl: ITfLangBarMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnadviseEventSink(dwcookie) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThreadMarshalInterface<Impl: ITfLangBarMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwthreadid: u32, dwtype: u32, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetThreadMarshalInterface(dwthreadid, dwtype, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThreadLangBarItemMgr<Impl: ITfLangBarMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwthreadid: u32, pplbi: *mut ::windows::core::RawPtr, pdwthreadid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetThreadLangBarItemMgr(dwthreadid, ::core::mem::transmute_copy(&pplbi), ::core::mem::transmute_copy(&pdwthreadid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputProcessorProfiles<Impl: ITfLangBarMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwthreadid: u32, ppaip: *mut ::windows::core::RawPtr, pdwthreadid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputProcessorProfiles(dwthreadid, ::core::mem::transmute_copy(&ppaip), ::core::mem::transmute_copy(&pdwthreadid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RestoreLastFocus<Impl: ITfLangBarMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwthreadid: *mut u32, fprev: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RestoreLastFocus(::core::mem::transmute_copy(&pdwthreadid), &*(&fprev as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetModalInput<Impl: ITfLangBarMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psink: ::windows::core::RawPtr, dwthreadid: u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetModalInput(&*(&psink as *const <ITfLangBarEventSink as ::windows::core::Abi>::Abi as *const <ITfLangBarEventSink as ::windows::core::DefaultType>::DefaultType), dwthreadid, dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowFloating<Impl: ITfLangBarMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowFloating(dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetShowFloatingStatus<Impl: ITfLangBarMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetShowFloatingStatus(::core::mem::transmute_copy(&pdwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITfLangBarMgr>,
            ::windows::core::GetTrustLevel,
            AdviseEventSink::<Impl, OFFSET>,
            UnadviseEventSink::<Impl, OFFSET>,
            GetThreadMarshalInterface::<Impl, OFFSET>,
            GetThreadLangBarItemMgr::<Impl, OFFSET>,
            GetInputProcessorProfiles::<Impl, OFFSET>,
            RestoreLastFocus::<Impl, OFFSET>,
            SetModalInput::<Impl, OFFSET>,
            ShowFloating::<Impl, OFFSET>,
            GetShowFloatingStatus::<Impl, OFFSET>,
        )
    }
}
pub trait ITfLanguageProfileNotifySinkImpl: Sized {
    fn OnLanguageChange();
    fn OnLanguageChanged();
}
impl ::windows::core::RuntimeName for ITfLanguageProfileNotifySink {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfLanguageProfileNotifySink";
}
impl ITfLanguageProfileNotifySinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfLanguageProfileNotifySinkImpl, const OFFSET: isize>() -> ITfLanguageProfileNotifySinkVtbl {
        unsafe extern "system" fn OnLanguageChange<Impl: ITfLanguageProfileNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16, pfaccept: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnLanguageChange(langid, ::core::mem::transmute_copy(&pfaccept)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnLanguageChanged<Impl: ITfLanguageProfileNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnLanguageChanged() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfLanguageProfileNotifySink>, ::windows::core::GetTrustLevel, OnLanguageChange::<Impl, OFFSET>, OnLanguageChanged::<Impl, OFFSET>)
    }
}
pub trait ITfMSAAControlImpl: Sized {
    fn SystemEnableMSAA();
    fn SystemDisableMSAA();
}
impl ::windows::core::RuntimeName for ITfMSAAControl {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfMSAAControl";
}
impl ITfMSAAControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfMSAAControlImpl, const OFFSET: isize>() -> ITfMSAAControlVtbl {
        unsafe extern "system" fn SystemEnableMSAA<Impl: ITfMSAAControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemEnableMSAA() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemDisableMSAA<Impl: ITfMSAAControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemDisableMSAA() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfMSAAControl>, ::windows::core::GetTrustLevel, SystemEnableMSAA::<Impl, OFFSET>, SystemDisableMSAA::<Impl, OFFSET>)
    }
}
pub trait ITfMenuImpl: Sized {
    fn AddMenuItem();
}
impl ::windows::core::RuntimeName for ITfMenu {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfMenu";
}
impl ITfMenuVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfMenuImpl, const OFFSET: isize>() -> ITfMenuVtbl {
        unsafe extern "system" fn AddMenuItem<Impl: ITfMenuImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uid: u32, dwflags: u32, hbmp: super::super::Graphics::Gdi::HBITMAP, hbmpmask: super::super::Graphics::Gdi::HBITMAP, pch: super::super::Foundation::PWSTR, cch: u32, ppmenu: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddMenuItem(
                uid,
                dwflags,
                &*(&hbmp as *const <super::super::Graphics::Gdi::HBITMAP as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Gdi::HBITMAP as ::windows::core::DefaultType>::DefaultType),
                &*(&hbmpmask as *const <super::super::Graphics::Gdi::HBITMAP as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Gdi::HBITMAP as ::windows::core::DefaultType>::DefaultType),
                &*(&pch as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cch,
                &*(&ppmenu as *const <ITfMenu as ::windows::core::Abi>::Abi as *const <ITfMenu as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfMenu>, ::windows::core::GetTrustLevel, AddMenuItem::<Impl, OFFSET>)
    }
}
pub trait ITfMessagePumpImpl: Sized {
    fn PeekMessageA();
    fn GetMessageA();
    fn PeekMessageW();
    fn GetMessageW();
}
impl ::windows::core::RuntimeName for ITfMessagePump {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfMessagePump";
}
impl ITfMessagePumpVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfMessagePumpImpl, const OFFSET: isize>() -> ITfMessagePumpVtbl {
        unsafe extern "system" fn PeekMessageA<Impl: ITfMessagePumpImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekMessageA(::core::mem::transmute_copy(&pmsg), &*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), wmsgfiltermin, wmsgfiltermax, wremovemsg, ::core::mem::transmute_copy(&pfresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMessageA<Impl: ITfMessagePumpImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMessageA(::core::mem::transmute_copy(&pmsg), &*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), wmsgfiltermin, wmsgfiltermax, ::core::mem::transmute_copy(&pfresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekMessageW<Impl: ITfMessagePumpImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekMessageW(::core::mem::transmute_copy(&pmsg), &*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), wmsgfiltermin, wmsgfiltermax, wremovemsg, ::core::mem::transmute_copy(&pfresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMessageW<Impl: ITfMessagePumpImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMessageW(::core::mem::transmute_copy(&pmsg), &*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), wmsgfiltermin, wmsgfiltermax, ::core::mem::transmute_copy(&pfresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfMessagePump>, ::windows::core::GetTrustLevel, PeekMessageA::<Impl, OFFSET>, GetMessageA::<Impl, OFFSET>, PeekMessageW::<Impl, OFFSET>, GetMessageW::<Impl, OFFSET>)
    }
}
pub trait ITfMouseSinkImpl: Sized {
    fn OnMouseEvent();
}
impl ::windows::core::RuntimeName for ITfMouseSink {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfMouseSink";
}
impl ITfMouseSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfMouseSinkImpl, const OFFSET: isize>() -> ITfMouseSinkVtbl {
        unsafe extern "system" fn OnMouseEvent<Impl: ITfMouseSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uedge: u32, uquadrant: u32, dwbtnstatus: u32, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnMouseEvent(uedge, uquadrant, dwbtnstatus, ::core::mem::transmute_copy(&pfeaten)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfMouseSink>, ::windows::core::GetTrustLevel, OnMouseEvent::<Impl, OFFSET>)
    }
}
pub trait ITfMouseTrackerImpl: Sized {
    fn AdviseMouseSink();
    fn UnadviseMouseSink();
}
impl ::windows::core::RuntimeName for ITfMouseTracker {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfMouseTracker";
}
impl ITfMouseTrackerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfMouseTrackerImpl, const OFFSET: isize>() -> ITfMouseTrackerVtbl {
        unsafe extern "system" fn AdviseMouseSink<Impl: ITfMouseTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, range: ::windows::core::RawPtr, psink: ::windows::core::RawPtr, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdviseMouseSink(&*(&range as *const <ITfRange as ::windows::core::Abi>::Abi as *const <ITfRange as ::windows::core::DefaultType>::DefaultType), &*(&psink as *const <ITfMouseSink as ::windows::core::Abi>::Abi as *const <ITfMouseSink as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwcookie)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnadviseMouseSink<Impl: ITfMouseTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnadviseMouseSink(dwcookie) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfMouseTracker>, ::windows::core::GetTrustLevel, AdviseMouseSink::<Impl, OFFSET>, UnadviseMouseSink::<Impl, OFFSET>)
    }
}
pub trait ITfMouseTrackerACPImpl: Sized {
    fn AdviseMouseSink();
    fn UnadviseMouseSink();
}
impl ::windows::core::RuntimeName for ITfMouseTrackerACP {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfMouseTrackerACP";
}
impl ITfMouseTrackerACPVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfMouseTrackerACPImpl, const OFFSET: isize>() -> ITfMouseTrackerACPVtbl {
        unsafe extern "system" fn AdviseMouseSink<Impl: ITfMouseTrackerACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, range: ::windows::core::RawPtr, psink: ::windows::core::RawPtr, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdviseMouseSink(&*(&range as *const <ITfRangeACP as ::windows::core::Abi>::Abi as *const <ITfRangeACP as ::windows::core::DefaultType>::DefaultType), &*(&psink as *const <ITfMouseSink as ::windows::core::Abi>::Abi as *const <ITfMouseSink as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwcookie)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnadviseMouseSink<Impl: ITfMouseTrackerACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnadviseMouseSink(dwcookie) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfMouseTrackerACP>, ::windows::core::GetTrustLevel, AdviseMouseSink::<Impl, OFFSET>, UnadviseMouseSink::<Impl, OFFSET>)
    }
}
pub trait ITfPersistentPropertyLoaderACPImpl: Sized {
    fn LoadProperty();
}
impl ::windows::core::RuntimeName for ITfPersistentPropertyLoaderACP {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfPersistentPropertyLoaderACP";
}
impl ITfPersistentPropertyLoaderACPVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfPersistentPropertyLoaderACPImpl, const OFFSET: isize>() -> ITfPersistentPropertyLoaderACPVtbl {
        unsafe extern "system" fn LoadProperty<Impl: ITfPersistentPropertyLoaderACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadProperty(&*(&phdr as *const <TF_PERSISTENT_PROPERTY_HEADER_ACP as ::windows::core::Abi>::Abi as *const <TF_PERSISTENT_PROPERTY_HEADER_ACP as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfPersistentPropertyLoaderACP>, ::windows::core::GetTrustLevel, LoadProperty::<Impl, OFFSET>)
    }
}
pub trait ITfPreservedKeyNotifySinkImpl: Sized {
    fn OnUpdated();
}
impl ::windows::core::RuntimeName for ITfPreservedKeyNotifySink {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfPreservedKeyNotifySink";
}
impl ITfPreservedKeyNotifySinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfPreservedKeyNotifySinkImpl, const OFFSET: isize>() -> ITfPreservedKeyNotifySinkVtbl {
        unsafe extern "system" fn OnUpdated<Impl: ITfPreservedKeyNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprekey: *const TF_PRESERVEDKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnUpdated(&*(&pprekey as *const <TF_PRESERVEDKEY as ::windows::core::Abi>::Abi as *const <TF_PRESERVEDKEY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfPreservedKeyNotifySink>, ::windows::core::GetTrustLevel, OnUpdated::<Impl, OFFSET>)
    }
}
pub trait ITfPropertyImpl: Sized + ITfReadOnlyPropertyImpl {
    fn FindRange();
    fn SetValueStore();
    fn SetValue();
    fn Clear();
}
impl ::windows::core::RuntimeName for ITfProperty {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfProperty";
}
impl ITfPropertyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfPropertyImpl, const OFFSET: isize>() -> ITfPropertyVtbl {
        unsafe extern "system" fn FindRange<Impl: ITfPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, prange: ::windows::core::RawPtr, pprange: *mut ::windows::core::RawPtr, apos: TfAnchor) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindRange(ec, &*(&prange as *const <ITfRange as ::windows::core::Abi>::Abi as *const <ITfRange as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pprange), apos) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValueStore<Impl: ITfPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, prange: ::windows::core::RawPtr, ppropstore: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetValueStore(ec, &*(&prange as *const <ITfRange as ::windows::core::Abi>::Abi as *const <ITfRange as ::windows::core::DefaultType>::DefaultType), &*(&ppropstore as *const <ITfPropertyStore as ::windows::core::Abi>::Abi as *const <ITfPropertyStore as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: ITfPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, prange: ::windows::core::RawPtr, pvarvalue: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetValue(ec, &*(&prange as *const <ITfRange as ::windows::core::Abi>::Abi as *const <ITfRange as ::windows::core::DefaultType>::DefaultType), &*(&pvarvalue as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Impl: ITfPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clear(ec, &*(&prange as *const <ITfRange as ::windows::core::Abi>::Abi as *const <ITfRange as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfProperty>, ::windows::core::GetTrustLevel, FindRange::<Impl, OFFSET>, SetValueStore::<Impl, OFFSET>, SetValue::<Impl, OFFSET>, Clear::<Impl, OFFSET>)
    }
}
pub trait ITfPropertyStoreImpl: Sized {
    fn GetType();
    fn GetDataType();
    fn GetData();
    fn OnTextUpdated();
    fn Shrink();
    fn Divide();
    fn Clone();
    fn GetPropertyRangeCreator();
    fn Serialize();
}
impl ::windows::core::RuntimeName for ITfPropertyStore {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfPropertyStore";
}
impl ITfPropertyStoreVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfPropertyStoreImpl, const OFFSET: isize>() -> ITfPropertyStoreVtbl {
        unsafe extern "system" fn GetType<Impl: ITfPropertyStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetType(::core::mem::transmute_copy(&pguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataType<Impl: ITfPropertyStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwreserved: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDataType(::core::mem::transmute_copy(&pdwreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetData<Impl: ITfPropertyStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetData(::core::mem::transmute_copy(&pvarvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnTextUpdated<Impl: ITfPropertyStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, prangenew: ::windows::core::RawPtr, pfaccept: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnTextUpdated(dwflags, &*(&prangenew as *const <ITfRange as ::windows::core::Abi>::Abi as *const <ITfRange as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfaccept)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shrink<Impl: ITfPropertyStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prangenew: ::windows::core::RawPtr, pffree: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Shrink(&*(&prangenew as *const <ITfRange as ::windows::core::Abi>::Abi as *const <ITfRange as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pffree)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Divide<Impl: ITfPropertyStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prangethis: ::windows::core::RawPtr, prangenew: ::windows::core::RawPtr, pppropstore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Divide(&*(&prangethis as *const <ITfRange as ::windows::core::Abi>::Abi as *const <ITfRange as ::windows::core::DefaultType>::DefaultType), &*(&prangenew as *const <ITfRange as ::windows::core::Abi>::Abi as *const <ITfRange as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pppropstore)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: ITfPropertyStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropstore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppropstore)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyRangeCreator<Impl: ITfPropertyStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertyRangeCreator(::core::mem::transmute_copy(&pclsid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Serialize<Impl: ITfPropertyStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, pcb: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Serialize(&*(&pstream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcb)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITfPropertyStore>,
            ::windows::core::GetTrustLevel,
            GetType::<Impl, OFFSET>,
            GetDataType::<Impl, OFFSET>,
            GetData::<Impl, OFFSET>,
            OnTextUpdated::<Impl, OFFSET>,
            Shrink::<Impl, OFFSET>,
            Divide::<Impl, OFFSET>,
            Clone::<Impl, OFFSET>,
            GetPropertyRangeCreator::<Impl, OFFSET>,
            Serialize::<Impl, OFFSET>,
        )
    }
}
pub trait ITfQueryEmbeddedImpl: Sized {
    fn QueryInsertEmbedded();
}
impl ::windows::core::RuntimeName for ITfQueryEmbedded {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfQueryEmbedded";
}
impl ITfQueryEmbeddedVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfQueryEmbeddedImpl, const OFFSET: isize>() -> ITfQueryEmbeddedVtbl {
        unsafe extern "system" fn QueryInsertEmbedded<Impl: ITfQueryEmbeddedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidservice: *const ::windows::core::GUID, pformatetc: *const super::super::System::Com::FORMATETC, pfinsertable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryInsertEmbedded(&*(&pguidservice as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&pformatetc as *const <super::super::System::Com::FORMATETC as ::windows::core::Abi>::Abi as *const <super::super::System::Com::FORMATETC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfinsertable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfQueryEmbedded>, ::windows::core::GetTrustLevel, QueryInsertEmbedded::<Impl, OFFSET>)
    }
}
pub trait ITfRangeImpl: Sized {
    fn GetText();
    fn SetText();
    fn GetFormattedText();
    fn GetEmbedded();
    fn InsertEmbedded();
    fn ShiftStart();
    fn ShiftEnd();
    fn ShiftStartToRange();
    fn ShiftEndToRange();
    fn ShiftStartRegion();
    fn ShiftEndRegion();
    fn IsEmpty();
    fn Collapse();
    fn IsEqualStart();
    fn IsEqualEnd();
    fn CompareStart();
    fn CompareEnd();
    fn AdjustForInsert();
    fn GetGravity();
    fn SetGravity();
    fn Clone();
    fn GetContext();
}
impl ::windows::core::RuntimeName for ITfRange {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfRange";
}
impl ITfRangeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfRangeImpl, const OFFSET: isize>() -> ITfRangeVtbl {
        unsafe extern "system" fn GetText<Impl: ITfRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, dwflags: u32, pchtext: super::super::Foundation::PWSTR, cchmax: u32, pcch: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetText(ec, dwflags, ::core::mem::transmute_copy(&pchtext), cchmax, ::core::mem::transmute_copy(&pcch)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetText<Impl: ITfRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, dwflags: u32, pchtext: super::super::Foundation::PWSTR, cch: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetText(ec, dwflags, &*(&pchtext as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cch) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormattedText<Impl: ITfRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, ppdataobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFormattedText(ec, ::core::mem::transmute_copy(&ppdataobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEmbedded<Impl: ITfRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, rguidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEmbedded(ec, &*(&rguidservice as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertEmbedded<Impl: ITfRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, dwflags: u32, pdataobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InsertEmbedded(ec, dwflags, &*(&pdataobject as *const <super::super::System::Com::IDataObject as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IDataObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShiftStart<Impl: ITfRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShiftStart(ec, cchreq, ::core::mem::transmute_copy(&pcch), &*(&phalt as *const <TF_HALTCOND as ::windows::core::Abi>::Abi as *const <TF_HALTCOND as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShiftEnd<Impl: ITfRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShiftEnd(ec, cchreq, ::core::mem::transmute_copy(&pcch), &*(&phalt as *const <TF_HALTCOND as ::windows::core::Abi>::Abi as *const <TF_HALTCOND as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShiftStartToRange<Impl: ITfRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, prange: ::windows::core::RawPtr, apos: TfAnchor) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShiftStartToRange(ec, &*(&prange as *const <ITfRange as ::windows::core::Abi>::Abi as *const <ITfRange as ::windows::core::DefaultType>::DefaultType), apos) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShiftEndToRange<Impl: ITfRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, prange: ::windows::core::RawPtr, apos: TfAnchor) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShiftEndToRange(ec, &*(&prange as *const <ITfRange as ::windows::core::Abi>::Abi as *const <ITfRange as ::windows::core::DefaultType>::DefaultType), apos) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShiftStartRegion<Impl: ITfRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, dir: TfShiftDir, pfnoregion: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShiftStartRegion(ec, dir, ::core::mem::transmute_copy(&pfnoregion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShiftEndRegion<Impl: ITfRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, dir: TfShiftDir, pfnoregion: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShiftEndRegion(ec, dir, ::core::mem::transmute_copy(&pfnoregion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEmpty<Impl: ITfRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, pfempty: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEmpty(ec, ::core::mem::transmute_copy(&pfempty)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Collapse<Impl: ITfRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, apos: TfAnchor) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Collapse(ec, apos) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqualStart<Impl: ITfRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, pwith: ::windows::core::RawPtr, apos: TfAnchor, pfequal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEqualStart(ec, &*(&pwith as *const <ITfRange as ::windows::core::Abi>::Abi as *const <ITfRange as ::windows::core::DefaultType>::DefaultType), apos, ::core::mem::transmute_copy(&pfequal)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqualEnd<Impl: ITfRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, pwith: ::windows::core::RawPtr, apos: TfAnchor, pfequal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEqualEnd(ec, &*(&pwith as *const <ITfRange as ::windows::core::Abi>::Abi as *const <ITfRange as ::windows::core::DefaultType>::DefaultType), apos, ::core::mem::transmute_copy(&pfequal)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareStart<Impl: ITfRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, pwith: ::windows::core::RawPtr, apos: TfAnchor, plresult: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompareStart(ec, &*(&pwith as *const <ITfRange as ::windows::core::Abi>::Abi as *const <ITfRange as ::windows::core::DefaultType>::DefaultType), apos, ::core::mem::transmute_copy(&plresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareEnd<Impl: ITfRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, pwith: ::windows::core::RawPtr, apos: TfAnchor, plresult: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompareEnd(ec, &*(&pwith as *const <ITfRange as ::windows::core::Abi>::Abi as *const <ITfRange as ::windows::core::DefaultType>::DefaultType), apos, ::core::mem::transmute_copy(&plresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdjustForInsert<Impl: ITfRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, cchinsert: u32, pfinsertok: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdjustForInsert(ec, cchinsert, ::core::mem::transmute_copy(&pfinsertok)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGravity<Impl: ITfRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgstart: *mut TfGravity, pgend: *mut TfGravity) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGravity(::core::mem::transmute_copy(&pgstart), ::core::mem::transmute_copy(&pgend)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGravity<Impl: ITfRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, gstart: TfGravity, gend: TfGravity) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetGravity(ec, gstart, gend) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: ITfRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppclone)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContext<Impl: ITfRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContext(::core::mem::transmute_copy(&ppcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITfRange>,
            ::windows::core::GetTrustLevel,
            GetText::<Impl, OFFSET>,
            SetText::<Impl, OFFSET>,
            GetFormattedText::<Impl, OFFSET>,
            GetEmbedded::<Impl, OFFSET>,
            InsertEmbedded::<Impl, OFFSET>,
            ShiftStart::<Impl, OFFSET>,
            ShiftEnd::<Impl, OFFSET>,
            ShiftStartToRange::<Impl, OFFSET>,
            ShiftEndToRange::<Impl, OFFSET>,
            ShiftStartRegion::<Impl, OFFSET>,
            ShiftEndRegion::<Impl, OFFSET>,
            IsEmpty::<Impl, OFFSET>,
            Collapse::<Impl, OFFSET>,
            IsEqualStart::<Impl, OFFSET>,
            IsEqualEnd::<Impl, OFFSET>,
            CompareStart::<Impl, OFFSET>,
            CompareEnd::<Impl, OFFSET>,
            AdjustForInsert::<Impl, OFFSET>,
            GetGravity::<Impl, OFFSET>,
            SetGravity::<Impl, OFFSET>,
            Clone::<Impl, OFFSET>,
            GetContext::<Impl, OFFSET>,
        )
    }
}
pub trait ITfRangeACPImpl: Sized + ITfRangeImpl {
    fn GetExtent();
    fn SetExtent();
}
impl ::windows::core::RuntimeName for ITfRangeACP {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfRangeACP";
}
impl ITfRangeACPVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfRangeACPImpl, const OFFSET: isize>() -> ITfRangeACPVtbl {
        unsafe extern "system" fn GetExtent<Impl: ITfRangeACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pacpanchor: *mut i32, pcch: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetExtent(::core::mem::transmute_copy(&pacpanchor), ::core::mem::transmute_copy(&pcch)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtent<Impl: ITfRangeACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpanchor: i32, cch: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetExtent(acpanchor, cch) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfRangeACP>, ::windows::core::GetTrustLevel, GetExtent::<Impl, OFFSET>, SetExtent::<Impl, OFFSET>)
    }
}
pub trait ITfRangeBackupImpl: Sized {
    fn Restore();
}
impl ::windows::core::RuntimeName for ITfRangeBackup {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfRangeBackup";
}
impl ITfRangeBackupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfRangeBackupImpl, const OFFSET: isize>() -> ITfRangeBackupVtbl {
        unsafe extern "system" fn Restore<Impl: ITfRangeBackupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Restore(ec, &*(&prange as *const <ITfRange as ::windows::core::Abi>::Abi as *const <ITfRange as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfRangeBackup>, ::windows::core::GetTrustLevel, Restore::<Impl, OFFSET>)
    }
}
pub trait ITfReadOnlyPropertyImpl: Sized {
    fn GetType();
    fn EnumRanges();
    fn GetValue();
    fn GetContext();
}
impl ::windows::core::RuntimeName for ITfReadOnlyProperty {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfReadOnlyProperty";
}
impl ITfReadOnlyPropertyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfReadOnlyPropertyImpl, const OFFSET: isize>() -> ITfReadOnlyPropertyVtbl {
        unsafe extern "system" fn GetType<Impl: ITfReadOnlyPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetType(::core::mem::transmute_copy(&pguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumRanges<Impl: ITfReadOnlyPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, ppenum: *mut ::windows::core::RawPtr, ptargetrange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumRanges(ec, ::core::mem::transmute_copy(&ppenum), &*(&ptargetrange as *const <ITfRange as ::windows::core::Abi>::Abi as *const <ITfRange as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Impl: ITfReadOnlyPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, prange: ::windows::core::RawPtr, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetValue(ec, &*(&prange as *const <ITfRange as ::windows::core::Abi>::Abi as *const <ITfRange as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvarvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContext<Impl: ITfReadOnlyPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContext(::core::mem::transmute_copy(&ppcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfReadOnlyProperty>, ::windows::core::GetTrustLevel, GetType::<Impl, OFFSET>, EnumRanges::<Impl, OFFSET>, GetValue::<Impl, OFFSET>, GetContext::<Impl, OFFSET>)
    }
}
pub trait ITfReadingInformationUIElementImpl: Sized + ITfUIElementImpl {
    fn GetUpdatedFlags();
    fn GetContext();
    fn GetString();
    fn GetMaxReadingStringLength();
    fn GetErrorIndex();
    fn IsVerticalOrderPreferred();
}
impl ::windows::core::RuntimeName for ITfReadingInformationUIElement {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfReadingInformationUIElement";
}
impl ITfReadingInformationUIElementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfReadingInformationUIElementImpl, const OFFSET: isize>() -> ITfReadingInformationUIElementVtbl {
        unsafe extern "system" fn GetUpdatedFlags<Impl: ITfReadingInformationUIElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUpdatedFlags(::core::mem::transmute_copy(&pdwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContext<Impl: ITfReadingInformationUIElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppic: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContext(::core::mem::transmute_copy(&ppic)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetString<Impl: ITfReadingInformationUIElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetString(::core::mem::transmute_copy(&pstr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxReadingStringLength<Impl: ITfReadingInformationUIElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcchmax: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxReadingStringLength(::core::mem::transmute_copy(&pcchmax)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorIndex<Impl: ITfReadingInformationUIElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, perrorindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetErrorIndex(::core::mem::transmute_copy(&perrorindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsVerticalOrderPreferred<Impl: ITfReadingInformationUIElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfvertical: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsVerticalOrderPreferred(::core::mem::transmute_copy(&pfvertical)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfReadingInformationUIElement>, ::windows::core::GetTrustLevel, GetUpdatedFlags::<Impl, OFFSET>, GetContext::<Impl, OFFSET>, GetString::<Impl, OFFSET>, GetMaxReadingStringLength::<Impl, OFFSET>, GetErrorIndex::<Impl, OFFSET>, IsVerticalOrderPreferred::<Impl, OFFSET>)
    }
}
pub trait ITfReverseConversionImpl: Sized {
    fn DoReverseConversion();
}
impl ::windows::core::RuntimeName for ITfReverseConversion {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfReverseConversion";
}
impl ITfReverseConversionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfReverseConversionImpl, const OFFSET: isize>() -> ITfReverseConversionVtbl {
        unsafe extern "system" fn DoReverseConversion<Impl: ITfReverseConversionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpstr: super::super::Foundation::PWSTR, pplist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DoReverseConversion(&*(&lpstr as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pplist)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfReverseConversion>, ::windows::core::GetTrustLevel, DoReverseConversion::<Impl, OFFSET>)
    }
}
pub trait ITfReverseConversionListImpl: Sized {
    fn GetLength();
    fn GetString();
}
impl ::windows::core::RuntimeName for ITfReverseConversionList {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfReverseConversionList";
}
impl ITfReverseConversionListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfReverseConversionListImpl, const OFFSET: isize>() -> ITfReverseConversionListVtbl {
        unsafe extern "system" fn GetLength<Impl: ITfReverseConversionListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLength(::core::mem::transmute_copy(&puindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetString<Impl: ITfReverseConversionListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, pbstr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetString(uindex, ::core::mem::transmute_copy(&pbstr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfReverseConversionList>, ::windows::core::GetTrustLevel, GetLength::<Impl, OFFSET>, GetString::<Impl, OFFSET>)
    }
}
pub trait ITfReverseConversionMgrImpl: Sized {
    fn GetReverseConversion();
}
impl ::windows::core::RuntimeName for ITfReverseConversionMgr {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfReverseConversionMgr";
}
impl ITfReverseConversionMgrVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfReverseConversionMgrImpl, const OFFSET: isize>() -> ITfReverseConversionMgrVtbl {
        unsafe extern "system" fn GetReverseConversion<Impl: ITfReverseConversionMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16, guidprofile: *const ::windows::core::GUID, dwflag: u32, ppreverseconversion: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetReverseConversion(langid, &*(&guidprofile as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwflag, ::core::mem::transmute_copy(&ppreverseconversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfReverseConversionMgr>, ::windows::core::GetTrustLevel, GetReverseConversion::<Impl, OFFSET>)
    }
}
pub trait ITfSourceImpl: Sized {
    fn AdviseSink();
    fn UnadviseSink();
}
impl ::windows::core::RuntimeName for ITfSource {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfSource";
}
impl ITfSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfSourceImpl, const OFFSET: isize>() -> ITfSourceVtbl {
        unsafe extern "system" fn AdviseSink<Impl: ITfSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdviseSink(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&punk as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwcookie)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnadviseSink<Impl: ITfSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnadviseSink(dwcookie) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfSource>, ::windows::core::GetTrustLevel, AdviseSink::<Impl, OFFSET>, UnadviseSink::<Impl, OFFSET>)
    }
}
pub trait ITfSourceSingleImpl: Sized {
    fn AdviseSingleSink();
    fn UnadviseSingleSink();
}
impl ::windows::core::RuntimeName for ITfSourceSingle {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfSourceSingle";
}
impl ITfSourceSingleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfSourceSingleImpl, const OFFSET: isize>() -> ITfSourceSingleVtbl {
        unsafe extern "system" fn AdviseSingleSink<Impl: ITfSourceSingleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tid: u32, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdviseSingleSink(tid, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&punk as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnadviseSingleSink<Impl: ITfSourceSingleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tid: u32, riid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnadviseSingleSink(tid, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfSourceSingle>, ::windows::core::GetTrustLevel, AdviseSingleSink::<Impl, OFFSET>, UnadviseSingleSink::<Impl, OFFSET>)
    }
}
pub trait ITfSpeechUIServerImpl: Sized {
    fn Initialize();
    fn ShowUI();
    fn UpdateBalloon();
}
impl ::windows::core::RuntimeName for ITfSpeechUIServer {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfSpeechUIServer";
}
impl ITfSpeechUIServerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfSpeechUIServerImpl, const OFFSET: isize>() -> ITfSpeechUIServerVtbl {
        unsafe extern "system" fn Initialize<Impl: ITfSpeechUIServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowUI<Impl: ITfSpeechUIServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowUI(&*(&fshow as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateBalloon<Impl: ITfSpeechUIServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, style: TfLBBalloonStyle, pch: super::super::Foundation::PWSTR, cch: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateBalloon(style, &*(&pch as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cch) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfSpeechUIServer>, ::windows::core::GetTrustLevel, Initialize::<Impl, OFFSET>, ShowUI::<Impl, OFFSET>, UpdateBalloon::<Impl, OFFSET>)
    }
}
pub trait ITfStatusSinkImpl: Sized {
    fn OnStatusChange();
}
impl ::windows::core::RuntimeName for ITfStatusSink {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfStatusSink";
}
impl ITfStatusSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfStatusSinkImpl, const OFFSET: isize>() -> ITfStatusSinkVtbl {
        unsafe extern "system" fn OnStatusChange<Impl: ITfStatusSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnStatusChange(&*(&pic as *const <ITfContext as ::windows::core::Abi>::Abi as *const <ITfContext as ::windows::core::DefaultType>::DefaultType), dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfStatusSink>, ::windows::core::GetTrustLevel, OnStatusChange::<Impl, OFFSET>)
    }
}
pub trait ITfSystemDeviceTypeLangBarItemImpl: Sized {
    fn SetIconMode();
    fn GetIconMode();
}
impl ::windows::core::RuntimeName for ITfSystemDeviceTypeLangBarItem {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfSystemDeviceTypeLangBarItem";
}
impl ITfSystemDeviceTypeLangBarItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfSystemDeviceTypeLangBarItemImpl, const OFFSET: isize>() -> ITfSystemDeviceTypeLangBarItemVtbl {
        unsafe extern "system" fn SetIconMode<Impl: ITfSystemDeviceTypeLangBarItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: LANG_BAR_ITEM_ICON_MODE_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetIconMode(dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIconMode<Impl: ITfSystemDeviceTypeLangBarItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIconMode(::core::mem::transmute_copy(&pdwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfSystemDeviceTypeLangBarItem>, ::windows::core::GetTrustLevel, SetIconMode::<Impl, OFFSET>, GetIconMode::<Impl, OFFSET>)
    }
}
pub trait ITfSystemLangBarItemImpl: Sized {
    fn SetIcon();
    fn SetTooltipString();
}
impl ::windows::core::RuntimeName for ITfSystemLangBarItem {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfSystemLangBarItem";
}
impl ITfSystemLangBarItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfSystemLangBarItemImpl, const OFFSET: isize>() -> ITfSystemLangBarItemVtbl {
        unsafe extern "system" fn SetIcon<Impl: ITfSystemLangBarItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hicon: super::WindowsAndMessaging::HICON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetIcon(&*(&hicon as *const <super::WindowsAndMessaging::HICON as ::windows::core::Abi>::Abi as *const <super::WindowsAndMessaging::HICON as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTooltipString<Impl: ITfSystemLangBarItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchtooltip: super::super::Foundation::PWSTR, cch: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTooltipString(&*(&pchtooltip as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cch) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfSystemLangBarItem>, ::windows::core::GetTrustLevel, SetIcon::<Impl, OFFSET>, SetTooltipString::<Impl, OFFSET>)
    }
}
pub trait ITfSystemLangBarItemSinkImpl: Sized {
    fn InitMenu();
    fn OnMenuSelect();
}
impl ::windows::core::RuntimeName for ITfSystemLangBarItemSink {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfSystemLangBarItemSink";
}
impl ITfSystemLangBarItemSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfSystemLangBarItemSinkImpl, const OFFSET: isize>() -> ITfSystemLangBarItemSinkVtbl {
        unsafe extern "system" fn InitMenu<Impl: ITfSystemLangBarItemSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmenu: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitMenu(&*(&pmenu as *const <ITfMenu as ::windows::core::Abi>::Abi as *const <ITfMenu as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnMenuSelect<Impl: ITfSystemLangBarItemSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnMenuSelect(wid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfSystemLangBarItemSink>, ::windows::core::GetTrustLevel, InitMenu::<Impl, OFFSET>, OnMenuSelect::<Impl, OFFSET>)
    }
}
pub trait ITfSystemLangBarItemTextImpl: Sized {
    fn SetItemText();
    fn GetItemText();
}
impl ::windows::core::RuntimeName for ITfSystemLangBarItemText {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfSystemLangBarItemText";
}
impl ITfSystemLangBarItemTextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfSystemLangBarItemTextImpl, const OFFSET: isize>() -> ITfSystemLangBarItemTextVtbl {
        unsafe extern "system" fn SetItemText<Impl: ITfSystemLangBarItemTextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pch: super::super::Foundation::PWSTR, cch: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetItemText(&*(&pch as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cch) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemText<Impl: ITfSystemLangBarItemTextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemText(::core::mem::transmute_copy(&pbstrtext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfSystemLangBarItemText>, ::windows::core::GetTrustLevel, SetItemText::<Impl, OFFSET>, GetItemText::<Impl, OFFSET>)
    }
}
pub trait ITfTextEditSinkImpl: Sized {
    fn OnEndEdit();
}
impl ::windows::core::RuntimeName for ITfTextEditSink {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfTextEditSink";
}
impl ITfTextEditSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfTextEditSinkImpl, const OFFSET: isize>() -> ITfTextEditSinkVtbl {
        unsafe extern "system" fn OnEndEdit<Impl: ITfTextEditSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr, ecreadonly: u32, peditrecord: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnEndEdit(&*(&pic as *const <ITfContext as ::windows::core::Abi>::Abi as *const <ITfContext as ::windows::core::DefaultType>::DefaultType), ecreadonly, &*(&peditrecord as *const <ITfEditRecord as ::windows::core::Abi>::Abi as *const <ITfEditRecord as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfTextEditSink>, ::windows::core::GetTrustLevel, OnEndEdit::<Impl, OFFSET>)
    }
}
pub trait ITfTextInputProcessorImpl: Sized {
    fn Activate();
    fn Deactivate();
}
impl ::windows::core::RuntimeName for ITfTextInputProcessor {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfTextInputProcessor";
}
impl ITfTextInputProcessorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfTextInputProcessorImpl, const OFFSET: isize>() -> ITfTextInputProcessorVtbl {
        unsafe extern "system" fn Activate<Impl: ITfTextInputProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptim: ::windows::core::RawPtr, tid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Activate(&*(&ptim as *const <ITfThreadMgr as ::windows::core::Abi>::Abi as *const <ITfThreadMgr as ::windows::core::DefaultType>::DefaultType), tid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Deactivate<Impl: ITfTextInputProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Deactivate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfTextInputProcessor>, ::windows::core::GetTrustLevel, Activate::<Impl, OFFSET>, Deactivate::<Impl, OFFSET>)
    }
}
pub trait ITfTextInputProcessorExImpl: Sized + ITfTextInputProcessorImpl {
    fn ActivateEx();
}
impl ::windows::core::RuntimeName for ITfTextInputProcessorEx {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfTextInputProcessorEx";
}
impl ITfTextInputProcessorExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfTextInputProcessorExImpl, const OFFSET: isize>() -> ITfTextInputProcessorExVtbl {
        unsafe extern "system" fn ActivateEx<Impl: ITfTextInputProcessorExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptim: ::windows::core::RawPtr, tid: u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivateEx(&*(&ptim as *const <ITfThreadMgr as ::windows::core::Abi>::Abi as *const <ITfThreadMgr as ::windows::core::DefaultType>::DefaultType), tid, dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfTextInputProcessorEx>, ::windows::core::GetTrustLevel, ActivateEx::<Impl, OFFSET>)
    }
}
pub trait ITfTextLayoutSinkImpl: Sized {
    fn OnLayoutChange();
}
impl ::windows::core::RuntimeName for ITfTextLayoutSink {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfTextLayoutSink";
}
impl ITfTextLayoutSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfTextLayoutSinkImpl, const OFFSET: isize>() -> ITfTextLayoutSinkVtbl {
        unsafe extern "system" fn OnLayoutChange<Impl: ITfTextLayoutSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr, lcode: TfLayoutCode, pview: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnLayoutChange(&*(&pic as *const <ITfContext as ::windows::core::Abi>::Abi as *const <ITfContext as ::windows::core::DefaultType>::DefaultType), lcode, &*(&pview as *const <ITfContextView as ::windows::core::Abi>::Abi as *const <ITfContextView as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfTextLayoutSink>, ::windows::core::GetTrustLevel, OnLayoutChange::<Impl, OFFSET>)
    }
}
pub trait ITfThreadFocusSinkImpl: Sized {
    fn OnSetThreadFocus();
    fn OnKillThreadFocus();
}
impl ::windows::core::RuntimeName for ITfThreadFocusSink {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfThreadFocusSink";
}
impl ITfThreadFocusSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfThreadFocusSinkImpl, const OFFSET: isize>() -> ITfThreadFocusSinkVtbl {
        unsafe extern "system" fn OnSetThreadFocus<Impl: ITfThreadFocusSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnSetThreadFocus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnKillThreadFocus<Impl: ITfThreadFocusSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnKillThreadFocus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfThreadFocusSink>, ::windows::core::GetTrustLevel, OnSetThreadFocus::<Impl, OFFSET>, OnKillThreadFocus::<Impl, OFFSET>)
    }
}
pub trait ITfThreadMgrImpl: Sized {
    fn Activate();
    fn Deactivate();
    fn CreateDocumentMgr();
    fn EnumDocumentMgrs();
    fn GetFocus();
    fn SetFocus();
    fn AssociateFocus();
    fn IsThreadFocus();
    fn GetFunctionProvider();
    fn EnumFunctionProviders();
    fn GetGlobalCompartment();
}
impl ::windows::core::RuntimeName for ITfThreadMgr {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfThreadMgr";
}
impl ITfThreadMgrVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfThreadMgrImpl, const OFFSET: isize>() -> ITfThreadMgrVtbl {
        unsafe extern "system" fn Activate<Impl: ITfThreadMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Activate(::core::mem::transmute_copy(&ptid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Deactivate<Impl: ITfThreadMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Deactivate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDocumentMgr<Impl: ITfThreadMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdim: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDocumentMgr(::core::mem::transmute_copy(&ppdim)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDocumentMgrs<Impl: ITfThreadMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumDocumentMgrs(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFocus<Impl: ITfThreadMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdimfocus: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFocus(::core::mem::transmute_copy(&ppdimfocus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFocus<Impl: ITfThreadMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdimfocus: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFocus(&*(&pdimfocus as *const <ITfDocumentMgr as ::windows::core::Abi>::Abi as *const <ITfDocumentMgr as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AssociateFocus<Impl: ITfThreadMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pdimnew: ::windows::core::RawPtr, ppdimprev: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AssociateFocus(&*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&pdimnew as *const <ITfDocumentMgr as ::windows::core::Abi>::Abi as *const <ITfDocumentMgr as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppdimprev)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsThreadFocus<Impl: ITfThreadMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfthreadfocus: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsThreadFocus(::core::mem::transmute_copy(&pfthreadfocus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunctionProvider<Impl: ITfThreadMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, ppfuncprov: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFunctionProvider(&*(&clsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppfuncprov)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumFunctionProviders<Impl: ITfThreadMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumFunctionProviders(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlobalCompartment<Impl: ITfThreadMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcompmgr: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGlobalCompartment(::core::mem::transmute_copy(&ppcompmgr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITfThreadMgr>,
            ::windows::core::GetTrustLevel,
            Activate::<Impl, OFFSET>,
            Deactivate::<Impl, OFFSET>,
            CreateDocumentMgr::<Impl, OFFSET>,
            EnumDocumentMgrs::<Impl, OFFSET>,
            GetFocus::<Impl, OFFSET>,
            SetFocus::<Impl, OFFSET>,
            AssociateFocus::<Impl, OFFSET>,
            IsThreadFocus::<Impl, OFFSET>,
            GetFunctionProvider::<Impl, OFFSET>,
            EnumFunctionProviders::<Impl, OFFSET>,
            GetGlobalCompartment::<Impl, OFFSET>,
        )
    }
}
pub trait ITfThreadMgr2Impl: Sized {
    fn Activate();
    fn Deactivate();
    fn CreateDocumentMgr();
    fn EnumDocumentMgrs();
    fn GetFocus();
    fn SetFocus();
    fn IsThreadFocus();
    fn GetFunctionProvider();
    fn EnumFunctionProviders();
    fn GetGlobalCompartment();
    fn ActivateEx();
    fn GetActiveFlags();
    fn SuspendKeystrokeHandling();
    fn ResumeKeystrokeHandling();
}
impl ::windows::core::RuntimeName for ITfThreadMgr2 {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfThreadMgr2";
}
impl ITfThreadMgr2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfThreadMgr2Impl, const OFFSET: isize>() -> ITfThreadMgr2Vtbl {
        unsafe extern "system" fn Activate<Impl: ITfThreadMgr2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Activate(::core::mem::transmute_copy(&ptid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Deactivate<Impl: ITfThreadMgr2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Deactivate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDocumentMgr<Impl: ITfThreadMgr2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdim: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDocumentMgr(::core::mem::transmute_copy(&ppdim)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDocumentMgrs<Impl: ITfThreadMgr2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumDocumentMgrs(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFocus<Impl: ITfThreadMgr2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdimfocus: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFocus(::core::mem::transmute_copy(&ppdimfocus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFocus<Impl: ITfThreadMgr2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdimfocus: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFocus(&*(&pdimfocus as *const <ITfDocumentMgr as ::windows::core::Abi>::Abi as *const <ITfDocumentMgr as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsThreadFocus<Impl: ITfThreadMgr2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfthreadfocus: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsThreadFocus(::core::mem::transmute_copy(&pfthreadfocus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunctionProvider<Impl: ITfThreadMgr2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, ppfuncprov: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFunctionProvider(&*(&clsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppfuncprov)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumFunctionProviders<Impl: ITfThreadMgr2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumFunctionProviders(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlobalCompartment<Impl: ITfThreadMgr2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcompmgr: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGlobalCompartment(::core::mem::transmute_copy(&ppcompmgr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivateEx<Impl: ITfThreadMgr2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptid: *mut u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivateEx(::core::mem::transmute_copy(&ptid), dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActiveFlags<Impl: ITfThreadMgr2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActiveFlags(::core::mem::transmute_copy(&lpdwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SuspendKeystrokeHandling<Impl: ITfThreadMgr2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SuspendKeystrokeHandling() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResumeKeystrokeHandling<Impl: ITfThreadMgr2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResumeKeystrokeHandling() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITfThreadMgr2>,
            ::windows::core::GetTrustLevel,
            Activate::<Impl, OFFSET>,
            Deactivate::<Impl, OFFSET>,
            CreateDocumentMgr::<Impl, OFFSET>,
            EnumDocumentMgrs::<Impl, OFFSET>,
            GetFocus::<Impl, OFFSET>,
            SetFocus::<Impl, OFFSET>,
            IsThreadFocus::<Impl, OFFSET>,
            GetFunctionProvider::<Impl, OFFSET>,
            EnumFunctionProviders::<Impl, OFFSET>,
            GetGlobalCompartment::<Impl, OFFSET>,
            ActivateEx::<Impl, OFFSET>,
            GetActiveFlags::<Impl, OFFSET>,
            SuspendKeystrokeHandling::<Impl, OFFSET>,
            ResumeKeystrokeHandling::<Impl, OFFSET>,
        )
    }
}
pub trait ITfThreadMgrEventSinkImpl: Sized {
    fn OnInitDocumentMgr();
    fn OnUninitDocumentMgr();
    fn OnSetFocus();
    fn OnPushContext();
    fn OnPopContext();
}
impl ::windows::core::RuntimeName for ITfThreadMgrEventSink {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfThreadMgrEventSink";
}
impl ITfThreadMgrEventSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfThreadMgrEventSinkImpl, const OFFSET: isize>() -> ITfThreadMgrEventSinkVtbl {
        unsafe extern "system" fn OnInitDocumentMgr<Impl: ITfThreadMgrEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdim: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnInitDocumentMgr(&*(&pdim as *const <ITfDocumentMgr as ::windows::core::Abi>::Abi as *const <ITfDocumentMgr as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnUninitDocumentMgr<Impl: ITfThreadMgrEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdim: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnUninitDocumentMgr(&*(&pdim as *const <ITfDocumentMgr as ::windows::core::Abi>::Abi as *const <ITfDocumentMgr as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnSetFocus<Impl: ITfThreadMgrEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdimfocus: ::windows::core::RawPtr, pdimprevfocus: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnSetFocus(&*(&pdimfocus as *const <ITfDocumentMgr as ::windows::core::Abi>::Abi as *const <ITfDocumentMgr as ::windows::core::DefaultType>::DefaultType), &*(&pdimprevfocus as *const <ITfDocumentMgr as ::windows::core::Abi>::Abi as *const <ITfDocumentMgr as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnPushContext<Impl: ITfThreadMgrEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnPushContext(&*(&pic as *const <ITfContext as ::windows::core::Abi>::Abi as *const <ITfContext as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnPopContext<Impl: ITfThreadMgrEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnPopContext(&*(&pic as *const <ITfContext as ::windows::core::Abi>::Abi as *const <ITfContext as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfThreadMgrEventSink>, ::windows::core::GetTrustLevel, OnInitDocumentMgr::<Impl, OFFSET>, OnUninitDocumentMgr::<Impl, OFFSET>, OnSetFocus::<Impl, OFFSET>, OnPushContext::<Impl, OFFSET>, OnPopContext::<Impl, OFFSET>)
    }
}
pub trait ITfThreadMgrExImpl: Sized + ITfThreadMgrImpl {
    fn ActivateEx();
    fn GetActiveFlags();
}
impl ::windows::core::RuntimeName for ITfThreadMgrEx {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfThreadMgrEx";
}
impl ITfThreadMgrExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfThreadMgrExImpl, const OFFSET: isize>() -> ITfThreadMgrExVtbl {
        unsafe extern "system" fn ActivateEx<Impl: ITfThreadMgrExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptid: *mut u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivateEx(::core::mem::transmute_copy(&ptid), dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActiveFlags<Impl: ITfThreadMgrExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActiveFlags(::core::mem::transmute_copy(&lpdwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfThreadMgrEx>, ::windows::core::GetTrustLevel, ActivateEx::<Impl, OFFSET>, GetActiveFlags::<Impl, OFFSET>)
    }
}
pub trait ITfToolTipUIElementImpl: Sized + ITfUIElementImpl {
    fn GetString();
}
impl ::windows::core::RuntimeName for ITfToolTipUIElement {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfToolTipUIElement";
}
impl ITfToolTipUIElementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfToolTipUIElementImpl, const OFFSET: isize>() -> ITfToolTipUIElementVtbl {
        unsafe extern "system" fn GetString<Impl: ITfToolTipUIElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetString(::core::mem::transmute_copy(&pstr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfToolTipUIElement>, ::windows::core::GetTrustLevel, GetString::<Impl, OFFSET>)
    }
}
pub trait ITfTransitoryExtensionSinkImpl: Sized {
    fn OnTransitoryExtensionUpdated();
}
impl ::windows::core::RuntimeName for ITfTransitoryExtensionSink {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfTransitoryExtensionSink";
}
impl ITfTransitoryExtensionSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfTransitoryExtensionSinkImpl, const OFFSET: isize>() -> ITfTransitoryExtensionSinkVtbl {
        unsafe extern "system" fn OnTransitoryExtensionUpdated<Impl: ITfTransitoryExtensionSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr, ecreadonly: u32, presultrange: ::windows::core::RawPtr, pcompositionrange: ::windows::core::RawPtr, pfdeleteresultrange: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnTransitoryExtensionUpdated(&*(&pic as *const <ITfContext as ::windows::core::Abi>::Abi as *const <ITfContext as ::windows::core::DefaultType>::DefaultType), ecreadonly, &*(&presultrange as *const <ITfRange as ::windows::core::Abi>::Abi as *const <ITfRange as ::windows::core::DefaultType>::DefaultType), &*(&pcompositionrange as *const <ITfRange as ::windows::core::Abi>::Abi as *const <ITfRange as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfdeleteresultrange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfTransitoryExtensionSink>, ::windows::core::GetTrustLevel, OnTransitoryExtensionUpdated::<Impl, OFFSET>)
    }
}
pub trait ITfTransitoryExtensionUIElementImpl: Sized + ITfUIElementImpl {
    fn GetDocumentMgr();
}
impl ::windows::core::RuntimeName for ITfTransitoryExtensionUIElement {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfTransitoryExtensionUIElement";
}
impl ITfTransitoryExtensionUIElementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfTransitoryExtensionUIElementImpl, const OFFSET: isize>() -> ITfTransitoryExtensionUIElementVtbl {
        unsafe extern "system" fn GetDocumentMgr<Impl: ITfTransitoryExtensionUIElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdim: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocumentMgr(::core::mem::transmute_copy(&ppdim)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfTransitoryExtensionUIElement>, ::windows::core::GetTrustLevel, GetDocumentMgr::<Impl, OFFSET>)
    }
}
pub trait ITfUIElementImpl: Sized {
    fn GetDescription();
    fn GetGUID();
    fn Show();
    fn IsShown();
}
impl ::windows::core::RuntimeName for ITfUIElement {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfUIElement";
}
impl ITfUIElementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfUIElementImpl, const OFFSET: isize>() -> ITfUIElementVtbl {
        unsafe extern "system" fn GetDescription<Impl: ITfUIElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDescription(::core::mem::transmute_copy(&pbstrdescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGUID<Impl: ITfUIElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGUID(::core::mem::transmute_copy(&pguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Show<Impl: ITfUIElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Show(&*(&bshow as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsShown<Impl: ITfUIElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbshow: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsShown(::core::mem::transmute_copy(&pbshow)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfUIElement>, ::windows::core::GetTrustLevel, GetDescription::<Impl, OFFSET>, GetGUID::<Impl, OFFSET>, Show::<Impl, OFFSET>, IsShown::<Impl, OFFSET>)
    }
}
pub trait ITfUIElementMgrImpl: Sized {
    fn BeginUIElement();
    fn UpdateUIElement();
    fn EndUIElement();
    fn GetUIElement();
    fn EnumUIElements();
}
impl ::windows::core::RuntimeName for ITfUIElementMgr {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfUIElementMgr";
}
impl ITfUIElementMgrVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfUIElementMgrImpl, const OFFSET: isize>() -> ITfUIElementMgrVtbl {
        unsafe extern "system" fn BeginUIElement<Impl: ITfUIElementMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pelement: ::windows::core::RawPtr, pbshow: *mut super::super::Foundation::BOOL, pdwuielementid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginUIElement(&*(&pelement as *const <ITfUIElement as ::windows::core::Abi>::Abi as *const <ITfUIElement as ::windows::core::DefaultType>::DefaultType), &*(&pbshow as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwuielementid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateUIElement<Impl: ITfUIElementMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwuielementid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateUIElement(dwuielementid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndUIElement<Impl: ITfUIElementMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwuielementid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndUIElement(dwuielementid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUIElement<Impl: ITfUIElementMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwuielementid: u32, ppelement: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUIElement(dwuielementid, ::core::mem::transmute_copy(&ppelement)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumUIElements<Impl: ITfUIElementMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumUIElements(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfUIElementMgr>, ::windows::core::GetTrustLevel, BeginUIElement::<Impl, OFFSET>, UpdateUIElement::<Impl, OFFSET>, EndUIElement::<Impl, OFFSET>, GetUIElement::<Impl, OFFSET>, EnumUIElements::<Impl, OFFSET>)
    }
}
pub trait ITfUIElementSinkImpl: Sized {
    fn BeginUIElement();
    fn UpdateUIElement();
    fn EndUIElement();
}
impl ::windows::core::RuntimeName for ITfUIElementSink {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.ITfUIElementSink";
}
impl ITfUIElementSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfUIElementSinkImpl, const OFFSET: isize>() -> ITfUIElementSinkVtbl {
        unsafe extern "system" fn BeginUIElement<Impl: ITfUIElementSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwuielementid: u32, pbshow: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginUIElement(dwuielementid, &*(&pbshow as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateUIElement<Impl: ITfUIElementSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwuielementid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateUIElement(dwuielementid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndUIElement<Impl: ITfUIElementSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwuielementid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndUIElement(dwuielementid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITfUIElementSink>, ::windows::core::GetTrustLevel, BeginUIElement::<Impl, OFFSET>, UpdateUIElement::<Impl, OFFSET>, EndUIElement::<Impl, OFFSET>)
    }
}
pub trait IUIManagerEventSinkImpl: Sized {
    fn OnWindowOpening();
    fn OnWindowOpened();
    fn OnWindowUpdating();
    fn OnWindowUpdated();
    fn OnWindowClosing();
    fn OnWindowClosed();
}
impl ::windows::core::RuntimeName for IUIManagerEventSink {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.IUIManagerEventSink";
}
impl IUIManagerEventSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIManagerEventSinkImpl, const OFFSET: isize>() -> IUIManagerEventSinkVtbl {
        unsafe extern "system" fn OnWindowOpening<Impl: IUIManagerEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcbounds: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnWindowOpening(&*(&prcbounds as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnWindowOpened<Impl: IUIManagerEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcbounds: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnWindowOpened(&*(&prcbounds as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnWindowUpdating<Impl: IUIManagerEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcupdatedbounds: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnWindowUpdating(&*(&prcupdatedbounds as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnWindowUpdated<Impl: IUIManagerEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcupdatedbounds: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnWindowUpdated(&*(&prcupdatedbounds as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnWindowClosing<Impl: IUIManagerEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnWindowClosing() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnWindowClosed<Impl: IUIManagerEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnWindowClosed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIManagerEventSink>, ::windows::core::GetTrustLevel, OnWindowOpening::<Impl, OFFSET>, OnWindowOpened::<Impl, OFFSET>, OnWindowUpdating::<Impl, OFFSET>, OnWindowUpdated::<Impl, OFFSET>, OnWindowClosing::<Impl, OFFSET>, OnWindowClosed::<Impl, OFFSET>)
    }
}
pub trait IVersionInfoImpl: Sized {
    fn GetSubcomponentCount();
    fn GetImplementationID();
    fn GetBuildVersion();
    fn GetComponentDescription();
    fn GetInstanceDescription();
}
impl ::windows::core::RuntimeName for IVersionInfo {
    const NAME: &'static str = "Windows.Win32.UI.TextServices.IVersionInfo";
}
impl IVersionInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVersionInfoImpl, const OFFSET: isize>() -> IVersionInfoVtbl {
        unsafe extern "system" fn GetSubcomponentCount<Impl: IVersionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulsub: u32, ulcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSubcomponentCount(ulsub, ::core::mem::transmute_copy(&ulcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImplementationID<Impl: IVersionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulsub: u32, implid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetImplementationID(ulsub, ::core::mem::transmute_copy(&implid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBuildVersion<Impl: IVersionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulsub: u32, pdwmajor: *mut u32, pdwminor: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBuildVersion(ulsub, ::core::mem::transmute_copy(&pdwmajor), ::core::mem::transmute_copy(&pdwminor)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetComponentDescription<Impl: IVersionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulsub: u32, pimplstr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetComponentDescription(ulsub, ::core::mem::transmute_copy(&pimplstr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInstanceDescription<Impl: IVersionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulsub: u32, pimplstr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInstanceDescription(ulsub, ::core::mem::transmute_copy(&pimplstr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVersionInfo>, ::windows::core::GetTrustLevel, GetSubcomponentCount::<Impl, OFFSET>, GetImplementationID::<Impl, OFFSET>, GetBuildVersion::<Impl, OFFSET>, GetComponentDescription::<Impl, OFFSET>, GetInstanceDescription::<Impl, OFFSET>)
    }
}
