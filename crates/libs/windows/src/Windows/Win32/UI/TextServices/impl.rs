#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAccClientDocMgrImpl: Sized {
    fn GetDocuments(&mut self) -> ::windows::core::Result<super::super::System::Com::IEnumUnknown>;
    fn LookupByHWND(&mut self, hwnd: super::super::Foundation::HWND, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn LookupByPoint(&mut self, pt: super::super::Foundation::POINT, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetFocused(&mut self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAccClientDocMgrVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccClientDocMgrImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccClientDocMgrVtbl {
        unsafe extern "system" fn GetDocuments<Impl: IAccClientDocMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumunknown: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocuments() {
                ::core::result::Result::Ok(ok__) => {
                    *enumunknown = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupByHWND<Impl: IAccClientDocMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LookupByHWND(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupByPoint<Impl: IAccClientDocMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pt: super::super::Foundation::POINT, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LookupByPoint(::core::mem::transmute_copy(&pt), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFocused<Impl: IAccClientDocMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFocused(::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetDocuments: GetDocuments::<Impl, IMPL_OFFSET>,
            LookupByHWND: LookupByHWND::<Impl, IMPL_OFFSET>,
            LookupByPoint: LookupByPoint::<Impl, IMPL_OFFSET>,
            GetFocused: GetFocused::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccClientDocMgr as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAccDictionaryImpl: Sized {
    fn GetLocalizedString(&mut self, term: *const ::windows::core::GUID, lcid: u32, presult: *mut super::super::Foundation::BSTR, plcid: *mut u32) -> ::windows::core::Result<()>;
    fn GetParentTerm(&mut self, term: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetMnemonicString(&mut self, term: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn LookupMnemonicTerm(&mut self, bstrmnemonic: super::super::Foundation::BSTR) -> ::windows::core::Result<::windows::core::GUID>;
    fn ConvertValueToString(&mut self, term: *const ::windows::core::GUID, lcid: u32, varvalue: super::super::System::Com::VARIANT, pbstrresult: *mut super::super::Foundation::BSTR, plcid: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAccDictionaryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccDictionaryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccDictionaryVtbl {
        unsafe extern "system" fn GetLocalizedString<Impl: IAccDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, term: *const ::windows::core::GUID, lcid: u32, presult: *mut super::super::Foundation::BSTR, plcid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLocalizedString(::core::mem::transmute_copy(&term), ::core::mem::transmute_copy(&lcid), ::core::mem::transmute_copy(&presult), ::core::mem::transmute_copy(&plcid)).into()
        }
        unsafe extern "system" fn GetParentTerm<Impl: IAccDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, term: *const ::windows::core::GUID, pparentterm: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetParentTerm(::core::mem::transmute_copy(&term)) {
                ::core::result::Result::Ok(ok__) => {
                    *pparentterm = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMnemonicString<Impl: IAccDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, term: *const ::windows::core::GUID, presult: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMnemonicString(::core::mem::transmute_copy(&term)) {
                ::core::result::Result::Ok(ok__) => {
                    *presult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupMnemonicTerm<Impl: IAccDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmnemonic: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pterm: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LookupMnemonicTerm(::core::mem::transmute_copy(&bstrmnemonic)) {
                ::core::result::Result::Ok(ok__) => {
                    *pterm = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertValueToString<Impl: IAccDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, term: *const ::windows::core::GUID, lcid: u32, varvalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pbstrresult: *mut super::super::Foundation::BSTR, plcid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConvertValueToString(::core::mem::transmute_copy(&term), ::core::mem::transmute_copy(&lcid), ::core::mem::transmute_copy(&varvalue), ::core::mem::transmute_copy(&pbstrresult), ::core::mem::transmute_copy(&plcid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetLocalizedString: GetLocalizedString::<Impl, IMPL_OFFSET>,
            GetParentTerm: GetParentTerm::<Impl, IMPL_OFFSET>,
            GetMnemonicString: GetMnemonicString::<Impl, IMPL_OFFSET>,
            LookupMnemonicTerm: LookupMnemonicTerm::<Impl, IMPL_OFFSET>,
            ConvertValueToString: ConvertValueToString::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccDictionary as ::windows::core::Interface>::IID
    }
}
pub trait IAccServerDocMgrImpl: Sized {
    fn NewDocument(&mut self, riid: *const ::windows::core::GUID, punk: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn RevokeDocument(&mut self, punk: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn OnDocumentFocus(&mut self, punk: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
impl IAccServerDocMgrVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccServerDocMgrImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccServerDocMgrVtbl {
        unsafe extern "system" fn NewDocument<Impl: IAccServerDocMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NewDocument(::core::mem::transmute_copy(&riid), ::core::mem::transmute(&punk)).into()
        }
        unsafe extern "system" fn RevokeDocument<Impl: IAccServerDocMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RevokeDocument(::core::mem::transmute(&punk)).into()
        }
        unsafe extern "system" fn OnDocumentFocus<Impl: IAccServerDocMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDocumentFocus(::core::mem::transmute(&punk)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            NewDocument: NewDocument::<Impl, IMPL_OFFSET>,
            RevokeDocument: RevokeDocument::<Impl, IMPL_OFFSET>,
            OnDocumentFocus: OnDocumentFocus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccServerDocMgr as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAccStoreImpl: Sized {
    fn Register(&mut self, riid: *const ::windows::core::GUID, punk: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn Unregister(&mut self, punk: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetDocuments(&mut self) -> ::windows::core::Result<super::super::System::Com::IEnumUnknown>;
    fn LookupByHWND(&mut self, hwnd: super::super::Foundation::HWND, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn LookupByPoint(&mut self, pt: super::super::Foundation::POINT, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn OnDocumentFocus(&mut self, punk: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetFocused(&mut self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAccStoreVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccStoreImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccStoreVtbl {
        unsafe extern "system" fn Register<Impl: IAccStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Register(::core::mem::transmute_copy(&riid), ::core::mem::transmute(&punk)).into()
        }
        unsafe extern "system" fn Unregister<Impl: IAccStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unregister(::core::mem::transmute(&punk)).into()
        }
        unsafe extern "system" fn GetDocuments<Impl: IAccStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumunknown: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocuments() {
                ::core::result::Result::Ok(ok__) => {
                    *enumunknown = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupByHWND<Impl: IAccStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LookupByHWND(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupByPoint<Impl: IAccStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pt: super::super::Foundation::POINT, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LookupByPoint(::core::mem::transmute_copy(&pt), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnDocumentFocus<Impl: IAccStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDocumentFocus(::core::mem::transmute(&punk)).into()
        }
        unsafe extern "system" fn GetFocused<Impl: IAccStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFocused(::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Register: Register::<Impl, IMPL_OFFSET>,
            Unregister: Unregister::<Impl, IMPL_OFFSET>,
            GetDocuments: GetDocuments::<Impl, IMPL_OFFSET>,
            LookupByHWND: LookupByHWND::<Impl, IMPL_OFFSET>,
            LookupByPoint: LookupByPoint::<Impl, IMPL_OFFSET>,
            OnDocumentFocus: OnDocumentFocus::<Impl, IMPL_OFFSET>,
            GetFocused: GetFocused::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccStore as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAnchorImpl: Sized {
    fn SetGravity(&mut self, gravity: TsGravity) -> ::windows::core::Result<()>;
    fn GetGravity(&mut self) -> ::windows::core::Result<TsGravity>;
    fn IsEqual(&mut self, pawith: ::core::option::Option<IAnchor>) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn Compare(&mut self, pawith: ::core::option::Option<IAnchor>) -> ::windows::core::Result<i32>;
    fn Shift(&mut self, dwflags: u32, cchreq: i32, pcch: *mut i32, pahaltanchor: ::core::option::Option<IAnchor>) -> ::windows::core::Result<()>;
    fn ShiftTo(&mut self, pasite: ::core::option::Option<IAnchor>) -> ::windows::core::Result<()>;
    fn ShiftRegion(&mut self, dwflags: u32, dir: TsShiftDir) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetChangeHistoryMask(&mut self, dwmask: u32) -> ::windows::core::Result<()>;
    fn GetChangeHistory(&mut self) -> ::windows::core::Result<ANCHOR_CHANGE_HISTORY_FLAGS>;
    fn ClearChangeHistory(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IAnchor>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAnchorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAnchorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAnchorVtbl {
        unsafe extern "system" fn SetGravity<Impl: IAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gravity: TsGravity) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGravity(::core::mem::transmute_copy(&gravity)).into()
        }
        unsafe extern "system" fn GetGravity<Impl: IAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgravity: *mut TsGravity) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGravity() {
                ::core::result::Result::Ok(ok__) => {
                    *pgravity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Impl: IAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pawith: ::windows::core::RawPtr, pfequal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEqual(::core::mem::transmute(&pawith)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfequal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Compare<Impl: IAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pawith: ::windows::core::RawPtr, plresult: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Compare(::core::mem::transmute(&pawith)) {
                ::core::result::Result::Ok(ok__) => {
                    *plresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shift<Impl: IAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, cchreq: i32, pcch: *mut i32, pahaltanchor: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Shift(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&cchreq), ::core::mem::transmute_copy(&pcch), ::core::mem::transmute(&pahaltanchor)).into()
        }
        unsafe extern "system" fn ShiftTo<Impl: IAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pasite: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShiftTo(::core::mem::transmute(&pasite)).into()
        }
        unsafe extern "system" fn ShiftRegion<Impl: IAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, dir: TsShiftDir, pfnoregion: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShiftRegion(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dir)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfnoregion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChangeHistoryMask<Impl: IAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetChangeHistoryMask(::core::mem::transmute_copy(&dwmask)).into()
        }
        unsafe extern "system" fn GetChangeHistory<Impl: IAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwhistory: *mut ANCHOR_CHANGE_HISTORY_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChangeHistory() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwhistory = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearChangeHistory<Impl: IAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearChangeHistory().into()
        }
        unsafe extern "system" fn Clone<Impl: IAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaclone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppaclone = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetGravity: SetGravity::<Impl, IMPL_OFFSET>,
            GetGravity: GetGravity::<Impl, IMPL_OFFSET>,
            IsEqual: IsEqual::<Impl, IMPL_OFFSET>,
            Compare: Compare::<Impl, IMPL_OFFSET>,
            Shift: Shift::<Impl, IMPL_OFFSET>,
            ShiftTo: ShiftTo::<Impl, IMPL_OFFSET>,
            ShiftRegion: ShiftRegion::<Impl, IMPL_OFFSET>,
            SetChangeHistoryMask: SetChangeHistoryMask::<Impl, IMPL_OFFSET>,
            GetChangeHistory: GetChangeHistory::<Impl, IMPL_OFFSET>,
            ClearChangeHistory: ClearChangeHistory::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAnchor as ::windows::core::Interface>::IID
    }
}
pub trait IClonableWrapperImpl: Sized {
    fn CloneNewWrapper(&mut self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IClonableWrapperVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClonableWrapperImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IClonableWrapperVtbl {
        unsafe extern "system" fn CloneNewWrapper<Impl: IClonableWrapperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CloneNewWrapper(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CloneNewWrapper: CloneNewWrapper::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClonableWrapper as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICoCreateLocallyImpl: Sized {
    fn CoCreateLocally(&mut self, rclsid: *const ::windows::core::GUID, dwclscontext: u32, riid: *const ::windows::core::GUID, punk: *mut ::core::option::Option<::windows::core::IUnknown>, riidparam: *const ::windows::core::GUID, punkparam: ::core::option::Option<::windows::core::IUnknown>, varparam: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICoCreateLocallyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoCreateLocallyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoCreateLocallyVtbl {
        unsafe extern "system" fn CoCreateLocally<Impl: ICoCreateLocallyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, dwclscontext: u32, riid: *const ::windows::core::GUID, punk: *mut *mut ::core::ffi::c_void, riidparam: *const ::windows::core::GUID, punkparam: *mut ::core::ffi::c_void, varparam: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CoCreateLocally(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&dwclscontext), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&punk), ::core::mem::transmute_copy(&riidparam), ::core::mem::transmute(&punkparam), ::core::mem::transmute_copy(&varparam)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CoCreateLocally: CoCreateLocally::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoCreateLocally as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICoCreatedLocallyImpl: Sized {
    fn LocalInit(&mut self, punklocalobject: ::core::option::Option<::windows::core::IUnknown>, riidparam: *const ::windows::core::GUID, punkparam: ::core::option::Option<::windows::core::IUnknown>, varparam: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICoCreatedLocallyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoCreatedLocallyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoCreatedLocallyVtbl {
        unsafe extern "system" fn LocalInit<Impl: ICoCreatedLocallyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punklocalobject: *mut ::core::ffi::c_void, riidparam: *const ::windows::core::GUID, punkparam: *mut ::core::ffi::c_void, varparam: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LocalInit(::core::mem::transmute(&punklocalobject), ::core::mem::transmute_copy(&riidparam), ::core::mem::transmute(&punkparam), ::core::mem::transmute_copy(&varparam)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), LocalInit: LocalInit::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoCreatedLocally as ::windows::core::Interface>::IID
    }
}
pub trait IDocWrapImpl: Sized {
    fn SetDoc(&mut self, riid: *const ::windows::core::GUID, punk: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetWrappedDoc(&mut self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
}
impl IDocWrapVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDocWrapImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDocWrapVtbl {
        unsafe extern "system" fn SetDoc<Impl: IDocWrapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDoc(::core::mem::transmute_copy(&riid), ::core::mem::transmute(&punk)).into()
        }
        unsafe extern "system" fn GetWrappedDoc<Impl: IDocWrapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWrappedDoc(::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetDoc: SetDoc::<Impl, IMPL_OFFSET>,
            GetWrappedDoc: GetWrappedDoc::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDocWrap as ::windows::core::Interface>::IID
    }
}
pub trait IEnumITfCompositionViewImpl: Sized {
    fn Clone(&mut self) -> ::windows::core::Result<IEnumITfCompositionView>;
    fn Next(&mut self, ulcount: u32, rgcompositionview: *mut ::core::option::Option<ITfCompositionView>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, ulcount: u32) -> ::windows::core::Result<()>;
}
impl IEnumITfCompositionViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumITfCompositionViewImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumITfCompositionViewVtbl {
        unsafe extern "system" fn Clone<Impl: IEnumITfCompositionViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumITfCompositionViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, rgcompositionview: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&rgcompositionview), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumITfCompositionViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumITfCompositionViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Clone: Clone::<Impl, IMPL_OFFSET>,
            Next: Next::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumITfCompositionView as ::windows::core::Interface>::IID
    }
}
pub trait IEnumSpeechCommandsImpl: Sized {
    fn Clone(&mut self) -> ::windows::core::Result<IEnumSpeechCommands>;
    fn Next(&mut self, ulcount: u32, pspcmds: *mut *mut u16, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, ulcount: u32) -> ::windows::core::Result<()>;
}
impl IEnumSpeechCommandsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSpeechCommandsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumSpeechCommandsVtbl {
        unsafe extern "system" fn Clone<Impl: IEnumSpeechCommandsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumSpeechCommandsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, pspcmds: *mut *mut u16, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pspcmds), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumSpeechCommandsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumSpeechCommandsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Clone: Clone::<Impl, IMPL_OFFSET>,
            Next: Next::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumSpeechCommands as ::windows::core::Interface>::IID
    }
}
pub trait IEnumTfCandidatesImpl: Sized {
    fn Clone(&mut self) -> ::windows::core::Result<IEnumTfCandidates>;
    fn Next(&mut self, ulcount: u32, ppcand: *mut ::core::option::Option<ITfCandidateString>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, ulcount: u32) -> ::windows::core::Result<()>;
}
impl IEnumTfCandidatesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTfCandidatesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumTfCandidatesVtbl {
        unsafe extern "system" fn Clone<Impl: IEnumTfCandidatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumTfCandidatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, ppcand: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&ppcand), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumTfCandidatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumTfCandidatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Clone: Clone::<Impl, IMPL_OFFSET>,
            Next: Next::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumTfCandidates as ::windows::core::Interface>::IID
    }
}
pub trait IEnumTfContextViewsImpl: Sized {
    fn Clone(&mut self) -> ::windows::core::Result<IEnumTfContextViews>;
    fn Next(&mut self, ulcount: u32, rgviews: *mut ::core::option::Option<ITfContextView>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, ulcount: u32) -> ::windows::core::Result<()>;
}
impl IEnumTfContextViewsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTfContextViewsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumTfContextViewsVtbl {
        unsafe extern "system" fn Clone<Impl: IEnumTfContextViewsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumTfContextViewsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, rgviews: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&rgviews), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumTfContextViewsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumTfContextViewsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Clone: Clone::<Impl, IMPL_OFFSET>,
            Next: Next::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumTfContextViews as ::windows::core::Interface>::IID
    }
}
pub trait IEnumTfContextsImpl: Sized {
    fn Clone(&mut self) -> ::windows::core::Result<IEnumTfContexts>;
    fn Next(&mut self, ulcount: u32, rgcontext: *mut ::core::option::Option<ITfContext>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, ulcount: u32) -> ::windows::core::Result<()>;
}
impl IEnumTfContextsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTfContextsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumTfContextsVtbl {
        unsafe extern "system" fn Clone<Impl: IEnumTfContextsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumTfContextsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, rgcontext: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&rgcontext), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumTfContextsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumTfContextsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Clone: Clone::<Impl, IMPL_OFFSET>,
            Next: Next::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumTfContexts as ::windows::core::Interface>::IID
    }
}
pub trait IEnumTfDisplayAttributeInfoImpl: Sized {
    fn Clone(&mut self) -> ::windows::core::Result<IEnumTfDisplayAttributeInfo>;
    fn Next(&mut self, ulcount: u32, rginfo: *mut ::core::option::Option<ITfDisplayAttributeInfo>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, ulcount: u32) -> ::windows::core::Result<()>;
}
impl IEnumTfDisplayAttributeInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTfDisplayAttributeInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumTfDisplayAttributeInfoVtbl {
        unsafe extern "system" fn Clone<Impl: IEnumTfDisplayAttributeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumTfDisplayAttributeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, rginfo: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&rginfo), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumTfDisplayAttributeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumTfDisplayAttributeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Clone: Clone::<Impl, IMPL_OFFSET>,
            Next: Next::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumTfDisplayAttributeInfo as ::windows::core::Interface>::IID
    }
}
pub trait IEnumTfDocumentMgrsImpl: Sized {
    fn Clone(&mut self) -> ::windows::core::Result<IEnumTfDocumentMgrs>;
    fn Next(&mut self, ulcount: u32, rgdocumentmgr: *mut ::core::option::Option<ITfDocumentMgr>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, ulcount: u32) -> ::windows::core::Result<()>;
}
impl IEnumTfDocumentMgrsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTfDocumentMgrsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumTfDocumentMgrsVtbl {
        unsafe extern "system" fn Clone<Impl: IEnumTfDocumentMgrsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumTfDocumentMgrsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, rgdocumentmgr: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&rgdocumentmgr), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumTfDocumentMgrsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumTfDocumentMgrsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Clone: Clone::<Impl, IMPL_OFFSET>,
            Next: Next::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumTfDocumentMgrs as ::windows::core::Interface>::IID
    }
}
pub trait IEnumTfFunctionProvidersImpl: Sized {
    fn Clone(&mut self) -> ::windows::core::Result<IEnumTfFunctionProviders>;
    fn Next(&mut self, ulcount: u32, ppcmdobj: *mut ::core::option::Option<ITfFunctionProvider>, pcfetch: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, ulcount: u32) -> ::windows::core::Result<()>;
}
impl IEnumTfFunctionProvidersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTfFunctionProvidersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumTfFunctionProvidersVtbl {
        unsafe extern "system" fn Clone<Impl: IEnumTfFunctionProvidersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumTfFunctionProvidersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, ppcmdobj: *mut ::windows::core::RawPtr, pcfetch: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&ppcmdobj), ::core::mem::transmute_copy(&pcfetch)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumTfFunctionProvidersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumTfFunctionProvidersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Clone: Clone::<Impl, IMPL_OFFSET>,
            Next: Next::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumTfFunctionProviders as ::windows::core::Interface>::IID
    }
}
pub trait IEnumTfInputProcessorProfilesImpl: Sized {
    fn Clone(&mut self) -> ::windows::core::Result<IEnumTfInputProcessorProfiles>;
    fn Next(&mut self, ulcount: u32, pprofile: *mut TF_INPUTPROCESSORPROFILE, pcfetch: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, ulcount: u32) -> ::windows::core::Result<()>;
}
impl IEnumTfInputProcessorProfilesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTfInputProcessorProfilesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumTfInputProcessorProfilesVtbl {
        unsafe extern "system" fn Clone<Impl: IEnumTfInputProcessorProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumTfInputProcessorProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, pprofile: *mut TF_INPUTPROCESSORPROFILE, pcfetch: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pprofile), ::core::mem::transmute_copy(&pcfetch)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumTfInputProcessorProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumTfInputProcessorProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Clone: Clone::<Impl, IMPL_OFFSET>,
            Next: Next::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumTfInputProcessorProfiles as ::windows::core::Interface>::IID
    }
}
pub trait IEnumTfLangBarItemsImpl: Sized {
    fn Clone(&mut self) -> ::windows::core::Result<IEnumTfLangBarItems>;
    fn Next(&mut self, ulcount: u32, ppitem: *mut ::core::option::Option<ITfLangBarItem>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, ulcount: u32) -> ::windows::core::Result<()>;
}
impl IEnumTfLangBarItemsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTfLangBarItemsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumTfLangBarItemsVtbl {
        unsafe extern "system" fn Clone<Impl: IEnumTfLangBarItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumTfLangBarItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, ppitem: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&ppitem), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumTfLangBarItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumTfLangBarItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Clone: Clone::<Impl, IMPL_OFFSET>,
            Next: Next::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumTfLangBarItems as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEnumTfLanguageProfilesImpl: Sized {
    fn Clone(&mut self) -> ::windows::core::Result<IEnumTfLanguageProfiles>;
    fn Next(&mut self, ulcount: u32, pprofile: *mut TF_LANGUAGEPROFILE, pcfetch: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, ulcount: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IEnumTfLanguageProfilesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTfLanguageProfilesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumTfLanguageProfilesVtbl {
        unsafe extern "system" fn Clone<Impl: IEnumTfLanguageProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumTfLanguageProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, pprofile: *mut TF_LANGUAGEPROFILE, pcfetch: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pprofile), ::core::mem::transmute_copy(&pcfetch)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumTfLanguageProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumTfLanguageProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Clone: Clone::<Impl, IMPL_OFFSET>,
            Next: Next::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumTfLanguageProfiles as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEnumTfLatticeElementsImpl: Sized {
    fn Clone(&mut self) -> ::windows::core::Result<IEnumTfLatticeElements>;
    fn Next(&mut self, ulcount: u32, rgselements: *mut TF_LMLATTELEMENT, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, ulcount: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IEnumTfLatticeElementsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTfLatticeElementsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumTfLatticeElementsVtbl {
        unsafe extern "system" fn Clone<Impl: IEnumTfLatticeElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumTfLatticeElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, rgselements: *mut TF_LMLATTELEMENT, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&rgselements), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumTfLatticeElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumTfLatticeElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Clone: Clone::<Impl, IMPL_OFFSET>,
            Next: Next::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumTfLatticeElements as ::windows::core::Interface>::IID
    }
}
pub trait IEnumTfPropertiesImpl: Sized {
    fn Clone(&mut self) -> ::windows::core::Result<IEnumTfProperties>;
    fn Next(&mut self, ulcount: u32, ppprop: *mut ::core::option::Option<ITfProperty>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, ulcount: u32) -> ::windows::core::Result<()>;
}
impl IEnumTfPropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTfPropertiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumTfPropertiesVtbl {
        unsafe extern "system" fn Clone<Impl: IEnumTfPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumTfPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, ppprop: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&ppprop), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumTfPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumTfPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Clone: Clone::<Impl, IMPL_OFFSET>,
            Next: Next::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumTfProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IEnumTfPropertyValueImpl: Sized {
    fn Clone(&mut self) -> ::windows::core::Result<IEnumTfPropertyValue>;
    fn Next(&mut self, ulcount: u32, rgvalues: *mut TF_PROPERTYVAL, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, ulcount: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IEnumTfPropertyValueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTfPropertyValueImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumTfPropertyValueVtbl {
        unsafe extern "system" fn Clone<Impl: IEnumTfPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumTfPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, rgvalues: *mut TF_PROPERTYVAL, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&rgvalues), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumTfPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumTfPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Clone: Clone::<Impl, IMPL_OFFSET>,
            Next: Next::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumTfPropertyValue as ::windows::core::Interface>::IID
    }
}
pub trait IEnumTfRangesImpl: Sized {
    fn Clone(&mut self) -> ::windows::core::Result<IEnumTfRanges>;
    fn Next(&mut self, ulcount: u32, pprange: *mut ::core::option::Option<ITfRange>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, ulcount: u32) -> ::windows::core::Result<()>;
}
impl IEnumTfRangesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTfRangesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumTfRangesVtbl {
        unsafe extern "system" fn Clone<Impl: IEnumTfRangesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumTfRangesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, pprange: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pprange), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumTfRangesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumTfRangesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Clone: Clone::<Impl, IMPL_OFFSET>,
            Next: Next::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumTfRanges as ::windows::core::Interface>::IID
    }
}
pub trait IEnumTfUIElementsImpl: Sized {
    fn Clone(&mut self) -> ::windows::core::Result<IEnumTfUIElements>;
    fn Next(&mut self, ulcount: u32, ppelement: *mut ::core::option::Option<ITfUIElement>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, ulcount: u32) -> ::windows::core::Result<()>;
}
impl IEnumTfUIElementsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTfUIElementsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumTfUIElementsVtbl {
        unsafe extern "system" fn Clone<Impl: IEnumTfUIElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumTfUIElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, ppelement: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&ppelement), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumTfUIElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumTfUIElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Clone: Clone::<Impl, IMPL_OFFSET>,
            Next: Next::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumTfUIElements as ::windows::core::Interface>::IID
    }
}
pub trait IInternalDocWrapImpl: Sized {
    fn NotifyRevoke(&mut self) -> ::windows::core::Result<()>;
}
impl IInternalDocWrapVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternalDocWrapImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInternalDocWrapVtbl {
        unsafe extern "system" fn NotifyRevoke<Impl: IInternalDocWrapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyRevoke().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), NotifyRevoke: NotifyRevoke::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInternalDocWrap as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISpeechCommandProviderImpl: Sized {
    fn EnumSpeechCommands(&mut self, langid: u16) -> ::windows::core::Result<IEnumSpeechCommands>;
    fn ProcessCommand(&mut self, pszcommand: super::super::Foundation::PWSTR, cch: u32, langid: u16) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISpeechCommandProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechCommandProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechCommandProviderVtbl {
        unsafe extern "system" fn EnumSpeechCommands<Impl: ISpeechCommandProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumSpeechCommands(::core::mem::transmute_copy(&langid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessCommand<Impl: ISpeechCommandProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcommand: super::super::Foundation::PWSTR, cch: u32, langid: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessCommand(::core::mem::transmute_copy(&pszcommand), ::core::mem::transmute_copy(&cch), ::core::mem::transmute_copy(&langid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            EnumSpeechCommands: EnumSpeechCommands::<Impl, IMPL_OFFSET>,
            ProcessCommand: ProcessCommand::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechCommandProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextStoreACPImpl: Sized {
    fn AdviseSink(&mut self, riid: *const ::windows::core::GUID, punk: ::core::option::Option<::windows::core::IUnknown>, dwmask: u32) -> ::windows::core::Result<()>;
    fn UnadviseSink(&mut self, punk: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn RequestLock(&mut self, dwlockflags: u32) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn GetStatus(&mut self) -> ::windows::core::Result<TS_STATUS>;
    fn QueryInsert(&mut self, acpteststart: i32, acptestend: i32, cch: u32, pacpresultstart: *mut i32, pacpresultend: *mut i32) -> ::windows::core::Result<()>;
    fn GetSelection(&mut self, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ACP, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn SetSelection(&mut self, ulcount: u32, pselection: *const TS_SELECTION_ACP) -> ::windows::core::Result<()>;
    fn GetText(&mut self, acpstart: i32, acpend: i32, pchplain: super::super::Foundation::PWSTR, cchplainreq: u32, pcchplainret: *mut u32, prgruninfo: *mut TS_RUNINFO, cruninforeq: u32, pcruninforet: *mut u32, pacpnext: *mut i32) -> ::windows::core::Result<()>;
    fn SetText(&mut self, dwflags: u32, acpstart: i32, acpend: i32, pchtext: super::super::Foundation::PWSTR, cch: u32) -> ::windows::core::Result<TS_TEXTCHANGE>;
    fn GetFormattedText(&mut self, acpstart: i32, acpend: i32) -> ::windows::core::Result<super::super::System::Com::IDataObject>;
    fn GetEmbedded(&mut self, acppos: i32, rguidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn QueryInsertEmbedded(&mut self, pguidservice: *const ::windows::core::GUID, pformatetc: *const super::super::System::Com::FORMATETC) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn InsertEmbedded(&mut self, dwflags: u32, acpstart: i32, acpend: i32, pdataobject: ::core::option::Option<super::super::System::Com::IDataObject>) -> ::windows::core::Result<TS_TEXTCHANGE>;
    fn InsertTextAtSelection(&mut self, dwflags: u32, pchtext: super::super::Foundation::PWSTR, cch: u32, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::Result<()>;
    fn InsertEmbeddedAtSelection(&mut self, dwflags: u32, pdataobject: ::core::option::Option<super::super::System::Com::IDataObject>, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::Result<()>;
    fn RequestSupportedAttrs(&mut self, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RequestAttrsAtPosition(&mut self, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::Result<()>;
    fn RequestAttrsTransitioningAtPosition(&mut self, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::Result<()>;
    fn FindNextAttrTransition(&mut self, acpstart: i32, acphalt: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32, pacpnext: *mut i32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> ::windows::core::Result<()>;
    fn RetrieveRequestedAttrs(&mut self, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn GetEndACP(&mut self) -> ::windows::core::Result<i32>;
    fn GetActiveView(&mut self) -> ::windows::core::Result<u32>;
    fn GetACPFromPoint(&mut self, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32) -> ::windows::core::Result<i32>;
    fn GetTextExt(&mut self, vcview: u32, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetScreenExt(&mut self, vcview: u32) -> ::windows::core::Result<super::super::Foundation::RECT>;
    fn GetWnd(&mut self, vcview: u32) -> ::windows::core::Result<super::super::Foundation::HWND>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextStoreACPVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextStoreACPImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextStoreACPVtbl {
        unsafe extern "system" fn AdviseSink<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void, dwmask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AdviseSink(::core::mem::transmute_copy(&riid), ::core::mem::transmute(&punk), ::core::mem::transmute_copy(&dwmask)).into()
        }
        unsafe extern "system" fn UnadviseSink<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnadviseSink(::core::mem::transmute(&punk)).into()
        }
        unsafe extern "system" fn RequestLock<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlockflags: u32, phrsession: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestLock(::core::mem::transmute_copy(&dwlockflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *phrsession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdcs: *mut TS_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pdcs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryInsert<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpteststart: i32, acptestend: i32, cch: u32, pacpresultstart: *mut i32, pacpresultend: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryInsert(::core::mem::transmute_copy(&acpteststart), ::core::mem::transmute_copy(&acptestend), ::core::mem::transmute_copy(&cch), ::core::mem::transmute_copy(&pacpresultstart), ::core::mem::transmute_copy(&pacpresultend)).into()
        }
        unsafe extern "system" fn GetSelection<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ACP, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSelection(::core::mem::transmute_copy(&ulindex), ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pselection), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn SetSelection<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, pselection: *const TS_SELECTION_ACP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelection(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pselection)).into()
        }
        unsafe extern "system" fn GetText<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, pchplain: super::super::Foundation::PWSTR, cchplainreq: u32, pcchplainret: *mut u32, prgruninfo: *mut TS_RUNINFO, cruninforeq: u32, pcruninforet: *mut u32, pacpnext: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetText(::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend), ::core::mem::transmute_copy(&pchplain), ::core::mem::transmute_copy(&cchplainreq), ::core::mem::transmute_copy(&pcchplainret), ::core::mem::transmute_copy(&prgruninfo), ::core::mem::transmute_copy(&cruninforeq), ::core::mem::transmute_copy(&pcruninforet), ::core::mem::transmute_copy(&pacpnext)).into()
        }
        unsafe extern "system" fn SetText<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, acpstart: i32, acpend: i32, pchtext: super::super::Foundation::PWSTR, cch: u32, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetText(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend), ::core::mem::transmute_copy(&pchtext), ::core::mem::transmute_copy(&cch)) {
                ::core::result::Result::Ok(ok__) => {
                    *pchange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormattedText<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, ppdataobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFormattedText(::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdataobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEmbedded<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acppos: i32, rguidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEmbedded(::core::mem::transmute_copy(&acppos), ::core::mem::transmute_copy(&rguidservice), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryInsertEmbedded<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidservice: *const ::windows::core::GUID, pformatetc: *const super::super::System::Com::FORMATETC, pfinsertable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryInsertEmbedded(::core::mem::transmute_copy(&pguidservice), ::core::mem::transmute_copy(&pformatetc)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfinsertable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertEmbedded<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, acpstart: i32, acpend: i32, pdataobject: ::windows::core::RawPtr, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InsertEmbedded(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend), ::core::mem::transmute(&pdataobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *pchange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertTextAtSelection<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pchtext: super::super::Foundation::PWSTR, cch: u32, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertTextAtSelection(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pchtext), ::core::mem::transmute_copy(&cch), ::core::mem::transmute_copy(&pacpstart), ::core::mem::transmute_copy(&pacpend), ::core::mem::transmute_copy(&pchange)).into()
        }
        unsafe extern "system" fn InsertEmbeddedAtSelection<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pdataobject: ::windows::core::RawPtr, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertEmbeddedAtSelection(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pdataobject), ::core::mem::transmute_copy(&pacpstart), ::core::mem::transmute_copy(&pacpend), ::core::mem::transmute_copy(&pchange)).into()
        }
        unsafe extern "system" fn RequestSupportedAttrs<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestSupportedAttrs(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&cfilterattrs), ::core::mem::transmute_copy(&pafilterattrs)).into()
        }
        unsafe extern "system" fn RequestAttrsAtPosition<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestAttrsAtPosition(::core::mem::transmute_copy(&acppos), ::core::mem::transmute_copy(&cfilterattrs), ::core::mem::transmute_copy(&pafilterattrs), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn RequestAttrsTransitioningAtPosition<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestAttrsTransitioningAtPosition(::core::mem::transmute_copy(&acppos), ::core::mem::transmute_copy(&cfilterattrs), ::core::mem::transmute_copy(&pafilterattrs), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn FindNextAttrTransition<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpstart: i32, acphalt: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32, pacpnext: *mut i32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FindNextAttrTransition(::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acphalt), ::core::mem::transmute_copy(&cfilterattrs), ::core::mem::transmute_copy(&pafilterattrs), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pacpnext), ::core::mem::transmute_copy(&pffound), ::core::mem::transmute_copy(&plfoundoffset)).into()
        }
        unsafe extern "system" fn RetrieveRequestedAttrs<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RetrieveRequestedAttrs(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&paattrvals), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn GetEndACP<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pacp: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEndACP() {
                ::core::result::Result::Ok(ok__) => {
                    *pacp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActiveView<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvcview: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActiveView() {
                ::core::result::Result::Ok(ok__) => {
                    *pvcview = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetACPFromPoint<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32, pacp: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetACPFromPoint(::core::mem::transmute_copy(&vcview), ::core::mem::transmute_copy(&ptscreen), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pacp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTextExt<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcview: u32, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTextExt(::core::mem::transmute_copy(&vcview), ::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend), ::core::mem::transmute_copy(&prc), ::core::mem::transmute_copy(&pfclipped)).into()
        }
        unsafe extern "system" fn GetScreenExt<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcview: u32, prc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetScreenExt(::core::mem::transmute_copy(&vcview)) {
                ::core::result::Result::Ok(ok__) => {
                    *prc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWnd<Impl: ITextStoreACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcview: u32, phwnd: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWnd(::core::mem::transmute_copy(&vcview)) {
                ::core::result::Result::Ok(ok__) => {
                    *phwnd = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AdviseSink: AdviseSink::<Impl, IMPL_OFFSET>,
            UnadviseSink: UnadviseSink::<Impl, IMPL_OFFSET>,
            RequestLock: RequestLock::<Impl, IMPL_OFFSET>,
            GetStatus: GetStatus::<Impl, IMPL_OFFSET>,
            QueryInsert: QueryInsert::<Impl, IMPL_OFFSET>,
            GetSelection: GetSelection::<Impl, IMPL_OFFSET>,
            SetSelection: SetSelection::<Impl, IMPL_OFFSET>,
            GetText: GetText::<Impl, IMPL_OFFSET>,
            SetText: SetText::<Impl, IMPL_OFFSET>,
            GetFormattedText: GetFormattedText::<Impl, IMPL_OFFSET>,
            GetEmbedded: GetEmbedded::<Impl, IMPL_OFFSET>,
            QueryInsertEmbedded: QueryInsertEmbedded::<Impl, IMPL_OFFSET>,
            InsertEmbedded: InsertEmbedded::<Impl, IMPL_OFFSET>,
            InsertTextAtSelection: InsertTextAtSelection::<Impl, IMPL_OFFSET>,
            InsertEmbeddedAtSelection: InsertEmbeddedAtSelection::<Impl, IMPL_OFFSET>,
            RequestSupportedAttrs: RequestSupportedAttrs::<Impl, IMPL_OFFSET>,
            RequestAttrsAtPosition: RequestAttrsAtPosition::<Impl, IMPL_OFFSET>,
            RequestAttrsTransitioningAtPosition: RequestAttrsTransitioningAtPosition::<Impl, IMPL_OFFSET>,
            FindNextAttrTransition: FindNextAttrTransition::<Impl, IMPL_OFFSET>,
            RetrieveRequestedAttrs: RetrieveRequestedAttrs::<Impl, IMPL_OFFSET>,
            GetEndACP: GetEndACP::<Impl, IMPL_OFFSET>,
            GetActiveView: GetActiveView::<Impl, IMPL_OFFSET>,
            GetACPFromPoint: GetACPFromPoint::<Impl, IMPL_OFFSET>,
            GetTextExt: GetTextExt::<Impl, IMPL_OFFSET>,
            GetScreenExt: GetScreenExt::<Impl, IMPL_OFFSET>,
            GetWnd: GetWnd::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextStoreACP as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextStoreACP2Impl: Sized {
    fn AdviseSink(&mut self, riid: *const ::windows::core::GUID, punk: ::core::option::Option<::windows::core::IUnknown>, dwmask: u32) -> ::windows::core::Result<()>;
    fn UnadviseSink(&mut self, punk: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn RequestLock(&mut self, dwlockflags: u32) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn GetStatus(&mut self) -> ::windows::core::Result<TS_STATUS>;
    fn QueryInsert(&mut self, acpteststart: i32, acptestend: i32, cch: u32, pacpresultstart: *mut i32, pacpresultend: *mut i32) -> ::windows::core::Result<()>;
    fn GetSelection(&mut self, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ACP, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn SetSelection(&mut self, ulcount: u32, pselection: *const TS_SELECTION_ACP) -> ::windows::core::Result<()>;
    fn GetText(&mut self, acpstart: i32, acpend: i32, pchplain: super::super::Foundation::PWSTR, cchplainreq: u32, pcchplainret: *mut u32, prgruninfo: *mut TS_RUNINFO, cruninforeq: u32, pcruninforet: *mut u32, pacpnext: *mut i32) -> ::windows::core::Result<()>;
    fn SetText(&mut self, dwflags: u32, acpstart: i32, acpend: i32, pchtext: super::super::Foundation::PWSTR, cch: u32) -> ::windows::core::Result<TS_TEXTCHANGE>;
    fn GetFormattedText(&mut self, acpstart: i32, acpend: i32) -> ::windows::core::Result<super::super::System::Com::IDataObject>;
    fn GetEmbedded(&mut self, acppos: i32, rguidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn QueryInsertEmbedded(&mut self, pguidservice: *const ::windows::core::GUID, pformatetc: *const super::super::System::Com::FORMATETC) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn InsertEmbedded(&mut self, dwflags: u32, acpstart: i32, acpend: i32, pdataobject: ::core::option::Option<super::super::System::Com::IDataObject>) -> ::windows::core::Result<TS_TEXTCHANGE>;
    fn InsertTextAtSelection(&mut self, dwflags: u32, pchtext: super::super::Foundation::PWSTR, cch: u32, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::Result<()>;
    fn InsertEmbeddedAtSelection(&mut self, dwflags: u32, pdataobject: ::core::option::Option<super::super::System::Com::IDataObject>, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::Result<()>;
    fn RequestSupportedAttrs(&mut self, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RequestAttrsAtPosition(&mut self, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::Result<()>;
    fn RequestAttrsTransitioningAtPosition(&mut self, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::Result<()>;
    fn FindNextAttrTransition(&mut self, acpstart: i32, acphalt: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32, pacpnext: *mut i32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> ::windows::core::Result<()>;
    fn RetrieveRequestedAttrs(&mut self, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn GetEndACP(&mut self) -> ::windows::core::Result<i32>;
    fn GetActiveView(&mut self) -> ::windows::core::Result<u32>;
    fn GetACPFromPoint(&mut self, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32) -> ::windows::core::Result<i32>;
    fn GetTextExt(&mut self, vcview: u32, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetScreenExt(&mut self, vcview: u32) -> ::windows::core::Result<super::super::Foundation::RECT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextStoreACP2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextStoreACP2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextStoreACP2Vtbl {
        unsafe extern "system" fn AdviseSink<Impl: ITextStoreACP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void, dwmask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AdviseSink(::core::mem::transmute_copy(&riid), ::core::mem::transmute(&punk), ::core::mem::transmute_copy(&dwmask)).into()
        }
        unsafe extern "system" fn UnadviseSink<Impl: ITextStoreACP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnadviseSink(::core::mem::transmute(&punk)).into()
        }
        unsafe extern "system" fn RequestLock<Impl: ITextStoreACP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlockflags: u32, phrsession: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestLock(::core::mem::transmute_copy(&dwlockflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *phrsession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: ITextStoreACP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdcs: *mut TS_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pdcs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryInsert<Impl: ITextStoreACP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpteststart: i32, acptestend: i32, cch: u32, pacpresultstart: *mut i32, pacpresultend: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryInsert(::core::mem::transmute_copy(&acpteststart), ::core::mem::transmute_copy(&acptestend), ::core::mem::transmute_copy(&cch), ::core::mem::transmute_copy(&pacpresultstart), ::core::mem::transmute_copy(&pacpresultend)).into()
        }
        unsafe extern "system" fn GetSelection<Impl: ITextStoreACP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ACP, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSelection(::core::mem::transmute_copy(&ulindex), ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pselection), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn SetSelection<Impl: ITextStoreACP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, pselection: *const TS_SELECTION_ACP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelection(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pselection)).into()
        }
        unsafe extern "system" fn GetText<Impl: ITextStoreACP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, pchplain: super::super::Foundation::PWSTR, cchplainreq: u32, pcchplainret: *mut u32, prgruninfo: *mut TS_RUNINFO, cruninforeq: u32, pcruninforet: *mut u32, pacpnext: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetText(::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend), ::core::mem::transmute_copy(&pchplain), ::core::mem::transmute_copy(&cchplainreq), ::core::mem::transmute_copy(&pcchplainret), ::core::mem::transmute_copy(&prgruninfo), ::core::mem::transmute_copy(&cruninforeq), ::core::mem::transmute_copy(&pcruninforet), ::core::mem::transmute_copy(&pacpnext)).into()
        }
        unsafe extern "system" fn SetText<Impl: ITextStoreACP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, acpstart: i32, acpend: i32, pchtext: super::super::Foundation::PWSTR, cch: u32, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetText(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend), ::core::mem::transmute_copy(&pchtext), ::core::mem::transmute_copy(&cch)) {
                ::core::result::Result::Ok(ok__) => {
                    *pchange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormattedText<Impl: ITextStoreACP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, ppdataobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFormattedText(::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdataobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEmbedded<Impl: ITextStoreACP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acppos: i32, rguidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEmbedded(::core::mem::transmute_copy(&acppos), ::core::mem::transmute_copy(&rguidservice), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryInsertEmbedded<Impl: ITextStoreACP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidservice: *const ::windows::core::GUID, pformatetc: *const super::super::System::Com::FORMATETC, pfinsertable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryInsertEmbedded(::core::mem::transmute_copy(&pguidservice), ::core::mem::transmute_copy(&pformatetc)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfinsertable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertEmbedded<Impl: ITextStoreACP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, acpstart: i32, acpend: i32, pdataobject: ::windows::core::RawPtr, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InsertEmbedded(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend), ::core::mem::transmute(&pdataobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *pchange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertTextAtSelection<Impl: ITextStoreACP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pchtext: super::super::Foundation::PWSTR, cch: u32, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertTextAtSelection(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pchtext), ::core::mem::transmute_copy(&cch), ::core::mem::transmute_copy(&pacpstart), ::core::mem::transmute_copy(&pacpend), ::core::mem::transmute_copy(&pchange)).into()
        }
        unsafe extern "system" fn InsertEmbeddedAtSelection<Impl: ITextStoreACP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pdataobject: ::windows::core::RawPtr, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertEmbeddedAtSelection(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pdataobject), ::core::mem::transmute_copy(&pacpstart), ::core::mem::transmute_copy(&pacpend), ::core::mem::transmute_copy(&pchange)).into()
        }
        unsafe extern "system" fn RequestSupportedAttrs<Impl: ITextStoreACP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestSupportedAttrs(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&cfilterattrs), ::core::mem::transmute_copy(&pafilterattrs)).into()
        }
        unsafe extern "system" fn RequestAttrsAtPosition<Impl: ITextStoreACP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestAttrsAtPosition(::core::mem::transmute_copy(&acppos), ::core::mem::transmute_copy(&cfilterattrs), ::core::mem::transmute_copy(&pafilterattrs), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn RequestAttrsTransitioningAtPosition<Impl: ITextStoreACP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestAttrsTransitioningAtPosition(::core::mem::transmute_copy(&acppos), ::core::mem::transmute_copy(&cfilterattrs), ::core::mem::transmute_copy(&pafilterattrs), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn FindNextAttrTransition<Impl: ITextStoreACP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpstart: i32, acphalt: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32, pacpnext: *mut i32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FindNextAttrTransition(::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acphalt), ::core::mem::transmute_copy(&cfilterattrs), ::core::mem::transmute_copy(&pafilterattrs), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pacpnext), ::core::mem::transmute_copy(&pffound), ::core::mem::transmute_copy(&plfoundoffset)).into()
        }
        unsafe extern "system" fn RetrieveRequestedAttrs<Impl: ITextStoreACP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RetrieveRequestedAttrs(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&paattrvals), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn GetEndACP<Impl: ITextStoreACP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pacp: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEndACP() {
                ::core::result::Result::Ok(ok__) => {
                    *pacp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActiveView<Impl: ITextStoreACP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvcview: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActiveView() {
                ::core::result::Result::Ok(ok__) => {
                    *pvcview = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetACPFromPoint<Impl: ITextStoreACP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32, pacp: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetACPFromPoint(::core::mem::transmute_copy(&vcview), ::core::mem::transmute_copy(&ptscreen), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pacp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTextExt<Impl: ITextStoreACP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcview: u32, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTextExt(::core::mem::transmute_copy(&vcview), ::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend), ::core::mem::transmute_copy(&prc), ::core::mem::transmute_copy(&pfclipped)).into()
        }
        unsafe extern "system" fn GetScreenExt<Impl: ITextStoreACP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcview: u32, prc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetScreenExt(::core::mem::transmute_copy(&vcview)) {
                ::core::result::Result::Ok(ok__) => {
                    *prc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AdviseSink: AdviseSink::<Impl, IMPL_OFFSET>,
            UnadviseSink: UnadviseSink::<Impl, IMPL_OFFSET>,
            RequestLock: RequestLock::<Impl, IMPL_OFFSET>,
            GetStatus: GetStatus::<Impl, IMPL_OFFSET>,
            QueryInsert: QueryInsert::<Impl, IMPL_OFFSET>,
            GetSelection: GetSelection::<Impl, IMPL_OFFSET>,
            SetSelection: SetSelection::<Impl, IMPL_OFFSET>,
            GetText: GetText::<Impl, IMPL_OFFSET>,
            SetText: SetText::<Impl, IMPL_OFFSET>,
            GetFormattedText: GetFormattedText::<Impl, IMPL_OFFSET>,
            GetEmbedded: GetEmbedded::<Impl, IMPL_OFFSET>,
            QueryInsertEmbedded: QueryInsertEmbedded::<Impl, IMPL_OFFSET>,
            InsertEmbedded: InsertEmbedded::<Impl, IMPL_OFFSET>,
            InsertTextAtSelection: InsertTextAtSelection::<Impl, IMPL_OFFSET>,
            InsertEmbeddedAtSelection: InsertEmbeddedAtSelection::<Impl, IMPL_OFFSET>,
            RequestSupportedAttrs: RequestSupportedAttrs::<Impl, IMPL_OFFSET>,
            RequestAttrsAtPosition: RequestAttrsAtPosition::<Impl, IMPL_OFFSET>,
            RequestAttrsTransitioningAtPosition: RequestAttrsTransitioningAtPosition::<Impl, IMPL_OFFSET>,
            FindNextAttrTransition: FindNextAttrTransition::<Impl, IMPL_OFFSET>,
            RetrieveRequestedAttrs: RetrieveRequestedAttrs::<Impl, IMPL_OFFSET>,
            GetEndACP: GetEndACP::<Impl, IMPL_OFFSET>,
            GetActiveView: GetActiveView::<Impl, IMPL_OFFSET>,
            GetACPFromPoint: GetACPFromPoint::<Impl, IMPL_OFFSET>,
            GetTextExt: GetTextExt::<Impl, IMPL_OFFSET>,
            GetScreenExt: GetScreenExt::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextStoreACP2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITextStoreACPExImpl: Sized {
    fn ScrollToRect(&mut self, acpstart: i32, acpend: i32, rc: super::super::Foundation::RECT, dwposition: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITextStoreACPExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextStoreACPExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextStoreACPExVtbl {
        unsafe extern "system" fn ScrollToRect<Impl: ITextStoreACPExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, rc: super::super::Foundation::RECT, dwposition: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ScrollToRect(::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend), ::core::mem::transmute_copy(&rc), ::core::mem::transmute_copy(&dwposition)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), ScrollToRect: ScrollToRect::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextStoreACPEx as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextStoreACPServicesImpl: Sized {
    fn Serialize(&mut self, pprop: ::core::option::Option<ITfProperty>, prange: ::core::option::Option<ITfRange>, phdr: *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: ::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn Unserialize(&mut self, pprop: ::core::option::Option<ITfProperty>, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: ::core::option::Option<super::super::System::Com::IStream>, ploader: ::core::option::Option<ITfPersistentPropertyLoaderACP>) -> ::windows::core::Result<()>;
    fn ForceLoadProperty(&mut self, pprop: ::core::option::Option<ITfProperty>) -> ::windows::core::Result<()>;
    fn CreateRange(&mut self, acpstart: i32, acpend: i32) -> ::windows::core::Result<ITfRangeACP>;
}
#[cfg(feature = "Win32_System_Com")]
impl ITextStoreACPServicesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextStoreACPServicesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextStoreACPServicesVtbl {
        unsafe extern "system" fn Serialize<Impl: ITextStoreACPServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprop: ::windows::core::RawPtr, prange: ::windows::core::RawPtr, phdr: *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Serialize(::core::mem::transmute(&pprop), ::core::mem::transmute(&prange), ::core::mem::transmute_copy(&phdr), ::core::mem::transmute(&pstream)).into()
        }
        unsafe extern "system" fn Unserialize<Impl: ITextStoreACPServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprop: ::windows::core::RawPtr, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: ::windows::core::RawPtr, ploader: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unserialize(::core::mem::transmute(&pprop), ::core::mem::transmute_copy(&phdr), ::core::mem::transmute(&pstream), ::core::mem::transmute(&ploader)).into()
        }
        unsafe extern "system" fn ForceLoadProperty<Impl: ITextStoreACPServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprop: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ForceLoadProperty(::core::mem::transmute(&pprop)).into()
        }
        unsafe extern "system" fn CreateRange<Impl: ITextStoreACPServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRange(::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Serialize: Serialize::<Impl, IMPL_OFFSET>,
            Unserialize: Unserialize::<Impl, IMPL_OFFSET>,
            ForceLoadProperty: ForceLoadProperty::<Impl, IMPL_OFFSET>,
            CreateRange: CreateRange::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextStoreACPServices as ::windows::core::Interface>::IID
    }
}
pub trait ITextStoreACPSinkImpl: Sized {
    fn OnTextChange(&mut self, dwflags: TEXT_STORE_TEXT_CHANGE_FLAGS, pchange: *const TS_TEXTCHANGE) -> ::windows::core::Result<()>;
    fn OnSelectionChange(&mut self) -> ::windows::core::Result<()>;
    fn OnLayoutChange(&mut self, lcode: TsLayoutCode, vcview: u32) -> ::windows::core::Result<()>;
    fn OnStatusChange(&mut self, dwflags: u32) -> ::windows::core::Result<()>;
    fn OnAttrsChange(&mut self, acpstart: i32, acpend: i32, cattrs: u32, paattrs: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnLockGranted(&mut self, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> ::windows::core::Result<()>;
    fn OnStartEditTransaction(&mut self) -> ::windows::core::Result<()>;
    fn OnEndEditTransaction(&mut self) -> ::windows::core::Result<()>;
}
impl ITextStoreACPSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextStoreACPSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextStoreACPSinkVtbl {
        unsafe extern "system" fn OnTextChange<Impl: ITextStoreACPSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: TEXT_STORE_TEXT_CHANGE_FLAGS, pchange: *const TS_TEXTCHANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnTextChange(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pchange)).into()
        }
        unsafe extern "system" fn OnSelectionChange<Impl: ITextStoreACPSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnSelectionChange().into()
        }
        unsafe extern "system" fn OnLayoutChange<Impl: ITextStoreACPSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcode: TsLayoutCode, vcview: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnLayoutChange(::core::mem::transmute_copy(&lcode), ::core::mem::transmute_copy(&vcview)).into()
        }
        unsafe extern "system" fn OnStatusChange<Impl: ITextStoreACPSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnStatusChange(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn OnAttrsChange<Impl: ITextStoreACPSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, cattrs: u32, paattrs: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnAttrsChange(::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend), ::core::mem::transmute_copy(&cattrs), ::core::mem::transmute_copy(&paattrs)).into()
        }
        unsafe extern "system" fn OnLockGranted<Impl: ITextStoreACPSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnLockGranted(::core::mem::transmute_copy(&dwlockflags)).into()
        }
        unsafe extern "system" fn OnStartEditTransaction<Impl: ITextStoreACPSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnStartEditTransaction().into()
        }
        unsafe extern "system" fn OnEndEditTransaction<Impl: ITextStoreACPSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnEndEditTransaction().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnTextChange: OnTextChange::<Impl, IMPL_OFFSET>,
            OnSelectionChange: OnSelectionChange::<Impl, IMPL_OFFSET>,
            OnLayoutChange: OnLayoutChange::<Impl, IMPL_OFFSET>,
            OnStatusChange: OnStatusChange::<Impl, IMPL_OFFSET>,
            OnAttrsChange: OnAttrsChange::<Impl, IMPL_OFFSET>,
            OnLockGranted: OnLockGranted::<Impl, IMPL_OFFSET>,
            OnStartEditTransaction: OnStartEditTransaction::<Impl, IMPL_OFFSET>,
            OnEndEditTransaction: OnEndEditTransaction::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextStoreACPSink as ::windows::core::Interface>::IID
    }
}
pub trait ITextStoreACPSinkExImpl: Sized + ITextStoreACPSinkImpl {
    fn OnDisconnect(&mut self) -> ::windows::core::Result<()>;
}
impl ITextStoreACPSinkExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextStoreACPSinkExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextStoreACPSinkExVtbl {
        unsafe extern "system" fn OnDisconnect<Impl: ITextStoreACPSinkExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDisconnect().into()
        }
        Self { base: ITextStoreACPSinkVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), OnDisconnect: OnDisconnect::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextStoreACPSinkEx as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextStoreAnchorImpl: Sized {
    fn AdviseSink(&mut self, riid: *const ::windows::core::GUID, punk: ::core::option::Option<::windows::core::IUnknown>, dwmask: u32) -> ::windows::core::Result<()>;
    fn UnadviseSink(&mut self, punk: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn RequestLock(&mut self, dwlockflags: u32) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn GetStatus(&mut self) -> ::windows::core::Result<TS_STATUS>;
    fn QueryInsert(&mut self, pateststart: ::core::option::Option<IAnchor>, patestend: ::core::option::Option<IAnchor>, cch: u32, pparesultstart: *mut ::core::option::Option<IAnchor>, pparesultend: *mut ::core::option::Option<IAnchor>) -> ::windows::core::Result<()>;
    fn GetSelection(&mut self, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ANCHOR, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn SetSelection(&mut self, ulcount: u32, pselection: *const TS_SELECTION_ANCHOR) -> ::windows::core::Result<()>;
    fn GetText(&mut self, dwflags: u32, pastart: ::core::option::Option<IAnchor>, paend: ::core::option::Option<IAnchor>, pchtext: super::super::Foundation::PWSTR, cchreq: u32, pcch: *mut u32, fupdateanchor: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetText(&mut self, dwflags: u32, pastart: ::core::option::Option<IAnchor>, paend: ::core::option::Option<IAnchor>, pchtext: super::super::Foundation::PWSTR, cch: u32) -> ::windows::core::Result<()>;
    fn GetFormattedText(&mut self, pastart: ::core::option::Option<IAnchor>, paend: ::core::option::Option<IAnchor>) -> ::windows::core::Result<super::super::System::Com::IDataObject>;
    fn GetEmbedded(&mut self, dwflags: u32, papos: ::core::option::Option<IAnchor>, rguidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn InsertEmbedded(&mut self, dwflags: u32, pastart: ::core::option::Option<IAnchor>, paend: ::core::option::Option<IAnchor>, pdataobject: ::core::option::Option<super::super::System::Com::IDataObject>) -> ::windows::core::Result<()>;
    fn RequestSupportedAttrs(&mut self, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RequestAttrsAtPosition(&mut self, papos: ::core::option::Option<IAnchor>, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::Result<()>;
    fn RequestAttrsTransitioningAtPosition(&mut self, papos: ::core::option::Option<IAnchor>, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::Result<()>;
    fn FindNextAttrTransition(&mut self, pastart: ::core::option::Option<IAnchor>, pahalt: ::core::option::Option<IAnchor>, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> ::windows::core::Result<()>;
    fn RetrieveRequestedAttrs(&mut self, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn GetStart(&mut self) -> ::windows::core::Result<IAnchor>;
    fn GetEnd(&mut self) -> ::windows::core::Result<IAnchor>;
    fn GetActiveView(&mut self) -> ::windows::core::Result<u32>;
    fn GetAnchorFromPoint(&mut self, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32) -> ::windows::core::Result<IAnchor>;
    fn GetTextExt(&mut self, vcview: u32, pastart: ::core::option::Option<IAnchor>, paend: ::core::option::Option<IAnchor>, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetScreenExt(&mut self, vcview: u32) -> ::windows::core::Result<super::super::Foundation::RECT>;
    fn GetWnd(&mut self, vcview: u32) -> ::windows::core::Result<super::super::Foundation::HWND>;
    fn QueryInsertEmbedded(&mut self, pguidservice: *const ::windows::core::GUID, pformatetc: *const super::super::System::Com::FORMATETC) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn InsertTextAtSelection(&mut self, dwflags: u32, pchtext: super::super::Foundation::PWSTR, cch: u32, ppastart: *mut ::core::option::Option<IAnchor>, ppaend: *mut ::core::option::Option<IAnchor>) -> ::windows::core::Result<()>;
    fn InsertEmbeddedAtSelection(&mut self, dwflags: u32, pdataobject: ::core::option::Option<super::super::System::Com::IDataObject>, ppastart: *mut ::core::option::Option<IAnchor>, ppaend: *mut ::core::option::Option<IAnchor>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextStoreAnchorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextStoreAnchorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextStoreAnchorVtbl {
        unsafe extern "system" fn AdviseSink<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void, dwmask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AdviseSink(::core::mem::transmute_copy(&riid), ::core::mem::transmute(&punk), ::core::mem::transmute_copy(&dwmask)).into()
        }
        unsafe extern "system" fn UnadviseSink<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnadviseSink(::core::mem::transmute(&punk)).into()
        }
        unsafe extern "system" fn RequestLock<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlockflags: u32, phrsession: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestLock(::core::mem::transmute_copy(&dwlockflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *phrsession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdcs: *mut TS_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pdcs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryInsert<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pateststart: ::windows::core::RawPtr, patestend: ::windows::core::RawPtr, cch: u32, pparesultstart: *mut ::windows::core::RawPtr, pparesultend: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryInsert(::core::mem::transmute(&pateststart), ::core::mem::transmute(&patestend), ::core::mem::transmute_copy(&cch), ::core::mem::transmute_copy(&pparesultstart), ::core::mem::transmute_copy(&pparesultend)).into()
        }
        unsafe extern "system" fn GetSelection<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ANCHOR, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSelection(::core::mem::transmute_copy(&ulindex), ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pselection), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn SetSelection<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, pselection: *const TS_SELECTION_ANCHOR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelection(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pselection)).into()
        }
        unsafe extern "system" fn GetText<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pastart: ::windows::core::RawPtr, paend: ::windows::core::RawPtr, pchtext: super::super::Foundation::PWSTR, cchreq: u32, pcch: *mut u32, fupdateanchor: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetText(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pastart), ::core::mem::transmute(&paend), ::core::mem::transmute_copy(&pchtext), ::core::mem::transmute_copy(&cchreq), ::core::mem::transmute_copy(&pcch), ::core::mem::transmute_copy(&fupdateanchor)).into()
        }
        unsafe extern "system" fn SetText<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pastart: ::windows::core::RawPtr, paend: ::windows::core::RawPtr, pchtext: super::super::Foundation::PWSTR, cch: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetText(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pastart), ::core::mem::transmute(&paend), ::core::mem::transmute_copy(&pchtext), ::core::mem::transmute_copy(&cch)).into()
        }
        unsafe extern "system" fn GetFormattedText<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pastart: ::windows::core::RawPtr, paend: ::windows::core::RawPtr, ppdataobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFormattedText(::core::mem::transmute(&pastart), ::core::mem::transmute(&paend)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdataobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEmbedded<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, papos: ::windows::core::RawPtr, rguidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEmbedded(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&papos), ::core::mem::transmute_copy(&rguidservice), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertEmbedded<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pastart: ::windows::core::RawPtr, paend: ::windows::core::RawPtr, pdataobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertEmbedded(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pastart), ::core::mem::transmute(&paend), ::core::mem::transmute(&pdataobject)).into()
        }
        unsafe extern "system" fn RequestSupportedAttrs<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestSupportedAttrs(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&cfilterattrs), ::core::mem::transmute_copy(&pafilterattrs)).into()
        }
        unsafe extern "system" fn RequestAttrsAtPosition<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, papos: ::windows::core::RawPtr, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestAttrsAtPosition(::core::mem::transmute(&papos), ::core::mem::transmute_copy(&cfilterattrs), ::core::mem::transmute_copy(&pafilterattrs), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn RequestAttrsTransitioningAtPosition<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, papos: ::windows::core::RawPtr, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestAttrsTransitioningAtPosition(::core::mem::transmute(&papos), ::core::mem::transmute_copy(&cfilterattrs), ::core::mem::transmute_copy(&pafilterattrs), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn FindNextAttrTransition<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pastart: ::windows::core::RawPtr, pahalt: ::windows::core::RawPtr, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FindNextAttrTransition(::core::mem::transmute(&pastart), ::core::mem::transmute(&pahalt), ::core::mem::transmute_copy(&cfilterattrs), ::core::mem::transmute_copy(&pafilterattrs), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pffound), ::core::mem::transmute_copy(&plfoundoffset)).into()
        }
        unsafe extern "system" fn RetrieveRequestedAttrs<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RetrieveRequestedAttrs(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&paattrvals), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn GetStart<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppastart: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStart() {
                ::core::result::Result::Ok(ok__) => {
                    *ppastart = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnd<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaend: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnd() {
                ::core::result::Result::Ok(ok__) => {
                    *ppaend = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActiveView<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvcview: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActiveView() {
                ::core::result::Result::Ok(ok__) => {
                    *pvcview = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAnchorFromPoint<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32, ppasite: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAnchorFromPoint(::core::mem::transmute_copy(&vcview), ::core::mem::transmute_copy(&ptscreen), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppasite = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTextExt<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcview: u32, pastart: ::windows::core::RawPtr, paend: ::windows::core::RawPtr, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTextExt(::core::mem::transmute_copy(&vcview), ::core::mem::transmute(&pastart), ::core::mem::transmute(&paend), ::core::mem::transmute_copy(&prc), ::core::mem::transmute_copy(&pfclipped)).into()
        }
        unsafe extern "system" fn GetScreenExt<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcview: u32, prc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetScreenExt(::core::mem::transmute_copy(&vcview)) {
                ::core::result::Result::Ok(ok__) => {
                    *prc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWnd<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcview: u32, phwnd: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWnd(::core::mem::transmute_copy(&vcview)) {
                ::core::result::Result::Ok(ok__) => {
                    *phwnd = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryInsertEmbedded<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidservice: *const ::windows::core::GUID, pformatetc: *const super::super::System::Com::FORMATETC, pfinsertable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryInsertEmbedded(::core::mem::transmute_copy(&pguidservice), ::core::mem::transmute_copy(&pformatetc)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfinsertable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertTextAtSelection<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pchtext: super::super::Foundation::PWSTR, cch: u32, ppastart: *mut ::windows::core::RawPtr, ppaend: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertTextAtSelection(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pchtext), ::core::mem::transmute_copy(&cch), ::core::mem::transmute_copy(&ppastart), ::core::mem::transmute_copy(&ppaend)).into()
        }
        unsafe extern "system" fn InsertEmbeddedAtSelection<Impl: ITextStoreAnchorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pdataobject: ::windows::core::RawPtr, ppastart: *mut ::windows::core::RawPtr, ppaend: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertEmbeddedAtSelection(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pdataobject), ::core::mem::transmute_copy(&ppastart), ::core::mem::transmute_copy(&ppaend)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AdviseSink: AdviseSink::<Impl, IMPL_OFFSET>,
            UnadviseSink: UnadviseSink::<Impl, IMPL_OFFSET>,
            RequestLock: RequestLock::<Impl, IMPL_OFFSET>,
            GetStatus: GetStatus::<Impl, IMPL_OFFSET>,
            QueryInsert: QueryInsert::<Impl, IMPL_OFFSET>,
            GetSelection: GetSelection::<Impl, IMPL_OFFSET>,
            SetSelection: SetSelection::<Impl, IMPL_OFFSET>,
            GetText: GetText::<Impl, IMPL_OFFSET>,
            SetText: SetText::<Impl, IMPL_OFFSET>,
            GetFormattedText: GetFormattedText::<Impl, IMPL_OFFSET>,
            GetEmbedded: GetEmbedded::<Impl, IMPL_OFFSET>,
            InsertEmbedded: InsertEmbedded::<Impl, IMPL_OFFSET>,
            RequestSupportedAttrs: RequestSupportedAttrs::<Impl, IMPL_OFFSET>,
            RequestAttrsAtPosition: RequestAttrsAtPosition::<Impl, IMPL_OFFSET>,
            RequestAttrsTransitioningAtPosition: RequestAttrsTransitioningAtPosition::<Impl, IMPL_OFFSET>,
            FindNextAttrTransition: FindNextAttrTransition::<Impl, IMPL_OFFSET>,
            RetrieveRequestedAttrs: RetrieveRequestedAttrs::<Impl, IMPL_OFFSET>,
            GetStart: GetStart::<Impl, IMPL_OFFSET>,
            GetEnd: GetEnd::<Impl, IMPL_OFFSET>,
            GetActiveView: GetActiveView::<Impl, IMPL_OFFSET>,
            GetAnchorFromPoint: GetAnchorFromPoint::<Impl, IMPL_OFFSET>,
            GetTextExt: GetTextExt::<Impl, IMPL_OFFSET>,
            GetScreenExt: GetScreenExt::<Impl, IMPL_OFFSET>,
            GetWnd: GetWnd::<Impl, IMPL_OFFSET>,
            QueryInsertEmbedded: QueryInsertEmbedded::<Impl, IMPL_OFFSET>,
            InsertTextAtSelection: InsertTextAtSelection::<Impl, IMPL_OFFSET>,
            InsertEmbeddedAtSelection: InsertEmbeddedAtSelection::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextStoreAnchor as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITextStoreAnchorExImpl: Sized {
    fn ScrollToRect(&mut self, pstart: ::core::option::Option<IAnchor>, pend: ::core::option::Option<IAnchor>, rc: super::super::Foundation::RECT, dwposition: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITextStoreAnchorExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextStoreAnchorExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextStoreAnchorExVtbl {
        unsafe extern "system" fn ScrollToRect<Impl: ITextStoreAnchorExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstart: ::windows::core::RawPtr, pend: ::windows::core::RawPtr, rc: super::super::Foundation::RECT, dwposition: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ScrollToRect(::core::mem::transmute(&pstart), ::core::mem::transmute(&pend), ::core::mem::transmute_copy(&rc), ::core::mem::transmute_copy(&dwposition)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), ScrollToRect: ScrollToRect::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextStoreAnchorEx as ::windows::core::Interface>::IID
    }
}
pub trait ITextStoreAnchorSinkImpl: Sized {
    fn OnTextChange(&mut self, dwflags: TEXT_STORE_CHANGE_FLAGS, pastart: ::core::option::Option<IAnchor>, paend: ::core::option::Option<IAnchor>) -> ::windows::core::Result<()>;
    fn OnSelectionChange(&mut self) -> ::windows::core::Result<()>;
    fn OnLayoutChange(&mut self, lcode: TsLayoutCode, vcview: u32) -> ::windows::core::Result<()>;
    fn OnStatusChange(&mut self, dwflags: u32) -> ::windows::core::Result<()>;
    fn OnAttrsChange(&mut self, pastart: ::core::option::Option<IAnchor>, paend: ::core::option::Option<IAnchor>, cattrs: u32, paattrs: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnLockGranted(&mut self, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> ::windows::core::Result<()>;
    fn OnStartEditTransaction(&mut self) -> ::windows::core::Result<()>;
    fn OnEndEditTransaction(&mut self) -> ::windows::core::Result<()>;
}
impl ITextStoreAnchorSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextStoreAnchorSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextStoreAnchorSinkVtbl {
        unsafe extern "system" fn OnTextChange<Impl: ITextStoreAnchorSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: TEXT_STORE_CHANGE_FLAGS, pastart: ::windows::core::RawPtr, paend: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnTextChange(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pastart), ::core::mem::transmute(&paend)).into()
        }
        unsafe extern "system" fn OnSelectionChange<Impl: ITextStoreAnchorSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnSelectionChange().into()
        }
        unsafe extern "system" fn OnLayoutChange<Impl: ITextStoreAnchorSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcode: TsLayoutCode, vcview: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnLayoutChange(::core::mem::transmute_copy(&lcode), ::core::mem::transmute_copy(&vcview)).into()
        }
        unsafe extern "system" fn OnStatusChange<Impl: ITextStoreAnchorSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnStatusChange(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn OnAttrsChange<Impl: ITextStoreAnchorSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pastart: ::windows::core::RawPtr, paend: ::windows::core::RawPtr, cattrs: u32, paattrs: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnAttrsChange(::core::mem::transmute(&pastart), ::core::mem::transmute(&paend), ::core::mem::transmute_copy(&cattrs), ::core::mem::transmute_copy(&paattrs)).into()
        }
        unsafe extern "system" fn OnLockGranted<Impl: ITextStoreAnchorSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnLockGranted(::core::mem::transmute_copy(&dwlockflags)).into()
        }
        unsafe extern "system" fn OnStartEditTransaction<Impl: ITextStoreAnchorSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnStartEditTransaction().into()
        }
        unsafe extern "system" fn OnEndEditTransaction<Impl: ITextStoreAnchorSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnEndEditTransaction().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnTextChange: OnTextChange::<Impl, IMPL_OFFSET>,
            OnSelectionChange: OnSelectionChange::<Impl, IMPL_OFFSET>,
            OnLayoutChange: OnLayoutChange::<Impl, IMPL_OFFSET>,
            OnStatusChange: OnStatusChange::<Impl, IMPL_OFFSET>,
            OnAttrsChange: OnAttrsChange::<Impl, IMPL_OFFSET>,
            OnLockGranted: OnLockGranted::<Impl, IMPL_OFFSET>,
            OnStartEditTransaction: OnStartEditTransaction::<Impl, IMPL_OFFSET>,
            OnEndEditTransaction: OnEndEditTransaction::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextStoreAnchorSink as ::windows::core::Interface>::IID
    }
}
pub trait ITextStoreSinkAnchorExImpl: Sized + ITextStoreAnchorSinkImpl {
    fn OnDisconnect(&mut self) -> ::windows::core::Result<()>;
}
impl ITextStoreSinkAnchorExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextStoreSinkAnchorExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextStoreSinkAnchorExVtbl {
        unsafe extern "system" fn OnDisconnect<Impl: ITextStoreSinkAnchorExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDisconnect().into()
        }
        Self { base: ITextStoreAnchorSinkVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), OnDisconnect: OnDisconnect::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextStoreSinkAnchorEx as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfActiveLanguageProfileNotifySinkImpl: Sized {
    fn OnActivated(&mut self, clsid: *const ::windows::core::GUID, guidprofile: *const ::windows::core::GUID, factivated: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfActiveLanguageProfileNotifySinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfActiveLanguageProfileNotifySinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfActiveLanguageProfileNotifySinkVtbl {
        unsafe extern "system" fn OnActivated<Impl: ITfActiveLanguageProfileNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, guidprofile: *const ::windows::core::GUID, factivated: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnActivated(::core::mem::transmute_copy(&clsid), ::core::mem::transmute_copy(&guidprofile), ::core::mem::transmute_copy(&factivated)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnActivated: OnActivated::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfActiveLanguageProfileNotifySink as ::windows::core::Interface>::IID
    }
}
pub trait ITfCandidateListImpl: Sized {
    fn EnumCandidates(&mut self) -> ::windows::core::Result<IEnumTfCandidates>;
    fn GetCandidate(&mut self, nindex: u32) -> ::windows::core::Result<ITfCandidateString>;
    fn GetCandidateNum(&mut self) -> ::windows::core::Result<u32>;
    fn SetResult(&mut self, nindex: u32, imcr: TfCandidateResult) -> ::windows::core::Result<()>;
}
impl ITfCandidateListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfCandidateListImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfCandidateListVtbl {
        unsafe extern "system" fn EnumCandidates<Impl: ITfCandidateListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumCandidates() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCandidate<Impl: ITfCandidateListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, ppcand: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCandidate(::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcand = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCandidateNum<Impl: ITfCandidateListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pncnt: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCandidateNum() {
                ::core::result::Result::Ok(ok__) => {
                    *pncnt = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetResult<Impl: ITfCandidateListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, imcr: TfCandidateResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetResult(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&imcr)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            EnumCandidates: EnumCandidates::<Impl, IMPL_OFFSET>,
            GetCandidate: GetCandidate::<Impl, IMPL_OFFSET>,
            GetCandidateNum: GetCandidateNum::<Impl, IMPL_OFFSET>,
            SetResult: SetResult::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfCandidateList as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfCandidateListUIElementImpl: Sized + ITfUIElementImpl {
    fn GetUpdatedFlags(&mut self) -> ::windows::core::Result<u32>;
    fn GetDocumentMgr(&mut self) -> ::windows::core::Result<ITfDocumentMgr>;
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetSelection(&mut self) -> ::windows::core::Result<u32>;
    fn GetString(&mut self, uindex: u32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetPageIndex(&mut self, pindex: *mut u32, usize: u32, pupagecnt: *mut u32) -> ::windows::core::Result<()>;
    fn SetPageIndex(&mut self, pindex: *const u32, upagecnt: u32) -> ::windows::core::Result<()>;
    fn GetCurrentPage(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfCandidateListUIElementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfCandidateListUIElementImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfCandidateListUIElementVtbl {
        unsafe extern "system" fn GetUpdatedFlags<Impl: ITfCandidateListUIElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUpdatedFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocumentMgr<Impl: ITfCandidateListUIElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdim: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocumentMgr() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdim = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Impl: ITfCandidateListUIElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pucount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelection<Impl: ITfCandidateListUIElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *puindex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetString<Impl: ITfCandidateListUIElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, pstr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetString(::core::mem::transmute_copy(&uindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPageIndex<Impl: ITfCandidateListUIElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pindex: *mut u32, usize: u32, pupagecnt: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPageIndex(::core::mem::transmute_copy(&pindex), ::core::mem::transmute_copy(&usize), ::core::mem::transmute_copy(&pupagecnt)).into()
        }
        unsafe extern "system" fn SetPageIndex<Impl: ITfCandidateListUIElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pindex: *const u32, upagecnt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPageIndex(::core::mem::transmute_copy(&pindex), ::core::mem::transmute_copy(&upagecnt)).into()
        }
        unsafe extern "system" fn GetCurrentPage<Impl: ITfCandidateListUIElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pupage: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentPage() {
                ::core::result::Result::Ok(ok__) => {
                    *pupage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ITfUIElementVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetUpdatedFlags: GetUpdatedFlags::<Impl, IMPL_OFFSET>,
            GetDocumentMgr: GetDocumentMgr::<Impl, IMPL_OFFSET>,
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
            GetSelection: GetSelection::<Impl, IMPL_OFFSET>,
            GetString: GetString::<Impl, IMPL_OFFSET>,
            GetPageIndex: GetPageIndex::<Impl, IMPL_OFFSET>,
            SetPageIndex: SetPageIndex::<Impl, IMPL_OFFSET>,
            GetCurrentPage: GetCurrentPage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfCandidateListUIElement as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfCandidateListUIElementBehaviorImpl: Sized + ITfUIElementImpl + ITfCandidateListUIElementImpl {
    fn SetSelection(&mut self, nindex: u32) -> ::windows::core::Result<()>;
    fn Finalize(&mut self) -> ::windows::core::Result<()>;
    fn Abort(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfCandidateListUIElementBehaviorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfCandidateListUIElementBehaviorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfCandidateListUIElementBehaviorVtbl {
        unsafe extern "system" fn SetSelection<Impl: ITfCandidateListUIElementBehaviorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelection(::core::mem::transmute_copy(&nindex)).into()
        }
        unsafe extern "system" fn Finalize<Impl: ITfCandidateListUIElementBehaviorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Finalize().into()
        }
        unsafe extern "system" fn Abort<Impl: ITfCandidateListUIElementBehaviorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Abort().into()
        }
        Self {
            base: ITfCandidateListUIElementVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetSelection: SetSelection::<Impl, IMPL_OFFSET>,
            Finalize: Finalize::<Impl, IMPL_OFFSET>,
            Abort: Abort::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfCandidateListUIElementBehavior as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfCandidateStringImpl: Sized {
    fn GetString(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetIndex(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfCandidateStringVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfCandidateStringImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfCandidateStringVtbl {
        unsafe extern "system" fn GetString<Impl: ITfCandidateStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetString() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIndex<Impl: ITfCandidateStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *pnindex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetString: GetString::<Impl, IMPL_OFFSET>,
            GetIndex: GetIndex::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfCandidateString as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ITfCategoryMgrImpl: Sized {
    fn RegisterCategory(&mut self, rclsid: *const ::windows::core::GUID, rcatid: *const ::windows::core::GUID, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn UnregisterCategory(&mut self, rclsid: *const ::windows::core::GUID, rcatid: *const ::windows::core::GUID, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn EnumCategoriesInItem(&mut self, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::System::Com::IEnumGUID>;
    fn EnumItemsInCategory(&mut self, rcatid: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::System::Com::IEnumGUID>;
    fn FindClosestCategory(&mut self, rguid: *const ::windows::core::GUID, pcatid: *mut ::windows::core::GUID, ppcatidlist: *const *const ::windows::core::GUID, ulcount: u32) -> ::windows::core::Result<()>;
    fn RegisterGUIDDescription(&mut self, rclsid: *const ::windows::core::GUID, rguid: *const ::windows::core::GUID, pchdesc: super::super::Foundation::PWSTR, cch: u32) -> ::windows::core::Result<()>;
    fn UnregisterGUIDDescription(&mut self, rclsid: *const ::windows::core::GUID, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetGUIDDescription(&mut self, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn RegisterGUIDDWORD(&mut self, rclsid: *const ::windows::core::GUID, rguid: *const ::windows::core::GUID, dw: u32) -> ::windows::core::Result<()>;
    fn UnregisterGUIDDWORD(&mut self, rclsid: *const ::windows::core::GUID, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetGUIDDWORD(&mut self, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<u32>;
    fn RegisterGUID(&mut self, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<u32>;
    fn GetGUID(&mut self, guidatom: u32) -> ::windows::core::Result<::windows::core::GUID>;
    fn IsEqualTfGuidAtom(&mut self, guidatom: u32, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ITfCategoryMgrVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfCategoryMgrImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfCategoryMgrVtbl {
        unsafe extern "system" fn RegisterCategory<Impl: ITfCategoryMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, rcatid: *const ::windows::core::GUID, rguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterCategory(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&rcatid), ::core::mem::transmute_copy(&rguid)).into()
        }
        unsafe extern "system" fn UnregisterCategory<Impl: ITfCategoryMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, rcatid: *const ::windows::core::GUID, rguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterCategory(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&rcatid), ::core::mem::transmute_copy(&rguid)).into()
        }
        unsafe extern "system" fn EnumCategoriesInItem<Impl: ITfCategoryMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumCategoriesInItem(::core::mem::transmute_copy(&rguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumItemsInCategory<Impl: ITfCategoryMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rcatid: *const ::windows::core::GUID, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumItemsInCategory(::core::mem::transmute_copy(&rcatid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindClosestCategory<Impl: ITfCategoryMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, pcatid: *mut ::windows::core::GUID, ppcatidlist: *const *const ::windows::core::GUID, ulcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FindClosestCategory(::core::mem::transmute_copy(&rguid), ::core::mem::transmute_copy(&pcatid), ::core::mem::transmute_copy(&ppcatidlist), ::core::mem::transmute_copy(&ulcount)).into()
        }
        unsafe extern "system" fn RegisterGUIDDescription<Impl: ITfCategoryMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, rguid: *const ::windows::core::GUID, pchdesc: super::super::Foundation::PWSTR, cch: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterGUIDDescription(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&rguid), ::core::mem::transmute_copy(&pchdesc), ::core::mem::transmute_copy(&cch)).into()
        }
        unsafe extern "system" fn UnregisterGUIDDescription<Impl: ITfCategoryMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, rguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterGUIDDescription(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&rguid)).into()
        }
        unsafe extern "system" fn GetGUIDDescription<Impl: ITfCategoryMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, pbstrdesc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGUIDDescription(::core::mem::transmute_copy(&rguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterGUIDDWORD<Impl: ITfCategoryMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, rguid: *const ::windows::core::GUID, dw: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterGUIDDWORD(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&rguid), ::core::mem::transmute_copy(&dw)).into()
        }
        unsafe extern "system" fn UnregisterGUIDDWORD<Impl: ITfCategoryMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, rguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterGUIDDWORD(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&rguid)).into()
        }
        unsafe extern "system" fn GetGUIDDWORD<Impl: ITfCategoryMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, pdw: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGUIDDWORD(::core::mem::transmute_copy(&rguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdw = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterGUID<Impl: ITfCategoryMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, pguidatom: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterGUID(::core::mem::transmute_copy(&rguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pguidatom = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGUID<Impl: ITfCategoryMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidatom: u32, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGUID(::core::mem::transmute_copy(&guidatom)) {
                ::core::result::Result::Ok(ok__) => {
                    *pguid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqualTfGuidAtom<Impl: ITfCategoryMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidatom: u32, rguid: *const ::windows::core::GUID, pfequal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEqualTfGuidAtom(::core::mem::transmute_copy(&guidatom), ::core::mem::transmute_copy(&rguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfequal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            RegisterCategory: RegisterCategory::<Impl, IMPL_OFFSET>,
            UnregisterCategory: UnregisterCategory::<Impl, IMPL_OFFSET>,
            EnumCategoriesInItem: EnumCategoriesInItem::<Impl, IMPL_OFFSET>,
            EnumItemsInCategory: EnumItemsInCategory::<Impl, IMPL_OFFSET>,
            FindClosestCategory: FindClosestCategory::<Impl, IMPL_OFFSET>,
            RegisterGUIDDescription: RegisterGUIDDescription::<Impl, IMPL_OFFSET>,
            UnregisterGUIDDescription: UnregisterGUIDDescription::<Impl, IMPL_OFFSET>,
            GetGUIDDescription: GetGUIDDescription::<Impl, IMPL_OFFSET>,
            RegisterGUIDDWORD: RegisterGUIDDWORD::<Impl, IMPL_OFFSET>,
            UnregisterGUIDDWORD: UnregisterGUIDDWORD::<Impl, IMPL_OFFSET>,
            GetGUIDDWORD: GetGUIDDWORD::<Impl, IMPL_OFFSET>,
            RegisterGUID: RegisterGUID::<Impl, IMPL_OFFSET>,
            GetGUID: GetGUID::<Impl, IMPL_OFFSET>,
            IsEqualTfGuidAtom: IsEqualTfGuidAtom::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfCategoryMgr as ::windows::core::Interface>::IID
    }
}
pub trait ITfCleanupContextDurationSinkImpl: Sized {
    fn OnStartCleanupContext(&mut self) -> ::windows::core::Result<()>;
    fn OnEndCleanupContext(&mut self) -> ::windows::core::Result<()>;
}
impl ITfCleanupContextDurationSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfCleanupContextDurationSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfCleanupContextDurationSinkVtbl {
        unsafe extern "system" fn OnStartCleanupContext<Impl: ITfCleanupContextDurationSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnStartCleanupContext().into()
        }
        unsafe extern "system" fn OnEndCleanupContext<Impl: ITfCleanupContextDurationSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnEndCleanupContext().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnStartCleanupContext: OnStartCleanupContext::<Impl, IMPL_OFFSET>,
            OnEndCleanupContext: OnEndCleanupContext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfCleanupContextDurationSink as ::windows::core::Interface>::IID
    }
}
pub trait ITfCleanupContextSinkImpl: Sized {
    fn OnCleanupContext(&mut self, ecwrite: u32, pic: ::core::option::Option<ITfContext>) -> ::windows::core::Result<()>;
}
impl ITfCleanupContextSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfCleanupContextSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfCleanupContextSinkVtbl {
        unsafe extern "system" fn OnCleanupContext<Impl: ITfCleanupContextSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ecwrite: u32, pic: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnCleanupContext(::core::mem::transmute_copy(&ecwrite), ::core::mem::transmute(&pic)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnCleanupContext: OnCleanupContext::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfCleanupContextSink as ::windows::core::Interface>::IID
    }
}
pub trait ITfClientIdImpl: Sized {
    fn GetClientId(&mut self, rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<u32>;
}
impl ITfClientIdVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfClientIdImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfClientIdVtbl {
        unsafe extern "system" fn GetClientId<Impl: ITfClientIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, ptid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClientId(::core::mem::transmute_copy(&rclsid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ptid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetClientId: GetClientId::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfClientId as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITfCompartmentImpl: Sized {
    fn SetValue(&mut self, tid: u32, pvarvalue: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GetValue(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITfCompartmentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfCompartmentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfCompartmentVtbl {
        unsafe extern "system" fn SetValue<Impl: ITfCompartmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tid: u32, pvarvalue: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&tid), ::core::mem::transmute_copy(&pvarvalue)).into()
        }
        unsafe extern "system" fn GetValue<Impl: ITfCompartmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetValue() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
            GetValue: GetValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfCompartment as ::windows::core::Interface>::IID
    }
}
pub trait ITfCompartmentEventSinkImpl: Sized {
    fn OnChange(&mut self, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl ITfCompartmentEventSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfCompartmentEventSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfCompartmentEventSinkVtbl {
        unsafe extern "system" fn OnChange<Impl: ITfCompartmentEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnChange(::core::mem::transmute_copy(&rguid)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnChange: OnChange::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfCompartmentEventSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITfCompartmentMgrImpl: Sized {
    fn GetCompartment(&mut self, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<ITfCompartment>;
    fn ClearCompartment(&mut self, tid: u32, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn EnumCompartments(&mut self) -> ::windows::core::Result<super::super::System::Com::IEnumGUID>;
}
#[cfg(feature = "Win32_System_Com")]
impl ITfCompartmentMgrVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfCompartmentMgrImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfCompartmentMgrVtbl {
        unsafe extern "system" fn GetCompartment<Impl: ITfCompartmentMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, ppcomp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCompartment(::core::mem::transmute_copy(&rguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcomp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearCompartment<Impl: ITfCompartmentMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tid: u32, rguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearCompartment(::core::mem::transmute_copy(&tid), ::core::mem::transmute_copy(&rguid)).into()
        }
        unsafe extern "system" fn EnumCompartments<Impl: ITfCompartmentMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumCompartments() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCompartment: GetCompartment::<Impl, IMPL_OFFSET>,
            ClearCompartment: ClearCompartment::<Impl, IMPL_OFFSET>,
            EnumCompartments: EnumCompartments::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfCompartmentMgr as ::windows::core::Interface>::IID
    }
}
pub trait ITfCompositionImpl: Sized {
    fn GetRange(&mut self) -> ::windows::core::Result<ITfRange>;
    fn ShiftStart(&mut self, ecwrite: u32, pnewstart: ::core::option::Option<ITfRange>) -> ::windows::core::Result<()>;
    fn ShiftEnd(&mut self, ecwrite: u32, pnewend: ::core::option::Option<ITfRange>) -> ::windows::core::Result<()>;
    fn EndComposition(&mut self, ecwrite: u32) -> ::windows::core::Result<()>;
}
impl ITfCompositionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfCompositionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfCompositionVtbl {
        unsafe extern "system" fn GetRange<Impl: ITfCompositionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRange() {
                ::core::result::Result::Ok(ok__) => {
                    *pprange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShiftStart<Impl: ITfCompositionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ecwrite: u32, pnewstart: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShiftStart(::core::mem::transmute_copy(&ecwrite), ::core::mem::transmute(&pnewstart)).into()
        }
        unsafe extern "system" fn ShiftEnd<Impl: ITfCompositionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ecwrite: u32, pnewend: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShiftEnd(::core::mem::transmute_copy(&ecwrite), ::core::mem::transmute(&pnewend)).into()
        }
        unsafe extern "system" fn EndComposition<Impl: ITfCompositionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ecwrite: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndComposition(::core::mem::transmute_copy(&ecwrite)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetRange: GetRange::<Impl, IMPL_OFFSET>,
            ShiftStart: ShiftStart::<Impl, IMPL_OFFSET>,
            ShiftEnd: ShiftEnd::<Impl, IMPL_OFFSET>,
            EndComposition: EndComposition::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfComposition as ::windows::core::Interface>::IID
    }
}
pub trait ITfCompositionSinkImpl: Sized {
    fn OnCompositionTerminated(&mut self, ecwrite: u32, pcomposition: ::core::option::Option<ITfComposition>) -> ::windows::core::Result<()>;
}
impl ITfCompositionSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfCompositionSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfCompositionSinkVtbl {
        unsafe extern "system" fn OnCompositionTerminated<Impl: ITfCompositionSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ecwrite: u32, pcomposition: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnCompositionTerminated(::core::mem::transmute_copy(&ecwrite), ::core::mem::transmute(&pcomposition)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnCompositionTerminated: OnCompositionTerminated::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfCompositionSink as ::windows::core::Interface>::IID
    }
}
pub trait ITfCompositionViewImpl: Sized {
    fn GetOwnerClsid(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetRange(&mut self) -> ::windows::core::Result<ITfRange>;
}
impl ITfCompositionViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfCompositionViewImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfCompositionViewVtbl {
        unsafe extern "system" fn GetOwnerClsid<Impl: ITfCompositionViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOwnerClsid() {
                ::core::result::Result::Ok(ok__) => {
                    *pclsid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRange<Impl: ITfCompositionViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRange() {
                ::core::result::Result::Ok(ok__) => {
                    *pprange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetOwnerClsid: GetOwnerClsid::<Impl, IMPL_OFFSET>,
            GetRange: GetRange::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfCompositionView as ::windows::core::Interface>::IID
    }
}
pub trait ITfConfigureSystemKeystrokeFeedImpl: Sized {
    fn DisableSystemKeystrokeFeed(&mut self) -> ::windows::core::Result<()>;
    fn EnableSystemKeystrokeFeed(&mut self) -> ::windows::core::Result<()>;
}
impl ITfConfigureSystemKeystrokeFeedVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfConfigureSystemKeystrokeFeedImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfConfigureSystemKeystrokeFeedVtbl {
        unsafe extern "system" fn DisableSystemKeystrokeFeed<Impl: ITfConfigureSystemKeystrokeFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisableSystemKeystrokeFeed().into()
        }
        unsafe extern "system" fn EnableSystemKeystrokeFeed<Impl: ITfConfigureSystemKeystrokeFeedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableSystemKeystrokeFeed().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            DisableSystemKeystrokeFeed: DisableSystemKeystrokeFeed::<Impl, IMPL_OFFSET>,
            EnableSystemKeystrokeFeed: EnableSystemKeystrokeFeed::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfConfigureSystemKeystrokeFeed as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfContextImpl: Sized {
    fn RequestEditSession(&mut self, tid: u32, pes: ::core::option::Option<ITfEditSession>, dwflags: TF_CONTEXT_EDIT_CONTEXT_FLAGS) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn InWriteSession(&mut self, tid: u32) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetSelection(&mut self, ec: u32, ulindex: u32, ulcount: u32, pselection: *mut TF_SELECTION, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn SetSelection(&mut self, ec: u32, ulcount: u32, pselection: *const TF_SELECTION) -> ::windows::core::Result<()>;
    fn GetStart(&mut self, ec: u32) -> ::windows::core::Result<ITfRange>;
    fn GetEnd(&mut self, ec: u32) -> ::windows::core::Result<ITfRange>;
    fn GetActiveView(&mut self) -> ::windows::core::Result<ITfContextView>;
    fn EnumViews(&mut self) -> ::windows::core::Result<IEnumTfContextViews>;
    fn GetStatus(&mut self) -> ::windows::core::Result<TS_STATUS>;
    fn GetProperty(&mut self, guidprop: *const ::windows::core::GUID) -> ::windows::core::Result<ITfProperty>;
    fn GetAppProperty(&mut self, guidprop: *const ::windows::core::GUID) -> ::windows::core::Result<ITfReadOnlyProperty>;
    fn TrackProperties(&mut self, prgprop: *const *const ::windows::core::GUID, cprop: u32, prgappprop: *const *const ::windows::core::GUID, cappprop: u32) -> ::windows::core::Result<ITfReadOnlyProperty>;
    fn EnumProperties(&mut self) -> ::windows::core::Result<IEnumTfProperties>;
    fn GetDocumentMgr(&mut self) -> ::windows::core::Result<ITfDocumentMgr>;
    fn CreateRangeBackup(&mut self, ec: u32, prange: ::core::option::Option<ITfRange>) -> ::windows::core::Result<ITfRangeBackup>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfContextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfContextVtbl {
        unsafe extern "system" fn RequestEditSession<Impl: ITfContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tid: u32, pes: ::windows::core::RawPtr, dwflags: TF_CONTEXT_EDIT_CONTEXT_FLAGS, phrsession: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestEditSession(::core::mem::transmute_copy(&tid), ::core::mem::transmute(&pes), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *phrsession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InWriteSession<Impl: ITfContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tid: u32, pfwritesession: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InWriteSession(::core::mem::transmute_copy(&tid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfwritesession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelection<Impl: ITfContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, ulindex: u32, ulcount: u32, pselection: *mut TF_SELECTION, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSelection(::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&ulindex), ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pselection), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn SetSelection<Impl: ITfContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, ulcount: u32, pselection: *const TF_SELECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelection(::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pselection)).into()
        }
        unsafe extern "system" fn GetStart<Impl: ITfContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, ppstart: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStart(::core::mem::transmute_copy(&ec)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppstart = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnd<Impl: ITfContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, ppend: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnd(::core::mem::transmute_copy(&ec)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppend = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActiveView<Impl: ITfContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActiveView() {
                ::core::result::Result::Ok(ok__) => {
                    *ppview = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumViews<Impl: ITfContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumViews() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: ITfContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdcs: *mut TS_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pdcs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: ITfContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidprop: *const ::windows::core::GUID, ppprop: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&guidprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAppProperty<Impl: ITfContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidprop: *const ::windows::core::GUID, ppprop: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAppProperty(::core::mem::transmute_copy(&guidprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrackProperties<Impl: ITfContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prgprop: *const *const ::windows::core::GUID, cprop: u32, prgappprop: *const *const ::windows::core::GUID, cappprop: u32, ppproperty: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrackProperties(::core::mem::transmute_copy(&prgprop), ::core::mem::transmute_copy(&cprop), ::core::mem::transmute_copy(&prgappprop), ::core::mem::transmute_copy(&cappprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppproperty = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumProperties<Impl: ITfContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocumentMgr<Impl: ITfContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocumentMgr() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdm = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRangeBackup<Impl: ITfContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, prange: ::windows::core::RawPtr, ppbackup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRangeBackup(::core::mem::transmute_copy(&ec), ::core::mem::transmute(&prange)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppbackup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            RequestEditSession: RequestEditSession::<Impl, IMPL_OFFSET>,
            InWriteSession: InWriteSession::<Impl, IMPL_OFFSET>,
            GetSelection: GetSelection::<Impl, IMPL_OFFSET>,
            SetSelection: SetSelection::<Impl, IMPL_OFFSET>,
            GetStart: GetStart::<Impl, IMPL_OFFSET>,
            GetEnd: GetEnd::<Impl, IMPL_OFFSET>,
            GetActiveView: GetActiveView::<Impl, IMPL_OFFSET>,
            EnumViews: EnumViews::<Impl, IMPL_OFFSET>,
            GetStatus: GetStatus::<Impl, IMPL_OFFSET>,
            GetProperty: GetProperty::<Impl, IMPL_OFFSET>,
            GetAppProperty: GetAppProperty::<Impl, IMPL_OFFSET>,
            TrackProperties: TrackProperties::<Impl, IMPL_OFFSET>,
            EnumProperties: EnumProperties::<Impl, IMPL_OFFSET>,
            GetDocumentMgr: GetDocumentMgr::<Impl, IMPL_OFFSET>,
            CreateRangeBackup: CreateRangeBackup::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfContext as ::windows::core::Interface>::IID
    }
}
pub trait ITfContextCompositionImpl: Sized {
    fn StartComposition(&mut self, ecwrite: u32, pcompositionrange: ::core::option::Option<ITfRange>, psink: ::core::option::Option<ITfCompositionSink>) -> ::windows::core::Result<ITfComposition>;
    fn EnumCompositions(&mut self) -> ::windows::core::Result<IEnumITfCompositionView>;
    fn FindComposition(&mut self, ecread: u32, ptestrange: ::core::option::Option<ITfRange>) -> ::windows::core::Result<IEnumITfCompositionView>;
    fn TakeOwnership(&mut self, ecwrite: u32, pcomposition: ::core::option::Option<ITfCompositionView>, psink: ::core::option::Option<ITfCompositionSink>) -> ::windows::core::Result<ITfComposition>;
}
impl ITfContextCompositionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfContextCompositionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfContextCompositionVtbl {
        unsafe extern "system" fn StartComposition<Impl: ITfContextCompositionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ecwrite: u32, pcompositionrange: ::windows::core::RawPtr, psink: ::windows::core::RawPtr, ppcomposition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartComposition(::core::mem::transmute_copy(&ecwrite), ::core::mem::transmute(&pcompositionrange), ::core::mem::transmute(&psink)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcomposition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumCompositions<Impl: ITfContextCompositionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumCompositions() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindComposition<Impl: ITfContextCompositionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ecread: u32, ptestrange: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindComposition(::core::mem::transmute_copy(&ecread), ::core::mem::transmute(&ptestrange)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TakeOwnership<Impl: ITfContextCompositionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ecwrite: u32, pcomposition: ::windows::core::RawPtr, psink: ::windows::core::RawPtr, ppcomposition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TakeOwnership(::core::mem::transmute_copy(&ecwrite), ::core::mem::transmute(&pcomposition), ::core::mem::transmute(&psink)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcomposition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            StartComposition: StartComposition::<Impl, IMPL_OFFSET>,
            EnumCompositions: EnumCompositions::<Impl, IMPL_OFFSET>,
            FindComposition: FindComposition::<Impl, IMPL_OFFSET>,
            TakeOwnership: TakeOwnership::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfContextComposition as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfContextKeyEventSinkImpl: Sized {
    fn OnKeyDown(&mut self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn OnKeyUp(&mut self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn OnTestKeyDown(&mut self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn OnTestKeyUp(&mut self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfContextKeyEventSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfContextKeyEventSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfContextKeyEventSinkVtbl {
        unsafe extern "system" fn OnKeyDown<Impl: ITfContextKeyEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnKeyDown(::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfeaten = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnKeyUp<Impl: ITfContextKeyEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnKeyUp(::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfeaten = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnTestKeyDown<Impl: ITfContextKeyEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnTestKeyDown(::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfeaten = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnTestKeyUp<Impl: ITfContextKeyEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnTestKeyUp(::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfeaten = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnKeyDown: OnKeyDown::<Impl, IMPL_OFFSET>,
            OnKeyUp: OnKeyUp::<Impl, IMPL_OFFSET>,
            OnTestKeyDown: OnTestKeyDown::<Impl, IMPL_OFFSET>,
            OnTestKeyUp: OnTestKeyUp::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfContextKeyEventSink as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITfContextOwnerImpl: Sized {
    fn GetACPFromPoint(&mut self, ptscreen: *const super::super::Foundation::POINT, dwflags: u32) -> ::windows::core::Result<i32>;
    fn GetTextExt(&mut self, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetScreenExt(&mut self) -> ::windows::core::Result<super::super::Foundation::RECT>;
    fn GetStatus(&mut self) -> ::windows::core::Result<TS_STATUS>;
    fn GetWnd(&mut self) -> ::windows::core::Result<super::super::Foundation::HWND>;
    fn GetAttribute(&mut self, rguidattribute: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITfContextOwnerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfContextOwnerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfContextOwnerVtbl {
        unsafe extern "system" fn GetACPFromPoint<Impl: ITfContextOwnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptscreen: *const super::super::Foundation::POINT, dwflags: u32, pacp: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetACPFromPoint(::core::mem::transmute_copy(&ptscreen), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pacp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTextExt<Impl: ITfContextOwnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTextExt(::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend), ::core::mem::transmute_copy(&prc), ::core::mem::transmute_copy(&pfclipped)).into()
        }
        unsafe extern "system" fn GetScreenExt<Impl: ITfContextOwnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetScreenExt() {
                ::core::result::Result::Ok(ok__) => {
                    *prc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: ITfContextOwnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdcs: *mut TS_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pdcs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWnd<Impl: ITfContextOwnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWnd() {
                ::core::result::Result::Ok(ok__) => {
                    *phwnd = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttribute<Impl: ITfContextOwnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidattribute: *const ::windows::core::GUID, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAttribute(::core::mem::transmute_copy(&rguidattribute)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetACPFromPoint: GetACPFromPoint::<Impl, IMPL_OFFSET>,
            GetTextExt: GetTextExt::<Impl, IMPL_OFFSET>,
            GetScreenExt: GetScreenExt::<Impl, IMPL_OFFSET>,
            GetStatus: GetStatus::<Impl, IMPL_OFFSET>,
            GetWnd: GetWnd::<Impl, IMPL_OFFSET>,
            GetAttribute: GetAttribute::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfContextOwner as ::windows::core::Interface>::IID
    }
}
pub trait ITfContextOwnerCompositionServicesImpl: Sized + ITfContextCompositionImpl {
    fn TerminateComposition(&mut self, pcomposition: ::core::option::Option<ITfCompositionView>) -> ::windows::core::Result<()>;
}
impl ITfContextOwnerCompositionServicesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfContextOwnerCompositionServicesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfContextOwnerCompositionServicesVtbl {
        unsafe extern "system" fn TerminateComposition<Impl: ITfContextOwnerCompositionServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcomposition: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TerminateComposition(::core::mem::transmute(&pcomposition)).into()
        }
        Self {
            base: ITfContextCompositionVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            TerminateComposition: TerminateComposition::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfContextOwnerCompositionServices as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfContextOwnerCompositionSinkImpl: Sized {
    fn OnStartComposition(&mut self, pcomposition: ::core::option::Option<ITfCompositionView>) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn OnUpdateComposition(&mut self, pcomposition: ::core::option::Option<ITfCompositionView>, prangenew: ::core::option::Option<ITfRange>) -> ::windows::core::Result<()>;
    fn OnEndComposition(&mut self, pcomposition: ::core::option::Option<ITfCompositionView>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfContextOwnerCompositionSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfContextOwnerCompositionSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfContextOwnerCompositionSinkVtbl {
        unsafe extern "system" fn OnStartComposition<Impl: ITfContextOwnerCompositionSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcomposition: ::windows::core::RawPtr, pfok: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnStartComposition(::core::mem::transmute(&pcomposition)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfok = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnUpdateComposition<Impl: ITfContextOwnerCompositionSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcomposition: ::windows::core::RawPtr, prangenew: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnUpdateComposition(::core::mem::transmute(&pcomposition), ::core::mem::transmute(&prangenew)).into()
        }
        unsafe extern "system" fn OnEndComposition<Impl: ITfContextOwnerCompositionSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcomposition: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnEndComposition(::core::mem::transmute(&pcomposition)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnStartComposition: OnStartComposition::<Impl, IMPL_OFFSET>,
            OnUpdateComposition: OnUpdateComposition::<Impl, IMPL_OFFSET>,
            OnEndComposition: OnEndComposition::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfContextOwnerCompositionSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITfContextOwnerServicesImpl: Sized {
    fn OnLayoutChange(&mut self) -> ::windows::core::Result<()>;
    fn OnStatusChange(&mut self, dwflags: u32) -> ::windows::core::Result<()>;
    fn OnAttributeChange(&mut self, rguidattribute: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn Serialize(&mut self, pprop: ::core::option::Option<ITfProperty>, prange: ::core::option::Option<ITfRange>, phdr: *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: ::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn Unserialize(&mut self, pprop: ::core::option::Option<ITfProperty>, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: ::core::option::Option<super::super::System::Com::IStream>, ploader: ::core::option::Option<ITfPersistentPropertyLoaderACP>) -> ::windows::core::Result<()>;
    fn ForceLoadProperty(&mut self, pprop: ::core::option::Option<ITfProperty>) -> ::windows::core::Result<()>;
    fn CreateRange(&mut self, acpstart: i32, acpend: i32) -> ::windows::core::Result<ITfRangeACP>;
}
#[cfg(feature = "Win32_System_Com")]
impl ITfContextOwnerServicesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfContextOwnerServicesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfContextOwnerServicesVtbl {
        unsafe extern "system" fn OnLayoutChange<Impl: ITfContextOwnerServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnLayoutChange().into()
        }
        unsafe extern "system" fn OnStatusChange<Impl: ITfContextOwnerServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnStatusChange(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn OnAttributeChange<Impl: ITfContextOwnerServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidattribute: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnAttributeChange(::core::mem::transmute_copy(&rguidattribute)).into()
        }
        unsafe extern "system" fn Serialize<Impl: ITfContextOwnerServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprop: ::windows::core::RawPtr, prange: ::windows::core::RawPtr, phdr: *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Serialize(::core::mem::transmute(&pprop), ::core::mem::transmute(&prange), ::core::mem::transmute_copy(&phdr), ::core::mem::transmute(&pstream)).into()
        }
        unsafe extern "system" fn Unserialize<Impl: ITfContextOwnerServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprop: ::windows::core::RawPtr, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: ::windows::core::RawPtr, ploader: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unserialize(::core::mem::transmute(&pprop), ::core::mem::transmute_copy(&phdr), ::core::mem::transmute(&pstream), ::core::mem::transmute(&ploader)).into()
        }
        unsafe extern "system" fn ForceLoadProperty<Impl: ITfContextOwnerServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprop: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ForceLoadProperty(::core::mem::transmute(&pprop)).into()
        }
        unsafe extern "system" fn CreateRange<Impl: ITfContextOwnerServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRange(::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnLayoutChange: OnLayoutChange::<Impl, IMPL_OFFSET>,
            OnStatusChange: OnStatusChange::<Impl, IMPL_OFFSET>,
            OnAttributeChange: OnAttributeChange::<Impl, IMPL_OFFSET>,
            Serialize: Serialize::<Impl, IMPL_OFFSET>,
            Unserialize: Unserialize::<Impl, IMPL_OFFSET>,
            ForceLoadProperty: ForceLoadProperty::<Impl, IMPL_OFFSET>,
            CreateRange: CreateRange::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfContextOwnerServices as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfContextViewImpl: Sized {
    fn GetRangeFromPoint(&mut self, ec: u32, ppt: *const super::super::Foundation::POINT, dwflags: u32) -> ::windows::core::Result<ITfRange>;
    fn GetTextExt(&mut self, ec: u32, prange: ::core::option::Option<ITfRange>, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetScreenExt(&mut self) -> ::windows::core::Result<super::super::Foundation::RECT>;
    fn GetWnd(&mut self) -> ::windows::core::Result<super::super::Foundation::HWND>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfContextViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfContextViewImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfContextViewVtbl {
        unsafe extern "system" fn GetRangeFromPoint<Impl: ITfContextViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, ppt: *const super::super::Foundation::POINT, dwflags: u32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRangeFromPoint(::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&ppt), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTextExt<Impl: ITfContextViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, prange: ::windows::core::RawPtr, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTextExt(::core::mem::transmute_copy(&ec), ::core::mem::transmute(&prange), ::core::mem::transmute_copy(&prc), ::core::mem::transmute_copy(&pfclipped)).into()
        }
        unsafe extern "system" fn GetScreenExt<Impl: ITfContextViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetScreenExt() {
                ::core::result::Result::Ok(ok__) => {
                    *prc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWnd<Impl: ITfContextViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWnd() {
                ::core::result::Result::Ok(ok__) => {
                    *phwnd = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetRangeFromPoint: GetRangeFromPoint::<Impl, IMPL_OFFSET>,
            GetTextExt: GetTextExt::<Impl, IMPL_OFFSET>,
            GetScreenExt: GetScreenExt::<Impl, IMPL_OFFSET>,
            GetWnd: GetWnd::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfContextView as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ITfCreatePropertyStoreImpl: Sized {
    fn IsStoreSerializable(&mut self, guidprop: *const ::windows::core::GUID, prange: ::core::option::Option<ITfRange>, ppropstore: ::core::option::Option<ITfPropertyStore>) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CreatePropertyStore(&mut self, guidprop: *const ::windows::core::GUID, prange: ::core::option::Option<ITfRange>, cb: u32, pstream: ::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<ITfPropertyStore>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ITfCreatePropertyStoreVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfCreatePropertyStoreImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfCreatePropertyStoreVtbl {
        unsafe extern "system" fn IsStoreSerializable<Impl: ITfCreatePropertyStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidprop: *const ::windows::core::GUID, prange: ::windows::core::RawPtr, ppropstore: ::windows::core::RawPtr, pfserializable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStoreSerializable(::core::mem::transmute_copy(&guidprop), ::core::mem::transmute(&prange), ::core::mem::transmute(&ppropstore)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfserializable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePropertyStore<Impl: ITfCreatePropertyStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidprop: *const ::windows::core::GUID, prange: ::windows::core::RawPtr, cb: u32, pstream: ::windows::core::RawPtr, ppstore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePropertyStore(::core::mem::transmute_copy(&guidprop), ::core::mem::transmute(&prange), ::core::mem::transmute_copy(&cb), ::core::mem::transmute(&pstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppstore = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            IsStoreSerializable: IsStoreSerializable::<Impl, IMPL_OFFSET>,
            CreatePropertyStore: CreatePropertyStore::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfCreatePropertyStore as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfDisplayAttributeInfoImpl: Sized {
    fn GetGUID(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetDescription(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetAttributeInfo(&mut self) -> ::windows::core::Result<TF_DISPLAYATTRIBUTE>;
    fn SetAttributeInfo(&mut self, pda: *const TF_DISPLAYATTRIBUTE) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfDisplayAttributeInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfDisplayAttributeInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfDisplayAttributeInfoVtbl {
        unsafe extern "system" fn GetGUID<Impl: ITfDisplayAttributeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGUID() {
                ::core::result::Result::Ok(ok__) => {
                    *pguid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescription<Impl: ITfDisplayAttributeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdesc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeInfo<Impl: ITfDisplayAttributeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pda: *mut TF_DISPLAYATTRIBUTE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAttributeInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *pda = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttributeInfo<Impl: ITfDisplayAttributeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pda: *const TF_DISPLAYATTRIBUTE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAttributeInfo(::core::mem::transmute_copy(&pda)).into()
        }
        unsafe extern "system" fn Reset<Impl: ITfDisplayAttributeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetGUID: GetGUID::<Impl, IMPL_OFFSET>,
            GetDescription: GetDescription::<Impl, IMPL_OFFSET>,
            GetAttributeInfo: GetAttributeInfo::<Impl, IMPL_OFFSET>,
            SetAttributeInfo: SetAttributeInfo::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfDisplayAttributeInfo as ::windows::core::Interface>::IID
    }
}
pub trait ITfDisplayAttributeMgrImpl: Sized {
    fn OnUpdateInfo(&mut self) -> ::windows::core::Result<()>;
    fn EnumDisplayAttributeInfo(&mut self) -> ::windows::core::Result<IEnumTfDisplayAttributeInfo>;
    fn GetDisplayAttributeInfo(&mut self, guid: *const ::windows::core::GUID, ppinfo: *mut ::core::option::Option<ITfDisplayAttributeInfo>, pclsidowner: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl ITfDisplayAttributeMgrVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfDisplayAttributeMgrImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfDisplayAttributeMgrVtbl {
        unsafe extern "system" fn OnUpdateInfo<Impl: ITfDisplayAttributeMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnUpdateInfo().into()
        }
        unsafe extern "system" fn EnumDisplayAttributeInfo<Impl: ITfDisplayAttributeMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumDisplayAttributeInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayAttributeInfo<Impl: ITfDisplayAttributeMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, ppinfo: *mut ::windows::core::RawPtr, pclsidowner: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDisplayAttributeInfo(::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&ppinfo), ::core::mem::transmute_copy(&pclsidowner)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnUpdateInfo: OnUpdateInfo::<Impl, IMPL_OFFSET>,
            EnumDisplayAttributeInfo: EnumDisplayAttributeInfo::<Impl, IMPL_OFFSET>,
            GetDisplayAttributeInfo: GetDisplayAttributeInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfDisplayAttributeMgr as ::windows::core::Interface>::IID
    }
}
pub trait ITfDisplayAttributeNotifySinkImpl: Sized {
    fn OnUpdateInfo(&mut self) -> ::windows::core::Result<()>;
}
impl ITfDisplayAttributeNotifySinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfDisplayAttributeNotifySinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfDisplayAttributeNotifySinkVtbl {
        unsafe extern "system" fn OnUpdateInfo<Impl: ITfDisplayAttributeNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnUpdateInfo().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnUpdateInfo: OnUpdateInfo::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfDisplayAttributeNotifySink as ::windows::core::Interface>::IID
    }
}
pub trait ITfDisplayAttributeProviderImpl: Sized {
    fn EnumDisplayAttributeInfo(&mut self) -> ::windows::core::Result<IEnumTfDisplayAttributeInfo>;
    fn GetDisplayAttributeInfo(&mut self, guid: *const ::windows::core::GUID) -> ::windows::core::Result<ITfDisplayAttributeInfo>;
}
impl ITfDisplayAttributeProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfDisplayAttributeProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfDisplayAttributeProviderVtbl {
        unsafe extern "system" fn EnumDisplayAttributeInfo<Impl: ITfDisplayAttributeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumDisplayAttributeInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayAttributeInfo<Impl: ITfDisplayAttributeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, ppinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDisplayAttributeInfo(::core::mem::transmute_copy(&guid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            EnumDisplayAttributeInfo: EnumDisplayAttributeInfo::<Impl, IMPL_OFFSET>,
            GetDisplayAttributeInfo: GetDisplayAttributeInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfDisplayAttributeProvider as ::windows::core::Interface>::IID
    }
}
pub trait ITfDocumentMgrImpl: Sized {
    fn CreateContext(&mut self, tidowner: u32, dwflags: u32, punk: ::core::option::Option<::windows::core::IUnknown>, ppic: *mut ::core::option::Option<ITfContext>, pectextstore: *mut u32) -> ::windows::core::Result<()>;
    fn Push(&mut self, pic: ::core::option::Option<ITfContext>) -> ::windows::core::Result<()>;
    fn Pop(&mut self, dwflags: u32) -> ::windows::core::Result<()>;
    fn GetTop(&mut self) -> ::windows::core::Result<ITfContext>;
    fn GetBase(&mut self) -> ::windows::core::Result<ITfContext>;
    fn EnumContexts(&mut self) -> ::windows::core::Result<IEnumTfContexts>;
}
impl ITfDocumentMgrVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfDocumentMgrImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfDocumentMgrVtbl {
        unsafe extern "system" fn CreateContext<Impl: ITfDocumentMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tidowner: u32, dwflags: u32, punk: *mut ::core::ffi::c_void, ppic: *mut ::windows::core::RawPtr, pectextstore: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateContext(::core::mem::transmute_copy(&tidowner), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&punk), ::core::mem::transmute_copy(&ppic), ::core::mem::transmute_copy(&pectextstore)).into()
        }
        unsafe extern "system" fn Push<Impl: ITfDocumentMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Push(::core::mem::transmute(&pic)).into()
        }
        unsafe extern "system" fn Pop<Impl: ITfDocumentMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Pop(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetTop<Impl: ITfDocumentMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppic: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTop() {
                ::core::result::Result::Ok(ok__) => {
                    *ppic = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBase<Impl: ITfDocumentMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppic: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBase() {
                ::core::result::Result::Ok(ok__) => {
                    *ppic = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumContexts<Impl: ITfDocumentMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumContexts() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateContext: CreateContext::<Impl, IMPL_OFFSET>,
            Push: Push::<Impl, IMPL_OFFSET>,
            Pop: Pop::<Impl, IMPL_OFFSET>,
            GetTop: GetTop::<Impl, IMPL_OFFSET>,
            GetBase: GetBase::<Impl, IMPL_OFFSET>,
            EnumContexts: EnumContexts::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfDocumentMgr as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfEditRecordImpl: Sized {
    fn GetSelectionStatus(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetTextAndPropertyUpdates(&mut self, dwflags: GET_TEXT_AND_PROPERTY_UPDATES_FLAGS, prgproperties: *const *const ::windows::core::GUID, cproperties: u32) -> ::windows::core::Result<IEnumTfRanges>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfEditRecordVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfEditRecordImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfEditRecordVtbl {
        unsafe extern "system" fn GetSelectionStatus<Impl: ITfEditRecordImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfchanged: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSelectionStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pfchanged = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTextAndPropertyUpdates<Impl: ITfEditRecordImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: GET_TEXT_AND_PROPERTY_UPDATES_FLAGS, prgproperties: *const *const ::windows::core::GUID, cproperties: u32, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTextAndPropertyUpdates(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&prgproperties), ::core::mem::transmute_copy(&cproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetSelectionStatus: GetSelectionStatus::<Impl, IMPL_OFFSET>,
            GetTextAndPropertyUpdates: GetTextAndPropertyUpdates::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfEditRecord as ::windows::core::Interface>::IID
    }
}
pub trait ITfEditSessionImpl: Sized {
    fn DoEditSession(&mut self, ec: u32) -> ::windows::core::Result<()>;
}
impl ITfEditSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfEditSessionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfEditSessionVtbl {
        unsafe extern "system" fn DoEditSession<Impl: ITfEditSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DoEditSession(::core::mem::transmute_copy(&ec)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), DoEditSession: DoEditSession::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfEditSession as ::windows::core::Interface>::IID
    }
}
pub trait ITfEditTransactionSinkImpl: Sized {
    fn OnStartEditTransaction(&mut self, pic: ::core::option::Option<ITfContext>) -> ::windows::core::Result<()>;
    fn OnEndEditTransaction(&mut self, pic: ::core::option::Option<ITfContext>) -> ::windows::core::Result<()>;
}
impl ITfEditTransactionSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfEditTransactionSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfEditTransactionSinkVtbl {
        unsafe extern "system" fn OnStartEditTransaction<Impl: ITfEditTransactionSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnStartEditTransaction(::core::mem::transmute(&pic)).into()
        }
        unsafe extern "system" fn OnEndEditTransaction<Impl: ITfEditTransactionSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnEndEditTransaction(::core::mem::transmute(&pic)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnStartEditTransaction: OnStartEditTransaction::<Impl, IMPL_OFFSET>,
            OnEndEditTransaction: OnEndEditTransaction::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfEditTransactionSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfFnAdviseTextImpl: Sized + ITfFunctionImpl {
    fn OnTextUpdate(&mut self, prange: ::core::option::Option<ITfRange>, pchtext: super::super::Foundation::PWSTR, cch: i32) -> ::windows::core::Result<()>;
    fn OnLatticeUpdate(&mut self, prange: ::core::option::Option<ITfRange>, plattice: ::core::option::Option<ITfLMLattice>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfFnAdviseTextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFnAdviseTextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfFnAdviseTextVtbl {
        unsafe extern "system" fn OnTextUpdate<Impl: ITfFnAdviseTextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, pchtext: super::super::Foundation::PWSTR, cch: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnTextUpdate(::core::mem::transmute(&prange), ::core::mem::transmute_copy(&pchtext), ::core::mem::transmute_copy(&cch)).into()
        }
        unsafe extern "system" fn OnLatticeUpdate<Impl: ITfFnAdviseTextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, plattice: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnLatticeUpdate(::core::mem::transmute(&prange), ::core::mem::transmute(&plattice)).into()
        }
        Self {
            base: ITfFunctionVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            OnTextUpdate: OnTextUpdate::<Impl, IMPL_OFFSET>,
            OnLatticeUpdate: OnLatticeUpdate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfFnAdviseText as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfFnBalloonImpl: Sized {
    fn UpdateBalloon(&mut self, style: TfLBBalloonStyle, pch: super::super::Foundation::PWSTR, cch: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfFnBalloonVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFnBalloonImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfFnBalloonVtbl {
        unsafe extern "system" fn UpdateBalloon<Impl: ITfFnBalloonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, style: TfLBBalloonStyle, pch: super::super::Foundation::PWSTR, cch: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateBalloon(::core::mem::transmute_copy(&style), ::core::mem::transmute_copy(&pch), ::core::mem::transmute_copy(&cch)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), UpdateBalloon: UpdateBalloon::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfFnBalloon as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfFnConfigureImpl: Sized + ITfFunctionImpl {
    fn Show(&mut self, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfFnConfigureVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFnConfigureImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfFnConfigureVtbl {
        unsafe extern "system" fn Show<Impl: ITfFnConfigureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Show(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&rguidprofile)).into()
        }
        Self { base: ITfFunctionVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Show: Show::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfFnConfigure as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfFnConfigureRegisterEudcImpl: Sized + ITfFunctionImpl {
    fn Show(&mut self, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const ::windows::core::GUID, bstrregistered: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfFnConfigureRegisterEudcVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFnConfigureRegisterEudcImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfFnConfigureRegisterEudcVtbl {
        unsafe extern "system" fn Show<Impl: ITfFnConfigureRegisterEudcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const ::windows::core::GUID, bstrregistered: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Show(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&rguidprofile), ::core::mem::transmute_copy(&bstrregistered)).into()
        }
        Self { base: ITfFunctionVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Show: Show::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfFnConfigureRegisterEudc as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfFnConfigureRegisterWordImpl: Sized + ITfFunctionImpl {
    fn Show(&mut self, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const ::windows::core::GUID, bstrregistered: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfFnConfigureRegisterWordVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFnConfigureRegisterWordImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfFnConfigureRegisterWordVtbl {
        unsafe extern "system" fn Show<Impl: ITfFnConfigureRegisterWordImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const ::windows::core::GUID, bstrregistered: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Show(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&rguidprofile), ::core::mem::transmute_copy(&bstrregistered)).into()
        }
        Self { base: ITfFunctionVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Show: Show::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfFnConfigureRegisterWord as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfFnCustomSpeechCommandImpl: Sized + ITfFunctionImpl {
    fn SetSpeechCommandProvider(&mut self, pspcmdprovider: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfFnCustomSpeechCommandVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFnCustomSpeechCommandImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfFnCustomSpeechCommandVtbl {
        unsafe extern "system" fn SetSpeechCommandProvider<Impl: ITfFnCustomSpeechCommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pspcmdprovider: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSpeechCommandProvider(::core::mem::transmute(&pspcmdprovider)).into()
        }
        Self {
            base: ITfFunctionVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetSpeechCommandProvider: SetSpeechCommandProvider::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfFnCustomSpeechCommand as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfFnGetLinguisticAlternatesImpl: Sized + ITfFunctionImpl {
    fn GetAlternates(&mut self, prange: ::core::option::Option<ITfRange>) -> ::windows::core::Result<ITfCandidateList>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfFnGetLinguisticAlternatesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFnGetLinguisticAlternatesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfFnGetLinguisticAlternatesVtbl {
        unsafe extern "system" fn GetAlternates<Impl: ITfFnGetLinguisticAlternatesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, ppcandidatelist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAlternates(::core::mem::transmute(&prange)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcandidatelist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ITfFunctionVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetAlternates: GetAlternates::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfFnGetLinguisticAlternates as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfFnGetPreferredTouchKeyboardLayoutImpl: Sized + ITfFunctionImpl {
    fn GetLayout(&mut self, ptkblayouttype: *mut TKBLayoutType, pwpreferredlayoutid: *const u16) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfFnGetPreferredTouchKeyboardLayoutVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFnGetPreferredTouchKeyboardLayoutImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfFnGetPreferredTouchKeyboardLayoutVtbl {
        unsafe extern "system" fn GetLayout<Impl: ITfFnGetPreferredTouchKeyboardLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptkblayouttype: *mut TKBLayoutType, pwpreferredlayoutid: *const u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLayout(::core::mem::transmute_copy(&ptkblayouttype), ::core::mem::transmute_copy(&pwpreferredlayoutid)).into()
        }
        Self { base: ITfFunctionVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetLayout: GetLayout::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfFnGetPreferredTouchKeyboardLayout as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfFnGetSAPIObjectImpl: Sized + ITfFunctionImpl {
    fn Get(&mut self, sobj: TfSapiObject) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfFnGetSAPIObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFnGetSAPIObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfFnGetSAPIObjectVtbl {
        unsafe extern "system" fn Get<Impl: ITfFnGetSAPIObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sobj: TfSapiObject, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Get(::core::mem::transmute_copy(&sobj)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ITfFunctionVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Get: Get::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfFnGetSAPIObject as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfFnLMInternalImpl: Sized + ITfFunctionImpl + ITfFnLMProcessorImpl {
    fn ProcessLattice(&mut self, prange: ::core::option::Option<ITfRange>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfFnLMInternalVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFnLMInternalImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfFnLMInternalVtbl {
        unsafe extern "system" fn ProcessLattice<Impl: ITfFnLMInternalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessLattice(::core::mem::transmute(&prange)).into()
        }
        Self { base: ITfFnLMProcessorVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), ProcessLattice: ProcessLattice::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfFnLMInternal as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfFnLMProcessorImpl: Sized + ITfFunctionImpl {
    fn QueryRange(&mut self, prange: ::core::option::Option<ITfRange>, ppnewrange: *mut ::core::option::Option<ITfRange>, pfaccepted: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn QueryLangID(&mut self, langid: u16) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetReconversion(&mut self, prange: ::core::option::Option<ITfRange>) -> ::windows::core::Result<ITfCandidateList>;
    fn Reconvert(&mut self, prange: ::core::option::Option<ITfRange>) -> ::windows::core::Result<()>;
    fn QueryKey(&mut self, fup: super::super::Foundation::BOOL, vkey: super::super::Foundation::WPARAM, lparamkeydata: super::super::Foundation::LPARAM) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn InvokeKey(&mut self, fup: super::super::Foundation::BOOL, vkey: super::super::Foundation::WPARAM, lparamkeydata: super::super::Foundation::LPARAM) -> ::windows::core::Result<()>;
    fn InvokeFunc(&mut self, pic: ::core::option::Option<ITfContext>, refguidfunc: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfFnLMProcessorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFnLMProcessorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfFnLMProcessorVtbl {
        unsafe extern "system" fn QueryRange<Impl: ITfFnLMProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, ppnewrange: *mut ::windows::core::RawPtr, pfaccepted: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryRange(::core::mem::transmute(&prange), ::core::mem::transmute_copy(&ppnewrange), ::core::mem::transmute_copy(&pfaccepted)).into()
        }
        unsafe extern "system" fn QueryLangID<Impl: ITfFnLMProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16, pfaccepted: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryLangID(::core::mem::transmute_copy(&langid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfaccepted = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReconversion<Impl: ITfFnLMProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, ppcandlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetReconversion(::core::mem::transmute(&prange)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcandlist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reconvert<Impl: ITfFnLMProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reconvert(::core::mem::transmute(&prange)).into()
        }
        unsafe extern "system" fn QueryKey<Impl: ITfFnLMProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fup: super::super::Foundation::BOOL, vkey: super::super::Foundation::WPARAM, lparamkeydata: super::super::Foundation::LPARAM, pfinterested: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryKey(::core::mem::transmute_copy(&fup), ::core::mem::transmute_copy(&vkey), ::core::mem::transmute_copy(&lparamkeydata)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfinterested = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvokeKey<Impl: ITfFnLMProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fup: super::super::Foundation::BOOL, vkey: super::super::Foundation::WPARAM, lparamkeydata: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InvokeKey(::core::mem::transmute_copy(&fup), ::core::mem::transmute_copy(&vkey), ::core::mem::transmute_copy(&lparamkeydata)).into()
        }
        unsafe extern "system" fn InvokeFunc<Impl: ITfFnLMProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr, refguidfunc: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InvokeFunc(::core::mem::transmute(&pic), ::core::mem::transmute_copy(&refguidfunc)).into()
        }
        Self {
            base: ITfFunctionVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            QueryRange: QueryRange::<Impl, IMPL_OFFSET>,
            QueryLangID: QueryLangID::<Impl, IMPL_OFFSET>,
            GetReconversion: GetReconversion::<Impl, IMPL_OFFSET>,
            Reconvert: Reconvert::<Impl, IMPL_OFFSET>,
            QueryKey: QueryKey::<Impl, IMPL_OFFSET>,
            InvokeKey: InvokeKey::<Impl, IMPL_OFFSET>,
            InvokeFunc: InvokeFunc::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfFnLMProcessor as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfFnLangProfileUtilImpl: Sized + ITfFunctionImpl {
    fn RegisterActiveProfiles(&mut self) -> ::windows::core::Result<()>;
    fn IsProfileAvailableForLang(&mut self, langid: u16) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfFnLangProfileUtilVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFnLangProfileUtilImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfFnLangProfileUtilVtbl {
        unsafe extern "system" fn RegisterActiveProfiles<Impl: ITfFnLangProfileUtilImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterActiveProfiles().into()
        }
        unsafe extern "system" fn IsProfileAvailableForLang<Impl: ITfFnLangProfileUtilImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16, pfavailable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsProfileAvailableForLang(::core::mem::transmute_copy(&langid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfavailable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ITfFunctionVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            RegisterActiveProfiles: RegisterActiveProfiles::<Impl, IMPL_OFFSET>,
            IsProfileAvailableForLang: IsProfileAvailableForLang::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfFnLangProfileUtil as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfFnPlayBackImpl: Sized + ITfFunctionImpl {
    fn QueryRange(&mut self, prange: ::core::option::Option<ITfRange>, ppnewrange: *mut ::core::option::Option<ITfRange>, pfplayable: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn Play(&mut self, prange: ::core::option::Option<ITfRange>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfFnPlayBackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFnPlayBackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfFnPlayBackVtbl {
        unsafe extern "system" fn QueryRange<Impl: ITfFnPlayBackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, ppnewrange: *mut ::windows::core::RawPtr, pfplayable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryRange(::core::mem::transmute(&prange), ::core::mem::transmute_copy(&ppnewrange), ::core::mem::transmute_copy(&pfplayable)).into()
        }
        unsafe extern "system" fn Play<Impl: ITfFnPlayBackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Play(::core::mem::transmute(&prange)).into()
        }
        Self {
            base: ITfFunctionVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            QueryRange: QueryRange::<Impl, IMPL_OFFSET>,
            Play: Play::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfFnPlayBack as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfFnPropertyUIStatusImpl: Sized + ITfFunctionImpl {
    fn GetStatus(&mut self, refguidprop: *const ::windows::core::GUID) -> ::windows::core::Result<u32>;
    fn SetStatus(&mut self, refguidprop: *const ::windows::core::GUID, dw: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfFnPropertyUIStatusVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFnPropertyUIStatusImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfFnPropertyUIStatusVtbl {
        unsafe extern "system" fn GetStatus<Impl: ITfFnPropertyUIStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, refguidprop: *const ::windows::core::GUID, pdw: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus(::core::mem::transmute_copy(&refguidprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdw = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStatus<Impl: ITfFnPropertyUIStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, refguidprop: *const ::windows::core::GUID, dw: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStatus(::core::mem::transmute_copy(&refguidprop), ::core::mem::transmute_copy(&dw)).into()
        }
        Self {
            base: ITfFunctionVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetStatus: GetStatus::<Impl, IMPL_OFFSET>,
            SetStatus: SetStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfFnPropertyUIStatus as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfFnReconversionImpl: Sized + ITfFunctionImpl {
    fn QueryRange(&mut self, prange: ::core::option::Option<ITfRange>, ppnewrange: *mut ::core::option::Option<ITfRange>, pfconvertable: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetReconversion(&mut self, prange: ::core::option::Option<ITfRange>) -> ::windows::core::Result<ITfCandidateList>;
    fn Reconvert(&mut self, prange: ::core::option::Option<ITfRange>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfFnReconversionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFnReconversionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfFnReconversionVtbl {
        unsafe extern "system" fn QueryRange<Impl: ITfFnReconversionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, ppnewrange: *mut ::windows::core::RawPtr, pfconvertable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryRange(::core::mem::transmute(&prange), ::core::mem::transmute_copy(&ppnewrange), ::core::mem::transmute_copy(&pfconvertable)).into()
        }
        unsafe extern "system" fn GetReconversion<Impl: ITfFnReconversionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, ppcandlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetReconversion(::core::mem::transmute(&prange)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcandlist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reconvert<Impl: ITfFnReconversionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reconvert(::core::mem::transmute(&prange)).into()
        }
        Self {
            base: ITfFunctionVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            QueryRange: QueryRange::<Impl, IMPL_OFFSET>,
            GetReconversion: GetReconversion::<Impl, IMPL_OFFSET>,
            Reconvert: Reconvert::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfFnReconversion as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfFnSearchCandidateProviderImpl: Sized + ITfFunctionImpl {
    fn GetSearchCandidates(&mut self, bstrquery: super::super::Foundation::BSTR, bstrapplicationid: super::super::Foundation::BSTR) -> ::windows::core::Result<ITfCandidateList>;
    fn SetResult(&mut self, bstrquery: super::super::Foundation::BSTR, bstrapplicationid: super::super::Foundation::BSTR, bstrresult: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfFnSearchCandidateProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFnSearchCandidateProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfFnSearchCandidateProviderVtbl {
        unsafe extern "system" fn GetSearchCandidates<Impl: ITfFnSearchCandidateProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrapplicationid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pplist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSearchCandidates(::core::mem::transmute_copy(&bstrquery), ::core::mem::transmute_copy(&bstrapplicationid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pplist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetResult<Impl: ITfFnSearchCandidateProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrapplicationid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrresult: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetResult(::core::mem::transmute_copy(&bstrquery), ::core::mem::transmute_copy(&bstrapplicationid), ::core::mem::transmute_copy(&bstrresult)).into()
        }
        Self {
            base: ITfFunctionVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetSearchCandidates: GetSearchCandidates::<Impl, IMPL_OFFSET>,
            SetResult: SetResult::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfFnSearchCandidateProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfFnShowHelpImpl: Sized + ITfFunctionImpl {
    fn Show(&mut self, hwndparent: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfFnShowHelpVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFnShowHelpImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfFnShowHelpVtbl {
        unsafe extern "system" fn Show<Impl: ITfFnShowHelpImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Show(::core::mem::transmute_copy(&hwndparent)).into()
        }
        Self { base: ITfFunctionVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Show: Show::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfFnShowHelp as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfFunctionImpl: Sized {
    fn GetDisplayName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfFunctionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFunctionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfFunctionVtbl {
        unsafe extern "system" fn GetDisplayName<Impl: ITfFunctionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetDisplayName: GetDisplayName::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfFunction as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfFunctionProviderImpl: Sized {
    fn GetType(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetDescription(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetFunction(&mut self, rguid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfFunctionProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFunctionProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfFunctionProviderVtbl {
        unsafe extern "system" fn GetType<Impl: ITfFunctionProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetType() {
                ::core::result::Result::Ok(ok__) => {
                    *pguid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescription<Impl: ITfFunctionProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdesc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunction<Impl: ITfFunctionProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFunction(::core::mem::transmute_copy(&rguid), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetType: GetType::<Impl, IMPL_OFFSET>,
            GetDescription: GetDescription::<Impl, IMPL_OFFSET>,
            GetFunction: GetFunction::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfFunctionProvider as ::windows::core::Interface>::IID
    }
}
pub trait ITfInputProcessorProfileActivationSinkImpl: Sized {
    fn OnActivated(&mut self, dwprofiletype: u32, langid: u16, clsid: *const ::windows::core::GUID, catid: *const ::windows::core::GUID, guidprofile: *const ::windows::core::GUID, hkl: HKL, dwflags: u32) -> ::windows::core::Result<()>;
}
impl ITfInputProcessorProfileActivationSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfInputProcessorProfileActivationSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfInputProcessorProfileActivationSinkVtbl {
        unsafe extern "system" fn OnActivated<Impl: ITfInputProcessorProfileActivationSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwprofiletype: u32, langid: u16, clsid: *const ::windows::core::GUID, catid: *const ::windows::core::GUID, guidprofile: *const ::windows::core::GUID, hkl: HKL, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnActivated(::core::mem::transmute_copy(&dwprofiletype), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&clsid), ::core::mem::transmute_copy(&catid), ::core::mem::transmute_copy(&guidprofile), ::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&dwflags)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnActivated: OnActivated::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfInputProcessorProfileActivationSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfInputProcessorProfileMgrImpl: Sized {
    fn ActivateProfile(&mut self, dwprofiletype: u32, langid: u16, clsid: *const ::windows::core::GUID, guidprofile: *const ::windows::core::GUID, hkl: HKL, dwflags: u32) -> ::windows::core::Result<()>;
    fn DeactivateProfile(&mut self, dwprofiletype: u32, langid: u16, clsid: *const ::windows::core::GUID, guidprofile: *const ::windows::core::GUID, hkl: HKL, dwflags: u32) -> ::windows::core::Result<()>;
    fn GetProfile(&mut self, dwprofiletype: u32, langid: u16, clsid: *const ::windows::core::GUID, guidprofile: *const ::windows::core::GUID, hkl: HKL) -> ::windows::core::Result<TF_INPUTPROCESSORPROFILE>;
    fn EnumProfiles(&mut self, langid: u16) -> ::windows::core::Result<IEnumTfInputProcessorProfiles>;
    fn ReleaseInputProcessor(&mut self, rclsid: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::Result<()>;
    fn RegisterProfile(&mut self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, pchdesc: super::super::Foundation::PWSTR, cchdesc: u32, pchiconfile: super::super::Foundation::PWSTR, cchfile: u32, uiconindex: u32, hklsubstitute: HKL, dwpreferredlayout: u32, benabledbydefault: super::super::Foundation::BOOL, dwflags: u32) -> ::windows::core::Result<()>;
    fn UnregisterProfile(&mut self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::Result<()>;
    fn GetActiveProfile(&mut self, catid: *const ::windows::core::GUID) -> ::windows::core::Result<TF_INPUTPROCESSORPROFILE>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfInputProcessorProfileMgrVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfInputProcessorProfileMgrImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfInputProcessorProfileMgrVtbl {
        unsafe extern "system" fn ActivateProfile<Impl: ITfInputProcessorProfileMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwprofiletype: u32, langid: u16, clsid: *const ::windows::core::GUID, guidprofile: *const ::windows::core::GUID, hkl: HKL, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ActivateProfile(::core::mem::transmute_copy(&dwprofiletype), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&clsid), ::core::mem::transmute_copy(&guidprofile), ::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn DeactivateProfile<Impl: ITfInputProcessorProfileMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwprofiletype: u32, langid: u16, clsid: *const ::windows::core::GUID, guidprofile: *const ::windows::core::GUID, hkl: HKL, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeactivateProfile(::core::mem::transmute_copy(&dwprofiletype), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&clsid), ::core::mem::transmute_copy(&guidprofile), ::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetProfile<Impl: ITfInputProcessorProfileMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwprofiletype: u32, langid: u16, clsid: *const ::windows::core::GUID, guidprofile: *const ::windows::core::GUID, hkl: HKL, pprofile: *mut TF_INPUTPROCESSORPROFILE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProfile(::core::mem::transmute_copy(&dwprofiletype), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&clsid), ::core::mem::transmute_copy(&guidprofile), ::core::mem::transmute_copy(&hkl)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprofile = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumProfiles<Impl: ITfInputProcessorProfileMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumProfiles(::core::mem::transmute_copy(&langid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseInputProcessor<Impl: ITfInputProcessorProfileMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseInputProcessor(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn RegisterProfile<Impl: ITfInputProcessorProfileMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, pchdesc: super::super::Foundation::PWSTR, cchdesc: u32, pchiconfile: super::super::Foundation::PWSTR, cchfile: u32, uiconindex: u32, hklsubstitute: HKL, dwpreferredlayout: u32, benabledbydefault: super::super::Foundation::BOOL, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .RegisterProfile(
                    ::core::mem::transmute_copy(&rclsid),
                    ::core::mem::transmute_copy(&langid),
                    ::core::mem::transmute_copy(&guidprofile),
                    ::core::mem::transmute_copy(&pchdesc),
                    ::core::mem::transmute_copy(&cchdesc),
                    ::core::mem::transmute_copy(&pchiconfile),
                    ::core::mem::transmute_copy(&cchfile),
                    ::core::mem::transmute_copy(&uiconindex),
                    ::core::mem::transmute_copy(&hklsubstitute),
                    ::core::mem::transmute_copy(&dwpreferredlayout),
                    ::core::mem::transmute_copy(&benabledbydefault),
                    ::core::mem::transmute_copy(&dwflags),
                )
                .into()
        }
        unsafe extern "system" fn UnregisterProfile<Impl: ITfInputProcessorProfileMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterProfile(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&guidprofile), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetActiveProfile<Impl: ITfInputProcessorProfileMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, catid: *const ::windows::core::GUID, pprofile: *mut TF_INPUTPROCESSORPROFILE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActiveProfile(::core::mem::transmute_copy(&catid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprofile = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ActivateProfile: ActivateProfile::<Impl, IMPL_OFFSET>,
            DeactivateProfile: DeactivateProfile::<Impl, IMPL_OFFSET>,
            GetProfile: GetProfile::<Impl, IMPL_OFFSET>,
            EnumProfiles: EnumProfiles::<Impl, IMPL_OFFSET>,
            ReleaseInputProcessor: ReleaseInputProcessor::<Impl, IMPL_OFFSET>,
            RegisterProfile: RegisterProfile::<Impl, IMPL_OFFSET>,
            UnregisterProfile: UnregisterProfile::<Impl, IMPL_OFFSET>,
            GetActiveProfile: GetActiveProfile::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfInputProcessorProfileMgr as ::windows::core::Interface>::IID
    }
}
pub trait ITfInputProcessorProfileSubstituteLayoutImpl: Sized {
    fn GetSubstituteKeyboardLayout(&mut self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID) -> ::windows::core::Result<HKL>;
}
impl ITfInputProcessorProfileSubstituteLayoutVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfInputProcessorProfileSubstituteLayoutImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfInputProcessorProfileSubstituteLayoutVtbl {
        unsafe extern "system" fn GetSubstituteKeyboardLayout<Impl: ITfInputProcessorProfileSubstituteLayoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, phkl: *mut HKL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSubstituteKeyboardLayout(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&guidprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    *phkl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetSubstituteKeyboardLayout: GetSubstituteKeyboardLayout::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfInputProcessorProfileSubstituteLayout as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ITfInputProcessorProfilesImpl: Sized {
    fn Register(&mut self, rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn Unregister(&mut self, rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn AddLanguageProfile(&mut self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, pchdesc: super::super::Foundation::PWSTR, cchdesc: u32, pchiconfile: super::super::Foundation::PWSTR, cchfile: u32, uiconindex: u32) -> ::windows::core::Result<()>;
    fn RemoveLanguageProfile(&mut self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn EnumInputProcessorInfo(&mut self) -> ::windows::core::Result<super::super::System::Com::IEnumGUID>;
    fn GetDefaultLanguageProfile(&mut self, langid: u16, catid: *const ::windows::core::GUID, pclsid: *mut ::windows::core::GUID, pguidprofile: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn SetDefaultLanguageProfile(&mut self, langid: u16, rclsid: *const ::windows::core::GUID, guidprofiles: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn ActivateLanguageProfile(&mut self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofiles: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetActiveLanguageProfile(&mut self, rclsid: *const ::windows::core::GUID, plangid: *mut u16, pguidprofile: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetLanguageProfileDescription(&mut self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetCurrentLanguage(&mut self) -> ::windows::core::Result<u16>;
    fn ChangeCurrentLanguage(&mut self, langid: u16) -> ::windows::core::Result<()>;
    fn GetLanguageList(&mut self, pplangid: *mut *mut u16, pulcount: *mut u32) -> ::windows::core::Result<()>;
    fn EnumLanguageProfiles(&mut self, langid: u16) -> ::windows::core::Result<IEnumTfLanguageProfiles>;
    fn EnableLanguageProfile(&mut self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, fenable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn IsEnabledLanguageProfile(&mut self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn EnableLanguageProfileByDefault(&mut self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, fenable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SubstituteKeyboardLayout(&mut self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, hkl: HKL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ITfInputProcessorProfilesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfInputProcessorProfilesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfInputProcessorProfilesVtbl {
        unsafe extern "system" fn Register<Impl: ITfInputProcessorProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Register(::core::mem::transmute_copy(&rclsid)).into()
        }
        unsafe extern "system" fn Unregister<Impl: ITfInputProcessorProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unregister(::core::mem::transmute_copy(&rclsid)).into()
        }
        unsafe extern "system" fn AddLanguageProfile<Impl: ITfInputProcessorProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, pchdesc: super::super::Foundation::PWSTR, cchdesc: u32, pchiconfile: super::super::Foundation::PWSTR, cchfile: u32, uiconindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddLanguageProfile(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&guidprofile), ::core::mem::transmute_copy(&pchdesc), ::core::mem::transmute_copy(&cchdesc), ::core::mem::transmute_copy(&pchiconfile), ::core::mem::transmute_copy(&cchfile), ::core::mem::transmute_copy(&uiconindex)).into()
        }
        unsafe extern "system" fn RemoveLanguageProfile<Impl: ITfInputProcessorProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveLanguageProfile(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&guidprofile)).into()
        }
        unsafe extern "system" fn EnumInputProcessorInfo<Impl: ITfInputProcessorProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumInputProcessorInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultLanguageProfile<Impl: ITfInputProcessorProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16, catid: *const ::windows::core::GUID, pclsid: *mut ::windows::core::GUID, pguidprofile: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDefaultLanguageProfile(::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&catid), ::core::mem::transmute_copy(&pclsid), ::core::mem::transmute_copy(&pguidprofile)).into()
        }
        unsafe extern "system" fn SetDefaultLanguageProfile<Impl: ITfInputProcessorProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16, rclsid: *const ::windows::core::GUID, guidprofiles: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultLanguageProfile(::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&guidprofiles)).into()
        }
        unsafe extern "system" fn ActivateLanguageProfile<Impl: ITfInputProcessorProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofiles: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ActivateLanguageProfile(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&guidprofiles)).into()
        }
        unsafe extern "system" fn GetActiveLanguageProfile<Impl: ITfInputProcessorProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, plangid: *mut u16, pguidprofile: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetActiveLanguageProfile(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&plangid), ::core::mem::transmute_copy(&pguidprofile)).into()
        }
        unsafe extern "system" fn GetLanguageProfileDescription<Impl: ITfInputProcessorProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, pbstrprofile: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLanguageProfileDescription(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&guidprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrprofile = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentLanguage<Impl: ITfInputProcessorProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plangid: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentLanguage() {
                ::core::result::Result::Ok(ok__) => {
                    *plangid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChangeCurrentLanguage<Impl: ITfInputProcessorProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ChangeCurrentLanguage(::core::mem::transmute_copy(&langid)).into()
        }
        unsafe extern "system" fn GetLanguageList<Impl: ITfInputProcessorProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplangid: *mut *mut u16, pulcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLanguageList(::core::mem::transmute_copy(&pplangid), ::core::mem::transmute_copy(&pulcount)).into()
        }
        unsafe extern "system" fn EnumLanguageProfiles<Impl: ITfInputProcessorProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumLanguageProfiles(::core::mem::transmute_copy(&langid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableLanguageProfile<Impl: ITfInputProcessorProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableLanguageProfile(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&guidprofile), ::core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn IsEnabledLanguageProfile<Impl: ITfInputProcessorProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, pfenable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEnabledLanguageProfile(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&guidprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfenable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableLanguageProfileByDefault<Impl: ITfInputProcessorProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableLanguageProfileByDefault(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&guidprofile), ::core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn SubstituteKeyboardLayout<Impl: ITfInputProcessorProfilesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, hkl: HKL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SubstituteKeyboardLayout(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&guidprofile), ::core::mem::transmute_copy(&hkl)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Register: Register::<Impl, IMPL_OFFSET>,
            Unregister: Unregister::<Impl, IMPL_OFFSET>,
            AddLanguageProfile: AddLanguageProfile::<Impl, IMPL_OFFSET>,
            RemoveLanguageProfile: RemoveLanguageProfile::<Impl, IMPL_OFFSET>,
            EnumInputProcessorInfo: EnumInputProcessorInfo::<Impl, IMPL_OFFSET>,
            GetDefaultLanguageProfile: GetDefaultLanguageProfile::<Impl, IMPL_OFFSET>,
            SetDefaultLanguageProfile: SetDefaultLanguageProfile::<Impl, IMPL_OFFSET>,
            ActivateLanguageProfile: ActivateLanguageProfile::<Impl, IMPL_OFFSET>,
            GetActiveLanguageProfile: GetActiveLanguageProfile::<Impl, IMPL_OFFSET>,
            GetLanguageProfileDescription: GetLanguageProfileDescription::<Impl, IMPL_OFFSET>,
            GetCurrentLanguage: GetCurrentLanguage::<Impl, IMPL_OFFSET>,
            ChangeCurrentLanguage: ChangeCurrentLanguage::<Impl, IMPL_OFFSET>,
            GetLanguageList: GetLanguageList::<Impl, IMPL_OFFSET>,
            EnumLanguageProfiles: EnumLanguageProfiles::<Impl, IMPL_OFFSET>,
            EnableLanguageProfile: EnableLanguageProfile::<Impl, IMPL_OFFSET>,
            IsEnabledLanguageProfile: IsEnabledLanguageProfile::<Impl, IMPL_OFFSET>,
            EnableLanguageProfileByDefault: EnableLanguageProfileByDefault::<Impl, IMPL_OFFSET>,
            SubstituteKeyboardLayout: SubstituteKeyboardLayout::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfInputProcessorProfiles as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ITfInputProcessorProfilesExImpl: Sized + ITfInputProcessorProfilesImpl {
    fn SetLanguageProfileDisplayName(&mut self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, pchfile: super::super::Foundation::PWSTR, cchfile: u32, uresid: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ITfInputProcessorProfilesExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfInputProcessorProfilesExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfInputProcessorProfilesExVtbl {
        unsafe extern "system" fn SetLanguageProfileDisplayName<Impl: ITfInputProcessorProfilesExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, pchfile: super::super::Foundation::PWSTR, cchfile: u32, uresid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLanguageProfileDisplayName(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&guidprofile), ::core::mem::transmute_copy(&pchfile), ::core::mem::transmute_copy(&cchfile), ::core::mem::transmute_copy(&uresid)).into()
        }
        Self {
            base: ITfInputProcessorProfilesVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetLanguageProfileDisplayName: SetLanguageProfileDisplayName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfInputProcessorProfilesEx as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfInputScopeImpl: Sized {
    fn GetInputScopes(&mut self, pprginputscopes: *mut *mut InputScope, pccount: *mut u32) -> ::windows::core::Result<()>;
    fn GetPhrase(&mut self, ppbstrphrases: *mut *mut super::super::Foundation::BSTR, pccount: *mut u32) -> ::windows::core::Result<()>;
    fn GetRegularExpression(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetSRGS(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetXML(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfInputScopeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfInputScopeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfInputScopeVtbl {
        unsafe extern "system" fn GetInputScopes<Impl: ITfInputScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprginputscopes: *mut *mut InputScope, pccount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInputScopes(::core::mem::transmute_copy(&pprginputscopes), ::core::mem::transmute_copy(&pccount)).into()
        }
        unsafe extern "system" fn GetPhrase<Impl: ITfInputScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbstrphrases: *mut *mut super::super::Foundation::BSTR, pccount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPhrase(::core::mem::transmute_copy(&ppbstrphrases), ::core::mem::transmute_copy(&pccount)).into()
        }
        unsafe extern "system" fn GetRegularExpression<Impl: ITfInputScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrregexp: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRegularExpression() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrregexp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSRGS<Impl: ITfInputScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsrgs: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSRGS() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrsrgs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXML<Impl: ITfInputScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrxml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetXML() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrxml = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetInputScopes: GetInputScopes::<Impl, IMPL_OFFSET>,
            GetPhrase: GetPhrase::<Impl, IMPL_OFFSET>,
            GetRegularExpression: GetRegularExpression::<Impl, IMPL_OFFSET>,
            GetSRGS: GetSRGS::<Impl, IMPL_OFFSET>,
            GetXML: GetXML::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfInputScope as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ITfInputScope2Impl: Sized + ITfInputScopeImpl {
    fn EnumWordList(&mut self) -> ::windows::core::Result<super::super::System::Com::IEnumString>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ITfInputScope2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfInputScope2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfInputScope2Vtbl {
        unsafe extern "system" fn EnumWordList<Impl: ITfInputScope2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumstring: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumWordList() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumstring = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ITfInputScopeVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), EnumWordList: EnumWordList::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfInputScope2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ITfInsertAtSelectionImpl: Sized {
    fn InsertTextAtSelection(&mut self, ec: u32, dwflags: INSERT_TEXT_AT_SELECTION_FLAGS, pchtext: super::super::Foundation::PWSTR, cch: i32) -> ::windows::core::Result<ITfRange>;
    fn InsertEmbeddedAtSelection(&mut self, ec: u32, dwflags: u32, pdataobject: ::core::option::Option<super::super::System::Com::IDataObject>) -> ::windows::core::Result<ITfRange>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ITfInsertAtSelectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfInsertAtSelectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfInsertAtSelectionVtbl {
        unsafe extern "system" fn InsertTextAtSelection<Impl: ITfInsertAtSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, dwflags: INSERT_TEXT_AT_SELECTION_FLAGS, pchtext: super::super::Foundation::PWSTR, cch: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InsertTextAtSelection(::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pchtext), ::core::mem::transmute_copy(&cch)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertEmbeddedAtSelection<Impl: ITfInsertAtSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, dwflags: u32, pdataobject: ::windows::core::RawPtr, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InsertEmbeddedAtSelection(::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pdataobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            InsertTextAtSelection: InsertTextAtSelection::<Impl, IMPL_OFFSET>,
            InsertEmbeddedAtSelection: InsertEmbeddedAtSelection::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfInsertAtSelection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfIntegratableCandidateListUIElementImpl: Sized {
    fn SetIntegrationStyle(&mut self, guidintegrationstyle: ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetSelectionStyle(&mut self) -> ::windows::core::Result<TfIntegratableCandidateListSelectionStyle>;
    fn OnKeyDown(&mut self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn ShowCandidateNumbers(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn FinalizeExactCompositionString(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfIntegratableCandidateListUIElementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfIntegratableCandidateListUIElementImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfIntegratableCandidateListUIElementVtbl {
        unsafe extern "system" fn SetIntegrationStyle<Impl: ITfIntegratableCandidateListUIElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidintegrationstyle: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIntegrationStyle(::core::mem::transmute_copy(&guidintegrationstyle)).into()
        }
        unsafe extern "system" fn GetSelectionStyle<Impl: ITfIntegratableCandidateListUIElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptfselectionstyle: *mut TfIntegratableCandidateListSelectionStyle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSelectionStyle() {
                ::core::result::Result::Ok(ok__) => {
                    *ptfselectionstyle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnKeyDown<Impl: ITfIntegratableCandidateListUIElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnKeyDown(::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfeaten = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowCandidateNumbers<Impl: ITfIntegratableCandidateListUIElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfshow: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowCandidateNumbers() {
                ::core::result::Result::Ok(ok__) => {
                    *pfshow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FinalizeExactCompositionString<Impl: ITfIntegratableCandidateListUIElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FinalizeExactCompositionString().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetIntegrationStyle: SetIntegrationStyle::<Impl, IMPL_OFFSET>,
            GetSelectionStyle: GetSelectionStyle::<Impl, IMPL_OFFSET>,
            OnKeyDown: OnKeyDown::<Impl, IMPL_OFFSET>,
            ShowCandidateNumbers: ShowCandidateNumbers::<Impl, IMPL_OFFSET>,
            FinalizeExactCompositionString: FinalizeExactCompositionString::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfIntegratableCandidateListUIElement as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfKeyEventSinkImpl: Sized {
    fn OnSetFocus(&mut self, fforeground: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn OnTestKeyDown(&mut self, pic: ::core::option::Option<ITfContext>, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn OnTestKeyUp(&mut self, pic: ::core::option::Option<ITfContext>, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn OnKeyDown(&mut self, pic: ::core::option::Option<ITfContext>, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn OnKeyUp(&mut self, pic: ::core::option::Option<ITfContext>, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn OnPreservedKey(&mut self, pic: ::core::option::Option<ITfContext>, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfKeyEventSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfKeyEventSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfKeyEventSinkVtbl {
        unsafe extern "system" fn OnSetFocus<Impl: ITfKeyEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fforeground: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnSetFocus(::core::mem::transmute_copy(&fforeground)).into()
        }
        unsafe extern "system" fn OnTestKeyDown<Impl: ITfKeyEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnTestKeyDown(::core::mem::transmute(&pic), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfeaten = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnTestKeyUp<Impl: ITfKeyEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnTestKeyUp(::core::mem::transmute(&pic), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfeaten = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnKeyDown<Impl: ITfKeyEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnKeyDown(::core::mem::transmute(&pic), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfeaten = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnKeyUp<Impl: ITfKeyEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnKeyUp(::core::mem::transmute(&pic), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfeaten = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnPreservedKey<Impl: ITfKeyEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr, rguid: *const ::windows::core::GUID, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnPreservedKey(::core::mem::transmute(&pic), ::core::mem::transmute_copy(&rguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfeaten = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnSetFocus: OnSetFocus::<Impl, IMPL_OFFSET>,
            OnTestKeyDown: OnTestKeyDown::<Impl, IMPL_OFFSET>,
            OnTestKeyUp: OnTestKeyUp::<Impl, IMPL_OFFSET>,
            OnKeyDown: OnKeyDown::<Impl, IMPL_OFFSET>,
            OnKeyUp: OnKeyUp::<Impl, IMPL_OFFSET>,
            OnPreservedKey: OnPreservedKey::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfKeyEventSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfKeyTraceEventSinkImpl: Sized {
    fn OnKeyTraceDown(&mut self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<()>;
    fn OnKeyTraceUp(&mut self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfKeyTraceEventSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfKeyTraceEventSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfKeyTraceEventSinkVtbl {
        unsafe extern "system" fn OnKeyTraceDown<Impl: ITfKeyTraceEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnKeyTraceDown(::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn OnKeyTraceUp<Impl: ITfKeyTraceEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnKeyTraceUp(::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnKeyTraceDown: OnKeyTraceDown::<Impl, IMPL_OFFSET>,
            OnKeyTraceUp: OnKeyTraceUp::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfKeyTraceEventSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfKeystrokeMgrImpl: Sized {
    fn AdviseKeyEventSink(&mut self, tid: u32, psink: ::core::option::Option<ITfKeyEventSink>, fforeground: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn UnadviseKeyEventSink(&mut self, tid: u32) -> ::windows::core::Result<()>;
    fn GetForeground(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TestKeyDown(&mut self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn TestKeyUp(&mut self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn KeyDown(&mut self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn KeyUp(&mut self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetPreservedKey(&mut self, pic: ::core::option::Option<ITfContext>, pprekey: *const TF_PRESERVEDKEY) -> ::windows::core::Result<::windows::core::GUID>;
    fn IsPreservedKey(&mut self, rguid: *const ::windows::core::GUID, pprekey: *const TF_PRESERVEDKEY) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn PreserveKey(&mut self, tid: u32, rguid: *const ::windows::core::GUID, prekey: *const TF_PRESERVEDKEY, pchdesc: super::super::Foundation::PWSTR, cchdesc: u32) -> ::windows::core::Result<()>;
    fn UnpreserveKey(&mut self, rguid: *const ::windows::core::GUID, pprekey: *const TF_PRESERVEDKEY) -> ::windows::core::Result<()>;
    fn SetPreservedKeyDescription(&mut self, rguid: *const ::windows::core::GUID, pchdesc: super::super::Foundation::PWSTR, cchdesc: u32) -> ::windows::core::Result<()>;
    fn GetPreservedKeyDescription(&mut self, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SimulatePreservedKey(&mut self, pic: ::core::option::Option<ITfContext>, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfKeystrokeMgrVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfKeystrokeMgrImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfKeystrokeMgrVtbl {
        unsafe extern "system" fn AdviseKeyEventSink<Impl: ITfKeystrokeMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tid: u32, psink: ::windows::core::RawPtr, fforeground: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AdviseKeyEventSink(::core::mem::transmute_copy(&tid), ::core::mem::transmute(&psink), ::core::mem::transmute_copy(&fforeground)).into()
        }
        unsafe extern "system" fn UnadviseKeyEventSink<Impl: ITfKeystrokeMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnadviseKeyEventSink(::core::mem::transmute_copy(&tid)).into()
        }
        unsafe extern "system" fn GetForeground<Impl: ITfKeystrokeMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForeground() {
                ::core::result::Result::Ok(ok__) => {
                    *pclsid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TestKeyDown<Impl: ITfKeystrokeMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TestKeyDown(::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfeaten = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TestKeyUp<Impl: ITfKeystrokeMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TestKeyUp(::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfeaten = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyDown<Impl: ITfKeystrokeMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyDown(::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfeaten = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyUp<Impl: ITfKeystrokeMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyUp(::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfeaten = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreservedKey<Impl: ITfKeystrokeMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr, pprekey: *const TF_PRESERVEDKEY, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPreservedKey(::core::mem::transmute(&pic), ::core::mem::transmute_copy(&pprekey)) {
                ::core::result::Result::Ok(ok__) => {
                    *pguid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPreservedKey<Impl: ITfKeystrokeMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, pprekey: *const TF_PRESERVEDKEY, pfregistered: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPreservedKey(::core::mem::transmute_copy(&rguid), ::core::mem::transmute_copy(&pprekey)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfregistered = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreserveKey<Impl: ITfKeystrokeMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tid: u32, rguid: *const ::windows::core::GUID, prekey: *const TF_PRESERVEDKEY, pchdesc: super::super::Foundation::PWSTR, cchdesc: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PreserveKey(::core::mem::transmute_copy(&tid), ::core::mem::transmute_copy(&rguid), ::core::mem::transmute_copy(&prekey), ::core::mem::transmute_copy(&pchdesc), ::core::mem::transmute_copy(&cchdesc)).into()
        }
        unsafe extern "system" fn UnpreserveKey<Impl: ITfKeystrokeMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, pprekey: *const TF_PRESERVEDKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnpreserveKey(::core::mem::transmute_copy(&rguid), ::core::mem::transmute_copy(&pprekey)).into()
        }
        unsafe extern "system" fn SetPreservedKeyDescription<Impl: ITfKeystrokeMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, pchdesc: super::super::Foundation::PWSTR, cchdesc: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreservedKeyDescription(::core::mem::transmute_copy(&rguid), ::core::mem::transmute_copy(&pchdesc), ::core::mem::transmute_copy(&cchdesc)).into()
        }
        unsafe extern "system" fn GetPreservedKeyDescription<Impl: ITfKeystrokeMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, pbstrdesc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPreservedKeyDescription(::core::mem::transmute_copy(&rguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SimulatePreservedKey<Impl: ITfKeystrokeMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr, rguid: *const ::windows::core::GUID, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SimulatePreservedKey(::core::mem::transmute(&pic), ::core::mem::transmute_copy(&rguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfeaten = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AdviseKeyEventSink: AdviseKeyEventSink::<Impl, IMPL_OFFSET>,
            UnadviseKeyEventSink: UnadviseKeyEventSink::<Impl, IMPL_OFFSET>,
            GetForeground: GetForeground::<Impl, IMPL_OFFSET>,
            TestKeyDown: TestKeyDown::<Impl, IMPL_OFFSET>,
            TestKeyUp: TestKeyUp::<Impl, IMPL_OFFSET>,
            KeyDown: KeyDown::<Impl, IMPL_OFFSET>,
            KeyUp: KeyUp::<Impl, IMPL_OFFSET>,
            GetPreservedKey: GetPreservedKey::<Impl, IMPL_OFFSET>,
            IsPreservedKey: IsPreservedKey::<Impl, IMPL_OFFSET>,
            PreserveKey: PreserveKey::<Impl, IMPL_OFFSET>,
            UnpreserveKey: UnpreserveKey::<Impl, IMPL_OFFSET>,
            SetPreservedKeyDescription: SetPreservedKeyDescription::<Impl, IMPL_OFFSET>,
            GetPreservedKeyDescription: GetPreservedKeyDescription::<Impl, IMPL_OFFSET>,
            SimulatePreservedKey: SimulatePreservedKey::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfKeystrokeMgr as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfLMLatticeImpl: Sized {
    fn QueryType(&mut self, rguidtype: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn EnumLatticeElements(&mut self, dwframestart: u32, rguidtype: *const ::windows::core::GUID) -> ::windows::core::Result<IEnumTfLatticeElements>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfLMLatticeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfLMLatticeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfLMLatticeVtbl {
        unsafe extern "system" fn QueryType<Impl: ITfLMLatticeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidtype: *const ::windows::core::GUID, pfsupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryType(::core::mem::transmute_copy(&rguidtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfsupported = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumLatticeElements<Impl: ITfLMLatticeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwframestart: u32, rguidtype: *const ::windows::core::GUID, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumLatticeElements(::core::mem::transmute_copy(&dwframestart), ::core::mem::transmute_copy(&rguidtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            QueryType: QueryType::<Impl, IMPL_OFFSET>,
            EnumLatticeElements: EnumLatticeElements::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfLMLattice as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfLangBarEventSinkImpl: Sized {
    fn OnSetFocus(&mut self, dwthreadid: u32) -> ::windows::core::Result<()>;
    fn OnThreadTerminate(&mut self, dwthreadid: u32) -> ::windows::core::Result<()>;
    fn OnThreadItemChange(&mut self, dwthreadid: u32) -> ::windows::core::Result<()>;
    fn OnModalInput(&mut self, dwthreadid: u32, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<()>;
    fn ShowFloating(&mut self, dwflags: u32) -> ::windows::core::Result<()>;
    fn GetItemFloatingRect(&mut self, dwthreadid: u32, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::RECT>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfLangBarEventSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfLangBarEventSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfLangBarEventSinkVtbl {
        unsafe extern "system" fn OnSetFocus<Impl: ITfLangBarEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwthreadid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnSetFocus(::core::mem::transmute_copy(&dwthreadid)).into()
        }
        unsafe extern "system" fn OnThreadTerminate<Impl: ITfLangBarEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwthreadid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnThreadTerminate(::core::mem::transmute_copy(&dwthreadid)).into()
        }
        unsafe extern "system" fn OnThreadItemChange<Impl: ITfLangBarEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwthreadid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnThreadItemChange(::core::mem::transmute_copy(&dwthreadid)).into()
        }
        unsafe extern "system" fn OnModalInput<Impl: ITfLangBarEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwthreadid: u32, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnModalInput(::core::mem::transmute_copy(&dwthreadid), ::core::mem::transmute_copy(&umsg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn ShowFloating<Impl: ITfLangBarEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowFloating(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetItemFloatingRect<Impl: ITfLangBarEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwthreadid: u32, rguid: *const ::windows::core::GUID, prc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemFloatingRect(::core::mem::transmute_copy(&dwthreadid), ::core::mem::transmute_copy(&rguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *prc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnSetFocus: OnSetFocus::<Impl, IMPL_OFFSET>,
            OnThreadTerminate: OnThreadTerminate::<Impl, IMPL_OFFSET>,
            OnThreadItemChange: OnThreadItemChange::<Impl, IMPL_OFFSET>,
            OnModalInput: OnModalInput::<Impl, IMPL_OFFSET>,
            ShowFloating: ShowFloating::<Impl, IMPL_OFFSET>,
            GetItemFloatingRect: GetItemFloatingRect::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfLangBarEventSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfLangBarItemImpl: Sized {
    fn GetInfo(&mut self) -> ::windows::core::Result<TF_LANGBARITEMINFO>;
    fn GetStatus(&mut self) -> ::windows::core::Result<u32>;
    fn Show(&mut self, fshow: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetTooltipString(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfLangBarItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfLangBarItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfLangBarItemVtbl {
        unsafe extern "system" fn GetInfo<Impl: ITfLangBarItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *mut TF_LANGBARITEMINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *pinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: ITfLangBarItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Show<Impl: ITfLangBarItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Show(::core::mem::transmute_copy(&fshow)).into()
        }
        unsafe extern "system" fn GetTooltipString<Impl: ITfLangBarItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtooltip: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTooltipString() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrtooltip = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetInfo: GetInfo::<Impl, IMPL_OFFSET>,
            GetStatus: GetStatus::<Impl, IMPL_OFFSET>,
            Show: Show::<Impl, IMPL_OFFSET>,
            GetTooltipString: GetTooltipString::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfLangBarItem as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfLangBarItemBalloonImpl: Sized + ITfLangBarItemImpl {
    fn OnClick(&mut self, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn GetPreferredSize(&mut self, pszdefault: *const super::super::Foundation::SIZE) -> ::windows::core::Result<super::super::Foundation::SIZE>;
    fn GetBalloonInfo(&mut self) -> ::windows::core::Result<TF_LBBALLOONINFO>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfLangBarItemBalloonVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfLangBarItemBalloonImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfLangBarItemBalloonVtbl {
        unsafe extern "system" fn OnClick<Impl: ITfLangBarItemBalloonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnClick(::core::mem::transmute_copy(&click), ::core::mem::transmute_copy(&pt), ::core::mem::transmute_copy(&prcarea)).into()
        }
        unsafe extern "system" fn GetPreferredSize<Impl: ITfLangBarItemBalloonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdefault: *const super::super::Foundation::SIZE, psz: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPreferredSize(::core::mem::transmute_copy(&pszdefault)) {
                ::core::result::Result::Ok(ok__) => {
                    *psz = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBalloonInfo<Impl: ITfLangBarItemBalloonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *mut TF_LBBALLOONINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBalloonInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *pinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ITfLangBarItemVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            OnClick: OnClick::<Impl, IMPL_OFFSET>,
            GetPreferredSize: GetPreferredSize::<Impl, IMPL_OFFSET>,
            GetBalloonInfo: GetBalloonInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfLangBarItemBalloon as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait ITfLangBarItemBitmapImpl: Sized + ITfLangBarItemImpl {
    fn OnClick(&mut self, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn GetPreferredSize(&mut self, pszdefault: *const super::super::Foundation::SIZE) -> ::windows::core::Result<super::super::Foundation::SIZE>;
    fn DrawBitmap(&mut self, bmwidth: i32, bmheight: i32, dwflags: u32, phbmp: *mut super::super::Graphics::Gdi::HBITMAP, phbmpmask: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ITfLangBarItemBitmapVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfLangBarItemBitmapImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfLangBarItemBitmapVtbl {
        unsafe extern "system" fn OnClick<Impl: ITfLangBarItemBitmapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnClick(::core::mem::transmute_copy(&click), ::core::mem::transmute_copy(&pt), ::core::mem::transmute_copy(&prcarea)).into()
        }
        unsafe extern "system" fn GetPreferredSize<Impl: ITfLangBarItemBitmapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdefault: *const super::super::Foundation::SIZE, psz: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPreferredSize(::core::mem::transmute_copy(&pszdefault)) {
                ::core::result::Result::Ok(ok__) => {
                    *psz = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawBitmap<Impl: ITfLangBarItemBitmapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmwidth: i32, bmheight: i32, dwflags: u32, phbmp: *mut super::super::Graphics::Gdi::HBITMAP, phbmpmask: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DrawBitmap(::core::mem::transmute_copy(&bmwidth), ::core::mem::transmute_copy(&bmheight), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&phbmp), ::core::mem::transmute_copy(&phbmpmask)).into()
        }
        Self {
            base: ITfLangBarItemVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            OnClick: OnClick::<Impl, IMPL_OFFSET>,
            GetPreferredSize: GetPreferredSize::<Impl, IMPL_OFFSET>,
            DrawBitmap: DrawBitmap::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfLangBarItemBitmap as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait ITfLangBarItemBitmapButtonImpl: Sized + ITfLangBarItemImpl {
    fn OnClick(&mut self, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn InitMenu(&mut self, pmenu: ::core::option::Option<ITfMenu>) -> ::windows::core::Result<()>;
    fn OnMenuSelect(&mut self, wid: u32) -> ::windows::core::Result<()>;
    fn GetPreferredSize(&mut self, pszdefault: *const super::super::Foundation::SIZE) -> ::windows::core::Result<super::super::Foundation::SIZE>;
    fn DrawBitmap(&mut self, bmwidth: i32, bmheight: i32, dwflags: u32, phbmp: *mut super::super::Graphics::Gdi::HBITMAP, phbmpmask: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::Result<()>;
    fn GetText(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ITfLangBarItemBitmapButtonVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfLangBarItemBitmapButtonImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfLangBarItemBitmapButtonVtbl {
        unsafe extern "system" fn OnClick<Impl: ITfLangBarItemBitmapButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnClick(::core::mem::transmute_copy(&click), ::core::mem::transmute_copy(&pt), ::core::mem::transmute_copy(&prcarea)).into()
        }
        unsafe extern "system" fn InitMenu<Impl: ITfLangBarItemBitmapButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmenu: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitMenu(::core::mem::transmute(&pmenu)).into()
        }
        unsafe extern "system" fn OnMenuSelect<Impl: ITfLangBarItemBitmapButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnMenuSelect(::core::mem::transmute_copy(&wid)).into()
        }
        unsafe extern "system" fn GetPreferredSize<Impl: ITfLangBarItemBitmapButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdefault: *const super::super::Foundation::SIZE, psz: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPreferredSize(::core::mem::transmute_copy(&pszdefault)) {
                ::core::result::Result::Ok(ok__) => {
                    *psz = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawBitmap<Impl: ITfLangBarItemBitmapButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmwidth: i32, bmheight: i32, dwflags: u32, phbmp: *mut super::super::Graphics::Gdi::HBITMAP, phbmpmask: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DrawBitmap(::core::mem::transmute_copy(&bmwidth), ::core::mem::transmute_copy(&bmheight), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&phbmp), ::core::mem::transmute_copy(&phbmpmask)).into()
        }
        unsafe extern "system" fn GetText<Impl: ITfLangBarItemBitmapButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetText() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrtext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ITfLangBarItemVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            OnClick: OnClick::<Impl, IMPL_OFFSET>,
            InitMenu: InitMenu::<Impl, IMPL_OFFSET>,
            OnMenuSelect: OnMenuSelect::<Impl, IMPL_OFFSET>,
            GetPreferredSize: GetPreferredSize::<Impl, IMPL_OFFSET>,
            DrawBitmap: DrawBitmap::<Impl, IMPL_OFFSET>,
            GetText: GetText::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfLangBarItemBitmapButton as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait ITfLangBarItemButtonImpl: Sized + ITfLangBarItemImpl {
    fn OnClick(&mut self, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn InitMenu(&mut self, pmenu: ::core::option::Option<ITfMenu>) -> ::windows::core::Result<()>;
    fn OnMenuSelect(&mut self, wid: u32) -> ::windows::core::Result<()>;
    fn GetIcon(&mut self) -> ::windows::core::Result<super::WindowsAndMessaging::HICON>;
    fn GetText(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ITfLangBarItemButtonVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfLangBarItemButtonImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfLangBarItemButtonVtbl {
        unsafe extern "system" fn OnClick<Impl: ITfLangBarItemButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnClick(::core::mem::transmute_copy(&click), ::core::mem::transmute_copy(&pt), ::core::mem::transmute_copy(&prcarea)).into()
        }
        unsafe extern "system" fn InitMenu<Impl: ITfLangBarItemButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmenu: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitMenu(::core::mem::transmute(&pmenu)).into()
        }
        unsafe extern "system" fn OnMenuSelect<Impl: ITfLangBarItemButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnMenuSelect(::core::mem::transmute_copy(&wid)).into()
        }
        unsafe extern "system" fn GetIcon<Impl: ITfLangBarItemButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phicon: *mut super::WindowsAndMessaging::HICON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIcon() {
                ::core::result::Result::Ok(ok__) => {
                    *phicon = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetText<Impl: ITfLangBarItemButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetText() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrtext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ITfLangBarItemVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            OnClick: OnClick::<Impl, IMPL_OFFSET>,
            InitMenu: InitMenu::<Impl, IMPL_OFFSET>,
            OnMenuSelect: OnMenuSelect::<Impl, IMPL_OFFSET>,
            GetIcon: GetIcon::<Impl, IMPL_OFFSET>,
            GetText: GetText::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfLangBarItemButton as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfLangBarItemMgrImpl: Sized {
    fn EnumItems(&mut self) -> ::windows::core::Result<IEnumTfLangBarItems>;
    fn GetItem(&mut self, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<ITfLangBarItem>;
    fn AddItem(&mut self, punk: ::core::option::Option<ITfLangBarItem>) -> ::windows::core::Result<()>;
    fn RemoveItem(&mut self, punk: ::core::option::Option<ITfLangBarItem>) -> ::windows::core::Result<()>;
    fn AdviseItemSink(&mut self, punk: ::core::option::Option<ITfLangBarItemSink>, pdwcookie: *mut u32, rguiditem: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn UnadviseItemSink(&mut self, dwcookie: u32) -> ::windows::core::Result<()>;
    fn GetItemFloatingRect(&mut self, dwthreadid: u32, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::RECT>;
    fn GetItemsStatus(&mut self, ulcount: u32, prgguid: *const ::windows::core::GUID, pdwstatus: *mut u32) -> ::windows::core::Result<()>;
    fn GetItemNum(&mut self) -> ::windows::core::Result<u32>;
    fn GetItems(&mut self, ulcount: u32, ppitem: *mut ::core::option::Option<ITfLangBarItem>, pinfo: *mut TF_LANGBARITEMINFO, pdwstatus: *mut u32, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn AdviseItemsSink(&mut self, ulcount: u32, ppunk: *const ::core::option::Option<ITfLangBarItemSink>, pguiditem: *const ::windows::core::GUID, pdwcookie: *mut u32) -> ::windows::core::Result<()>;
    fn UnadviseItemsSink(&mut self, ulcount: u32, pdwcookie: *const u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfLangBarItemMgrVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfLangBarItemMgrImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfLangBarItemMgrVtbl {
        unsafe extern "system" fn EnumItems<Impl: ITfLangBarItemMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumItems() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItem<Impl: ITfLangBarItemMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItem(::core::mem::transmute_copy(&rguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddItem<Impl: ITfLangBarItemMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddItem(::core::mem::transmute(&punk)).into()
        }
        unsafe extern "system" fn RemoveItem<Impl: ITfLangBarItemMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveItem(::core::mem::transmute(&punk)).into()
        }
        unsafe extern "system" fn AdviseItemSink<Impl: ITfLangBarItemMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: ::windows::core::RawPtr, pdwcookie: *mut u32, rguiditem: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AdviseItemSink(::core::mem::transmute(&punk), ::core::mem::transmute_copy(&pdwcookie), ::core::mem::transmute_copy(&rguiditem)).into()
        }
        unsafe extern "system" fn UnadviseItemSink<Impl: ITfLangBarItemMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnadviseItemSink(::core::mem::transmute_copy(&dwcookie)).into()
        }
        unsafe extern "system" fn GetItemFloatingRect<Impl: ITfLangBarItemMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwthreadid: u32, rguid: *const ::windows::core::GUID, prc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemFloatingRect(::core::mem::transmute_copy(&dwthreadid), ::core::mem::transmute_copy(&rguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *prc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemsStatus<Impl: ITfLangBarItemMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, prgguid: *const ::windows::core::GUID, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetItemsStatus(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&prgguid), ::core::mem::transmute_copy(&pdwstatus)).into()
        }
        unsafe extern "system" fn GetItemNum<Impl: ITfLangBarItemMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemNum() {
                ::core::result::Result::Ok(ok__) => {
                    *pulcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItems<Impl: ITfLangBarItemMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, ppitem: *mut ::windows::core::RawPtr, pinfo: *mut TF_LANGBARITEMINFO, pdwstatus: *mut u32, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetItems(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&ppitem), ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&pdwstatus), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn AdviseItemsSink<Impl: ITfLangBarItemMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, ppunk: *const ::windows::core::RawPtr, pguiditem: *const ::windows::core::GUID, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AdviseItemsSink(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&ppunk), ::core::mem::transmute_copy(&pguiditem), ::core::mem::transmute_copy(&pdwcookie)).into()
        }
        unsafe extern "system" fn UnadviseItemsSink<Impl: ITfLangBarItemMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, pdwcookie: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnadviseItemsSink(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pdwcookie)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            EnumItems: EnumItems::<Impl, IMPL_OFFSET>,
            GetItem: GetItem::<Impl, IMPL_OFFSET>,
            AddItem: AddItem::<Impl, IMPL_OFFSET>,
            RemoveItem: RemoveItem::<Impl, IMPL_OFFSET>,
            AdviseItemSink: AdviseItemSink::<Impl, IMPL_OFFSET>,
            UnadviseItemSink: UnadviseItemSink::<Impl, IMPL_OFFSET>,
            GetItemFloatingRect: GetItemFloatingRect::<Impl, IMPL_OFFSET>,
            GetItemsStatus: GetItemsStatus::<Impl, IMPL_OFFSET>,
            GetItemNum: GetItemNum::<Impl, IMPL_OFFSET>,
            GetItems: GetItems::<Impl, IMPL_OFFSET>,
            AdviseItemsSink: AdviseItemsSink::<Impl, IMPL_OFFSET>,
            UnadviseItemsSink: UnadviseItemsSink::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfLangBarItemMgr as ::windows::core::Interface>::IID
    }
}
pub trait ITfLangBarItemSinkImpl: Sized {
    fn OnUpdate(&mut self, dwflags: u32) -> ::windows::core::Result<()>;
}
impl ITfLangBarItemSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfLangBarItemSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfLangBarItemSinkVtbl {
        unsafe extern "system" fn OnUpdate<Impl: ITfLangBarItemSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnUpdate(::core::mem::transmute_copy(&dwflags)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnUpdate: OnUpdate::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfLangBarItemSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfLangBarMgrImpl: Sized {
    fn AdviseEventSink(&mut self, psink: ::core::option::Option<ITfLangBarEventSink>, hwnd: super::super::Foundation::HWND, dwflags: u32, pdwcookie: *const u32) -> ::windows::core::Result<()>;
    fn UnadviseEventSink(&mut self, dwcookie: u32) -> ::windows::core::Result<()>;
    fn GetThreadMarshalInterface(&mut self, dwthreadid: u32, dwtype: u32, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetThreadLangBarItemMgr(&mut self, dwthreadid: u32, pplbi: *mut ::core::option::Option<ITfLangBarItemMgr>, pdwthreadid: *mut u32) -> ::windows::core::Result<()>;
    fn GetInputProcessorProfiles(&mut self, dwthreadid: u32, ppaip: *mut ::core::option::Option<ITfInputProcessorProfiles>, pdwthreadid: *mut u32) -> ::windows::core::Result<()>;
    fn RestoreLastFocus(&mut self, pdwthreadid: *mut u32, fprev: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetModalInput(&mut self, psink: ::core::option::Option<ITfLangBarEventSink>, dwthreadid: u32, dwflags: u32) -> ::windows::core::Result<()>;
    fn ShowFloating(&mut self, dwflags: u32) -> ::windows::core::Result<()>;
    fn GetShowFloatingStatus(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfLangBarMgrVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfLangBarMgrImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfLangBarMgrVtbl {
        unsafe extern "system" fn AdviseEventSink<Impl: ITfLangBarMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psink: ::windows::core::RawPtr, hwnd: super::super::Foundation::HWND, dwflags: u32, pdwcookie: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AdviseEventSink(::core::mem::transmute(&psink), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pdwcookie)).into()
        }
        unsafe extern "system" fn UnadviseEventSink<Impl: ITfLangBarMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnadviseEventSink(::core::mem::transmute_copy(&dwcookie)).into()
        }
        unsafe extern "system" fn GetThreadMarshalInterface<Impl: ITfLangBarMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwthreadid: u32, dwtype: u32, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetThreadMarshalInterface(::core::mem::transmute_copy(&dwthreadid), ::core::mem::transmute_copy(&dwtype), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThreadLangBarItemMgr<Impl: ITfLangBarMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwthreadid: u32, pplbi: *mut ::windows::core::RawPtr, pdwthreadid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetThreadLangBarItemMgr(::core::mem::transmute_copy(&dwthreadid), ::core::mem::transmute_copy(&pplbi), ::core::mem::transmute_copy(&pdwthreadid)).into()
        }
        unsafe extern "system" fn GetInputProcessorProfiles<Impl: ITfLangBarMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwthreadid: u32, ppaip: *mut ::windows::core::RawPtr, pdwthreadid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInputProcessorProfiles(::core::mem::transmute_copy(&dwthreadid), ::core::mem::transmute_copy(&ppaip), ::core::mem::transmute_copy(&pdwthreadid)).into()
        }
        unsafe extern "system" fn RestoreLastFocus<Impl: ITfLangBarMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwthreadid: *mut u32, fprev: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RestoreLastFocus(::core::mem::transmute_copy(&pdwthreadid), ::core::mem::transmute_copy(&fprev)).into()
        }
        unsafe extern "system" fn SetModalInput<Impl: ITfLangBarMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psink: ::windows::core::RawPtr, dwthreadid: u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetModalInput(::core::mem::transmute(&psink), ::core::mem::transmute_copy(&dwthreadid), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn ShowFloating<Impl: ITfLangBarMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowFloating(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetShowFloatingStatus<Impl: ITfLangBarMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetShowFloatingStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AdviseEventSink: AdviseEventSink::<Impl, IMPL_OFFSET>,
            UnadviseEventSink: UnadviseEventSink::<Impl, IMPL_OFFSET>,
            GetThreadMarshalInterface: GetThreadMarshalInterface::<Impl, IMPL_OFFSET>,
            GetThreadLangBarItemMgr: GetThreadLangBarItemMgr::<Impl, IMPL_OFFSET>,
            GetInputProcessorProfiles: GetInputProcessorProfiles::<Impl, IMPL_OFFSET>,
            RestoreLastFocus: RestoreLastFocus::<Impl, IMPL_OFFSET>,
            SetModalInput: SetModalInput::<Impl, IMPL_OFFSET>,
            ShowFloating: ShowFloating::<Impl, IMPL_OFFSET>,
            GetShowFloatingStatus: GetShowFloatingStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfLangBarMgr as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfLanguageProfileNotifySinkImpl: Sized {
    fn OnLanguageChange(&mut self, langid: u16) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn OnLanguageChanged(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfLanguageProfileNotifySinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfLanguageProfileNotifySinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfLanguageProfileNotifySinkVtbl {
        unsafe extern "system" fn OnLanguageChange<Impl: ITfLanguageProfileNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16, pfaccept: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnLanguageChange(::core::mem::transmute_copy(&langid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfaccept = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnLanguageChanged<Impl: ITfLanguageProfileNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnLanguageChanged().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnLanguageChange: OnLanguageChange::<Impl, IMPL_OFFSET>,
            OnLanguageChanged: OnLanguageChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfLanguageProfileNotifySink as ::windows::core::Interface>::IID
    }
}
pub trait ITfMSAAControlImpl: Sized {
    fn SystemEnableMSAA(&mut self) -> ::windows::core::Result<()>;
    fn SystemDisableMSAA(&mut self) -> ::windows::core::Result<()>;
}
impl ITfMSAAControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfMSAAControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfMSAAControlVtbl {
        unsafe extern "system" fn SystemEnableMSAA<Impl: ITfMSAAControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SystemEnableMSAA().into()
        }
        unsafe extern "system" fn SystemDisableMSAA<Impl: ITfMSAAControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SystemDisableMSAA().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SystemEnableMSAA: SystemEnableMSAA::<Impl, IMPL_OFFSET>,
            SystemDisableMSAA: SystemDisableMSAA::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfMSAAControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait ITfMenuImpl: Sized {
    fn AddMenuItem(&mut self, uid: u32, dwflags: u32, hbmp: super::super::Graphics::Gdi::HBITMAP, hbmpmask: super::super::Graphics::Gdi::HBITMAP, pch: super::super::Foundation::PWSTR, cch: u32, ppmenu: *mut ::core::option::Option<ITfMenu>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ITfMenuVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfMenuImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfMenuVtbl {
        unsafe extern "system" fn AddMenuItem<Impl: ITfMenuImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uid: u32, dwflags: u32, hbmp: super::super::Graphics::Gdi::HBITMAP, hbmpmask: super::super::Graphics::Gdi::HBITMAP, pch: super::super::Foundation::PWSTR, cch: u32, ppmenu: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddMenuItem(::core::mem::transmute_copy(&uid), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&hbmp), ::core::mem::transmute_copy(&hbmpmask), ::core::mem::transmute_copy(&pch), ::core::mem::transmute_copy(&cch), ::core::mem::transmute_copy(&ppmenu)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), AddMenuItem: AddMenuItem::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfMenu as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait ITfMessagePumpImpl: Sized {
    fn PeekMessageA(&mut self, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetMessageA(&mut self, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn PeekMessageW(&mut self, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetMessageW(&mut self, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ITfMessagePumpVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfMessagePumpImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfMessagePumpVtbl {
        unsafe extern "system" fn PeekMessageA<Impl: ITfMessagePumpImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PeekMessageA(::core::mem::transmute_copy(&pmsg), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&wmsgfiltermin), ::core::mem::transmute_copy(&wmsgfiltermax), ::core::mem::transmute_copy(&wremovemsg), ::core::mem::transmute_copy(&pfresult)).into()
        }
        unsafe extern "system" fn GetMessageA<Impl: ITfMessagePumpImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMessageA(::core::mem::transmute_copy(&pmsg), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&wmsgfiltermin), ::core::mem::transmute_copy(&wmsgfiltermax), ::core::mem::transmute_copy(&pfresult)).into()
        }
        unsafe extern "system" fn PeekMessageW<Impl: ITfMessagePumpImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PeekMessageW(::core::mem::transmute_copy(&pmsg), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&wmsgfiltermin), ::core::mem::transmute_copy(&wmsgfiltermax), ::core::mem::transmute_copy(&wremovemsg), ::core::mem::transmute_copy(&pfresult)).into()
        }
        unsafe extern "system" fn GetMessageW<Impl: ITfMessagePumpImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMessageW(::core::mem::transmute_copy(&pmsg), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&wmsgfiltermin), ::core::mem::transmute_copy(&wmsgfiltermax), ::core::mem::transmute_copy(&pfresult)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            PeekMessageA: PeekMessageA::<Impl, IMPL_OFFSET>,
            GetMessageA: GetMessageA::<Impl, IMPL_OFFSET>,
            PeekMessageW: PeekMessageW::<Impl, IMPL_OFFSET>,
            GetMessageW: GetMessageW::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfMessagePump as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfMouseSinkImpl: Sized {
    fn OnMouseEvent(&mut self, uedge: u32, uquadrant: u32, dwbtnstatus: u32) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfMouseSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfMouseSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfMouseSinkVtbl {
        unsafe extern "system" fn OnMouseEvent<Impl: ITfMouseSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uedge: u32, uquadrant: u32, dwbtnstatus: u32, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnMouseEvent(::core::mem::transmute_copy(&uedge), ::core::mem::transmute_copy(&uquadrant), ::core::mem::transmute_copy(&dwbtnstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfeaten = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnMouseEvent: OnMouseEvent::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfMouseSink as ::windows::core::Interface>::IID
    }
}
pub trait ITfMouseTrackerImpl: Sized {
    fn AdviseMouseSink(&mut self, range: ::core::option::Option<ITfRange>, psink: ::core::option::Option<ITfMouseSink>) -> ::windows::core::Result<u32>;
    fn UnadviseMouseSink(&mut self, dwcookie: u32) -> ::windows::core::Result<()>;
}
impl ITfMouseTrackerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfMouseTrackerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfMouseTrackerVtbl {
        unsafe extern "system" fn AdviseMouseSink<Impl: ITfMouseTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, range: ::windows::core::RawPtr, psink: ::windows::core::RawPtr, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdviseMouseSink(::core::mem::transmute(&range), ::core::mem::transmute(&psink)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnadviseMouseSink<Impl: ITfMouseTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnadviseMouseSink(::core::mem::transmute_copy(&dwcookie)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AdviseMouseSink: AdviseMouseSink::<Impl, IMPL_OFFSET>,
            UnadviseMouseSink: UnadviseMouseSink::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfMouseTracker as ::windows::core::Interface>::IID
    }
}
pub trait ITfMouseTrackerACPImpl: Sized {
    fn AdviseMouseSink(&mut self, range: ::core::option::Option<ITfRangeACP>, psink: ::core::option::Option<ITfMouseSink>) -> ::windows::core::Result<u32>;
    fn UnadviseMouseSink(&mut self, dwcookie: u32) -> ::windows::core::Result<()>;
}
impl ITfMouseTrackerACPVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfMouseTrackerACPImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfMouseTrackerACPVtbl {
        unsafe extern "system" fn AdviseMouseSink<Impl: ITfMouseTrackerACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, range: ::windows::core::RawPtr, psink: ::windows::core::RawPtr, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdviseMouseSink(::core::mem::transmute(&range), ::core::mem::transmute(&psink)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnadviseMouseSink<Impl: ITfMouseTrackerACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnadviseMouseSink(::core::mem::transmute_copy(&dwcookie)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AdviseMouseSink: AdviseMouseSink::<Impl, IMPL_OFFSET>,
            UnadviseMouseSink: UnadviseMouseSink::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfMouseTrackerACP as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITfPersistentPropertyLoaderACPImpl: Sized {
    fn LoadProperty(&mut self, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP) -> ::windows::core::Result<super::super::System::Com::IStream>;
}
#[cfg(feature = "Win32_System_Com")]
impl ITfPersistentPropertyLoaderACPVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfPersistentPropertyLoaderACPImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfPersistentPropertyLoaderACPVtbl {
        unsafe extern "system" fn LoadProperty<Impl: ITfPersistentPropertyLoaderACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadProperty(::core::mem::transmute_copy(&phdr)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), LoadProperty: LoadProperty::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfPersistentPropertyLoaderACP as ::windows::core::Interface>::IID
    }
}
pub trait ITfPreservedKeyNotifySinkImpl: Sized {
    fn OnUpdated(&mut self, pprekey: *const TF_PRESERVEDKEY) -> ::windows::core::Result<()>;
}
impl ITfPreservedKeyNotifySinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfPreservedKeyNotifySinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfPreservedKeyNotifySinkVtbl {
        unsafe extern "system" fn OnUpdated<Impl: ITfPreservedKeyNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprekey: *const TF_PRESERVEDKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnUpdated(::core::mem::transmute_copy(&pprekey)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnUpdated: OnUpdated::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfPreservedKeyNotifySink as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITfPropertyImpl: Sized + ITfReadOnlyPropertyImpl {
    fn FindRange(&mut self, ec: u32, prange: ::core::option::Option<ITfRange>, pprange: *mut ::core::option::Option<ITfRange>, apos: TfAnchor) -> ::windows::core::Result<()>;
    fn SetValueStore(&mut self, ec: u32, prange: ::core::option::Option<ITfRange>, ppropstore: ::core::option::Option<ITfPropertyStore>) -> ::windows::core::Result<()>;
    fn SetValue(&mut self, ec: u32, prange: ::core::option::Option<ITfRange>, pvarvalue: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Clear(&mut self, ec: u32, prange: ::core::option::Option<ITfRange>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITfPropertyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfPropertyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfPropertyVtbl {
        unsafe extern "system" fn FindRange<Impl: ITfPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, prange: ::windows::core::RawPtr, pprange: *mut ::windows::core::RawPtr, apos: TfAnchor) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FindRange(::core::mem::transmute_copy(&ec), ::core::mem::transmute(&prange), ::core::mem::transmute_copy(&pprange), ::core::mem::transmute_copy(&apos)).into()
        }
        unsafe extern "system" fn SetValueStore<Impl: ITfPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, prange: ::windows::core::RawPtr, ppropstore: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValueStore(::core::mem::transmute_copy(&ec), ::core::mem::transmute(&prange), ::core::mem::transmute(&ppropstore)).into()
        }
        unsafe extern "system" fn SetValue<Impl: ITfPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, prange: ::windows::core::RawPtr, pvarvalue: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&ec), ::core::mem::transmute(&prange), ::core::mem::transmute_copy(&pvarvalue)).into()
        }
        unsafe extern "system" fn Clear<Impl: ITfPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear(::core::mem::transmute_copy(&ec), ::core::mem::transmute(&prange)).into()
        }
        Self {
            base: ITfReadOnlyPropertyVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            FindRange: FindRange::<Impl, IMPL_OFFSET>,
            SetValueStore: SetValueStore::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfProperty as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITfPropertyStoreImpl: Sized {
    fn GetType(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetDataType(&mut self) -> ::windows::core::Result<u32>;
    fn GetData(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn OnTextUpdated(&mut self, dwflags: u32, prangenew: ::core::option::Option<ITfRange>) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn Shrink(&mut self, prangenew: ::core::option::Option<ITfRange>) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn Divide(&mut self, prangethis: ::core::option::Option<ITfRange>, prangenew: ::core::option::Option<ITfRange>) -> ::windows::core::Result<ITfPropertyStore>;
    fn Clone(&mut self) -> ::windows::core::Result<ITfPropertyStore>;
    fn GetPropertyRangeCreator(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Serialize(&mut self, pstream: ::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITfPropertyStoreVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfPropertyStoreImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfPropertyStoreVtbl {
        unsafe extern "system" fn GetType<Impl: ITfPropertyStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetType() {
                ::core::result::Result::Ok(ok__) => {
                    *pguid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataType<Impl: ITfPropertyStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwreserved: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDataType() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwreserved = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetData<Impl: ITfPropertyStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetData() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnTextUpdated<Impl: ITfPropertyStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, prangenew: ::windows::core::RawPtr, pfaccept: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnTextUpdated(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&prangenew)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfaccept = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shrink<Impl: ITfPropertyStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prangenew: ::windows::core::RawPtr, pffree: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Shrink(::core::mem::transmute(&prangenew)) {
                ::core::result::Result::Ok(ok__) => {
                    *pffree = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Divide<Impl: ITfPropertyStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prangethis: ::windows::core::RawPtr, prangenew: ::windows::core::RawPtr, pppropstore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Divide(::core::mem::transmute(&prangethis), ::core::mem::transmute(&prangenew)) {
                ::core::result::Result::Ok(ok__) => {
                    *pppropstore = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: ITfPropertyStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropstore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppropstore = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyRangeCreator<Impl: ITfPropertyStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertyRangeCreator() {
                ::core::result::Result::Ok(ok__) => {
                    *pclsid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Serialize<Impl: ITfPropertyStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, pcb: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Serialize(::core::mem::transmute(&pstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcb = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetType: GetType::<Impl, IMPL_OFFSET>,
            GetDataType: GetDataType::<Impl, IMPL_OFFSET>,
            GetData: GetData::<Impl, IMPL_OFFSET>,
            OnTextUpdated: OnTextUpdated::<Impl, IMPL_OFFSET>,
            Shrink: Shrink::<Impl, IMPL_OFFSET>,
            Divide: Divide::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
            GetPropertyRangeCreator: GetPropertyRangeCreator::<Impl, IMPL_OFFSET>,
            Serialize: Serialize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfPropertyStore as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ITfQueryEmbeddedImpl: Sized {
    fn QueryInsertEmbedded(&mut self, pguidservice: *const ::windows::core::GUID, pformatetc: *const super::super::System::Com::FORMATETC) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ITfQueryEmbeddedVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfQueryEmbeddedImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfQueryEmbeddedVtbl {
        unsafe extern "system" fn QueryInsertEmbedded<Impl: ITfQueryEmbeddedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidservice: *const ::windows::core::GUID, pformatetc: *const super::super::System::Com::FORMATETC, pfinsertable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryInsertEmbedded(::core::mem::transmute_copy(&pguidservice), ::core::mem::transmute_copy(&pformatetc)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfinsertable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), QueryInsertEmbedded: QueryInsertEmbedded::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfQueryEmbedded as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ITfRangeImpl: Sized {
    fn GetText(&mut self, ec: u32, dwflags: u32, pchtext: super::super::Foundation::PWSTR, cchmax: u32, pcch: *mut u32) -> ::windows::core::Result<()>;
    fn SetText(&mut self, ec: u32, dwflags: u32, pchtext: super::super::Foundation::PWSTR, cch: i32) -> ::windows::core::Result<()>;
    fn GetFormattedText(&mut self, ec: u32) -> ::windows::core::Result<super::super::System::Com::IDataObject>;
    fn GetEmbedded(&mut self, ec: u32, rguidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn InsertEmbedded(&mut self, ec: u32, dwflags: u32, pdataobject: ::core::option::Option<super::super::System::Com::IDataObject>) -> ::windows::core::Result<()>;
    fn ShiftStart(&mut self, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> ::windows::core::Result<()>;
    fn ShiftEnd(&mut self, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> ::windows::core::Result<()>;
    fn ShiftStartToRange(&mut self, ec: u32, prange: ::core::option::Option<ITfRange>, apos: TfAnchor) -> ::windows::core::Result<()>;
    fn ShiftEndToRange(&mut self, ec: u32, prange: ::core::option::Option<ITfRange>, apos: TfAnchor) -> ::windows::core::Result<()>;
    fn ShiftStartRegion(&mut self, ec: u32, dir: TfShiftDir) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn ShiftEndRegion(&mut self, ec: u32, dir: TfShiftDir) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsEmpty(&mut self, ec: u32) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn Collapse(&mut self, ec: u32, apos: TfAnchor) -> ::windows::core::Result<()>;
    fn IsEqualStart(&mut self, ec: u32, pwith: ::core::option::Option<ITfRange>, apos: TfAnchor) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsEqualEnd(&mut self, ec: u32, pwith: ::core::option::Option<ITfRange>, apos: TfAnchor) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CompareStart(&mut self, ec: u32, pwith: ::core::option::Option<ITfRange>, apos: TfAnchor) -> ::windows::core::Result<i32>;
    fn CompareEnd(&mut self, ec: u32, pwith: ::core::option::Option<ITfRange>, apos: TfAnchor) -> ::windows::core::Result<i32>;
    fn AdjustForInsert(&mut self, ec: u32, cchinsert: u32) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetGravity(&mut self, pgstart: *mut TfGravity, pgend: *mut TfGravity) -> ::windows::core::Result<()>;
    fn SetGravity(&mut self, ec: u32, gstart: TfGravity, gend: TfGravity) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<ITfRange>;
    fn GetContext(&mut self) -> ::windows::core::Result<ITfContext>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ITfRangeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfRangeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfRangeVtbl {
        unsafe extern "system" fn GetText<Impl: ITfRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, dwflags: u32, pchtext: super::super::Foundation::PWSTR, cchmax: u32, pcch: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetText(::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pchtext), ::core::mem::transmute_copy(&cchmax), ::core::mem::transmute_copy(&pcch)).into()
        }
        unsafe extern "system" fn SetText<Impl: ITfRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, dwflags: u32, pchtext: super::super::Foundation::PWSTR, cch: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetText(::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pchtext), ::core::mem::transmute_copy(&cch)).into()
        }
        unsafe extern "system" fn GetFormattedText<Impl: ITfRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, ppdataobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFormattedText(::core::mem::transmute_copy(&ec)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdataobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEmbedded<Impl: ITfRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, rguidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEmbedded(::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&rguidservice), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertEmbedded<Impl: ITfRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, dwflags: u32, pdataobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertEmbedded(::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pdataobject)).into()
        }
        unsafe extern "system" fn ShiftStart<Impl: ITfRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShiftStart(::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&cchreq), ::core::mem::transmute_copy(&pcch), ::core::mem::transmute_copy(&phalt)).into()
        }
        unsafe extern "system" fn ShiftEnd<Impl: ITfRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShiftEnd(::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&cchreq), ::core::mem::transmute_copy(&pcch), ::core::mem::transmute_copy(&phalt)).into()
        }
        unsafe extern "system" fn ShiftStartToRange<Impl: ITfRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, prange: ::windows::core::RawPtr, apos: TfAnchor) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShiftStartToRange(::core::mem::transmute_copy(&ec), ::core::mem::transmute(&prange), ::core::mem::transmute_copy(&apos)).into()
        }
        unsafe extern "system" fn ShiftEndToRange<Impl: ITfRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, prange: ::windows::core::RawPtr, apos: TfAnchor) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShiftEndToRange(::core::mem::transmute_copy(&ec), ::core::mem::transmute(&prange), ::core::mem::transmute_copy(&apos)).into()
        }
        unsafe extern "system" fn ShiftStartRegion<Impl: ITfRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, dir: TfShiftDir, pfnoregion: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShiftStartRegion(::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&dir)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfnoregion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShiftEndRegion<Impl: ITfRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, dir: TfShiftDir, pfnoregion: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShiftEndRegion(::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&dir)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfnoregion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEmpty<Impl: ITfRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, pfempty: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEmpty(::core::mem::transmute_copy(&ec)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfempty = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Collapse<Impl: ITfRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, apos: TfAnchor) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Collapse(::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&apos)).into()
        }
        unsafe extern "system" fn IsEqualStart<Impl: ITfRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, pwith: ::windows::core::RawPtr, apos: TfAnchor, pfequal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEqualStart(::core::mem::transmute_copy(&ec), ::core::mem::transmute(&pwith), ::core::mem::transmute_copy(&apos)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfequal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqualEnd<Impl: ITfRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, pwith: ::windows::core::RawPtr, apos: TfAnchor, pfequal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEqualEnd(::core::mem::transmute_copy(&ec), ::core::mem::transmute(&pwith), ::core::mem::transmute_copy(&apos)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfequal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareStart<Impl: ITfRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, pwith: ::windows::core::RawPtr, apos: TfAnchor, plresult: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompareStart(::core::mem::transmute_copy(&ec), ::core::mem::transmute(&pwith), ::core::mem::transmute_copy(&apos)) {
                ::core::result::Result::Ok(ok__) => {
                    *plresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareEnd<Impl: ITfRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, pwith: ::windows::core::RawPtr, apos: TfAnchor, plresult: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompareEnd(::core::mem::transmute_copy(&ec), ::core::mem::transmute(&pwith), ::core::mem::transmute_copy(&apos)) {
                ::core::result::Result::Ok(ok__) => {
                    *plresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdjustForInsert<Impl: ITfRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, cchinsert: u32, pfinsertok: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdjustForInsert(::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&cchinsert)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfinsertok = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGravity<Impl: ITfRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgstart: *mut TfGravity, pgend: *mut TfGravity) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetGravity(::core::mem::transmute_copy(&pgstart), ::core::mem::transmute_copy(&pgend)).into()
        }
        unsafe extern "system" fn SetGravity<Impl: ITfRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, gstart: TfGravity, gend: TfGravity) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGravity(::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&gstart), ::core::mem::transmute_copy(&gend)).into()
        }
        unsafe extern "system" fn Clone<Impl: ITfRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppclone = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContext<Impl: ITfRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContext() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetText: GetText::<Impl, IMPL_OFFSET>,
            SetText: SetText::<Impl, IMPL_OFFSET>,
            GetFormattedText: GetFormattedText::<Impl, IMPL_OFFSET>,
            GetEmbedded: GetEmbedded::<Impl, IMPL_OFFSET>,
            InsertEmbedded: InsertEmbedded::<Impl, IMPL_OFFSET>,
            ShiftStart: ShiftStart::<Impl, IMPL_OFFSET>,
            ShiftEnd: ShiftEnd::<Impl, IMPL_OFFSET>,
            ShiftStartToRange: ShiftStartToRange::<Impl, IMPL_OFFSET>,
            ShiftEndToRange: ShiftEndToRange::<Impl, IMPL_OFFSET>,
            ShiftStartRegion: ShiftStartRegion::<Impl, IMPL_OFFSET>,
            ShiftEndRegion: ShiftEndRegion::<Impl, IMPL_OFFSET>,
            IsEmpty: IsEmpty::<Impl, IMPL_OFFSET>,
            Collapse: Collapse::<Impl, IMPL_OFFSET>,
            IsEqualStart: IsEqualStart::<Impl, IMPL_OFFSET>,
            IsEqualEnd: IsEqualEnd::<Impl, IMPL_OFFSET>,
            CompareStart: CompareStart::<Impl, IMPL_OFFSET>,
            CompareEnd: CompareEnd::<Impl, IMPL_OFFSET>,
            AdjustForInsert: AdjustForInsert::<Impl, IMPL_OFFSET>,
            GetGravity: GetGravity::<Impl, IMPL_OFFSET>,
            SetGravity: SetGravity::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
            GetContext: GetContext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfRange as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ITfRangeACPImpl: Sized + ITfRangeImpl {
    fn GetExtent(&mut self, pacpanchor: *mut i32, pcch: *mut i32) -> ::windows::core::Result<()>;
    fn SetExtent(&mut self, acpanchor: i32, cch: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ITfRangeACPVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfRangeACPImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfRangeACPVtbl {
        unsafe extern "system" fn GetExtent<Impl: ITfRangeACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pacpanchor: *mut i32, pcch: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetExtent(::core::mem::transmute_copy(&pacpanchor), ::core::mem::transmute_copy(&pcch)).into()
        }
        unsafe extern "system" fn SetExtent<Impl: ITfRangeACPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpanchor: i32, cch: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExtent(::core::mem::transmute_copy(&acpanchor), ::core::mem::transmute_copy(&cch)).into()
        }
        Self {
            base: ITfRangeVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetExtent: GetExtent::<Impl, IMPL_OFFSET>,
            SetExtent: SetExtent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfRangeACP as ::windows::core::Interface>::IID
    }
}
pub trait ITfRangeBackupImpl: Sized {
    fn Restore(&mut self, ec: u32, prange: ::core::option::Option<ITfRange>) -> ::windows::core::Result<()>;
}
impl ITfRangeBackupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfRangeBackupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfRangeBackupVtbl {
        unsafe extern "system" fn Restore<Impl: ITfRangeBackupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Restore(::core::mem::transmute_copy(&ec), ::core::mem::transmute(&prange)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Restore: Restore::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfRangeBackup as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITfReadOnlyPropertyImpl: Sized {
    fn GetType(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn EnumRanges(&mut self, ec: u32, ppenum: *mut ::core::option::Option<IEnumTfRanges>, ptargetrange: ::core::option::Option<ITfRange>) -> ::windows::core::Result<()>;
    fn GetValue(&mut self, ec: u32, prange: ::core::option::Option<ITfRange>) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn GetContext(&mut self) -> ::windows::core::Result<ITfContext>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITfReadOnlyPropertyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfReadOnlyPropertyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfReadOnlyPropertyVtbl {
        unsafe extern "system" fn GetType<Impl: ITfReadOnlyPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetType() {
                ::core::result::Result::Ok(ok__) => {
                    *pguid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumRanges<Impl: ITfReadOnlyPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, ppenum: *mut ::windows::core::RawPtr, ptargetrange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumRanges(::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&ppenum), ::core::mem::transmute(&ptargetrange)).into()
        }
        unsafe extern "system" fn GetValue<Impl: ITfReadOnlyPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, prange: ::windows::core::RawPtr, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetValue(::core::mem::transmute_copy(&ec), ::core::mem::transmute(&prange)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContext<Impl: ITfReadOnlyPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContext() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetType: GetType::<Impl, IMPL_OFFSET>,
            EnumRanges: EnumRanges::<Impl, IMPL_OFFSET>,
            GetValue: GetValue::<Impl, IMPL_OFFSET>,
            GetContext: GetContext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfReadOnlyProperty as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfReadingInformationUIElementImpl: Sized + ITfUIElementImpl {
    fn GetUpdatedFlags(&mut self) -> ::windows::core::Result<u32>;
    fn GetContext(&mut self) -> ::windows::core::Result<ITfContext>;
    fn GetString(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetMaxReadingStringLength(&mut self) -> ::windows::core::Result<u32>;
    fn GetErrorIndex(&mut self) -> ::windows::core::Result<u32>;
    fn IsVerticalOrderPreferred(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfReadingInformationUIElementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfReadingInformationUIElementImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfReadingInformationUIElementVtbl {
        unsafe extern "system" fn GetUpdatedFlags<Impl: ITfReadingInformationUIElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUpdatedFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContext<Impl: ITfReadingInformationUIElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppic: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContext() {
                ::core::result::Result::Ok(ok__) => {
                    *ppic = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetString<Impl: ITfReadingInformationUIElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetString() {
                ::core::result::Result::Ok(ok__) => {
                    *pstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxReadingStringLength<Impl: ITfReadingInformationUIElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcchmax: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxReadingStringLength() {
                ::core::result::Result::Ok(ok__) => {
                    *pcchmax = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorIndex<Impl: ITfReadingInformationUIElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, perrorindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetErrorIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *perrorindex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsVerticalOrderPreferred<Impl: ITfReadingInformationUIElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfvertical: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsVerticalOrderPreferred() {
                ::core::result::Result::Ok(ok__) => {
                    *pfvertical = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ITfUIElementVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetUpdatedFlags: GetUpdatedFlags::<Impl, IMPL_OFFSET>,
            GetContext: GetContext::<Impl, IMPL_OFFSET>,
            GetString: GetString::<Impl, IMPL_OFFSET>,
            GetMaxReadingStringLength: GetMaxReadingStringLength::<Impl, IMPL_OFFSET>,
            GetErrorIndex: GetErrorIndex::<Impl, IMPL_OFFSET>,
            IsVerticalOrderPreferred: IsVerticalOrderPreferred::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfReadingInformationUIElement as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfReverseConversionImpl: Sized {
    fn DoReverseConversion(&mut self, lpstr: super::super::Foundation::PWSTR) -> ::windows::core::Result<ITfReverseConversionList>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfReverseConversionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfReverseConversionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfReverseConversionVtbl {
        unsafe extern "system" fn DoReverseConversion<Impl: ITfReverseConversionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpstr: super::super::Foundation::PWSTR, pplist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DoReverseConversion(::core::mem::transmute_copy(&lpstr)) {
                ::core::result::Result::Ok(ok__) => {
                    *pplist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), DoReverseConversion: DoReverseConversion::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfReverseConversion as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfReverseConversionListImpl: Sized {
    fn GetLength(&mut self) -> ::windows::core::Result<u32>;
    fn GetString(&mut self, uindex: u32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfReverseConversionListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfReverseConversionListImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfReverseConversionListVtbl {
        unsafe extern "system" fn GetLength<Impl: ITfReverseConversionListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLength() {
                ::core::result::Result::Ok(ok__) => {
                    *puindex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetString<Impl: ITfReverseConversionListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, pbstr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetString(::core::mem::transmute_copy(&uindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetLength: GetLength::<Impl, IMPL_OFFSET>,
            GetString: GetString::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfReverseConversionList as ::windows::core::Interface>::IID
    }
}
pub trait ITfReverseConversionMgrImpl: Sized {
    fn GetReverseConversion(&mut self, langid: u16, guidprofile: *const ::windows::core::GUID, dwflag: u32) -> ::windows::core::Result<ITfReverseConversion>;
}
impl ITfReverseConversionMgrVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfReverseConversionMgrImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfReverseConversionMgrVtbl {
        unsafe extern "system" fn GetReverseConversion<Impl: ITfReverseConversionMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16, guidprofile: *const ::windows::core::GUID, dwflag: u32, ppreverseconversion: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetReverseConversion(::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&guidprofile), ::core::mem::transmute_copy(&dwflag)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppreverseconversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetReverseConversion: GetReverseConversion::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfReverseConversionMgr as ::windows::core::Interface>::IID
    }
}
pub trait ITfSourceImpl: Sized {
    fn AdviseSink(&mut self, riid: *const ::windows::core::GUID, punk: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<u32>;
    fn UnadviseSink(&mut self, dwcookie: u32) -> ::windows::core::Result<()>;
}
impl ITfSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfSourceVtbl {
        unsafe extern "system" fn AdviseSink<Impl: ITfSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdviseSink(::core::mem::transmute_copy(&riid), ::core::mem::transmute(&punk)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnadviseSink<Impl: ITfSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnadviseSink(::core::mem::transmute_copy(&dwcookie)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AdviseSink: AdviseSink::<Impl, IMPL_OFFSET>,
            UnadviseSink: UnadviseSink::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfSource as ::windows::core::Interface>::IID
    }
}
pub trait ITfSourceSingleImpl: Sized {
    fn AdviseSingleSink(&mut self, tid: u32, riid: *const ::windows::core::GUID, punk: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn UnadviseSingleSink(&mut self, tid: u32, riid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl ITfSourceSingleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfSourceSingleImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfSourceSingleVtbl {
        unsafe extern "system" fn AdviseSingleSink<Impl: ITfSourceSingleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tid: u32, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AdviseSingleSink(::core::mem::transmute_copy(&tid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute(&punk)).into()
        }
        unsafe extern "system" fn UnadviseSingleSink<Impl: ITfSourceSingleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tid: u32, riid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnadviseSingleSink(::core::mem::transmute_copy(&tid), ::core::mem::transmute_copy(&riid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AdviseSingleSink: AdviseSingleSink::<Impl, IMPL_OFFSET>,
            UnadviseSingleSink: UnadviseSingleSink::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfSourceSingle as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfSpeechUIServerImpl: Sized {
    fn Initialize(&mut self) -> ::windows::core::Result<()>;
    fn ShowUI(&mut self, fshow: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn UpdateBalloon(&mut self, style: TfLBBalloonStyle, pch: super::super::Foundation::PWSTR, cch: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfSpeechUIServerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfSpeechUIServerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfSpeechUIServerVtbl {
        unsafe extern "system" fn Initialize<Impl: ITfSpeechUIServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize().into()
        }
        unsafe extern "system" fn ShowUI<Impl: ITfSpeechUIServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowUI(::core::mem::transmute_copy(&fshow)).into()
        }
        unsafe extern "system" fn UpdateBalloon<Impl: ITfSpeechUIServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, style: TfLBBalloonStyle, pch: super::super::Foundation::PWSTR, cch: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateBalloon(::core::mem::transmute_copy(&style), ::core::mem::transmute_copy(&pch), ::core::mem::transmute_copy(&cch)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            ShowUI: ShowUI::<Impl, IMPL_OFFSET>,
            UpdateBalloon: UpdateBalloon::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfSpeechUIServer as ::windows::core::Interface>::IID
    }
}
pub trait ITfStatusSinkImpl: Sized {
    fn OnStatusChange(&mut self, pic: ::core::option::Option<ITfContext>, dwflags: u32) -> ::windows::core::Result<()>;
}
impl ITfStatusSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfStatusSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfStatusSinkVtbl {
        unsafe extern "system" fn OnStatusChange<Impl: ITfStatusSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnStatusChange(::core::mem::transmute(&pic), ::core::mem::transmute_copy(&dwflags)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnStatusChange: OnStatusChange::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfStatusSink as ::windows::core::Interface>::IID
    }
}
pub trait ITfSystemDeviceTypeLangBarItemImpl: Sized {
    fn SetIconMode(&mut self, dwflags: LANG_BAR_ITEM_ICON_MODE_FLAGS) -> ::windows::core::Result<()>;
    fn GetIconMode(&mut self) -> ::windows::core::Result<u32>;
}
impl ITfSystemDeviceTypeLangBarItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfSystemDeviceTypeLangBarItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfSystemDeviceTypeLangBarItemVtbl {
        unsafe extern "system" fn SetIconMode<Impl: ITfSystemDeviceTypeLangBarItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: LANG_BAR_ITEM_ICON_MODE_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIconMode(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetIconMode<Impl: ITfSystemDeviceTypeLangBarItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIconMode() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetIconMode: SetIconMode::<Impl, IMPL_OFFSET>,
            GetIconMode: GetIconMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfSystemDeviceTypeLangBarItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait ITfSystemLangBarItemImpl: Sized {
    fn SetIcon(&mut self, hicon: super::WindowsAndMessaging::HICON) -> ::windows::core::Result<()>;
    fn SetTooltipString(&mut self, pchtooltip: super::super::Foundation::PWSTR, cch: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ITfSystemLangBarItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfSystemLangBarItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfSystemLangBarItemVtbl {
        unsafe extern "system" fn SetIcon<Impl: ITfSystemLangBarItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hicon: super::WindowsAndMessaging::HICON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIcon(::core::mem::transmute_copy(&hicon)).into()
        }
        unsafe extern "system" fn SetTooltipString<Impl: ITfSystemLangBarItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchtooltip: super::super::Foundation::PWSTR, cch: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTooltipString(::core::mem::transmute_copy(&pchtooltip), ::core::mem::transmute_copy(&cch)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetIcon: SetIcon::<Impl, IMPL_OFFSET>,
            SetTooltipString: SetTooltipString::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfSystemLangBarItem as ::windows::core::Interface>::IID
    }
}
pub trait ITfSystemLangBarItemSinkImpl: Sized {
    fn InitMenu(&mut self, pmenu: ::core::option::Option<ITfMenu>) -> ::windows::core::Result<()>;
    fn OnMenuSelect(&mut self, wid: u32) -> ::windows::core::Result<()>;
}
impl ITfSystemLangBarItemSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfSystemLangBarItemSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfSystemLangBarItemSinkVtbl {
        unsafe extern "system" fn InitMenu<Impl: ITfSystemLangBarItemSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmenu: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitMenu(::core::mem::transmute(&pmenu)).into()
        }
        unsafe extern "system" fn OnMenuSelect<Impl: ITfSystemLangBarItemSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnMenuSelect(::core::mem::transmute_copy(&wid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            InitMenu: InitMenu::<Impl, IMPL_OFFSET>,
            OnMenuSelect: OnMenuSelect::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfSystemLangBarItemSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfSystemLangBarItemTextImpl: Sized {
    fn SetItemText(&mut self, pch: super::super::Foundation::PWSTR, cch: u32) -> ::windows::core::Result<()>;
    fn GetItemText(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfSystemLangBarItemTextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfSystemLangBarItemTextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfSystemLangBarItemTextVtbl {
        unsafe extern "system" fn SetItemText<Impl: ITfSystemLangBarItemTextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pch: super::super::Foundation::PWSTR, cch: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetItemText(::core::mem::transmute_copy(&pch), ::core::mem::transmute_copy(&cch)).into()
        }
        unsafe extern "system" fn GetItemText<Impl: ITfSystemLangBarItemTextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemText() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrtext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetItemText: SetItemText::<Impl, IMPL_OFFSET>,
            GetItemText: GetItemText::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfSystemLangBarItemText as ::windows::core::Interface>::IID
    }
}
pub trait ITfTextEditSinkImpl: Sized {
    fn OnEndEdit(&mut self, pic: ::core::option::Option<ITfContext>, ecreadonly: u32, peditrecord: ::core::option::Option<ITfEditRecord>) -> ::windows::core::Result<()>;
}
impl ITfTextEditSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfTextEditSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfTextEditSinkVtbl {
        unsafe extern "system" fn OnEndEdit<Impl: ITfTextEditSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr, ecreadonly: u32, peditrecord: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnEndEdit(::core::mem::transmute(&pic), ::core::mem::transmute_copy(&ecreadonly), ::core::mem::transmute(&peditrecord)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnEndEdit: OnEndEdit::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfTextEditSink as ::windows::core::Interface>::IID
    }
}
pub trait ITfTextInputProcessorImpl: Sized {
    fn Activate(&mut self, ptim: ::core::option::Option<ITfThreadMgr>, tid: u32) -> ::windows::core::Result<()>;
    fn Deactivate(&mut self) -> ::windows::core::Result<()>;
}
impl ITfTextInputProcessorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfTextInputProcessorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfTextInputProcessorVtbl {
        unsafe extern "system" fn Activate<Impl: ITfTextInputProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptim: ::windows::core::RawPtr, tid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Activate(::core::mem::transmute(&ptim), ::core::mem::transmute_copy(&tid)).into()
        }
        unsafe extern "system" fn Deactivate<Impl: ITfTextInputProcessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Deactivate().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Activate: Activate::<Impl, IMPL_OFFSET>,
            Deactivate: Deactivate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfTextInputProcessor as ::windows::core::Interface>::IID
    }
}
pub trait ITfTextInputProcessorExImpl: Sized + ITfTextInputProcessorImpl {
    fn ActivateEx(&mut self, ptim: ::core::option::Option<ITfThreadMgr>, tid: u32, dwflags: u32) -> ::windows::core::Result<()>;
}
impl ITfTextInputProcessorExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfTextInputProcessorExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfTextInputProcessorExVtbl {
        unsafe extern "system" fn ActivateEx<Impl: ITfTextInputProcessorExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptim: ::windows::core::RawPtr, tid: u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ActivateEx(::core::mem::transmute(&ptim), ::core::mem::transmute_copy(&tid), ::core::mem::transmute_copy(&dwflags)).into()
        }
        Self { base: ITfTextInputProcessorVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), ActivateEx: ActivateEx::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfTextInputProcessorEx as ::windows::core::Interface>::IID
    }
}
pub trait ITfTextLayoutSinkImpl: Sized {
    fn OnLayoutChange(&mut self, pic: ::core::option::Option<ITfContext>, lcode: TfLayoutCode, pview: ::core::option::Option<ITfContextView>) -> ::windows::core::Result<()>;
}
impl ITfTextLayoutSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfTextLayoutSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfTextLayoutSinkVtbl {
        unsafe extern "system" fn OnLayoutChange<Impl: ITfTextLayoutSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr, lcode: TfLayoutCode, pview: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnLayoutChange(::core::mem::transmute(&pic), ::core::mem::transmute_copy(&lcode), ::core::mem::transmute(&pview)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnLayoutChange: OnLayoutChange::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfTextLayoutSink as ::windows::core::Interface>::IID
    }
}
pub trait ITfThreadFocusSinkImpl: Sized {
    fn OnSetThreadFocus(&mut self) -> ::windows::core::Result<()>;
    fn OnKillThreadFocus(&mut self) -> ::windows::core::Result<()>;
}
impl ITfThreadFocusSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfThreadFocusSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfThreadFocusSinkVtbl {
        unsafe extern "system" fn OnSetThreadFocus<Impl: ITfThreadFocusSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnSetThreadFocus().into()
        }
        unsafe extern "system" fn OnKillThreadFocus<Impl: ITfThreadFocusSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnKillThreadFocus().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnSetThreadFocus: OnSetThreadFocus::<Impl, IMPL_OFFSET>,
            OnKillThreadFocus: OnKillThreadFocus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfThreadFocusSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfThreadMgrImpl: Sized {
    fn Activate(&mut self) -> ::windows::core::Result<u32>;
    fn Deactivate(&mut self) -> ::windows::core::Result<()>;
    fn CreateDocumentMgr(&mut self) -> ::windows::core::Result<ITfDocumentMgr>;
    fn EnumDocumentMgrs(&mut self) -> ::windows::core::Result<IEnumTfDocumentMgrs>;
    fn GetFocus(&mut self) -> ::windows::core::Result<ITfDocumentMgr>;
    fn SetFocus(&mut self, pdimfocus: ::core::option::Option<ITfDocumentMgr>) -> ::windows::core::Result<()>;
    fn AssociateFocus(&mut self, hwnd: super::super::Foundation::HWND, pdimnew: ::core::option::Option<ITfDocumentMgr>) -> ::windows::core::Result<ITfDocumentMgr>;
    fn IsThreadFocus(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetFunctionProvider(&mut self, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<ITfFunctionProvider>;
    fn EnumFunctionProviders(&mut self) -> ::windows::core::Result<IEnumTfFunctionProviders>;
    fn GetGlobalCompartment(&mut self) -> ::windows::core::Result<ITfCompartmentMgr>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfThreadMgrVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfThreadMgrImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfThreadMgrVtbl {
        unsafe extern "system" fn Activate<Impl: ITfThreadMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Activate() {
                ::core::result::Result::Ok(ok__) => {
                    *ptid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Deactivate<Impl: ITfThreadMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Deactivate().into()
        }
        unsafe extern "system" fn CreateDocumentMgr<Impl: ITfThreadMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdim: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDocumentMgr() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdim = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDocumentMgrs<Impl: ITfThreadMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumDocumentMgrs() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFocus<Impl: ITfThreadMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdimfocus: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFocus() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdimfocus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFocus<Impl: ITfThreadMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdimfocus: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFocus(::core::mem::transmute(&pdimfocus)).into()
        }
        unsafe extern "system" fn AssociateFocus<Impl: ITfThreadMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pdimnew: ::windows::core::RawPtr, ppdimprev: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AssociateFocus(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute(&pdimnew)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdimprev = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsThreadFocus<Impl: ITfThreadMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfthreadfocus: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsThreadFocus() {
                ::core::result::Result::Ok(ok__) => {
                    *pfthreadfocus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunctionProvider<Impl: ITfThreadMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, ppfuncprov: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFunctionProvider(::core::mem::transmute_copy(&clsid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppfuncprov = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumFunctionProviders<Impl: ITfThreadMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumFunctionProviders() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlobalCompartment<Impl: ITfThreadMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcompmgr: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGlobalCompartment() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcompmgr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Activate: Activate::<Impl, IMPL_OFFSET>,
            Deactivate: Deactivate::<Impl, IMPL_OFFSET>,
            CreateDocumentMgr: CreateDocumentMgr::<Impl, IMPL_OFFSET>,
            EnumDocumentMgrs: EnumDocumentMgrs::<Impl, IMPL_OFFSET>,
            GetFocus: GetFocus::<Impl, IMPL_OFFSET>,
            SetFocus: SetFocus::<Impl, IMPL_OFFSET>,
            AssociateFocus: AssociateFocus::<Impl, IMPL_OFFSET>,
            IsThreadFocus: IsThreadFocus::<Impl, IMPL_OFFSET>,
            GetFunctionProvider: GetFunctionProvider::<Impl, IMPL_OFFSET>,
            EnumFunctionProviders: EnumFunctionProviders::<Impl, IMPL_OFFSET>,
            GetGlobalCompartment: GetGlobalCompartment::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfThreadMgr as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfThreadMgr2Impl: Sized {
    fn Activate(&mut self) -> ::windows::core::Result<u32>;
    fn Deactivate(&mut self) -> ::windows::core::Result<()>;
    fn CreateDocumentMgr(&mut self) -> ::windows::core::Result<ITfDocumentMgr>;
    fn EnumDocumentMgrs(&mut self) -> ::windows::core::Result<IEnumTfDocumentMgrs>;
    fn GetFocus(&mut self) -> ::windows::core::Result<ITfDocumentMgr>;
    fn SetFocus(&mut self, pdimfocus: ::core::option::Option<ITfDocumentMgr>) -> ::windows::core::Result<()>;
    fn IsThreadFocus(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetFunctionProvider(&mut self, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<ITfFunctionProvider>;
    fn EnumFunctionProviders(&mut self) -> ::windows::core::Result<IEnumTfFunctionProviders>;
    fn GetGlobalCompartment(&mut self) -> ::windows::core::Result<ITfCompartmentMgr>;
    fn ActivateEx(&mut self, ptid: *mut u32, dwflags: u32) -> ::windows::core::Result<()>;
    fn GetActiveFlags(&mut self) -> ::windows::core::Result<u32>;
    fn SuspendKeystrokeHandling(&mut self) -> ::windows::core::Result<()>;
    fn ResumeKeystrokeHandling(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfThreadMgr2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfThreadMgr2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfThreadMgr2Vtbl {
        unsafe extern "system" fn Activate<Impl: ITfThreadMgr2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Activate() {
                ::core::result::Result::Ok(ok__) => {
                    *ptid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Deactivate<Impl: ITfThreadMgr2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Deactivate().into()
        }
        unsafe extern "system" fn CreateDocumentMgr<Impl: ITfThreadMgr2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdim: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDocumentMgr() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdim = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDocumentMgrs<Impl: ITfThreadMgr2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumDocumentMgrs() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFocus<Impl: ITfThreadMgr2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdimfocus: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFocus() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdimfocus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFocus<Impl: ITfThreadMgr2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdimfocus: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFocus(::core::mem::transmute(&pdimfocus)).into()
        }
        unsafe extern "system" fn IsThreadFocus<Impl: ITfThreadMgr2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfthreadfocus: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsThreadFocus() {
                ::core::result::Result::Ok(ok__) => {
                    *pfthreadfocus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunctionProvider<Impl: ITfThreadMgr2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, ppfuncprov: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFunctionProvider(::core::mem::transmute_copy(&clsid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppfuncprov = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumFunctionProviders<Impl: ITfThreadMgr2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumFunctionProviders() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlobalCompartment<Impl: ITfThreadMgr2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcompmgr: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGlobalCompartment() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcompmgr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivateEx<Impl: ITfThreadMgr2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptid: *mut u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ActivateEx(::core::mem::transmute_copy(&ptid), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetActiveFlags<Impl: ITfThreadMgr2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActiveFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *lpdwflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SuspendKeystrokeHandling<Impl: ITfThreadMgr2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SuspendKeystrokeHandling().into()
        }
        unsafe extern "system" fn ResumeKeystrokeHandling<Impl: ITfThreadMgr2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResumeKeystrokeHandling().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Activate: Activate::<Impl, IMPL_OFFSET>,
            Deactivate: Deactivate::<Impl, IMPL_OFFSET>,
            CreateDocumentMgr: CreateDocumentMgr::<Impl, IMPL_OFFSET>,
            EnumDocumentMgrs: EnumDocumentMgrs::<Impl, IMPL_OFFSET>,
            GetFocus: GetFocus::<Impl, IMPL_OFFSET>,
            SetFocus: SetFocus::<Impl, IMPL_OFFSET>,
            IsThreadFocus: IsThreadFocus::<Impl, IMPL_OFFSET>,
            GetFunctionProvider: GetFunctionProvider::<Impl, IMPL_OFFSET>,
            EnumFunctionProviders: EnumFunctionProviders::<Impl, IMPL_OFFSET>,
            GetGlobalCompartment: GetGlobalCompartment::<Impl, IMPL_OFFSET>,
            ActivateEx: ActivateEx::<Impl, IMPL_OFFSET>,
            GetActiveFlags: GetActiveFlags::<Impl, IMPL_OFFSET>,
            SuspendKeystrokeHandling: SuspendKeystrokeHandling::<Impl, IMPL_OFFSET>,
            ResumeKeystrokeHandling: ResumeKeystrokeHandling::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfThreadMgr2 as ::windows::core::Interface>::IID
    }
}
pub trait ITfThreadMgrEventSinkImpl: Sized {
    fn OnInitDocumentMgr(&mut self, pdim: ::core::option::Option<ITfDocumentMgr>) -> ::windows::core::Result<()>;
    fn OnUninitDocumentMgr(&mut self, pdim: ::core::option::Option<ITfDocumentMgr>) -> ::windows::core::Result<()>;
    fn OnSetFocus(&mut self, pdimfocus: ::core::option::Option<ITfDocumentMgr>, pdimprevfocus: ::core::option::Option<ITfDocumentMgr>) -> ::windows::core::Result<()>;
    fn OnPushContext(&mut self, pic: ::core::option::Option<ITfContext>) -> ::windows::core::Result<()>;
    fn OnPopContext(&mut self, pic: ::core::option::Option<ITfContext>) -> ::windows::core::Result<()>;
}
impl ITfThreadMgrEventSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfThreadMgrEventSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfThreadMgrEventSinkVtbl {
        unsafe extern "system" fn OnInitDocumentMgr<Impl: ITfThreadMgrEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdim: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnInitDocumentMgr(::core::mem::transmute(&pdim)).into()
        }
        unsafe extern "system" fn OnUninitDocumentMgr<Impl: ITfThreadMgrEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdim: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnUninitDocumentMgr(::core::mem::transmute(&pdim)).into()
        }
        unsafe extern "system" fn OnSetFocus<Impl: ITfThreadMgrEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdimfocus: ::windows::core::RawPtr, pdimprevfocus: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnSetFocus(::core::mem::transmute(&pdimfocus), ::core::mem::transmute(&pdimprevfocus)).into()
        }
        unsafe extern "system" fn OnPushContext<Impl: ITfThreadMgrEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnPushContext(::core::mem::transmute(&pic)).into()
        }
        unsafe extern "system" fn OnPopContext<Impl: ITfThreadMgrEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnPopContext(::core::mem::transmute(&pic)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnInitDocumentMgr: OnInitDocumentMgr::<Impl, IMPL_OFFSET>,
            OnUninitDocumentMgr: OnUninitDocumentMgr::<Impl, IMPL_OFFSET>,
            OnSetFocus: OnSetFocus::<Impl, IMPL_OFFSET>,
            OnPushContext: OnPushContext::<Impl, IMPL_OFFSET>,
            OnPopContext: OnPopContext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfThreadMgrEventSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfThreadMgrExImpl: Sized + ITfThreadMgrImpl {
    fn ActivateEx(&mut self, ptid: *mut u32, dwflags: u32) -> ::windows::core::Result<()>;
    fn GetActiveFlags(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfThreadMgrExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfThreadMgrExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfThreadMgrExVtbl {
        unsafe extern "system" fn ActivateEx<Impl: ITfThreadMgrExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptid: *mut u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ActivateEx(::core::mem::transmute_copy(&ptid), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetActiveFlags<Impl: ITfThreadMgrExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActiveFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *lpdwflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ITfThreadMgrVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ActivateEx: ActivateEx::<Impl, IMPL_OFFSET>,
            GetActiveFlags: GetActiveFlags::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfThreadMgrEx as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfToolTipUIElementImpl: Sized + ITfUIElementImpl {
    fn GetString(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfToolTipUIElementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfToolTipUIElementImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfToolTipUIElementVtbl {
        unsafe extern "system" fn GetString<Impl: ITfToolTipUIElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetString() {
                ::core::result::Result::Ok(ok__) => {
                    *pstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ITfUIElementVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetString: GetString::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfToolTipUIElement as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfTransitoryExtensionSinkImpl: Sized {
    fn OnTransitoryExtensionUpdated(&mut self, pic: ::core::option::Option<ITfContext>, ecreadonly: u32, presultrange: ::core::option::Option<ITfRange>, pcompositionrange: ::core::option::Option<ITfRange>) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfTransitoryExtensionSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfTransitoryExtensionSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfTransitoryExtensionSinkVtbl {
        unsafe extern "system" fn OnTransitoryExtensionUpdated<Impl: ITfTransitoryExtensionSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr, ecreadonly: u32, presultrange: ::windows::core::RawPtr, pcompositionrange: ::windows::core::RawPtr, pfdeleteresultrange: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnTransitoryExtensionUpdated(::core::mem::transmute(&pic), ::core::mem::transmute_copy(&ecreadonly), ::core::mem::transmute(&presultrange), ::core::mem::transmute(&pcompositionrange)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfdeleteresultrange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnTransitoryExtensionUpdated: OnTransitoryExtensionUpdated::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfTransitoryExtensionSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfTransitoryExtensionUIElementImpl: Sized + ITfUIElementImpl {
    fn GetDocumentMgr(&mut self) -> ::windows::core::Result<ITfDocumentMgr>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfTransitoryExtensionUIElementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfTransitoryExtensionUIElementImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfTransitoryExtensionUIElementVtbl {
        unsafe extern "system" fn GetDocumentMgr<Impl: ITfTransitoryExtensionUIElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdim: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocumentMgr() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdim = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ITfUIElementVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDocumentMgr: GetDocumentMgr::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfTransitoryExtensionUIElement as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfUIElementImpl: Sized {
    fn GetDescription(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetGUID(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Show(&mut self, bshow: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn IsShown(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfUIElementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfUIElementImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfUIElementVtbl {
        unsafe extern "system" fn GetDescription<Impl: ITfUIElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGUID<Impl: ITfUIElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGUID() {
                ::core::result::Result::Ok(ok__) => {
                    *pguid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Show<Impl: ITfUIElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Show(::core::mem::transmute_copy(&bshow)).into()
        }
        unsafe extern "system" fn IsShown<Impl: ITfUIElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbshow: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsShown() {
                ::core::result::Result::Ok(ok__) => {
                    *pbshow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetDescription: GetDescription::<Impl, IMPL_OFFSET>,
            GetGUID: GetGUID::<Impl, IMPL_OFFSET>,
            Show: Show::<Impl, IMPL_OFFSET>,
            IsShown: IsShown::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfUIElement as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfUIElementMgrImpl: Sized {
    fn BeginUIElement(&mut self, pelement: ::core::option::Option<ITfUIElement>, pbshow: *mut super::super::Foundation::BOOL, pdwuielementid: *mut u32) -> ::windows::core::Result<()>;
    fn UpdateUIElement(&mut self, dwuielementid: u32) -> ::windows::core::Result<()>;
    fn EndUIElement(&mut self, dwuielementid: u32) -> ::windows::core::Result<()>;
    fn GetUIElement(&mut self, dwuielementid: u32) -> ::windows::core::Result<ITfUIElement>;
    fn EnumUIElements(&mut self) -> ::windows::core::Result<IEnumTfUIElements>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfUIElementMgrVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfUIElementMgrImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfUIElementMgrVtbl {
        unsafe extern "system" fn BeginUIElement<Impl: ITfUIElementMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pelement: ::windows::core::RawPtr, pbshow: *mut super::super::Foundation::BOOL, pdwuielementid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginUIElement(::core::mem::transmute(&pelement), ::core::mem::transmute_copy(&pbshow), ::core::mem::transmute_copy(&pdwuielementid)).into()
        }
        unsafe extern "system" fn UpdateUIElement<Impl: ITfUIElementMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwuielementid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateUIElement(::core::mem::transmute_copy(&dwuielementid)).into()
        }
        unsafe extern "system" fn EndUIElement<Impl: ITfUIElementMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwuielementid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndUIElement(::core::mem::transmute_copy(&dwuielementid)).into()
        }
        unsafe extern "system" fn GetUIElement<Impl: ITfUIElementMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwuielementid: u32, ppelement: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUIElement(::core::mem::transmute_copy(&dwuielementid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppelement = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumUIElements<Impl: ITfUIElementMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumUIElements() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            BeginUIElement: BeginUIElement::<Impl, IMPL_OFFSET>,
            UpdateUIElement: UpdateUIElement::<Impl, IMPL_OFFSET>,
            EndUIElement: EndUIElement::<Impl, IMPL_OFFSET>,
            GetUIElement: GetUIElement::<Impl, IMPL_OFFSET>,
            EnumUIElements: EnumUIElements::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfUIElementMgr as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfUIElementSinkImpl: Sized {
    fn BeginUIElement(&mut self, dwuielementid: u32, pbshow: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn UpdateUIElement(&mut self, dwuielementid: u32) -> ::windows::core::Result<()>;
    fn EndUIElement(&mut self, dwuielementid: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfUIElementSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfUIElementSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfUIElementSinkVtbl {
        unsafe extern "system" fn BeginUIElement<Impl: ITfUIElementSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwuielementid: u32, pbshow: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginUIElement(::core::mem::transmute_copy(&dwuielementid), ::core::mem::transmute_copy(&pbshow)).into()
        }
        unsafe extern "system" fn UpdateUIElement<Impl: ITfUIElementSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwuielementid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateUIElement(::core::mem::transmute_copy(&dwuielementid)).into()
        }
        unsafe extern "system" fn EndUIElement<Impl: ITfUIElementSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwuielementid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndUIElement(::core::mem::transmute_copy(&dwuielementid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            BeginUIElement: BeginUIElement::<Impl, IMPL_OFFSET>,
            UpdateUIElement: UpdateUIElement::<Impl, IMPL_OFFSET>,
            EndUIElement: EndUIElement::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfUIElementSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUIManagerEventSinkImpl: Sized {
    fn OnWindowOpening(&mut self, prcbounds: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn OnWindowOpened(&mut self, prcbounds: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn OnWindowUpdating(&mut self, prcupdatedbounds: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn OnWindowUpdated(&mut self, prcupdatedbounds: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn OnWindowClosing(&mut self) -> ::windows::core::Result<()>;
    fn OnWindowClosed(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IUIManagerEventSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIManagerEventSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIManagerEventSinkVtbl {
        unsafe extern "system" fn OnWindowOpening<Impl: IUIManagerEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcbounds: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnWindowOpening(::core::mem::transmute_copy(&prcbounds)).into()
        }
        unsafe extern "system" fn OnWindowOpened<Impl: IUIManagerEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcbounds: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnWindowOpened(::core::mem::transmute_copy(&prcbounds)).into()
        }
        unsafe extern "system" fn OnWindowUpdating<Impl: IUIManagerEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcupdatedbounds: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnWindowUpdating(::core::mem::transmute_copy(&prcupdatedbounds)).into()
        }
        unsafe extern "system" fn OnWindowUpdated<Impl: IUIManagerEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcupdatedbounds: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnWindowUpdated(::core::mem::transmute_copy(&prcupdatedbounds)).into()
        }
        unsafe extern "system" fn OnWindowClosing<Impl: IUIManagerEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnWindowClosing().into()
        }
        unsafe extern "system" fn OnWindowClosed<Impl: IUIManagerEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnWindowClosed().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnWindowOpening: OnWindowOpening::<Impl, IMPL_OFFSET>,
            OnWindowOpened: OnWindowOpened::<Impl, IMPL_OFFSET>,
            OnWindowUpdating: OnWindowUpdating::<Impl, IMPL_OFFSET>,
            OnWindowUpdated: OnWindowUpdated::<Impl, IMPL_OFFSET>,
            OnWindowClosing: OnWindowClosing::<Impl, IMPL_OFFSET>,
            OnWindowClosed: OnWindowClosed::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIManagerEventSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVersionInfoImpl: Sized {
    fn GetSubcomponentCount(&mut self, ulsub: u32) -> ::windows::core::Result<u32>;
    fn GetImplementationID(&mut self, ulsub: u32) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetBuildVersion(&mut self, ulsub: u32, pdwmajor: *mut u32, pdwminor: *mut u32) -> ::windows::core::Result<()>;
    fn GetComponentDescription(&mut self, ulsub: u32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetInstanceDescription(&mut self, ulsub: u32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVersionInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVersionInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVersionInfoVtbl {
        unsafe extern "system" fn GetSubcomponentCount<Impl: IVersionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulsub: u32, ulcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSubcomponentCount(::core::mem::transmute_copy(&ulsub)) {
                ::core::result::Result::Ok(ok__) => {
                    *ulcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImplementationID<Impl: IVersionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulsub: u32, implid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetImplementationID(::core::mem::transmute_copy(&ulsub)) {
                ::core::result::Result::Ok(ok__) => {
                    *implid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBuildVersion<Impl: IVersionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulsub: u32, pdwmajor: *mut u32, pdwminor: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBuildVersion(::core::mem::transmute_copy(&ulsub), ::core::mem::transmute_copy(&pdwmajor), ::core::mem::transmute_copy(&pdwminor)).into()
        }
        unsafe extern "system" fn GetComponentDescription<Impl: IVersionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulsub: u32, pimplstr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetComponentDescription(::core::mem::transmute_copy(&ulsub)) {
                ::core::result::Result::Ok(ok__) => {
                    *pimplstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInstanceDescription<Impl: IVersionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulsub: u32, pimplstr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInstanceDescription(::core::mem::transmute_copy(&ulsub)) {
                ::core::result::Result::Ok(ok__) => {
                    *pimplstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetSubcomponentCount: GetSubcomponentCount::<Impl, IMPL_OFFSET>,
            GetImplementationID: GetImplementationID::<Impl, IMPL_OFFSET>,
            GetBuildVersion: GetBuildVersion::<Impl, IMPL_OFFSET>,
            GetComponentDescription: GetComponentDescription::<Impl, IMPL_OFFSET>,
            GetInstanceDescription: GetInstanceDescription::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVersionInfo as ::windows::core::Interface>::IID
    }
}
