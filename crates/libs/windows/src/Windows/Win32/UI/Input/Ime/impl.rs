#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
impl IActiveIMEVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMEImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IActiveIMEVtbl {
        unsafe extern "system" fn Inquire<Impl: IActiveIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsysteminfoflags: u32, pimeinfo: *mut IMEINFO, szwndclass: super::super::super::Foundation::PWSTR, pdwprivate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConversionList<Impl: IActiveIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, szsource: super::super::super::Foundation::PWSTR, uflag: u32, ubuflen: u32, pdest: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Configure<Impl: IActiveIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pregisterword: *const REGISTERWORDW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Destroy<Impl: IActiveIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ureserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Escape<Impl: IActiveIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetActiveContext<Impl: IActiveIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, fflag: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProcessKey<Impl: IActiveIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, uvirkey: u32, lparam: u32, pbkeystate: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Notify<Impl: IActiveIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwaction: u32, dwindex: u32, dwvalue: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Select<Impl: IActiveIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, fselect: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCompositionString<Impl: IActiveIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ToAsciiEx<Impl: IActiveIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uvirkey: u32, uscancode: u32, pbkeystate: *const u8, fustate: u32, himc: super::super::super::Globalization::HIMC, pdwtransbuf: *mut u32, pusize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RegisterWord<Impl: IActiveIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szreading: super::super::super::Foundation::PWSTR, dwstyle: u32, szstring: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnregisterWord<Impl: IActiveIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szreading: super::super::super::Foundation::PWSTR, dwstyle: u32, szstring: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRegisterWordStyle<Impl: IActiveIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nitem: u32, pstylebuf: *mut STYLEBUFW, pubufsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumRegisterWord<Impl: IActiveIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szreading: super::super::super::Foundation::PWSTR, dwstyle: u32, szregister: super::super::super::Foundation::PWSTR, pdata: *const ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCodePageA<Impl: IActiveIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ucodepage: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLangId<Impl: IActiveIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plid: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Inquire: Inquire::<Impl, IMPL_OFFSET>,
            ConversionList: ConversionList::<Impl, IMPL_OFFSET>,
            Configure: Configure::<Impl, IMPL_OFFSET>,
            Destroy: Destroy::<Impl, IMPL_OFFSET>,
            Escape: Escape::<Impl, IMPL_OFFSET>,
            SetActiveContext: SetActiveContext::<Impl, IMPL_OFFSET>,
            ProcessKey: ProcessKey::<Impl, IMPL_OFFSET>,
            Notify: Notify::<Impl, IMPL_OFFSET>,
            Select: Select::<Impl, IMPL_OFFSET>,
            SetCompositionString: SetCompositionString::<Impl, IMPL_OFFSET>,
            ToAsciiEx: ToAsciiEx::<Impl, IMPL_OFFSET>,
            RegisterWord: RegisterWord::<Impl, IMPL_OFFSET>,
            UnregisterWord: UnregisterWord::<Impl, IMPL_OFFSET>,
            GetRegisterWordStyle: GetRegisterWordStyle::<Impl, IMPL_OFFSET>,
            EnumRegisterWord: EnumRegisterWord::<Impl, IMPL_OFFSET>,
            GetCodePageA: GetCodePageA::<Impl, IMPL_OFFSET>,
            GetLangId: GetLangId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActiveIME as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
pub trait IActiveIME2Impl: Sized + IActiveIMEImpl {
    fn Sleep();
    fn Unsleep();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
impl IActiveIME2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIME2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IActiveIME2Vtbl {
        unsafe extern "system" fn Sleep<Impl: IActiveIME2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unsleep<Impl: IActiveIME2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fdead: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IActiveIMEVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Sleep: Sleep::<Impl, IMPL_OFFSET>,
            Unsleep: Unsleep::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActiveIME2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_TextServices"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_TextServices"))]
