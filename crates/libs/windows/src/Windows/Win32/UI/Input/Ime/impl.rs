#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
pub trait IActiveIME_Impl: Sized {
    fn Inquire(&self, dwsysteminfoflags: u32, pimeinfo: *mut IMEINFO, szwndclass: ::windows::core::PWSTR, pdwprivate: *mut u32) -> ::windows::core::Result<()>;
    fn ConversionList(&self, himc: super::super::super::Globalization::HIMC, szsource: &::windows::core::PCWSTR, uflag: u32, ubuflen: u32, pdest: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::Result<()>;
    fn Configure(&self, hkl: super::super::TextServices::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pregisterword: *const REGISTERWORDW) -> ::windows::core::Result<()>;
    fn Destroy(&self, ureserved: u32) -> ::windows::core::Result<()>;
    fn Escape(&self, himc: super::super::super::Globalization::HIMC, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::Result<()>;
    fn SetActiveContext(&self, himc: super::super::super::Globalization::HIMC, fflag: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn ProcessKey(&self, himc: super::super::super::Globalization::HIMC, uvirkey: u32, lparam: u32, pbkeystate: *const u8) -> ::windows::core::Result<()>;
    fn Notify(&self, himc: super::super::super::Globalization::HIMC, dwaction: u32, dwindex: u32, dwvalue: u32) -> ::windows::core::Result<()>;
    fn Select(&self, himc: super::super::super::Globalization::HIMC, fselect: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetCompositionString(&self, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows::core::Result<()>;
    fn ToAsciiEx(&self, uvirkey: u32, uscancode: u32, pbkeystate: *const u8, fustate: u32, himc: super::super::super::Globalization::HIMC, pdwtransbuf: *mut u32, pusize: *mut u32) -> ::windows::core::Result<()>;
    fn RegisterWord(&self, szreading: &::windows::core::PCWSTR, dwstyle: u32, szstring: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn UnregisterWord(&self, szreading: &::windows::core::PCWSTR, dwstyle: u32, szstring: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetRegisterWordStyle(&self, nitem: u32, pstylebuf: *mut STYLEBUFW, pubufsize: *mut u32) -> ::windows::core::Result<()>;
    fn EnumRegisterWord(&self, szreading: &::windows::core::PCWSTR, dwstyle: u32, szregister: &::windows::core::PCWSTR, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<IEnumRegisterWordW>;
    fn GetCodePageA(&self) -> ::windows::core::Result<u32>;
    fn GetLangId(&self) -> ::windows::core::Result<u16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
impl IActiveIME_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIME_Impl, const OFFSET: isize>() -> IActiveIME_Vtbl {
        unsafe extern "system" fn Inquire<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsysteminfoflags: u32, pimeinfo: *mut IMEINFO, szwndclass: ::windows::core::PWSTR, pdwprivate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Inquire(::core::mem::transmute_copy(&dwsysteminfoflags), ::core::mem::transmute_copy(&pimeinfo), ::core::mem::transmute_copy(&szwndclass), ::core::mem::transmute_copy(&pdwprivate)).into()
        }
        unsafe extern "system" fn ConversionList<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, szsource: ::windows::core::PCWSTR, uflag: u32, ubuflen: u32, pdest: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ConversionList(::core::mem::transmute_copy(&himc), ::core::mem::transmute(&szsource), ::core::mem::transmute_copy(&uflag), ::core::mem::transmute_copy(&ubuflen), ::core::mem::transmute_copy(&pdest), ::core::mem::transmute_copy(&pucopied)).into()
        }
        unsafe extern "system" fn Configure<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pregisterword: *const REGISTERWORDW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Configure(::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&dwmode), ::core::mem::transmute_copy(&pregisterword)).into()
        }
        unsafe extern "system" fn Destroy<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ureserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Destroy(::core::mem::transmute_copy(&ureserved)).into()
        }
        unsafe extern "system" fn Escape<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Escape(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&uescape), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&plresult)).into()
        }
        unsafe extern "system" fn SetActiveContext<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, fflag: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetActiveContext(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&fflag)).into()
        }
        unsafe extern "system" fn ProcessKey<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, uvirkey: u32, lparam: u32, pbkeystate: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ProcessKey(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&uvirkey), ::core::mem::transmute_copy(&lparam), ::core::mem::transmute_copy(&pbkeystate)).into()
        }
        unsafe extern "system" fn Notify<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwaction: u32, dwindex: u32, dwvalue: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Notify(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwaction), ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&dwvalue)).into()
        }
        unsafe extern "system" fn Select<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, fselect: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Select(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&fselect)).into()
        }
        unsafe extern "system" fn SetCompositionString<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCompositionString(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pcomp), ::core::mem::transmute_copy(&dwcomplen), ::core::mem::transmute_copy(&pread), ::core::mem::transmute_copy(&dwreadlen)).into()
        }
        unsafe extern "system" fn ToAsciiEx<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uvirkey: u32, uscancode: u32, pbkeystate: *const u8, fustate: u32, himc: super::super::super::Globalization::HIMC, pdwtransbuf: *mut u32, pusize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ToAsciiEx(::core::mem::transmute_copy(&uvirkey), ::core::mem::transmute_copy(&uscancode), ::core::mem::transmute_copy(&pbkeystate), ::core::mem::transmute_copy(&fustate), ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&pdwtransbuf), ::core::mem::transmute_copy(&pusize)).into()
        }
        unsafe extern "system" fn RegisterWord<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szreading: ::windows::core::PCWSTR, dwstyle: u32, szstring: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RegisterWord(::core::mem::transmute(&szreading), ::core::mem::transmute_copy(&dwstyle), ::core::mem::transmute(&szstring)).into()
        }
        unsafe extern "system" fn UnregisterWord<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szreading: ::windows::core::PCWSTR, dwstyle: u32, szstring: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnregisterWord(::core::mem::transmute(&szreading), ::core::mem::transmute_copy(&dwstyle), ::core::mem::transmute(&szstring)).into()
        }
        unsafe extern "system" fn GetRegisterWordStyle<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nitem: u32, pstylebuf: *mut STYLEBUFW, pubufsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRegisterWordStyle(::core::mem::transmute_copy(&nitem), ::core::mem::transmute_copy(&pstylebuf), ::core::mem::transmute_copy(&pubufsize)).into()
        }
        unsafe extern "system" fn EnumRegisterWord<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szreading: ::windows::core::PCWSTR, dwstyle: u32, szregister: ::windows::core::PCWSTR, pdata: *const ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumRegisterWord(::core::mem::transmute(&szreading), ::core::mem::transmute_copy(&dwstyle), ::core::mem::transmute(&szregister), ::core::mem::transmute_copy(&pdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCodePageA<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ucodepage: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCodePageA() {
                ::core::result::Result::Ok(ok__) => {
                    *ucodepage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLangId<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plid: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLangId() {
                ::core::result::Result::Ok(ok__) => {
                    *plid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Inquire: Inquire::<Identity, Impl, OFFSET>,
            ConversionList: ConversionList::<Identity, Impl, OFFSET>,
            Configure: Configure::<Identity, Impl, OFFSET>,
            Destroy: Destroy::<Identity, Impl, OFFSET>,
            Escape: Escape::<Identity, Impl, OFFSET>,
            SetActiveContext: SetActiveContext::<Identity, Impl, OFFSET>,
            ProcessKey: ProcessKey::<Identity, Impl, OFFSET>,
            Notify: Notify::<Identity, Impl, OFFSET>,
            Select: Select::<Identity, Impl, OFFSET>,
            SetCompositionString: SetCompositionString::<Identity, Impl, OFFSET>,
            ToAsciiEx: ToAsciiEx::<Identity, Impl, OFFSET>,
            RegisterWord: RegisterWord::<Identity, Impl, OFFSET>,
            UnregisterWord: UnregisterWord::<Identity, Impl, OFFSET>,
            GetRegisterWordStyle: GetRegisterWordStyle::<Identity, Impl, OFFSET>,
            EnumRegisterWord: EnumRegisterWord::<Identity, Impl, OFFSET>,
            GetCodePageA: GetCodePageA::<Identity, Impl, OFFSET>,
            GetLangId: GetLangId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActiveIME as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
pub trait IActiveIME2_Impl: Sized + IActiveIME_Impl {
    fn Sleep(&self) -> ::windows::core::Result<()>;
    fn Unsleep(&self, fdead: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
impl IActiveIME2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIME2_Impl, const OFFSET: isize>() -> IActiveIME2_Vtbl {
        unsafe extern "system" fn Sleep<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIME2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Sleep().into()
        }
        unsafe extern "system" fn Unsleep<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIME2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fdead: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Unsleep(::core::mem::transmute_copy(&fdead)).into()
        }
        Self { base: IActiveIME_Vtbl::new::<Identity, Impl, OFFSET>(), Sleep: Sleep::<Identity, Impl, OFFSET>, Unsleep: Unsleep::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActiveIME2 as ::windows::core::Interface>::IID || iid == &<IActiveIME as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_TextServices"))]
pub trait IActiveIMMApp_Impl: Sized {
    fn AssociateContext(&self, hwnd: super::super::super::Foundation::HWND, hime: super::super::super::Globalization::HIMC) -> ::windows::core::Result<super::super::super::Globalization::HIMC>;
    fn ConfigureIMEA(&self, hkl: super::super::TextServices::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pdata: *const REGISTERWORDA) -> ::windows::core::Result<()>;
    fn ConfigureIMEW(&self, hkl: super::super::TextServices::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pdata: *const REGISTERWORDW) -> ::windows::core::Result<()>;
    fn CreateContext(&self) -> ::windows::core::Result<super::super::super::Globalization::HIMC>;
    fn DestroyContext(&self, hime: super::super::super::Globalization::HIMC) -> ::windows::core::Result<()>;
    fn EnumRegisterWordA(&self, hkl: super::super::TextServices::HKL, szreading: &::windows::core::PCSTR, dwstyle: u32, szregister: &::windows::core::PCSTR, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<IEnumRegisterWordA>;
    fn EnumRegisterWordW(&self, hkl: super::super::TextServices::HKL, szreading: &::windows::core::PCWSTR, dwstyle: u32, szregister: &::windows::core::PCWSTR, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<IEnumRegisterWordW>;
    fn EscapeA(&self, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::Result<()>;
    fn EscapeW(&self, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::Result<()>;
    fn GetCandidateListA(&self, himc: super::super::super::Globalization::HIMC, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::Result<()>;
    fn GetCandidateListW(&self, himc: super::super::super::Globalization::HIMC, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::Result<()>;
    fn GetCandidateListCountA(&self, himc: super::super::super::Globalization::HIMC, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> ::windows::core::Result<()>;
    fn GetCandidateListCountW(&self, himc: super::super::super::Globalization::HIMC, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> ::windows::core::Result<()>;
    fn GetCandidateWindow(&self, himc: super::super::super::Globalization::HIMC, dwindex: u32) -> ::windows::core::Result<CANDIDATEFORM>;
    fn GetCompositionFontA(&self, himc: super::super::super::Globalization::HIMC) -> ::windows::core::Result<super::super::super::Graphics::Gdi::LOGFONTA>;
    fn GetCompositionFontW(&self, himc: super::super::super::Globalization::HIMC) -> ::windows::core::Result<super::super::super::Graphics::Gdi::LOGFONTW>;
    fn GetCompositionStringA(&self, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetCompositionStringW(&self, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetCompositionWindow(&self, himc: super::super::super::Globalization::HIMC) -> ::windows::core::Result<COMPOSITIONFORM>;
    fn GetContext(&self, hwnd: super::super::super::Foundation::HWND) -> ::windows::core::Result<super::super::super::Globalization::HIMC>;
    fn GetConversionListA(&self, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, psrc: &::windows::core::PCSTR, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::Result<()>;
    fn GetConversionListW(&self, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, psrc: &::windows::core::PCWSTR, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::Result<()>;
    fn GetConversionStatus(&self, himc: super::super::super::Globalization::HIMC, pfdwconversion: *mut u32, pfdwsentence: *mut u32) -> ::windows::core::Result<()>;
    fn GetDefaultIMEWnd(&self, hwnd: super::super::super::Foundation::HWND) -> ::windows::core::Result<super::super::super::Foundation::HWND>;
    fn GetDescriptionA(&self, hkl: super::super::TextServices::HKL, ubuflen: u32, szdescription: ::windows::core::PSTR, pucopied: *mut u32) -> ::windows::core::Result<()>;
    fn GetDescriptionW(&self, hkl: super::super::TextServices::HKL, ubuflen: u32, szdescription: ::windows::core::PWSTR, pucopied: *mut u32) -> ::windows::core::Result<()>;
    fn GetGuideLineA(&self, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, pbuf: ::windows::core::PSTR, pdwresult: *mut u32) -> ::windows::core::Result<()>;
    fn GetGuideLineW(&self, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, pbuf: ::windows::core::PWSTR, pdwresult: *mut u32) -> ::windows::core::Result<()>;
    fn GetIMEFileNameA(&self, hkl: super::super::TextServices::HKL, ubuflen: u32, szfilename: ::windows::core::PSTR, pucopied: *mut u32) -> ::windows::core::Result<()>;
    fn GetIMEFileNameW(&self, hkl: super::super::TextServices::HKL, ubuflen: u32, szfilename: ::windows::core::PWSTR, pucopied: *mut u32) -> ::windows::core::Result<()>;
    fn GetOpenStatus(&self, himc: super::super::super::Globalization::HIMC) -> ::windows::core::Result<()>;
    fn GetProperty(&self, hkl: super::super::TextServices::HKL, fdwindex: u32) -> ::windows::core::Result<u32>;
    fn GetRegisterWordStyleA(&self, hkl: super::super::TextServices::HKL, nitem: u32, pstylebuf: *mut STYLEBUFA, pucopied: *mut u32) -> ::windows::core::Result<()>;
    fn GetRegisterWordStyleW(&self, hkl: super::super::TextServices::HKL, nitem: u32, pstylebuf: *mut STYLEBUFW, pucopied: *mut u32) -> ::windows::core::Result<()>;
    fn GetStatusWindowPos(&self, himc: super::super::super::Globalization::HIMC) -> ::windows::core::Result<super::super::super::Foundation::POINT>;
    fn GetVirtualKey(&self, hwnd: super::super::super::Foundation::HWND) -> ::windows::core::Result<u32>;
    fn InstallIMEA(&self, szimefilename: &::windows::core::PCSTR, szlayouttext: &::windows::core::PCSTR) -> ::windows::core::Result<super::super::TextServices::HKL>;
    fn InstallIMEW(&self, szimefilename: &::windows::core::PCWSTR, szlayouttext: &::windows::core::PCWSTR) -> ::windows::core::Result<super::super::TextServices::HKL>;
    fn IsIME(&self, hkl: super::super::TextServices::HKL) -> ::windows::core::Result<()>;
    fn IsUIMessageA(&self, hwndime: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows::core::Result<()>;
    fn IsUIMessageW(&self, hwndime: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows::core::Result<()>;
    fn NotifyIME(&self, himc: super::super::super::Globalization::HIMC, dwaction: u32, dwindex: u32, dwvalue: u32) -> ::windows::core::Result<()>;
    fn RegisterWordA(&self, hkl: super::super::TextServices::HKL, szreading: &::windows::core::PCSTR, dwstyle: u32, szregister: &::windows::core::PCSTR) -> ::windows::core::Result<()>;
    fn RegisterWordW(&self, hkl: super::super::TextServices::HKL, szreading: &::windows::core::PCWSTR, dwstyle: u32, szregister: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn ReleaseContext(&self, hwnd: super::super::super::Foundation::HWND, himc: super::super::super::Globalization::HIMC) -> ::windows::core::Result<()>;
    fn SetCandidateWindow(&self, himc: super::super::super::Globalization::HIMC, pcandidate: *const CANDIDATEFORM) -> ::windows::core::Result<()>;
    fn SetCompositionFontA(&self, himc: super::super::super::Globalization::HIMC, plf: *const super::super::super::Graphics::Gdi::LOGFONTA) -> ::windows::core::Result<()>;
    fn SetCompositionFontW(&self, himc: super::super::super::Globalization::HIMC, plf: *const super::super::super::Graphics::Gdi::LOGFONTW) -> ::windows::core::Result<()>;
    fn SetCompositionStringA(&self, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows::core::Result<()>;
    fn SetCompositionStringW(&self, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows::core::Result<()>;
    fn SetCompositionWindow(&self, himc: super::super::super::Globalization::HIMC, pcompform: *const COMPOSITIONFORM) -> ::windows::core::Result<()>;
    fn SetConversionStatus(&self, himc: super::super::super::Globalization::HIMC, fdwconversion: u32, fdwsentence: u32) -> ::windows::core::Result<()>;
    fn SetOpenStatus(&self, himc: super::super::super::Globalization::HIMC, fopen: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetStatusWindowPos(&self, himc: super::super::super::Globalization::HIMC, pptpos: *const super::super::super::Foundation::POINT) -> ::windows::core::Result<()>;
    fn SimulateHotKey(&self, hwnd: super::super::super::Foundation::HWND, dwhotkeyid: u32) -> ::windows::core::Result<()>;
    fn UnregisterWordA(&self, hkl: super::super::TextServices::HKL, szreading: &::windows::core::PCSTR, dwstyle: u32, szunregister: &::windows::core::PCSTR) -> ::windows::core::Result<()>;
    fn UnregisterWordW(&self, hkl: super::super::TextServices::HKL, szreading: &::windows::core::PCWSTR, dwstyle: u32, szunregister: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn Activate(&self, frestorelayout: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn Deactivate(&self) -> ::windows::core::Result<()>;
    fn OnDefWindowProc(&self, hwnd: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows::core::Result<super::super::super::Foundation::LRESULT>;
    fn FilterClientWindows(&self, aaclasslist: *const u16, usize: u32) -> ::windows::core::Result<()>;
    fn GetCodePageA(&self, hkl: super::super::TextServices::HKL) -> ::windows::core::Result<u32>;
    fn GetLangId(&self, hkl: super::super::TextServices::HKL) -> ::windows::core::Result<u16>;
    fn AssociateContextEx(&self, hwnd: super::super::super::Foundation::HWND, himc: super::super::super::Globalization::HIMC, dwflags: u32) -> ::windows::core::Result<()>;
    fn DisableIME(&self, idthread: u32) -> ::windows::core::Result<()>;
    fn GetImeMenuItemsA(&self, himc: super::super::super::Globalization::HIMC, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOA, pimemenu: *mut IMEMENUITEMINFOA, dwsize: u32, pdwresult: *mut u32) -> ::windows::core::Result<()>;
    fn GetImeMenuItemsW(&self, himc: super::super::super::Globalization::HIMC, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOW, pimemenu: *mut IMEMENUITEMINFOW, dwsize: u32, pdwresult: *mut u32) -> ::windows::core::Result<()>;
    fn EnumInputContext(&self, idthread: u32) -> ::windows::core::Result<IEnumInputContext>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_TextServices"))]
impl IActiveIMMApp_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>() -> IActiveIMMApp_Vtbl {
        unsafe extern "system" fn AssociateContext<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, hime: super::super::super::Globalization::HIMC, phprev: *mut super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AssociateContext(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&hime)) {
                ::core::result::Result::Ok(ok__) => {
                    *phprev = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConfigureIMEA<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pdata: *const REGISTERWORDA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ConfigureIMEA(::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&dwmode), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn ConfigureIMEW<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pdata: *const REGISTERWORDW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ConfigureIMEW(::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&dwmode), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn CreateContext<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phimc: *mut super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateContext() {
                ::core::result::Result::Ok(ok__) => {
                    *phimc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestroyContext<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hime: super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DestroyContext(::core::mem::transmute_copy(&hime)).into()
        }
        unsafe extern "system" fn EnumRegisterWordA<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: ::windows::core::PCSTR, dwstyle: u32, szregister: ::windows::core::PCSTR, pdata: *const ::core::ffi::c_void, penum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumRegisterWordA(::core::mem::transmute_copy(&hkl), ::core::mem::transmute(&szreading), ::core::mem::transmute_copy(&dwstyle), ::core::mem::transmute(&szregister), ::core::mem::transmute_copy(&pdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *penum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumRegisterWordW<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: ::windows::core::PCWSTR, dwstyle: u32, szregister: ::windows::core::PCWSTR, pdata: *const ::core::ffi::c_void, penum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumRegisterWordW(::core::mem::transmute_copy(&hkl), ::core::mem::transmute(&szreading), ::core::mem::transmute_copy(&dwstyle), ::core::mem::transmute(&szregister), ::core::mem::transmute_copy(&pdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *penum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EscapeA<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EscapeA(::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&uescape), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&plresult)).into()
        }
        unsafe extern "system" fn EscapeW<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EscapeW(::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&uescape), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&plresult)).into()
        }
        unsafe extern "system" fn GetCandidateListA<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCandidateListA(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&ubuflen), ::core::mem::transmute_copy(&pcandlist), ::core::mem::transmute_copy(&pucopied)).into()
        }
        unsafe extern "system" fn GetCandidateListW<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCandidateListW(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&ubuflen), ::core::mem::transmute_copy(&pcandlist), ::core::mem::transmute_copy(&pucopied)).into()
        }
        unsafe extern "system" fn GetCandidateListCountA<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCandidateListCountA(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&pdwlistsize), ::core::mem::transmute_copy(&pdwbuflen)).into()
        }
        unsafe extern "system" fn GetCandidateListCountW<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCandidateListCountW(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&pdwlistsize), ::core::mem::transmute_copy(&pdwbuflen)).into()
        }
        unsafe extern "system" fn GetCandidateWindow<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcandidate: *mut CANDIDATEFORM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCandidateWindow(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcandidate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCompositionFontA<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *mut super::super::super::Graphics::Gdi::LOGFONTA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCompositionFontA(::core::mem::transmute_copy(&himc)) {
                ::core::result::Result::Ok(ok__) => {
                    *plf = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCompositionFontW<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *mut super::super::super::Graphics::Gdi::LOGFONTW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCompositionFontW(::core::mem::transmute_copy(&himc)) {
                ::core::result::Result::Ok(ok__) => {
                    *plf = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCompositionStringA<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCompositionStringA(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&dwbuflen), ::core::mem::transmute_copy(&plcopied), ::core::mem::transmute_copy(&pbuf)).into()
        }
        unsafe extern "system" fn GetCompositionStringW<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCompositionStringW(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&dwbuflen), ::core::mem::transmute_copy(&plcopied), ::core::mem::transmute_copy(&pbuf)).into()
        }
        unsafe extern "system" fn GetCompositionWindow<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pcompform: *mut COMPOSITIONFORM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCompositionWindow(::core::mem::transmute_copy(&himc)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcompform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContext<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, phimc: *mut super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetContext(::core::mem::transmute_copy(&hwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    *phimc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConversionListA<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, psrc: ::windows::core::PCSTR, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetConversionListA(::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&himc), ::core::mem::transmute(&psrc), ::core::mem::transmute_copy(&ubuflen), ::core::mem::transmute_copy(&uflag), ::core::mem::transmute_copy(&pdst), ::core::mem::transmute_copy(&pucopied)).into()
        }
        unsafe extern "system" fn GetConversionListW<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, psrc: ::windows::core::PCWSTR, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetConversionListW(::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&himc), ::core::mem::transmute(&psrc), ::core::mem::transmute_copy(&ubuflen), ::core::mem::transmute_copy(&uflag), ::core::mem::transmute_copy(&pdst), ::core::mem::transmute_copy(&pucopied)).into()
        }
        unsafe extern "system" fn GetConversionStatus<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pfdwconversion: *mut u32, pfdwsentence: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetConversionStatus(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&pfdwconversion), ::core::mem::transmute_copy(&pfdwsentence)).into()
        }
        unsafe extern "system" fn GetDefaultIMEWnd<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, phdefwnd: *mut super::super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDefaultIMEWnd(::core::mem::transmute_copy(&hwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    *phdefwnd = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescriptionA<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szdescription: ::windows::core::PSTR, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDescriptionA(::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&ubuflen), ::core::mem::transmute_copy(&szdescription), ::core::mem::transmute_copy(&pucopied)).into()
        }
        unsafe extern "system" fn GetDescriptionW<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szdescription: ::windows::core::PWSTR, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDescriptionW(::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&ubuflen), ::core::mem::transmute_copy(&szdescription), ::core::mem::transmute_copy(&pucopied)).into()
        }
        unsafe extern "system" fn GetGuideLineA<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, pbuf: ::windows::core::PSTR, pdwresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetGuideLineA(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&dwbuflen), ::core::mem::transmute_copy(&pbuf), ::core::mem::transmute_copy(&pdwresult)).into()
        }
        unsafe extern "system" fn GetGuideLineW<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, pbuf: ::windows::core::PWSTR, pdwresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetGuideLineW(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&dwbuflen), ::core::mem::transmute_copy(&pbuf), ::core::mem::transmute_copy(&pdwresult)).into()
        }
        unsafe extern "system" fn GetIMEFileNameA<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szfilename: ::windows::core::PSTR, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetIMEFileNameA(::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&ubuflen), ::core::mem::transmute_copy(&szfilename), ::core::mem::transmute_copy(&pucopied)).into()
        }
        unsafe extern "system" fn GetIMEFileNameW<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szfilename: ::windows::core::PWSTR, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetIMEFileNameW(::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&ubuflen), ::core::mem::transmute_copy(&szfilename), ::core::mem::transmute_copy(&pucopied)).into()
        }
        unsafe extern "system" fn GetOpenStatus<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetOpenStatus(::core::mem::transmute_copy(&himc)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, fdwindex: u32, pdwproperty: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&fdwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwproperty = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRegisterWordStyleA<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, nitem: u32, pstylebuf: *mut STYLEBUFA, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRegisterWordStyleA(::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&nitem), ::core::mem::transmute_copy(&pstylebuf), ::core::mem::transmute_copy(&pucopied)).into()
        }
        unsafe extern "system" fn GetRegisterWordStyleW<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, nitem: u32, pstylebuf: *mut STYLEBUFW, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRegisterWordStyleW(::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&nitem), ::core::mem::transmute_copy(&pstylebuf), ::core::mem::transmute_copy(&pucopied)).into()
        }
        unsafe extern "system" fn GetStatusWindowPos<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pptpos: *mut super::super::super::Foundation::POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStatusWindowPos(::core::mem::transmute_copy(&himc)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptpos = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVirtualKey<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, puvirtualkey: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetVirtualKey(::core::mem::transmute_copy(&hwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    *puvirtualkey = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallIMEA<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szimefilename: ::windows::core::PCSTR, szlayouttext: ::windows::core::PCSTR, phkl: *mut super::super::TextServices::HKL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InstallIMEA(::core::mem::transmute(&szimefilename), ::core::mem::transmute(&szlayouttext)) {
                ::core::result::Result::Ok(ok__) => {
                    *phkl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallIMEW<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szimefilename: ::windows::core::PCWSTR, szlayouttext: ::windows::core::PCWSTR, phkl: *mut super::super::TextServices::HKL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InstallIMEW(::core::mem::transmute(&szimefilename), ::core::mem::transmute(&szlayouttext)) {
                ::core::result::Result::Ok(ok__) => {
                    *phkl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsIME<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsIME(::core::mem::transmute_copy(&hkl)).into()
        }
        unsafe extern "system" fn IsUIMessageA<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndime: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsUIMessageA(::core::mem::transmute_copy(&hwndime), ::core::mem::transmute_copy(&msg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn IsUIMessageW<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndime: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsUIMessageW(::core::mem::transmute_copy(&hwndime), ::core::mem::transmute_copy(&msg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn NotifyIME<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwaction: u32, dwindex: u32, dwvalue: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NotifyIME(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwaction), ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&dwvalue)).into()
        }
        unsafe extern "system" fn RegisterWordA<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: ::windows::core::PCSTR, dwstyle: u32, szregister: ::windows::core::PCSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RegisterWordA(::core::mem::transmute_copy(&hkl), ::core::mem::transmute(&szreading), ::core::mem::transmute_copy(&dwstyle), ::core::mem::transmute(&szregister)).into()
        }
        unsafe extern "system" fn RegisterWordW<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: ::windows::core::PCWSTR, dwstyle: u32, szregister: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RegisterWordW(::core::mem::transmute_copy(&hkl), ::core::mem::transmute(&szreading), ::core::mem::transmute_copy(&dwstyle), ::core::mem::transmute(&szregister)).into()
        }
        unsafe extern "system" fn ReleaseContext<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, himc: super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReleaseContext(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&himc)).into()
        }
        unsafe extern "system" fn SetCandidateWindow<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pcandidate: *const CANDIDATEFORM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCandidateWindow(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&pcandidate)).into()
        }
        unsafe extern "system" fn SetCompositionFontA<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *const super::super::super::Graphics::Gdi::LOGFONTA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCompositionFontA(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&plf)).into()
        }
        unsafe extern "system" fn SetCompositionFontW<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *const super::super::super::Graphics::Gdi::LOGFONTW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCompositionFontW(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&plf)).into()
        }
        unsafe extern "system" fn SetCompositionStringA<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCompositionStringA(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pcomp), ::core::mem::transmute_copy(&dwcomplen), ::core::mem::transmute_copy(&pread), ::core::mem::transmute_copy(&dwreadlen)).into()
        }
        unsafe extern "system" fn SetCompositionStringW<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCompositionStringW(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pcomp), ::core::mem::transmute_copy(&dwcomplen), ::core::mem::transmute_copy(&pread), ::core::mem::transmute_copy(&dwreadlen)).into()
        }
        unsafe extern "system" fn SetCompositionWindow<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pcompform: *const COMPOSITIONFORM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCompositionWindow(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&pcompform)).into()
        }
        unsafe extern "system" fn SetConversionStatus<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, fdwconversion: u32, fdwsentence: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetConversionStatus(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&fdwconversion), ::core::mem::transmute_copy(&fdwsentence)).into()
        }
        unsafe extern "system" fn SetOpenStatus<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, fopen: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOpenStatus(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&fopen)).into()
        }
        unsafe extern "system" fn SetStatusWindowPos<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pptpos: *const super::super::super::Foundation::POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStatusWindowPos(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&pptpos)).into()
        }
        unsafe extern "system" fn SimulateHotKey<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, dwhotkeyid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SimulateHotKey(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&dwhotkeyid)).into()
        }
        unsafe extern "system" fn UnregisterWordA<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: ::windows::core::PCSTR, dwstyle: u32, szunregister: ::windows::core::PCSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnregisterWordA(::core::mem::transmute_copy(&hkl), ::core::mem::transmute(&szreading), ::core::mem::transmute_copy(&dwstyle), ::core::mem::transmute(&szunregister)).into()
        }
        unsafe extern "system" fn UnregisterWordW<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: ::windows::core::PCWSTR, dwstyle: u32, szunregister: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnregisterWordW(::core::mem::transmute_copy(&hkl), ::core::mem::transmute(&szreading), ::core::mem::transmute_copy(&dwstyle), ::core::mem::transmute(&szunregister)).into()
        }
        unsafe extern "system" fn Activate<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frestorelayout: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Activate(::core::mem::transmute_copy(&frestorelayout)).into()
        }
        unsafe extern "system" fn Deactivate<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Deactivate().into()
        }
        unsafe extern "system" fn OnDefWindowProc<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OnDefWindowProc(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&msg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    *plresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FilterClientWindows<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aaclasslist: *const u16, usize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FilterClientWindows(::core::mem::transmute_copy(&aaclasslist), ::core::mem::transmute_copy(&usize)).into()
        }
        unsafe extern "system" fn GetCodePageA<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ucodepage: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCodePageA(::core::mem::transmute_copy(&hkl)) {
                ::core::result::Result::Ok(ok__) => {
                    *ucodepage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLangId<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, plid: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLangId(::core::mem::transmute_copy(&hkl)) {
                ::core::result::Result::Ok(ok__) => {
                    *plid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AssociateContextEx<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, himc: super::super::super::Globalization::HIMC, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AssociateContextEx(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn DisableIME<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idthread: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DisableIME(::core::mem::transmute_copy(&idthread)).into()
        }
        unsafe extern "system" fn GetImeMenuItemsA<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOA, pimemenu: *mut IMEMENUITEMINFOA, dwsize: u32, pdwresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetImeMenuItemsA(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwtype), ::core::mem::transmute_copy(&pimeparentmenu), ::core::mem::transmute_copy(&pimemenu), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&pdwresult)).into()
        }
        unsafe extern "system" fn GetImeMenuItemsW<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOW, pimemenu: *mut IMEMENUITEMINFOW, dwsize: u32, pdwresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetImeMenuItemsW(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwtype), ::core::mem::transmute_copy(&pimeparentmenu), ::core::mem::transmute_copy(&pimemenu), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&pdwresult)).into()
        }
        unsafe extern "system" fn EnumInputContext<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idthread: u32, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumInputContext(::core::mem::transmute_copy(&idthread)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AssociateContext: AssociateContext::<Identity, Impl, OFFSET>,
            ConfigureIMEA: ConfigureIMEA::<Identity, Impl, OFFSET>,
            ConfigureIMEW: ConfigureIMEW::<Identity, Impl, OFFSET>,
            CreateContext: CreateContext::<Identity, Impl, OFFSET>,
            DestroyContext: DestroyContext::<Identity, Impl, OFFSET>,
            EnumRegisterWordA: EnumRegisterWordA::<Identity, Impl, OFFSET>,
            EnumRegisterWordW: EnumRegisterWordW::<Identity, Impl, OFFSET>,
            EscapeA: EscapeA::<Identity, Impl, OFFSET>,
            EscapeW: EscapeW::<Identity, Impl, OFFSET>,
            GetCandidateListA: GetCandidateListA::<Identity, Impl, OFFSET>,
            GetCandidateListW: GetCandidateListW::<Identity, Impl, OFFSET>,
            GetCandidateListCountA: GetCandidateListCountA::<Identity, Impl, OFFSET>,
            GetCandidateListCountW: GetCandidateListCountW::<Identity, Impl, OFFSET>,
            GetCandidateWindow: GetCandidateWindow::<Identity, Impl, OFFSET>,
            GetCompositionFontA: GetCompositionFontA::<Identity, Impl, OFFSET>,
            GetCompositionFontW: GetCompositionFontW::<Identity, Impl, OFFSET>,
            GetCompositionStringA: GetCompositionStringA::<Identity, Impl, OFFSET>,
            GetCompositionStringW: GetCompositionStringW::<Identity, Impl, OFFSET>,
            GetCompositionWindow: GetCompositionWindow::<Identity, Impl, OFFSET>,
            GetContext: GetContext::<Identity, Impl, OFFSET>,
            GetConversionListA: GetConversionListA::<Identity, Impl, OFFSET>,
            GetConversionListW: GetConversionListW::<Identity, Impl, OFFSET>,
            GetConversionStatus: GetConversionStatus::<Identity, Impl, OFFSET>,
            GetDefaultIMEWnd: GetDefaultIMEWnd::<Identity, Impl, OFFSET>,
            GetDescriptionA: GetDescriptionA::<Identity, Impl, OFFSET>,
            GetDescriptionW: GetDescriptionW::<Identity, Impl, OFFSET>,
            GetGuideLineA: GetGuideLineA::<Identity, Impl, OFFSET>,
            GetGuideLineW: GetGuideLineW::<Identity, Impl, OFFSET>,
            GetIMEFileNameA: GetIMEFileNameA::<Identity, Impl, OFFSET>,
            GetIMEFileNameW: GetIMEFileNameW::<Identity, Impl, OFFSET>,
            GetOpenStatus: GetOpenStatus::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            GetRegisterWordStyleA: GetRegisterWordStyleA::<Identity, Impl, OFFSET>,
            GetRegisterWordStyleW: GetRegisterWordStyleW::<Identity, Impl, OFFSET>,
            GetStatusWindowPos: GetStatusWindowPos::<Identity, Impl, OFFSET>,
            GetVirtualKey: GetVirtualKey::<Identity, Impl, OFFSET>,
            InstallIMEA: InstallIMEA::<Identity, Impl, OFFSET>,
            InstallIMEW: InstallIMEW::<Identity, Impl, OFFSET>,
            IsIME: IsIME::<Identity, Impl, OFFSET>,
            IsUIMessageA: IsUIMessageA::<Identity, Impl, OFFSET>,
            IsUIMessageW: IsUIMessageW::<Identity, Impl, OFFSET>,
            NotifyIME: NotifyIME::<Identity, Impl, OFFSET>,
            RegisterWordA: RegisterWordA::<Identity, Impl, OFFSET>,
            RegisterWordW: RegisterWordW::<Identity, Impl, OFFSET>,
            ReleaseContext: ReleaseContext::<Identity, Impl, OFFSET>,
            SetCandidateWindow: SetCandidateWindow::<Identity, Impl, OFFSET>,
            SetCompositionFontA: SetCompositionFontA::<Identity, Impl, OFFSET>,
            SetCompositionFontW: SetCompositionFontW::<Identity, Impl, OFFSET>,
            SetCompositionStringA: SetCompositionStringA::<Identity, Impl, OFFSET>,
            SetCompositionStringW: SetCompositionStringW::<Identity, Impl, OFFSET>,
            SetCompositionWindow: SetCompositionWindow::<Identity, Impl, OFFSET>,
            SetConversionStatus: SetConversionStatus::<Identity, Impl, OFFSET>,
            SetOpenStatus: SetOpenStatus::<Identity, Impl, OFFSET>,
            SetStatusWindowPos: SetStatusWindowPos::<Identity, Impl, OFFSET>,
            SimulateHotKey: SimulateHotKey::<Identity, Impl, OFFSET>,
            UnregisterWordA: UnregisterWordA::<Identity, Impl, OFFSET>,
            UnregisterWordW: UnregisterWordW::<Identity, Impl, OFFSET>,
            Activate: Activate::<Identity, Impl, OFFSET>,
            Deactivate: Deactivate::<Identity, Impl, OFFSET>,
            OnDefWindowProc: OnDefWindowProc::<Identity, Impl, OFFSET>,
            FilterClientWindows: FilterClientWindows::<Identity, Impl, OFFSET>,
            GetCodePageA: GetCodePageA::<Identity, Impl, OFFSET>,
            GetLangId: GetLangId::<Identity, Impl, OFFSET>,
            AssociateContextEx: AssociateContextEx::<Identity, Impl, OFFSET>,
            DisableIME: DisableIME::<Identity, Impl, OFFSET>,
            GetImeMenuItemsA: GetImeMenuItemsA::<Identity, Impl, OFFSET>,
            GetImeMenuItemsW: GetImeMenuItemsW::<Identity, Impl, OFFSET>,
            EnumInputContext: EnumInputContext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActiveIMMApp as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_TextServices"))]
pub trait IActiveIMMIME_Impl: Sized {
    fn AssociateContext(&self, hwnd: super::super::super::Foundation::HWND, hime: super::super::super::Globalization::HIMC) -> ::windows::core::Result<super::super::super::Globalization::HIMC>;
    fn ConfigureIMEA(&self, hkl: super::super::TextServices::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pdata: *const REGISTERWORDA) -> ::windows::core::Result<()>;
    fn ConfigureIMEW(&self, hkl: super::super::TextServices::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pdata: *const REGISTERWORDW) -> ::windows::core::Result<()>;
    fn CreateContext(&self) -> ::windows::core::Result<super::super::super::Globalization::HIMC>;
    fn DestroyContext(&self, hime: super::super::super::Globalization::HIMC) -> ::windows::core::Result<()>;
    fn EnumRegisterWordA(&self, hkl: super::super::TextServices::HKL, szreading: &::windows::core::PCSTR, dwstyle: u32, szregister: &::windows::core::PCSTR, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<IEnumRegisterWordA>;
    fn EnumRegisterWordW(&self, hkl: super::super::TextServices::HKL, szreading: &::windows::core::PCWSTR, dwstyle: u32, szregister: &::windows::core::PCWSTR, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<IEnumRegisterWordW>;
    fn EscapeA(&self, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::Result<()>;
    fn EscapeW(&self, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::Result<()>;
    fn GetCandidateListA(&self, himc: super::super::super::Globalization::HIMC, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::Result<()>;
    fn GetCandidateListW(&self, himc: super::super::super::Globalization::HIMC, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::Result<()>;
    fn GetCandidateListCountA(&self, himc: super::super::super::Globalization::HIMC, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> ::windows::core::Result<()>;
    fn GetCandidateListCountW(&self, himc: super::super::super::Globalization::HIMC, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> ::windows::core::Result<()>;
    fn GetCandidateWindow(&self, himc: super::super::super::Globalization::HIMC, dwindex: u32) -> ::windows::core::Result<CANDIDATEFORM>;
    fn GetCompositionFontA(&self, himc: super::super::super::Globalization::HIMC) -> ::windows::core::Result<super::super::super::Graphics::Gdi::LOGFONTA>;
    fn GetCompositionFontW(&self, himc: super::super::super::Globalization::HIMC) -> ::windows::core::Result<super::super::super::Graphics::Gdi::LOGFONTW>;
    fn GetCompositionStringA(&self, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetCompositionStringW(&self, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetCompositionWindow(&self, himc: super::super::super::Globalization::HIMC) -> ::windows::core::Result<COMPOSITIONFORM>;
    fn GetContext(&self, hwnd: super::super::super::Foundation::HWND) -> ::windows::core::Result<super::super::super::Globalization::HIMC>;
    fn GetConversionListA(&self, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, psrc: &::windows::core::PCSTR, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::Result<()>;
    fn GetConversionListW(&self, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, psrc: &::windows::core::PCWSTR, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::Result<()>;
    fn GetConversionStatus(&self, himc: super::super::super::Globalization::HIMC, pfdwconversion: *mut u32, pfdwsentence: *mut u32) -> ::windows::core::Result<()>;
    fn GetDefaultIMEWnd(&self, hwnd: super::super::super::Foundation::HWND) -> ::windows::core::Result<super::super::super::Foundation::HWND>;
    fn GetDescriptionA(&self, hkl: super::super::TextServices::HKL, ubuflen: u32, szdescription: ::windows::core::PSTR, pucopied: *mut u32) -> ::windows::core::Result<()>;
    fn GetDescriptionW(&self, hkl: super::super::TextServices::HKL, ubuflen: u32, szdescription: ::windows::core::PWSTR, pucopied: *mut u32) -> ::windows::core::Result<()>;
    fn GetGuideLineA(&self, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, pbuf: ::windows::core::PSTR, pdwresult: *mut u32) -> ::windows::core::Result<()>;
    fn GetGuideLineW(&self, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, pbuf: ::windows::core::PWSTR, pdwresult: *mut u32) -> ::windows::core::Result<()>;
    fn GetIMEFileNameA(&self, hkl: super::super::TextServices::HKL, ubuflen: u32, szfilename: ::windows::core::PSTR, pucopied: *mut u32) -> ::windows::core::Result<()>;
    fn GetIMEFileNameW(&self, hkl: super::super::TextServices::HKL, ubuflen: u32, szfilename: ::windows::core::PWSTR, pucopied: *mut u32) -> ::windows::core::Result<()>;
    fn GetOpenStatus(&self, himc: super::super::super::Globalization::HIMC) -> ::windows::core::Result<()>;
    fn GetProperty(&self, hkl: super::super::TextServices::HKL, fdwindex: u32) -> ::windows::core::Result<u32>;
    fn GetRegisterWordStyleA(&self, hkl: super::super::TextServices::HKL, nitem: u32, pstylebuf: *mut STYLEBUFA, pucopied: *mut u32) -> ::windows::core::Result<()>;
    fn GetRegisterWordStyleW(&self, hkl: super::super::TextServices::HKL, nitem: u32, pstylebuf: *mut STYLEBUFW, pucopied: *mut u32) -> ::windows::core::Result<()>;
    fn GetStatusWindowPos(&self, himc: super::super::super::Globalization::HIMC) -> ::windows::core::Result<super::super::super::Foundation::POINT>;
    fn GetVirtualKey(&self, hwnd: super::super::super::Foundation::HWND) -> ::windows::core::Result<u32>;
    fn InstallIMEA(&self, szimefilename: &::windows::core::PCSTR, szlayouttext: &::windows::core::PCSTR) -> ::windows::core::Result<super::super::TextServices::HKL>;
    fn InstallIMEW(&self, szimefilename: &::windows::core::PCWSTR, szlayouttext: &::windows::core::PCWSTR) -> ::windows::core::Result<super::super::TextServices::HKL>;
    fn IsIME(&self, hkl: super::super::TextServices::HKL) -> ::windows::core::Result<()>;
    fn IsUIMessageA(&self, hwndime: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows::core::Result<()>;
    fn IsUIMessageW(&self, hwndime: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows::core::Result<()>;
    fn NotifyIME(&self, himc: super::super::super::Globalization::HIMC, dwaction: u32, dwindex: u32, dwvalue: u32) -> ::windows::core::Result<()>;
    fn RegisterWordA(&self, hkl: super::super::TextServices::HKL, szreading: &::windows::core::PCSTR, dwstyle: u32, szregister: &::windows::core::PCSTR) -> ::windows::core::Result<()>;
    fn RegisterWordW(&self, hkl: super::super::TextServices::HKL, szreading: &::windows::core::PCWSTR, dwstyle: u32, szregister: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn ReleaseContext(&self, hwnd: super::super::super::Foundation::HWND, himc: super::super::super::Globalization::HIMC) -> ::windows::core::Result<()>;
    fn SetCandidateWindow(&self, himc: super::super::super::Globalization::HIMC, pcandidate: *const CANDIDATEFORM) -> ::windows::core::Result<()>;
    fn SetCompositionFontA(&self, himc: super::super::super::Globalization::HIMC, plf: *const super::super::super::Graphics::Gdi::LOGFONTA) -> ::windows::core::Result<()>;
    fn SetCompositionFontW(&self, himc: super::super::super::Globalization::HIMC, plf: *const super::super::super::Graphics::Gdi::LOGFONTW) -> ::windows::core::Result<()>;
    fn SetCompositionStringA(&self, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows::core::Result<()>;
    fn SetCompositionStringW(&self, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows::core::Result<()>;
    fn SetCompositionWindow(&self, himc: super::super::super::Globalization::HIMC, pcompform: *const COMPOSITIONFORM) -> ::windows::core::Result<()>;
    fn SetConversionStatus(&self, himc: super::super::super::Globalization::HIMC, fdwconversion: u32, fdwsentence: u32) -> ::windows::core::Result<()>;
    fn SetOpenStatus(&self, himc: super::super::super::Globalization::HIMC, fopen: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetStatusWindowPos(&self, himc: super::super::super::Globalization::HIMC, pptpos: *const super::super::super::Foundation::POINT) -> ::windows::core::Result<()>;
    fn SimulateHotKey(&self, hwnd: super::super::super::Foundation::HWND, dwhotkeyid: u32) -> ::windows::core::Result<()>;
    fn UnregisterWordA(&self, hkl: super::super::TextServices::HKL, szreading: &::windows::core::PCSTR, dwstyle: u32, szunregister: &::windows::core::PCSTR) -> ::windows::core::Result<()>;
    fn UnregisterWordW(&self, hkl: super::super::TextServices::HKL, szreading: &::windows::core::PCWSTR, dwstyle: u32, szunregister: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GenerateMessage(&self, himc: super::super::super::Globalization::HIMC) -> ::windows::core::Result<()>;
    fn LockIMC(&self, himc: super::super::super::Globalization::HIMC) -> ::windows::core::Result<*mut INPUTCONTEXT>;
    fn UnlockIMC(&self, himc: super::super::super::Globalization::HIMC) -> ::windows::core::Result<()>;
    fn GetIMCLockCount(&self, himc: super::super::super::Globalization::HIMC) -> ::windows::core::Result<u32>;
    fn CreateIMCC(&self, dwsize: u32) -> ::windows::core::Result<super::super::super::Globalization::HIMCC>;
    fn DestroyIMCC(&self, himcc: super::super::super::Globalization::HIMCC) -> ::windows::core::Result<()>;
    fn LockIMCC(&self, himcc: super::super::super::Globalization::HIMCC, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn UnlockIMCC(&self, himcc: super::super::super::Globalization::HIMCC) -> ::windows::core::Result<()>;
    fn ReSizeIMCC(&self, himcc: super::super::super::Globalization::HIMCC, dwsize: u32) -> ::windows::core::Result<super::super::super::Globalization::HIMCC>;
    fn GetIMCCSize(&self, himcc: super::super::super::Globalization::HIMCC) -> ::windows::core::Result<u32>;
    fn GetIMCCLockCount(&self, himcc: super::super::super::Globalization::HIMCC) -> ::windows::core::Result<u32>;
    fn GetHotKey(&self, dwhotkeyid: u32, pumodifiers: *mut u32, puvkey: *mut u32, phkl: *mut super::super::TextServices::HKL) -> ::windows::core::Result<()>;
    fn SetHotKey(&self, dwhotkeyid: u32, umodifiers: u32, uvkey: u32, hkl: super::super::TextServices::HKL) -> ::windows::core::Result<()>;
    fn CreateSoftKeyboard(&self, utype: u32, howner: super::super::super::Foundation::HWND, x: i32, y: i32) -> ::windows::core::Result<super::super::super::Foundation::HWND>;
    fn DestroySoftKeyboard(&self, hsoftkbdwnd: super::super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn ShowSoftKeyboard(&self, hsoftkbdwnd: super::super::super::Foundation::HWND, ncmdshow: i32) -> ::windows::core::Result<()>;
    fn GetCodePageA(&self, hkl: super::super::TextServices::HKL) -> ::windows::core::Result<u32>;
    fn GetLangId(&self, hkl: super::super::TextServices::HKL) -> ::windows::core::Result<u16>;
    fn KeybdEvent(&self, lgidime: u16, bvk: u8, bscan: u8, dwflags: u32, dwextrainfo: u32) -> ::windows::core::Result<()>;
    fn LockModal(&self) -> ::windows::core::Result<()>;
    fn UnlockModal(&self) -> ::windows::core::Result<()>;
    fn AssociateContextEx(&self, hwnd: super::super::super::Foundation::HWND, himc: super::super::super::Globalization::HIMC, dwflags: u32) -> ::windows::core::Result<()>;
    fn DisableIME(&self, idthread: u32) -> ::windows::core::Result<()>;
    fn GetImeMenuItemsA(&self, himc: super::super::super::Globalization::HIMC, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOA, pimemenu: *mut IMEMENUITEMINFOA, dwsize: u32, pdwresult: *mut u32) -> ::windows::core::Result<()>;
    fn GetImeMenuItemsW(&self, himc: super::super::super::Globalization::HIMC, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOW, pimemenu: *mut IMEMENUITEMINFOW, dwsize: u32, pdwresult: *mut u32) -> ::windows::core::Result<()>;
    fn EnumInputContext(&self, idthread: u32) -> ::windows::core::Result<IEnumInputContext>;
    fn RequestMessageA(&self, himc: super::super::super::Globalization::HIMC, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows::core::Result<super::super::super::Foundation::LRESULT>;
    fn RequestMessageW(&self, himc: super::super::super::Globalization::HIMC, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows::core::Result<super::super::super::Foundation::LRESULT>;
    fn SendIMCA(&self, hwnd: super::super::super::Foundation::HWND, umsg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows::core::Result<super::super::super::Foundation::LRESULT>;
    fn SendIMCW(&self, hwnd: super::super::super::Foundation::HWND, umsg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows::core::Result<super::super::super::Foundation::LRESULT>;
    fn IsSleeping(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_TextServices"))]
impl IActiveIMMIME_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>() -> IActiveIMMIME_Vtbl {
        unsafe extern "system" fn AssociateContext<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, hime: super::super::super::Globalization::HIMC, phprev: *mut super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AssociateContext(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&hime)) {
                ::core::result::Result::Ok(ok__) => {
                    *phprev = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConfigureIMEA<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pdata: *const REGISTERWORDA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ConfigureIMEA(::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&dwmode), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn ConfigureIMEW<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pdata: *const REGISTERWORDW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ConfigureIMEW(::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&dwmode), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn CreateContext<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phimc: *mut super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateContext() {
                ::core::result::Result::Ok(ok__) => {
                    *phimc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestroyContext<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hime: super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DestroyContext(::core::mem::transmute_copy(&hime)).into()
        }
        unsafe extern "system" fn EnumRegisterWordA<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: ::windows::core::PCSTR, dwstyle: u32, szregister: ::windows::core::PCSTR, pdata: *const ::core::ffi::c_void, penum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumRegisterWordA(::core::mem::transmute_copy(&hkl), ::core::mem::transmute(&szreading), ::core::mem::transmute_copy(&dwstyle), ::core::mem::transmute(&szregister), ::core::mem::transmute_copy(&pdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *penum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumRegisterWordW<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: ::windows::core::PCWSTR, dwstyle: u32, szregister: ::windows::core::PCWSTR, pdata: *const ::core::ffi::c_void, penum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumRegisterWordW(::core::mem::transmute_copy(&hkl), ::core::mem::transmute(&szreading), ::core::mem::transmute_copy(&dwstyle), ::core::mem::transmute(&szregister), ::core::mem::transmute_copy(&pdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *penum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EscapeA<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EscapeA(::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&uescape), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&plresult)).into()
        }
        unsafe extern "system" fn EscapeW<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EscapeW(::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&uescape), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&plresult)).into()
        }
        unsafe extern "system" fn GetCandidateListA<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCandidateListA(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&ubuflen), ::core::mem::transmute_copy(&pcandlist), ::core::mem::transmute_copy(&pucopied)).into()
        }
        unsafe extern "system" fn GetCandidateListW<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCandidateListW(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&ubuflen), ::core::mem::transmute_copy(&pcandlist), ::core::mem::transmute_copy(&pucopied)).into()
        }
        unsafe extern "system" fn GetCandidateListCountA<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCandidateListCountA(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&pdwlistsize), ::core::mem::transmute_copy(&pdwbuflen)).into()
        }
        unsafe extern "system" fn GetCandidateListCountW<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCandidateListCountW(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&pdwlistsize), ::core::mem::transmute_copy(&pdwbuflen)).into()
        }
        unsafe extern "system" fn GetCandidateWindow<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcandidate: *mut CANDIDATEFORM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCandidateWindow(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcandidate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCompositionFontA<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *mut super::super::super::Graphics::Gdi::LOGFONTA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCompositionFontA(::core::mem::transmute_copy(&himc)) {
                ::core::result::Result::Ok(ok__) => {
                    *plf = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCompositionFontW<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *mut super::super::super::Graphics::Gdi::LOGFONTW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCompositionFontW(::core::mem::transmute_copy(&himc)) {
                ::core::result::Result::Ok(ok__) => {
                    *plf = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCompositionStringA<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCompositionStringA(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&dwbuflen), ::core::mem::transmute_copy(&plcopied), ::core::mem::transmute_copy(&pbuf)).into()
        }
        unsafe extern "system" fn GetCompositionStringW<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCompositionStringW(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&dwbuflen), ::core::mem::transmute_copy(&plcopied), ::core::mem::transmute_copy(&pbuf)).into()
        }
        unsafe extern "system" fn GetCompositionWindow<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pcompform: *mut COMPOSITIONFORM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCompositionWindow(::core::mem::transmute_copy(&himc)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcompform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContext<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, phimc: *mut super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetContext(::core::mem::transmute_copy(&hwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    *phimc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConversionListA<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, psrc: ::windows::core::PCSTR, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetConversionListA(::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&himc), ::core::mem::transmute(&psrc), ::core::mem::transmute_copy(&ubuflen), ::core::mem::transmute_copy(&uflag), ::core::mem::transmute_copy(&pdst), ::core::mem::transmute_copy(&pucopied)).into()
        }
        unsafe extern "system" fn GetConversionListW<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, psrc: ::windows::core::PCWSTR, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetConversionListW(::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&himc), ::core::mem::transmute(&psrc), ::core::mem::transmute_copy(&ubuflen), ::core::mem::transmute_copy(&uflag), ::core::mem::transmute_copy(&pdst), ::core::mem::transmute_copy(&pucopied)).into()
        }
        unsafe extern "system" fn GetConversionStatus<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pfdwconversion: *mut u32, pfdwsentence: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetConversionStatus(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&pfdwconversion), ::core::mem::transmute_copy(&pfdwsentence)).into()
        }
        unsafe extern "system" fn GetDefaultIMEWnd<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, phdefwnd: *mut super::super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDefaultIMEWnd(::core::mem::transmute_copy(&hwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    *phdefwnd = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescriptionA<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szdescription: ::windows::core::PSTR, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDescriptionA(::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&ubuflen), ::core::mem::transmute_copy(&szdescription), ::core::mem::transmute_copy(&pucopied)).into()
        }
        unsafe extern "system" fn GetDescriptionW<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szdescription: ::windows::core::PWSTR, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDescriptionW(::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&ubuflen), ::core::mem::transmute_copy(&szdescription), ::core::mem::transmute_copy(&pucopied)).into()
        }
        unsafe extern "system" fn GetGuideLineA<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, pbuf: ::windows::core::PSTR, pdwresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetGuideLineA(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&dwbuflen), ::core::mem::transmute_copy(&pbuf), ::core::mem::transmute_copy(&pdwresult)).into()
        }
        unsafe extern "system" fn GetGuideLineW<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, pbuf: ::windows::core::PWSTR, pdwresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetGuideLineW(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&dwbuflen), ::core::mem::transmute_copy(&pbuf), ::core::mem::transmute_copy(&pdwresult)).into()
        }
        unsafe extern "system" fn GetIMEFileNameA<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szfilename: ::windows::core::PSTR, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetIMEFileNameA(::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&ubuflen), ::core::mem::transmute_copy(&szfilename), ::core::mem::transmute_copy(&pucopied)).into()
        }
        unsafe extern "system" fn GetIMEFileNameW<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szfilename: ::windows::core::PWSTR, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetIMEFileNameW(::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&ubuflen), ::core::mem::transmute_copy(&szfilename), ::core::mem::transmute_copy(&pucopied)).into()
        }
        unsafe extern "system" fn GetOpenStatus<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetOpenStatus(::core::mem::transmute_copy(&himc)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, fdwindex: u32, pdwproperty: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&fdwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwproperty = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRegisterWordStyleA<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, nitem: u32, pstylebuf: *mut STYLEBUFA, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRegisterWordStyleA(::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&nitem), ::core::mem::transmute_copy(&pstylebuf), ::core::mem::transmute_copy(&pucopied)).into()
        }
        unsafe extern "system" fn GetRegisterWordStyleW<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, nitem: u32, pstylebuf: *mut STYLEBUFW, pucopied: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRegisterWordStyleW(::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&nitem), ::core::mem::transmute_copy(&pstylebuf), ::core::mem::transmute_copy(&pucopied)).into()
        }
        unsafe extern "system" fn GetStatusWindowPos<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pptpos: *mut super::super::super::Foundation::POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStatusWindowPos(::core::mem::transmute_copy(&himc)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptpos = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVirtualKey<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, puvirtualkey: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetVirtualKey(::core::mem::transmute_copy(&hwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    *puvirtualkey = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallIMEA<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szimefilename: ::windows::core::PCSTR, szlayouttext: ::windows::core::PCSTR, phkl: *mut super::super::TextServices::HKL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InstallIMEA(::core::mem::transmute(&szimefilename), ::core::mem::transmute(&szlayouttext)) {
                ::core::result::Result::Ok(ok__) => {
                    *phkl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallIMEW<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szimefilename: ::windows::core::PCWSTR, szlayouttext: ::windows::core::PCWSTR, phkl: *mut super::super::TextServices::HKL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InstallIMEW(::core::mem::transmute(&szimefilename), ::core::mem::transmute(&szlayouttext)) {
                ::core::result::Result::Ok(ok__) => {
                    *phkl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsIME<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsIME(::core::mem::transmute_copy(&hkl)).into()
        }
        unsafe extern "system" fn IsUIMessageA<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndime: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsUIMessageA(::core::mem::transmute_copy(&hwndime), ::core::mem::transmute_copy(&msg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn IsUIMessageW<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndime: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsUIMessageW(::core::mem::transmute_copy(&hwndime), ::core::mem::transmute_copy(&msg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn NotifyIME<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwaction: u32, dwindex: u32, dwvalue: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NotifyIME(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwaction), ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&dwvalue)).into()
        }
        unsafe extern "system" fn RegisterWordA<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: ::windows::core::PCSTR, dwstyle: u32, szregister: ::windows::core::PCSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RegisterWordA(::core::mem::transmute_copy(&hkl), ::core::mem::transmute(&szreading), ::core::mem::transmute_copy(&dwstyle), ::core::mem::transmute(&szregister)).into()
        }
        unsafe extern "system" fn RegisterWordW<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: ::windows::core::PCWSTR, dwstyle: u32, szregister: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RegisterWordW(::core::mem::transmute_copy(&hkl), ::core::mem::transmute(&szreading), ::core::mem::transmute_copy(&dwstyle), ::core::mem::transmute(&szregister)).into()
        }
        unsafe extern "system" fn ReleaseContext<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, himc: super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReleaseContext(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&himc)).into()
        }
        unsafe extern "system" fn SetCandidateWindow<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pcandidate: *const CANDIDATEFORM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCandidateWindow(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&pcandidate)).into()
        }
        unsafe extern "system" fn SetCompositionFontA<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *const super::super::super::Graphics::Gdi::LOGFONTA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCompositionFontA(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&plf)).into()
        }
        unsafe extern "system" fn SetCompositionFontW<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *const super::super::super::Graphics::Gdi::LOGFONTW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCompositionFontW(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&plf)).into()
        }
        unsafe extern "system" fn SetCompositionStringA<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCompositionStringA(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pcomp), ::core::mem::transmute_copy(&dwcomplen), ::core::mem::transmute_copy(&pread), ::core::mem::transmute_copy(&dwreadlen)).into()
        }
        unsafe extern "system" fn SetCompositionStringW<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCompositionStringW(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pcomp), ::core::mem::transmute_copy(&dwcomplen), ::core::mem::transmute_copy(&pread), ::core::mem::transmute_copy(&dwreadlen)).into()
        }
        unsafe extern "system" fn SetCompositionWindow<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pcompform: *const COMPOSITIONFORM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCompositionWindow(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&pcompform)).into()
        }
        unsafe extern "system" fn SetConversionStatus<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, fdwconversion: u32, fdwsentence: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetConversionStatus(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&fdwconversion), ::core::mem::transmute_copy(&fdwsentence)).into()
        }
        unsafe extern "system" fn SetOpenStatus<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, fopen: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOpenStatus(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&fopen)).into()
        }
        unsafe extern "system" fn SetStatusWindowPos<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pptpos: *const super::super::super::Foundation::POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStatusWindowPos(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&pptpos)).into()
        }
        unsafe extern "system" fn SimulateHotKey<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, dwhotkeyid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SimulateHotKey(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&dwhotkeyid)).into()
        }
        unsafe extern "system" fn UnregisterWordA<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: ::windows::core::PCSTR, dwstyle: u32, szunregister: ::windows::core::PCSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnregisterWordA(::core::mem::transmute_copy(&hkl), ::core::mem::transmute(&szreading), ::core::mem::transmute_copy(&dwstyle), ::core::mem::transmute(&szunregister)).into()
        }
        unsafe extern "system" fn UnregisterWordW<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: ::windows::core::PCWSTR, dwstyle: u32, szunregister: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnregisterWordW(::core::mem::transmute_copy(&hkl), ::core::mem::transmute(&szreading), ::core::mem::transmute_copy(&dwstyle), ::core::mem::transmute(&szunregister)).into()
        }
        unsafe extern "system" fn GenerateMessage<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GenerateMessage(::core::mem::transmute_copy(&himc)).into()
        }
        unsafe extern "system" fn LockIMC<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, ppimc: *mut *mut INPUTCONTEXT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LockIMC(::core::mem::transmute_copy(&himc)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppimc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnlockIMC<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnlockIMC(::core::mem::transmute_copy(&himc)).into()
        }
        unsafe extern "system" fn GetIMCLockCount<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pdwlockcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetIMCLockCount(::core::mem::transmute_copy(&himc)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwlockcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateIMCC<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsize: u32, phimcc: *mut super::super::super::Globalization::HIMCC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateIMCC(::core::mem::transmute_copy(&dwsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *phimcc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestroyIMCC<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himcc: super::super::super::Globalization::HIMCC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DestroyIMCC(::core::mem::transmute_copy(&himcc)).into()
        }
        unsafe extern "system" fn LockIMCC<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himcc: super::super::super::Globalization::HIMCC, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LockIMCC(::core::mem::transmute_copy(&himcc), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn UnlockIMCC<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himcc: super::super::super::Globalization::HIMCC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnlockIMCC(::core::mem::transmute_copy(&himcc)).into()
        }
        unsafe extern "system" fn ReSizeIMCC<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himcc: super::super::super::Globalization::HIMCC, dwsize: u32, phimcc: *mut super::super::super::Globalization::HIMCC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReSizeIMCC(::core::mem::transmute_copy(&himcc), ::core::mem::transmute_copy(&dwsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *phimcc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIMCCSize<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himcc: super::super::super::Globalization::HIMCC, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetIMCCSize(::core::mem::transmute_copy(&himcc)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIMCCLockCount<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himcc: super::super::super::Globalization::HIMCC, pdwlockcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetIMCCLockCount(::core::mem::transmute_copy(&himcc)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwlockcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHotKey<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwhotkeyid: u32, pumodifiers: *mut u32, puvkey: *mut u32, phkl: *mut super::super::TextServices::HKL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetHotKey(::core::mem::transmute_copy(&dwhotkeyid), ::core::mem::transmute_copy(&pumodifiers), ::core::mem::transmute_copy(&puvkey), ::core::mem::transmute_copy(&phkl)).into()
        }
        unsafe extern "system" fn SetHotKey<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwhotkeyid: u32, umodifiers: u32, uvkey: u32, hkl: super::super::TextServices::HKL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHotKey(::core::mem::transmute_copy(&dwhotkeyid), ::core::mem::transmute_copy(&umodifiers), ::core::mem::transmute_copy(&uvkey), ::core::mem::transmute_copy(&hkl)).into()
        }
        unsafe extern "system" fn CreateSoftKeyboard<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, utype: u32, howner: super::super::super::Foundation::HWND, x: i32, y: i32, phsoftkbdwnd: *mut super::super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateSoftKeyboard(::core::mem::transmute_copy(&utype), ::core::mem::transmute_copy(&howner), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)) {
                ::core::result::Result::Ok(ok__) => {
                    *phsoftkbdwnd = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestroySoftKeyboard<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hsoftkbdwnd: super::super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DestroySoftKeyboard(::core::mem::transmute_copy(&hsoftkbdwnd)).into()
        }
        unsafe extern "system" fn ShowSoftKeyboard<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hsoftkbdwnd: super::super::super::Foundation::HWND, ncmdshow: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ShowSoftKeyboard(::core::mem::transmute_copy(&hsoftkbdwnd), ::core::mem::transmute_copy(&ncmdshow)).into()
        }
        unsafe extern "system" fn GetCodePageA<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ucodepage: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCodePageA(::core::mem::transmute_copy(&hkl)) {
                ::core::result::Result::Ok(ok__) => {
                    *ucodepage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLangId<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, plid: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLangId(::core::mem::transmute_copy(&hkl)) {
                ::core::result::Result::Ok(ok__) => {
                    *plid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeybdEvent<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lgidime: u16, bvk: u8, bscan: u8, dwflags: u32, dwextrainfo: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).KeybdEvent(::core::mem::transmute_copy(&lgidime), ::core::mem::transmute_copy(&bvk), ::core::mem::transmute_copy(&bscan), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwextrainfo)).into()
        }
        unsafe extern "system" fn LockModal<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LockModal().into()
        }
        unsafe extern "system" fn UnlockModal<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnlockModal().into()
        }
        unsafe extern "system" fn AssociateContextEx<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, himc: super::super::super::Globalization::HIMC, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AssociateContextEx(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn DisableIME<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idthread: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DisableIME(::core::mem::transmute_copy(&idthread)).into()
        }
        unsafe extern "system" fn GetImeMenuItemsA<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOA, pimemenu: *mut IMEMENUITEMINFOA, dwsize: u32, pdwresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetImeMenuItemsA(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwtype), ::core::mem::transmute_copy(&pimeparentmenu), ::core::mem::transmute_copy(&pimemenu), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&pdwresult)).into()
        }
        unsafe extern "system" fn GetImeMenuItemsW<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOW, pimemenu: *mut IMEMENUITEMINFOW, dwsize: u32, pdwresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetImeMenuItemsW(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwtype), ::core::mem::transmute_copy(&pimeparentmenu), ::core::mem::transmute_copy(&pimemenu), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&pdwresult)).into()
        }
        unsafe extern "system" fn EnumInputContext<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idthread: u32, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumInputContext(::core::mem::transmute_copy(&idthread)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestMessageA<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RequestMessageA(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    *plresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestMessageW<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RequestMessageW(::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    *plresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendIMCA<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, umsg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SendIMCA(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&umsg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    *plresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendIMCW<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, umsg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SendIMCW(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&umsg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    *plresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSleeping<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsSleeping().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AssociateContext: AssociateContext::<Identity, Impl, OFFSET>,
            ConfigureIMEA: ConfigureIMEA::<Identity, Impl, OFFSET>,
            ConfigureIMEW: ConfigureIMEW::<Identity, Impl, OFFSET>,
            CreateContext: CreateContext::<Identity, Impl, OFFSET>,
            DestroyContext: DestroyContext::<Identity, Impl, OFFSET>,
            EnumRegisterWordA: EnumRegisterWordA::<Identity, Impl, OFFSET>,
            EnumRegisterWordW: EnumRegisterWordW::<Identity, Impl, OFFSET>,
            EscapeA: EscapeA::<Identity, Impl, OFFSET>,
            EscapeW: EscapeW::<Identity, Impl, OFFSET>,
            GetCandidateListA: GetCandidateListA::<Identity, Impl, OFFSET>,
            GetCandidateListW: GetCandidateListW::<Identity, Impl, OFFSET>,
            GetCandidateListCountA: GetCandidateListCountA::<Identity, Impl, OFFSET>,
            GetCandidateListCountW: GetCandidateListCountW::<Identity, Impl, OFFSET>,
            GetCandidateWindow: GetCandidateWindow::<Identity, Impl, OFFSET>,
            GetCompositionFontA: GetCompositionFontA::<Identity, Impl, OFFSET>,
            GetCompositionFontW: GetCompositionFontW::<Identity, Impl, OFFSET>,
            GetCompositionStringA: GetCompositionStringA::<Identity, Impl, OFFSET>,
            GetCompositionStringW: GetCompositionStringW::<Identity, Impl, OFFSET>,
            GetCompositionWindow: GetCompositionWindow::<Identity, Impl, OFFSET>,
            GetContext: GetContext::<Identity, Impl, OFFSET>,
            GetConversionListA: GetConversionListA::<Identity, Impl, OFFSET>,
            GetConversionListW: GetConversionListW::<Identity, Impl, OFFSET>,
            GetConversionStatus: GetConversionStatus::<Identity, Impl, OFFSET>,
            GetDefaultIMEWnd: GetDefaultIMEWnd::<Identity, Impl, OFFSET>,
            GetDescriptionA: GetDescriptionA::<Identity, Impl, OFFSET>,
            GetDescriptionW: GetDescriptionW::<Identity, Impl, OFFSET>,
            GetGuideLineA: GetGuideLineA::<Identity, Impl, OFFSET>,
            GetGuideLineW: GetGuideLineW::<Identity, Impl, OFFSET>,
            GetIMEFileNameA: GetIMEFileNameA::<Identity, Impl, OFFSET>,
            GetIMEFileNameW: GetIMEFileNameW::<Identity, Impl, OFFSET>,
            GetOpenStatus: GetOpenStatus::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            GetRegisterWordStyleA: GetRegisterWordStyleA::<Identity, Impl, OFFSET>,
            GetRegisterWordStyleW: GetRegisterWordStyleW::<Identity, Impl, OFFSET>,
            GetStatusWindowPos: GetStatusWindowPos::<Identity, Impl, OFFSET>,
            GetVirtualKey: GetVirtualKey::<Identity, Impl, OFFSET>,
            InstallIMEA: InstallIMEA::<Identity, Impl, OFFSET>,
            InstallIMEW: InstallIMEW::<Identity, Impl, OFFSET>,
            IsIME: IsIME::<Identity, Impl, OFFSET>,
            IsUIMessageA: IsUIMessageA::<Identity, Impl, OFFSET>,
            IsUIMessageW: IsUIMessageW::<Identity, Impl, OFFSET>,
            NotifyIME: NotifyIME::<Identity, Impl, OFFSET>,
            RegisterWordA: RegisterWordA::<Identity, Impl, OFFSET>,
            RegisterWordW: RegisterWordW::<Identity, Impl, OFFSET>,
            ReleaseContext: ReleaseContext::<Identity, Impl, OFFSET>,
            SetCandidateWindow: SetCandidateWindow::<Identity, Impl, OFFSET>,
            SetCompositionFontA: SetCompositionFontA::<Identity, Impl, OFFSET>,
            SetCompositionFontW: SetCompositionFontW::<Identity, Impl, OFFSET>,
            SetCompositionStringA: SetCompositionStringA::<Identity, Impl, OFFSET>,
            SetCompositionStringW: SetCompositionStringW::<Identity, Impl, OFFSET>,
            SetCompositionWindow: SetCompositionWindow::<Identity, Impl, OFFSET>,
            SetConversionStatus: SetConversionStatus::<Identity, Impl, OFFSET>,
            SetOpenStatus: SetOpenStatus::<Identity, Impl, OFFSET>,
            SetStatusWindowPos: SetStatusWindowPos::<Identity, Impl, OFFSET>,
            SimulateHotKey: SimulateHotKey::<Identity, Impl, OFFSET>,
            UnregisterWordA: UnregisterWordA::<Identity, Impl, OFFSET>,
            UnregisterWordW: UnregisterWordW::<Identity, Impl, OFFSET>,
            GenerateMessage: GenerateMessage::<Identity, Impl, OFFSET>,
            LockIMC: LockIMC::<Identity, Impl, OFFSET>,
            UnlockIMC: UnlockIMC::<Identity, Impl, OFFSET>,
            GetIMCLockCount: GetIMCLockCount::<Identity, Impl, OFFSET>,
            CreateIMCC: CreateIMCC::<Identity, Impl, OFFSET>,
            DestroyIMCC: DestroyIMCC::<Identity, Impl, OFFSET>,
            LockIMCC: LockIMCC::<Identity, Impl, OFFSET>,
            UnlockIMCC: UnlockIMCC::<Identity, Impl, OFFSET>,
            ReSizeIMCC: ReSizeIMCC::<Identity, Impl, OFFSET>,
            GetIMCCSize: GetIMCCSize::<Identity, Impl, OFFSET>,
            GetIMCCLockCount: GetIMCCLockCount::<Identity, Impl, OFFSET>,
            GetHotKey: GetHotKey::<Identity, Impl, OFFSET>,
            SetHotKey: SetHotKey::<Identity, Impl, OFFSET>,
            CreateSoftKeyboard: CreateSoftKeyboard::<Identity, Impl, OFFSET>,
            DestroySoftKeyboard: DestroySoftKeyboard::<Identity, Impl, OFFSET>,
            ShowSoftKeyboard: ShowSoftKeyboard::<Identity, Impl, OFFSET>,
            GetCodePageA: GetCodePageA::<Identity, Impl, OFFSET>,
            GetLangId: GetLangId::<Identity, Impl, OFFSET>,
            KeybdEvent: KeybdEvent::<Identity, Impl, OFFSET>,
            LockModal: LockModal::<Identity, Impl, OFFSET>,
            UnlockModal: UnlockModal::<Identity, Impl, OFFSET>,
            AssociateContextEx: AssociateContextEx::<Identity, Impl, OFFSET>,
            DisableIME: DisableIME::<Identity, Impl, OFFSET>,
            GetImeMenuItemsA: GetImeMenuItemsA::<Identity, Impl, OFFSET>,
            GetImeMenuItemsW: GetImeMenuItemsW::<Identity, Impl, OFFSET>,
            EnumInputContext: EnumInputContext::<Identity, Impl, OFFSET>,
            RequestMessageA: RequestMessageA::<Identity, Impl, OFFSET>,
            RequestMessageW: RequestMessageW::<Identity, Impl, OFFSET>,
            SendIMCA: SendIMCA::<Identity, Impl, OFFSET>,
            SendIMCW: SendIMCW::<Identity, Impl, OFFSET>,
            IsSleeping: IsSleeping::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActiveIMMIME as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IActiveIMMMessagePumpOwner_Impl: Sized {
    fn Start(&self) -> ::windows::core::Result<()>;
    fn End(&self) -> ::windows::core::Result<()>;
    fn OnTranslateMessage(&self, pmsg: *const super::super::WindowsAndMessaging::MSG) -> ::windows::core::Result<()>;
    fn Pause(&self) -> ::windows::core::Result<u32>;
    fn Resume(&self, dwcookie: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IActiveIMMMessagePumpOwner_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMMessagePumpOwner_Impl, const OFFSET: isize>() -> IActiveIMMMessagePumpOwner_Vtbl {
        unsafe extern "system" fn Start<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMMessagePumpOwner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn End<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMMessagePumpOwner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).End().into()
        }
        unsafe extern "system" fn OnTranslateMessage<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMMessagePumpOwner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *const super::super::WindowsAndMessaging::MSG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnTranslateMessage(::core::mem::transmute_copy(&pmsg)).into()
        }
        unsafe extern "system" fn Pause<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMMessagePumpOwner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Pause() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resume<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMMessagePumpOwner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Resume(::core::mem::transmute_copy(&dwcookie)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Start: Start::<Identity, Impl, OFFSET>,
            End: End::<Identity, Impl, OFFSET>,
            OnTranslateMessage: OnTranslateMessage::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActiveIMMMessagePumpOwner as ::windows::core::Interface>::IID
    }
}
pub trait IActiveIMMRegistrar_Impl: Sized {
    fn RegisterIME(&self, rclsid: *const ::windows::core::GUID, lgid: u16, psziconfile: &::windows::core::PCWSTR, pszdesc: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn UnregisterIME(&self, rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl IActiveIMMRegistrar_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMRegistrar_Impl, const OFFSET: isize>() -> IActiveIMMRegistrar_Vtbl {
        unsafe extern "system" fn RegisterIME<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMRegistrar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, lgid: u16, psziconfile: ::windows::core::PCWSTR, pszdesc: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RegisterIME(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&lgid), ::core::mem::transmute(&psziconfile), ::core::mem::transmute(&pszdesc)).into()
        }
        unsafe extern "system" fn UnregisterIME<Identity: ::windows::core::IUnknownImpl, Impl: IActiveIMMRegistrar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnregisterIME(::core::mem::transmute_copy(&rclsid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            RegisterIME: RegisterIME::<Identity, Impl, OFFSET>,
            UnregisterIME: UnregisterIME::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActiveIMMRegistrar as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Globalization")]
pub trait IEnumInputContext_Impl: Sized {
    fn Clone(&self) -> ::windows::core::Result<IEnumInputContext>;
    fn Next(&self, ulcount: u32, rginputcontext: *mut super::super::super::Globalization::HIMC, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Skip(&self, ulcount: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Globalization")]
impl IEnumInputContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumInputContext_Impl, const OFFSET: isize>() -> IEnumInputContext_Vtbl {
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumInputContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumInputContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, rginputcontext: *mut super::super::super::Globalization::HIMC, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&rginputcontext), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumInputContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumInputContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumInputContext as ::windows::core::Interface>::IID
    }
}
pub trait IEnumRegisterWordA_Impl: Sized {
    fn Clone(&self) -> ::windows::core::Result<IEnumRegisterWordA>;
    fn Next(&self, ulcount: u32, rgregisterword: *mut REGISTERWORDA, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Skip(&self, ulcount: u32) -> ::windows::core::Result<()>;
}
impl IEnumRegisterWordA_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumRegisterWordA_Impl, const OFFSET: isize>() -> IEnumRegisterWordA_Vtbl {
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumRegisterWordA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumRegisterWordA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, rgregisterword: *mut REGISTERWORDA, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&rgregisterword), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumRegisterWordA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumRegisterWordA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumRegisterWordA as ::windows::core::Interface>::IID
    }
}
pub trait IEnumRegisterWordW_Impl: Sized {
    fn Clone(&self) -> ::windows::core::Result<IEnumRegisterWordW>;
    fn Next(&self, ulcount: u32, rgregisterword: *mut REGISTERWORDW, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Skip(&self, ulcount: u32) -> ::windows::core::Result<()>;
}
impl IEnumRegisterWordW_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumRegisterWordW_Impl, const OFFSET: isize>() -> IEnumRegisterWordW_Vtbl {
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumRegisterWordW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumRegisterWordW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, rgregisterword: *mut REGISTERWORDW, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&rgregisterword), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumRegisterWordW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumRegisterWordW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumRegisterWordW as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IFEClassFactory_Impl: Sized + super::super::super::System::Com::IClassFactory_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IFEClassFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFEClassFactory_Impl, const OFFSET: isize>() -> IFEClassFactory_Vtbl {
        Self { base: super::super::super::System::Com::IClassFactory_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFEClassFactory as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IClassFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IFECommon_Impl: Sized {
    fn IsDefaultIME(&self, szname: &::windows::core::PCSTR, cszname: i32) -> ::windows::core::Result<()>;
    fn SetDefaultIME(&self) -> ::windows::core::Result<()>;
    fn InvokeWordRegDialog(&self, pimedlg: *mut IMEDLG) -> ::windows::core::Result<()>;
    fn InvokeDictToolDialog(&self, pimedlg: *mut IMEDLG) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IFECommon_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFECommon_Impl, const OFFSET: isize>() -> IFECommon_Vtbl {
        unsafe extern "system" fn IsDefaultIME<Identity: ::windows::core::IUnknownImpl, Impl: IFECommon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szname: ::windows::core::PCSTR, cszname: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsDefaultIME(::core::mem::transmute(&szname), ::core::mem::transmute_copy(&cszname)).into()
        }
        unsafe extern "system" fn SetDefaultIME<Identity: ::windows::core::IUnknownImpl, Impl: IFECommon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDefaultIME().into()
        }
        unsafe extern "system" fn InvokeWordRegDialog<Identity: ::windows::core::IUnknownImpl, Impl: IFECommon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimedlg: *mut IMEDLG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InvokeWordRegDialog(::core::mem::transmute_copy(&pimedlg)).into()
        }
        unsafe extern "system" fn InvokeDictToolDialog<Identity: ::windows::core::IUnknownImpl, Impl: IFECommon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimedlg: *mut IMEDLG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InvokeDictToolDialog(::core::mem::transmute_copy(&pimedlg)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            IsDefaultIME: IsDefaultIME::<Identity, Impl, OFFSET>,
            SetDefaultIME: SetDefaultIME::<Identity, Impl, OFFSET>,
            InvokeWordRegDialog: InvokeWordRegDialog::<Identity, Impl, OFFSET>,
            InvokeDictToolDialog: InvokeDictToolDialog::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFECommon as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IFEDictionary_Impl: Sized {
    fn Open(&self, pchdictpath: &::windows::core::PSTR, pshf: *mut IMESHF) -> ::windows::core::Result<()>;
    fn Close(&self) -> ::windows::core::Result<()>;
    fn GetHeader(&self, pchdictpath: &::windows::core::PSTR, pshf: *mut IMESHF, pjfmt: *mut IMEFMT, pultype: *mut u32) -> ::windows::core::Result<()>;
    fn DisplayProperty(&self, hwnd: super::super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn GetPosTable(&self, prgpostbl: *mut *mut POSTBL, pcpostbl: *mut i32) -> ::windows::core::Result<()>;
    fn GetWords(&self, pwchfirst: &::windows::core::PCWSTR, pwchlast: &::windows::core::PCWSTR, pwchdisplay: &::windows::core::PCWSTR, ulpos: u32, ulselect: u32, ulwordsrc: u32, pchbuffer: *mut u8, cbbuffer: u32, pcwrd: *mut u32) -> ::windows::core::Result<()>;
    fn NextWords(&self, pchbuffer: *mut u8, cbbuffer: u32, pcwrd: *mut u32) -> ::windows::core::Result<()>;
    fn Create(&self, pchdictpath: &::windows::core::PCSTR, pshf: *mut IMESHF) -> ::windows::core::Result<()>;
    fn SetHeader(&self, pshf: *mut IMESHF) -> ::windows::core::Result<()>;
    fn ExistWord(&self, pwrd: *mut IMEWRD) -> ::windows::core::Result<()>;
    fn ExistDependency(&self, pdp: *mut IMEDP) -> ::windows::core::Result<()>;
    fn RegisterWord(&self, reg: IMEREG, pwrd: *mut IMEWRD) -> ::windows::core::Result<()>;
    fn RegisterDependency(&self, reg: IMEREG, pdp: *mut IMEDP) -> ::windows::core::Result<()>;
    fn GetDependencies(&self, pwchkakarireading: &::windows::core::PCWSTR, pwchkakaridisplay: &::windows::core::PCWSTR, ulkakaripos: u32, pwchukereading: &::windows::core::PCWSTR, pwchukedisplay: &::windows::core::PCWSTR, ulukepos: u32, jrel: IMEREL, ulwordsrc: u32, pchbuffer: *mut u8, cbbuffer: u32, pcdp: *mut u32) -> ::windows::core::Result<()>;
    fn NextDependencies(&self, pchbuffer: *mut u8, cbbuffer: u32, pcdp: *mut u32) -> ::windows::core::Result<()>;
    fn ConvertFromOldMSIME(&self, pchdic: &::windows::core::PCSTR, pfnlog: &PFNLOG, reg: IMEREG) -> ::windows::core::Result<()>;
    fn ConvertFromUserToSys(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IFEDictionary_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFEDictionary_Impl, const OFFSET: isize>() -> IFEDictionary_Vtbl {
        unsafe extern "system" fn Open<Identity: ::windows::core::IUnknownImpl, Impl: IFEDictionary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchdictpath: ::windows::core::PSTR, pshf: *mut IMESHF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Open(::core::mem::transmute(&pchdictpath), ::core::mem::transmute_copy(&pshf)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl, Impl: IFEDictionary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn GetHeader<Identity: ::windows::core::IUnknownImpl, Impl: IFEDictionary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchdictpath: ::windows::core::PSTR, pshf: *mut IMESHF, pjfmt: *mut IMEFMT, pultype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetHeader(::core::mem::transmute(&pchdictpath), ::core::mem::transmute_copy(&pshf), ::core::mem::transmute_copy(&pjfmt), ::core::mem::transmute_copy(&pultype)).into()
        }
        unsafe extern "system" fn DisplayProperty<Identity: ::windows::core::IUnknownImpl, Impl: IFEDictionary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DisplayProperty(::core::mem::transmute_copy(&hwnd)).into()
        }
        unsafe extern "system" fn GetPosTable<Identity: ::windows::core::IUnknownImpl, Impl: IFEDictionary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prgpostbl: *mut *mut POSTBL, pcpostbl: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPosTable(::core::mem::transmute_copy(&prgpostbl), ::core::mem::transmute_copy(&pcpostbl)).into()
        }
        unsafe extern "system" fn GetWords<Identity: ::windows::core::IUnknownImpl, Impl: IFEDictionary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchfirst: ::windows::core::PCWSTR, pwchlast: ::windows::core::PCWSTR, pwchdisplay: ::windows::core::PCWSTR, ulpos: u32, ulselect: u32, ulwordsrc: u32, pchbuffer: *mut u8, cbbuffer: u32, pcwrd: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetWords(::core::mem::transmute(&pwchfirst), ::core::mem::transmute(&pwchlast), ::core::mem::transmute(&pwchdisplay), ::core::mem::transmute_copy(&ulpos), ::core::mem::transmute_copy(&ulselect), ::core::mem::transmute_copy(&ulwordsrc), ::core::mem::transmute_copy(&pchbuffer), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&pcwrd)).into()
        }
        unsafe extern "system" fn NextWords<Identity: ::windows::core::IUnknownImpl, Impl: IFEDictionary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchbuffer: *mut u8, cbbuffer: u32, pcwrd: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NextWords(::core::mem::transmute_copy(&pchbuffer), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&pcwrd)).into()
        }
        unsafe extern "system" fn Create<Identity: ::windows::core::IUnknownImpl, Impl: IFEDictionary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchdictpath: ::windows::core::PCSTR, pshf: *mut IMESHF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Create(::core::mem::transmute(&pchdictpath), ::core::mem::transmute_copy(&pshf)).into()
        }
        unsafe extern "system" fn SetHeader<Identity: ::windows::core::IUnknownImpl, Impl: IFEDictionary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshf: *mut IMESHF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHeader(::core::mem::transmute_copy(&pshf)).into()
        }
        unsafe extern "system" fn ExistWord<Identity: ::windows::core::IUnknownImpl, Impl: IFEDictionary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwrd: *mut IMEWRD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ExistWord(::core::mem::transmute_copy(&pwrd)).into()
        }
        unsafe extern "system" fn ExistDependency<Identity: ::windows::core::IUnknownImpl, Impl: IFEDictionary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdp: *mut IMEDP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ExistDependency(::core::mem::transmute_copy(&pdp)).into()
        }
        unsafe extern "system" fn RegisterWord<Identity: ::windows::core::IUnknownImpl, Impl: IFEDictionary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reg: IMEREG, pwrd: *mut IMEWRD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RegisterWord(::core::mem::transmute_copy(&reg), ::core::mem::transmute_copy(&pwrd)).into()
        }
        unsafe extern "system" fn RegisterDependency<Identity: ::windows::core::IUnknownImpl, Impl: IFEDictionary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reg: IMEREG, pdp: *mut IMEDP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RegisterDependency(::core::mem::transmute_copy(&reg), ::core::mem::transmute_copy(&pdp)).into()
        }
        unsafe extern "system" fn GetDependencies<Identity: ::windows::core::IUnknownImpl, Impl: IFEDictionary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchkakarireading: ::windows::core::PCWSTR, pwchkakaridisplay: ::windows::core::PCWSTR, ulkakaripos: u32, pwchukereading: ::windows::core::PCWSTR, pwchukedisplay: ::windows::core::PCWSTR, ulukepos: u32, jrel: IMEREL, ulwordsrc: u32, pchbuffer: *mut u8, cbbuffer: u32, pcdp: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDependencies(::core::mem::transmute(&pwchkakarireading), ::core::mem::transmute(&pwchkakaridisplay), ::core::mem::transmute_copy(&ulkakaripos), ::core::mem::transmute(&pwchukereading), ::core::mem::transmute(&pwchukedisplay), ::core::mem::transmute_copy(&ulukepos), ::core::mem::transmute_copy(&jrel), ::core::mem::transmute_copy(&ulwordsrc), ::core::mem::transmute_copy(&pchbuffer), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&pcdp)).into()
        }
        unsafe extern "system" fn NextDependencies<Identity: ::windows::core::IUnknownImpl, Impl: IFEDictionary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchbuffer: *mut u8, cbbuffer: u32, pcdp: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NextDependencies(::core::mem::transmute_copy(&pchbuffer), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&pcdp)).into()
        }
        unsafe extern "system" fn ConvertFromOldMSIME<Identity: ::windows::core::IUnknownImpl, Impl: IFEDictionary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchdic: ::windows::core::PCSTR, pfnlog: ::windows::core::RawPtr, reg: IMEREG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ConvertFromOldMSIME(::core::mem::transmute(&pchdic), ::core::mem::transmute(&pfnlog), ::core::mem::transmute_copy(&reg)).into()
        }
        unsafe extern "system" fn ConvertFromUserToSys<Identity: ::windows::core::IUnknownImpl, Impl: IFEDictionary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ConvertFromUserToSys().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            GetHeader: GetHeader::<Identity, Impl, OFFSET>,
            DisplayProperty: DisplayProperty::<Identity, Impl, OFFSET>,
            GetPosTable: GetPosTable::<Identity, Impl, OFFSET>,
            GetWords: GetWords::<Identity, Impl, OFFSET>,
            NextWords: NextWords::<Identity, Impl, OFFSET>,
            Create: Create::<Identity, Impl, OFFSET>,
            SetHeader: SetHeader::<Identity, Impl, OFFSET>,
            ExistWord: ExistWord::<Identity, Impl, OFFSET>,
            ExistDependency: ExistDependency::<Identity, Impl, OFFSET>,
            RegisterWord: RegisterWord::<Identity, Impl, OFFSET>,
            RegisterDependency: RegisterDependency::<Identity, Impl, OFFSET>,
            GetDependencies: GetDependencies::<Identity, Impl, OFFSET>,
            NextDependencies: NextDependencies::<Identity, Impl, OFFSET>,
            ConvertFromOldMSIME: ConvertFromOldMSIME::<Identity, Impl, OFFSET>,
            ConvertFromUserToSys: ConvertFromUserToSys::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFEDictionary as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IFELanguage_Impl: Sized {
    fn Open(&self) -> ::windows::core::Result<()>;
    fn Close(&self) -> ::windows::core::Result<()>;
    fn GetJMorphResult(&self, dwrequest: u32, dwcmode: u32, cwchinput: i32, pwchinput: &::windows::core::PCWSTR, pfcinfo: *mut u32, ppresult: *mut *mut MORRSLT) -> ::windows::core::Result<()>;
    fn GetConversionModeCaps(&self, pdwcaps: *mut u32) -> ::windows::core::Result<()>;
    fn GetPhonetic(&self, string: &super::super::super::Foundation::BSTR, start: i32, length: i32, phonetic: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetConversion(&self, string: &super::super::super::Foundation::BSTR, start: i32, length: i32, result: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IFELanguage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFELanguage_Impl, const OFFSET: isize>() -> IFELanguage_Vtbl {
        unsafe extern "system" fn Open<Identity: ::windows::core::IUnknownImpl, Impl: IFELanguage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Open().into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl, Impl: IFELanguage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn GetJMorphResult<Identity: ::windows::core::IUnknownImpl, Impl: IFELanguage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwrequest: u32, dwcmode: u32, cwchinput: i32, pwchinput: ::windows::core::PCWSTR, pfcinfo: *mut u32, ppresult: *mut *mut MORRSLT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetJMorphResult(::core::mem::transmute_copy(&dwrequest), ::core::mem::transmute_copy(&dwcmode), ::core::mem::transmute_copy(&cwchinput), ::core::mem::transmute(&pwchinput), ::core::mem::transmute_copy(&pfcinfo), ::core::mem::transmute_copy(&ppresult)).into()
        }
        unsafe extern "system" fn GetConversionModeCaps<Identity: ::windows::core::IUnknownImpl, Impl: IFELanguage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcaps: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetConversionModeCaps(::core::mem::transmute_copy(&pdwcaps)).into()
        }
        unsafe extern "system" fn GetPhonetic<Identity: ::windows::core::IUnknownImpl, Impl: IFELanguage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, string: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, start: i32, length: i32, phonetic: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPhonetic(::core::mem::transmute(&string), ::core::mem::transmute_copy(&start), ::core::mem::transmute_copy(&length), ::core::mem::transmute_copy(&phonetic)).into()
        }
        unsafe extern "system" fn GetConversion<Identity: ::windows::core::IUnknownImpl, Impl: IFELanguage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, string: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, start: i32, length: i32, result: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetConversion(::core::mem::transmute(&string), ::core::mem::transmute_copy(&start), ::core::mem::transmute_copy(&length), ::core::mem::transmute_copy(&result)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            GetJMorphResult: GetJMorphResult::<Identity, Impl, OFFSET>,
            GetConversionModeCaps: GetConversionModeCaps::<Identity, Impl, OFFSET>,
            GetPhonetic: GetPhonetic::<Identity, Impl, OFFSET>,
            GetConversion: GetConversion::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFELanguage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IImePad_Impl: Sized {
    fn Request(&self, piimepadapplet: &::core::option::Option<IImePadApplet>, reqid: IME_PAD_REQUEST_FLAGS, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IImePad_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImePad_Impl, const OFFSET: isize>() -> IImePad_Vtbl {
        unsafe extern "system" fn Request<Identity: ::windows::core::IUnknownImpl, Impl: IImePad_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piimepadapplet: ::windows::core::RawPtr, reqid: IME_PAD_REQUEST_FLAGS, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Request(::core::mem::transmute(&piimepadapplet), ::core::mem::transmute_copy(&reqid), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Request: Request::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IImePad as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IImePadApplet_Impl: Sized {
    fn Initialize(&self, lpiimepad: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn Terminate(&self) -> ::windows::core::Result<()>;
    fn GetAppletConfig(&self, lpappletcfg: *mut IMEAPPLETCFG) -> ::windows::core::Result<()>;
    fn CreateUI(&self, hwndparent: super::super::super::Foundation::HWND, lpimeappletui: *mut IMEAPPLETUI) -> ::windows::core::Result<()>;
    fn Notify(&self, lpimepad: &::core::option::Option<::windows::core::IUnknown>, notify: i32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IImePadApplet_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImePadApplet_Impl, const OFFSET: isize>() -> IImePadApplet_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IImePadApplet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpiimepad: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&lpiimepad)).into()
        }
        unsafe extern "system" fn Terminate<Identity: ::windows::core::IUnknownImpl, Impl: IImePadApplet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Terminate().into()
        }
        unsafe extern "system" fn GetAppletConfig<Identity: ::windows::core::IUnknownImpl, Impl: IImePadApplet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpappletcfg: *mut IMEAPPLETCFG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAppletConfig(::core::mem::transmute_copy(&lpappletcfg)).into()
        }
        unsafe extern "system" fn CreateUI<Identity: ::windows::core::IUnknownImpl, Impl: IImePadApplet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::Foundation::HWND, lpimeappletui: *mut IMEAPPLETUI) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateUI(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&lpimeappletui)).into()
        }
        unsafe extern "system" fn Notify<Identity: ::windows::core::IUnknownImpl, Impl: IImePadApplet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpimepad: *mut ::core::ffi::c_void, notify: i32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Notify(::core::mem::transmute(&lpimepad), ::core::mem::transmute_copy(&notify), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Terminate: Terminate::<Identity, Impl, OFFSET>,
            GetAppletConfig: GetAppletConfig::<Identity, Impl, OFFSET>,
            CreateUI: CreateUI::<Identity, Impl, OFFSET>,
            Notify: Notify::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IImePadApplet as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IImePlugInDictDictionaryList_Impl: Sized {
    fn GetDictionariesInUse(&self, prgdictionaryguid: *mut *mut super::super::super::System::Com::SAFEARRAY, prgdatecreated: *mut *mut super::super::super::System::Com::SAFEARRAY, prgfencrypted: *mut *mut super::super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn DeleteDictionary(&self, bstrdictionaryguid: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IImePlugInDictDictionaryList_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImePlugInDictDictionaryList_Impl, const OFFSET: isize>() -> IImePlugInDictDictionaryList_Vtbl {
        unsafe extern "system" fn GetDictionariesInUse<Identity: ::windows::core::IUnknownImpl, Impl: IImePlugInDictDictionaryList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prgdictionaryguid: *mut *mut super::super::super::System::Com::SAFEARRAY, prgdatecreated: *mut *mut super::super::super::System::Com::SAFEARRAY, prgfencrypted: *mut *mut super::super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDictionariesInUse(::core::mem::transmute_copy(&prgdictionaryguid), ::core::mem::transmute_copy(&prgdatecreated), ::core::mem::transmute_copy(&prgfencrypted)).into()
        }
        unsafe extern "system" fn DeleteDictionary<Identity: ::windows::core::IUnknownImpl, Impl: IImePlugInDictDictionaryList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdictionaryguid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteDictionary(::core::mem::transmute(&bstrdictionaryguid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetDictionariesInUse: GetDictionariesInUse::<Identity, Impl, OFFSET>,
            DeleteDictionary: DeleteDictionary::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IImePlugInDictDictionaryList as ::windows::core::Interface>::IID
    }
}
pub trait IImeSpecifyApplets_Impl: Sized {
    fn GetAppletIIDList(&self, refiid: *const ::windows::core::GUID, lpiidlist: *mut APPLETIDLIST) -> ::windows::core::Result<()>;
}
impl IImeSpecifyApplets_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImeSpecifyApplets_Impl, const OFFSET: isize>() -> IImeSpecifyApplets_Vtbl {
        unsafe extern "system" fn GetAppletIIDList<Identity: ::windows::core::IUnknownImpl, Impl: IImeSpecifyApplets_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, refiid: *const ::windows::core::GUID, lpiidlist: *mut APPLETIDLIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAppletIIDList(::core::mem::transmute_copy(&refiid), ::core::mem::transmute_copy(&lpiidlist)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetAppletIIDList: GetAppletIIDList::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IImeSpecifyApplets as ::windows::core::Interface>::IID
    }
}
