#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAccClientDocMgr_Impl: Sized {
    fn GetDocuments(&mut self) -> ::windows::core::Result<super::super::System::Com::IEnumUnknown>;
    fn LookupByHWND(&mut self, hwnd: super::super::Foundation::HWND, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn LookupByPoint(&mut self, pt: super::super::Foundation::POINT, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetFocused(&mut self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAccClientDocMgr_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccClientDocMgr_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccClientDocMgr_Vtbl {
        unsafe extern "system" fn GetDocuments<Impl: IAccClientDocMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumunknown: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocuments() {
                ::core::result::Result::Ok(ok__) => {
                    *enumunknown = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupByHWND<Impl: IAccClientDocMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LookupByHWND(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupByPoint<Impl: IAccClientDocMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pt: super::super::Foundation::POINT, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LookupByPoint(::core::mem::transmute_copy(&pt), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFocused<Impl: IAccClientDocMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IAccDictionary_Impl: Sized {
    fn GetLocalizedString(&mut self, term: *const ::windows::core::GUID, lcid: u32, presult: *mut super::super::Foundation::BSTR, plcid: *mut u32) -> ::windows::core::Result<()>;
    fn GetParentTerm(&mut self, term: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetMnemonicString(&mut self, term: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn LookupMnemonicTerm(&mut self, bstrmnemonic: super::super::Foundation::BSTR) -> ::windows::core::Result<::windows::core::GUID>;
    fn ConvertValueToString(&mut self, term: *const ::windows::core::GUID, lcid: u32, varvalue: super::super::System::Com::VARIANT, pbstrresult: *mut super::super::Foundation::BSTR, plcid: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAccDictionary_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccDictionary_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccDictionary_Vtbl {
        unsafe extern "system" fn GetLocalizedString<Impl: IAccDictionary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, term: *const ::windows::core::GUID, lcid: u32, presult: *mut super::super::Foundation::BSTR, plcid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLocalizedString(::core::mem::transmute_copy(&term), ::core::mem::transmute_copy(&lcid), ::core::mem::transmute_copy(&presult), ::core::mem::transmute_copy(&plcid)).into()
        }
        unsafe extern "system" fn GetParentTerm<Impl: IAccDictionary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, term: *const ::windows::core::GUID, pparentterm: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetParentTerm(::core::mem::transmute_copy(&term)) {
                ::core::result::Result::Ok(ok__) => {
                    *pparentterm = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMnemonicString<Impl: IAccDictionary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, term: *const ::windows::core::GUID, presult: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMnemonicString(::core::mem::transmute_copy(&term)) {
                ::core::result::Result::Ok(ok__) => {
                    *presult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupMnemonicTerm<Impl: IAccDictionary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmnemonic: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pterm: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LookupMnemonicTerm(::core::mem::transmute_copy(&bstrmnemonic)) {
                ::core::result::Result::Ok(ok__) => {
                    *pterm = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertValueToString<Impl: IAccDictionary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, term: *const ::windows::core::GUID, lcid: u32, varvalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pbstrresult: *mut super::super::Foundation::BSTR, plcid: *mut u32) -> ::windows::core::HRESULT {
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
pub trait IAccServerDocMgr_Impl: Sized {
    fn NewDocument(&mut self, riid: *const ::windows::core::GUID, punk: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn RevokeDocument(&mut self, punk: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn OnDocumentFocus(&mut self, punk: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
impl IAccServerDocMgr_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccServerDocMgr_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccServerDocMgr_Vtbl {
        unsafe extern "system" fn NewDocument<Impl: IAccServerDocMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NewDocument(::core::mem::transmute_copy(&riid), ::core::mem::transmute(&punk)).into()
        }
        unsafe extern "system" fn RevokeDocument<Impl: IAccServerDocMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RevokeDocument(::core::mem::transmute(&punk)).into()
        }
        unsafe extern "system" fn OnDocumentFocus<Impl: IAccServerDocMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IAccStore_Impl: Sized {
    fn Register(&mut self, riid: *const ::windows::core::GUID, punk: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn Unregister(&mut self, punk: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetDocuments(&mut self) -> ::windows::core::Result<super::super::System::Com::IEnumUnknown>;
    fn LookupByHWND(&mut self, hwnd: super::super::Foundation::HWND, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn LookupByPoint(&mut self, pt: super::super::Foundation::POINT, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn OnDocumentFocus(&mut self, punk: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetFocused(&mut self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAccStore_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccStore_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccStore_Vtbl {
        unsafe extern "system" fn Register<Impl: IAccStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Register(::core::mem::transmute_copy(&riid), ::core::mem::transmute(&punk)).into()
        }
        unsafe extern "system" fn Unregister<Impl: IAccStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unregister(::core::mem::transmute(&punk)).into()
        }
        unsafe extern "system" fn GetDocuments<Impl: IAccStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumunknown: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocuments() {
                ::core::result::Result::Ok(ok__) => {
                    *enumunknown = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupByHWND<Impl: IAccStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LookupByHWND(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupByPoint<Impl: IAccStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pt: super::super::Foundation::POINT, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LookupByPoint(::core::mem::transmute_copy(&pt), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnDocumentFocus<Impl: IAccStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDocumentFocus(::core::mem::transmute(&punk)).into()
        }
        unsafe extern "system" fn GetFocused<Impl: IAccStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IAnchor_Impl: Sized {
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
impl IAnchor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAnchor_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAnchor_Vtbl {
        unsafe extern "system" fn SetGravity<Impl: IAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gravity: TsGravity) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGravity(::core::mem::transmute_copy(&gravity)).into()
        }
        unsafe extern "system" fn GetGravity<Impl: IAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgravity: *mut TsGravity) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGravity() {
                ::core::result::Result::Ok(ok__) => {
                    *pgravity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Impl: IAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pawith: ::windows::core::RawPtr, pfequal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEqual(::core::mem::transmute(&pawith)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfequal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Compare<Impl: IAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pawith: ::windows::core::RawPtr, plresult: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Compare(::core::mem::transmute(&pawith)) {
                ::core::result::Result::Ok(ok__) => {
                    *plresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shift<Impl: IAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, cchreq: i32, pcch: *mut i32, pahaltanchor: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Shift(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&cchreq), ::core::mem::transmute_copy(&pcch), ::core::mem::transmute(&pahaltanchor)).into()
        }
        unsafe extern "system" fn ShiftTo<Impl: IAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pasite: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShiftTo(::core::mem::transmute(&pasite)).into()
        }
        unsafe extern "system" fn ShiftRegion<Impl: IAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, dir: TsShiftDir, pfnoregion: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShiftRegion(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dir)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfnoregion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChangeHistoryMask<Impl: IAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetChangeHistoryMask(::core::mem::transmute_copy(&dwmask)).into()
        }
        unsafe extern "system" fn GetChangeHistory<Impl: IAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwhistory: *mut ANCHOR_CHANGE_HISTORY_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChangeHistory() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwhistory = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearChangeHistory<Impl: IAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearChangeHistory().into()
        }
        unsafe extern "system" fn Clone<Impl: IAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaclone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IClonableWrapper_Impl: Sized {
    fn CloneNewWrapper(&mut self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IClonableWrapper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClonableWrapper_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IClonableWrapper_Vtbl {
        unsafe extern "system" fn CloneNewWrapper<Impl: IClonableWrapper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait ICoCreateLocally_Impl: Sized {
    fn CoCreateLocally(&mut self, rclsid: *const ::windows::core::GUID, dwclscontext: u32, riid: *const ::windows::core::GUID, punk: *mut ::core::option::Option<::windows::core::IUnknown>, riidparam: *const ::windows::core::GUID, punkparam: ::core::option::Option<::windows::core::IUnknown>, varparam: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICoCreateLocally_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoCreateLocally_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoCreateLocally_Vtbl {
        unsafe extern "system" fn CoCreateLocally<Impl: ICoCreateLocally_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, dwclscontext: u32, riid: *const ::windows::core::GUID, punk: *mut *mut ::core::ffi::c_void, riidparam: *const ::windows::core::GUID, punkparam: *mut ::core::ffi::c_void, varparam: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
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
pub trait ICoCreatedLocally_Impl: Sized {
    fn LocalInit(&mut self, punklocalobject: ::core::option::Option<::windows::core::IUnknown>, riidparam: *const ::windows::core::GUID, punkparam: ::core::option::Option<::windows::core::IUnknown>, varparam: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICoCreatedLocally_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoCreatedLocally_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoCreatedLocally_Vtbl {
        unsafe extern "system" fn LocalInit<Impl: ICoCreatedLocally_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punklocalobject: *mut ::core::ffi::c_void, riidparam: *const ::windows::core::GUID, punkparam: *mut ::core::ffi::c_void, varparam: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LocalInit(::core::mem::transmute(&punklocalobject), ::core::mem::transmute_copy(&riidparam), ::core::mem::transmute(&punkparam), ::core::mem::transmute_copy(&varparam)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), LocalInit: LocalInit::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoCreatedLocally as ::windows::core::Interface>::IID
    }
}
pub trait IDocWrap_Impl: Sized {
    fn SetDoc(&mut self, riid: *const ::windows::core::GUID, punk: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetWrappedDoc(&mut self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
}
impl IDocWrap_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDocWrap_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDocWrap_Vtbl {
        unsafe extern "system" fn SetDoc<Impl: IDocWrap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDoc(::core::mem::transmute_copy(&riid), ::core::mem::transmute(&punk)).into()
        }
        unsafe extern "system" fn GetWrappedDoc<Impl: IDocWrap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IEnumITfCompositionView_Impl: Sized {
    fn Clone(&mut self) -> ::windows::core::Result<IEnumITfCompositionView>;
    fn Next(&mut self, ulcount: u32, rgcompositionview: *mut ::core::option::Option<ITfCompositionView>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, ulcount: u32) -> ::windows::core::Result<()>;
}
impl IEnumITfCompositionView_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumITfCompositionView_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumITfCompositionView_Vtbl {
        unsafe extern "system" fn Clone<Impl: IEnumITfCompositionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumITfCompositionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, rgcompositionview: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&rgcompositionview), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumITfCompositionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumITfCompositionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
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
pub trait IEnumSpeechCommands_Impl: Sized {
    fn Clone(&mut self) -> ::windows::core::Result<IEnumSpeechCommands>;
    fn Next(&mut self, ulcount: u32, pspcmds: *mut *mut u16, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, ulcount: u32) -> ::windows::core::Result<()>;
}
impl IEnumSpeechCommands_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSpeechCommands_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumSpeechCommands_Vtbl {
        unsafe extern "system" fn Clone<Impl: IEnumSpeechCommands_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumSpeechCommands_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, pspcmds: *mut *mut u16, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pspcmds), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumSpeechCommands_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumSpeechCommands_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
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
pub trait IEnumTfCandidates_Impl: Sized {
    fn Clone(&mut self) -> ::windows::core::Result<IEnumTfCandidates>;
    fn Next(&mut self, ulcount: u32, ppcand: *mut ::core::option::Option<ITfCandidateString>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, ulcount: u32) -> ::windows::core::Result<()>;
}
impl IEnumTfCandidates_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTfCandidates_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumTfCandidates_Vtbl {
        unsafe extern "system" fn Clone<Impl: IEnumTfCandidates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumTfCandidates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, ppcand: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&ppcand), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumTfCandidates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumTfCandidates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
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
pub trait IEnumTfContextViews_Impl: Sized {
    fn Clone(&mut self) -> ::windows::core::Result<IEnumTfContextViews>;
    fn Next(&mut self, ulcount: u32, rgviews: *mut ::core::option::Option<ITfContextView>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, ulcount: u32) -> ::windows::core::Result<()>;
}
impl IEnumTfContextViews_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTfContextViews_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumTfContextViews_Vtbl {
        unsafe extern "system" fn Clone<Impl: IEnumTfContextViews_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumTfContextViews_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, rgviews: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&rgviews), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumTfContextViews_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumTfContextViews_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
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
pub trait IEnumTfContexts_Impl: Sized {
    fn Clone(&mut self) -> ::windows::core::Result<IEnumTfContexts>;
    fn Next(&mut self, ulcount: u32, rgcontext: *mut ::core::option::Option<ITfContext>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, ulcount: u32) -> ::windows::core::Result<()>;
}
impl IEnumTfContexts_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTfContexts_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumTfContexts_Vtbl {
        unsafe extern "system" fn Clone<Impl: IEnumTfContexts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumTfContexts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, rgcontext: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&rgcontext), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumTfContexts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumTfContexts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
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
pub trait IEnumTfDisplayAttributeInfo_Impl: Sized {
    fn Clone(&mut self) -> ::windows::core::Result<IEnumTfDisplayAttributeInfo>;
    fn Next(&mut self, ulcount: u32, rginfo: *mut ::core::option::Option<ITfDisplayAttributeInfo>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, ulcount: u32) -> ::windows::core::Result<()>;
}
impl IEnumTfDisplayAttributeInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTfDisplayAttributeInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumTfDisplayAttributeInfo_Vtbl {
        unsafe extern "system" fn Clone<Impl: IEnumTfDisplayAttributeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumTfDisplayAttributeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, rginfo: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&rginfo), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumTfDisplayAttributeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumTfDisplayAttributeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
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
pub trait IEnumTfDocumentMgrs_Impl: Sized {
    fn Clone(&mut self) -> ::windows::core::Result<IEnumTfDocumentMgrs>;
    fn Next(&mut self, ulcount: u32, rgdocumentmgr: *mut ::core::option::Option<ITfDocumentMgr>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, ulcount: u32) -> ::windows::core::Result<()>;
}
impl IEnumTfDocumentMgrs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTfDocumentMgrs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumTfDocumentMgrs_Vtbl {
        unsafe extern "system" fn Clone<Impl: IEnumTfDocumentMgrs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumTfDocumentMgrs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, rgdocumentmgr: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&rgdocumentmgr), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumTfDocumentMgrs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumTfDocumentMgrs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
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
pub trait IEnumTfFunctionProviders_Impl: Sized {
    fn Clone(&mut self) -> ::windows::core::Result<IEnumTfFunctionProviders>;
    fn Next(&mut self, ulcount: u32, ppcmdobj: *mut ::core::option::Option<ITfFunctionProvider>, pcfetch: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, ulcount: u32) -> ::windows::core::Result<()>;
}
impl IEnumTfFunctionProviders_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTfFunctionProviders_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumTfFunctionProviders_Vtbl {
        unsafe extern "system" fn Clone<Impl: IEnumTfFunctionProviders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumTfFunctionProviders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, ppcmdobj: *mut ::windows::core::RawPtr, pcfetch: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&ppcmdobj), ::core::mem::transmute_copy(&pcfetch)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumTfFunctionProviders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumTfFunctionProviders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
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
pub trait IEnumTfInputProcessorProfiles_Impl: Sized {
    fn Clone(&mut self) -> ::windows::core::Result<IEnumTfInputProcessorProfiles>;
    fn Next(&mut self, ulcount: u32, pprofile: *mut TF_INPUTPROCESSORPROFILE, pcfetch: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, ulcount: u32) -> ::windows::core::Result<()>;
}
impl IEnumTfInputProcessorProfiles_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTfInputProcessorProfiles_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumTfInputProcessorProfiles_Vtbl {
        unsafe extern "system" fn Clone<Impl: IEnumTfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumTfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, pprofile: *mut TF_INPUTPROCESSORPROFILE, pcfetch: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pprofile), ::core::mem::transmute_copy(&pcfetch)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumTfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumTfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
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
pub trait IEnumTfLangBarItems_Impl: Sized {
    fn Clone(&mut self) -> ::windows::core::Result<IEnumTfLangBarItems>;
    fn Next(&mut self, ulcount: u32, ppitem: *mut ::core::option::Option<ITfLangBarItem>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, ulcount: u32) -> ::windows::core::Result<()>;
}
impl IEnumTfLangBarItems_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTfLangBarItems_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumTfLangBarItems_Vtbl {
        unsafe extern "system" fn Clone<Impl: IEnumTfLangBarItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumTfLangBarItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, ppitem: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&ppitem), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumTfLangBarItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumTfLangBarItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
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
pub trait IEnumTfLanguageProfiles_Impl: Sized {
    fn Clone(&mut self) -> ::windows::core::Result<IEnumTfLanguageProfiles>;
    fn Next(&mut self, ulcount: u32, pprofile: *mut TF_LANGUAGEPROFILE, pcfetch: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, ulcount: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IEnumTfLanguageProfiles_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTfLanguageProfiles_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumTfLanguageProfiles_Vtbl {
        unsafe extern "system" fn Clone<Impl: IEnumTfLanguageProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumTfLanguageProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, pprofile: *mut TF_LANGUAGEPROFILE, pcfetch: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pprofile), ::core::mem::transmute_copy(&pcfetch)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumTfLanguageProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumTfLanguageProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
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
pub trait IEnumTfLatticeElements_Impl: Sized {
    fn Clone(&mut self) -> ::windows::core::Result<IEnumTfLatticeElements>;
    fn Next(&mut self, ulcount: u32, rgselements: *mut TF_LMLATTELEMENT, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, ulcount: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IEnumTfLatticeElements_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTfLatticeElements_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumTfLatticeElements_Vtbl {
        unsafe extern "system" fn Clone<Impl: IEnumTfLatticeElements_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumTfLatticeElements_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, rgselements: *mut TF_LMLATTELEMENT, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&rgselements), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumTfLatticeElements_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumTfLatticeElements_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
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
pub trait IEnumTfProperties_Impl: Sized {
    fn Clone(&mut self) -> ::windows::core::Result<IEnumTfProperties>;
    fn Next(&mut self, ulcount: u32, ppprop: *mut ::core::option::Option<ITfProperty>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, ulcount: u32) -> ::windows::core::Result<()>;
}
impl IEnumTfProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTfProperties_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumTfProperties_Vtbl {
        unsafe extern "system" fn Clone<Impl: IEnumTfProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumTfProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, ppprop: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&ppprop), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumTfProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumTfProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
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
pub trait IEnumTfPropertyValue_Impl: Sized {
    fn Clone(&mut self) -> ::windows::core::Result<IEnumTfPropertyValue>;
    fn Next(&mut self, ulcount: u32, rgvalues: *mut TF_PROPERTYVAL, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, ulcount: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IEnumTfPropertyValue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTfPropertyValue_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumTfPropertyValue_Vtbl {
        unsafe extern "system" fn Clone<Impl: IEnumTfPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumTfPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, rgvalues: *mut TF_PROPERTYVAL, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&rgvalues), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumTfPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumTfPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
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
pub trait IEnumTfRanges_Impl: Sized {
    fn Clone(&mut self) -> ::windows::core::Result<IEnumTfRanges>;
    fn Next(&mut self, ulcount: u32, pprange: *mut ::core::option::Option<ITfRange>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, ulcount: u32) -> ::windows::core::Result<()>;
}
impl IEnumTfRanges_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTfRanges_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumTfRanges_Vtbl {
        unsafe extern "system" fn Clone<Impl: IEnumTfRanges_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumTfRanges_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, pprange: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pprange), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumTfRanges_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumTfRanges_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
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
pub trait IEnumTfUIElements_Impl: Sized {
    fn Clone(&mut self) -> ::windows::core::Result<IEnumTfUIElements>;
    fn Next(&mut self, ulcount: u32, ppelement: *mut ::core::option::Option<ITfUIElement>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, ulcount: u32) -> ::windows::core::Result<()>;
}
impl IEnumTfUIElements_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTfUIElements_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumTfUIElements_Vtbl {
        unsafe extern "system" fn Clone<Impl: IEnumTfUIElements_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumTfUIElements_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, ppelement: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&ppelement), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumTfUIElements_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumTfUIElements_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
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
pub trait IInternalDocWrap_Impl: Sized {
    fn NotifyRevoke(&mut self) -> ::windows::core::Result<()>;
}
impl IInternalDocWrap_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternalDocWrap_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInternalDocWrap_Vtbl {
        unsafe extern "system" fn NotifyRevoke<Impl: IInternalDocWrap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait ISpeechCommandProvider_Impl: Sized {
    fn EnumSpeechCommands(&mut self, langid: u16) -> ::windows::core::Result<IEnumSpeechCommands>;
    fn ProcessCommand(&mut self, pszcommand: super::super::Foundation::PWSTR, cch: u32, langid: u16) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISpeechCommandProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechCommandProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechCommandProvider_Vtbl {
        unsafe extern "system" fn EnumSpeechCommands<Impl: ISpeechCommandProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumSpeechCommands(::core::mem::transmute_copy(&langid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessCommand<Impl: ISpeechCommandProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcommand: super::super::Foundation::PWSTR, cch: u32, langid: u16) -> ::windows::core::HRESULT {
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
pub trait ITextStoreACP_Impl: Sized {
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
impl ITextStoreACP_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextStoreACP_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextStoreACP_Vtbl {
        unsafe extern "system" fn AdviseSink<Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void, dwmask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AdviseSink(::core::mem::transmute_copy(&riid), ::core::mem::transmute(&punk), ::core::mem::transmute_copy(&dwmask)).into()
        }
        unsafe extern "system" fn UnadviseSink<Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnadviseSink(::core::mem::transmute(&punk)).into()
        }
        unsafe extern "system" fn RequestLock<Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlockflags: u32, phrsession: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestLock(::core::mem::transmute_copy(&dwlockflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *phrsession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdcs: *mut TS_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pdcs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryInsert<Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpteststart: i32, acptestend: i32, cch: u32, pacpresultstart: *mut i32, pacpresultend: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryInsert(::core::mem::transmute_copy(&acpteststart), ::core::mem::transmute_copy(&acptestend), ::core::mem::transmute_copy(&cch), ::core::mem::transmute_copy(&pacpresultstart), ::core::mem::transmute_copy(&pacpresultend)).into()
        }
        unsafe extern "system" fn GetSelection<Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ACP, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSelection(::core::mem::transmute_copy(&ulindex), ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pselection), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn SetSelection<Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, pselection: *const TS_SELECTION_ACP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelection(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pselection)).into()
        }
        unsafe extern "system" fn GetText<Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, pchplain: super::super::Foundation::PWSTR, cchplainreq: u32, pcchplainret: *mut u32, prgruninfo: *mut TS_RUNINFO, cruninforeq: u32, pcruninforet: *mut u32, pacpnext: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetText(::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend), ::core::mem::transmute_copy(&pchplain), ::core::mem::transmute_copy(&cchplainreq), ::core::mem::transmute_copy(&pcchplainret), ::core::mem::transmute_copy(&prgruninfo), ::core::mem::transmute_copy(&cruninforeq), ::core::mem::transmute_copy(&pcruninforet), ::core::mem::transmute_copy(&pacpnext)).into()
        }
        unsafe extern "system" fn SetText<Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, acpstart: i32, acpend: i32, pchtext: super::super::Foundation::PWSTR, cch: u32, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetText(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend), ::core::mem::transmute_copy(&pchtext), ::core::mem::transmute_copy(&cch)) {
                ::core::result::Result::Ok(ok__) => {
                    *pchange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormattedText<Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, ppdataobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFormattedText(::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdataobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEmbedded<Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acppos: i32, rguidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEmbedded(::core::mem::transmute_copy(&acppos), ::core::mem::transmute_copy(&rguidservice), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryInsertEmbedded<Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidservice: *const ::windows::core::GUID, pformatetc: *const super::super::System::Com::FORMATETC, pfinsertable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryInsertEmbedded(::core::mem::transmute_copy(&pguidservice), ::core::mem::transmute_copy(&pformatetc)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfinsertable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertEmbedded<Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, acpstart: i32, acpend: i32, pdataobject: ::windows::core::RawPtr, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InsertEmbedded(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend), ::core::mem::transmute(&pdataobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *pchange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertTextAtSelection<Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pchtext: super::super::Foundation::PWSTR, cch: u32, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertTextAtSelection(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pchtext), ::core::mem::transmute_copy(&cch), ::core::mem::transmute_copy(&pacpstart), ::core::mem::transmute_copy(&pacpend), ::core::mem::transmute_copy(&pchange)).into()
        }
        unsafe extern "system" fn InsertEmbeddedAtSelection<Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pdataobject: ::windows::core::RawPtr, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertEmbeddedAtSelection(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pdataobject), ::core::mem::transmute_copy(&pacpstart), ::core::mem::transmute_copy(&pacpend), ::core::mem::transmute_copy(&pchange)).into()
        }
        unsafe extern "system" fn RequestSupportedAttrs<Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestSupportedAttrs(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&cfilterattrs), ::core::mem::transmute_copy(&pafilterattrs)).into()
        }
        unsafe extern "system" fn RequestAttrsAtPosition<Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestAttrsAtPosition(::core::mem::transmute_copy(&acppos), ::core::mem::transmute_copy(&cfilterattrs), ::core::mem::transmute_copy(&pafilterattrs), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn RequestAttrsTransitioningAtPosition<Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestAttrsTransitioningAtPosition(::core::mem::transmute_copy(&acppos), ::core::mem::transmute_copy(&cfilterattrs), ::core::mem::transmute_copy(&pafilterattrs), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn FindNextAttrTransition<Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpstart: i32, acphalt: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32, pacpnext: *mut i32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FindNextAttrTransition(::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acphalt), ::core::mem::transmute_copy(&cfilterattrs), ::core::mem::transmute_copy(&pafilterattrs), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pacpnext), ::core::mem::transmute_copy(&pffound), ::core::mem::transmute_copy(&plfoundoffset)).into()
        }
        unsafe extern "system" fn RetrieveRequestedAttrs<Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RetrieveRequestedAttrs(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&paattrvals), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn GetEndACP<Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pacp: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEndACP() {
                ::core::result::Result::Ok(ok__) => {
                    *pacp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActiveView<Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvcview: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActiveView() {
                ::core::result::Result::Ok(ok__) => {
                    *pvcview = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetACPFromPoint<Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32, pacp: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetACPFromPoint(::core::mem::transmute_copy(&vcview), ::core::mem::transmute_copy(&ptscreen), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pacp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTextExt<Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcview: u32, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTextExt(::core::mem::transmute_copy(&vcview), ::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend), ::core::mem::transmute_copy(&prc), ::core::mem::transmute_copy(&pfclipped)).into()
        }
        unsafe extern "system" fn GetScreenExt<Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcview: u32, prc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetScreenExt(::core::mem::transmute_copy(&vcview)) {
                ::core::result::Result::Ok(ok__) => {
                    *prc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWnd<Impl: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcview: u32, phwnd: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
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
pub trait ITextStoreACP2_Impl: Sized {
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
impl ITextStoreACP2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextStoreACP2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextStoreACP2_Vtbl {
        unsafe extern "system" fn AdviseSink<Impl: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void, dwmask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AdviseSink(::core::mem::transmute_copy(&riid), ::core::mem::transmute(&punk), ::core::mem::transmute_copy(&dwmask)).into()
        }
        unsafe extern "system" fn UnadviseSink<Impl: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnadviseSink(::core::mem::transmute(&punk)).into()
        }
        unsafe extern "system" fn RequestLock<Impl: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlockflags: u32, phrsession: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestLock(::core::mem::transmute_copy(&dwlockflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *phrsession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdcs: *mut TS_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pdcs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryInsert<Impl: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpteststart: i32, acptestend: i32, cch: u32, pacpresultstart: *mut i32, pacpresultend: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryInsert(::core::mem::transmute_copy(&acpteststart), ::core::mem::transmute_copy(&acptestend), ::core::mem::transmute_copy(&cch), ::core::mem::transmute_copy(&pacpresultstart), ::core::mem::transmute_copy(&pacpresultend)).into()
        }
        unsafe extern "system" fn GetSelection<Impl: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ACP, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSelection(::core::mem::transmute_copy(&ulindex), ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pselection), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn SetSelection<Impl: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, pselection: *const TS_SELECTION_ACP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelection(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pselection)).into()
        }
        unsafe extern "system" fn GetText<Impl: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, pchplain: super::super::Foundation::PWSTR, cchplainreq: u32, pcchplainret: *mut u32, prgruninfo: *mut TS_RUNINFO, cruninforeq: u32, pcruninforet: *mut u32, pacpnext: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetText(::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend), ::core::mem::transmute_copy(&pchplain), ::core::mem::transmute_copy(&cchplainreq), ::core::mem::transmute_copy(&pcchplainret), ::core::mem::transmute_copy(&prgruninfo), ::core::mem::transmute_copy(&cruninforeq), ::core::mem::transmute_copy(&pcruninforet), ::core::mem::transmute_copy(&pacpnext)).into()
        }
        unsafe extern "system" fn SetText<Impl: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, acpstart: i32, acpend: i32, pchtext: super::super::Foundation::PWSTR, cch: u32, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetText(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend), ::core::mem::transmute_copy(&pchtext), ::core::mem::transmute_copy(&cch)) {
                ::core::result::Result::Ok(ok__) => {
                    *pchange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormattedText<Impl: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, ppdataobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFormattedText(::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdataobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEmbedded<Impl: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acppos: i32, rguidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEmbedded(::core::mem::transmute_copy(&acppos), ::core::mem::transmute_copy(&rguidservice), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryInsertEmbedded<Impl: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidservice: *const ::windows::core::GUID, pformatetc: *const super::super::System::Com::FORMATETC, pfinsertable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryInsertEmbedded(::core::mem::transmute_copy(&pguidservice), ::core::mem::transmute_copy(&pformatetc)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfinsertable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertEmbedded<Impl: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, acpstart: i32, acpend: i32, pdataobject: ::windows::core::RawPtr, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InsertEmbedded(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend), ::core::mem::transmute(&pdataobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *pchange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertTextAtSelection<Impl: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pchtext: super::super::Foundation::PWSTR, cch: u32, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertTextAtSelection(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pchtext), ::core::mem::transmute_copy(&cch), ::core::mem::transmute_copy(&pacpstart), ::core::mem::transmute_copy(&pacpend), ::core::mem::transmute_copy(&pchange)).into()
        }
        unsafe extern "system" fn InsertEmbeddedAtSelection<Impl: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pdataobject: ::windows::core::RawPtr, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertEmbeddedAtSelection(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pdataobject), ::core::mem::transmute_copy(&pacpstart), ::core::mem::transmute_copy(&pacpend), ::core::mem::transmute_copy(&pchange)).into()
        }
        unsafe extern "system" fn RequestSupportedAttrs<Impl: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestSupportedAttrs(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&cfilterattrs), ::core::mem::transmute_copy(&pafilterattrs)).into()
        }
        unsafe extern "system" fn RequestAttrsAtPosition<Impl: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestAttrsAtPosition(::core::mem::transmute_copy(&acppos), ::core::mem::transmute_copy(&cfilterattrs), ::core::mem::transmute_copy(&pafilterattrs), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn RequestAttrsTransitioningAtPosition<Impl: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestAttrsTransitioningAtPosition(::core::mem::transmute_copy(&acppos), ::core::mem::transmute_copy(&cfilterattrs), ::core::mem::transmute_copy(&pafilterattrs), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn FindNextAttrTransition<Impl: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpstart: i32, acphalt: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32, pacpnext: *mut i32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FindNextAttrTransition(::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acphalt), ::core::mem::transmute_copy(&cfilterattrs), ::core::mem::transmute_copy(&pafilterattrs), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pacpnext), ::core::mem::transmute_copy(&pffound), ::core::mem::transmute_copy(&plfoundoffset)).into()
        }
        unsafe extern "system" fn RetrieveRequestedAttrs<Impl: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RetrieveRequestedAttrs(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&paattrvals), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn GetEndACP<Impl: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pacp: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEndACP() {
                ::core::result::Result::Ok(ok__) => {
                    *pacp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActiveView<Impl: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvcview: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActiveView() {
                ::core::result::Result::Ok(ok__) => {
                    *pvcview = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetACPFromPoint<Impl: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32, pacp: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetACPFromPoint(::core::mem::transmute_copy(&vcview), ::core::mem::transmute_copy(&ptscreen), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pacp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTextExt<Impl: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcview: u32, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTextExt(::core::mem::transmute_copy(&vcview), ::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend), ::core::mem::transmute_copy(&prc), ::core::mem::transmute_copy(&pfclipped)).into()
        }
        unsafe extern "system" fn GetScreenExt<Impl: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcview: u32, prc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
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
pub trait ITextStoreACPEx_Impl: Sized {
    fn ScrollToRect(&mut self, acpstart: i32, acpend: i32, rc: super::super::Foundation::RECT, dwposition: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITextStoreACPEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextStoreACPEx_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextStoreACPEx_Vtbl {
        unsafe extern "system" fn ScrollToRect<Impl: ITextStoreACPEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, rc: super::super::Foundation::RECT, dwposition: u32) -> ::windows::core::HRESULT {
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
pub trait ITextStoreACPServices_Impl: Sized {
    fn Serialize(&mut self, pprop: ::core::option::Option<ITfProperty>, prange: ::core::option::Option<ITfRange>, phdr: *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: ::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn Unserialize(&mut self, pprop: ::core::option::Option<ITfProperty>, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: ::core::option::Option<super::super::System::Com::IStream>, ploader: ::core::option::Option<ITfPersistentPropertyLoaderACP>) -> ::windows::core::Result<()>;
    fn ForceLoadProperty(&mut self, pprop: ::core::option::Option<ITfProperty>) -> ::windows::core::Result<()>;
    fn CreateRange(&mut self, acpstart: i32, acpend: i32) -> ::windows::core::Result<ITfRangeACP>;
}
#[cfg(feature = "Win32_System_Com")]
impl ITextStoreACPServices_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextStoreACPServices_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextStoreACPServices_Vtbl {
        unsafe extern "system" fn Serialize<Impl: ITextStoreACPServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprop: ::windows::core::RawPtr, prange: ::windows::core::RawPtr, phdr: *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Serialize(::core::mem::transmute(&pprop), ::core::mem::transmute(&prange), ::core::mem::transmute_copy(&phdr), ::core::mem::transmute(&pstream)).into()
        }
        unsafe extern "system" fn Unserialize<Impl: ITextStoreACPServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprop: ::windows::core::RawPtr, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: ::windows::core::RawPtr, ploader: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unserialize(::core::mem::transmute(&pprop), ::core::mem::transmute_copy(&phdr), ::core::mem::transmute(&pstream), ::core::mem::transmute(&ploader)).into()
        }
        unsafe extern "system" fn ForceLoadProperty<Impl: ITextStoreACPServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprop: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ForceLoadProperty(::core::mem::transmute(&pprop)).into()
        }
        unsafe extern "system" fn CreateRange<Impl: ITextStoreACPServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ITextStoreACPSink_Impl: Sized {
    fn OnTextChange(&mut self, dwflags: TEXT_STORE_TEXT_CHANGE_FLAGS, pchange: *const TS_TEXTCHANGE) -> ::windows::core::Result<()>;
    fn OnSelectionChange(&mut self) -> ::windows::core::Result<()>;
    fn OnLayoutChange(&mut self, lcode: TsLayoutCode, vcview: u32) -> ::windows::core::Result<()>;
    fn OnStatusChange(&mut self, dwflags: u32) -> ::windows::core::Result<()>;
    fn OnAttrsChange(&mut self, acpstart: i32, acpend: i32, cattrs: u32, paattrs: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnLockGranted(&mut self, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> ::windows::core::Result<()>;
    fn OnStartEditTransaction(&mut self) -> ::windows::core::Result<()>;
    fn OnEndEditTransaction(&mut self) -> ::windows::core::Result<()>;
}
impl ITextStoreACPSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextStoreACPSink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextStoreACPSink_Vtbl {
        unsafe extern "system" fn OnTextChange<Impl: ITextStoreACPSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: TEXT_STORE_TEXT_CHANGE_FLAGS, pchange: *const TS_TEXTCHANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnTextChange(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pchange)).into()
        }
        unsafe extern "system" fn OnSelectionChange<Impl: ITextStoreACPSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnSelectionChange().into()
        }
        unsafe extern "system" fn OnLayoutChange<Impl: ITextStoreACPSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcode: TsLayoutCode, vcview: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnLayoutChange(::core::mem::transmute_copy(&lcode), ::core::mem::transmute_copy(&vcview)).into()
        }
        unsafe extern "system" fn OnStatusChange<Impl: ITextStoreACPSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnStatusChange(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn OnAttrsChange<Impl: ITextStoreACPSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, cattrs: u32, paattrs: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnAttrsChange(::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend), ::core::mem::transmute_copy(&cattrs), ::core::mem::transmute_copy(&paattrs)).into()
        }
        unsafe extern "system" fn OnLockGranted<Impl: ITextStoreACPSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnLockGranted(::core::mem::transmute_copy(&dwlockflags)).into()
        }
        unsafe extern "system" fn OnStartEditTransaction<Impl: ITextStoreACPSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnStartEditTransaction().into()
        }
        unsafe extern "system" fn OnEndEditTransaction<Impl: ITextStoreACPSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait ITextStoreACPSinkEx_Impl: Sized + ITextStoreACPSink_Impl {
    fn OnDisconnect(&mut self) -> ::windows::core::Result<()>;
}
impl ITextStoreACPSinkEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextStoreACPSinkEx_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextStoreACPSinkEx_Vtbl {
        unsafe extern "system" fn OnDisconnect<Impl: ITextStoreACPSinkEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDisconnect().into()
        }
        Self { base: ITextStoreACPSink_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), OnDisconnect: OnDisconnect::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextStoreACPSinkEx as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextStoreAnchor_Impl: Sized {
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
impl ITextStoreAnchor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextStoreAnchor_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextStoreAnchor_Vtbl {
        unsafe extern "system" fn AdviseSink<Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void, dwmask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AdviseSink(::core::mem::transmute_copy(&riid), ::core::mem::transmute(&punk), ::core::mem::transmute_copy(&dwmask)).into()
        }
        unsafe extern "system" fn UnadviseSink<Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnadviseSink(::core::mem::transmute(&punk)).into()
        }
        unsafe extern "system" fn RequestLock<Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlockflags: u32, phrsession: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestLock(::core::mem::transmute_copy(&dwlockflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *phrsession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdcs: *mut TS_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pdcs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryInsert<Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pateststart: ::windows::core::RawPtr, patestend: ::windows::core::RawPtr, cch: u32, pparesultstart: *mut ::windows::core::RawPtr, pparesultend: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryInsert(::core::mem::transmute(&pateststart), ::core::mem::transmute(&patestend), ::core::mem::transmute_copy(&cch), ::core::mem::transmute_copy(&pparesultstart), ::core::mem::transmute_copy(&pparesultend)).into()
        }
        unsafe extern "system" fn GetSelection<Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ANCHOR, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSelection(::core::mem::transmute_copy(&ulindex), ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pselection), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn SetSelection<Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, pselection: *const TS_SELECTION_ANCHOR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelection(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pselection)).into()
        }
        unsafe extern "system" fn GetText<Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pastart: ::windows::core::RawPtr, paend: ::windows::core::RawPtr, pchtext: super::super::Foundation::PWSTR, cchreq: u32, pcch: *mut u32, fupdateanchor: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetText(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pastart), ::core::mem::transmute(&paend), ::core::mem::transmute_copy(&pchtext), ::core::mem::transmute_copy(&cchreq), ::core::mem::transmute_copy(&pcch), ::core::mem::transmute_copy(&fupdateanchor)).into()
        }
        unsafe extern "system" fn SetText<Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pastart: ::windows::core::RawPtr, paend: ::windows::core::RawPtr, pchtext: super::super::Foundation::PWSTR, cch: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetText(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pastart), ::core::mem::transmute(&paend), ::core::mem::transmute_copy(&pchtext), ::core::mem::transmute_copy(&cch)).into()
        }
        unsafe extern "system" fn GetFormattedText<Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pastart: ::windows::core::RawPtr, paend: ::windows::core::RawPtr, ppdataobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFormattedText(::core::mem::transmute(&pastart), ::core::mem::transmute(&paend)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdataobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEmbedded<Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, papos: ::windows::core::RawPtr, rguidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEmbedded(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&papos), ::core::mem::transmute_copy(&rguidservice), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertEmbedded<Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pastart: ::windows::core::RawPtr, paend: ::windows::core::RawPtr, pdataobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertEmbedded(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pastart), ::core::mem::transmute(&paend), ::core::mem::transmute(&pdataobject)).into()
        }
        unsafe extern "system" fn RequestSupportedAttrs<Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestSupportedAttrs(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&cfilterattrs), ::core::mem::transmute_copy(&pafilterattrs)).into()
        }
        unsafe extern "system" fn RequestAttrsAtPosition<Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, papos: ::windows::core::RawPtr, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestAttrsAtPosition(::core::mem::transmute(&papos), ::core::mem::transmute_copy(&cfilterattrs), ::core::mem::transmute_copy(&pafilterattrs), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn RequestAttrsTransitioningAtPosition<Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, papos: ::windows::core::RawPtr, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestAttrsTransitioningAtPosition(::core::mem::transmute(&papos), ::core::mem::transmute_copy(&cfilterattrs), ::core::mem::transmute_copy(&pafilterattrs), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn FindNextAttrTransition<Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pastart: ::windows::core::RawPtr, pahalt: ::windows::core::RawPtr, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FindNextAttrTransition(::core::mem::transmute(&pastart), ::core::mem::transmute(&pahalt), ::core::mem::transmute_copy(&cfilterattrs), ::core::mem::transmute_copy(&pafilterattrs), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pffound), ::core::mem::transmute_copy(&plfoundoffset)).into()
        }
        unsafe extern "system" fn RetrieveRequestedAttrs<Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RetrieveRequestedAttrs(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&paattrvals), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn GetStart<Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppastart: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStart() {
                ::core::result::Result::Ok(ok__) => {
                    *ppastart = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnd<Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaend: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnd() {
                ::core::result::Result::Ok(ok__) => {
                    *ppaend = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActiveView<Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvcview: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActiveView() {
                ::core::result::Result::Ok(ok__) => {
                    *pvcview = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAnchorFromPoint<Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32, ppasite: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAnchorFromPoint(::core::mem::transmute_copy(&vcview), ::core::mem::transmute_copy(&ptscreen), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppasite = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTextExt<Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcview: u32, pastart: ::windows::core::RawPtr, paend: ::windows::core::RawPtr, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTextExt(::core::mem::transmute_copy(&vcview), ::core::mem::transmute(&pastart), ::core::mem::transmute(&paend), ::core::mem::transmute_copy(&prc), ::core::mem::transmute_copy(&pfclipped)).into()
        }
        unsafe extern "system" fn GetScreenExt<Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcview: u32, prc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetScreenExt(::core::mem::transmute_copy(&vcview)) {
                ::core::result::Result::Ok(ok__) => {
                    *prc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWnd<Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcview: u32, phwnd: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWnd(::core::mem::transmute_copy(&vcview)) {
                ::core::result::Result::Ok(ok__) => {
                    *phwnd = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryInsertEmbedded<Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidservice: *const ::windows::core::GUID, pformatetc: *const super::super::System::Com::FORMATETC, pfinsertable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryInsertEmbedded(::core::mem::transmute_copy(&pguidservice), ::core::mem::transmute_copy(&pformatetc)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfinsertable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertTextAtSelection<Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pchtext: super::super::Foundation::PWSTR, cch: u32, ppastart: *mut ::windows::core::RawPtr, ppaend: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertTextAtSelection(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pchtext), ::core::mem::transmute_copy(&cch), ::core::mem::transmute_copy(&ppastart), ::core::mem::transmute_copy(&ppaend)).into()
        }
        unsafe extern "system" fn InsertEmbeddedAtSelection<Impl: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pdataobject: ::windows::core::RawPtr, ppastart: *mut ::windows::core::RawPtr, ppaend: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ITextStoreAnchorEx_Impl: Sized {
    fn ScrollToRect(&mut self, pstart: ::core::option::Option<IAnchor>, pend: ::core::option::Option<IAnchor>, rc: super::super::Foundation::RECT, dwposition: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITextStoreAnchorEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextStoreAnchorEx_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextStoreAnchorEx_Vtbl {
        unsafe extern "system" fn ScrollToRect<Impl: ITextStoreAnchorEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstart: ::windows::core::RawPtr, pend: ::windows::core::RawPtr, rc: super::super::Foundation::RECT, dwposition: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ScrollToRect(::core::mem::transmute(&pstart), ::core::mem::transmute(&pend), ::core::mem::transmute_copy(&rc), ::core::mem::transmute_copy(&dwposition)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), ScrollToRect: ScrollToRect::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextStoreAnchorEx as ::windows::core::Interface>::IID
    }
}
pub trait ITextStoreAnchorSink_Impl: Sized {
    fn OnTextChange(&mut self, dwflags: TEXT_STORE_CHANGE_FLAGS, pastart: ::core::option::Option<IAnchor>, paend: ::core::option::Option<IAnchor>) -> ::windows::core::Result<()>;
    fn OnSelectionChange(&mut self) -> ::windows::core::Result<()>;
    fn OnLayoutChange(&mut self, lcode: TsLayoutCode, vcview: u32) -> ::windows::core::Result<()>;
    fn OnStatusChange(&mut self, dwflags: u32) -> ::windows::core::Result<()>;
    fn OnAttrsChange(&mut self, pastart: ::core::option::Option<IAnchor>, paend: ::core::option::Option<IAnchor>, cattrs: u32, paattrs: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnLockGranted(&mut self, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> ::windows::core::Result<()>;
    fn OnStartEditTransaction(&mut self) -> ::windows::core::Result<()>;
    fn OnEndEditTransaction(&mut self) -> ::windows::core::Result<()>;
}
impl ITextStoreAnchorSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextStoreAnchorSink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextStoreAnchorSink_Vtbl {
        unsafe extern "system" fn OnTextChange<Impl: ITextStoreAnchorSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: TEXT_STORE_CHANGE_FLAGS, pastart: ::windows::core::RawPtr, paend: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnTextChange(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pastart), ::core::mem::transmute(&paend)).into()
        }
        unsafe extern "system" fn OnSelectionChange<Impl: ITextStoreAnchorSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnSelectionChange().into()
        }
        unsafe extern "system" fn OnLayoutChange<Impl: ITextStoreAnchorSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcode: TsLayoutCode, vcview: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnLayoutChange(::core::mem::transmute_copy(&lcode), ::core::mem::transmute_copy(&vcview)).into()
        }
        unsafe extern "system" fn OnStatusChange<Impl: ITextStoreAnchorSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnStatusChange(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn OnAttrsChange<Impl: ITextStoreAnchorSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pastart: ::windows::core::RawPtr, paend: ::windows::core::RawPtr, cattrs: u32, paattrs: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnAttrsChange(::core::mem::transmute(&pastart), ::core::mem::transmute(&paend), ::core::mem::transmute_copy(&cattrs), ::core::mem::transmute_copy(&paattrs)).into()
        }
        unsafe extern "system" fn OnLockGranted<Impl: ITextStoreAnchorSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnLockGranted(::core::mem::transmute_copy(&dwlockflags)).into()
        }
        unsafe extern "system" fn OnStartEditTransaction<Impl: ITextStoreAnchorSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnStartEditTransaction().into()
        }
        unsafe extern "system" fn OnEndEditTransaction<Impl: ITextStoreAnchorSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait ITextStoreSinkAnchorEx_Impl: Sized + ITextStoreAnchorSink_Impl {
    fn OnDisconnect(&mut self) -> ::windows::core::Result<()>;
}
impl ITextStoreSinkAnchorEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextStoreSinkAnchorEx_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextStoreSinkAnchorEx_Vtbl {
        unsafe extern "system" fn OnDisconnect<Impl: ITextStoreSinkAnchorEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDisconnect().into()
        }
        Self { base: ITextStoreAnchorSink_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), OnDisconnect: OnDisconnect::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextStoreSinkAnchorEx as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfActiveLanguageProfileNotifySink_Impl: Sized {
    fn OnActivated(&mut self, clsid: *const ::windows::core::GUID, guidprofile: *const ::windows::core::GUID, factivated: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfActiveLanguageProfileNotifySink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfActiveLanguageProfileNotifySink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfActiveLanguageProfileNotifySink_Vtbl {
        unsafe extern "system" fn OnActivated<Impl: ITfActiveLanguageProfileNotifySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, guidprofile: *const ::windows::core::GUID, factivated: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnActivated(::core::mem::transmute_copy(&clsid), ::core::mem::transmute_copy(&guidprofile), ::core::mem::transmute_copy(&factivated)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnActivated: OnActivated::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfActiveLanguageProfileNotifySink as ::windows::core::Interface>::IID
    }
}
pub trait ITfCandidateList_Impl: Sized {
    fn EnumCandidates(&mut self) -> ::windows::core::Result<IEnumTfCandidates>;
    fn GetCandidate(&mut self, nindex: u32) -> ::windows::core::Result<ITfCandidateString>;
    fn GetCandidateNum(&mut self) -> ::windows::core::Result<u32>;
    fn SetResult(&mut self, nindex: u32, imcr: TfCandidateResult) -> ::windows::core::Result<()>;
}
impl ITfCandidateList_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfCandidateList_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfCandidateList_Vtbl {
        unsafe extern "system" fn EnumCandidates<Impl: ITfCandidateList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumCandidates() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCandidate<Impl: ITfCandidateList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, ppcand: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCandidate(::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcand = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCandidateNum<Impl: ITfCandidateList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pncnt: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCandidateNum() {
                ::core::result::Result::Ok(ok__) => {
                    *pncnt = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetResult<Impl: ITfCandidateList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, imcr: TfCandidateResult) -> ::windows::core::HRESULT {
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
pub trait ITfCandidateListUIElement_Impl: Sized + ITfUIElement_Impl {
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
impl ITfCandidateListUIElement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfCandidateListUIElement_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfCandidateListUIElement_Vtbl {
        unsafe extern "system" fn GetUpdatedFlags<Impl: ITfCandidateListUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUpdatedFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocumentMgr<Impl: ITfCandidateListUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdim: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocumentMgr() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdim = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Impl: ITfCandidateListUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pucount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelection<Impl: ITfCandidateListUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *puindex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetString<Impl: ITfCandidateListUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, pstr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetString(::core::mem::transmute_copy(&uindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPageIndex<Impl: ITfCandidateListUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pindex: *mut u32, usize: u32, pupagecnt: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPageIndex(::core::mem::transmute_copy(&pindex), ::core::mem::transmute_copy(&usize), ::core::mem::transmute_copy(&pupagecnt)).into()
        }
        unsafe extern "system" fn SetPageIndex<Impl: ITfCandidateListUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pindex: *const u32, upagecnt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPageIndex(::core::mem::transmute_copy(&pindex), ::core::mem::transmute_copy(&upagecnt)).into()
        }
        unsafe extern "system" fn GetCurrentPage<Impl: ITfCandidateListUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pupage: *mut u32) -> ::windows::core::HRESULT {
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
            base: ITfUIElement_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait ITfCandidateListUIElementBehavior_Impl: Sized + ITfUIElement_Impl + ITfCandidateListUIElement_Impl {
    fn SetSelection(&mut self, nindex: u32) -> ::windows::core::Result<()>;
    fn Finalize(&mut self) -> ::windows::core::Result<()>;
    fn Abort(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfCandidateListUIElementBehavior_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfCandidateListUIElementBehavior_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfCandidateListUIElementBehavior_Vtbl {
        unsafe extern "system" fn SetSelection<Impl: ITfCandidateListUIElementBehavior_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelection(::core::mem::transmute_copy(&nindex)).into()
        }
        unsafe extern "system" fn Finalize<Impl: ITfCandidateListUIElementBehavior_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Finalize().into()
        }
        unsafe extern "system" fn Abort<Impl: ITfCandidateListUIElementBehavior_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Abort().into()
        }
        Self {
            base: ITfCandidateListUIElement_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait ITfCandidateString_Impl: Sized {
    fn GetString(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetIndex(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfCandidateString_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfCandidateString_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfCandidateString_Vtbl {
        unsafe extern "system" fn GetString<Impl: ITfCandidateString_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetString() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIndex<Impl: ITfCandidateString_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnindex: *mut u32) -> ::windows::core::HRESULT {
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
pub trait ITfCategoryMgr_Impl: Sized {
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
impl ITfCategoryMgr_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfCategoryMgr_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfCategoryMgr_Vtbl {
        unsafe extern "system" fn RegisterCategory<Impl: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, rcatid: *const ::windows::core::GUID, rguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterCategory(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&rcatid), ::core::mem::transmute_copy(&rguid)).into()
        }
        unsafe extern "system" fn UnregisterCategory<Impl: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, rcatid: *const ::windows::core::GUID, rguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterCategory(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&rcatid), ::core::mem::transmute_copy(&rguid)).into()
        }
        unsafe extern "system" fn EnumCategoriesInItem<Impl: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumCategoriesInItem(::core::mem::transmute_copy(&rguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumItemsInCategory<Impl: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rcatid: *const ::windows::core::GUID, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumItemsInCategory(::core::mem::transmute_copy(&rcatid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindClosestCategory<Impl: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, pcatid: *mut ::windows::core::GUID, ppcatidlist: *const *const ::windows::core::GUID, ulcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FindClosestCategory(::core::mem::transmute_copy(&rguid), ::core::mem::transmute_copy(&pcatid), ::core::mem::transmute_copy(&ppcatidlist), ::core::mem::transmute_copy(&ulcount)).into()
        }
        unsafe extern "system" fn RegisterGUIDDescription<Impl: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, rguid: *const ::windows::core::GUID, pchdesc: super::super::Foundation::PWSTR, cch: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterGUIDDescription(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&rguid), ::core::mem::transmute_copy(&pchdesc), ::core::mem::transmute_copy(&cch)).into()
        }
        unsafe extern "system" fn UnregisterGUIDDescription<Impl: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, rguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterGUIDDescription(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&rguid)).into()
        }
        unsafe extern "system" fn GetGUIDDescription<Impl: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, pbstrdesc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGUIDDescription(::core::mem::transmute_copy(&rguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterGUIDDWORD<Impl: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, rguid: *const ::windows::core::GUID, dw: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterGUIDDWORD(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&rguid), ::core::mem::transmute_copy(&dw)).into()
        }
        unsafe extern "system" fn UnregisterGUIDDWORD<Impl: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, rguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterGUIDDWORD(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&rguid)).into()
        }
        unsafe extern "system" fn GetGUIDDWORD<Impl: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, pdw: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGUIDDWORD(::core::mem::transmute_copy(&rguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdw = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterGUID<Impl: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, pguidatom: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterGUID(::core::mem::transmute_copy(&rguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pguidatom = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGUID<Impl: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidatom: u32, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGUID(::core::mem::transmute_copy(&guidatom)) {
                ::core::result::Result::Ok(ok__) => {
                    *pguid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqualTfGuidAtom<Impl: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidatom: u32, rguid: *const ::windows::core::GUID, pfequal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
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
pub trait ITfCleanupContextDurationSink_Impl: Sized {
    fn OnStartCleanupContext(&mut self) -> ::windows::core::Result<()>;
    fn OnEndCleanupContext(&mut self) -> ::windows::core::Result<()>;
}
impl ITfCleanupContextDurationSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfCleanupContextDurationSink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfCleanupContextDurationSink_Vtbl {
        unsafe extern "system" fn OnStartCleanupContext<Impl: ITfCleanupContextDurationSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnStartCleanupContext().into()
        }
        unsafe extern "system" fn OnEndCleanupContext<Impl: ITfCleanupContextDurationSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait ITfCleanupContextSink_Impl: Sized {
    fn OnCleanupContext(&mut self, ecwrite: u32, pic: ::core::option::Option<ITfContext>) -> ::windows::core::Result<()>;
}
impl ITfCleanupContextSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfCleanupContextSink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfCleanupContextSink_Vtbl {
        unsafe extern "system" fn OnCleanupContext<Impl: ITfCleanupContextSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ecwrite: u32, pic: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnCleanupContext(::core::mem::transmute_copy(&ecwrite), ::core::mem::transmute(&pic)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnCleanupContext: OnCleanupContext::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfCleanupContextSink as ::windows::core::Interface>::IID
    }
}
pub trait ITfClientId_Impl: Sized {
    fn GetClientId(&mut self, rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<u32>;
}
impl ITfClientId_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfClientId_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfClientId_Vtbl {
        unsafe extern "system" fn GetClientId<Impl: ITfClientId_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, ptid: *mut u32) -> ::windows::core::HRESULT {
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
pub trait ITfCompartment_Impl: Sized {
    fn SetValue(&mut self, tid: u32, pvarvalue: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GetValue(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITfCompartment_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfCompartment_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfCompartment_Vtbl {
        unsafe extern "system" fn SetValue<Impl: ITfCompartment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tid: u32, pvarvalue: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&tid), ::core::mem::transmute_copy(&pvarvalue)).into()
        }
        unsafe extern "system" fn GetValue<Impl: ITfCompartment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
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
pub trait ITfCompartmentEventSink_Impl: Sized {
    fn OnChange(&mut self, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl ITfCompartmentEventSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfCompartmentEventSink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfCompartmentEventSink_Vtbl {
        unsafe extern "system" fn OnChange<Impl: ITfCompartmentEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
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
pub trait ITfCompartmentMgr_Impl: Sized {
    fn GetCompartment(&mut self, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<ITfCompartment>;
    fn ClearCompartment(&mut self, tid: u32, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn EnumCompartments(&mut self) -> ::windows::core::Result<super::super::System::Com::IEnumGUID>;
}
#[cfg(feature = "Win32_System_Com")]
impl ITfCompartmentMgr_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfCompartmentMgr_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfCompartmentMgr_Vtbl {
        unsafe extern "system" fn GetCompartment<Impl: ITfCompartmentMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, ppcomp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCompartment(::core::mem::transmute_copy(&rguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcomp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearCompartment<Impl: ITfCompartmentMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tid: u32, rguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearCompartment(::core::mem::transmute_copy(&tid), ::core::mem::transmute_copy(&rguid)).into()
        }
        unsafe extern "system" fn EnumCompartments<Impl: ITfCompartmentMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ITfComposition_Impl: Sized {
    fn GetRange(&mut self) -> ::windows::core::Result<ITfRange>;
    fn ShiftStart(&mut self, ecwrite: u32, pnewstart: ::core::option::Option<ITfRange>) -> ::windows::core::Result<()>;
    fn ShiftEnd(&mut self, ecwrite: u32, pnewend: ::core::option::Option<ITfRange>) -> ::windows::core::Result<()>;
    fn EndComposition(&mut self, ecwrite: u32) -> ::windows::core::Result<()>;
}
impl ITfComposition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfComposition_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfComposition_Vtbl {
        unsafe extern "system" fn GetRange<Impl: ITfComposition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRange() {
                ::core::result::Result::Ok(ok__) => {
                    *pprange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShiftStart<Impl: ITfComposition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ecwrite: u32, pnewstart: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShiftStart(::core::mem::transmute_copy(&ecwrite), ::core::mem::transmute(&pnewstart)).into()
        }
        unsafe extern "system" fn ShiftEnd<Impl: ITfComposition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ecwrite: u32, pnewend: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShiftEnd(::core::mem::transmute_copy(&ecwrite), ::core::mem::transmute(&pnewend)).into()
        }
        unsafe extern "system" fn EndComposition<Impl: ITfComposition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ecwrite: u32) -> ::windows::core::HRESULT {
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
pub trait ITfCompositionSink_Impl: Sized {
    fn OnCompositionTerminated(&mut self, ecwrite: u32, pcomposition: ::core::option::Option<ITfComposition>) -> ::windows::core::Result<()>;
}
impl ITfCompositionSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfCompositionSink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfCompositionSink_Vtbl {
        unsafe extern "system" fn OnCompositionTerminated<Impl: ITfCompositionSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ecwrite: u32, pcomposition: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnCompositionTerminated(::core::mem::transmute_copy(&ecwrite), ::core::mem::transmute(&pcomposition)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnCompositionTerminated: OnCompositionTerminated::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfCompositionSink as ::windows::core::Interface>::IID
    }
}
pub trait ITfCompositionView_Impl: Sized {
    fn GetOwnerClsid(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetRange(&mut self) -> ::windows::core::Result<ITfRange>;
}
impl ITfCompositionView_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfCompositionView_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfCompositionView_Vtbl {
        unsafe extern "system" fn GetOwnerClsid<Impl: ITfCompositionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOwnerClsid() {
                ::core::result::Result::Ok(ok__) => {
                    *pclsid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRange<Impl: ITfCompositionView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ITfConfigureSystemKeystrokeFeed_Impl: Sized {
    fn DisableSystemKeystrokeFeed(&mut self) -> ::windows::core::Result<()>;
    fn EnableSystemKeystrokeFeed(&mut self) -> ::windows::core::Result<()>;
}
impl ITfConfigureSystemKeystrokeFeed_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfConfigureSystemKeystrokeFeed_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfConfigureSystemKeystrokeFeed_Vtbl {
        unsafe extern "system" fn DisableSystemKeystrokeFeed<Impl: ITfConfigureSystemKeystrokeFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisableSystemKeystrokeFeed().into()
        }
        unsafe extern "system" fn EnableSystemKeystrokeFeed<Impl: ITfConfigureSystemKeystrokeFeed_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait ITfContext_Impl: Sized {
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
impl ITfContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfContext_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfContext_Vtbl {
        unsafe extern "system" fn RequestEditSession<Impl: ITfContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tid: u32, pes: ::windows::core::RawPtr, dwflags: TF_CONTEXT_EDIT_CONTEXT_FLAGS, phrsession: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestEditSession(::core::mem::transmute_copy(&tid), ::core::mem::transmute(&pes), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *phrsession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InWriteSession<Impl: ITfContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tid: u32, pfwritesession: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InWriteSession(::core::mem::transmute_copy(&tid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfwritesession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelection<Impl: ITfContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, ulindex: u32, ulcount: u32, pselection: *mut TF_SELECTION, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSelection(::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&ulindex), ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pselection), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn SetSelection<Impl: ITfContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, ulcount: u32, pselection: *const TF_SELECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelection(::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pselection)).into()
        }
        unsafe extern "system" fn GetStart<Impl: ITfContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, ppstart: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStart(::core::mem::transmute_copy(&ec)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppstart = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnd<Impl: ITfContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, ppend: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnd(::core::mem::transmute_copy(&ec)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppend = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActiveView<Impl: ITfContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActiveView() {
                ::core::result::Result::Ok(ok__) => {
                    *ppview = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumViews<Impl: ITfContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumViews() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: ITfContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdcs: *mut TS_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pdcs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: ITfContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidprop: *const ::windows::core::GUID, ppprop: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&guidprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAppProperty<Impl: ITfContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidprop: *const ::windows::core::GUID, ppprop: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAppProperty(::core::mem::transmute_copy(&guidprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrackProperties<Impl: ITfContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prgprop: *const *const ::windows::core::GUID, cprop: u32, prgappprop: *const *const ::windows::core::GUID, cappprop: u32, ppproperty: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrackProperties(::core::mem::transmute_copy(&prgprop), ::core::mem::transmute_copy(&cprop), ::core::mem::transmute_copy(&prgappprop), ::core::mem::transmute_copy(&cappprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppproperty = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumProperties<Impl: ITfContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocumentMgr<Impl: ITfContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocumentMgr() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdm = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRangeBackup<Impl: ITfContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, prange: ::windows::core::RawPtr, ppbackup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ITfContextComposition_Impl: Sized {
    fn StartComposition(&mut self, ecwrite: u32, pcompositionrange: ::core::option::Option<ITfRange>, psink: ::core::option::Option<ITfCompositionSink>) -> ::windows::core::Result<ITfComposition>;
    fn EnumCompositions(&mut self) -> ::windows::core::Result<IEnumITfCompositionView>;
    fn FindComposition(&mut self, ecread: u32, ptestrange: ::core::option::Option<ITfRange>) -> ::windows::core::Result<IEnumITfCompositionView>;
    fn TakeOwnership(&mut self, ecwrite: u32, pcomposition: ::core::option::Option<ITfCompositionView>, psink: ::core::option::Option<ITfCompositionSink>) -> ::windows::core::Result<ITfComposition>;
}
impl ITfContextComposition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfContextComposition_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfContextComposition_Vtbl {
        unsafe extern "system" fn StartComposition<Impl: ITfContextComposition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ecwrite: u32, pcompositionrange: ::windows::core::RawPtr, psink: ::windows::core::RawPtr, ppcomposition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartComposition(::core::mem::transmute_copy(&ecwrite), ::core::mem::transmute(&pcompositionrange), ::core::mem::transmute(&psink)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcomposition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumCompositions<Impl: ITfContextComposition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumCompositions() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindComposition<Impl: ITfContextComposition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ecread: u32, ptestrange: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindComposition(::core::mem::transmute_copy(&ecread), ::core::mem::transmute(&ptestrange)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TakeOwnership<Impl: ITfContextComposition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ecwrite: u32, pcomposition: ::windows::core::RawPtr, psink: ::windows::core::RawPtr, ppcomposition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ITfContextKeyEventSink_Impl: Sized {
    fn OnKeyDown(&mut self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn OnKeyUp(&mut self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn OnTestKeyDown(&mut self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn OnTestKeyUp(&mut self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfContextKeyEventSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfContextKeyEventSink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfContextKeyEventSink_Vtbl {
        unsafe extern "system" fn OnKeyDown<Impl: ITfContextKeyEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnKeyDown(::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfeaten = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnKeyUp<Impl: ITfContextKeyEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnKeyUp(::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfeaten = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnTestKeyDown<Impl: ITfContextKeyEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnTestKeyDown(::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfeaten = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnTestKeyUp<Impl: ITfContextKeyEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
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
pub trait ITfContextOwner_Impl: Sized {
    fn GetACPFromPoint(&mut self, ptscreen: *const super::super::Foundation::POINT, dwflags: u32) -> ::windows::core::Result<i32>;
    fn GetTextExt(&mut self, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetScreenExt(&mut self) -> ::windows::core::Result<super::super::Foundation::RECT>;
    fn GetStatus(&mut self) -> ::windows::core::Result<TS_STATUS>;
    fn GetWnd(&mut self) -> ::windows::core::Result<super::super::Foundation::HWND>;
    fn GetAttribute(&mut self, rguidattribute: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITfContextOwner_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfContextOwner_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfContextOwner_Vtbl {
        unsafe extern "system" fn GetACPFromPoint<Impl: ITfContextOwner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptscreen: *const super::super::Foundation::POINT, dwflags: u32, pacp: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetACPFromPoint(::core::mem::transmute_copy(&ptscreen), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pacp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTextExt<Impl: ITfContextOwner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTextExt(::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend), ::core::mem::transmute_copy(&prc), ::core::mem::transmute_copy(&pfclipped)).into()
        }
        unsafe extern "system" fn GetScreenExt<Impl: ITfContextOwner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetScreenExt() {
                ::core::result::Result::Ok(ok__) => {
                    *prc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: ITfContextOwner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdcs: *mut TS_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pdcs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWnd<Impl: ITfContextOwner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWnd() {
                ::core::result::Result::Ok(ok__) => {
                    *phwnd = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttribute<Impl: ITfContextOwner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidattribute: *const ::windows::core::GUID, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
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
pub trait ITfContextOwnerCompositionServices_Impl: Sized + ITfContextComposition_Impl {
    fn TerminateComposition(&mut self, pcomposition: ::core::option::Option<ITfCompositionView>) -> ::windows::core::Result<()>;
}
impl ITfContextOwnerCompositionServices_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfContextOwnerCompositionServices_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfContextOwnerCompositionServices_Vtbl {
        unsafe extern "system" fn TerminateComposition<Impl: ITfContextOwnerCompositionServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcomposition: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TerminateComposition(::core::mem::transmute(&pcomposition)).into()
        }
        Self {
            base: ITfContextComposition_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            TerminateComposition: TerminateComposition::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfContextOwnerCompositionServices as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfContextOwnerCompositionSink_Impl: Sized {
    fn OnStartComposition(&mut self, pcomposition: ::core::option::Option<ITfCompositionView>) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn OnUpdateComposition(&mut self, pcomposition: ::core::option::Option<ITfCompositionView>, prangenew: ::core::option::Option<ITfRange>) -> ::windows::core::Result<()>;
    fn OnEndComposition(&mut self, pcomposition: ::core::option::Option<ITfCompositionView>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfContextOwnerCompositionSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfContextOwnerCompositionSink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfContextOwnerCompositionSink_Vtbl {
        unsafe extern "system" fn OnStartComposition<Impl: ITfContextOwnerCompositionSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcomposition: ::windows::core::RawPtr, pfok: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnStartComposition(::core::mem::transmute(&pcomposition)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfok = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnUpdateComposition<Impl: ITfContextOwnerCompositionSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcomposition: ::windows::core::RawPtr, prangenew: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnUpdateComposition(::core::mem::transmute(&pcomposition), ::core::mem::transmute(&prangenew)).into()
        }
        unsafe extern "system" fn OnEndComposition<Impl: ITfContextOwnerCompositionSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcomposition: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ITfContextOwnerServices_Impl: Sized {
    fn OnLayoutChange(&mut self) -> ::windows::core::Result<()>;
    fn OnStatusChange(&mut self, dwflags: u32) -> ::windows::core::Result<()>;
    fn OnAttributeChange(&mut self, rguidattribute: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn Serialize(&mut self, pprop: ::core::option::Option<ITfProperty>, prange: ::core::option::Option<ITfRange>, phdr: *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: ::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn Unserialize(&mut self, pprop: ::core::option::Option<ITfProperty>, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: ::core::option::Option<super::super::System::Com::IStream>, ploader: ::core::option::Option<ITfPersistentPropertyLoaderACP>) -> ::windows::core::Result<()>;
    fn ForceLoadProperty(&mut self, pprop: ::core::option::Option<ITfProperty>) -> ::windows::core::Result<()>;
    fn CreateRange(&mut self, acpstart: i32, acpend: i32) -> ::windows::core::Result<ITfRangeACP>;
}
#[cfg(feature = "Win32_System_Com")]
impl ITfContextOwnerServices_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfContextOwnerServices_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfContextOwnerServices_Vtbl {
        unsafe extern "system" fn OnLayoutChange<Impl: ITfContextOwnerServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnLayoutChange().into()
        }
        unsafe extern "system" fn OnStatusChange<Impl: ITfContextOwnerServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnStatusChange(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn OnAttributeChange<Impl: ITfContextOwnerServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidattribute: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnAttributeChange(::core::mem::transmute_copy(&rguidattribute)).into()
        }
        unsafe extern "system" fn Serialize<Impl: ITfContextOwnerServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprop: ::windows::core::RawPtr, prange: ::windows::core::RawPtr, phdr: *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Serialize(::core::mem::transmute(&pprop), ::core::mem::transmute(&prange), ::core::mem::transmute_copy(&phdr), ::core::mem::transmute(&pstream)).into()
        }
        unsafe extern "system" fn Unserialize<Impl: ITfContextOwnerServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprop: ::windows::core::RawPtr, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: ::windows::core::RawPtr, ploader: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unserialize(::core::mem::transmute(&pprop), ::core::mem::transmute_copy(&phdr), ::core::mem::transmute(&pstream), ::core::mem::transmute(&ploader)).into()
        }
        unsafe extern "system" fn ForceLoadProperty<Impl: ITfContextOwnerServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprop: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ForceLoadProperty(::core::mem::transmute(&pprop)).into()
        }
        unsafe extern "system" fn CreateRange<Impl: ITfContextOwnerServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ITfContextView_Impl: Sized {
    fn GetRangeFromPoint(&mut self, ec: u32, ppt: *const super::super::Foundation::POINT, dwflags: u32) -> ::windows::core::Result<ITfRange>;
    fn GetTextExt(&mut self, ec: u32, prange: ::core::option::Option<ITfRange>, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetScreenExt(&mut self) -> ::windows::core::Result<super::super::Foundation::RECT>;
    fn GetWnd(&mut self) -> ::windows::core::Result<super::super::Foundation::HWND>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfContextView_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfContextView_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfContextView_Vtbl {
        unsafe extern "system" fn GetRangeFromPoint<Impl: ITfContextView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, ppt: *const super::super::Foundation::POINT, dwflags: u32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRangeFromPoint(::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&ppt), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTextExt<Impl: ITfContextView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, prange: ::windows::core::RawPtr, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTextExt(::core::mem::transmute_copy(&ec), ::core::mem::transmute(&prange), ::core::mem::transmute_copy(&prc), ::core::mem::transmute_copy(&pfclipped)).into()
        }
        unsafe extern "system" fn GetScreenExt<Impl: ITfContextView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetScreenExt() {
                ::core::result::Result::Ok(ok__) => {
                    *prc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWnd<Impl: ITfContextView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
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
pub trait ITfCreatePropertyStore_Impl: Sized {
    fn IsStoreSerializable(&mut self, guidprop: *const ::windows::core::GUID, prange: ::core::option::Option<ITfRange>, ppropstore: ::core::option::Option<ITfPropertyStore>) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CreatePropertyStore(&mut self, guidprop: *const ::windows::core::GUID, prange: ::core::option::Option<ITfRange>, cb: u32, pstream: ::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<ITfPropertyStore>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ITfCreatePropertyStore_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfCreatePropertyStore_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfCreatePropertyStore_Vtbl {
        unsafe extern "system" fn IsStoreSerializable<Impl: ITfCreatePropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidprop: *const ::windows::core::GUID, prange: ::windows::core::RawPtr, ppropstore: ::windows::core::RawPtr, pfserializable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStoreSerializable(::core::mem::transmute_copy(&guidprop), ::core::mem::transmute(&prange), ::core::mem::transmute(&ppropstore)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfserializable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePropertyStore<Impl: ITfCreatePropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidprop: *const ::windows::core::GUID, prange: ::windows::core::RawPtr, cb: u32, pstream: ::windows::core::RawPtr, ppstore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ITfDisplayAttributeInfo_Impl: Sized {
    fn GetGUID(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetDescription(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetAttributeInfo(&mut self) -> ::windows::core::Result<TF_DISPLAYATTRIBUTE>;
    fn SetAttributeInfo(&mut self, pda: *const TF_DISPLAYATTRIBUTE) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfDisplayAttributeInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfDisplayAttributeInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfDisplayAttributeInfo_Vtbl {
        unsafe extern "system" fn GetGUID<Impl: ITfDisplayAttributeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGUID() {
                ::core::result::Result::Ok(ok__) => {
                    *pguid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescription<Impl: ITfDisplayAttributeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdesc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeInfo<Impl: ITfDisplayAttributeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pda: *mut TF_DISPLAYATTRIBUTE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAttributeInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *pda = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttributeInfo<Impl: ITfDisplayAttributeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pda: *const TF_DISPLAYATTRIBUTE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAttributeInfo(::core::mem::transmute_copy(&pda)).into()
        }
        unsafe extern "system" fn Reset<Impl: ITfDisplayAttributeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait ITfDisplayAttributeMgr_Impl: Sized {
    fn OnUpdateInfo(&mut self) -> ::windows::core::Result<()>;
    fn EnumDisplayAttributeInfo(&mut self) -> ::windows::core::Result<IEnumTfDisplayAttributeInfo>;
    fn GetDisplayAttributeInfo(&mut self, guid: *const ::windows::core::GUID, ppinfo: *mut ::core::option::Option<ITfDisplayAttributeInfo>, pclsidowner: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl ITfDisplayAttributeMgr_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfDisplayAttributeMgr_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfDisplayAttributeMgr_Vtbl {
        unsafe extern "system" fn OnUpdateInfo<Impl: ITfDisplayAttributeMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnUpdateInfo().into()
        }
        unsafe extern "system" fn EnumDisplayAttributeInfo<Impl: ITfDisplayAttributeMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumDisplayAttributeInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayAttributeInfo<Impl: ITfDisplayAttributeMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, ppinfo: *mut ::windows::core::RawPtr, pclsidowner: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
pub trait ITfDisplayAttributeNotifySink_Impl: Sized {
    fn OnUpdateInfo(&mut self) -> ::windows::core::Result<()>;
}
impl ITfDisplayAttributeNotifySink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfDisplayAttributeNotifySink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfDisplayAttributeNotifySink_Vtbl {
        unsafe extern "system" fn OnUpdateInfo<Impl: ITfDisplayAttributeNotifySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnUpdateInfo().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnUpdateInfo: OnUpdateInfo::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfDisplayAttributeNotifySink as ::windows::core::Interface>::IID
    }
}
pub trait ITfDisplayAttributeProvider_Impl: Sized {
    fn EnumDisplayAttributeInfo(&mut self) -> ::windows::core::Result<IEnumTfDisplayAttributeInfo>;
    fn GetDisplayAttributeInfo(&mut self, guid: *const ::windows::core::GUID) -> ::windows::core::Result<ITfDisplayAttributeInfo>;
}
impl ITfDisplayAttributeProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfDisplayAttributeProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfDisplayAttributeProvider_Vtbl {
        unsafe extern "system" fn EnumDisplayAttributeInfo<Impl: ITfDisplayAttributeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumDisplayAttributeInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayAttributeInfo<Impl: ITfDisplayAttributeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, ppinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ITfDocumentMgr_Impl: Sized {
    fn CreateContext(&mut self, tidowner: u32, dwflags: u32, punk: ::core::option::Option<::windows::core::IUnknown>, ppic: *mut ::core::option::Option<ITfContext>, pectextstore: *mut u32) -> ::windows::core::Result<()>;
    fn Push(&mut self, pic: ::core::option::Option<ITfContext>) -> ::windows::core::Result<()>;
    fn Pop(&mut self, dwflags: u32) -> ::windows::core::Result<()>;
    fn GetTop(&mut self) -> ::windows::core::Result<ITfContext>;
    fn GetBase(&mut self) -> ::windows::core::Result<ITfContext>;
    fn EnumContexts(&mut self) -> ::windows::core::Result<IEnumTfContexts>;
}
impl ITfDocumentMgr_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfDocumentMgr_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfDocumentMgr_Vtbl {
        unsafe extern "system" fn CreateContext<Impl: ITfDocumentMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tidowner: u32, dwflags: u32, punk: *mut ::core::ffi::c_void, ppic: *mut ::windows::core::RawPtr, pectextstore: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateContext(::core::mem::transmute_copy(&tidowner), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&punk), ::core::mem::transmute_copy(&ppic), ::core::mem::transmute_copy(&pectextstore)).into()
        }
        unsafe extern "system" fn Push<Impl: ITfDocumentMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Push(::core::mem::transmute(&pic)).into()
        }
        unsafe extern "system" fn Pop<Impl: ITfDocumentMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Pop(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetTop<Impl: ITfDocumentMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppic: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTop() {
                ::core::result::Result::Ok(ok__) => {
                    *ppic = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBase<Impl: ITfDocumentMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppic: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBase() {
                ::core::result::Result::Ok(ok__) => {
                    *ppic = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumContexts<Impl: ITfDocumentMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ITfEditRecord_Impl: Sized {
    fn GetSelectionStatus(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetTextAndPropertyUpdates(&mut self, dwflags: GET_TEXT_AND_PROPERTY_UPDATES_FLAGS, prgproperties: *const *const ::windows::core::GUID, cproperties: u32) -> ::windows::core::Result<IEnumTfRanges>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfEditRecord_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfEditRecord_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfEditRecord_Vtbl {
        unsafe extern "system" fn GetSelectionStatus<Impl: ITfEditRecord_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfchanged: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSelectionStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pfchanged = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTextAndPropertyUpdates<Impl: ITfEditRecord_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: GET_TEXT_AND_PROPERTY_UPDATES_FLAGS, prgproperties: *const *const ::windows::core::GUID, cproperties: u32, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ITfEditSession_Impl: Sized {
    fn DoEditSession(&mut self, ec: u32) -> ::windows::core::Result<()>;
}
impl ITfEditSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfEditSession_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfEditSession_Vtbl {
        unsafe extern "system" fn DoEditSession<Impl: ITfEditSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DoEditSession(::core::mem::transmute_copy(&ec)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), DoEditSession: DoEditSession::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfEditSession as ::windows::core::Interface>::IID
    }
}
pub trait ITfEditTransactionSink_Impl: Sized {
    fn OnStartEditTransaction(&mut self, pic: ::core::option::Option<ITfContext>) -> ::windows::core::Result<()>;
    fn OnEndEditTransaction(&mut self, pic: ::core::option::Option<ITfContext>) -> ::windows::core::Result<()>;
}
impl ITfEditTransactionSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfEditTransactionSink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfEditTransactionSink_Vtbl {
        unsafe extern "system" fn OnStartEditTransaction<Impl: ITfEditTransactionSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnStartEditTransaction(::core::mem::transmute(&pic)).into()
        }
        unsafe extern "system" fn OnEndEditTransaction<Impl: ITfEditTransactionSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ITfFnAdviseText_Impl: Sized + ITfFunction_Impl {
    fn OnTextUpdate(&mut self, prange: ::core::option::Option<ITfRange>, pchtext: super::super::Foundation::PWSTR, cch: i32) -> ::windows::core::Result<()>;
    fn OnLatticeUpdate(&mut self, prange: ::core::option::Option<ITfRange>, plattice: ::core::option::Option<ITfLMLattice>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfFnAdviseText_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFnAdviseText_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfFnAdviseText_Vtbl {
        unsafe extern "system" fn OnTextUpdate<Impl: ITfFnAdviseText_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, pchtext: super::super::Foundation::PWSTR, cch: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnTextUpdate(::core::mem::transmute(&prange), ::core::mem::transmute_copy(&pchtext), ::core::mem::transmute_copy(&cch)).into()
        }
        unsafe extern "system" fn OnLatticeUpdate<Impl: ITfFnAdviseText_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, plattice: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnLatticeUpdate(::core::mem::transmute(&prange), ::core::mem::transmute(&plattice)).into()
        }
        Self {
            base: ITfFunction_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            OnTextUpdate: OnTextUpdate::<Impl, IMPL_OFFSET>,
            OnLatticeUpdate: OnLatticeUpdate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfFnAdviseText as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfFnBalloon_Impl: Sized {
    fn UpdateBalloon(&mut self, style: TfLBBalloonStyle, pch: super::super::Foundation::PWSTR, cch: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfFnBalloon_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFnBalloon_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfFnBalloon_Vtbl {
        unsafe extern "system" fn UpdateBalloon<Impl: ITfFnBalloon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, style: TfLBBalloonStyle, pch: super::super::Foundation::PWSTR, cch: u32) -> ::windows::core::HRESULT {
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
pub trait ITfFnConfigure_Impl: Sized + ITfFunction_Impl {
    fn Show(&mut self, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfFnConfigure_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFnConfigure_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfFnConfigure_Vtbl {
        unsafe extern "system" fn Show<Impl: ITfFnConfigure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Show(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&rguidprofile)).into()
        }
        Self { base: ITfFunction_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Show: Show::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfFnConfigure as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfFnConfigureRegisterEudc_Impl: Sized + ITfFunction_Impl {
    fn Show(&mut self, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const ::windows::core::GUID, bstrregistered: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfFnConfigureRegisterEudc_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFnConfigureRegisterEudc_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfFnConfigureRegisterEudc_Vtbl {
        unsafe extern "system" fn Show<Impl: ITfFnConfigureRegisterEudc_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const ::windows::core::GUID, bstrregistered: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Show(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&rguidprofile), ::core::mem::transmute_copy(&bstrregistered)).into()
        }
        Self { base: ITfFunction_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Show: Show::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfFnConfigureRegisterEudc as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfFnConfigureRegisterWord_Impl: Sized + ITfFunction_Impl {
    fn Show(&mut self, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const ::windows::core::GUID, bstrregistered: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfFnConfigureRegisterWord_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFnConfigureRegisterWord_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfFnConfigureRegisterWord_Vtbl {
        unsafe extern "system" fn Show<Impl: ITfFnConfigureRegisterWord_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const ::windows::core::GUID, bstrregistered: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Show(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&rguidprofile), ::core::mem::transmute_copy(&bstrregistered)).into()
        }
        Self { base: ITfFunction_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Show: Show::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfFnConfigureRegisterWord as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfFnCustomSpeechCommand_Impl: Sized + ITfFunction_Impl {
    fn SetSpeechCommandProvider(&mut self, pspcmdprovider: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfFnCustomSpeechCommand_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFnCustomSpeechCommand_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfFnCustomSpeechCommand_Vtbl {
        unsafe extern "system" fn SetSpeechCommandProvider<Impl: ITfFnCustomSpeechCommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pspcmdprovider: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSpeechCommandProvider(::core::mem::transmute(&pspcmdprovider)).into()
        }
        Self {
            base: ITfFunction_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetSpeechCommandProvider: SetSpeechCommandProvider::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfFnCustomSpeechCommand as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfFnGetLinguisticAlternates_Impl: Sized + ITfFunction_Impl {
    fn GetAlternates(&mut self, prange: ::core::option::Option<ITfRange>) -> ::windows::core::Result<ITfCandidateList>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfFnGetLinguisticAlternates_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFnGetLinguisticAlternates_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfFnGetLinguisticAlternates_Vtbl {
        unsafe extern "system" fn GetAlternates<Impl: ITfFnGetLinguisticAlternates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, ppcandidatelist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAlternates(::core::mem::transmute(&prange)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcandidatelist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ITfFunction_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetAlternates: GetAlternates::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfFnGetLinguisticAlternates as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfFnGetPreferredTouchKeyboardLayout_Impl: Sized + ITfFunction_Impl {
    fn GetLayout(&mut self, ptkblayouttype: *mut TKBLayoutType, pwpreferredlayoutid: *const u16) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfFnGetPreferredTouchKeyboardLayout_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFnGetPreferredTouchKeyboardLayout_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfFnGetPreferredTouchKeyboardLayout_Vtbl {
        unsafe extern "system" fn GetLayout<Impl: ITfFnGetPreferredTouchKeyboardLayout_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptkblayouttype: *mut TKBLayoutType, pwpreferredlayoutid: *const u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLayout(::core::mem::transmute_copy(&ptkblayouttype), ::core::mem::transmute_copy(&pwpreferredlayoutid)).into()
        }
        Self { base: ITfFunction_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetLayout: GetLayout::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfFnGetPreferredTouchKeyboardLayout as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfFnGetSAPIObject_Impl: Sized + ITfFunction_Impl {
    fn Get(&mut self, sobj: TfSapiObject) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfFnGetSAPIObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFnGetSAPIObject_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfFnGetSAPIObject_Vtbl {
        unsafe extern "system" fn Get<Impl: ITfFnGetSAPIObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sobj: TfSapiObject, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Get(::core::mem::transmute_copy(&sobj)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ITfFunction_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Get: Get::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfFnGetSAPIObject as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfFnLMInternal_Impl: Sized + ITfFunction_Impl + ITfFnLMProcessor_Impl {
    fn ProcessLattice(&mut self, prange: ::core::option::Option<ITfRange>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfFnLMInternal_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFnLMInternal_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfFnLMInternal_Vtbl {
        unsafe extern "system" fn ProcessLattice<Impl: ITfFnLMInternal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessLattice(::core::mem::transmute(&prange)).into()
        }
        Self { base: ITfFnLMProcessor_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), ProcessLattice: ProcessLattice::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfFnLMInternal as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfFnLMProcessor_Impl: Sized + ITfFunction_Impl {
    fn QueryRange(&mut self, prange: ::core::option::Option<ITfRange>, ppnewrange: *mut ::core::option::Option<ITfRange>, pfaccepted: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn QueryLangID(&mut self, langid: u16) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetReconversion(&mut self, prange: ::core::option::Option<ITfRange>) -> ::windows::core::Result<ITfCandidateList>;
    fn Reconvert(&mut self, prange: ::core::option::Option<ITfRange>) -> ::windows::core::Result<()>;
    fn QueryKey(&mut self, fup: super::super::Foundation::BOOL, vkey: super::super::Foundation::WPARAM, lparamkeydata: super::super::Foundation::LPARAM) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn InvokeKey(&mut self, fup: super::super::Foundation::BOOL, vkey: super::super::Foundation::WPARAM, lparamkeydata: super::super::Foundation::LPARAM) -> ::windows::core::Result<()>;
    fn InvokeFunc(&mut self, pic: ::core::option::Option<ITfContext>, refguidfunc: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfFnLMProcessor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFnLMProcessor_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfFnLMProcessor_Vtbl {
        unsafe extern "system" fn QueryRange<Impl: ITfFnLMProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, ppnewrange: *mut ::windows::core::RawPtr, pfaccepted: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryRange(::core::mem::transmute(&prange), ::core::mem::transmute_copy(&ppnewrange), ::core::mem::transmute_copy(&pfaccepted)).into()
        }
        unsafe extern "system" fn QueryLangID<Impl: ITfFnLMProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16, pfaccepted: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryLangID(::core::mem::transmute_copy(&langid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfaccepted = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReconversion<Impl: ITfFnLMProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, ppcandlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetReconversion(::core::mem::transmute(&prange)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcandlist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reconvert<Impl: ITfFnLMProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reconvert(::core::mem::transmute(&prange)).into()
        }
        unsafe extern "system" fn QueryKey<Impl: ITfFnLMProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fup: super::super::Foundation::BOOL, vkey: super::super::Foundation::WPARAM, lparamkeydata: super::super::Foundation::LPARAM, pfinterested: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryKey(::core::mem::transmute_copy(&fup), ::core::mem::transmute_copy(&vkey), ::core::mem::transmute_copy(&lparamkeydata)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfinterested = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvokeKey<Impl: ITfFnLMProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fup: super::super::Foundation::BOOL, vkey: super::super::Foundation::WPARAM, lparamkeydata: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InvokeKey(::core::mem::transmute_copy(&fup), ::core::mem::transmute_copy(&vkey), ::core::mem::transmute_copy(&lparamkeydata)).into()
        }
        unsafe extern "system" fn InvokeFunc<Impl: ITfFnLMProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr, refguidfunc: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InvokeFunc(::core::mem::transmute(&pic), ::core::mem::transmute_copy(&refguidfunc)).into()
        }
        Self {
            base: ITfFunction_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait ITfFnLangProfileUtil_Impl: Sized + ITfFunction_Impl {
    fn RegisterActiveProfiles(&mut self) -> ::windows::core::Result<()>;
    fn IsProfileAvailableForLang(&mut self, langid: u16) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfFnLangProfileUtil_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFnLangProfileUtil_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfFnLangProfileUtil_Vtbl {
        unsafe extern "system" fn RegisterActiveProfiles<Impl: ITfFnLangProfileUtil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterActiveProfiles().into()
        }
        unsafe extern "system" fn IsProfileAvailableForLang<Impl: ITfFnLangProfileUtil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16, pfavailable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
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
            base: ITfFunction_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            RegisterActiveProfiles: RegisterActiveProfiles::<Impl, IMPL_OFFSET>,
            IsProfileAvailableForLang: IsProfileAvailableForLang::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfFnLangProfileUtil as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfFnPlayBack_Impl: Sized + ITfFunction_Impl {
    fn QueryRange(&mut self, prange: ::core::option::Option<ITfRange>, ppnewrange: *mut ::core::option::Option<ITfRange>, pfplayable: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn Play(&mut self, prange: ::core::option::Option<ITfRange>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfFnPlayBack_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFnPlayBack_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfFnPlayBack_Vtbl {
        unsafe extern "system" fn QueryRange<Impl: ITfFnPlayBack_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, ppnewrange: *mut ::windows::core::RawPtr, pfplayable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryRange(::core::mem::transmute(&prange), ::core::mem::transmute_copy(&ppnewrange), ::core::mem::transmute_copy(&pfplayable)).into()
        }
        unsafe extern "system" fn Play<Impl: ITfFnPlayBack_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Play(::core::mem::transmute(&prange)).into()
        }
        Self {
            base: ITfFunction_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            QueryRange: QueryRange::<Impl, IMPL_OFFSET>,
            Play: Play::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfFnPlayBack as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfFnPropertyUIStatus_Impl: Sized + ITfFunction_Impl {
    fn GetStatus(&mut self, refguidprop: *const ::windows::core::GUID) -> ::windows::core::Result<u32>;
    fn SetStatus(&mut self, refguidprop: *const ::windows::core::GUID, dw: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfFnPropertyUIStatus_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFnPropertyUIStatus_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfFnPropertyUIStatus_Vtbl {
        unsafe extern "system" fn GetStatus<Impl: ITfFnPropertyUIStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, refguidprop: *const ::windows::core::GUID, pdw: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus(::core::mem::transmute_copy(&refguidprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdw = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStatus<Impl: ITfFnPropertyUIStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, refguidprop: *const ::windows::core::GUID, dw: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStatus(::core::mem::transmute_copy(&refguidprop), ::core::mem::transmute_copy(&dw)).into()
        }
        Self {
            base: ITfFunction_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetStatus: GetStatus::<Impl, IMPL_OFFSET>,
            SetStatus: SetStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfFnPropertyUIStatus as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfFnReconversion_Impl: Sized + ITfFunction_Impl {
    fn QueryRange(&mut self, prange: ::core::option::Option<ITfRange>, ppnewrange: *mut ::core::option::Option<ITfRange>, pfconvertable: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetReconversion(&mut self, prange: ::core::option::Option<ITfRange>) -> ::windows::core::Result<ITfCandidateList>;
    fn Reconvert(&mut self, prange: ::core::option::Option<ITfRange>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfFnReconversion_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFnReconversion_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfFnReconversion_Vtbl {
        unsafe extern "system" fn QueryRange<Impl: ITfFnReconversion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, ppnewrange: *mut ::windows::core::RawPtr, pfconvertable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryRange(::core::mem::transmute(&prange), ::core::mem::transmute_copy(&ppnewrange), ::core::mem::transmute_copy(&pfconvertable)).into()
        }
        unsafe extern "system" fn GetReconversion<Impl: ITfFnReconversion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, ppcandlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetReconversion(::core::mem::transmute(&prange)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcandlist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reconvert<Impl: ITfFnReconversion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reconvert(::core::mem::transmute(&prange)).into()
        }
        Self {
            base: ITfFunction_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait ITfFnSearchCandidateProvider_Impl: Sized + ITfFunction_Impl {
    fn GetSearchCandidates(&mut self, bstrquery: super::super::Foundation::BSTR, bstrapplicationid: super::super::Foundation::BSTR) -> ::windows::core::Result<ITfCandidateList>;
    fn SetResult(&mut self, bstrquery: super::super::Foundation::BSTR, bstrapplicationid: super::super::Foundation::BSTR, bstrresult: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfFnSearchCandidateProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFnSearchCandidateProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfFnSearchCandidateProvider_Vtbl {
        unsafe extern "system" fn GetSearchCandidates<Impl: ITfFnSearchCandidateProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrapplicationid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pplist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSearchCandidates(::core::mem::transmute_copy(&bstrquery), ::core::mem::transmute_copy(&bstrapplicationid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pplist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetResult<Impl: ITfFnSearchCandidateProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrapplicationid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrresult: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetResult(::core::mem::transmute_copy(&bstrquery), ::core::mem::transmute_copy(&bstrapplicationid), ::core::mem::transmute_copy(&bstrresult)).into()
        }
        Self {
            base: ITfFunction_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetSearchCandidates: GetSearchCandidates::<Impl, IMPL_OFFSET>,
            SetResult: SetResult::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfFnSearchCandidateProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfFnShowHelp_Impl: Sized + ITfFunction_Impl {
    fn Show(&mut self, hwndparent: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfFnShowHelp_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFnShowHelp_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfFnShowHelp_Vtbl {
        unsafe extern "system" fn Show<Impl: ITfFnShowHelp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Show(::core::mem::transmute_copy(&hwndparent)).into()
        }
        Self { base: ITfFunction_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Show: Show::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfFnShowHelp as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfFunction_Impl: Sized {
    fn GetDisplayName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfFunction_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFunction_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfFunction_Vtbl {
        unsafe extern "system" fn GetDisplayName<Impl: ITfFunction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
pub trait ITfFunctionProvider_Impl: Sized {
    fn GetType(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetDescription(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetFunction(&mut self, rguid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfFunctionProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfFunctionProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfFunctionProvider_Vtbl {
        unsafe extern "system" fn GetType<Impl: ITfFunctionProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetType() {
                ::core::result::Result::Ok(ok__) => {
                    *pguid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescription<Impl: ITfFunctionProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdesc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunction<Impl: ITfFunctionProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait ITfInputProcessorProfileActivationSink_Impl: Sized {
    fn OnActivated(&mut self, dwprofiletype: u32, langid: u16, clsid: *const ::windows::core::GUID, catid: *const ::windows::core::GUID, guidprofile: *const ::windows::core::GUID, hkl: HKL, dwflags: u32) -> ::windows::core::Result<()>;
}
impl ITfInputProcessorProfileActivationSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfInputProcessorProfileActivationSink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfInputProcessorProfileActivationSink_Vtbl {
        unsafe extern "system" fn OnActivated<Impl: ITfInputProcessorProfileActivationSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwprofiletype: u32, langid: u16, clsid: *const ::windows::core::GUID, catid: *const ::windows::core::GUID, guidprofile: *const ::windows::core::GUID, hkl: HKL, dwflags: u32) -> ::windows::core::HRESULT {
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
pub trait ITfInputProcessorProfileMgr_Impl: Sized {
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
impl ITfInputProcessorProfileMgr_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfInputProcessorProfileMgr_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfInputProcessorProfileMgr_Vtbl {
        unsafe extern "system" fn ActivateProfile<Impl: ITfInputProcessorProfileMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwprofiletype: u32, langid: u16, clsid: *const ::windows::core::GUID, guidprofile: *const ::windows::core::GUID, hkl: HKL, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ActivateProfile(::core::mem::transmute_copy(&dwprofiletype), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&clsid), ::core::mem::transmute_copy(&guidprofile), ::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn DeactivateProfile<Impl: ITfInputProcessorProfileMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwprofiletype: u32, langid: u16, clsid: *const ::windows::core::GUID, guidprofile: *const ::windows::core::GUID, hkl: HKL, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeactivateProfile(::core::mem::transmute_copy(&dwprofiletype), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&clsid), ::core::mem::transmute_copy(&guidprofile), ::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetProfile<Impl: ITfInputProcessorProfileMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwprofiletype: u32, langid: u16, clsid: *const ::windows::core::GUID, guidprofile: *const ::windows::core::GUID, hkl: HKL, pprofile: *mut TF_INPUTPROCESSORPROFILE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProfile(::core::mem::transmute_copy(&dwprofiletype), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&clsid), ::core::mem::transmute_copy(&guidprofile), ::core::mem::transmute_copy(&hkl)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprofile = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumProfiles<Impl: ITfInputProcessorProfileMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumProfiles(::core::mem::transmute_copy(&langid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseInputProcessor<Impl: ITfInputProcessorProfileMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseInputProcessor(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn RegisterProfile<Impl: ITfInputProcessorProfileMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, pchdesc: super::super::Foundation::PWSTR, cchdesc: u32, pchiconfile: super::super::Foundation::PWSTR, cchfile: u32, uiconindex: u32, hklsubstitute: HKL, dwpreferredlayout: u32, benabledbydefault: super::super::Foundation::BOOL, dwflags: u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UnregisterProfile<Impl: ITfInputProcessorProfileMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterProfile(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&guidprofile), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetActiveProfile<Impl: ITfInputProcessorProfileMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, catid: *const ::windows::core::GUID, pprofile: *mut TF_INPUTPROCESSORPROFILE) -> ::windows::core::HRESULT {
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
pub trait ITfInputProcessorProfileSubstituteLayout_Impl: Sized {
    fn GetSubstituteKeyboardLayout(&mut self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID) -> ::windows::core::Result<HKL>;
}
impl ITfInputProcessorProfileSubstituteLayout_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfInputProcessorProfileSubstituteLayout_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfInputProcessorProfileSubstituteLayout_Vtbl {
        unsafe extern "system" fn GetSubstituteKeyboardLayout<Impl: ITfInputProcessorProfileSubstituteLayout_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, phkl: *mut HKL) -> ::windows::core::HRESULT {
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
pub trait ITfInputProcessorProfiles_Impl: Sized {
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
impl ITfInputProcessorProfiles_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfInputProcessorProfiles_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfInputProcessorProfiles_Vtbl {
        unsafe extern "system" fn Register<Impl: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Register(::core::mem::transmute_copy(&rclsid)).into()
        }
        unsafe extern "system" fn Unregister<Impl: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unregister(::core::mem::transmute_copy(&rclsid)).into()
        }
        unsafe extern "system" fn AddLanguageProfile<Impl: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, pchdesc: super::super::Foundation::PWSTR, cchdesc: u32, pchiconfile: super::super::Foundation::PWSTR, cchfile: u32, uiconindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddLanguageProfile(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&guidprofile), ::core::mem::transmute_copy(&pchdesc), ::core::mem::transmute_copy(&cchdesc), ::core::mem::transmute_copy(&pchiconfile), ::core::mem::transmute_copy(&cchfile), ::core::mem::transmute_copy(&uiconindex)).into()
        }
        unsafe extern "system" fn RemoveLanguageProfile<Impl: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveLanguageProfile(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&guidprofile)).into()
        }
        unsafe extern "system" fn EnumInputProcessorInfo<Impl: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumInputProcessorInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultLanguageProfile<Impl: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16, catid: *const ::windows::core::GUID, pclsid: *mut ::windows::core::GUID, pguidprofile: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDefaultLanguageProfile(::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&catid), ::core::mem::transmute_copy(&pclsid), ::core::mem::transmute_copy(&pguidprofile)).into()
        }
        unsafe extern "system" fn SetDefaultLanguageProfile<Impl: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16, rclsid: *const ::windows::core::GUID, guidprofiles: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultLanguageProfile(::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&guidprofiles)).into()
        }
        unsafe extern "system" fn ActivateLanguageProfile<Impl: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofiles: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ActivateLanguageProfile(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&guidprofiles)).into()
        }
        unsafe extern "system" fn GetActiveLanguageProfile<Impl: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, plangid: *mut u16, pguidprofile: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetActiveLanguageProfile(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&plangid), ::core::mem::transmute_copy(&pguidprofile)).into()
        }
        unsafe extern "system" fn GetLanguageProfileDescription<Impl: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, pbstrprofile: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLanguageProfileDescription(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&guidprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrprofile = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentLanguage<Impl: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plangid: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentLanguage() {
                ::core::result::Result::Ok(ok__) => {
                    *plangid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChangeCurrentLanguage<Impl: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ChangeCurrentLanguage(::core::mem::transmute_copy(&langid)).into()
        }
        unsafe extern "system" fn GetLanguageList<Impl: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplangid: *mut *mut u16, pulcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLanguageList(::core::mem::transmute_copy(&pplangid), ::core::mem::transmute_copy(&pulcount)).into()
        }
        unsafe extern "system" fn EnumLanguageProfiles<Impl: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumLanguageProfiles(::core::mem::transmute_copy(&langid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableLanguageProfile<Impl: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableLanguageProfile(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&guidprofile), ::core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn IsEnabledLanguageProfile<Impl: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, pfenable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEnabledLanguageProfile(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&guidprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfenable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableLanguageProfileByDefault<Impl: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableLanguageProfileByDefault(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&guidprofile), ::core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn SubstituteKeyboardLayout<Impl: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, hkl: HKL) -> ::windows::core::HRESULT {
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
pub trait ITfInputProcessorProfilesEx_Impl: Sized + ITfInputProcessorProfiles_Impl {
    fn SetLanguageProfileDisplayName(&mut self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, pchfile: super::super::Foundation::PWSTR, cchfile: u32, uresid: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ITfInputProcessorProfilesEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfInputProcessorProfilesEx_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfInputProcessorProfilesEx_Vtbl {
        unsafe extern "system" fn SetLanguageProfileDisplayName<Impl: ITfInputProcessorProfilesEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, pchfile: super::super::Foundation::PWSTR, cchfile: u32, uresid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLanguageProfileDisplayName(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&guidprofile), ::core::mem::transmute_copy(&pchfile), ::core::mem::transmute_copy(&cchfile), ::core::mem::transmute_copy(&uresid)).into()
        }
        Self {
            base: ITfInputProcessorProfiles_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetLanguageProfileDisplayName: SetLanguageProfileDisplayName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfInputProcessorProfilesEx as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfInputScope_Impl: Sized {
    fn GetInputScopes(&mut self, pprginputscopes: *mut *mut InputScope, pccount: *mut u32) -> ::windows::core::Result<()>;
    fn GetPhrase(&mut self, ppbstrphrases: *mut *mut super::super::Foundation::BSTR, pccount: *mut u32) -> ::windows::core::Result<()>;
    fn GetRegularExpression(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetSRGS(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetXML(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfInputScope_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfInputScope_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfInputScope_Vtbl {
        unsafe extern "system" fn GetInputScopes<Impl: ITfInputScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprginputscopes: *mut *mut InputScope, pccount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInputScopes(::core::mem::transmute_copy(&pprginputscopes), ::core::mem::transmute_copy(&pccount)).into()
        }
        unsafe extern "system" fn GetPhrase<Impl: ITfInputScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbstrphrases: *mut *mut super::super::Foundation::BSTR, pccount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPhrase(::core::mem::transmute_copy(&ppbstrphrases), ::core::mem::transmute_copy(&pccount)).into()
        }
        unsafe extern "system" fn GetRegularExpression<Impl: ITfInputScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrregexp: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRegularExpression() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrregexp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSRGS<Impl: ITfInputScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsrgs: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSRGS() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrsrgs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXML<Impl: ITfInputScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrxml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
pub trait ITfInputScope2_Impl: Sized + ITfInputScope_Impl {
    fn EnumWordList(&mut self) -> ::windows::core::Result<super::super::System::Com::IEnumString>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ITfInputScope2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfInputScope2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfInputScope2_Vtbl {
        unsafe extern "system" fn EnumWordList<Impl: ITfInputScope2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumstring: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumWordList() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumstring = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ITfInputScope_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), EnumWordList: EnumWordList::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfInputScope2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ITfInsertAtSelection_Impl: Sized {
    fn InsertTextAtSelection(&mut self, ec: u32, dwflags: INSERT_TEXT_AT_SELECTION_FLAGS, pchtext: super::super::Foundation::PWSTR, cch: i32) -> ::windows::core::Result<ITfRange>;
    fn InsertEmbeddedAtSelection(&mut self, ec: u32, dwflags: u32, pdataobject: ::core::option::Option<super::super::System::Com::IDataObject>) -> ::windows::core::Result<ITfRange>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ITfInsertAtSelection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfInsertAtSelection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfInsertAtSelection_Vtbl {
        unsafe extern "system" fn InsertTextAtSelection<Impl: ITfInsertAtSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, dwflags: INSERT_TEXT_AT_SELECTION_FLAGS, pchtext: super::super::Foundation::PWSTR, cch: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InsertTextAtSelection(::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pchtext), ::core::mem::transmute_copy(&cch)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertEmbeddedAtSelection<Impl: ITfInsertAtSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, dwflags: u32, pdataobject: ::windows::core::RawPtr, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ITfIntegratableCandidateListUIElement_Impl: Sized {
    fn SetIntegrationStyle(&mut self, guidintegrationstyle: ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetSelectionStyle(&mut self) -> ::windows::core::Result<TfIntegratableCandidateListSelectionStyle>;
    fn OnKeyDown(&mut self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn ShowCandidateNumbers(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn FinalizeExactCompositionString(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfIntegratableCandidateListUIElement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfIntegratableCandidateListUIElement_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfIntegratableCandidateListUIElement_Vtbl {
        unsafe extern "system" fn SetIntegrationStyle<Impl: ITfIntegratableCandidateListUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidintegrationstyle: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIntegrationStyle(::core::mem::transmute_copy(&guidintegrationstyle)).into()
        }
        unsafe extern "system" fn GetSelectionStyle<Impl: ITfIntegratableCandidateListUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptfselectionstyle: *mut TfIntegratableCandidateListSelectionStyle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSelectionStyle() {
                ::core::result::Result::Ok(ok__) => {
                    *ptfselectionstyle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnKeyDown<Impl: ITfIntegratableCandidateListUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnKeyDown(::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfeaten = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowCandidateNumbers<Impl: ITfIntegratableCandidateListUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfshow: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowCandidateNumbers() {
                ::core::result::Result::Ok(ok__) => {
                    *pfshow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FinalizeExactCompositionString<Impl: ITfIntegratableCandidateListUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait ITfKeyEventSink_Impl: Sized {
    fn OnSetFocus(&mut self, fforeground: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn OnTestKeyDown(&mut self, pic: ::core::option::Option<ITfContext>, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn OnTestKeyUp(&mut self, pic: ::core::option::Option<ITfContext>, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn OnKeyDown(&mut self, pic: ::core::option::Option<ITfContext>, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn OnKeyUp(&mut self, pic: ::core::option::Option<ITfContext>, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn OnPreservedKey(&mut self, pic: ::core::option::Option<ITfContext>, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfKeyEventSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfKeyEventSink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfKeyEventSink_Vtbl {
        unsafe extern "system" fn OnSetFocus<Impl: ITfKeyEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fforeground: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnSetFocus(::core::mem::transmute_copy(&fforeground)).into()
        }
        unsafe extern "system" fn OnTestKeyDown<Impl: ITfKeyEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnTestKeyDown(::core::mem::transmute(&pic), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfeaten = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnTestKeyUp<Impl: ITfKeyEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnTestKeyUp(::core::mem::transmute(&pic), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfeaten = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnKeyDown<Impl: ITfKeyEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnKeyDown(::core::mem::transmute(&pic), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfeaten = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnKeyUp<Impl: ITfKeyEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnKeyUp(::core::mem::transmute(&pic), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfeaten = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnPreservedKey<Impl: ITfKeyEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr, rguid: *const ::windows::core::GUID, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
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
pub trait ITfKeyTraceEventSink_Impl: Sized {
    fn OnKeyTraceDown(&mut self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<()>;
    fn OnKeyTraceUp(&mut self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfKeyTraceEventSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfKeyTraceEventSink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfKeyTraceEventSink_Vtbl {
        unsafe extern "system" fn OnKeyTraceDown<Impl: ITfKeyTraceEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnKeyTraceDown(::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn OnKeyTraceUp<Impl: ITfKeyTraceEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
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
pub trait ITfKeystrokeMgr_Impl: Sized {
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
impl ITfKeystrokeMgr_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfKeystrokeMgr_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfKeystrokeMgr_Vtbl {
        unsafe extern "system" fn AdviseKeyEventSink<Impl: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tid: u32, psink: ::windows::core::RawPtr, fforeground: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AdviseKeyEventSink(::core::mem::transmute_copy(&tid), ::core::mem::transmute(&psink), ::core::mem::transmute_copy(&fforeground)).into()
        }
        unsafe extern "system" fn UnadviseKeyEventSink<Impl: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnadviseKeyEventSink(::core::mem::transmute_copy(&tid)).into()
        }
        unsafe extern "system" fn GetForeground<Impl: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForeground() {
                ::core::result::Result::Ok(ok__) => {
                    *pclsid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TestKeyDown<Impl: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TestKeyDown(::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfeaten = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TestKeyUp<Impl: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TestKeyUp(::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfeaten = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyDown<Impl: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyDown(::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfeaten = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyUp<Impl: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyUp(::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfeaten = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreservedKey<Impl: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr, pprekey: *const TF_PRESERVEDKEY, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPreservedKey(::core::mem::transmute(&pic), ::core::mem::transmute_copy(&pprekey)) {
                ::core::result::Result::Ok(ok__) => {
                    *pguid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPreservedKey<Impl: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, pprekey: *const TF_PRESERVEDKEY, pfregistered: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPreservedKey(::core::mem::transmute_copy(&rguid), ::core::mem::transmute_copy(&pprekey)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfregistered = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreserveKey<Impl: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tid: u32, rguid: *const ::windows::core::GUID, prekey: *const TF_PRESERVEDKEY, pchdesc: super::super::Foundation::PWSTR, cchdesc: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PreserveKey(::core::mem::transmute_copy(&tid), ::core::mem::transmute_copy(&rguid), ::core::mem::transmute_copy(&prekey), ::core::mem::transmute_copy(&pchdesc), ::core::mem::transmute_copy(&cchdesc)).into()
        }
        unsafe extern "system" fn UnpreserveKey<Impl: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, pprekey: *const TF_PRESERVEDKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnpreserveKey(::core::mem::transmute_copy(&rguid), ::core::mem::transmute_copy(&pprekey)).into()
        }
        unsafe extern "system" fn SetPreservedKeyDescription<Impl: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, pchdesc: super::super::Foundation::PWSTR, cchdesc: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreservedKeyDescription(::core::mem::transmute_copy(&rguid), ::core::mem::transmute_copy(&pchdesc), ::core::mem::transmute_copy(&cchdesc)).into()
        }
        unsafe extern "system" fn GetPreservedKeyDescription<Impl: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, pbstrdesc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPreservedKeyDescription(::core::mem::transmute_copy(&rguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SimulatePreservedKey<Impl: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr, rguid: *const ::windows::core::GUID, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
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
pub trait ITfLMLattice_Impl: Sized {
    fn QueryType(&mut self, rguidtype: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn EnumLatticeElements(&mut self, dwframestart: u32, rguidtype: *const ::windows::core::GUID) -> ::windows::core::Result<IEnumTfLatticeElements>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfLMLattice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfLMLattice_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfLMLattice_Vtbl {
        unsafe extern "system" fn QueryType<Impl: ITfLMLattice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidtype: *const ::windows::core::GUID, pfsupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryType(::core::mem::transmute_copy(&rguidtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfsupported = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumLatticeElements<Impl: ITfLMLattice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwframestart: u32, rguidtype: *const ::windows::core::GUID, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ITfLangBarEventSink_Impl: Sized {
    fn OnSetFocus(&mut self, dwthreadid: u32) -> ::windows::core::Result<()>;
    fn OnThreadTerminate(&mut self, dwthreadid: u32) -> ::windows::core::Result<()>;
    fn OnThreadItemChange(&mut self, dwthreadid: u32) -> ::windows::core::Result<()>;
    fn OnModalInput(&mut self, dwthreadid: u32, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<()>;
    fn ShowFloating(&mut self, dwflags: u32) -> ::windows::core::Result<()>;
    fn GetItemFloatingRect(&mut self, dwthreadid: u32, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::RECT>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfLangBarEventSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfLangBarEventSink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfLangBarEventSink_Vtbl {
        unsafe extern "system" fn OnSetFocus<Impl: ITfLangBarEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwthreadid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnSetFocus(::core::mem::transmute_copy(&dwthreadid)).into()
        }
        unsafe extern "system" fn OnThreadTerminate<Impl: ITfLangBarEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwthreadid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnThreadTerminate(::core::mem::transmute_copy(&dwthreadid)).into()
        }
        unsafe extern "system" fn OnThreadItemChange<Impl: ITfLangBarEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwthreadid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnThreadItemChange(::core::mem::transmute_copy(&dwthreadid)).into()
        }
        unsafe extern "system" fn OnModalInput<Impl: ITfLangBarEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwthreadid: u32, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnModalInput(::core::mem::transmute_copy(&dwthreadid), ::core::mem::transmute_copy(&umsg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn ShowFloating<Impl: ITfLangBarEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowFloating(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetItemFloatingRect<Impl: ITfLangBarEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwthreadid: u32, rguid: *const ::windows::core::GUID, prc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
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
pub trait ITfLangBarItem_Impl: Sized {
    fn GetInfo(&mut self) -> ::windows::core::Result<TF_LANGBARITEMINFO>;
    fn GetStatus(&mut self) -> ::windows::core::Result<u32>;
    fn Show(&mut self, fshow: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetTooltipString(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfLangBarItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfLangBarItem_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfLangBarItem_Vtbl {
        unsafe extern "system" fn GetInfo<Impl: ITfLangBarItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *mut TF_LANGBARITEMINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *pinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: ITfLangBarItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Show<Impl: ITfLangBarItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Show(::core::mem::transmute_copy(&fshow)).into()
        }
        unsafe extern "system" fn GetTooltipString<Impl: ITfLangBarItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtooltip: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
pub trait ITfLangBarItemBalloon_Impl: Sized + ITfLangBarItem_Impl {
    fn OnClick(&mut self, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn GetPreferredSize(&mut self, pszdefault: *const super::super::Foundation::SIZE) -> ::windows::core::Result<super::super::Foundation::SIZE>;
    fn GetBalloonInfo(&mut self) -> ::windows::core::Result<TF_LBBALLOONINFO>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfLangBarItemBalloon_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfLangBarItemBalloon_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfLangBarItemBalloon_Vtbl {
        unsafe extern "system" fn OnClick<Impl: ITfLangBarItemBalloon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnClick(::core::mem::transmute_copy(&click), ::core::mem::transmute_copy(&pt), ::core::mem::transmute_copy(&prcarea)).into()
        }
        unsafe extern "system" fn GetPreferredSize<Impl: ITfLangBarItemBalloon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdefault: *const super::super::Foundation::SIZE, psz: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPreferredSize(::core::mem::transmute_copy(&pszdefault)) {
                ::core::result::Result::Ok(ok__) => {
                    *psz = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBalloonInfo<Impl: ITfLangBarItemBalloon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *mut TF_LBBALLOONINFO) -> ::windows::core::HRESULT {
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
            base: ITfLangBarItem_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait ITfLangBarItemBitmap_Impl: Sized + ITfLangBarItem_Impl {
    fn OnClick(&mut self, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn GetPreferredSize(&mut self, pszdefault: *const super::super::Foundation::SIZE) -> ::windows::core::Result<super::super::Foundation::SIZE>;
    fn DrawBitmap(&mut self, bmwidth: i32, bmheight: i32, dwflags: u32, phbmp: *mut super::super::Graphics::Gdi::HBITMAP, phbmpmask: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ITfLangBarItemBitmap_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfLangBarItemBitmap_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfLangBarItemBitmap_Vtbl {
        unsafe extern "system" fn OnClick<Impl: ITfLangBarItemBitmap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnClick(::core::mem::transmute_copy(&click), ::core::mem::transmute_copy(&pt), ::core::mem::transmute_copy(&prcarea)).into()
        }
        unsafe extern "system" fn GetPreferredSize<Impl: ITfLangBarItemBitmap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdefault: *const super::super::Foundation::SIZE, psz: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPreferredSize(::core::mem::transmute_copy(&pszdefault)) {
                ::core::result::Result::Ok(ok__) => {
                    *psz = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawBitmap<Impl: ITfLangBarItemBitmap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmwidth: i32, bmheight: i32, dwflags: u32, phbmp: *mut super::super::Graphics::Gdi::HBITMAP, phbmpmask: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DrawBitmap(::core::mem::transmute_copy(&bmwidth), ::core::mem::transmute_copy(&bmheight), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&phbmp), ::core::mem::transmute_copy(&phbmpmask)).into()
        }
        Self {
            base: ITfLangBarItem_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait ITfLangBarItemBitmapButton_Impl: Sized + ITfLangBarItem_Impl {
    fn OnClick(&mut self, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn InitMenu(&mut self, pmenu: ::core::option::Option<ITfMenu>) -> ::windows::core::Result<()>;
    fn OnMenuSelect(&mut self, wid: u32) -> ::windows::core::Result<()>;
    fn GetPreferredSize(&mut self, pszdefault: *const super::super::Foundation::SIZE) -> ::windows::core::Result<super::super::Foundation::SIZE>;
    fn DrawBitmap(&mut self, bmwidth: i32, bmheight: i32, dwflags: u32, phbmp: *mut super::super::Graphics::Gdi::HBITMAP, phbmpmask: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::Result<()>;
    fn GetText(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ITfLangBarItemBitmapButton_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfLangBarItemBitmapButton_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfLangBarItemBitmapButton_Vtbl {
        unsafe extern "system" fn OnClick<Impl: ITfLangBarItemBitmapButton_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnClick(::core::mem::transmute_copy(&click), ::core::mem::transmute_copy(&pt), ::core::mem::transmute_copy(&prcarea)).into()
        }
        unsafe extern "system" fn InitMenu<Impl: ITfLangBarItemBitmapButton_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmenu: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitMenu(::core::mem::transmute(&pmenu)).into()
        }
        unsafe extern "system" fn OnMenuSelect<Impl: ITfLangBarItemBitmapButton_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnMenuSelect(::core::mem::transmute_copy(&wid)).into()
        }
        unsafe extern "system" fn GetPreferredSize<Impl: ITfLangBarItemBitmapButton_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdefault: *const super::super::Foundation::SIZE, psz: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPreferredSize(::core::mem::transmute_copy(&pszdefault)) {
                ::core::result::Result::Ok(ok__) => {
                    *psz = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawBitmap<Impl: ITfLangBarItemBitmapButton_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmwidth: i32, bmheight: i32, dwflags: u32, phbmp: *mut super::super::Graphics::Gdi::HBITMAP, phbmpmask: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DrawBitmap(::core::mem::transmute_copy(&bmwidth), ::core::mem::transmute_copy(&bmheight), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&phbmp), ::core::mem::transmute_copy(&phbmpmask)).into()
        }
        unsafe extern "system" fn GetText<Impl: ITfLangBarItemBitmapButton_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            base: ITfLangBarItem_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait ITfLangBarItemButton_Impl: Sized + ITfLangBarItem_Impl {
    fn OnClick(&mut self, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn InitMenu(&mut self, pmenu: ::core::option::Option<ITfMenu>) -> ::windows::core::Result<()>;
    fn OnMenuSelect(&mut self, wid: u32) -> ::windows::core::Result<()>;
    fn GetIcon(&mut self) -> ::windows::core::Result<super::WindowsAndMessaging::HICON>;
    fn GetText(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ITfLangBarItemButton_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfLangBarItemButton_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfLangBarItemButton_Vtbl {
        unsafe extern "system" fn OnClick<Impl: ITfLangBarItemButton_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnClick(::core::mem::transmute_copy(&click), ::core::mem::transmute_copy(&pt), ::core::mem::transmute_copy(&prcarea)).into()
        }
        unsafe extern "system" fn InitMenu<Impl: ITfLangBarItemButton_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmenu: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitMenu(::core::mem::transmute(&pmenu)).into()
        }
        unsafe extern "system" fn OnMenuSelect<Impl: ITfLangBarItemButton_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnMenuSelect(::core::mem::transmute_copy(&wid)).into()
        }
        unsafe extern "system" fn GetIcon<Impl: ITfLangBarItemButton_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phicon: *mut super::WindowsAndMessaging::HICON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIcon() {
                ::core::result::Result::Ok(ok__) => {
                    *phicon = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetText<Impl: ITfLangBarItemButton_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            base: ITfLangBarItem_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait ITfLangBarItemMgr_Impl: Sized {
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
impl ITfLangBarItemMgr_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfLangBarItemMgr_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfLangBarItemMgr_Vtbl {
        unsafe extern "system" fn EnumItems<Impl: ITfLangBarItemMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumItems() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItem<Impl: ITfLangBarItemMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItem(::core::mem::transmute_copy(&rguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddItem<Impl: ITfLangBarItemMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddItem(::core::mem::transmute(&punk)).into()
        }
        unsafe extern "system" fn RemoveItem<Impl: ITfLangBarItemMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveItem(::core::mem::transmute(&punk)).into()
        }
        unsafe extern "system" fn AdviseItemSink<Impl: ITfLangBarItemMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: ::windows::core::RawPtr, pdwcookie: *mut u32, rguiditem: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AdviseItemSink(::core::mem::transmute(&punk), ::core::mem::transmute_copy(&pdwcookie), ::core::mem::transmute_copy(&rguiditem)).into()
        }
        unsafe extern "system" fn UnadviseItemSink<Impl: ITfLangBarItemMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnadviseItemSink(::core::mem::transmute_copy(&dwcookie)).into()
        }
        unsafe extern "system" fn GetItemFloatingRect<Impl: ITfLangBarItemMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwthreadid: u32, rguid: *const ::windows::core::GUID, prc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemFloatingRect(::core::mem::transmute_copy(&dwthreadid), ::core::mem::transmute_copy(&rguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *prc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemsStatus<Impl: ITfLangBarItemMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, prgguid: *const ::windows::core::GUID, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetItemsStatus(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&prgguid), ::core::mem::transmute_copy(&pdwstatus)).into()
        }
        unsafe extern "system" fn GetItemNum<Impl: ITfLangBarItemMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemNum() {
                ::core::result::Result::Ok(ok__) => {
                    *pulcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItems<Impl: ITfLangBarItemMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, ppitem: *mut ::windows::core::RawPtr, pinfo: *mut TF_LANGBARITEMINFO, pdwstatus: *mut u32, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetItems(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&ppitem), ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&pdwstatus), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn AdviseItemsSink<Impl: ITfLangBarItemMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, ppunk: *const ::windows::core::RawPtr, pguiditem: *const ::windows::core::GUID, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AdviseItemsSink(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&ppunk), ::core::mem::transmute_copy(&pguiditem), ::core::mem::transmute_copy(&pdwcookie)).into()
        }
        unsafe extern "system" fn UnadviseItemsSink<Impl: ITfLangBarItemMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, pdwcookie: *const u32) -> ::windows::core::HRESULT {
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
pub trait ITfLangBarItemSink_Impl: Sized {
    fn OnUpdate(&mut self, dwflags: u32) -> ::windows::core::Result<()>;
}
impl ITfLangBarItemSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfLangBarItemSink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfLangBarItemSink_Vtbl {
        unsafe extern "system" fn OnUpdate<Impl: ITfLangBarItemSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
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
pub trait ITfLangBarMgr_Impl: Sized {
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
impl ITfLangBarMgr_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfLangBarMgr_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfLangBarMgr_Vtbl {
        unsafe extern "system" fn AdviseEventSink<Impl: ITfLangBarMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psink: ::windows::core::RawPtr, hwnd: super::super::Foundation::HWND, dwflags: u32, pdwcookie: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AdviseEventSink(::core::mem::transmute(&psink), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pdwcookie)).into()
        }
        unsafe extern "system" fn UnadviseEventSink<Impl: ITfLangBarMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnadviseEventSink(::core::mem::transmute_copy(&dwcookie)).into()
        }
        unsafe extern "system" fn GetThreadMarshalInterface<Impl: ITfLangBarMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwthreadid: u32, dwtype: u32, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetThreadMarshalInterface(::core::mem::transmute_copy(&dwthreadid), ::core::mem::transmute_copy(&dwtype), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThreadLangBarItemMgr<Impl: ITfLangBarMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwthreadid: u32, pplbi: *mut ::windows::core::RawPtr, pdwthreadid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetThreadLangBarItemMgr(::core::mem::transmute_copy(&dwthreadid), ::core::mem::transmute_copy(&pplbi), ::core::mem::transmute_copy(&pdwthreadid)).into()
        }
        unsafe extern "system" fn GetInputProcessorProfiles<Impl: ITfLangBarMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwthreadid: u32, ppaip: *mut ::windows::core::RawPtr, pdwthreadid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInputProcessorProfiles(::core::mem::transmute_copy(&dwthreadid), ::core::mem::transmute_copy(&ppaip), ::core::mem::transmute_copy(&pdwthreadid)).into()
        }
        unsafe extern "system" fn RestoreLastFocus<Impl: ITfLangBarMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwthreadid: *mut u32, fprev: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RestoreLastFocus(::core::mem::transmute_copy(&pdwthreadid), ::core::mem::transmute_copy(&fprev)).into()
        }
        unsafe extern "system" fn SetModalInput<Impl: ITfLangBarMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psink: ::windows::core::RawPtr, dwthreadid: u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetModalInput(::core::mem::transmute(&psink), ::core::mem::transmute_copy(&dwthreadid), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn ShowFloating<Impl: ITfLangBarMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowFloating(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetShowFloatingStatus<Impl: ITfLangBarMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
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
pub trait ITfLanguageProfileNotifySink_Impl: Sized {
    fn OnLanguageChange(&mut self, langid: u16) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn OnLanguageChanged(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfLanguageProfileNotifySink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfLanguageProfileNotifySink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfLanguageProfileNotifySink_Vtbl {
        unsafe extern "system" fn OnLanguageChange<Impl: ITfLanguageProfileNotifySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16, pfaccept: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnLanguageChange(::core::mem::transmute_copy(&langid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfaccept = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnLanguageChanged<Impl: ITfLanguageProfileNotifySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait ITfMSAAControl_Impl: Sized {
    fn SystemEnableMSAA(&mut self) -> ::windows::core::Result<()>;
    fn SystemDisableMSAA(&mut self) -> ::windows::core::Result<()>;
}
impl ITfMSAAControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfMSAAControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfMSAAControl_Vtbl {
        unsafe extern "system" fn SystemEnableMSAA<Impl: ITfMSAAControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SystemEnableMSAA().into()
        }
        unsafe extern "system" fn SystemDisableMSAA<Impl: ITfMSAAControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait ITfMenu_Impl: Sized {
    fn AddMenuItem(&mut self, uid: u32, dwflags: u32, hbmp: super::super::Graphics::Gdi::HBITMAP, hbmpmask: super::super::Graphics::Gdi::HBITMAP, pch: super::super::Foundation::PWSTR, cch: u32, ppmenu: *mut ::core::option::Option<ITfMenu>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ITfMenu_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfMenu_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfMenu_Vtbl {
        unsafe extern "system" fn AddMenuItem<Impl: ITfMenu_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uid: u32, dwflags: u32, hbmp: super::super::Graphics::Gdi::HBITMAP, hbmpmask: super::super::Graphics::Gdi::HBITMAP, pch: super::super::Foundation::PWSTR, cch: u32, ppmenu: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ITfMessagePump_Impl: Sized {
    fn PeekMessageA(&mut self, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetMessageA(&mut self, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn PeekMessageW(&mut self, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetMessageW(&mut self, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ITfMessagePump_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfMessagePump_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfMessagePump_Vtbl {
        unsafe extern "system" fn PeekMessageA<Impl: ITfMessagePump_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PeekMessageA(::core::mem::transmute_copy(&pmsg), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&wmsgfiltermin), ::core::mem::transmute_copy(&wmsgfiltermax), ::core::mem::transmute_copy(&wremovemsg), ::core::mem::transmute_copy(&pfresult)).into()
        }
        unsafe extern "system" fn GetMessageA<Impl: ITfMessagePump_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMessageA(::core::mem::transmute_copy(&pmsg), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&wmsgfiltermin), ::core::mem::transmute_copy(&wmsgfiltermax), ::core::mem::transmute_copy(&pfresult)).into()
        }
        unsafe extern "system" fn PeekMessageW<Impl: ITfMessagePump_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PeekMessageW(::core::mem::transmute_copy(&pmsg), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&wmsgfiltermin), ::core::mem::transmute_copy(&wmsgfiltermax), ::core::mem::transmute_copy(&wremovemsg), ::core::mem::transmute_copy(&pfresult)).into()
        }
        unsafe extern "system" fn GetMessageW<Impl: ITfMessagePump_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
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
pub trait ITfMouseSink_Impl: Sized {
    fn OnMouseEvent(&mut self, uedge: u32, uquadrant: u32, dwbtnstatus: u32) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfMouseSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfMouseSink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfMouseSink_Vtbl {
        unsafe extern "system" fn OnMouseEvent<Impl: ITfMouseSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uedge: u32, uquadrant: u32, dwbtnstatus: u32, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
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
pub trait ITfMouseTracker_Impl: Sized {
    fn AdviseMouseSink(&mut self, range: ::core::option::Option<ITfRange>, psink: ::core::option::Option<ITfMouseSink>) -> ::windows::core::Result<u32>;
    fn UnadviseMouseSink(&mut self, dwcookie: u32) -> ::windows::core::Result<()>;
}
impl ITfMouseTracker_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfMouseTracker_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfMouseTracker_Vtbl {
        unsafe extern "system" fn AdviseMouseSink<Impl: ITfMouseTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, range: ::windows::core::RawPtr, psink: ::windows::core::RawPtr, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdviseMouseSink(::core::mem::transmute(&range), ::core::mem::transmute(&psink)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnadviseMouseSink<Impl: ITfMouseTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT {
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
pub trait ITfMouseTrackerACP_Impl: Sized {
    fn AdviseMouseSink(&mut self, range: ::core::option::Option<ITfRangeACP>, psink: ::core::option::Option<ITfMouseSink>) -> ::windows::core::Result<u32>;
    fn UnadviseMouseSink(&mut self, dwcookie: u32) -> ::windows::core::Result<()>;
}
impl ITfMouseTrackerACP_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfMouseTrackerACP_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfMouseTrackerACP_Vtbl {
        unsafe extern "system" fn AdviseMouseSink<Impl: ITfMouseTrackerACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, range: ::windows::core::RawPtr, psink: ::windows::core::RawPtr, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdviseMouseSink(::core::mem::transmute(&range), ::core::mem::transmute(&psink)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnadviseMouseSink<Impl: ITfMouseTrackerACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT {
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
pub trait ITfPersistentPropertyLoaderACP_Impl: Sized {
    fn LoadProperty(&mut self, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP) -> ::windows::core::Result<super::super::System::Com::IStream>;
}
#[cfg(feature = "Win32_System_Com")]
impl ITfPersistentPropertyLoaderACP_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfPersistentPropertyLoaderACP_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfPersistentPropertyLoaderACP_Vtbl {
        unsafe extern "system" fn LoadProperty<Impl: ITfPersistentPropertyLoaderACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ITfPreservedKeyNotifySink_Impl: Sized {
    fn OnUpdated(&mut self, pprekey: *const TF_PRESERVEDKEY) -> ::windows::core::Result<()>;
}
impl ITfPreservedKeyNotifySink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfPreservedKeyNotifySink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfPreservedKeyNotifySink_Vtbl {
        unsafe extern "system" fn OnUpdated<Impl: ITfPreservedKeyNotifySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprekey: *const TF_PRESERVEDKEY) -> ::windows::core::HRESULT {
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
pub trait ITfProperty_Impl: Sized + ITfReadOnlyProperty_Impl {
    fn FindRange(&mut self, ec: u32, prange: ::core::option::Option<ITfRange>, pprange: *mut ::core::option::Option<ITfRange>, apos: TfAnchor) -> ::windows::core::Result<()>;
    fn SetValueStore(&mut self, ec: u32, prange: ::core::option::Option<ITfRange>, ppropstore: ::core::option::Option<ITfPropertyStore>) -> ::windows::core::Result<()>;
    fn SetValue(&mut self, ec: u32, prange: ::core::option::Option<ITfRange>, pvarvalue: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Clear(&mut self, ec: u32, prange: ::core::option::Option<ITfRange>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITfProperty_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfProperty_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfProperty_Vtbl {
        unsafe extern "system" fn FindRange<Impl: ITfProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, prange: ::windows::core::RawPtr, pprange: *mut ::windows::core::RawPtr, apos: TfAnchor) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FindRange(::core::mem::transmute_copy(&ec), ::core::mem::transmute(&prange), ::core::mem::transmute_copy(&pprange), ::core::mem::transmute_copy(&apos)).into()
        }
        unsafe extern "system" fn SetValueStore<Impl: ITfProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, prange: ::windows::core::RawPtr, ppropstore: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValueStore(::core::mem::transmute_copy(&ec), ::core::mem::transmute(&prange), ::core::mem::transmute(&ppropstore)).into()
        }
        unsafe extern "system" fn SetValue<Impl: ITfProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, prange: ::windows::core::RawPtr, pvarvalue: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&ec), ::core::mem::transmute(&prange), ::core::mem::transmute_copy(&pvarvalue)).into()
        }
        unsafe extern "system" fn Clear<Impl: ITfProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear(::core::mem::transmute_copy(&ec), ::core::mem::transmute(&prange)).into()
        }
        Self {
            base: ITfReadOnlyProperty_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait ITfPropertyStore_Impl: Sized {
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
impl ITfPropertyStore_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfPropertyStore_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfPropertyStore_Vtbl {
        unsafe extern "system" fn GetType<Impl: ITfPropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetType() {
                ::core::result::Result::Ok(ok__) => {
                    *pguid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataType<Impl: ITfPropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwreserved: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDataType() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwreserved = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetData<Impl: ITfPropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetData() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnTextUpdated<Impl: ITfPropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, prangenew: ::windows::core::RawPtr, pfaccept: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnTextUpdated(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&prangenew)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfaccept = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shrink<Impl: ITfPropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prangenew: ::windows::core::RawPtr, pffree: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Shrink(::core::mem::transmute(&prangenew)) {
                ::core::result::Result::Ok(ok__) => {
                    *pffree = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Divide<Impl: ITfPropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prangethis: ::windows::core::RawPtr, prangenew: ::windows::core::RawPtr, pppropstore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Divide(::core::mem::transmute(&prangethis), ::core::mem::transmute(&prangenew)) {
                ::core::result::Result::Ok(ok__) => {
                    *pppropstore = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: ITfPropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropstore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppropstore = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyRangeCreator<Impl: ITfPropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertyRangeCreator() {
                ::core::result::Result::Ok(ok__) => {
                    *pclsid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Serialize<Impl: ITfPropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, pcb: *mut u32) -> ::windows::core::HRESULT {
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
pub trait ITfQueryEmbedded_Impl: Sized {
    fn QueryInsertEmbedded(&mut self, pguidservice: *const ::windows::core::GUID, pformatetc: *const super::super::System::Com::FORMATETC) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ITfQueryEmbedded_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfQueryEmbedded_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfQueryEmbedded_Vtbl {
        unsafe extern "system" fn QueryInsertEmbedded<Impl: ITfQueryEmbedded_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidservice: *const ::windows::core::GUID, pformatetc: *const super::super::System::Com::FORMATETC, pfinsertable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
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
pub trait ITfRange_Impl: Sized {
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
impl ITfRange_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfRange_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfRange_Vtbl {
        unsafe extern "system" fn GetText<Impl: ITfRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, dwflags: u32, pchtext: super::super::Foundation::PWSTR, cchmax: u32, pcch: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetText(::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pchtext), ::core::mem::transmute_copy(&cchmax), ::core::mem::transmute_copy(&pcch)).into()
        }
        unsafe extern "system" fn SetText<Impl: ITfRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, dwflags: u32, pchtext: super::super::Foundation::PWSTR, cch: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetText(::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pchtext), ::core::mem::transmute_copy(&cch)).into()
        }
        unsafe extern "system" fn GetFormattedText<Impl: ITfRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, ppdataobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFormattedText(::core::mem::transmute_copy(&ec)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdataobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEmbedded<Impl: ITfRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, rguidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEmbedded(::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&rguidservice), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertEmbedded<Impl: ITfRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, dwflags: u32, pdataobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertEmbedded(::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pdataobject)).into()
        }
        unsafe extern "system" fn ShiftStart<Impl: ITfRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShiftStart(::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&cchreq), ::core::mem::transmute_copy(&pcch), ::core::mem::transmute_copy(&phalt)).into()
        }
        unsafe extern "system" fn ShiftEnd<Impl: ITfRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShiftEnd(::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&cchreq), ::core::mem::transmute_copy(&pcch), ::core::mem::transmute_copy(&phalt)).into()
        }
        unsafe extern "system" fn ShiftStartToRange<Impl: ITfRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, prange: ::windows::core::RawPtr, apos: TfAnchor) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShiftStartToRange(::core::mem::transmute_copy(&ec), ::core::mem::transmute(&prange), ::core::mem::transmute_copy(&apos)).into()
        }
        unsafe extern "system" fn ShiftEndToRange<Impl: ITfRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, prange: ::windows::core::RawPtr, apos: TfAnchor) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShiftEndToRange(::core::mem::transmute_copy(&ec), ::core::mem::transmute(&prange), ::core::mem::transmute_copy(&apos)).into()
        }
        unsafe extern "system" fn ShiftStartRegion<Impl: ITfRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, dir: TfShiftDir, pfnoregion: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShiftStartRegion(::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&dir)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfnoregion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShiftEndRegion<Impl: ITfRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, dir: TfShiftDir, pfnoregion: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShiftEndRegion(::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&dir)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfnoregion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEmpty<Impl: ITfRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, pfempty: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEmpty(::core::mem::transmute_copy(&ec)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfempty = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Collapse<Impl: ITfRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, apos: TfAnchor) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Collapse(::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&apos)).into()
        }
        unsafe extern "system" fn IsEqualStart<Impl: ITfRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, pwith: ::windows::core::RawPtr, apos: TfAnchor, pfequal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEqualStart(::core::mem::transmute_copy(&ec), ::core::mem::transmute(&pwith), ::core::mem::transmute_copy(&apos)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfequal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqualEnd<Impl: ITfRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, pwith: ::windows::core::RawPtr, apos: TfAnchor, pfequal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEqualEnd(::core::mem::transmute_copy(&ec), ::core::mem::transmute(&pwith), ::core::mem::transmute_copy(&apos)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfequal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareStart<Impl: ITfRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, pwith: ::windows::core::RawPtr, apos: TfAnchor, plresult: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompareStart(::core::mem::transmute_copy(&ec), ::core::mem::transmute(&pwith), ::core::mem::transmute_copy(&apos)) {
                ::core::result::Result::Ok(ok__) => {
                    *plresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareEnd<Impl: ITfRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, pwith: ::windows::core::RawPtr, apos: TfAnchor, plresult: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompareEnd(::core::mem::transmute_copy(&ec), ::core::mem::transmute(&pwith), ::core::mem::transmute_copy(&apos)) {
                ::core::result::Result::Ok(ok__) => {
                    *plresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdjustForInsert<Impl: ITfRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, cchinsert: u32, pfinsertok: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdjustForInsert(::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&cchinsert)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfinsertok = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGravity<Impl: ITfRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgstart: *mut TfGravity, pgend: *mut TfGravity) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetGravity(::core::mem::transmute_copy(&pgstart), ::core::mem::transmute_copy(&pgend)).into()
        }
        unsafe extern "system" fn SetGravity<Impl: ITfRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, gstart: TfGravity, gend: TfGravity) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGravity(::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&gstart), ::core::mem::transmute_copy(&gend)).into()
        }
        unsafe extern "system" fn Clone<Impl: ITfRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppclone = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContext<Impl: ITfRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ITfRangeACP_Impl: Sized + ITfRange_Impl {
    fn GetExtent(&mut self, pacpanchor: *mut i32, pcch: *mut i32) -> ::windows::core::Result<()>;
    fn SetExtent(&mut self, acpanchor: i32, cch: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ITfRangeACP_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfRangeACP_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfRangeACP_Vtbl {
        unsafe extern "system" fn GetExtent<Impl: ITfRangeACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pacpanchor: *mut i32, pcch: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetExtent(::core::mem::transmute_copy(&pacpanchor), ::core::mem::transmute_copy(&pcch)).into()
        }
        unsafe extern "system" fn SetExtent<Impl: ITfRangeACP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, acpanchor: i32, cch: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExtent(::core::mem::transmute_copy(&acpanchor), ::core::mem::transmute_copy(&cch)).into()
        }
        Self {
            base: ITfRange_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetExtent: GetExtent::<Impl, IMPL_OFFSET>,
            SetExtent: SetExtent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfRangeACP as ::windows::core::Interface>::IID
    }
}
pub trait ITfRangeBackup_Impl: Sized {
    fn Restore(&mut self, ec: u32, prange: ::core::option::Option<ITfRange>) -> ::windows::core::Result<()>;
}
impl ITfRangeBackup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfRangeBackup_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfRangeBackup_Vtbl {
        unsafe extern "system" fn Restore<Impl: ITfRangeBackup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ITfReadOnlyProperty_Impl: Sized {
    fn GetType(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn EnumRanges(&mut self, ec: u32, ppenum: *mut ::core::option::Option<IEnumTfRanges>, ptargetrange: ::core::option::Option<ITfRange>) -> ::windows::core::Result<()>;
    fn GetValue(&mut self, ec: u32, prange: ::core::option::Option<ITfRange>) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn GetContext(&mut self) -> ::windows::core::Result<ITfContext>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITfReadOnlyProperty_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfReadOnlyProperty_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfReadOnlyProperty_Vtbl {
        unsafe extern "system" fn GetType<Impl: ITfReadOnlyProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetType() {
                ::core::result::Result::Ok(ok__) => {
                    *pguid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumRanges<Impl: ITfReadOnlyProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, ppenum: *mut ::windows::core::RawPtr, ptargetrange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumRanges(::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&ppenum), ::core::mem::transmute(&ptargetrange)).into()
        }
        unsafe extern "system" fn GetValue<Impl: ITfReadOnlyProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ec: u32, prange: ::windows::core::RawPtr, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetValue(::core::mem::transmute_copy(&ec), ::core::mem::transmute(&prange)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContext<Impl: ITfReadOnlyProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ITfReadingInformationUIElement_Impl: Sized + ITfUIElement_Impl {
    fn GetUpdatedFlags(&mut self) -> ::windows::core::Result<u32>;
    fn GetContext(&mut self) -> ::windows::core::Result<ITfContext>;
    fn GetString(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetMaxReadingStringLength(&mut self) -> ::windows::core::Result<u32>;
    fn GetErrorIndex(&mut self) -> ::windows::core::Result<u32>;
    fn IsVerticalOrderPreferred(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfReadingInformationUIElement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfReadingInformationUIElement_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfReadingInformationUIElement_Vtbl {
        unsafe extern "system" fn GetUpdatedFlags<Impl: ITfReadingInformationUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUpdatedFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContext<Impl: ITfReadingInformationUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppic: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContext() {
                ::core::result::Result::Ok(ok__) => {
                    *ppic = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetString<Impl: ITfReadingInformationUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetString() {
                ::core::result::Result::Ok(ok__) => {
                    *pstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxReadingStringLength<Impl: ITfReadingInformationUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcchmax: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxReadingStringLength() {
                ::core::result::Result::Ok(ok__) => {
                    *pcchmax = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorIndex<Impl: ITfReadingInformationUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, perrorindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetErrorIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *perrorindex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsVerticalOrderPreferred<Impl: ITfReadingInformationUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfvertical: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
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
            base: ITfUIElement_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait ITfReverseConversion_Impl: Sized {
    fn DoReverseConversion(&mut self, lpstr: super::super::Foundation::PWSTR) -> ::windows::core::Result<ITfReverseConversionList>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfReverseConversion_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfReverseConversion_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfReverseConversion_Vtbl {
        unsafe extern "system" fn DoReverseConversion<Impl: ITfReverseConversion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpstr: super::super::Foundation::PWSTR, pplist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ITfReverseConversionList_Impl: Sized {
    fn GetLength(&mut self) -> ::windows::core::Result<u32>;
    fn GetString(&mut self, uindex: u32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfReverseConversionList_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfReverseConversionList_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfReverseConversionList_Vtbl {
        unsafe extern "system" fn GetLength<Impl: ITfReverseConversionList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLength() {
                ::core::result::Result::Ok(ok__) => {
                    *puindex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetString<Impl: ITfReverseConversionList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, pbstr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
pub trait ITfReverseConversionMgr_Impl: Sized {
    fn GetReverseConversion(&mut self, langid: u16, guidprofile: *const ::windows::core::GUID, dwflag: u32) -> ::windows::core::Result<ITfReverseConversion>;
}
impl ITfReverseConversionMgr_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfReverseConversionMgr_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfReverseConversionMgr_Vtbl {
        unsafe extern "system" fn GetReverseConversion<Impl: ITfReverseConversionMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16, guidprofile: *const ::windows::core::GUID, dwflag: u32, ppreverseconversion: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ITfSource_Impl: Sized {
    fn AdviseSink(&mut self, riid: *const ::windows::core::GUID, punk: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<u32>;
    fn UnadviseSink(&mut self, dwcookie: u32) -> ::windows::core::Result<()>;
}
impl ITfSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfSource_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfSource_Vtbl {
        unsafe extern "system" fn AdviseSink<Impl: ITfSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdviseSink(::core::mem::transmute_copy(&riid), ::core::mem::transmute(&punk)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnadviseSink<Impl: ITfSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT {
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
pub trait ITfSourceSingle_Impl: Sized {
    fn AdviseSingleSink(&mut self, tid: u32, riid: *const ::windows::core::GUID, punk: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn UnadviseSingleSink(&mut self, tid: u32, riid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl ITfSourceSingle_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfSourceSingle_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfSourceSingle_Vtbl {
        unsafe extern "system" fn AdviseSingleSink<Impl: ITfSourceSingle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tid: u32, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AdviseSingleSink(::core::mem::transmute_copy(&tid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute(&punk)).into()
        }
        unsafe extern "system" fn UnadviseSingleSink<Impl: ITfSourceSingle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tid: u32, riid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
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
pub trait ITfSpeechUIServer_Impl: Sized {
    fn Initialize(&mut self) -> ::windows::core::Result<()>;
    fn ShowUI(&mut self, fshow: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn UpdateBalloon(&mut self, style: TfLBBalloonStyle, pch: super::super::Foundation::PWSTR, cch: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfSpeechUIServer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfSpeechUIServer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfSpeechUIServer_Vtbl {
        unsafe extern "system" fn Initialize<Impl: ITfSpeechUIServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize().into()
        }
        unsafe extern "system" fn ShowUI<Impl: ITfSpeechUIServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowUI(::core::mem::transmute_copy(&fshow)).into()
        }
        unsafe extern "system" fn UpdateBalloon<Impl: ITfSpeechUIServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, style: TfLBBalloonStyle, pch: super::super::Foundation::PWSTR, cch: u32) -> ::windows::core::HRESULT {
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
pub trait ITfStatusSink_Impl: Sized {
    fn OnStatusChange(&mut self, pic: ::core::option::Option<ITfContext>, dwflags: u32) -> ::windows::core::Result<()>;
}
impl ITfStatusSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfStatusSink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfStatusSink_Vtbl {
        unsafe extern "system" fn OnStatusChange<Impl: ITfStatusSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnStatusChange(::core::mem::transmute(&pic), ::core::mem::transmute_copy(&dwflags)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnStatusChange: OnStatusChange::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfStatusSink as ::windows::core::Interface>::IID
    }
}
pub trait ITfSystemDeviceTypeLangBarItem_Impl: Sized {
    fn SetIconMode(&mut self, dwflags: LANG_BAR_ITEM_ICON_MODE_FLAGS) -> ::windows::core::Result<()>;
    fn GetIconMode(&mut self) -> ::windows::core::Result<u32>;
}
impl ITfSystemDeviceTypeLangBarItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfSystemDeviceTypeLangBarItem_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfSystemDeviceTypeLangBarItem_Vtbl {
        unsafe extern "system" fn SetIconMode<Impl: ITfSystemDeviceTypeLangBarItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: LANG_BAR_ITEM_ICON_MODE_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIconMode(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetIconMode<Impl: ITfSystemDeviceTypeLangBarItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
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
pub trait ITfSystemLangBarItem_Impl: Sized {
    fn SetIcon(&mut self, hicon: super::WindowsAndMessaging::HICON) -> ::windows::core::Result<()>;
    fn SetTooltipString(&mut self, pchtooltip: super::super::Foundation::PWSTR, cch: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ITfSystemLangBarItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfSystemLangBarItem_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfSystemLangBarItem_Vtbl {
        unsafe extern "system" fn SetIcon<Impl: ITfSystemLangBarItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hicon: super::WindowsAndMessaging::HICON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIcon(::core::mem::transmute_copy(&hicon)).into()
        }
        unsafe extern "system" fn SetTooltipString<Impl: ITfSystemLangBarItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchtooltip: super::super::Foundation::PWSTR, cch: u32) -> ::windows::core::HRESULT {
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
pub trait ITfSystemLangBarItemSink_Impl: Sized {
    fn InitMenu(&mut self, pmenu: ::core::option::Option<ITfMenu>) -> ::windows::core::Result<()>;
    fn OnMenuSelect(&mut self, wid: u32) -> ::windows::core::Result<()>;
}
impl ITfSystemLangBarItemSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfSystemLangBarItemSink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfSystemLangBarItemSink_Vtbl {
        unsafe extern "system" fn InitMenu<Impl: ITfSystemLangBarItemSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmenu: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitMenu(::core::mem::transmute(&pmenu)).into()
        }
        unsafe extern "system" fn OnMenuSelect<Impl: ITfSystemLangBarItemSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wid: u32) -> ::windows::core::HRESULT {
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
pub trait ITfSystemLangBarItemText_Impl: Sized {
    fn SetItemText(&mut self, pch: super::super::Foundation::PWSTR, cch: u32) -> ::windows::core::Result<()>;
    fn GetItemText(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfSystemLangBarItemText_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfSystemLangBarItemText_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfSystemLangBarItemText_Vtbl {
        unsafe extern "system" fn SetItemText<Impl: ITfSystemLangBarItemText_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pch: super::super::Foundation::PWSTR, cch: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetItemText(::core::mem::transmute_copy(&pch), ::core::mem::transmute_copy(&cch)).into()
        }
        unsafe extern "system" fn GetItemText<Impl: ITfSystemLangBarItemText_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
pub trait ITfTextEditSink_Impl: Sized {
    fn OnEndEdit(&mut self, pic: ::core::option::Option<ITfContext>, ecreadonly: u32, peditrecord: ::core::option::Option<ITfEditRecord>) -> ::windows::core::Result<()>;
}
impl ITfTextEditSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfTextEditSink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfTextEditSink_Vtbl {
        unsafe extern "system" fn OnEndEdit<Impl: ITfTextEditSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr, ecreadonly: u32, peditrecord: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnEndEdit(::core::mem::transmute(&pic), ::core::mem::transmute_copy(&ecreadonly), ::core::mem::transmute(&peditrecord)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnEndEdit: OnEndEdit::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfTextEditSink as ::windows::core::Interface>::IID
    }
}
pub trait ITfTextInputProcessor_Impl: Sized {
    fn Activate(&mut self, ptim: ::core::option::Option<ITfThreadMgr>, tid: u32) -> ::windows::core::Result<()>;
    fn Deactivate(&mut self) -> ::windows::core::Result<()>;
}
impl ITfTextInputProcessor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfTextInputProcessor_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfTextInputProcessor_Vtbl {
        unsafe extern "system" fn Activate<Impl: ITfTextInputProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptim: ::windows::core::RawPtr, tid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Activate(::core::mem::transmute(&ptim), ::core::mem::transmute_copy(&tid)).into()
        }
        unsafe extern "system" fn Deactivate<Impl: ITfTextInputProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait ITfTextInputProcessorEx_Impl: Sized + ITfTextInputProcessor_Impl {
    fn ActivateEx(&mut self, ptim: ::core::option::Option<ITfThreadMgr>, tid: u32, dwflags: u32) -> ::windows::core::Result<()>;
}
impl ITfTextInputProcessorEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfTextInputProcessorEx_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfTextInputProcessorEx_Vtbl {
        unsafe extern "system" fn ActivateEx<Impl: ITfTextInputProcessorEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptim: ::windows::core::RawPtr, tid: u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ActivateEx(::core::mem::transmute(&ptim), ::core::mem::transmute_copy(&tid), ::core::mem::transmute_copy(&dwflags)).into()
        }
        Self { base: ITfTextInputProcessor_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), ActivateEx: ActivateEx::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfTextInputProcessorEx as ::windows::core::Interface>::IID
    }
}
pub trait ITfTextLayoutSink_Impl: Sized {
    fn OnLayoutChange(&mut self, pic: ::core::option::Option<ITfContext>, lcode: TfLayoutCode, pview: ::core::option::Option<ITfContextView>) -> ::windows::core::Result<()>;
}
impl ITfTextLayoutSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfTextLayoutSink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfTextLayoutSink_Vtbl {
        unsafe extern "system" fn OnLayoutChange<Impl: ITfTextLayoutSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr, lcode: TfLayoutCode, pview: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnLayoutChange(::core::mem::transmute(&pic), ::core::mem::transmute_copy(&lcode), ::core::mem::transmute(&pview)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnLayoutChange: OnLayoutChange::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfTextLayoutSink as ::windows::core::Interface>::IID
    }
}
pub trait ITfThreadFocusSink_Impl: Sized {
    fn OnSetThreadFocus(&mut self) -> ::windows::core::Result<()>;
    fn OnKillThreadFocus(&mut self) -> ::windows::core::Result<()>;
}
impl ITfThreadFocusSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfThreadFocusSink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfThreadFocusSink_Vtbl {
        unsafe extern "system" fn OnSetThreadFocus<Impl: ITfThreadFocusSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnSetThreadFocus().into()
        }
        unsafe extern "system" fn OnKillThreadFocus<Impl: ITfThreadFocusSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait ITfThreadMgr_Impl: Sized {
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
impl ITfThreadMgr_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfThreadMgr_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfThreadMgr_Vtbl {
        unsafe extern "system" fn Activate<Impl: ITfThreadMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Activate() {
                ::core::result::Result::Ok(ok__) => {
                    *ptid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Deactivate<Impl: ITfThreadMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Deactivate().into()
        }
        unsafe extern "system" fn CreateDocumentMgr<Impl: ITfThreadMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdim: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDocumentMgr() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdim = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDocumentMgrs<Impl: ITfThreadMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumDocumentMgrs() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFocus<Impl: ITfThreadMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdimfocus: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFocus() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdimfocus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFocus<Impl: ITfThreadMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdimfocus: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFocus(::core::mem::transmute(&pdimfocus)).into()
        }
        unsafe extern "system" fn AssociateFocus<Impl: ITfThreadMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pdimnew: ::windows::core::RawPtr, ppdimprev: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AssociateFocus(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute(&pdimnew)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdimprev = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsThreadFocus<Impl: ITfThreadMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfthreadfocus: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsThreadFocus() {
                ::core::result::Result::Ok(ok__) => {
                    *pfthreadfocus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunctionProvider<Impl: ITfThreadMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, ppfuncprov: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFunctionProvider(::core::mem::transmute_copy(&clsid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppfuncprov = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumFunctionProviders<Impl: ITfThreadMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumFunctionProviders() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlobalCompartment<Impl: ITfThreadMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcompmgr: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ITfThreadMgr2_Impl: Sized {
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
impl ITfThreadMgr2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfThreadMgr2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfThreadMgr2_Vtbl {
        unsafe extern "system" fn Activate<Impl: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Activate() {
                ::core::result::Result::Ok(ok__) => {
                    *ptid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Deactivate<Impl: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Deactivate().into()
        }
        unsafe extern "system" fn CreateDocumentMgr<Impl: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdim: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDocumentMgr() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdim = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDocumentMgrs<Impl: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumDocumentMgrs() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFocus<Impl: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdimfocus: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFocus() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdimfocus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFocus<Impl: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdimfocus: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFocus(::core::mem::transmute(&pdimfocus)).into()
        }
        unsafe extern "system" fn IsThreadFocus<Impl: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfthreadfocus: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsThreadFocus() {
                ::core::result::Result::Ok(ok__) => {
                    *pfthreadfocus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunctionProvider<Impl: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, ppfuncprov: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFunctionProvider(::core::mem::transmute_copy(&clsid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppfuncprov = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumFunctionProviders<Impl: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumFunctionProviders() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlobalCompartment<Impl: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcompmgr: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGlobalCompartment() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcompmgr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivateEx<Impl: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptid: *mut u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ActivateEx(::core::mem::transmute_copy(&ptid), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetActiveFlags<Impl: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActiveFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *lpdwflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SuspendKeystrokeHandling<Impl: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SuspendKeystrokeHandling().into()
        }
        unsafe extern "system" fn ResumeKeystrokeHandling<Impl: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait ITfThreadMgrEventSink_Impl: Sized {
    fn OnInitDocumentMgr(&mut self, pdim: ::core::option::Option<ITfDocumentMgr>) -> ::windows::core::Result<()>;
    fn OnUninitDocumentMgr(&mut self, pdim: ::core::option::Option<ITfDocumentMgr>) -> ::windows::core::Result<()>;
    fn OnSetFocus(&mut self, pdimfocus: ::core::option::Option<ITfDocumentMgr>, pdimprevfocus: ::core::option::Option<ITfDocumentMgr>) -> ::windows::core::Result<()>;
    fn OnPushContext(&mut self, pic: ::core::option::Option<ITfContext>) -> ::windows::core::Result<()>;
    fn OnPopContext(&mut self, pic: ::core::option::Option<ITfContext>) -> ::windows::core::Result<()>;
}
impl ITfThreadMgrEventSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfThreadMgrEventSink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfThreadMgrEventSink_Vtbl {
        unsafe extern "system" fn OnInitDocumentMgr<Impl: ITfThreadMgrEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdim: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnInitDocumentMgr(::core::mem::transmute(&pdim)).into()
        }
        unsafe extern "system" fn OnUninitDocumentMgr<Impl: ITfThreadMgrEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdim: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnUninitDocumentMgr(::core::mem::transmute(&pdim)).into()
        }
        unsafe extern "system" fn OnSetFocus<Impl: ITfThreadMgrEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdimfocus: ::windows::core::RawPtr, pdimprevfocus: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnSetFocus(::core::mem::transmute(&pdimfocus), ::core::mem::transmute(&pdimprevfocus)).into()
        }
        unsafe extern "system" fn OnPushContext<Impl: ITfThreadMgrEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnPushContext(::core::mem::transmute(&pic)).into()
        }
        unsafe extern "system" fn OnPopContext<Impl: ITfThreadMgrEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ITfThreadMgrEx_Impl: Sized + ITfThreadMgr_Impl {
    fn ActivateEx(&mut self, ptid: *mut u32, dwflags: u32) -> ::windows::core::Result<()>;
    fn GetActiveFlags(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfThreadMgrEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfThreadMgrEx_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfThreadMgrEx_Vtbl {
        unsafe extern "system" fn ActivateEx<Impl: ITfThreadMgrEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptid: *mut u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ActivateEx(::core::mem::transmute_copy(&ptid), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetActiveFlags<Impl: ITfThreadMgrEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdwflags: *mut u32) -> ::windows::core::HRESULT {
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
            base: ITfThreadMgr_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ActivateEx: ActivateEx::<Impl, IMPL_OFFSET>,
            GetActiveFlags: GetActiveFlags::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfThreadMgrEx as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfToolTipUIElement_Impl: Sized + ITfUIElement_Impl {
    fn GetString(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfToolTipUIElement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfToolTipUIElement_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfToolTipUIElement_Vtbl {
        unsafe extern "system" fn GetString<Impl: ITfToolTipUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetString() {
                ::core::result::Result::Ok(ok__) => {
                    *pstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ITfUIElement_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetString: GetString::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfToolTipUIElement as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfTransitoryExtensionSink_Impl: Sized {
    fn OnTransitoryExtensionUpdated(&mut self, pic: ::core::option::Option<ITfContext>, ecreadonly: u32, presultrange: ::core::option::Option<ITfRange>, pcompositionrange: ::core::option::Option<ITfRange>) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfTransitoryExtensionSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfTransitoryExtensionSink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfTransitoryExtensionSink_Vtbl {
        unsafe extern "system" fn OnTransitoryExtensionUpdated<Impl: ITfTransitoryExtensionSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr, ecreadonly: u32, presultrange: ::windows::core::RawPtr, pcompositionrange: ::windows::core::RawPtr, pfdeleteresultrange: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
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
pub trait ITfTransitoryExtensionUIElement_Impl: Sized + ITfUIElement_Impl {
    fn GetDocumentMgr(&mut self) -> ::windows::core::Result<ITfDocumentMgr>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfTransitoryExtensionUIElement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfTransitoryExtensionUIElement_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfTransitoryExtensionUIElement_Vtbl {
        unsafe extern "system" fn GetDocumentMgr<Impl: ITfTransitoryExtensionUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdim: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocumentMgr() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdim = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ITfUIElement_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDocumentMgr: GetDocumentMgr::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITfTransitoryExtensionUIElement as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITfUIElement_Impl: Sized {
    fn GetDescription(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetGUID(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Show(&mut self, bshow: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn IsShown(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfUIElement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfUIElement_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfUIElement_Vtbl {
        unsafe extern "system" fn GetDescription<Impl: ITfUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGUID<Impl: ITfUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGUID() {
                ::core::result::Result::Ok(ok__) => {
                    *pguid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Show<Impl: ITfUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Show(::core::mem::transmute_copy(&bshow)).into()
        }
        unsafe extern "system" fn IsShown<Impl: ITfUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbshow: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
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
pub trait ITfUIElementMgr_Impl: Sized {
    fn BeginUIElement(&mut self, pelement: ::core::option::Option<ITfUIElement>, pbshow: *mut super::super::Foundation::BOOL, pdwuielementid: *mut u32) -> ::windows::core::Result<()>;
    fn UpdateUIElement(&mut self, dwuielementid: u32) -> ::windows::core::Result<()>;
    fn EndUIElement(&mut self, dwuielementid: u32) -> ::windows::core::Result<()>;
    fn GetUIElement(&mut self, dwuielementid: u32) -> ::windows::core::Result<ITfUIElement>;
    fn EnumUIElements(&mut self) -> ::windows::core::Result<IEnumTfUIElements>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfUIElementMgr_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfUIElementMgr_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfUIElementMgr_Vtbl {
        unsafe extern "system" fn BeginUIElement<Impl: ITfUIElementMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pelement: ::windows::core::RawPtr, pbshow: *mut super::super::Foundation::BOOL, pdwuielementid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginUIElement(::core::mem::transmute(&pelement), ::core::mem::transmute_copy(&pbshow), ::core::mem::transmute_copy(&pdwuielementid)).into()
        }
        unsafe extern "system" fn UpdateUIElement<Impl: ITfUIElementMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwuielementid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateUIElement(::core::mem::transmute_copy(&dwuielementid)).into()
        }
        unsafe extern "system" fn EndUIElement<Impl: ITfUIElementMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwuielementid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndUIElement(::core::mem::transmute_copy(&dwuielementid)).into()
        }
        unsafe extern "system" fn GetUIElement<Impl: ITfUIElementMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwuielementid: u32, ppelement: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUIElement(::core::mem::transmute_copy(&dwuielementid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppelement = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumUIElements<Impl: ITfUIElementMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ITfUIElementSink_Impl: Sized {
    fn BeginUIElement(&mut self, dwuielementid: u32, pbshow: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn UpdateUIElement(&mut self, dwuielementid: u32) -> ::windows::core::Result<()>;
    fn EndUIElement(&mut self, dwuielementid: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITfUIElementSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITfUIElementSink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITfUIElementSink_Vtbl {
        unsafe extern "system" fn BeginUIElement<Impl: ITfUIElementSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwuielementid: u32, pbshow: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginUIElement(::core::mem::transmute_copy(&dwuielementid), ::core::mem::transmute_copy(&pbshow)).into()
        }
        unsafe extern "system" fn UpdateUIElement<Impl: ITfUIElementSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwuielementid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateUIElement(::core::mem::transmute_copy(&dwuielementid)).into()
        }
        unsafe extern "system" fn EndUIElement<Impl: ITfUIElementSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwuielementid: u32) -> ::windows::core::HRESULT {
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
pub trait IUIManagerEventSink_Impl: Sized {
    fn OnWindowOpening(&mut self, prcbounds: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn OnWindowOpened(&mut self, prcbounds: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn OnWindowUpdating(&mut self, prcupdatedbounds: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn OnWindowUpdated(&mut self, prcupdatedbounds: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn OnWindowClosing(&mut self) -> ::windows::core::Result<()>;
    fn OnWindowClosed(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IUIManagerEventSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIManagerEventSink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIManagerEventSink_Vtbl {
        unsafe extern "system" fn OnWindowOpening<Impl: IUIManagerEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcbounds: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnWindowOpening(::core::mem::transmute_copy(&prcbounds)).into()
        }
        unsafe extern "system" fn OnWindowOpened<Impl: IUIManagerEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcbounds: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnWindowOpened(::core::mem::transmute_copy(&prcbounds)).into()
        }
        unsafe extern "system" fn OnWindowUpdating<Impl: IUIManagerEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcupdatedbounds: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnWindowUpdating(::core::mem::transmute_copy(&prcupdatedbounds)).into()
        }
        unsafe extern "system" fn OnWindowUpdated<Impl: IUIManagerEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcupdatedbounds: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnWindowUpdated(::core::mem::transmute_copy(&prcupdatedbounds)).into()
        }
        unsafe extern "system" fn OnWindowClosing<Impl: IUIManagerEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnWindowClosing().into()
        }
        unsafe extern "system" fn OnWindowClosed<Impl: IUIManagerEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IVersionInfo_Impl: Sized {
    fn GetSubcomponentCount(&mut self, ulsub: u32) -> ::windows::core::Result<u32>;
    fn GetImplementationID(&mut self, ulsub: u32) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetBuildVersion(&mut self, ulsub: u32, pdwmajor: *mut u32, pdwminor: *mut u32) -> ::windows::core::Result<()>;
    fn GetComponentDescription(&mut self, ulsub: u32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetInstanceDescription(&mut self, ulsub: u32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IVersionInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVersionInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVersionInfo_Vtbl {
        unsafe extern "system" fn GetSubcomponentCount<Impl: IVersionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulsub: u32, ulcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSubcomponentCount(::core::mem::transmute_copy(&ulsub)) {
                ::core::result::Result::Ok(ok__) => {
                    *ulcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImplementationID<Impl: IVersionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulsub: u32, implid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetImplementationID(::core::mem::transmute_copy(&ulsub)) {
                ::core::result::Result::Ok(ok__) => {
                    *implid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBuildVersion<Impl: IVersionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulsub: u32, pdwmajor: *mut u32, pdwminor: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBuildVersion(::core::mem::transmute_copy(&ulsub), ::core::mem::transmute_copy(&pdwmajor), ::core::mem::transmute_copy(&pdwminor)).into()
        }
        unsafe extern "system" fn GetComponentDescription<Impl: IVersionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulsub: u32, pimplstr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetComponentDescription(::core::mem::transmute_copy(&ulsub)) {
                ::core::result::Result::Ok(ok__) => {
                    *pimplstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInstanceDescription<Impl: IVersionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulsub: u32, pimplstr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