impl IActiveIMMAppVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMAppImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IActiveIMMAppVtbl {
        unsafe extern "system" fn AssociateContext<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, hime: super::super::super::Globalization::HIMC, phprev: *mut super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConfigureIMEA<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pdata: *const REGISTERWORDA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConfigureIMEW<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pdata: *const REGISTERWORDW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateContext<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phimc: *mut super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DestroyContext<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hime: super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumRegisterWordA<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: super::super::super::Foundation::PSTR, dwstyle: u32, szregister: super::super::super::Foundation::PSTR, pdata: *const ::core::ffi::c_void, penum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumRegisterWordW<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: super::super::super::Foundation::PWSTR, dwstyle: u32, szregister: super::super::super::Foundation::PWSTR, pdata: *const ::core::ffi::c_void, penum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EscapeA<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EscapeW<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCandidateListA<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCandidateListW<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCandidateListCountA<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCandidateListCountW<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCandidateWindow<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcandidate: *mut CANDIDATEFORM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCompositionFontA<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *mut super::super::super::Graphics::Gdi::LOGFONTA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCompositionFontW<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *mut super::super::super::Graphics::Gdi::LOGFONTW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCompositionStringA<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCompositionStringW<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCompositionWindow<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pcompform: *mut COMPOSITIONFORM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetContext<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, phimc: *mut super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConversionListA<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, psrc: super::super::super::Foundation::PSTR, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConversionListW<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, psrc: super::super::super::Foundation::PWSTR, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConversionStatus<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pfdwconversion: *mut u32, pfdwsentence: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDefaultIMEWnd<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, phdefwnd: *mut super::super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDescriptionA<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szdescription: super::super::super::Foundation::PSTR, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDescriptionW<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szdescription: super::super::super::Foundation::PWSTR, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGuideLineA<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, pbuf: super::super::super::Foundation::PSTR, pdwresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGuideLineW<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, pbuf: super::super::super::Foundation::PWSTR, pdwresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIMEFileNameA<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szfilename: super::super::super::Foundation::PSTR, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIMEFileNameW<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szfilename: super::super::super::Foundation::PWSTR, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOpenStatus<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProperty<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, fdwindex: u32, pdwproperty: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRegisterWordStyleA<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, nitem: u32, pstylebuf: *mut STYLEBUFA, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRegisterWordStyleW<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, nitem: u32, pstylebuf: *mut STYLEBUFW, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStatusWindowPos<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pptpos: *mut super::super::super::Foundation::POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVirtualKey<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, puvirtualkey: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InstallIMEA<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szimefilename: super::super::super::Foundation::PSTR, szlayouttext: super::super::super::Foundation::PSTR, phkl: *mut super::super::TextServices::HKL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InstallIMEW<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szimefilename: super::super::super::Foundation::PWSTR, szlayouttext: super::super::super::Foundation::PWSTR, phkl: *mut super::super::TextServices::HKL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsIME<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsUIMessageA<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndime: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsUIMessageW<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndime: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NotifyIME<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwaction: u32, dwindex: u32, dwvalue: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RegisterWordA<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: super::super::super::Foundation::PSTR, dwstyle: u32, szregister: super::super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RegisterWordW<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: super::super::super::Foundation::PWSTR, dwstyle: u32, szregister: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseContext<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, himc: super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCandidateWindow<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pcandidate: *const CANDIDATEFORM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCompositionFontA<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *const super::super::super::Graphics::Gdi::LOGFONTA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCompositionFontW<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *const super::super::super::Graphics::Gdi::LOGFONTW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCompositionStringA<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCompositionStringW<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCompositionWindow<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pcompform: *const COMPOSITIONFORM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetConversionStatus<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, fdwconversion: u32, fdwsentence: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOpenStatus<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, fopen: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStatusWindowPos<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pptpos: *const super::super::super::Foundation::POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SimulateHotKey<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, dwhotkeyid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnregisterWordA<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: super::super::super::Foundation::PSTR, dwstyle: u32, szunregister: super::super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnregisterWordW<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: super::super::super::Foundation::PWSTR, dwstyle: u32, szunregister: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Activate<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frestorelayout: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Deactivate<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnDefWindowProc<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FilterClientWindows<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aaclasslist: *const u16, usize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCodePageA<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ucodepage: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLangId<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, plid: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AssociateContextEx<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, himc: super::super::super::Globalization::HIMC, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisableIME<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idthread: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetImeMenuItemsA<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOA, pimemenu: *mut IMEMENUITEMINFOA, dwsize: u32, pdwresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetImeMenuItemsW<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOW, pimemenu: *mut IMEMENUITEMINFOW, dwsize: u32, pdwresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumInputContext<Impl: IActiveIMMAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idthread: u32, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AssociateContext: AssociateContext::<Impl, IMPL_OFFSET>,
            ConfigureIMEA: ConfigureIMEA::<Impl, IMPL_OFFSET>,
            ConfigureIMEW: ConfigureIMEW::<Impl, IMPL_OFFSET>,
            CreateContext: CreateContext::<Impl, IMPL_OFFSET>,
            DestroyContext: DestroyContext::<Impl, IMPL_OFFSET>,
            EnumRegisterWordA: EnumRegisterWordA::<Impl, IMPL_OFFSET>,
            EnumRegisterWordW: EnumRegisterWordW::<Impl, IMPL_OFFSET>,
            EscapeA: EscapeA::<Impl, IMPL_OFFSET>,
            EscapeW: EscapeW::<Impl, IMPL_OFFSET>,
            GetCandidateListA: GetCandidateListA::<Impl, IMPL_OFFSET>,
            GetCandidateListW: GetCandidateListW::<Impl, IMPL_OFFSET>,
            GetCandidateListCountA: GetCandidateListCountA::<Impl, IMPL_OFFSET>,
            GetCandidateListCountW: GetCandidateListCountW::<Impl, IMPL_OFFSET>,
            GetCandidateWindow: GetCandidateWindow::<Impl, IMPL_OFFSET>,
            GetCompositionFontA: GetCompositionFontA::<Impl, IMPL_OFFSET>,
            GetCompositionFontW: GetCompositionFontW::<Impl, IMPL_OFFSET>,
            GetCompositionStringA: GetCompositionStringA::<Impl, IMPL_OFFSET>,
            GetCompositionStringW: GetCompositionStringW::<Impl, IMPL_OFFSET>,
            GetCompositionWindow: GetCompositionWindow::<Impl, IMPL_OFFSET>,
            GetContext: GetContext::<Impl, IMPL_OFFSET>,
            GetConversionListA: GetConversionListA::<Impl, IMPL_OFFSET>,
            GetConversionListW: GetConversionListW::<Impl, IMPL_OFFSET>,
            GetConversionStatus: GetConversionStatus::<Impl, IMPL_OFFSET>,
            GetDefaultIMEWnd: GetDefaultIMEWnd::<Impl, IMPL_OFFSET>,
            GetDescriptionA: GetDescriptionA::<Impl, IMPL_OFFSET>,
            GetDescriptionW: GetDescriptionW::<Impl, IMPL_OFFSET>,
            GetGuideLineA: GetGuideLineA::<Impl, IMPL_OFFSET>,
            GetGuideLineW: GetGuideLineW::<Impl, IMPL_OFFSET>,
            GetIMEFileNameA: GetIMEFileNameA::<Impl, IMPL_OFFSET>,
            GetIMEFileNameW: GetIMEFileNameW::<Impl, IMPL_OFFSET>,
            GetOpenStatus: GetOpenStatus::<Impl, IMPL_OFFSET>,
            GetProperty: GetProperty::<Impl, IMPL_OFFSET>,
            GetRegisterWordStyleA: GetRegisterWordStyleA::<Impl, IMPL_OFFSET>,
            GetRegisterWordStyleW: GetRegisterWordStyleW::<Impl, IMPL_OFFSET>,
            GetStatusWindowPos: GetStatusWindowPos::<Impl, IMPL_OFFSET>,
            GetVirtualKey: GetVirtualKey::<Impl, IMPL_OFFSET>,
            InstallIMEA: InstallIMEA::<Impl, IMPL_OFFSET>,
            InstallIMEW: InstallIMEW::<Impl, IMPL_OFFSET>,
            IsIME: IsIME::<Impl, IMPL_OFFSET>,
            IsUIMessageA: IsUIMessageA::<Impl, IMPL_OFFSET>,
            IsUIMessageW: IsUIMessageW::<Impl, IMPL_OFFSET>,
            NotifyIME: NotifyIME::<Impl, IMPL_OFFSET>,
            RegisterWordA: RegisterWordA::<Impl, IMPL_OFFSET>,
            RegisterWordW: RegisterWordW::<Impl, IMPL_OFFSET>,
            ReleaseContext: ReleaseContext::<Impl, IMPL_OFFSET>,
            SetCandidateWindow: SetCandidateWindow::<Impl, IMPL_OFFSET>,
            SetCompositionFontA: SetCompositionFontA::<Impl, IMPL_OFFSET>,
            SetCompositionFontW: SetCompositionFontW::<Impl, IMPL_OFFSET>,
            SetCompositionStringA: SetCompositionStringA::<Impl, IMPL_OFFSET>,
            SetCompositionStringW: SetCompositionStringW::<Impl, IMPL_OFFSET>,
            SetCompositionWindow: SetCompositionWindow::<Impl, IMPL_OFFSET>,
            SetConversionStatus: SetConversionStatus::<Impl, IMPL_OFFSET>,
            SetOpenStatus: SetOpenStatus::<Impl, IMPL_OFFSET>,
            SetStatusWindowPos: SetStatusWindowPos::<Impl, IMPL_OFFSET>,
            SimulateHotKey: SimulateHotKey::<Impl, IMPL_OFFSET>,
            UnregisterWordA: UnregisterWordA::<Impl, IMPL_OFFSET>,
            UnregisterWordW: UnregisterWordW::<Impl, IMPL_OFFSET>,
            Activate: Activate::<Impl, IMPL_OFFSET>,
            Deactivate: Deactivate::<Impl, IMPL_OFFSET>,
            OnDefWindowProc: OnDefWindowProc::<Impl, IMPL_OFFSET>,
            FilterClientWindows: FilterClientWindows::<Impl, IMPL_OFFSET>,
            GetCodePageA: GetCodePageA::<Impl, IMPL_OFFSET>,
            GetLangId: GetLangId::<Impl, IMPL_OFFSET>,
            AssociateContextEx: AssociateContextEx::<Impl, IMPL_OFFSET>,
            DisableIME: DisableIME::<Impl, IMPL_OFFSET>,
            GetImeMenuItemsA: GetImeMenuItemsA::<Impl, IMPL_OFFSET>,
            GetImeMenuItemsW: GetImeMenuItemsW::<Impl, IMPL_OFFSET>,
            EnumInputContext: EnumInputContext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActiveIMMApp as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_TextServices"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_TextServices"))]
