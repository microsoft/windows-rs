pub trait IActiveIMEImpl: Sized {
    fn Inquire();
    fn ConversionList();
    fn Configure();
    fn Destroy();
    fn Escape();
    fn SetActiveContext();
    fn ProcessKey();
    fn Notify();
    fn Select();
    fn SetCompositionString();
    fn ToAsciiEx();
    fn RegisterWord();
    fn UnregisterWord();
    fn GetRegisterWordStyle();
    fn EnumRegisterWord();
    fn GetCodePageA();
    fn GetLangId();
}
impl ::windows::core::RuntimeName for IActiveIME {
    const NAME: &'static str = "Windows.Win32.UI.Input.Ime.IActiveIME";
}
impl IActiveIMEVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMEImpl, const OFFSET: isize>() -> IActiveIMEVtbl {
        unsafe extern "system" fn Inquire<Impl: IActiveIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsysteminfoflags: u32, pimeinfo: *mut IMEINFO, szwndclass: super::super::super::Foundation::PWSTR, pdwprivate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Inquire(dwsysteminfoflags, ::core::mem::transmute_copy(&pimeinfo), ::core::mem::transmute_copy(&szwndclass), ::core::mem::transmute_copy(&pdwprivate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConversionList<Impl: IActiveIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, szsource: super::super::super::Foundation::PWSTR, uflag: u32, ubuflen: u32, pdest: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConversionList(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), &*(&szsource as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), uflag, ubuflen, ::core::mem::transmute_copy(&pdest), ::core::mem::transmute_copy(&pucopied)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Configure<Impl: IActiveIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pregisterword: *const REGISTERWORDW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Configure(
                &*(&hkl as *const <super::super::TextServices::HKL as ::windows::core::Abi>::Abi as *const <super::super::TextServices::HKL as ::windows::core::DefaultType>::DefaultType),
                &*(&hwnd as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                dwmode,
                &*(&pregisterword as *const <REGISTERWORDW as ::windows::core::Abi>::Abi as *const <REGISTERWORDW as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Destroy<Impl: IActiveIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ureserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Destroy(ureserved) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Escape<Impl: IActiveIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Escape(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), uescape, &*(&pdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&plresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActiveContext<Impl: IActiveIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, fflag: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetActiveContext(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), &*(&fflag as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessKey<Impl: IActiveIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, uvirkey: u32, lparam: u32, pbkeystate: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessKey(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), uvirkey, lparam, pbkeystate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Notify<Impl: IActiveIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwaction: u32, dwindex: u32, dwvalue: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Notify(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), dwaction, dwindex, dwvalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Select<Impl: IActiveIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, fselect: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Select(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), &*(&fselect as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompositionString<Impl: IActiveIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCompositionString(
                &*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType),
                dwindex,
                &*(&pcomp as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                dwcomplen,
                &*(&pread as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                dwreadlen,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ToAsciiEx<Impl: IActiveIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uvirkey: u32, uscancode: u32, pbkeystate: *const u8, fustate: u32, himc: super::super::super::Globalization::HIMC, pdwtransbuf: *mut u32, pusize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ToAsciiEx(uvirkey, uscancode, pbkeystate, fustate, &*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwtransbuf), ::core::mem::transmute_copy(&pusize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterWord<Impl: IActiveIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szreading: super::super::super::Foundation::PWSTR, dwstyle: u32, szstring: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterWord(&*(&szreading as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dwstyle, &*(&szstring as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterWord<Impl: IActiveIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szreading: super::super::super::Foundation::PWSTR, dwstyle: u32, szstring: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnregisterWord(&*(&szreading as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dwstyle, &*(&szstring as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRegisterWordStyle<Impl: IActiveIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nitem: u32, pstylebuf: *mut STYLEBUFW, pubufsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRegisterWordStyle(nitem, ::core::mem::transmute_copy(&pstylebuf), ::core::mem::transmute_copy(&pubufsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumRegisterWord<Impl: IActiveIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szreading: super::super::super::Foundation::PWSTR, dwstyle: u32, szregister: super::super::super::Foundation::PWSTR, pdata: *const ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumRegisterWord(
                &*(&szreading as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                dwstyle,
                &*(&szregister as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppenum),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCodePageA<Impl: IActiveIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ucodepage: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCodePageA(::core::mem::transmute_copy(&ucodepage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLangId<Impl: IActiveIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plid: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLangId(::core::mem::transmute_copy(&plid)) {
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
            ::windows::core::GetRuntimeClassName::<IActiveIME>,
            ::windows::core::GetTrustLevel,
            Inquire::<Impl, OFFSET>,
            ConversionList::<Impl, OFFSET>,
            Configure::<Impl, OFFSET>,
            Destroy::<Impl, OFFSET>,
            Escape::<Impl, OFFSET>,
            SetActiveContext::<Impl, OFFSET>,
            ProcessKey::<Impl, OFFSET>,
            Notify::<Impl, OFFSET>,
            Select::<Impl, OFFSET>,
            SetCompositionString::<Impl, OFFSET>,
            ToAsciiEx::<Impl, OFFSET>,
            RegisterWord::<Impl, OFFSET>,
            UnregisterWord::<Impl, OFFSET>,
            GetRegisterWordStyle::<Impl, OFFSET>,
            EnumRegisterWord::<Impl, OFFSET>,
            GetCodePageA::<Impl, OFFSET>,
            GetLangId::<Impl, OFFSET>,
        )
    }
}
pub trait IActiveIME2Impl: Sized + IActiveIMEImpl {
    fn Sleep();
    fn Unsleep();
}
impl ::windows::core::RuntimeName for IActiveIME2 {
    const NAME: &'static str = "Windows.Win32.UI.Input.Ime.IActiveIME2";
}
impl IActiveIME2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIME2Impl, const OFFSET: isize>() -> IActiveIME2Vtbl {
        unsafe extern "system" fn Sleep<Impl: IActiveIME2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Sleep() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unsleep<Impl: IActiveIME2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fdead: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unsleep(&*(&fdead as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IActiveIME2>, ::windows::core::GetTrustLevel, Sleep::<Impl, OFFSET>, Unsleep::<Impl, OFFSET>)
    }
}
pub trait IActiveIMMAppImpl: Sized {
    fn AssociateContext();
    fn ConfigureIMEA();
    fn ConfigureIMEW();
    fn CreateContext();
    fn DestroyContext();
    fn EnumRegisterWordA();
    fn EnumRegisterWordW();
    fn EscapeA();
    fn EscapeW();
    fn GetCandidateListA();
    fn GetCandidateListW();
    fn GetCandidateListCountA();
    fn GetCandidateListCountW();
    fn GetCandidateWindow();
    fn GetCompositionFontA();
    fn GetCompositionFontW();
    fn GetCompositionStringA();
    fn GetCompositionStringW();
    fn GetCompositionWindow();
    fn GetContext();
    fn GetConversionListA();
    fn GetConversionListW();
    fn GetConversionStatus();
    fn GetDefaultIMEWnd();
    fn GetDescriptionA();
    fn GetDescriptionW();
    fn GetGuideLineA();
    fn GetGuideLineW();
    fn GetIMEFileNameA();
    fn GetIMEFileNameW();
    fn GetOpenStatus();
    fn GetProperty();
    fn GetRegisterWordStyleA();
    fn GetRegisterWordStyleW();
    fn GetStatusWindowPos();
    fn GetVirtualKey();
    fn InstallIMEA();
    fn InstallIMEW();
    fn IsIME();
    fn IsUIMessageA();
    fn IsUIMessageW();
    fn NotifyIME();
    fn RegisterWordA();
    fn RegisterWordW();
    fn ReleaseContext();
    fn SetCandidateWindow();
    fn SetCompositionFontA();
    fn SetCompositionFontW();
    fn SetCompositionStringA();
    fn SetCompositionStringW();
    fn SetCompositionWindow();
    fn SetConversionStatus();
    fn SetOpenStatus();
    fn SetStatusWindowPos();
    fn SimulateHotKey();
    fn UnregisterWordA();
    fn UnregisterWordW();
    fn Activate();
    fn Deactivate();
    fn OnDefWindowProc();
    fn FilterClientWindows();
    fn GetCodePageA();
    fn GetLangId();
    fn AssociateContextEx();
    fn DisableIME();
    fn GetImeMenuItemsA();
    fn GetImeMenuItemsW();
    fn EnumInputContext();
}
impl ::windows::core::RuntimeName for IActiveIMMApp {
    const NAME: &'static str = "Windows.Win32.UI.Input.Ime.IActiveIMMApp";
}
impl IActiveIMMAppVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMAppImpl, const OFFSET: isize>() -> IActiveIMMAppVtbl {
        unsafe extern "system" fn AssociateContext<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, hime: super::super::super::Globalization::HIMC, phprev: *mut super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AssociateContext(&*(&hwnd as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&hime as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&phprev)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConfigureIMEA<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pdata: *const REGISTERWORDA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConfigureIMEA(
                &*(&hkl as *const <super::super::TextServices::HKL as ::windows::core::Abi>::Abi as *const <super::super::TextServices::HKL as ::windows::core::DefaultType>::DefaultType),
                &*(&hwnd as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                dwmode,
                &*(&pdata as *const <REGISTERWORDA as ::windows::core::Abi>::Abi as *const <REGISTERWORDA as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConfigureIMEW<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pdata: *const REGISTERWORDW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConfigureIMEW(
                &*(&hkl as *const <super::super::TextServices::HKL as ::windows::core::Abi>::Abi as *const <super::super::TextServices::HKL as ::windows::core::DefaultType>::DefaultType),
                &*(&hwnd as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                dwmode,
                &*(&pdata as *const <REGISTERWORDW as ::windows::core::Abi>::Abi as *const <REGISTERWORDW as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateContext<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phimc: *mut super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateContext(::core::mem::transmute_copy(&phimc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestroyContext<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hime: super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DestroyContext(&*(&hime as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumRegisterWordA<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: super::super::super::Foundation::PSTR, dwstyle: u32, szregister: super::super::super::Foundation::PSTR, pdata: *const ::core::ffi::c_void, penum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumRegisterWordA(
                &*(&hkl as *const <super::super::TextServices::HKL as ::windows::core::Abi>::Abi as *const <super::super::TextServices::HKL as ::windows::core::DefaultType>::DefaultType),
                &*(&szreading as *const <super::super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
                dwstyle,
                &*(&szregister as *const <super::super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&penum),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumRegisterWordW<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: super::super::super::Foundation::PWSTR, dwstyle: u32, szregister: super::super::super::Foundation::PWSTR, pdata: *const ::core::ffi::c_void, penum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumRegisterWordW(
                &*(&hkl as *const <super::super::TextServices::HKL as ::windows::core::Abi>::Abi as *const <super::super::TextServices::HKL as ::windows::core::DefaultType>::DefaultType),
                &*(&szreading as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                dwstyle,
                &*(&szregister as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&penum),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EscapeA<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EscapeA(
                &*(&hkl as *const <super::super::TextServices::HKL as ::windows::core::Abi>::Abi as *const <super::super::TextServices::HKL as ::windows::core::DefaultType>::DefaultType),
                &*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType),
                uescape,
                &*(&pdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&plresult),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EscapeW<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EscapeW(
                &*(&hkl as *const <super::super::TextServices::HKL as ::windows::core::Abi>::Abi as *const <super::super::TextServices::HKL as ::windows::core::DefaultType>::DefaultType),
                &*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType),
                uescape,
                &*(&pdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&plresult),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCandidateListA<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCandidateListA(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), dwindex, ubuflen, ::core::mem::transmute_copy(&pcandlist), ::core::mem::transmute_copy(&pucopied)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCandidateListW<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCandidateListW(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), dwindex, ubuflen, ::core::mem::transmute_copy(&pcandlist), ::core::mem::transmute_copy(&pucopied)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCandidateListCountA<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCandidateListCountA(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwlistsize), ::core::mem::transmute_copy(&pdwbuflen)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCandidateListCountW<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCandidateListCountW(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwlistsize), ::core::mem::transmute_copy(&pdwbuflen)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCandidateWindow<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcandidate: *mut CANDIDATEFORM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCandidateWindow(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), dwindex, ::core::mem::transmute_copy(&pcandidate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCompositionFontA<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *mut super::super::super::Graphics::Gdi::LOGFONTA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCompositionFontA(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&plf)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCompositionFontW<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *mut super::super::super::Graphics::Gdi::LOGFONTW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCompositionFontW(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&plf)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCompositionStringA<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCompositionStringA(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), dwindex, dwbuflen, ::core::mem::transmute_copy(&plcopied), ::core::mem::transmute_copy(&pbuf)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCompositionStringW<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCompositionStringW(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), dwindex, dwbuflen, ::core::mem::transmute_copy(&plcopied), ::core::mem::transmute_copy(&pbuf)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCompositionWindow<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pcompform: *mut COMPOSITIONFORM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCompositionWindow(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcompform)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContext<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, phimc: *mut super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContext(&*(&hwnd as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&phimc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConversionListA<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, psrc: super::super::super::Foundation::PSTR, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConversionListA(
                &*(&hkl as *const <super::super::TextServices::HKL as ::windows::core::Abi>::Abi as *const <super::super::TextServices::HKL as ::windows::core::DefaultType>::DefaultType),
                &*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType),
                &*(&psrc as *const <super::super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
                ubuflen,
                uflag,
                ::core::mem::transmute_copy(&pdst),
                ::core::mem::transmute_copy(&pucopied),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConversionListW<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, psrc: super::super::super::Foundation::PWSTR, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConversionListW(
                &*(&hkl as *const <super::super::TextServices::HKL as ::windows::core::Abi>::Abi as *const <super::super::TextServices::HKL as ::windows::core::DefaultType>::DefaultType),
                &*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType),
                &*(&psrc as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                ubuflen,
                uflag,
                ::core::mem::transmute_copy(&pdst),
                ::core::mem::transmute_copy(&pucopied),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConversionStatus<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pfdwconversion: *mut u32, pfdwsentence: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConversionStatus(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfdwconversion), ::core::mem::transmute_copy(&pfdwsentence)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultIMEWnd<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, phdefwnd: *mut super::super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultIMEWnd(&*(&hwnd as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&phdefwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescriptionA<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szdescription: super::super::super::Foundation::PSTR, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDescriptionA(&*(&hkl as *const <super::super::TextServices::HKL as ::windows::core::Abi>::Abi as *const <super::super::TextServices::HKL as ::windows::core::DefaultType>::DefaultType), ubuflen, ::core::mem::transmute_copy(&szdescription), ::core::mem::transmute_copy(&pucopied)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescriptionW<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szdescription: super::super::super::Foundation::PWSTR, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDescriptionW(&*(&hkl as *const <super::super::TextServices::HKL as ::windows::core::Abi>::Abi as *const <super::super::TextServices::HKL as ::windows::core::DefaultType>::DefaultType), ubuflen, ::core::mem::transmute_copy(&szdescription), ::core::mem::transmute_copy(&pucopied)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGuideLineA<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, pbuf: super::super::super::Foundation::PSTR, pdwresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGuideLineA(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), dwindex, dwbuflen, ::core::mem::transmute_copy(&pbuf), ::core::mem::transmute_copy(&pdwresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGuideLineW<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, pbuf: super::super::super::Foundation::PWSTR, pdwresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGuideLineW(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), dwindex, dwbuflen, ::core::mem::transmute_copy(&pbuf), ::core::mem::transmute_copy(&pdwresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIMEFileNameA<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szfilename: super::super::super::Foundation::PSTR, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIMEFileNameA(&*(&hkl as *const <super::super::TextServices::HKL as ::windows::core::Abi>::Abi as *const <super::super::TextServices::HKL as ::windows::core::DefaultType>::DefaultType), ubuflen, ::core::mem::transmute_copy(&szfilename), ::core::mem::transmute_copy(&pucopied)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIMEFileNameW<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szfilename: super::super::super::Foundation::PWSTR, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIMEFileNameW(&*(&hkl as *const <super::super::TextServices::HKL as ::windows::core::Abi>::Abi as *const <super::super::TextServices::HKL as ::windows::core::DefaultType>::DefaultType), ubuflen, ::core::mem::transmute_copy(&szfilename), ::core::mem::transmute_copy(&pucopied)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOpenStatus<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOpenStatus(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, fdwindex: u32, pdwproperty: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(&*(&hkl as *const <super::super::TextServices::HKL as ::windows::core::Abi>::Abi as *const <super::super::TextServices::HKL as ::windows::core::DefaultType>::DefaultType), fdwindex, ::core::mem::transmute_copy(&pdwproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRegisterWordStyleA<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, nitem: u32, pstylebuf: *mut STYLEBUFA, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRegisterWordStyleA(&*(&hkl as *const <super::super::TextServices::HKL as ::windows::core::Abi>::Abi as *const <super::super::TextServices::HKL as ::windows::core::DefaultType>::DefaultType), nitem, ::core::mem::transmute_copy(&pstylebuf), ::core::mem::transmute_copy(&pucopied)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRegisterWordStyleW<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, nitem: u32, pstylebuf: *mut STYLEBUFW, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRegisterWordStyleW(&*(&hkl as *const <super::super::TextServices::HKL as ::windows::core::Abi>::Abi as *const <super::super::TextServices::HKL as ::windows::core::DefaultType>::DefaultType), nitem, ::core::mem::transmute_copy(&pstylebuf), ::core::mem::transmute_copy(&pucopied)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatusWindowPos<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pptpos: *mut super::super::super::Foundation::POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatusWindowPos(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pptpos)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVirtualKey<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, puvirtualkey: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVirtualKey(&*(&hwnd as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&puvirtualkey)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallIMEA<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szimefilename: super::super::super::Foundation::PSTR, szlayouttext: super::super::super::Foundation::PSTR, phkl: *mut super::super::TextServices::HKL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstallIMEA(&*(&szimefilename as *const <super::super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), &*(&szlayouttext as *const <super::super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&phkl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallIMEW<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szimefilename: super::super::super::Foundation::PWSTR, szlayouttext: super::super::super::Foundation::PWSTR, phkl: *mut super::super::TextServices::HKL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstallIMEW(&*(&szimefilename as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&szlayouttext as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&phkl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsIME<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsIME(&*(&hkl as *const <super::super::TextServices::HKL as ::windows::core::Abi>::Abi as *const <super::super::TextServices::HKL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUIMessageA<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndime: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsUIMessageA(
                &*(&hwndime as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                msg,
                &*(&wparam as *const <super::super::super::Foundation::WPARAM as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::WPARAM as ::windows::core::DefaultType>::DefaultType),
                &*(&lparam as *const <super::super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUIMessageW<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndime: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsUIMessageW(
                &*(&hwndime as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                msg,
                &*(&wparam as *const <super::super::super::Foundation::WPARAM as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::WPARAM as ::windows::core::DefaultType>::DefaultType),
                &*(&lparam as *const <super::super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyIME<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwaction: u32, dwindex: u32, dwvalue: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotifyIME(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), dwaction, dwindex, dwvalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterWordA<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: super::super::super::Foundation::PSTR, dwstyle: u32, szregister: super::super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterWordA(
                &*(&hkl as *const <super::super::TextServices::HKL as ::windows::core::Abi>::Abi as *const <super::super::TextServices::HKL as ::windows::core::DefaultType>::DefaultType),
                &*(&szreading as *const <super::super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
                dwstyle,
                &*(&szregister as *const <super::super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterWordW<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: super::super::super::Foundation::PWSTR, dwstyle: u32, szregister: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterWordW(
                &*(&hkl as *const <super::super::TextServices::HKL as ::windows::core::Abi>::Abi as *const <super::super::TextServices::HKL as ::windows::core::DefaultType>::DefaultType),
                &*(&szreading as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                dwstyle,
                &*(&szregister as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseContext<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, himc: super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReleaseContext(&*(&hwnd as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCandidateWindow<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pcandidate: *const CANDIDATEFORM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCandidateWindow(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), &*(&pcandidate as *const <CANDIDATEFORM as ::windows::core::Abi>::Abi as *const <CANDIDATEFORM as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompositionFontA<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *const super::super::super::Graphics::Gdi::LOGFONTA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCompositionFontA(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), &*(&plf as *const <super::super::super::Graphics::Gdi::LOGFONTA as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Gdi::LOGFONTA as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompositionFontW<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *const super::super::super::Graphics::Gdi::LOGFONTW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCompositionFontW(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), &*(&plf as *const <super::super::super::Graphics::Gdi::LOGFONTW as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Gdi::LOGFONTW as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompositionStringA<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCompositionStringA(
                &*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType),
                dwindex,
                &*(&pcomp as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                dwcomplen,
                &*(&pread as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                dwreadlen,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompositionStringW<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCompositionStringW(
                &*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType),
                dwindex,
                &*(&pcomp as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                dwcomplen,
                &*(&pread as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                dwreadlen,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompositionWindow<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pcompform: *const COMPOSITIONFORM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCompositionWindow(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), &*(&pcompform as *const <COMPOSITIONFORM as ::windows::core::Abi>::Abi as *const <COMPOSITIONFORM as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConversionStatus<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, fdwconversion: u32, fdwsentence: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetConversionStatus(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), fdwconversion, fdwsentence) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpenStatus<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, fopen: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOpenStatus(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), &*(&fopen as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStatusWindowPos<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pptpos: *const super::super::super::Foundation::POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetStatusWindowPos(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), &*(&pptpos as *const <super::super::super::Foundation::POINT as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::POINT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SimulateHotKey<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, dwhotkeyid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SimulateHotKey(&*(&hwnd as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), dwhotkeyid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterWordA<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: super::super::super::Foundation::PSTR, dwstyle: u32, szunregister: super::super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnregisterWordA(
                &*(&hkl as *const <super::super::TextServices::HKL as ::windows::core::Abi>::Abi as *const <super::super::TextServices::HKL as ::windows::core::DefaultType>::DefaultType),
                &*(&szreading as *const <super::super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
                dwstyle,
                &*(&szunregister as *const <super::super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterWordW<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: super::super::super::Foundation::PWSTR, dwstyle: u32, szunregister: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnregisterWordW(
                &*(&hkl as *const <super::super::TextServices::HKL as ::windows::core::Abi>::Abi as *const <super::super::TextServices::HKL as ::windows::core::DefaultType>::DefaultType),
                &*(&szreading as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                dwstyle,
                &*(&szunregister as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Activate<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frestorelayout: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Activate(&*(&frestorelayout as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Deactivate<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OnDefWindowProc<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnDefWindowProc(
                &*(&hwnd as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                msg,
                &*(&wparam as *const <super::super::super::Foundation::WPARAM as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::WPARAM as ::windows::core::DefaultType>::DefaultType),
                &*(&lparam as *const <super::super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&plresult),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FilterClientWindows<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aaclasslist: *const u16, usize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FilterClientWindows(aaclasslist, usize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCodePageA<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ucodepage: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCodePageA(&*(&hkl as *const <super::super::TextServices::HKL as ::windows::core::Abi>::Abi as *const <super::super::TextServices::HKL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ucodepage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLangId<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, plid: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLangId(&*(&hkl as *const <super::super::TextServices::HKL as ::windows::core::Abi>::Abi as *const <super::super::TextServices::HKL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&plid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AssociateContextEx<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, himc: super::super::super::Globalization::HIMC, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AssociateContextEx(&*(&hwnd as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisableIME<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idthread: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisableIME(idthread) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImeMenuItemsA<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOA, pimemenu: *mut IMEMENUITEMINFOA, dwsize: u32, pdwresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetImeMenuItemsA(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), dwflags, dwtype, &*(&pimeparentmenu as *const <IMEMENUITEMINFOA as ::windows::core::Abi>::Abi as *const <IMEMENUITEMINFOA as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pimemenu), dwsize, ::core::mem::transmute_copy(&pdwresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImeMenuItemsW<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOW, pimemenu: *mut IMEMENUITEMINFOW, dwsize: u32, pdwresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetImeMenuItemsW(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), dwflags, dwtype, &*(&pimeparentmenu as *const <IMEMENUITEMINFOW as ::windows::core::Abi>::Abi as *const <IMEMENUITEMINFOW as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pimemenu), dwsize, ::core::mem::transmute_copy(&pdwresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumInputContext<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idthread: u32, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumInputContext(idthread, ::core::mem::transmute_copy(&ppenum)) {
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
            ::windows::core::GetRuntimeClassName::<IActiveIMMApp>,
            ::windows::core::GetTrustLevel,
            AssociateContext::<Impl, OFFSET>,
            ConfigureIMEA::<Impl, OFFSET>,
            ConfigureIMEW::<Impl, OFFSET>,
            CreateContext::<Impl, OFFSET>,
            DestroyContext::<Impl, OFFSET>,
            EnumRegisterWordA::<Impl, OFFSET>,
            EnumRegisterWordW::<Impl, OFFSET>,
            EscapeA::<Impl, OFFSET>,
            EscapeW::<Impl, OFFSET>,
            GetCandidateListA::<Impl, OFFSET>,
            GetCandidateListW::<Impl, OFFSET>,
            GetCandidateListCountA::<Impl, OFFSET>,
            GetCandidateListCountW::<Impl, OFFSET>,
            GetCandidateWindow::<Impl, OFFSET>,
            GetCompositionFontA::<Impl, OFFSET>,
            GetCompositionFontW::<Impl, OFFSET>,
            GetCompositionStringA::<Impl, OFFSET>,
            GetCompositionStringW::<Impl, OFFSET>,
            GetCompositionWindow::<Impl, OFFSET>,
            GetContext::<Impl, OFFSET>,
            GetConversionListA::<Impl, OFFSET>,
            GetConversionListW::<Impl, OFFSET>,
            GetConversionStatus::<Impl, OFFSET>,
            GetDefaultIMEWnd::<Impl, OFFSET>,
            GetDescriptionA::<Impl, OFFSET>,
            GetDescriptionW::<Impl, OFFSET>,
            GetGuideLineA::<Impl, OFFSET>,
            GetGuideLineW::<Impl, OFFSET>,
            GetIMEFileNameA::<Impl, OFFSET>,
            GetIMEFileNameW::<Impl, OFFSET>,
            GetOpenStatus::<Impl, OFFSET>,
            GetProperty::<Impl, OFFSET>,
            GetRegisterWordStyleA::<Impl, OFFSET>,
            GetRegisterWordStyleW::<Impl, OFFSET>,
            GetStatusWindowPos::<Impl, OFFSET>,
            GetVirtualKey::<Impl, OFFSET>,
            InstallIMEA::<Impl, OFFSET>,
            InstallIMEW::<Impl, OFFSET>,
            IsIME::<Impl, OFFSET>,
            IsUIMessageA::<Impl, OFFSET>,
            IsUIMessageW::<Impl, OFFSET>,
            NotifyIME::<Impl, OFFSET>,
            RegisterWordA::<Impl, OFFSET>,
            RegisterWordW::<Impl, OFFSET>,
            ReleaseContext::<Impl, OFFSET>,
            SetCandidateWindow::<Impl, OFFSET>,
            SetCompositionFontA::<Impl, OFFSET>,
            SetCompositionFontW::<Impl, OFFSET>,
            SetCompositionStringA::<Impl, OFFSET>,
            SetCompositionStringW::<Impl, OFFSET>,
            SetCompositionWindow::<Impl, OFFSET>,
            SetConversionStatus::<Impl, OFFSET>,
            SetOpenStatus::<Impl, OFFSET>,
            SetStatusWindowPos::<Impl, OFFSET>,
            SimulateHotKey::<Impl, OFFSET>,
            UnregisterWordA::<Impl, OFFSET>,
            UnregisterWordW::<Impl, OFFSET>,
            Activate::<Impl, OFFSET>,
            Deactivate::<Impl, OFFSET>,
            OnDefWindowProc::<Impl, OFFSET>,
            FilterClientWindows::<Impl, OFFSET>,
            GetCodePageA::<Impl, OFFSET>,
            GetLangId::<Impl, OFFSET>,
            AssociateContextEx::<Impl, OFFSET>,
            DisableIME::<Impl, OFFSET>,
            GetImeMenuItemsA::<Impl, OFFSET>,
            GetImeMenuItemsW::<Impl, OFFSET>,
            EnumInputContext::<Impl, OFFSET>,
        )
    }
}
pub trait IActiveIMMIMEImpl: Sized {
    fn AssociateContext();
    fn ConfigureIMEA();
    fn ConfigureIMEW();
    fn CreateContext();
    fn DestroyContext();
    fn EnumRegisterWordA();
    fn EnumRegisterWordW();
    fn EscapeA();
    fn EscapeW();
    fn GetCandidateListA();
    fn GetCandidateListW();
    fn GetCandidateListCountA();
    fn GetCandidateListCountW();
    fn GetCandidateWindow();
    fn GetCompositionFontA();
    fn GetCompositionFontW();
    fn GetCompositionStringA();
    fn GetCompositionStringW();
    fn GetCompositionWindow();
    fn GetContext();
    fn GetConversionListA();
    fn GetConversionListW();
    fn GetConversionStatus();
    fn GetDefaultIMEWnd();
    fn GetDescriptionA();
    fn GetDescriptionW();
    fn GetGuideLineA();
    fn GetGuideLineW();
    fn GetIMEFileNameA();
    fn GetIMEFileNameW();
    fn GetOpenStatus();
    fn GetProperty();
    fn GetRegisterWordStyleA();
    fn GetRegisterWordStyleW();
    fn GetStatusWindowPos();
    fn GetVirtualKey();
    fn InstallIMEA();
    fn InstallIMEW();
    fn IsIME();
    fn IsUIMessageA();
    fn IsUIMessageW();
    fn NotifyIME();
    fn RegisterWordA();
    fn RegisterWordW();
    fn ReleaseContext();
    fn SetCandidateWindow();
    fn SetCompositionFontA();
    fn SetCompositionFontW();
    fn SetCompositionStringA();
    fn SetCompositionStringW();
    fn SetCompositionWindow();
    fn SetConversionStatus();
    fn SetOpenStatus();
    fn SetStatusWindowPos();
    fn SimulateHotKey();
    fn UnregisterWordA();
    fn UnregisterWordW();
    fn GenerateMessage();
    fn LockIMC();
    fn UnlockIMC();
    fn GetIMCLockCount();
    fn CreateIMCC();
    fn DestroyIMCC();
    fn LockIMCC();
    fn UnlockIMCC();
    fn ReSizeIMCC();
    fn GetIMCCSize();
    fn GetIMCCLockCount();
    fn GetHotKey();
    fn SetHotKey();
    fn CreateSoftKeyboard();
    fn DestroySoftKeyboard();
    fn ShowSoftKeyboard();
    fn GetCodePageA();
    fn GetLangId();
    fn KeybdEvent();
    fn LockModal();
    fn UnlockModal();
    fn AssociateContextEx();
    fn DisableIME();
    fn GetImeMenuItemsA();
    fn GetImeMenuItemsW();
    fn EnumInputContext();
    fn RequestMessageA();
    fn RequestMessageW();
    fn SendIMCA();
    fn SendIMCW();
    fn IsSleeping();
}
impl ::windows::core::RuntimeName for IActiveIMMIME {
    const NAME: &'static str = "Windows.Win32.UI.Input.Ime.IActiveIMMIME";
}
impl IActiveIMMIMEVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIMEImpl, const OFFSET: isize>() -> IActiveIMMIMEVtbl {
        unsafe extern "system" fn AssociateContext<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, hime: super::super::super::Globalization::HIMC, phprev: *mut super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AssociateContext(&*(&hwnd as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&hime as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&phprev)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConfigureIMEA<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pdata: *const REGISTERWORDA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConfigureIMEA(
                &*(&hkl as *const <super::super::TextServices::HKL as ::windows::core::Abi>::Abi as *const <super::super::TextServices::HKL as ::windows::core::DefaultType>::DefaultType),
                &*(&hwnd as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                dwmode,
                &*(&pdata as *const <REGISTERWORDA as ::windows::core::Abi>::Abi as *const <REGISTERWORDA as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConfigureIMEW<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pdata: *const REGISTERWORDW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConfigureIMEW(
                &*(&hkl as *const <super::super::TextServices::HKL as ::windows::core::Abi>::Abi as *const <super::super::TextServices::HKL as ::windows::core::DefaultType>::DefaultType),
                &*(&hwnd as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                dwmode,
                &*(&pdata as *const <REGISTERWORDW as ::windows::core::Abi>::Abi as *const <REGISTERWORDW as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateContext<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phimc: *mut super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateContext(::core::mem::transmute_copy(&phimc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestroyContext<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hime: super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DestroyContext(&*(&hime as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumRegisterWordA<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: super::super::super::Foundation::PSTR, dwstyle: u32, szregister: super::super::super::Foundation::PSTR, pdata: *const ::core::ffi::c_void, penum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumRegisterWordA(
                &*(&hkl as *const <super::super::TextServices::HKL as ::windows::core::Abi>::Abi as *const <super::super::TextServices::HKL as ::windows::core::DefaultType>::DefaultType),
                &*(&szreading as *const <super::super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
                dwstyle,
                &*(&szregister as *const <super::super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&penum),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumRegisterWordW<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: super::super::super::Foundation::PWSTR, dwstyle: u32, szregister: super::super::super::Foundation::PWSTR, pdata: *const ::core::ffi::c_void, penum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumRegisterWordW(
                &*(&hkl as *const <super::super::TextServices::HKL as ::windows::core::Abi>::Abi as *const <super::super::TextServices::HKL as ::windows::core::DefaultType>::DefaultType),
                &*(&szreading as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                dwstyle,
                &*(&szregister as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&penum),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EscapeA<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EscapeA(
                &*(&hkl as *const <super::super::TextServices::HKL as ::windows::core::Abi>::Abi as *const <super::super::TextServices::HKL as ::windows::core::DefaultType>::DefaultType),
                &*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType),
                uescape,
                &*(&pdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&plresult),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EscapeW<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EscapeW(
                &*(&hkl as *const <super::super::TextServices::HKL as ::windows::core::Abi>::Abi as *const <super::super::TextServices::HKL as ::windows::core::DefaultType>::DefaultType),
                &*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType),
                uescape,
                &*(&pdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&plresult),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCandidateListA<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCandidateListA(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), dwindex, ubuflen, ::core::mem::transmute_copy(&pcandlist), ::core::mem::transmute_copy(&pucopied)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCandidateListW<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCandidateListW(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), dwindex, ubuflen, ::core::mem::transmute_copy(&pcandlist), ::core::mem::transmute_copy(&pucopied)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCandidateListCountA<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCandidateListCountA(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwlistsize), ::core::mem::transmute_copy(&pdwbuflen)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCandidateListCountW<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCandidateListCountW(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwlistsize), ::core::mem::transmute_copy(&pdwbuflen)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCandidateWindow<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcandidate: *mut CANDIDATEFORM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCandidateWindow(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), dwindex, ::core::mem::transmute_copy(&pcandidate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCompositionFontA<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *mut super::super::super::Graphics::Gdi::LOGFONTA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCompositionFontA(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&plf)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCompositionFontW<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *mut super::super::super::Graphics::Gdi::LOGFONTW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCompositionFontW(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&plf)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCompositionStringA<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCompositionStringA(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), dwindex, dwbuflen, ::core::mem::transmute_copy(&plcopied), ::core::mem::transmute_copy(&pbuf)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCompositionStringW<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCompositionStringW(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), dwindex, dwbuflen, ::core::mem::transmute_copy(&plcopied), ::core::mem::transmute_copy(&pbuf)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCompositionWindow<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pcompform: *mut COMPOSITIONFORM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCompositionWindow(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcompform)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContext<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, phimc: *mut super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContext(&*(&hwnd as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&phimc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConversionListA<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, psrc: super::super::super::Foundation::PSTR, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConversionListA(
                &*(&hkl as *const <super::super::TextServices::HKL as ::windows::core::Abi>::Abi as *const <super::super::TextServices::HKL as ::windows::core::DefaultType>::DefaultType),
                &*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType),
                &*(&psrc as *const <super::super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
                ubuflen,
                uflag,
                ::core::mem::transmute_copy(&pdst),
                ::core::mem::transmute_copy(&pucopied),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConversionListW<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, psrc: super::super::super::Foundation::PWSTR, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConversionListW(
                &*(&hkl as *const <super::super::TextServices::HKL as ::windows::core::Abi>::Abi as *const <super::super::TextServices::HKL as ::windows::core::DefaultType>::DefaultType),
                &*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType),
                &*(&psrc as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                ubuflen,
                uflag,
                ::core::mem::transmute_copy(&pdst),
                ::core::mem::transmute_copy(&pucopied),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConversionStatus<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pfdwconversion: *mut u32, pfdwsentence: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConversionStatus(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfdwconversion), ::core::mem::transmute_copy(&pfdwsentence)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultIMEWnd<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, phdefwnd: *mut super::super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultIMEWnd(&*(&hwnd as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&phdefwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescriptionA<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szdescription: super::super::super::Foundation::PSTR, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDescriptionA(&*(&hkl as *const <super::super::TextServices::HKL as ::windows::core::Abi>::Abi as *const <super::super::TextServices::HKL as ::windows::core::DefaultType>::DefaultType), ubuflen, ::core::mem::transmute_copy(&szdescription), ::core::mem::transmute_copy(&pucopied)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescriptionW<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szdescription: super::super::super::Foundation::PWSTR, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDescriptionW(&*(&hkl as *const <super::super::TextServices::HKL as ::windows::core::Abi>::Abi as *const <super::super::TextServices::HKL as ::windows::core::DefaultType>::DefaultType), ubuflen, ::core::mem::transmute_copy(&szdescription), ::core::mem::transmute_copy(&pucopied)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGuideLineA<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, pbuf: super::super::super::Foundation::PSTR, pdwresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGuideLineA(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), dwindex, dwbuflen, ::core::mem::transmute_copy(&pbuf), ::core::mem::transmute_copy(&pdwresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGuideLineW<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, pbuf: super::super::super::Foundation::PWSTR, pdwresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGuideLineW(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), dwindex, dwbuflen, ::core::mem::transmute_copy(&pbuf), ::core::mem::transmute_copy(&pdwresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIMEFileNameA<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szfilename: super::super::super::Foundation::PSTR, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIMEFileNameA(&*(&hkl as *const <super::super::TextServices::HKL as ::windows::core::Abi>::Abi as *const <super::super::TextServices::HKL as ::windows::core::DefaultType>::DefaultType), ubuflen, ::core::mem::transmute_copy(&szfilename), ::core::mem::transmute_copy(&pucopied)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIMEFileNameW<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szfilename: super::super::super::Foundation::PWSTR, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIMEFileNameW(&*(&hkl as *const <super::super::TextServices::HKL as ::windows::core::Abi>::Abi as *const <super::super::TextServices::HKL as ::windows::core::DefaultType>::DefaultType), ubuflen, ::core::mem::transmute_copy(&szfilename), ::core::mem::transmute_copy(&pucopied)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOpenStatus<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOpenStatus(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, fdwindex: u32, pdwproperty: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(&*(&hkl as *const <super::super::TextServices::HKL as ::windows::core::Abi>::Abi as *const <super::super::TextServices::HKL as ::windows::core::DefaultType>::DefaultType), fdwindex, ::core::mem::transmute_copy(&pdwproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRegisterWordStyleA<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, nitem: u32, pstylebuf: *mut STYLEBUFA, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRegisterWordStyleA(&*(&hkl as *const <super::super::TextServices::HKL as ::windows::core::Abi>::Abi as *const <super::super::TextServices::HKL as ::windows::core::DefaultType>::DefaultType), nitem, ::core::mem::transmute_copy(&pstylebuf), ::core::mem::transmute_copy(&pucopied)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRegisterWordStyleW<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, nitem: u32, pstylebuf: *mut STYLEBUFW, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRegisterWordStyleW(&*(&hkl as *const <super::super::TextServices::HKL as ::windows::core::Abi>::Abi as *const <super::super::TextServices::HKL as ::windows::core::DefaultType>::DefaultType), nitem, ::core::mem::transmute_copy(&pstylebuf), ::core::mem::transmute_copy(&pucopied)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatusWindowPos<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pptpos: *mut super::super::super::Foundation::POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatusWindowPos(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pptpos)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVirtualKey<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, puvirtualkey: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVirtualKey(&*(&hwnd as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&puvirtualkey)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallIMEA<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szimefilename: super::super::super::Foundation::PSTR, szlayouttext: super::super::super::Foundation::PSTR, phkl: *mut super::super::TextServices::HKL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstallIMEA(&*(&szimefilename as *const <super::super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), &*(&szlayouttext as *const <super::super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&phkl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallIMEW<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szimefilename: super::super::super::Foundation::PWSTR, szlayouttext: super::super::super::Foundation::PWSTR, phkl: *mut super::super::TextServices::HKL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstallIMEW(&*(&szimefilename as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&szlayouttext as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&phkl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsIME<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsIME(&*(&hkl as *const <super::super::TextServices::HKL as ::windows::core::Abi>::Abi as *const <super::super::TextServices::HKL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUIMessageA<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndime: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsUIMessageA(
                &*(&hwndime as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                msg,
                &*(&wparam as *const <super::super::super::Foundation::WPARAM as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::WPARAM as ::windows::core::DefaultType>::DefaultType),
                &*(&lparam as *const <super::super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUIMessageW<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndime: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsUIMessageW(
                &*(&hwndime as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                msg,
                &*(&wparam as *const <super::super::super::Foundation::WPARAM as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::WPARAM as ::windows::core::DefaultType>::DefaultType),
                &*(&lparam as *const <super::super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyIME<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwaction: u32, dwindex: u32, dwvalue: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotifyIME(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), dwaction, dwindex, dwvalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterWordA<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: super::super::super::Foundation::PSTR, dwstyle: u32, szregister: super::super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterWordA(
                &*(&hkl as *const <super::super::TextServices::HKL as ::windows::core::Abi>::Abi as *const <super::super::TextServices::HKL as ::windows::core::DefaultType>::DefaultType),
                &*(&szreading as *const <super::super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
                dwstyle,
                &*(&szregister as *const <super::super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterWordW<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: super::super::super::Foundation::PWSTR, dwstyle: u32, szregister: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterWordW(
                &*(&hkl as *const <super::super::TextServices::HKL as ::windows::core::Abi>::Abi as *const <super::super::TextServices::HKL as ::windows::core::DefaultType>::DefaultType),
                &*(&szreading as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                dwstyle,
                &*(&szregister as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseContext<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, himc: super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReleaseContext(&*(&hwnd as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCandidateWindow<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pcandidate: *const CANDIDATEFORM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCandidateWindow(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), &*(&pcandidate as *const <CANDIDATEFORM as ::windows::core::Abi>::Abi as *const <CANDIDATEFORM as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompositionFontA<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *const super::super::super::Graphics::Gdi::LOGFONTA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCompositionFontA(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), &*(&plf as *const <super::super::super::Graphics::Gdi::LOGFONTA as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Gdi::LOGFONTA as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompositionFontW<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *const super::super::super::Graphics::Gdi::LOGFONTW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCompositionFontW(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), &*(&plf as *const <super::super::super::Graphics::Gdi::LOGFONTW as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Gdi::LOGFONTW as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompositionStringA<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCompositionStringA(
                &*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType),
                dwindex,
                &*(&pcomp as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                dwcomplen,
                &*(&pread as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                dwreadlen,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompositionStringW<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCompositionStringW(
                &*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType),
                dwindex,
                &*(&pcomp as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                dwcomplen,
                &*(&pread as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                dwreadlen,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompositionWindow<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pcompform: *const COMPOSITIONFORM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCompositionWindow(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), &*(&pcompform as *const <COMPOSITIONFORM as ::windows::core::Abi>::Abi as *const <COMPOSITIONFORM as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConversionStatus<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, fdwconversion: u32, fdwsentence: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetConversionStatus(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), fdwconversion, fdwsentence) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpenStatus<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, fopen: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOpenStatus(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), &*(&fopen as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStatusWindowPos<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pptpos: *const super::super::super::Foundation::POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetStatusWindowPos(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), &*(&pptpos as *const <super::super::super::Foundation::POINT as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::POINT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SimulateHotKey<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, dwhotkeyid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SimulateHotKey(&*(&hwnd as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), dwhotkeyid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterWordA<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: super::super::super::Foundation::PSTR, dwstyle: u32, szunregister: super::super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnregisterWordA(
                &*(&hkl as *const <super::super::TextServices::HKL as ::windows::core::Abi>::Abi as *const <super::super::TextServices::HKL as ::windows::core::DefaultType>::DefaultType),
                &*(&szreading as *const <super::super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
                dwstyle,
                &*(&szunregister as *const <super::super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterWordW<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: super::super::super::Foundation::PWSTR, dwstyle: u32, szunregister: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnregisterWordW(
                &*(&hkl as *const <super::super::TextServices::HKL as ::windows::core::Abi>::Abi as *const <super::super::TextServices::HKL as ::windows::core::DefaultType>::DefaultType),
                &*(&szreading as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                dwstyle,
                &*(&szunregister as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenerateMessage<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GenerateMessage(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockIMC<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, ppimc: *mut *mut INPUTCONTEXT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LockIMC(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppimc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnlockIMC<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnlockIMC(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIMCLockCount<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pdwlockcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIMCLockCount(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwlockcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateIMCC<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsize: u32, phimcc: *mut super::super::super::Globalization::HIMCC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateIMCC(dwsize, ::core::mem::transmute_copy(&phimcc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestroyIMCC<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himcc: super::super::super::Globalization::HIMCC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DestroyIMCC(&*(&himcc as *const <super::super::super::Globalization::HIMCC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMCC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockIMCC<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himcc: super::super::super::Globalization::HIMCC, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LockIMCC(&*(&himcc as *const <super::super::super::Globalization::HIMCC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMCC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnlockIMCC<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himcc: super::super::super::Globalization::HIMCC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnlockIMCC(&*(&himcc as *const <super::super::super::Globalization::HIMCC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMCC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReSizeIMCC<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himcc: super::super::super::Globalization::HIMCC, dwsize: u32, phimcc: *mut super::super::super::Globalization::HIMCC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReSizeIMCC(&*(&himcc as *const <super::super::super::Globalization::HIMCC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMCC as ::windows::core::DefaultType>::DefaultType), dwsize, ::core::mem::transmute_copy(&phimcc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIMCCSize<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himcc: super::super::super::Globalization::HIMCC, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIMCCSize(&*(&himcc as *const <super::super::super::Globalization::HIMCC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMCC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIMCCLockCount<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himcc: super::super::super::Globalization::HIMCC, pdwlockcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIMCCLockCount(&*(&himcc as *const <super::super::super::Globalization::HIMCC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMCC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwlockcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHotKey<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwhotkeyid: u32, pumodifiers: *mut u32, puvkey: *mut u32, phkl: *mut super::super::TextServices::HKL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHotKey(dwhotkeyid, ::core::mem::transmute_copy(&pumodifiers), ::core::mem::transmute_copy(&puvkey), ::core::mem::transmute_copy(&phkl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHotKey<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwhotkeyid: u32, umodifiers: u32, uvkey: u32, hkl: super::super::TextServices::HKL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetHotKey(dwhotkeyid, umodifiers, uvkey, &*(&hkl as *const <super::super::TextServices::HKL as ::windows::core::Abi>::Abi as *const <super::super::TextServices::HKL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSoftKeyboard<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, utype: u32, howner: super::super::super::Foundation::HWND, x: i32, y: i32, phsoftkbdwnd: *mut super::super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSoftKeyboard(utype, &*(&howner as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), x, y, ::core::mem::transmute_copy(&phsoftkbdwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestroySoftKeyboard<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hsoftkbdwnd: super::super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DestroySoftKeyboard(&*(&hsoftkbdwnd as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowSoftKeyboard<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hsoftkbdwnd: super::super::super::Foundation::HWND, ncmdshow: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowSoftKeyboard(&*(&hsoftkbdwnd as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), ncmdshow) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCodePageA<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ucodepage: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCodePageA(&*(&hkl as *const <super::super::TextServices::HKL as ::windows::core::Abi>::Abi as *const <super::super::TextServices::HKL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ucodepage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLangId<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, plid: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLangId(&*(&hkl as *const <super::super::TextServices::HKL as ::windows::core::Abi>::Abi as *const <super::super::TextServices::HKL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&plid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeybdEvent<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lgidime: u16, bvk: u8, bscan: u8, dwflags: u32, dwextrainfo: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeybdEvent(lgidime, bvk, bscan, dwflags, dwextrainfo) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockModal<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LockModal() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnlockModal<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnlockModal() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AssociateContextEx<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, himc: super::super::super::Globalization::HIMC, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AssociateContextEx(&*(&hwnd as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisableIME<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idthread: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisableIME(idthread) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImeMenuItemsA<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOA, pimemenu: *mut IMEMENUITEMINFOA, dwsize: u32, pdwresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetImeMenuItemsA(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), dwflags, dwtype, &*(&pimeparentmenu as *const <IMEMENUITEMINFOA as ::windows::core::Abi>::Abi as *const <IMEMENUITEMINFOA as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pimemenu), dwsize, ::core::mem::transmute_copy(&pdwresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImeMenuItemsW<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOW, pimemenu: *mut IMEMENUITEMINFOW, dwsize: u32, pdwresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetImeMenuItemsW(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType), dwflags, dwtype, &*(&pimeparentmenu as *const <IMEMENUITEMINFOW as ::windows::core::Abi>::Abi as *const <IMEMENUITEMINFOW as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pimemenu), dwsize, ::core::mem::transmute_copy(&pdwresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumInputContext<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idthread: u32, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumInputContext(idthread, ::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestMessageA<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestMessageA(
                &*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType),
                &*(&wparam as *const <super::super::super::Foundation::WPARAM as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::WPARAM as ::windows::core::DefaultType>::DefaultType),
                &*(&lparam as *const <super::super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&plresult),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestMessageW<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestMessageW(
                &*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType),
                &*(&wparam as *const <super::super::super::Foundation::WPARAM as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::WPARAM as ::windows::core::DefaultType>::DefaultType),
                &*(&lparam as *const <super::super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&plresult),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendIMCA<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, umsg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendIMCA(
                &*(&hwnd as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                umsg,
                &*(&wparam as *const <super::super::super::Foundation::WPARAM as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::WPARAM as ::windows::core::DefaultType>::DefaultType),
                &*(&lparam as *const <super::super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&plresult),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendIMCW<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, umsg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendIMCW(
                &*(&hwnd as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                umsg,
                &*(&wparam as *const <super::super::super::Foundation::WPARAM as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::WPARAM as ::windows::core::DefaultType>::DefaultType),
                &*(&lparam as *const <super::super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&plresult),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSleeping<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSleeping() {
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
            ::windows::core::GetRuntimeClassName::<IActiveIMMIME>,
            ::windows::core::GetTrustLevel,
            AssociateContext::<Impl, OFFSET>,
            ConfigureIMEA::<Impl, OFFSET>,
            ConfigureIMEW::<Impl, OFFSET>,
            CreateContext::<Impl, OFFSET>,
            DestroyContext::<Impl, OFFSET>,
            EnumRegisterWordA::<Impl, OFFSET>,
            EnumRegisterWordW::<Impl, OFFSET>,
            EscapeA::<Impl, OFFSET>,
            EscapeW::<Impl, OFFSET>,
            GetCandidateListA::<Impl, OFFSET>,
            GetCandidateListW::<Impl, OFFSET>,
            GetCandidateListCountA::<Impl, OFFSET>,
            GetCandidateListCountW::<Impl, OFFSET>,
            GetCandidateWindow::<Impl, OFFSET>,
            GetCompositionFontA::<Impl, OFFSET>,
            GetCompositionFontW::<Impl, OFFSET>,
            GetCompositionStringA::<Impl, OFFSET>,
            GetCompositionStringW::<Impl, OFFSET>,
            GetCompositionWindow::<Impl, OFFSET>,
            GetContext::<Impl, OFFSET>,
            GetConversionListA::<Impl, OFFSET>,
            GetConversionListW::<Impl, OFFSET>,
            GetConversionStatus::<Impl, OFFSET>,
            GetDefaultIMEWnd::<Impl, OFFSET>,
            GetDescriptionA::<Impl, OFFSET>,
            GetDescriptionW::<Impl, OFFSET>,
            GetGuideLineA::<Impl, OFFSET>,
            GetGuideLineW::<Impl, OFFSET>,
            GetIMEFileNameA::<Impl, OFFSET>,
            GetIMEFileNameW::<Impl, OFFSET>,
            GetOpenStatus::<Impl, OFFSET>,
            GetProperty::<Impl, OFFSET>,
            GetRegisterWordStyleA::<Impl, OFFSET>,
            GetRegisterWordStyleW::<Impl, OFFSET>,
            GetStatusWindowPos::<Impl, OFFSET>,
            GetVirtualKey::<Impl, OFFSET>,
            InstallIMEA::<Impl, OFFSET>,
            InstallIMEW::<Impl, OFFSET>,
            IsIME::<Impl, OFFSET>,
            IsUIMessageA::<Impl, OFFSET>,
            IsUIMessageW::<Impl, OFFSET>,
            NotifyIME::<Impl, OFFSET>,
            RegisterWordA::<Impl, OFFSET>,
            RegisterWordW::<Impl, OFFSET>,
            ReleaseContext::<Impl, OFFSET>,
            SetCandidateWindow::<Impl, OFFSET>,
            SetCompositionFontA::<Impl, OFFSET>,
            SetCompositionFontW::<Impl, OFFSET>,
            SetCompositionStringA::<Impl, OFFSET>,
            SetCompositionStringW::<Impl, OFFSET>,
            SetCompositionWindow::<Impl, OFFSET>,
            SetConversionStatus::<Impl, OFFSET>,
            SetOpenStatus::<Impl, OFFSET>,
            SetStatusWindowPos::<Impl, OFFSET>,
            SimulateHotKey::<Impl, OFFSET>,
            UnregisterWordA::<Impl, OFFSET>,
            UnregisterWordW::<Impl, OFFSET>,
            GenerateMessage::<Impl, OFFSET>,
            LockIMC::<Impl, OFFSET>,
            UnlockIMC::<Impl, OFFSET>,
            GetIMCLockCount::<Impl, OFFSET>,
            CreateIMCC::<Impl, OFFSET>,
            DestroyIMCC::<Impl, OFFSET>,
            LockIMCC::<Impl, OFFSET>,
            UnlockIMCC::<Impl, OFFSET>,
            ReSizeIMCC::<Impl, OFFSET>,
            GetIMCCSize::<Impl, OFFSET>,
            GetIMCCLockCount::<Impl, OFFSET>,
            GetHotKey::<Impl, OFFSET>,
            SetHotKey::<Impl, OFFSET>,
            CreateSoftKeyboard::<Impl, OFFSET>,
            DestroySoftKeyboard::<Impl, OFFSET>,
            ShowSoftKeyboard::<Impl, OFFSET>,
            GetCodePageA::<Impl, OFFSET>,
            GetLangId::<Impl, OFFSET>,
            KeybdEvent::<Impl, OFFSET>,
            LockModal::<Impl, OFFSET>,
            UnlockModal::<Impl, OFFSET>,
            AssociateContextEx::<Impl, OFFSET>,
            DisableIME::<Impl, OFFSET>,
            GetImeMenuItemsA::<Impl, OFFSET>,
            GetImeMenuItemsW::<Impl, OFFSET>,
            EnumInputContext::<Impl, OFFSET>,
            RequestMessageA::<Impl, OFFSET>,
            RequestMessageW::<Impl, OFFSET>,
            SendIMCA::<Impl, OFFSET>,
            SendIMCW::<Impl, OFFSET>,
            IsSleeping::<Impl, OFFSET>,
        )
    }
}
pub trait IActiveIMMMessagePumpOwnerImpl: Sized {
    fn Start();
    fn End();
    fn OnTranslateMessage();
    fn Pause();
    fn Resume();
}
impl ::windows::core::RuntimeName for IActiveIMMMessagePumpOwner {
    const NAME: &'static str = "Windows.Win32.UI.Input.Ime.IActiveIMMMessagePumpOwner";
}
impl IActiveIMMMessagePumpOwnerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMMessagePumpOwnerImpl, const OFFSET: isize>() -> IActiveIMMMessagePumpOwnerVtbl {
        unsafe extern "system" fn Start<Impl: IActiveIMMMessagePumpOwnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Start() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn End<Impl: IActiveIMMMessagePumpOwnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).End() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnTranslateMessage<Impl: IActiveIMMMessagePumpOwnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *const super::super::WindowsAndMessaging::MSG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnTranslateMessage(&*(&pmsg as *const <super::super::WindowsAndMessaging::MSG as ::windows::core::Abi>::Abi as *const <super::super::WindowsAndMessaging::MSG as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pause<Impl: IActiveIMMMessagePumpOwnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pause(::core::mem::transmute_copy(&pdwcookie)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resume<Impl: IActiveIMMMessagePumpOwnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Resume(dwcookie) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IActiveIMMMessagePumpOwner>, ::windows::core::GetTrustLevel, Start::<Impl, OFFSET>, End::<Impl, OFFSET>, OnTranslateMessage::<Impl, OFFSET>, Pause::<Impl, OFFSET>, Resume::<Impl, OFFSET>)
    }
}
pub trait IActiveIMMRegistrarImpl: Sized {
    fn RegisterIME();
    fn UnregisterIME();
}
impl ::windows::core::RuntimeName for IActiveIMMRegistrar {
    const NAME: &'static str = "Windows.Win32.UI.Input.Ime.IActiveIMMRegistrar";
}
impl IActiveIMMRegistrarVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMRegistrarImpl, const OFFSET: isize>() -> IActiveIMMRegistrarVtbl {
        unsafe extern "system" fn RegisterIME<Impl: IActiveIMMRegistrarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, lgid: u16, psziconfile: super::super::super::Foundation::PWSTR, pszdesc: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterIME(
                &*(&rclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                lgid,
                &*(&psziconfile as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszdesc as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterIME<Impl: IActiveIMMRegistrarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnregisterIME(&*(&rclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IActiveIMMRegistrar>, ::windows::core::GetTrustLevel, RegisterIME::<Impl, OFFSET>, UnregisterIME::<Impl, OFFSET>)
    }
}
pub trait IEnumInputContextImpl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
impl ::windows::core::RuntimeName for IEnumInputContext {
    const NAME: &'static str = "Windows.Win32.UI.Input.Ime.IEnumInputContext";
}
impl IEnumInputContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumInputContextImpl, const OFFSET: isize>() -> IEnumInputContextVtbl {
        unsafe extern "system" fn Clone<Impl: IEnumInputContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Next<Impl: IEnumInputContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, rginputcontext: *mut super::super::super::Globalization::HIMC, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(ulcount, ::core::mem::transmute_copy(&rginputcontext), ::core::mem::transmute_copy(&pcfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumInputContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Skip<Impl: IEnumInputContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumInputContext>, ::windows::core::GetTrustLevel, Clone::<Impl, OFFSET>, Next::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Skip::<Impl, OFFSET>)
    }
}
pub trait IEnumRegisterWordAImpl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
impl ::windows::core::RuntimeName for IEnumRegisterWordA {
    const NAME: &'static str = "Windows.Win32.UI.Input.Ime.IEnumRegisterWordA";
}
impl IEnumRegisterWordAVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumRegisterWordAImpl, const OFFSET: isize>() -> IEnumRegisterWordAVtbl {
        unsafe extern "system" fn Clone<Impl: IEnumRegisterWordAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Next<Impl: IEnumRegisterWordAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, rgregisterword: *mut REGISTERWORDA, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(ulcount, ::core::mem::transmute_copy(&rgregisterword), ::core::mem::transmute_copy(&pcfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumRegisterWordAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Skip<Impl: IEnumRegisterWordAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumRegisterWordA>, ::windows::core::GetTrustLevel, Clone::<Impl, OFFSET>, Next::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Skip::<Impl, OFFSET>)
    }
}
pub trait IEnumRegisterWordWImpl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
impl ::windows::core::RuntimeName for IEnumRegisterWordW {
    const NAME: &'static str = "Windows.Win32.UI.Input.Ime.IEnumRegisterWordW";
}
impl IEnumRegisterWordWVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumRegisterWordWImpl, const OFFSET: isize>() -> IEnumRegisterWordWVtbl {
        unsafe extern "system" fn Clone<Impl: IEnumRegisterWordWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Next<Impl: IEnumRegisterWordWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, rgregisterword: *mut REGISTERWORDW, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(ulcount, ::core::mem::transmute_copy(&rgregisterword), ::core::mem::transmute_copy(&pcfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumRegisterWordWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Skip<Impl: IEnumRegisterWordWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumRegisterWordW>, ::windows::core::GetTrustLevel, Clone::<Impl, OFFSET>, Next::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Skip::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFEClassFactoryImpl: Sized + IClassFactoryImpl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFEClassFactory {
    const NAME: &'static str = "Windows.Win32.UI.Input.Ime.IFEClassFactory";
}
#[cfg(feature = "Win32_System_Com")]
impl IFEClassFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFEClassFactoryImpl, const OFFSET: isize>() -> IFEClassFactoryVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFEClassFactory>, ::windows::core::GetTrustLevel)
    }
}
pub trait IFECommonImpl: Sized {
    fn IsDefaultIME();
    fn SetDefaultIME();
    fn InvokeWordRegDialog();
    fn InvokeDictToolDialog();
}
impl ::windows::core::RuntimeName for IFECommon {
    const NAME: &'static str = "Windows.Win32.UI.Input.Ime.IFECommon";
}
impl IFECommonVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFECommonImpl, const OFFSET: isize>() -> IFECommonVtbl {
        unsafe extern "system" fn IsDefaultIME<Impl: IFECommonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szname: super::super::super::Foundation::PSTR, cszname: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDefaultIME(&*(&szname as *const <super::super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), cszname) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultIME<Impl: IFECommonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDefaultIME() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvokeWordRegDialog<Impl: IFECommonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimedlg: *mut IMEDLG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InvokeWordRegDialog(&*(&pimedlg as *const <IMEDLG as ::windows::core::Abi>::Abi as *const <IMEDLG as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvokeDictToolDialog<Impl: IFECommonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimedlg: *mut IMEDLG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InvokeDictToolDialog(&*(&pimedlg as *const <IMEDLG as ::windows::core::Abi>::Abi as *const <IMEDLG as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFECommon>, ::windows::core::GetTrustLevel, IsDefaultIME::<Impl, OFFSET>, SetDefaultIME::<Impl, OFFSET>, InvokeWordRegDialog::<Impl, OFFSET>, InvokeDictToolDialog::<Impl, OFFSET>)
    }
}
pub trait IFEDictionaryImpl: Sized {
    fn Open();
    fn Close();
    fn GetHeader();
    fn DisplayProperty();
    fn GetPosTable();
    fn GetWords();
    fn NextWords();
    fn Create();
    fn SetHeader();
    fn ExistWord();
    fn ExistDependency();
    fn RegisterWord();
    fn RegisterDependency();
    fn GetDependencies();
    fn NextDependencies();
    fn ConvertFromOldMSIME();
    fn ConvertFromUserToSys();
}
impl ::windows::core::RuntimeName for IFEDictionary {
    const NAME: &'static str = "Windows.Win32.UI.Input.Ime.IFEDictionary";
}
impl IFEDictionaryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFEDictionaryImpl, const OFFSET: isize>() -> IFEDictionaryVtbl {
        unsafe extern "system" fn Open<Impl: IFEDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchdictpath: super::super::super::Foundation::PSTR, pshf: *mut IMESHF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Open(&*(&pchdictpath as *const <super::super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), &*(&pshf as *const <IMESHF as ::windows::core::Abi>::Abi as *const <IMESHF as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IFEDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Close() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHeader<Impl: IFEDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchdictpath: super::super::super::Foundation::PSTR, pshf: *mut IMESHF, pjfmt: *mut IMEFMT, pultype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHeader(&*(&pchdictpath as *const <super::super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), &*(&pshf as *const <IMESHF as ::windows::core::Abi>::Abi as *const <IMESHF as ::windows::core::DefaultType>::DefaultType), pjfmt, pultype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayProperty<Impl: IFEDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayProperty(&*(&hwnd as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPosTable<Impl: IFEDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prgpostbl: *mut *mut POSTBL, pcpostbl: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPosTable(&*(&prgpostbl as *const <POSTBL as ::windows::core::Abi>::Abi as *const <POSTBL as ::windows::core::DefaultType>::DefaultType), pcpostbl) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWords<Impl: IFEDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchfirst: super::super::super::Foundation::PWSTR, pwchlast: super::super::super::Foundation::PWSTR, pwchdisplay: super::super::super::Foundation::PWSTR, ulpos: u32, ulselect: u32, ulwordsrc: u32, pchbuffer: *mut u8, cbbuffer: u32, pcwrd: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWords(
                &*(&pwchfirst as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pwchlast as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pwchdisplay as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                ulpos,
                ulselect,
                ulwordsrc,
                pchbuffer,
                cbbuffer,
                pcwrd,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextWords<Impl: IFEDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchbuffer: *mut u8, cbbuffer: u32, pcwrd: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NextWords(pchbuffer, cbbuffer, pcwrd) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Impl: IFEDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchdictpath: super::super::super::Foundation::PSTR, pshf: *mut IMESHF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&pchdictpath as *const <super::super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), &*(&pshf as *const <IMESHF as ::windows::core::Abi>::Abi as *const <IMESHF as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHeader<Impl: IFEDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshf: *mut IMESHF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetHeader(&*(&pshf as *const <IMESHF as ::windows::core::Abi>::Abi as *const <IMESHF as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExistWord<Impl: IFEDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwrd: *mut IMEWRD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExistWord(&*(&pwrd as *const <IMEWRD as ::windows::core::Abi>::Abi as *const <IMEWRD as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExistDependency<Impl: IFEDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdp: *mut IMEDP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExistDependency(&*(&pdp as *const <IMEDP as ::windows::core::Abi>::Abi as *const <IMEDP as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterWord<Impl: IFEDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reg: IMEREG, pwrd: *mut IMEWRD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterWord(reg, &*(&pwrd as *const <IMEWRD as ::windows::core::Abi>::Abi as *const <IMEWRD as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterDependency<Impl: IFEDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reg: IMEREG, pdp: *mut IMEDP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterDependency(reg, &*(&pdp as *const <IMEDP as ::windows::core::Abi>::Abi as *const <IMEDP as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDependencies<Impl: IFEDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchkakarireading: super::super::super::Foundation::PWSTR, pwchkakaridisplay: super::super::super::Foundation::PWSTR, ulkakaripos: u32, pwchukereading: super::super::super::Foundation::PWSTR, pwchukedisplay: super::super::super::Foundation::PWSTR, ulukepos: u32, jrel: IMEREL, ulwordsrc: u32, pchbuffer: *mut u8, cbbuffer: u32, pcdp: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDependencies(
                &*(&pwchkakarireading as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pwchkakaridisplay as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                ulkakaripos,
                &*(&pwchukereading as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pwchukedisplay as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                ulukepos,
                jrel,
                ulwordsrc,
                pchbuffer,
                cbbuffer,
                pcdp,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextDependencies<Impl: IFEDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchbuffer: *mut u8, cbbuffer: u32, pcdp: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NextDependencies(pchbuffer, cbbuffer, pcdp) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertFromOldMSIME<Impl: IFEDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchdic: super::super::super::Foundation::PSTR, pfnlog: ::windows::core::RawPtr, reg: IMEREG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertFromOldMSIME(&*(&pchdic as *const <super::super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), &*(&pfnlog as *const <PFNLOG as ::windows::core::Abi>::Abi as *const <PFNLOG as ::windows::core::DefaultType>::DefaultType), reg) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertFromUserToSys<Impl: IFEDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertFromUserToSys() {
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
            ::windows::core::GetRuntimeClassName::<IFEDictionary>,
            ::windows::core::GetTrustLevel,
            Open::<Impl, OFFSET>,
            Close::<Impl, OFFSET>,
            GetHeader::<Impl, OFFSET>,
            DisplayProperty::<Impl, OFFSET>,
            GetPosTable::<Impl, OFFSET>,
            GetWords::<Impl, OFFSET>,
            NextWords::<Impl, OFFSET>,
            Create::<Impl, OFFSET>,
            SetHeader::<Impl, OFFSET>,
            ExistWord::<Impl, OFFSET>,
            ExistDependency::<Impl, OFFSET>,
            RegisterWord::<Impl, OFFSET>,
            RegisterDependency::<Impl, OFFSET>,
            GetDependencies::<Impl, OFFSET>,
            NextDependencies::<Impl, OFFSET>,
            ConvertFromOldMSIME::<Impl, OFFSET>,
            ConvertFromUserToSys::<Impl, OFFSET>,
        )
    }
}
pub trait IFELanguageImpl: Sized {
    fn Open();
    fn Close();
    fn GetJMorphResult();
    fn GetConversionModeCaps();
    fn GetPhonetic();
    fn GetConversion();
}
impl ::windows::core::RuntimeName for IFELanguage {
    const NAME: &'static str = "Windows.Win32.UI.Input.Ime.IFELanguage";
}
impl IFELanguageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFELanguageImpl, const OFFSET: isize>() -> IFELanguageVtbl {
        unsafe extern "system" fn Open<Impl: IFELanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Open() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IFELanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Close() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetJMorphResult<Impl: IFELanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwrequest: u32, dwcmode: u32, cwchinput: i32, pwchinput: super::super::super::Foundation::PWSTR, pfcinfo: *mut u32, ppresult: *mut *mut MORRSLT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetJMorphResult(dwrequest, dwcmode, cwchinput, &*(&pwchinput as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), pfcinfo, &*(&ppresult as *const <MORRSLT as ::windows::core::Abi>::Abi as *const <MORRSLT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConversionModeCaps<Impl: IFELanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcaps: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConversionModeCaps(pdwcaps) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPhonetic<Impl: IFELanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, string: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, start: i32, length: i32, phonetic: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPhonetic(&*(&string as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), start, length, &*(&phonetic as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConversion<Impl: IFELanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, string: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, start: i32, length: i32, result: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConversion(&*(&string as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), start, length, &*(&result as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFELanguage>, ::windows::core::GetTrustLevel, Open::<Impl, OFFSET>, Close::<Impl, OFFSET>, GetJMorphResult::<Impl, OFFSET>, GetConversionModeCaps::<Impl, OFFSET>, GetPhonetic::<Impl, OFFSET>, GetConversion::<Impl, OFFSET>)
    }
}
pub trait IImePadImpl: Sized {
    fn Request();
}
impl ::windows::core::RuntimeName for IImePad {
    const NAME: &'static str = "Windows.Win32.UI.Input.Ime.IImePad";
}
impl IImePadVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImePadImpl, const OFFSET: isize>() -> IImePadVtbl {
        unsafe extern "system" fn Request<Impl: IImePadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piimepadapplet: ::windows::core::RawPtr, reqid: IME_PAD_REQUEST_FLAGS, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Request(
                &*(&piimepadapplet as *const <IImePadApplet as ::windows::core::Abi>::Abi as *const <IImePadApplet as ::windows::core::DefaultType>::DefaultType),
                reqid,
                &*(&wparam as *const <super::super::super::Foundation::WPARAM as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::WPARAM as ::windows::core::DefaultType>::DefaultType),
                &*(&lparam as *const <super::super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IImePad>, ::windows::core::GetTrustLevel, Request::<Impl, OFFSET>)
    }
}
pub trait IImePadAppletImpl: Sized {
    fn Initialize();
    fn Terminate();
    fn GetAppletConfig();
    fn CreateUI();
    fn Notify();
}
impl ::windows::core::RuntimeName for IImePadApplet {
    const NAME: &'static str = "Windows.Win32.UI.Input.Ime.IImePadApplet";
}
impl IImePadAppletVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImePadAppletImpl, const OFFSET: isize>() -> IImePadAppletVtbl {
        unsafe extern "system" fn Initialize<Impl: IImePadAppletImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpiimepad: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&lpiimepad as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Terminate<Impl: IImePadAppletImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Terminate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAppletConfig<Impl: IImePadAppletImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpappletcfg: *mut IMEAPPLETCFG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAppletConfig(&*(&lpappletcfg as *const <IMEAPPLETCFG as ::windows::core::Abi>::Abi as *const <IMEAPPLETCFG as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateUI<Impl: IImePadAppletImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::Foundation::HWND, lpimeappletui: *mut IMEAPPLETUI) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateUI(&*(&hwndparent as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&lpimeappletui as *const <IMEAPPLETUI as ::windows::core::Abi>::Abi as *const <IMEAPPLETUI as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Notify<Impl: IImePadAppletImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpimepad: *mut ::core::ffi::c_void, notify: i32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Notify(
                &*(&lpimepad as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                notify,
                &*(&wparam as *const <super::super::super::Foundation::WPARAM as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::WPARAM as ::windows::core::DefaultType>::DefaultType),
                &*(&lparam as *const <super::super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IImePadApplet>, ::windows::core::GetTrustLevel, Initialize::<Impl, OFFSET>, Terminate::<Impl, OFFSET>, GetAppletConfig::<Impl, OFFSET>, CreateUI::<Impl, OFFSET>, Notify::<Impl, OFFSET>)
    }
}
pub trait IImePlugInDictDictionaryListImpl: Sized {
    fn GetDictionariesInUse();
    fn DeleteDictionary();
}
impl ::windows::core::RuntimeName for IImePlugInDictDictionaryList {
    const NAME: &'static str = "Windows.Win32.UI.Input.Ime.IImePlugInDictDictionaryList";
}
impl IImePlugInDictDictionaryListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImePlugInDictDictionaryListImpl, const OFFSET: isize>() -> IImePlugInDictDictionaryListVtbl {
        unsafe extern "system" fn GetDictionariesInUse<Impl: IImePlugInDictDictionaryListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prgdictionaryguid: *mut *mut super::super::super::System::Com::SAFEARRAY, prgdatecreated: *mut *mut super::super::super::System::Com::SAFEARRAY, prgfencrypted: *mut *mut super::super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDictionariesInUse(::core::mem::transmute_copy(&prgdictionaryguid), &*(&prgdatecreated as *const <super::super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType), &*(&prgfencrypted as *const <super::super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteDictionary<Impl: IImePlugInDictDictionaryListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdictionaryguid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteDictionary(&*(&bstrdictionaryguid as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IImePlugInDictDictionaryList>, ::windows::core::GetTrustLevel, GetDictionariesInUse::<Impl, OFFSET>, DeleteDictionary::<Impl, OFFSET>)
    }
}
pub trait IImeSpecifyAppletsImpl: Sized {
    fn GetAppletIIDList();
}
impl ::windows::core::RuntimeName for IImeSpecifyApplets {
    const NAME: &'static str = "Windows.Win32.UI.Input.Ime.IImeSpecifyApplets";
}
impl IImeSpecifyAppletsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImeSpecifyAppletsImpl, const OFFSET: isize>() -> IImeSpecifyAppletsVtbl {
        unsafe extern "system" fn GetAppletIIDList<Impl: IImeSpecifyAppletsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, refiid: *const ::windows::core::GUID, lpiidlist: *mut APPLETIDLIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAppletIIDList(&*(&refiid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&lpiidlist as *const <APPLETIDLIST as ::windows::core::Abi>::Abi as *const <APPLETIDLIST as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IImeSpecifyApplets>, ::windows::core::GetTrustLevel, GetAppletIIDList::<Impl, OFFSET>)
    }
}