impl IActiveIMMIMEVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIMEImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IActiveIMMIMEVtbl {
        unsafe extern "system" fn AssociateContext<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, hime: super::super::super::Globalization::HIMC, phprev: *mut super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConfigureIMEA<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pdata: *const REGISTERWORDA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConfigureIMEW<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pdata: *const REGISTERWORDW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateContext<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phimc: *mut super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DestroyContext<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hime: super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumRegisterWordA<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: super::super::super::Foundation::PSTR, dwstyle: u32, szregister: super::super::super::Foundation::PSTR, pdata: *const ::core::ffi::c_void, penum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumRegisterWordW<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: super::super::super::Foundation::PWSTR, dwstyle: u32, szregister: super::super::super::Foundation::PWSTR, pdata: *const ::core::ffi::c_void, penum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EscapeA<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EscapeW<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCandidateListA<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCandidateListW<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCandidateListCountA<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCandidateListCountW<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCandidateWindow<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcandidate: *mut CANDIDATEFORM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCompositionFontA<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *mut super::super::super::Graphics::Gdi::LOGFONTA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCompositionFontW<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *mut super::super::super::Graphics::Gdi::LOGFONTW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCompositionStringA<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCompositionStringW<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCompositionWindow<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pcompform: *mut COMPOSITIONFORM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetContext<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, phimc: *mut super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConversionListA<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, psrc: super::super::super::Foundation::PSTR, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConversionListW<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, psrc: super::super::super::Foundation::PWSTR, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConversionStatus<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pfdwconversion: *mut u32, pfdwsentence: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDefaultIMEWnd<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, phdefwnd: *mut super::super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDescriptionA<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szdescription: super::super::super::Foundation::PSTR, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDescriptionW<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szdescription: super::super::super::Foundation::PWSTR, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGuideLineA<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, pbuf: super::super::super::Foundation::PSTR, pdwresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGuideLineW<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, pbuf: super::super::super::Foundation::PWSTR, pdwresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIMEFileNameA<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szfilename: super::super::super::Foundation::PSTR, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIMEFileNameW<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szfilename: super::super::super::Foundation::PWSTR, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOpenStatus<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProperty<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, fdwindex: u32, pdwproperty: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRegisterWordStyleA<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, nitem: u32, pstylebuf: *mut STYLEBUFA, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRegisterWordStyleW<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, nitem: u32, pstylebuf: *mut STYLEBUFW, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStatusWindowPos<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pptpos: *mut super::super::super::Foundation::POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVirtualKey<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, puvirtualkey: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InstallIMEA<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szimefilename: super::super::super::Foundation::PSTR, szlayouttext: super::super::super::Foundation::PSTR, phkl: *mut super::super::TextServices::HKL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InstallIMEW<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szimefilename: super::super::super::Foundation::PWSTR, szlayouttext: super::super::super::Foundation::PWSTR, phkl: *mut super::super::TextServices::HKL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsIME<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsUIMessageA<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndime: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsUIMessageW<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndime: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NotifyIME<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwaction: u32, dwindex: u32, dwvalue: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RegisterWordA<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: super::super::super::Foundation::PSTR, dwstyle: u32, szregister: super::super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RegisterWordW<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: super::super::super::Foundation::PWSTR, dwstyle: u32, szregister: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseContext<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, himc: super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCandidateWindow<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pcandidate: *const CANDIDATEFORM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCompositionFontA<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *const super::super::super::Graphics::Gdi::LOGFONTA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCompositionFontW<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *const super::super::super::Graphics::Gdi::LOGFONTW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCompositionStringA<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCompositionStringW<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCompositionWindow<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pcompform: *const COMPOSITIONFORM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetConversionStatus<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, fdwconversion: u32, fdwsentence: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOpenStatus<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, fopen: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStatusWindowPos<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pptpos: *const super::super::super::Foundation::POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SimulateHotKey<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, dwhotkeyid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnregisterWordA<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: super::super::super::Foundation::PSTR, dwstyle: u32, szunregister: super::super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnregisterWordW<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: super::super::super::Foundation::PWSTR, dwstyle: u32, szunregister: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GenerateMessage<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LockIMC<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, ppimc: *mut *mut INPUTCONTEXT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnlockIMC<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIMCLockCount<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pdwlockcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateIMCC<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsize: u32, phimcc: *mut super::super::super::Globalization::HIMCC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DestroyIMCC<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himcc: super::super::super::Globalization::HIMCC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LockIMCC<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himcc: super::super::super::Globalization::HIMCC, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnlockIMCC<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himcc: super::super::super::Globalization::HIMCC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReSizeIMCC<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himcc: super::super::super::Globalization::HIMCC, dwsize: u32, phimcc: *mut super::super::super::Globalization::HIMCC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIMCCSize<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himcc: super::super::super::Globalization::HIMCC, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIMCCLockCount<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himcc: super::super::super::Globalization::HIMCC, pdwlockcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHotKey<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwhotkeyid: u32, pumodifiers: *mut u32, puvkey: *mut u32, phkl: *mut super::super::TextServices::HKL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHotKey<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwhotkeyid: u32, umodifiers: u32, uvkey: u32, hkl: super::super::TextServices::HKL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSoftKeyboard<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, utype: u32, howner: super::super::super::Foundation::HWND, x: i32, y: i32, phsoftkbdwnd: *mut super::super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DestroySoftKeyboard<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hsoftkbdwnd: super::super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ShowSoftKeyboard<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hsoftkbdwnd: super::super::super::Foundation::HWND, ncmdshow: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCodePageA<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ucodepage: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLangId<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, plid: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn KeybdEvent<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lgidime: u16, bvk: u8, bscan: u8, dwflags: u32, dwextrainfo: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LockModal<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnlockModal<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AssociateContextEx<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, himc: super::super::super::Globalization::HIMC, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisableIME<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idthread: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetImeMenuItemsA<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOA, pimemenu: *mut IMEMENUITEMINFOA, dwsize: u32, pdwresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetImeMenuItemsW<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOW, pimemenu: *mut IMEMENUITEMINFOW, dwsize: u32, pdwresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumInputContext<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idthread: u32, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequestMessageA<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequestMessageW<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SendIMCA<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, umsg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SendIMCW<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, umsg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsSleeping<Impl: IActiveIMMIMEImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AssociateContext: AssociateContext::<Impl, IMPL_OFFSET>,
            ConfigureIMEA: ConfigureIMEA::<Impl, IMPL_OFFSET>,
            ConfigureIMEW: ConfigureIMEW::<Impl, IMPL_OFFSET>,
            CreateContext: CreateContext::<Impl, IMPL_OFFSET>,
            DestroyContext: DestroyContext::<Impl, IMPL_OFFSET>,
            EnumRegisterWordA: EnumRegisterWordA::<Impl, IMPL_OFFSET>,
            EnumRegisterWordW: EnumRegisterWordW::<Impl, IMPL_OFFSET>,
            EscapeA: EscapeA::<Impl, IMPL_OFFSET>,
            EscapeW: EscapeW::<Impl, IMPL_OFFSET>,
            GetCandidateListA: GetCandidateListA::<Impl, IMPL_OFFSET>,
            GetCandidateListW: GetCandidateListW::<Impl, IMPL_OFFSET>,
            GetCandidateListCountA: GetCandidateListCountA::<Impl, IMPL_OFFSET>,
            GetCandidateListCountW: GetCandidateListCountW::<Impl, IMPL_OFFSET>,
            GetCandidateWindow: GetCandidateWindow::<Impl, IMPL_OFFSET>,
            GetCompositionFontA: GetCompositionFontA::<Impl, IMPL_OFFSET>,
            GetCompositionFontW: GetCompositionFontW::<Impl, IMPL_OFFSET>,
            GetCompositionStringA: GetCompositionStringA::<Impl, IMPL_OFFSET>,
            GetCompositionStringW: GetCompositionStringW::<Impl, IMPL_OFFSET>,
            GetCompositionWindow: GetCompositionWindow::<Impl, IMPL_OFFSET>,
            GetContext: GetContext::<Impl, IMPL_OFFSET>,
            GetConversionListA: GetConversionListA::<Impl, IMPL_OFFSET>,
            GetConversionListW: GetConversionListW::<Impl, IMPL_OFFSET>,
            GetConversionStatus: GetConversionStatus::<Impl, IMPL_OFFSET>,
            GetDefaultIMEWnd: GetDefaultIMEWnd::<Impl, IMPL_OFFSET>,
            GetDescriptionA: GetDescriptionA::<Impl, IMPL_OFFSET>,
            GetDescriptionW: GetDescriptionW::<Impl, IMPL_OFFSET>,
            GetGuideLineA: GetGuideLineA::<Impl, IMPL_OFFSET>,
            GetGuideLineW: GetGuideLineW::<Impl, IMPL_OFFSET>,
            GetIMEFileNameA: GetIMEFileNameA::<Impl, IMPL_OFFSET>,
            GetIMEFileNameW: GetIMEFileNameW::<Impl, IMPL_OFFSET>,
            GetOpenStatus: GetOpenStatus::<Impl, IMPL_OFFSET>,
            GetProperty: GetProperty::<Impl, IMPL_OFFSET>,
            GetRegisterWordStyleA: GetRegisterWordStyleA::<Impl, IMPL_OFFSET>,
            GetRegisterWordStyleW: GetRegisterWordStyleW::<Impl, IMPL_OFFSET>,
            GetStatusWindowPos: GetStatusWindowPos::<Impl, IMPL_OFFSET>,
            GetVirtualKey: GetVirtualKey::<Impl, IMPL_OFFSET>,
            InstallIMEA: InstallIMEA::<Impl, IMPL_OFFSET>,
            InstallIMEW: InstallIMEW::<Impl, IMPL_OFFSET>,
            IsIME: IsIME::<Impl, IMPL_OFFSET>,
            IsUIMessageA: IsUIMessageA::<Impl, IMPL_OFFSET>,
            IsUIMessageW: IsUIMessageW::<Impl, IMPL_OFFSET>,
            NotifyIME: NotifyIME::<Impl, IMPL_OFFSET>,
            RegisterWordA: RegisterWordA::<Impl, IMPL_OFFSET>,
            RegisterWordW: RegisterWordW::<Impl, IMPL_OFFSET>,
            ReleaseContext: ReleaseContext::<Impl, IMPL_OFFSET>,
            SetCandidateWindow: SetCandidateWindow::<Impl, IMPL_OFFSET>,
            SetCompositionFontA: SetCompositionFontA::<Impl, IMPL_OFFSET>,
            SetCompositionFontW: SetCompositionFontW::<Impl, IMPL_OFFSET>,
            SetCompositionStringA: SetCompositionStringA::<Impl, IMPL_OFFSET>,
            SetCompositionStringW: SetCompositionStringW::<Impl, IMPL_OFFSET>,
            SetCompositionWindow: SetCompositionWindow::<Impl, IMPL_OFFSET>,
            SetConversionStatus: SetConversionStatus::<Impl, IMPL_OFFSET>,
            SetOpenStatus: SetOpenStatus::<Impl, IMPL_OFFSET>,
            SetStatusWindowPos: SetStatusWindowPos::<Impl, IMPL_OFFSET>,
            SimulateHotKey: SimulateHotKey::<Impl, IMPL_OFFSET>,
            UnregisterWordA: UnregisterWordA::<Impl, IMPL_OFFSET>,
            UnregisterWordW: UnregisterWordW::<Impl, IMPL_OFFSET>,
            GenerateMessage: GenerateMessage::<Impl, IMPL_OFFSET>,
            LockIMC: LockIMC::<Impl, IMPL_OFFSET>,
            UnlockIMC: UnlockIMC::<Impl, IMPL_OFFSET>,
            GetIMCLockCount: GetIMCLockCount::<Impl, IMPL_OFFSET>,
            CreateIMCC: CreateIMCC::<Impl, IMPL_OFFSET>,
            DestroyIMCC: DestroyIMCC::<Impl, IMPL_OFFSET>,
            LockIMCC: LockIMCC::<Impl, IMPL_OFFSET>,
            UnlockIMCC: UnlockIMCC::<Impl, IMPL_OFFSET>,
            ReSizeIMCC: ReSizeIMCC::<Impl, IMPL_OFFSET>,
            GetIMCCSize: GetIMCCSize::<Impl, IMPL_OFFSET>,
            GetIMCCLockCount: GetIMCCLockCount::<Impl, IMPL_OFFSET>,
            GetHotKey: GetHotKey::<Impl, IMPL_OFFSET>,
            SetHotKey: SetHotKey::<Impl, IMPL_OFFSET>,
            CreateSoftKeyboard: CreateSoftKeyboard::<Impl, IMPL_OFFSET>,
            DestroySoftKeyboard: DestroySoftKeyboard::<Impl, IMPL_OFFSET>,
            ShowSoftKeyboard: ShowSoftKeyboard::<Impl, IMPL_OFFSET>,
            GetCodePageA: GetCodePageA::<Impl, IMPL_OFFSET>,
            GetLangId: GetLangId::<Impl, IMPL_OFFSET>,
            KeybdEvent: KeybdEvent::<Impl, IMPL_OFFSET>,
            LockModal: LockModal::<Impl, IMPL_OFFSET>,
            UnlockModal: UnlockModal::<Impl, IMPL_OFFSET>,
            AssociateContextEx: AssociateContextEx::<Impl, IMPL_OFFSET>,
            DisableIME: DisableIME::<Impl, IMPL_OFFSET>,
            GetImeMenuItemsA: GetImeMenuItemsA::<Impl, IMPL_OFFSET>,
            GetImeMenuItemsW: GetImeMenuItemsW::<Impl, IMPL_OFFSET>,
            EnumInputContext: EnumInputContext::<Impl, IMPL_OFFSET>,
            RequestMessageA: RequestMessageA::<Impl, IMPL_OFFSET>,
            RequestMessageW: RequestMessageW::<Impl, IMPL_OFFSET>,
            SendIMCA: SendIMCA::<Impl, IMPL_OFFSET>,
            SendIMCW: SendIMCW::<Impl, IMPL_OFFSET>,
            IsSleeping: IsSleeping::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActiveIMMIME as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IActiveIMMMessagePumpOwnerImpl: Sized {
    fn Start();
    fn End();
    fn OnTranslateMessage();
    fn Pause();
    fn Resume();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IActiveIMMMessagePumpOwnerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMMessagePumpOwnerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IActiveIMMMessagePumpOwnerVtbl {
        unsafe extern "system" fn Start<Impl: IActiveIMMMessagePumpOwnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn End<Impl: IActiveIMMMessagePumpOwnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnTranslateMessage<Impl: IActiveIMMMessagePumpOwnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *const super::super::WindowsAndMessaging::MSG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Pause<Impl: IActiveIMMMessagePumpOwnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Resume<Impl: IActiveIMMMessagePumpOwnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Start: Start::<Impl, IMPL_OFFSET>,
            End: End::<Impl, IMPL_OFFSET>,
            OnTranslateMessage: OnTranslateMessage::<Impl, IMPL_OFFSET>,
            Pause: Pause::<Impl, IMPL_OFFSET>,
            Resume: Resume::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActiveIMMMessagePumpOwner as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IActiveIMMRegistrarImpl: Sized {
    fn RegisterIME();
    fn UnregisterIME();
}
#[cfg(feature = "Win32_Foundation")]
impl IActiveIMMRegistrarVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMRegistrarImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IActiveIMMRegistrarVtbl {
        unsafe extern "system" fn RegisterIME<Impl: IActiveIMMRegistrarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, lgid: u16, psziconfile: super::super::super::Foundation::PWSTR, pszdesc: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnregisterIME<Impl: IActiveIMMRegistrarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            RegisterIME: RegisterIME::<Impl, IMPL_OFFSET>,
            UnregisterIME: UnregisterIME::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActiveIMMRegistrar as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Globalization")]
pub trait IEnumInputContextImpl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
#[cfg(feature = "Win32_Globalization")]
impl IEnumInputContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumInputContextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumInputContextVtbl {
        unsafe extern "system" fn Clone<Impl: IEnumInputContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Next<Impl: IEnumInputContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, rginputcontext: *mut super::super::super::Globalization::HIMC, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumInputContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumInputContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
        iid == &<IEnumInputContext as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEnumRegisterWordAImpl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
#[cfg(feature = "Win32_Foundation")]
impl IEnumRegisterWordAVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumRegisterWordAImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumRegisterWordAVtbl {
        unsafe extern "system" fn Clone<Impl: IEnumRegisterWordAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Next<Impl: IEnumRegisterWordAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, rgregisterword: *mut REGISTERWORDA, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumRegisterWordAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumRegisterWordAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
        iid == &<IEnumRegisterWordA as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEnumRegisterWordWImpl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
#[cfg(feature = "Win32_Foundation")]
impl IEnumRegisterWordWVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumRegisterWordWImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumRegisterWordWVtbl {
        unsafe extern "system" fn Clone<Impl: IEnumRegisterWordWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Next<Impl: IEnumRegisterWordWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, rgregisterword: *mut REGISTERWORDW, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumRegisterWordWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumRegisterWordWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
        iid == &<IEnumRegisterWordW as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IFEClassFactoryImpl: Sized + IClassFactoryImpl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IFEClassFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFEClassFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFEClassFactoryVtbl {
        Self { base: IClassFactoryVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFEClassFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IFECommonImpl: Sized {
    fn IsDefaultIME();
    fn SetDefaultIME();
    fn InvokeWordRegDialog();
    fn InvokeDictToolDialog();
}
#[cfg(feature = "Win32_Foundation")]
impl IFECommonVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFECommonImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFECommonVtbl {
        unsafe extern "system" fn IsDefaultIME<Impl: IFECommonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szname: super::super::super::Foundation::PSTR, cszname: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDefaultIME<Impl: IFECommonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InvokeWordRegDialog<Impl: IFECommonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimedlg: *mut IMEDLG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InvokeDictToolDialog<Impl: IFECommonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimedlg: *mut IMEDLG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            IsDefaultIME: IsDefaultIME::<Impl, IMPL_OFFSET>,
            SetDefaultIME: SetDefaultIME::<Impl, IMPL_OFFSET>,
            InvokeWordRegDialog: InvokeWordRegDialog::<Impl, IMPL_OFFSET>,
            InvokeDictToolDialog: InvokeDictToolDialog::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFECommon as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IFEDictionaryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFEDictionaryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFEDictionaryVtbl {
        unsafe extern "system" fn Open<Impl: IFEDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchdictpath: super::super::super::Foundation::PSTR, pshf: *mut IMESHF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: IFEDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHeader<Impl: IFEDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchdictpath: super::super::super::Foundation::PSTR, pshf: *mut IMESHF, pjfmt: *mut IMEFMT, pultype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisplayProperty<Impl: IFEDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPosTable<Impl: IFEDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prgpostbl: *mut *mut POSTBL, pcpostbl: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetWords<Impl: IFEDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchfirst: super::super::super::Foundation::PWSTR, pwchlast: super::super::super::Foundation::PWSTR, pwchdisplay: super::super::super::Foundation::PWSTR, ulpos: u32, ulselect: u32, ulwordsrc: u32, pchbuffer: *mut u8, cbbuffer: u32, pcwrd: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NextWords<Impl: IFEDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchbuffer: *mut u8, cbbuffer: u32, pcwrd: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Create<Impl: IFEDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchdictpath: super::super::super::Foundation::PSTR, pshf: *mut IMESHF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHeader<Impl: IFEDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshf: *mut IMESHF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExistWord<Impl: IFEDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwrd: *mut IMEWRD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExistDependency<Impl: IFEDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdp: *mut IMEDP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RegisterWord<Impl: IFEDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reg: IMEREG, pwrd: *mut IMEWRD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RegisterDependency<Impl: IFEDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reg: IMEREG, pdp: *mut IMEDP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDependencies<Impl: IFEDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchkakarireading: super::super::super::Foundation::PWSTR, pwchkakaridisplay: super::super::super::Foundation::PWSTR, ulkakaripos: u32, pwchukereading: super::super::super::Foundation::PWSTR, pwchukedisplay: super::super::super::Foundation::PWSTR, ulukepos: u32, jrel: IMEREL, ulwordsrc: u32, pchbuffer: *mut u8, cbbuffer: u32, pcdp: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NextDependencies<Impl: IFEDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchbuffer: *mut u8, cbbuffer: u32, pcdp: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConvertFromOldMSIME<Impl: IFEDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchdic: super::super::super::Foundation::PSTR, pfnlog: ::windows::core::RawPtr, reg: IMEREG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConvertFromUserToSys<Impl: IFEDictionaryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Open: Open::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
            GetHeader: GetHeader::<Impl, IMPL_OFFSET>,
            DisplayProperty: DisplayProperty::<Impl, IMPL_OFFSET>,
            GetPosTable: GetPosTable::<Impl, IMPL_OFFSET>,
            GetWords: GetWords::<Impl, IMPL_OFFSET>,
            NextWords: NextWords::<Impl, IMPL_OFFSET>,
            Create: Create::<Impl, IMPL_OFFSET>,
            SetHeader: SetHeader::<Impl, IMPL_OFFSET>,
            ExistWord: ExistWord::<Impl, IMPL_OFFSET>,
            ExistDependency: ExistDependency::<Impl, IMPL_OFFSET>,
            RegisterWord: RegisterWord::<Impl, IMPL_OFFSET>,
            RegisterDependency: RegisterDependency::<Impl, IMPL_OFFSET>,
            GetDependencies: GetDependencies::<Impl, IMPL_OFFSET>,
            NextDependencies: NextDependencies::<Impl, IMPL_OFFSET>,
            ConvertFromOldMSIME: ConvertFromOldMSIME::<Impl, IMPL_OFFSET>,
            ConvertFromUserToSys: ConvertFromUserToSys::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFEDictionary as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IFELanguageImpl: Sized {
    fn Open();
    fn Close();
    fn GetJMorphResult();
    fn GetConversionModeCaps();
    fn GetPhonetic();
    fn GetConversion();
}
#[cfg(feature = "Win32_Foundation")]
impl IFELanguageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFELanguageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFELanguageVtbl {
        unsafe extern "system" fn Open<Impl: IFELanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: IFELanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetJMorphResult<Impl: IFELanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwrequest: u32, dwcmode: u32, cwchinput: i32, pwchinput: super::super::super::Foundation::PWSTR, pfcinfo: *mut u32, ppresult: *mut *mut MORRSLT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConversionModeCaps<Impl: IFELanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcaps: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPhonetic<Impl: IFELanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, string: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, start: i32, length: i32, phonetic: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConversion<Impl: IFELanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, string: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, start: i32, length: i32, result: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Open: Open::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
            GetJMorphResult: GetJMorphResult::<Impl, IMPL_OFFSET>,
            GetConversionModeCaps: GetConversionModeCaps::<Impl, IMPL_OFFSET>,
            GetPhonetic: GetPhonetic::<Impl, IMPL_OFFSET>,
            GetConversion: GetConversion::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFELanguage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IImePadImpl: Sized {
    fn Request();
}
#[cfg(feature = "Win32_Foundation")]
impl IImePadVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImePadImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IImePadVtbl {
        unsafe extern "system" fn Request<Impl: IImePadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piimepadapplet: ::windows::core::RawPtr, reqid: IME_PAD_REQUEST_FLAGS, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Request: Request::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IImePad as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IImePadAppletImpl: Sized {
    fn Initialize();
    fn Terminate();
    fn GetAppletConfig();
    fn CreateUI();
    fn Notify();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IImePadAppletVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImePadAppletImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IImePadAppletVtbl {
        unsafe extern "system" fn Initialize<Impl: IImePadAppletImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpiimepad: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Terminate<Impl: IImePadAppletImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAppletConfig<Impl: IImePadAppletImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpappletcfg: *mut IMEAPPLETCFG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateUI<Impl: IImePadAppletImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::Foundation::HWND, lpimeappletui: *mut IMEAPPLETUI) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Notify<Impl: IImePadAppletImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpimepad: *mut ::core::ffi::c_void, notify: i32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            Terminate: Terminate::<Impl, IMPL_OFFSET>,
            GetAppletConfig: GetAppletConfig::<Impl, IMPL_OFFSET>,
            CreateUI: CreateUI::<Impl, IMPL_OFFSET>,
            Notify: Notify::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IImePadApplet as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IImePlugInDictDictionaryListImpl: Sized {
    fn GetDictionariesInUse();
    fn DeleteDictionary();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IImePlugInDictDictionaryListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImePlugInDictDictionaryListImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IImePlugInDictDictionaryListVtbl {
        unsafe extern "system" fn GetDictionariesInUse<Impl: IImePlugInDictDictionaryListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prgdictionaryguid: *mut *mut super::super::super::System::Com::SAFEARRAY, prgdatecreated: *mut *mut super::super::super::System::Com::SAFEARRAY, prgfencrypted: *mut *mut super::super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteDictionary<Impl: IImePlugInDictDictionaryListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdictionaryguid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetDictionariesInUse: GetDictionariesInUse::<Impl, IMPL_OFFSET>,
            DeleteDictionary: DeleteDictionary::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IImePlugInDictDictionaryList as ::windows::core::Interface>::IID
    }
}
pub trait IImeSpecifyAppletsImpl: Sized {
    fn GetAppletIIDList();
}
impl IImeSpecifyAppletsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImeSpecifyAppletsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IImeSpecifyAppletsVtbl {
        unsafe extern "system" fn GetAppletIIDList<Impl: IImeSpecifyAppletsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, refiid: *const ::windows::core::GUID, lpiidlist: *mut APPLETIDLIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetAppletIIDList: GetAppletIIDList::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IImeSpecifyApplets as ::windows::core::Interface>::IID
    }
}
