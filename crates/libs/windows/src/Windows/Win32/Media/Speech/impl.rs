pub trait IEnumSpObjectTokens_Impl: Sized {
    fn Next(&mut self, celt: u32, pelt: *mut ::core::option::Option<ISpObjectToken>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumSpObjectTokens>;
    fn Item(&mut self, index: u32) -> ::windows::core::Result<ISpObjectToken>;
    fn GetCount(&mut self, pcount: *mut u32) -> ::windows::core::Result<()>;
}
impl IEnumSpObjectTokens_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSpObjectTokens_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumSpObjectTokens_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumSpObjectTokens_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, pelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&pelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumSpObjectTokens_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumSpObjectTokens_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumSpObjectTokens_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IEnumSpObjectTokens_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pptoken: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptoken = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Impl: IEnumSpObjectTokens_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCount(::core::mem::transmute_copy(&pcount)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumSpObjectTokens as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait ISpAudio_Impl: Sized + super::super::System::Com::ISequentialStream_Impl + super::super::System::Com::IStream_Impl + ISpStreamFormat_Impl {
    fn SetState(&mut self, newstate: SPAUDIOSTATE, ullreserved: u64) -> ::windows::core::Result<()>;
    fn SetFormat(&mut self, rguidfmtid: *const ::windows::core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> ::windows::core::Result<()>;
    fn GetStatus(&mut self, pstatus: *mut SPAUDIOSTATUS) -> ::windows::core::Result<()>;
    fn SetBufferInfo(&mut self, pbuffinfo: *const SPAUDIOBUFFERINFO) -> ::windows::core::Result<()>;
    fn GetBufferInfo(&mut self, pbuffinfo: *mut SPAUDIOBUFFERINFO) -> ::windows::core::Result<()>;
    fn GetDefaultFormat(&mut self, pformatid: *mut ::windows::core::GUID, ppcomemwaveformatex: *mut *mut super::Audio::WAVEFORMATEX) -> ::windows::core::Result<()>;
    fn EventHandle(&mut self) -> super::super::Foundation::HANDLE;
    fn GetVolumeLevel(&mut self, plevel: *mut u32) -> ::windows::core::Result<()>;
    fn SetVolumeLevel(&mut self, level: u32) -> ::windows::core::Result<()>;
    fn GetBufferNotifySize(&mut self, pcbsize: *mut u32) -> ::windows::core::Result<()>;
    fn SetBufferNotifySize(&mut self, cbsize: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl ISpAudio_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpAudio_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpAudio_Vtbl {
        unsafe extern "system" fn SetState<Impl: ISpAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newstate: SPAUDIOSTATE, ullreserved: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetState(::core::mem::transmute_copy(&newstate), ::core::mem::transmute_copy(&ullreserved)).into()
        }
        unsafe extern "system" fn SetFormat<Impl: ISpAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidfmtid: *const ::windows::core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFormat(::core::mem::transmute_copy(&rguidfmtid), ::core::mem::transmute_copy(&pwaveformatex)).into()
        }
        unsafe extern "system" fn GetStatus<Impl: ISpAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut SPAUDIOSTATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStatus(::core::mem::transmute_copy(&pstatus)).into()
        }
        unsafe extern "system" fn SetBufferInfo<Impl: ISpAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffinfo: *const SPAUDIOBUFFERINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBufferInfo(::core::mem::transmute_copy(&pbuffinfo)).into()
        }
        unsafe extern "system" fn GetBufferInfo<Impl: ISpAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffinfo: *mut SPAUDIOBUFFERINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBufferInfo(::core::mem::transmute_copy(&pbuffinfo)).into()
        }
        unsafe extern "system" fn GetDefaultFormat<Impl: ISpAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatid: *mut ::windows::core::GUID, ppcomemwaveformatex: *mut *mut super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDefaultFormat(::core::mem::transmute_copy(&pformatid), ::core::mem::transmute_copy(&ppcomemwaveformatex)).into()
        }
        unsafe extern "system" fn EventHandle<Impl: ISpAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EventHandle()
        }
        unsafe extern "system" fn GetVolumeLevel<Impl: ISpAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plevel: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetVolumeLevel(::core::mem::transmute_copy(&plevel)).into()
        }
        unsafe extern "system" fn SetVolumeLevel<Impl: ISpAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVolumeLevel(::core::mem::transmute_copy(&level)).into()
        }
        unsafe extern "system" fn GetBufferNotifySize<Impl: ISpAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBufferNotifySize(::core::mem::transmute_copy(&pcbsize)).into()
        }
        unsafe extern "system" fn SetBufferNotifySize<Impl: ISpAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBufferNotifySize(::core::mem::transmute_copy(&cbsize)).into()
        }
        Self {
            base: ISpStreamFormat_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetState: SetState::<Impl, IMPL_OFFSET>,
            SetFormat: SetFormat::<Impl, IMPL_OFFSET>,
            GetStatus: GetStatus::<Impl, IMPL_OFFSET>,
            SetBufferInfo: SetBufferInfo::<Impl, IMPL_OFFSET>,
            GetBufferInfo: GetBufferInfo::<Impl, IMPL_OFFSET>,
            GetDefaultFormat: GetDefaultFormat::<Impl, IMPL_OFFSET>,
            EventHandle: EventHandle::<Impl, IMPL_OFFSET>,
            GetVolumeLevel: GetVolumeLevel::<Impl, IMPL_OFFSET>,
            SetVolumeLevel: SetVolumeLevel::<Impl, IMPL_OFFSET>,
            GetBufferNotifySize: GetBufferNotifySize::<Impl, IMPL_OFFSET>,
            SetBufferNotifySize: SetBufferNotifySize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpAudio as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISpContainerLexicon_Impl: Sized + ISpLexicon_Impl {
    fn AddLexicon(&mut self, paddlexicon: &::core::option::Option<ISpLexicon>, dwflags: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISpContainerLexicon_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpContainerLexicon_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpContainerLexicon_Vtbl {
        unsafe extern "system" fn AddLexicon<Impl: ISpContainerLexicon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddlexicon: ::windows::core::RawPtr, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddLexicon(::core::mem::transmute(&paddlexicon), ::core::mem::transmute_copy(&dwflags)).into()
        }
        Self { base: ISpLexicon_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), AddLexicon: AddLexicon::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpContainerLexicon as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISpDataKey_Impl: Sized {
    fn SetData(&mut self, pszvaluename: super::super::Foundation::PWSTR, cbdata: u32, pdata: *const u8) -> ::windows::core::Result<()>;
    fn GetData(&mut self, pszvaluename: super::super::Foundation::PWSTR, pcbdata: *mut u32, pdata: *mut u8) -> ::windows::core::Result<()>;
    fn SetStringValue(&mut self, pszvaluename: super::super::Foundation::PWSTR, pszvalue: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetStringValue(&mut self, pszvaluename: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetDWORD(&mut self, pszvaluename: super::super::Foundation::PWSTR, dwvalue: u32) -> ::windows::core::Result<()>;
    fn GetDWORD(&mut self, pszvaluename: super::super::Foundation::PWSTR, pdwvalue: *mut u32) -> ::windows::core::Result<()>;
    fn OpenKey(&mut self, pszsubkeyname: super::super::Foundation::PWSTR) -> ::windows::core::Result<ISpDataKey>;
    fn CreateKey(&mut self, pszsubkey: super::super::Foundation::PWSTR) -> ::windows::core::Result<ISpDataKey>;
    fn DeleteKey(&mut self, pszsubkey: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn DeleteValue(&mut self, pszvaluename: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn EnumKeys(&mut self, index: u32) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn EnumValues(&mut self, index: u32) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISpDataKey_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpDataKey_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpDataKey_Vtbl {
        unsafe extern "system" fn SetData<Impl: ISpDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszvaluename: super::super::Foundation::PWSTR, cbdata: u32, pdata: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetData(::core::mem::transmute_copy(&pszvaluename), ::core::mem::transmute_copy(&cbdata), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn GetData<Impl: ISpDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszvaluename: super::super::Foundation::PWSTR, pcbdata: *mut u32, pdata: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetData(::core::mem::transmute_copy(&pszvaluename), ::core::mem::transmute_copy(&pcbdata), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetStringValue<Impl: ISpDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszvaluename: super::super::Foundation::PWSTR, pszvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStringValue(::core::mem::transmute_copy(&pszvaluename), ::core::mem::transmute_copy(&pszvalue)).into()
        }
        unsafe extern "system" fn GetStringValue<Impl: ISpDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszvaluename: super::super::Foundation::PWSTR, ppszvalue: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStringValue(::core::mem::transmute_copy(&pszvaluename)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppszvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDWORD<Impl: ISpDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszvaluename: super::super::Foundation::PWSTR, dwvalue: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDWORD(::core::mem::transmute_copy(&pszvaluename), ::core::mem::transmute_copy(&dwvalue)).into()
        }
        unsafe extern "system" fn GetDWORD<Impl: ISpDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszvaluename: super::super::Foundation::PWSTR, pdwvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDWORD(::core::mem::transmute_copy(&pszvaluename), ::core::mem::transmute_copy(&pdwvalue)).into()
        }
        unsafe extern "system" fn OpenKey<Impl: ISpDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubkeyname: super::super::Foundation::PWSTR, ppsubkey: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenKey(::core::mem::transmute_copy(&pszsubkeyname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsubkey = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateKey<Impl: ISpDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubkey: super::super::Foundation::PWSTR, ppsubkey: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateKey(::core::mem::transmute_copy(&pszsubkey)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsubkey = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteKey<Impl: ISpDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubkey: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteKey(::core::mem::transmute_copy(&pszsubkey)).into()
        }
        unsafe extern "system" fn DeleteValue<Impl: ISpDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszvaluename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteValue(::core::mem::transmute_copy(&pszvaluename)).into()
        }
        unsafe extern "system" fn EnumKeys<Impl: ISpDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ppszsubkeyname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumKeys(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppszsubkeyname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumValues<Impl: ISpDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ppszvaluename: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumValues(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppszvaluename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetData: SetData::<Impl, IMPL_OFFSET>,
            GetData: GetData::<Impl, IMPL_OFFSET>,
            SetStringValue: SetStringValue::<Impl, IMPL_OFFSET>,
            GetStringValue: GetStringValue::<Impl, IMPL_OFFSET>,
            SetDWORD: SetDWORD::<Impl, IMPL_OFFSET>,
            GetDWORD: GetDWORD::<Impl, IMPL_OFFSET>,
            OpenKey: OpenKey::<Impl, IMPL_OFFSET>,
            CreateKey: CreateKey::<Impl, IMPL_OFFSET>,
            DeleteKey: DeleteKey::<Impl, IMPL_OFFSET>,
            DeleteValue: DeleteValue::<Impl, IMPL_OFFSET>,
            EnumKeys: EnumKeys::<Impl, IMPL_OFFSET>,
            EnumValues: EnumValues::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpDataKey as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISpDisplayAlternates_Impl: Sized {
    fn GetDisplayAlternates(&mut self, pphrase: *const SPDISPLAYPHRASE, crequestcount: u32, ppcomemphrases: *mut *mut SPDISPLAYPHRASE, pcphrasesreturned: *mut u32) -> ::windows::core::Result<()>;
    fn SetFullStopTrailSpace(&mut self, ultrailspace: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISpDisplayAlternates_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpDisplayAlternates_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpDisplayAlternates_Vtbl {
        unsafe extern "system" fn GetDisplayAlternates<Impl: ISpDisplayAlternates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphrase: *const SPDISPLAYPHRASE, crequestcount: u32, ppcomemphrases: *mut *mut SPDISPLAYPHRASE, pcphrasesreturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDisplayAlternates(::core::mem::transmute_copy(&pphrase), ::core::mem::transmute_copy(&crequestcount), ::core::mem::transmute_copy(&ppcomemphrases), ::core::mem::transmute_copy(&pcphrasesreturned)).into()
        }
        unsafe extern "system" fn SetFullStopTrailSpace<Impl: ISpDisplayAlternates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ultrailspace: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFullStopTrailSpace(::core::mem::transmute_copy(&ultrailspace)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetDisplayAlternates: GetDisplayAlternates::<Impl, IMPL_OFFSET>,
            SetFullStopTrailSpace: SetFullStopTrailSpace::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpDisplayAlternates as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISpEnginePronunciation_Impl: Sized {
    fn Normalize(&mut self, pszword: super::super::Foundation::PWSTR, pszleftcontext: super::super::Foundation::PWSTR, pszrightcontext: super::super::Foundation::PWSTR, langid: u16, pnormalizationlist: *mut SPNORMALIZATIONLIST) -> ::windows::core::Result<()>;
    fn GetPronunciations(&mut self, pszword: super::super::Foundation::PWSTR, pszleftcontext: super::super::Foundation::PWSTR, pszrightcontext: super::super::Foundation::PWSTR, langid: u16, penginepronunciationlist: *mut SPWORDPRONUNCIATIONLIST) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISpEnginePronunciation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpEnginePronunciation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpEnginePronunciation_Vtbl {
        unsafe extern "system" fn Normalize<Impl: ISpEnginePronunciation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszword: super::super::Foundation::PWSTR, pszleftcontext: super::super::Foundation::PWSTR, pszrightcontext: super::super::Foundation::PWSTR, langid: u16, pnormalizationlist: *mut SPNORMALIZATIONLIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Normalize(::core::mem::transmute_copy(&pszword), ::core::mem::transmute_copy(&pszleftcontext), ::core::mem::transmute_copy(&pszrightcontext), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&pnormalizationlist)).into()
        }
        unsafe extern "system" fn GetPronunciations<Impl: ISpEnginePronunciation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszword: super::super::Foundation::PWSTR, pszleftcontext: super::super::Foundation::PWSTR, pszrightcontext: super::super::Foundation::PWSTR, langid: u16, penginepronunciationlist: *mut SPWORDPRONUNCIATIONLIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPronunciations(::core::mem::transmute_copy(&pszword), ::core::mem::transmute_copy(&pszleftcontext), ::core::mem::transmute_copy(&pszrightcontext), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&penginepronunciationlist)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Normalize: Normalize::<Impl, IMPL_OFFSET>,
            GetPronunciations: GetPronunciations::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpEnginePronunciation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISpEventSink_Impl: Sized {
    fn AddEvents(&mut self, peventarray: *const SPEVENT, ulcount: u32) -> ::windows::core::Result<()>;
    fn GetEventInterest(&mut self, pulleventinterest: *mut u64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISpEventSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpEventSink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpEventSink_Vtbl {
        unsafe extern "system" fn AddEvents<Impl: ISpEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventarray: *const SPEVENT, ulcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddEvents(::core::mem::transmute_copy(&peventarray), ::core::mem::transmute_copy(&ulcount)).into()
        }
        unsafe extern "system" fn GetEventInterest<Impl: ISpEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulleventinterest: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetEventInterest(::core::mem::transmute_copy(&pulleventinterest)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddEvents: AddEvents::<Impl, IMPL_OFFSET>,
            GetEventInterest: GetEventInterest::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpEventSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISpEventSource_Impl: Sized + ISpNotifySource_Impl {
    fn SetInterest(&mut self, ulleventinterest: u64, ullqueuedinterest: u64) -> ::windows::core::Result<()>;
    fn GetEvents(&mut self, ulcount: u32, peventarray: *mut SPEVENT, pulfetched: *mut u32) -> ::windows::core::Result<()>;
    fn GetInfo(&mut self, pinfo: *mut SPEVENTSOURCEINFO) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISpEventSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpEventSource_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpEventSource_Vtbl {
        unsafe extern "system" fn SetInterest<Impl: ISpEventSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulleventinterest: u64, ullqueuedinterest: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInterest(::core::mem::transmute_copy(&ulleventinterest), ::core::mem::transmute_copy(&ullqueuedinterest)).into()
        }
        unsafe extern "system" fn GetEvents<Impl: ISpEventSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, peventarray: *mut SPEVENT, pulfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetEvents(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&peventarray), ::core::mem::transmute_copy(&pulfetched)).into()
        }
        unsafe extern "system" fn GetInfo<Impl: ISpEventSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *mut SPEVENTSOURCEINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInfo(::core::mem::transmute_copy(&pinfo)).into()
        }
        Self {
            base: ISpNotifySource_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetInterest: SetInterest::<Impl, IMPL_OFFSET>,
            GetEvents: GetEvents::<Impl, IMPL_OFFSET>,
            GetInfo: GetInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpEventSource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISpEventSource2_Impl: Sized + ISpNotifySource_Impl + ISpEventSource_Impl {
    fn GetEventsEx(&mut self, ulcount: u32, peventarray: *mut SPEVENTEX, pulfetched: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISpEventSource2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpEventSource2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpEventSource2_Vtbl {
        unsafe extern "system" fn GetEventsEx<Impl: ISpEventSource2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, peventarray: *mut SPEVENTEX, pulfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetEventsEx(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&peventarray), ::core::mem::transmute_copy(&pulfetched)).into()
        }
        Self { base: ISpEventSource_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetEventsEx: GetEventsEx::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpEventSource2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpGrammarBuilder_Impl: Sized {
    fn ResetGrammar(&mut self, newlanguage: u16) -> ::windows::core::Result<()>;
    fn GetRule(&mut self, pszrulename: super::super::Foundation::PWSTR, dwruleid: u32, dwattributes: u32, fcreateifnotexist: super::super::Foundation::BOOL, phinitialstate: *mut *mut SPSTATEHANDLE__) -> ::windows::core::Result<()>;
    fn ClearRule(&mut self, hstate: *mut SPSTATEHANDLE__) -> ::windows::core::Result<()>;
    fn CreateNewState(&mut self, hstate: *mut SPSTATEHANDLE__, phstate: *mut *mut SPSTATEHANDLE__) -> ::windows::core::Result<()>;
    fn AddWordTransition(&mut self, hfromstate: *mut SPSTATEHANDLE__, htostate: *mut SPSTATEHANDLE__, psz: super::super::Foundation::PWSTR, pszseparators: super::super::Foundation::PWSTR, ewordtype: SPGRAMMARWORDTYPE, weight: f32, ppropinfo: *const SPPROPERTYINFO) -> ::windows::core::Result<()>;
    fn AddRuleTransition(&mut self, hfromstate: *mut SPSTATEHANDLE__, htostate: *mut SPSTATEHANDLE__, hrule: *mut SPSTATEHANDLE__, weight: f32, ppropinfo: *const SPPROPERTYINFO) -> ::windows::core::Result<()>;
    fn AddResource(&mut self, hrulestate: *mut SPSTATEHANDLE__, pszresourcename: super::super::Foundation::PWSTR, pszresourcevalue: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Commit(&mut self, dwreserved: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpGrammarBuilder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpGrammarBuilder_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpGrammarBuilder_Vtbl {
        unsafe extern "system" fn ResetGrammar<Impl: ISpGrammarBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newlanguage: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResetGrammar(::core::mem::transmute_copy(&newlanguage)).into()
        }
        unsafe extern "system" fn GetRule<Impl: ISpGrammarBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszrulename: super::super::Foundation::PWSTR, dwruleid: u32, dwattributes: u32, fcreateifnotexist: super::super::Foundation::BOOL, phinitialstate: *mut *mut SPSTATEHANDLE__) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRule(::core::mem::transmute_copy(&pszrulename), ::core::mem::transmute_copy(&dwruleid), ::core::mem::transmute_copy(&dwattributes), ::core::mem::transmute_copy(&fcreateifnotexist), ::core::mem::transmute_copy(&phinitialstate)).into()
        }
        unsafe extern "system" fn ClearRule<Impl: ISpGrammarBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hstate: *mut SPSTATEHANDLE__) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearRule(::core::mem::transmute_copy(&hstate)).into()
        }
        unsafe extern "system" fn CreateNewState<Impl: ISpGrammarBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hstate: *mut SPSTATEHANDLE__, phstate: *mut *mut SPSTATEHANDLE__) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateNewState(::core::mem::transmute_copy(&hstate), ::core::mem::transmute_copy(&phstate)).into()
        }
        unsafe extern "system" fn AddWordTransition<Impl: ISpGrammarBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hfromstate: *mut SPSTATEHANDLE__, htostate: *mut SPSTATEHANDLE__, psz: super::super::Foundation::PWSTR, pszseparators: super::super::Foundation::PWSTR, ewordtype: SPGRAMMARWORDTYPE, weight: f32, ppropinfo: *const SPPROPERTYINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddWordTransition(::core::mem::transmute_copy(&hfromstate), ::core::mem::transmute_copy(&htostate), ::core::mem::transmute_copy(&psz), ::core::mem::transmute_copy(&pszseparators), ::core::mem::transmute_copy(&ewordtype), ::core::mem::transmute_copy(&weight), ::core::mem::transmute_copy(&ppropinfo)).into()
        }
        unsafe extern "system" fn AddRuleTransition<Impl: ISpGrammarBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hfromstate: *mut SPSTATEHANDLE__, htostate: *mut SPSTATEHANDLE__, hrule: *mut SPSTATEHANDLE__, weight: f32, ppropinfo: *const SPPROPERTYINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddRuleTransition(::core::mem::transmute_copy(&hfromstate), ::core::mem::transmute_copy(&htostate), ::core::mem::transmute_copy(&hrule), ::core::mem::transmute_copy(&weight), ::core::mem::transmute_copy(&ppropinfo)).into()
        }
        unsafe extern "system" fn AddResource<Impl: ISpGrammarBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrulestate: *mut SPSTATEHANDLE__, pszresourcename: super::super::Foundation::PWSTR, pszresourcevalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddResource(::core::mem::transmute_copy(&hrulestate), ::core::mem::transmute_copy(&pszresourcename), ::core::mem::transmute_copy(&pszresourcevalue)).into()
        }
        unsafe extern "system" fn Commit<Impl: ISpGrammarBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Commit(::core::mem::transmute_copy(&dwreserved)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ResetGrammar: ResetGrammar::<Impl, IMPL_OFFSET>,
            GetRule: GetRule::<Impl, IMPL_OFFSET>,
            ClearRule: ClearRule::<Impl, IMPL_OFFSET>,
            CreateNewState: CreateNewState::<Impl, IMPL_OFFSET>,
            AddWordTransition: AddWordTransition::<Impl, IMPL_OFFSET>,
            AddRuleTransition: AddRuleTransition::<Impl, IMPL_OFFSET>,
            AddResource: AddResource::<Impl, IMPL_OFFSET>,
            Commit: Commit::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpGrammarBuilder as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISpGrammarBuilder2_Impl: Sized {
    fn AddTextSubset(&mut self, hfromstate: *mut SPSTATEHANDLE__, htostate: *mut SPSTATEHANDLE__, psz: super::super::Foundation::PWSTR, ematchmode: SPMATCHINGMODE) -> ::windows::core::Result<()>;
    fn SetPhoneticAlphabet(&mut self, phoneticalphabet: PHONETICALPHABET) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISpGrammarBuilder2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpGrammarBuilder2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpGrammarBuilder2_Vtbl {
        unsafe extern "system" fn AddTextSubset<Impl: ISpGrammarBuilder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hfromstate: *mut SPSTATEHANDLE__, htostate: *mut SPSTATEHANDLE__, psz: super::super::Foundation::PWSTR, ematchmode: SPMATCHINGMODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddTextSubset(::core::mem::transmute_copy(&hfromstate), ::core::mem::transmute_copy(&htostate), ::core::mem::transmute_copy(&psz), ::core::mem::transmute_copy(&ematchmode)).into()
        }
        unsafe extern "system" fn SetPhoneticAlphabet<Impl: ISpGrammarBuilder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phoneticalphabet: PHONETICALPHABET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPhoneticAlphabet(::core::mem::transmute_copy(&phoneticalphabet)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddTextSubset: AddTextSubset::<Impl, IMPL_OFFSET>,
            SetPhoneticAlphabet: SetPhoneticAlphabet::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpGrammarBuilder2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISpLexicon_Impl: Sized {
    fn GetPronunciations(&mut self, pszword: super::super::Foundation::PWSTR, langid: u16, dwflags: u32, pwordpronunciationlist: *mut SPWORDPRONUNCIATIONLIST) -> ::windows::core::Result<()>;
    fn AddPronunciation(&mut self, pszword: super::super::Foundation::PWSTR, langid: u16, epartofspeech: SPPARTOFSPEECH, pszpronunciation: *const u16) -> ::windows::core::Result<()>;
    fn RemovePronunciation(&mut self, pszword: super::super::Foundation::PWSTR, langid: u16, epartofspeech: SPPARTOFSPEECH, pszpronunciation: *const u16) -> ::windows::core::Result<()>;
    fn GetGeneration(&mut self, pdwgeneration: *mut u32) -> ::windows::core::Result<()>;
    fn GetGenerationChange(&mut self, dwflags: u32, pdwgeneration: *mut u32, pwordlist: *mut SPWORDLIST) -> ::windows::core::Result<()>;
    fn GetWords(&mut self, dwflags: u32, pdwgeneration: *mut u32, pdwcookie: *mut u32, pwordlist: *mut SPWORDLIST) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISpLexicon_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpLexicon_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpLexicon_Vtbl {
        unsafe extern "system" fn GetPronunciations<Impl: ISpLexicon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszword: super::super::Foundation::PWSTR, langid: u16, dwflags: u32, pwordpronunciationlist: *mut SPWORDPRONUNCIATIONLIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPronunciations(::core::mem::transmute_copy(&pszword), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pwordpronunciationlist)).into()
        }
        unsafe extern "system" fn AddPronunciation<Impl: ISpLexicon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszword: super::super::Foundation::PWSTR, langid: u16, epartofspeech: SPPARTOFSPEECH, pszpronunciation: *const u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPronunciation(::core::mem::transmute_copy(&pszword), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&epartofspeech), ::core::mem::transmute_copy(&pszpronunciation)).into()
        }
        unsafe extern "system" fn RemovePronunciation<Impl: ISpLexicon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszword: super::super::Foundation::PWSTR, langid: u16, epartofspeech: SPPARTOFSPEECH, pszpronunciation: *const u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePronunciation(::core::mem::transmute_copy(&pszword), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&epartofspeech), ::core::mem::transmute_copy(&pszpronunciation)).into()
        }
        unsafe extern "system" fn GetGeneration<Impl: ISpLexicon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwgeneration: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetGeneration(::core::mem::transmute_copy(&pdwgeneration)).into()
        }
        unsafe extern "system" fn GetGenerationChange<Impl: ISpLexicon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pdwgeneration: *mut u32, pwordlist: *mut SPWORDLIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetGenerationChange(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pdwgeneration), ::core::mem::transmute_copy(&pwordlist)).into()
        }
        unsafe extern "system" fn GetWords<Impl: ISpLexicon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pdwgeneration: *mut u32, pdwcookie: *mut u32, pwordlist: *mut SPWORDLIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetWords(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pdwgeneration), ::core::mem::transmute_copy(&pdwcookie), ::core::mem::transmute_copy(&pwordlist)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetPronunciations: GetPronunciations::<Impl, IMPL_OFFSET>,
            AddPronunciation: AddPronunciation::<Impl, IMPL_OFFSET>,
            RemovePronunciation: RemovePronunciation::<Impl, IMPL_OFFSET>,
            GetGeneration: GetGeneration::<Impl, IMPL_OFFSET>,
            GetGenerationChange: GetGenerationChange::<Impl, IMPL_OFFSET>,
            GetWords: GetWords::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpLexicon as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait ISpMMSysAudio_Impl: Sized + super::super::System::Com::ISequentialStream_Impl + super::super::System::Com::IStream_Impl + ISpStreamFormat_Impl + ISpAudio_Impl {
    fn GetDeviceId(&mut self, pudeviceid: *mut u32) -> ::windows::core::Result<()>;
    fn SetDeviceId(&mut self, udeviceid: u32) -> ::windows::core::Result<()>;
    fn GetMMHandle(&mut self, phandle: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetLineId(&mut self, pulineid: *mut u32) -> ::windows::core::Result<()>;
    fn SetLineId(&mut self, ulineid: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl ISpMMSysAudio_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpMMSysAudio_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpMMSysAudio_Vtbl {
        unsafe extern "system" fn GetDeviceId<Impl: ISpMMSysAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pudeviceid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceId(::core::mem::transmute_copy(&pudeviceid)).into()
        }
        unsafe extern "system" fn SetDeviceId<Impl: ISpMMSysAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, udeviceid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDeviceId(::core::mem::transmute_copy(&udeviceid)).into()
        }
        unsafe extern "system" fn GetMMHandle<Impl: ISpMMSysAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandle: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMMHandle(::core::mem::transmute_copy(&phandle)).into()
        }
        unsafe extern "system" fn GetLineId<Impl: ISpMMSysAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulineid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLineId(::core::mem::transmute_copy(&pulineid)).into()
        }
        unsafe extern "system" fn SetLineId<Impl: ISpMMSysAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulineid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLineId(::core::mem::transmute_copy(&ulineid)).into()
        }
        Self {
            base: ISpAudio_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetDeviceId: GetDeviceId::<Impl, IMPL_OFFSET>,
            SetDeviceId: SetDeviceId::<Impl, IMPL_OFFSET>,
            GetMMHandle: GetMMHandle::<Impl, IMPL_OFFSET>,
            GetLineId: GetLineId::<Impl, IMPL_OFFSET>,
            SetLineId: SetLineId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpMMSysAudio as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISpNotifyCallback_Impl: Sized {
    fn NotifyCallback(&mut self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISpNotifyCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpNotifyCallback_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpNotifyCallback_Vtbl {
        unsafe extern "system" fn NotifyCallback<Impl: ISpNotifyCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyCallback(::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into()
        }
        Self { NotifyCallback: NotifyCallback::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpNotifyCallback as ::windows::core::Interface>::IID
    }
}
pub trait ISpNotifySink_Impl: Sized {
    fn Notify(&mut self) -> ::windows::core::Result<()>;
}
impl ISpNotifySink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpNotifySink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpNotifySink_Vtbl {
        unsafe extern "system" fn Notify<Impl: ISpNotifySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Notify().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Notify: Notify::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpNotifySink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISpNotifySource_Impl: Sized {
    fn SetNotifySink(&mut self, pnotifysink: &::core::option::Option<ISpNotifySink>) -> ::windows::core::Result<()>;
    fn SetNotifyWindowMessage(&mut self, hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<()>;
    fn SetNotifyCallbackFunction(&mut self, pfncallback: *mut SPNOTIFYCALLBACK, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<()>;
    fn SetNotifyCallbackInterface(&mut self, pspcallback: &::core::option::Option<ISpNotifyCallback>, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<()>;
    fn SetNotifyWin32Event(&mut self) -> ::windows::core::Result<()>;
    fn WaitForNotifyEvent(&mut self, dwmilliseconds: u32) -> ::windows::core::Result<()>;
    fn GetNotifyEventHandle(&mut self) -> super::super::Foundation::HANDLE;
}
#[cfg(feature = "Win32_Foundation")]
impl ISpNotifySource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpNotifySource_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpNotifySource_Vtbl {
        unsafe extern "system" fn SetNotifySink<Impl: ISpNotifySource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnotifysink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNotifySink(::core::mem::transmute(&pnotifysink)).into()
        }
        unsafe extern "system" fn SetNotifyWindowMessage<Impl: ISpNotifySource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNotifyWindowMessage(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&msg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn SetNotifyCallbackFunction<Impl: ISpNotifySource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfncallback: *mut ::windows::core::RawPtr, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNotifyCallbackFunction(::core::mem::transmute_copy(&pfncallback), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn SetNotifyCallbackInterface<Impl: ISpNotifySource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pspcallback: ::windows::core::RawPtr, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNotifyCallbackInterface(::core::mem::transmute(&pspcallback), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn SetNotifyWin32Event<Impl: ISpNotifySource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNotifyWin32Event().into()
        }
        unsafe extern "system" fn WaitForNotifyEvent<Impl: ISpNotifySource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmilliseconds: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WaitForNotifyEvent(::core::mem::transmute_copy(&dwmilliseconds)).into()
        }
        unsafe extern "system" fn GetNotifyEventHandle<Impl: ISpNotifySource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNotifyEventHandle()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetNotifySink: SetNotifySink::<Impl, IMPL_OFFSET>,
            SetNotifyWindowMessage: SetNotifyWindowMessage::<Impl, IMPL_OFFSET>,
            SetNotifyCallbackFunction: SetNotifyCallbackFunction::<Impl, IMPL_OFFSET>,
            SetNotifyCallbackInterface: SetNotifyCallbackInterface::<Impl, IMPL_OFFSET>,
            SetNotifyWin32Event: SetNotifyWin32Event::<Impl, IMPL_OFFSET>,
            WaitForNotifyEvent: WaitForNotifyEvent::<Impl, IMPL_OFFSET>,
            GetNotifyEventHandle: GetNotifyEventHandle::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpNotifySource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISpNotifyTranslator_Impl: Sized + ISpNotifySink_Impl {
    fn InitWindowMessage(&mut self, hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<()>;
    fn InitCallback(&mut self, pfncallback: *mut SPNOTIFYCALLBACK, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<()>;
    fn InitSpNotifyCallback(&mut self, pspcallback: &::core::option::Option<ISpNotifyCallback>, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::Result<()>;
    fn InitWin32Event(&mut self, hevent: super::super::Foundation::HANDLE, fclosehandleonrelease: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn Wait(&mut self, dwmilliseconds: u32) -> ::windows::core::Result<()>;
    fn GetEventHandle(&mut self) -> super::super::Foundation::HANDLE;
}
#[cfg(feature = "Win32_Foundation")]
impl ISpNotifyTranslator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpNotifyTranslator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpNotifyTranslator_Vtbl {
        unsafe extern "system" fn InitWindowMessage<Impl: ISpNotifyTranslator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitWindowMessage(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&msg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn InitCallback<Impl: ISpNotifyTranslator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfncallback: *mut ::windows::core::RawPtr, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitCallback(::core::mem::transmute_copy(&pfncallback), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn InitSpNotifyCallback<Impl: ISpNotifyTranslator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pspcallback: ::windows::core::RawPtr, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitSpNotifyCallback(::core::mem::transmute(&pspcallback), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn InitWin32Event<Impl: ISpNotifyTranslator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, fclosehandleonrelease: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitWin32Event(::core::mem::transmute_copy(&hevent), ::core::mem::transmute_copy(&fclosehandleonrelease)).into()
        }
        unsafe extern "system" fn Wait<Impl: ISpNotifyTranslator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmilliseconds: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Wait(::core::mem::transmute_copy(&dwmilliseconds)).into()
        }
        unsafe extern "system" fn GetEventHandle<Impl: ISpNotifyTranslator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetEventHandle()
        }
        Self {
            base: ISpNotifySink_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            InitWindowMessage: InitWindowMessage::<Impl, IMPL_OFFSET>,
            InitCallback: InitCallback::<Impl, IMPL_OFFSET>,
            InitSpNotifyCallback: InitSpNotifyCallback::<Impl, IMPL_OFFSET>,
            InitWin32Event: InitWin32Event::<Impl, IMPL_OFFSET>,
            Wait: Wait::<Impl, IMPL_OFFSET>,
            GetEventHandle: GetEventHandle::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpNotifyTranslator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISpObjectToken_Impl: Sized + ISpDataKey_Impl {
    fn SetId(&mut self, pszcategoryid: super::super::Foundation::PWSTR, psztokenid: super::super::Foundation::PWSTR, fcreateifnotexist: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetId(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetCategory(&mut self) -> ::windows::core::Result<ISpObjectTokenCategory>;
    fn CreateInstance(&mut self, punkouter: &::core::option::Option<::windows::core::IUnknown>, dwclscontext: u32, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetStorageFileName(&mut self, clsidcaller: *const ::windows::core::GUID, pszvaluename: super::super::Foundation::PWSTR, pszfilenamespecifier: super::super::Foundation::PWSTR, nfolder: u32) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn RemoveStorageFileName(&mut self, clsidcaller: *const ::windows::core::GUID, pszkeyname: super::super::Foundation::PWSTR, fdeletefile: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn Remove(&mut self, pclsidcaller: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn IsUISupported(&mut self, psztypeofui: super::super::Foundation::PWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32, punkobject: &::core::option::Option<::windows::core::IUnknown>, pfsupported: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn DisplayUI(&mut self, hwndparent: super::super::Foundation::HWND, psztitle: super::super::Foundation::PWSTR, psztypeofui: super::super::Foundation::PWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32, punkobject: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn MatchesAttributes(&mut self, pszattributes: super::super::Foundation::PWSTR, pfmatches: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISpObjectToken_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpObjectToken_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpObjectToken_Vtbl {
        unsafe extern "system" fn SetId<Impl: ISpObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcategoryid: super::super::Foundation::PWSTR, psztokenid: super::super::Foundation::PWSTR, fcreateifnotexist: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetId(::core::mem::transmute_copy(&pszcategoryid), ::core::mem::transmute_copy(&psztokenid), ::core::mem::transmute_copy(&fcreateifnotexist)).into()
        }
        unsafe extern "system" fn GetId<Impl: ISpObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszcomemtokenid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetId() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszcomemtokenid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCategory<Impl: ISpObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptokencategory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCategory() {
                ::core::result::Result::Ok(ok__) => {
                    *pptokencategory = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstance<Impl: ISpObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, dwclscontext: u32, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateInstance(::core::mem::transmute(&punkouter), ::core::mem::transmute_copy(&dwclscontext), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvobject)).into()
        }
        unsafe extern "system" fn GetStorageFileName<Impl: ISpObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsidcaller: *const ::windows::core::GUID, pszvaluename: super::super::Foundation::PWSTR, pszfilenamespecifier: super::super::Foundation::PWSTR, nfolder: u32, ppszfilepath: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStorageFileName(::core::mem::transmute_copy(&clsidcaller), ::core::mem::transmute_copy(&pszvaluename), ::core::mem::transmute_copy(&pszfilenamespecifier), ::core::mem::transmute_copy(&nfolder)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppszfilepath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStorageFileName<Impl: ISpObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsidcaller: *const ::windows::core::GUID, pszkeyname: super::super::Foundation::PWSTR, fdeletefile: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStorageFileName(::core::mem::transmute_copy(&clsidcaller), ::core::mem::transmute_copy(&pszkeyname), ::core::mem::transmute_copy(&fdeletefile)).into()
        }
        unsafe extern "system" fn Remove<Impl: ISpObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsidcaller: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&pclsidcaller)).into()
        }
        unsafe extern "system" fn IsUISupported<Impl: ISpObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztypeofui: super::super::Foundation::PWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32, punkobject: *mut ::core::ffi::c_void, pfsupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsUISupported(::core::mem::transmute_copy(&psztypeofui), ::core::mem::transmute_copy(&pvextradata), ::core::mem::transmute_copy(&cbextradata), ::core::mem::transmute(&punkobject), ::core::mem::transmute_copy(&pfsupported)).into()
        }
        unsafe extern "system" fn DisplayUI<Impl: ISpObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, psztitle: super::super::Foundation::PWSTR, psztypeofui: super::super::Foundation::PWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32, punkobject: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisplayUI(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&psztitle), ::core::mem::transmute_copy(&psztypeofui), ::core::mem::transmute_copy(&pvextradata), ::core::mem::transmute_copy(&cbextradata), ::core::mem::transmute(&punkobject)).into()
        }
        unsafe extern "system" fn MatchesAttributes<Impl: ISpObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszattributes: super::super::Foundation::PWSTR, pfmatches: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MatchesAttributes(::core::mem::transmute_copy(&pszattributes), ::core::mem::transmute_copy(&pfmatches)).into()
        }
        Self {
            base: ISpDataKey_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetId: SetId::<Impl, IMPL_OFFSET>,
            GetId: GetId::<Impl, IMPL_OFFSET>,
            GetCategory: GetCategory::<Impl, IMPL_OFFSET>,
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
            GetStorageFileName: GetStorageFileName::<Impl, IMPL_OFFSET>,
            RemoveStorageFileName: RemoveStorageFileName::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            IsUISupported: IsUISupported::<Impl, IMPL_OFFSET>,
            DisplayUI: DisplayUI::<Impl, IMPL_OFFSET>,
            MatchesAttributes: MatchesAttributes::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpObjectToken as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISpObjectTokenCategory_Impl: Sized + ISpDataKey_Impl {
    fn SetId(&mut self, pszcategoryid: super::super::Foundation::PWSTR, fcreateifnotexist: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetId(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetDataKey(&mut self, spdkl: SPDATAKEYLOCATION) -> ::windows::core::Result<ISpDataKey>;
    fn EnumTokens(&mut self, pzsreqattribs: super::super::Foundation::PWSTR, pszoptattribs: super::super::Foundation::PWSTR) -> ::windows::core::Result<IEnumSpObjectTokens>;
    fn SetDefaultTokenId(&mut self, psztokenid: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetDefaultTokenId(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISpObjectTokenCategory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpObjectTokenCategory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpObjectTokenCategory_Vtbl {
        unsafe extern "system" fn SetId<Impl: ISpObjectTokenCategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcategoryid: super::super::Foundation::PWSTR, fcreateifnotexist: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetId(::core::mem::transmute_copy(&pszcategoryid), ::core::mem::transmute_copy(&fcreateifnotexist)).into()
        }
        unsafe extern "system" fn GetId<Impl: ISpObjectTokenCategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszcomemcategoryid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetId() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszcomemcategoryid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataKey<Impl: ISpObjectTokenCategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, spdkl: SPDATAKEYLOCATION, ppdatakey: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDataKey(::core::mem::transmute_copy(&spdkl)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdatakey = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumTokens<Impl: ISpObjectTokenCategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pzsreqattribs: super::super::Foundation::PWSTR, pszoptattribs: super::super::Foundation::PWSTR, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumTokens(::core::mem::transmute_copy(&pzsreqattribs), ::core::mem::transmute_copy(&pszoptattribs)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultTokenId<Impl: ISpObjectTokenCategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztokenid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultTokenId(::core::mem::transmute_copy(&psztokenid)).into()
        }
        unsafe extern "system" fn GetDefaultTokenId<Impl: ISpObjectTokenCategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszcomemtokenid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultTokenId() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszcomemtokenid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ISpDataKey_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetId: SetId::<Impl, IMPL_OFFSET>,
            GetId: GetId::<Impl, IMPL_OFFSET>,
            GetDataKey: GetDataKey::<Impl, IMPL_OFFSET>,
            EnumTokens: EnumTokens::<Impl, IMPL_OFFSET>,
            SetDefaultTokenId: SetDefaultTokenId::<Impl, IMPL_OFFSET>,
            GetDefaultTokenId: GetDefaultTokenId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpObjectTokenCategory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISpObjectTokenInit_Impl: Sized + ISpDataKey_Impl + ISpObjectToken_Impl {
    fn InitFromDataKey(&mut self, pszcategoryid: super::super::Foundation::PWSTR, psztokenid: super::super::Foundation::PWSTR, pdatakey: &::core::option::Option<ISpDataKey>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISpObjectTokenInit_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpObjectTokenInit_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpObjectTokenInit_Vtbl {
        unsafe extern "system" fn InitFromDataKey<Impl: ISpObjectTokenInit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcategoryid: super::super::Foundation::PWSTR, psztokenid: super::super::Foundation::PWSTR, pdatakey: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitFromDataKey(::core::mem::transmute_copy(&pszcategoryid), ::core::mem::transmute_copy(&psztokenid), ::core::mem::transmute(&pdatakey)).into()
        }
        Self { base: ISpObjectToken_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), InitFromDataKey: InitFromDataKey::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpObjectTokenInit as ::windows::core::Interface>::IID
    }
}
pub trait ISpObjectWithToken_Impl: Sized {
    fn SetObjectToken(&mut self, ptoken: &::core::option::Option<ISpObjectToken>) -> ::windows::core::Result<()>;
    fn GetObjectToken(&mut self) -> ::windows::core::Result<ISpObjectToken>;
}
impl ISpObjectWithToken_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpObjectWithToken_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpObjectWithToken_Vtbl {
        unsafe extern "system" fn SetObjectToken<Impl: ISpObjectWithToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptoken: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetObjectToken(::core::mem::transmute(&ptoken)).into()
        }
        unsafe extern "system" fn GetObjectToken<Impl: ISpObjectWithToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptoken: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetObjectToken() {
                ::core::result::Result::Ok(ok__) => {
                    *pptoken = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetObjectToken: SetObjectToken::<Impl, IMPL_OFFSET>,
            GetObjectToken: GetObjectToken::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpObjectWithToken as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISpPhoneConverter_Impl: Sized + ISpObjectWithToken_Impl {
    fn PhoneToId(&mut self, pszphone: super::super::Foundation::PWSTR) -> ::windows::core::Result<u16>;
    fn IdToPhone(&mut self, pid: *const u16, pszphone: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISpPhoneConverter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpPhoneConverter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpPhoneConverter_Vtbl {
        unsafe extern "system" fn PhoneToId<Impl: ISpPhoneConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszphone: super::super::Foundation::PWSTR, pid: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhoneToId(::core::mem::transmute_copy(&pszphone)) {
                ::core::result::Result::Ok(ok__) => {
                    *pid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IdToPhone<Impl: ISpPhoneConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pid: *const u16, pszphone: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IdToPhone(::core::mem::transmute_copy(&pid), ::core::mem::transmute_copy(&pszphone)).into()
        }
        Self {
            base: ISpObjectWithToken_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            PhoneToId: PhoneToId::<Impl, IMPL_OFFSET>,
            IdToPhone: IdToPhone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpPhoneConverter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISpPhoneticAlphabetConverter_Impl: Sized {
    fn GetLangId(&mut self) -> ::windows::core::Result<u16>;
    fn SetLangId(&mut self, langid: u16) -> ::windows::core::Result<()>;
    fn SAPI2UPS(&mut self, pszsapiid: *const u16, pszupsid: *mut u16, cmaxlength: u32) -> ::windows::core::Result<()>;
    fn UPS2SAPI(&mut self, pszupsid: *const u16, pszsapiid: *mut u16, cmaxlength: u32) -> ::windows::core::Result<()>;
    fn GetMaxConvertLength(&mut self, csrclength: u32, bsapi2ups: super::super::Foundation::BOOL) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISpPhoneticAlphabetConverter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpPhoneticAlphabetConverter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpPhoneticAlphabetConverter_Vtbl {
        unsafe extern "system" fn GetLangId<Impl: ISpPhoneticAlphabetConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plangid: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLangId() {
                ::core::result::Result::Ok(ok__) => {
                    *plangid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLangId<Impl: ISpPhoneticAlphabetConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLangId(::core::mem::transmute_copy(&langid)).into()
        }
        unsafe extern "system" fn SAPI2UPS<Impl: ISpPhoneticAlphabetConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsapiid: *const u16, pszupsid: *mut u16, cmaxlength: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SAPI2UPS(::core::mem::transmute_copy(&pszsapiid), ::core::mem::transmute_copy(&pszupsid), ::core::mem::transmute_copy(&cmaxlength)).into()
        }
        unsafe extern "system" fn UPS2SAPI<Impl: ISpPhoneticAlphabetConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszupsid: *const u16, pszsapiid: *mut u16, cmaxlength: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UPS2SAPI(::core::mem::transmute_copy(&pszupsid), ::core::mem::transmute_copy(&pszsapiid), ::core::mem::transmute_copy(&cmaxlength)).into()
        }
        unsafe extern "system" fn GetMaxConvertLength<Impl: ISpPhoneticAlphabetConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, csrclength: u32, bsapi2ups: super::super::Foundation::BOOL, pcmaxdestlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxConvertLength(::core::mem::transmute_copy(&csrclength), ::core::mem::transmute_copy(&bsapi2ups)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcmaxdestlength = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetLangId: GetLangId::<Impl, IMPL_OFFSET>,
            SetLangId: SetLangId::<Impl, IMPL_OFFSET>,
            SAPI2UPS: SAPI2UPS::<Impl, IMPL_OFFSET>,
            UPS2SAPI: UPS2SAPI::<Impl, IMPL_OFFSET>,
            GetMaxConvertLength: GetMaxConvertLength::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpPhoneticAlphabetConverter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISpPhoneticAlphabetSelection_Impl: Sized {
    fn IsAlphabetUPS(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetAlphabetToUPS(&mut self, fforceups: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISpPhoneticAlphabetSelection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpPhoneticAlphabetSelection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpPhoneticAlphabetSelection_Vtbl {
        unsafe extern "system" fn IsAlphabetUPS<Impl: ISpPhoneticAlphabetSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisups: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAlphabetUPS() {
                ::core::result::Result::Ok(ok__) => {
                    *pfisups = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlphabetToUPS<Impl: ISpPhoneticAlphabetSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fforceups: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlphabetToUPS(::core::mem::transmute_copy(&fforceups)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            IsAlphabetUPS: IsAlphabetUPS::<Impl, IMPL_OFFSET>,
            SetAlphabetToUPS: SetAlphabetToUPS::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpPhoneticAlphabetSelection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpPhrase_Impl: Sized {
    fn GetPhrase(&mut self) -> ::windows::core::Result<*mut SPPHRASE>;
    fn GetSerializedPhrase(&mut self) -> ::windows::core::Result<*mut SPSERIALIZEDPHRASE>;
    fn GetText(&mut self, ulstart: u32, ulcount: u32, fusetextreplacements: super::super::Foundation::BOOL, ppszcomemtext: *mut super::super::Foundation::PWSTR, pbdisplayattributes: *mut u8) -> ::windows::core::Result<()>;
    fn Discard(&mut self, dwvaluetypes: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpPhrase_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpPhrase_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpPhrase_Vtbl {
        unsafe extern "system" fn GetPhrase<Impl: ISpPhrase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcomemphrase: *mut *mut SPPHRASE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPhrase() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcomemphrase = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSerializedPhrase<Impl: ISpPhrase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcomemphrase: *mut *mut SPSERIALIZEDPHRASE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSerializedPhrase() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcomemphrase = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetText<Impl: ISpPhrase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulstart: u32, ulcount: u32, fusetextreplacements: super::super::Foundation::BOOL, ppszcomemtext: *mut super::super::Foundation::PWSTR, pbdisplayattributes: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetText(::core::mem::transmute_copy(&ulstart), ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&fusetextreplacements), ::core::mem::transmute_copy(&ppszcomemtext), ::core::mem::transmute_copy(&pbdisplayattributes)).into()
        }
        unsafe extern "system" fn Discard<Impl: ISpPhrase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwvaluetypes: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Discard(::core::mem::transmute_copy(&dwvaluetypes)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetPhrase: GetPhrase::<Impl, IMPL_OFFSET>,
            GetSerializedPhrase: GetSerializedPhrase::<Impl, IMPL_OFFSET>,
            GetText: GetText::<Impl, IMPL_OFFSET>,
            Discard: Discard::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpPhrase as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpPhrase2_Impl: Sized + ISpPhrase_Impl {
    fn GetXMLResult(&mut self, ppszcomemxmlresult: *mut super::super::Foundation::PWSTR, options: SPXMLRESULTOPTIONS) -> ::windows::core::Result<()>;
    fn GetXMLErrorInfo(&mut self, psemanticerrorinfo: *mut SPSEMANTICERRORINFO) -> ::windows::core::Result<()>;
    fn GetAudio(&mut self, ulstartelement: u32, celements: u32) -> ::windows::core::Result<ISpStreamFormat>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpPhrase2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpPhrase2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpPhrase2_Vtbl {
        unsafe extern "system" fn GetXMLResult<Impl: ISpPhrase2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszcomemxmlresult: *mut super::super::Foundation::PWSTR, options: SPXMLRESULTOPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetXMLResult(::core::mem::transmute_copy(&ppszcomemxmlresult), ::core::mem::transmute_copy(&options)).into()
        }
        unsafe extern "system" fn GetXMLErrorInfo<Impl: ISpPhrase2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psemanticerrorinfo: *mut SPSEMANTICERRORINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetXMLErrorInfo(::core::mem::transmute_copy(&psemanticerrorinfo)).into()
        }
        unsafe extern "system" fn GetAudio<Impl: ISpPhrase2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulstartelement: u32, celements: u32, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAudio(::core::mem::transmute_copy(&ulstartelement), ::core::mem::transmute_copy(&celements)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ISpPhrase_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetXMLResult: GetXMLResult::<Impl, IMPL_OFFSET>,
            GetXMLErrorInfo: GetXMLErrorInfo::<Impl, IMPL_OFFSET>,
            GetAudio: GetAudio::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpPhrase2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpPhraseAlt_Impl: Sized + ISpPhrase_Impl {
    fn GetAltInfo(&mut self, ppparent: *mut ::core::option::Option<ISpPhrase>, pulstartelementinparent: *mut u32, pcelementsinparent: *mut u32, pcelementsinalt: *mut u32) -> ::windows::core::Result<()>;
    fn Commit(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpPhraseAlt_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpPhraseAlt_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpPhraseAlt_Vtbl {
        unsafe extern "system" fn GetAltInfo<Impl: ISpPhraseAlt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppparent: *mut ::windows::core::RawPtr, pulstartelementinparent: *mut u32, pcelementsinparent: *mut u32, pcelementsinalt: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAltInfo(::core::mem::transmute_copy(&ppparent), ::core::mem::transmute_copy(&pulstartelementinparent), ::core::mem::transmute_copy(&pcelementsinparent), ::core::mem::transmute_copy(&pcelementsinalt)).into()
        }
        unsafe extern "system" fn Commit<Impl: ISpPhraseAlt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Commit().into()
        }
        Self {
            base: ISpPhrase_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetAltInfo: GetAltInfo::<Impl, IMPL_OFFSET>,
            Commit: Commit::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpPhraseAlt as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISpProperties_Impl: Sized {
    fn SetPropertyNum(&mut self, pname: super::super::Foundation::PWSTR, lvalue: i32) -> ::windows::core::Result<()>;
    fn GetPropertyNum(&mut self, pname: super::super::Foundation::PWSTR, plvalue: *mut i32) -> ::windows::core::Result<()>;
    fn SetPropertyString(&mut self, pname: super::super::Foundation::PWSTR, pvalue: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetPropertyString(&mut self, pname: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISpProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpProperties_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpProperties_Vtbl {
        unsafe extern "system" fn SetPropertyNum<Impl: ISpProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: super::super::Foundation::PWSTR, lvalue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPropertyNum(::core::mem::transmute_copy(&pname), ::core::mem::transmute_copy(&lvalue)).into()
        }
        unsafe extern "system" fn GetPropertyNum<Impl: ISpProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: super::super::Foundation::PWSTR, plvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPropertyNum(::core::mem::transmute_copy(&pname), ::core::mem::transmute_copy(&plvalue)).into()
        }
        unsafe extern "system" fn SetPropertyString<Impl: ISpProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: super::super::Foundation::PWSTR, pvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPropertyString(::core::mem::transmute_copy(&pname), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn GetPropertyString<Impl: ISpProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: super::super::Foundation::PWSTR, ppcomemvalue: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertyString(::core::mem::transmute_copy(&pname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcomemvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetPropertyNum: SetPropertyNum::<Impl, IMPL_OFFSET>,
            GetPropertyNum: GetPropertyNum::<Impl, IMPL_OFFSET>,
            SetPropertyString: SetPropertyString::<Impl, IMPL_OFFSET>,
            GetPropertyString: GetPropertyString::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio"))]
pub trait ISpRecoContext_Impl: Sized + ISpNotifySource_Impl + ISpEventSource_Impl {
    fn GetRecognizer(&mut self) -> ::windows::core::Result<ISpRecognizer>;
    fn CreateGrammar(&mut self, ullgrammarid: u64) -> ::windows::core::Result<ISpRecoGrammar>;
    fn GetStatus(&mut self, pstatus: *mut SPRECOCONTEXTSTATUS) -> ::windows::core::Result<()>;
    fn GetMaxAlternates(&mut self, pcalternates: *mut u32) -> ::windows::core::Result<()>;
    fn SetMaxAlternates(&mut self, calternates: u32) -> ::windows::core::Result<()>;
    fn SetAudioOptions(&mut self, options: SPAUDIOOPTIONS, paudioformatid: *const ::windows::core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> ::windows::core::Result<()>;
    fn GetAudioOptions(&mut self, poptions: *mut SPAUDIOOPTIONS, paudioformatid: *mut ::windows::core::GUID, ppcomemwfex: *mut *mut super::Audio::WAVEFORMATEX) -> ::windows::core::Result<()>;
    fn DeserializeResult(&mut self, pserializedresult: *const SPSERIALIZEDRESULT) -> ::windows::core::Result<ISpRecoResult>;
    fn Bookmark(&mut self, options: SPBOOKMARKOPTIONS, ullstreamposition: u64, lparamevent: super::super::Foundation::LPARAM) -> ::windows::core::Result<()>;
    fn SetAdaptationData(&mut self, padaptationdata: super::super::Foundation::PWSTR, cch: u32) -> ::windows::core::Result<()>;
    fn Pause(&mut self, dwreserved: u32) -> ::windows::core::Result<()>;
    fn Resume(&mut self, dwreserved: u32) -> ::windows::core::Result<()>;
    fn SetVoice(&mut self, pvoice: &::core::option::Option<ISpVoice>, fallowformatchanges: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetVoice(&mut self) -> ::windows::core::Result<ISpVoice>;
    fn SetVoicePurgeEvent(&mut self, ulleventinterest: u64) -> ::windows::core::Result<()>;
    fn GetVoicePurgeEvent(&mut self, pulleventinterest: *mut u64) -> ::windows::core::Result<()>;
    fn SetContextState(&mut self, econtextstate: SPCONTEXTSTATE) -> ::windows::core::Result<()>;
    fn GetContextState(&mut self, pecontextstate: *mut SPCONTEXTSTATE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio"))]
impl ISpRecoContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoContext_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpRecoContext_Vtbl {
        unsafe extern "system" fn GetRecognizer<Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprecognizer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRecognizer() {
                ::core::result::Result::Ok(ok__) => {
                    *pprecognizer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGrammar<Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ullgrammarid: u64, ppgrammar: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateGrammar(::core::mem::transmute_copy(&ullgrammarid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppgrammar = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut SPRECOCONTEXTSTATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStatus(::core::mem::transmute_copy(&pstatus)).into()
        }
        unsafe extern "system" fn GetMaxAlternates<Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcalternates: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMaxAlternates(::core::mem::transmute_copy(&pcalternates)).into()
        }
        unsafe extern "system" fn SetMaxAlternates<Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, calternates: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxAlternates(::core::mem::transmute_copy(&calternates)).into()
        }
        unsafe extern "system" fn SetAudioOptions<Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: SPAUDIOOPTIONS, paudioformatid: *const ::windows::core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAudioOptions(::core::mem::transmute_copy(&options), ::core::mem::transmute_copy(&paudioformatid), ::core::mem::transmute_copy(&pwaveformatex)).into()
        }
        unsafe extern "system" fn GetAudioOptions<Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poptions: *mut SPAUDIOOPTIONS, paudioformatid: *mut ::windows::core::GUID, ppcomemwfex: *mut *mut super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAudioOptions(::core::mem::transmute_copy(&poptions), ::core::mem::transmute_copy(&paudioformatid), ::core::mem::transmute_copy(&ppcomemwfex)).into()
        }
        unsafe extern "system" fn DeserializeResult<Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pserializedresult: *const SPSERIALIZEDRESULT, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeserializeResult(::core::mem::transmute_copy(&pserializedresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Bookmark<Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: SPBOOKMARKOPTIONS, ullstreamposition: u64, lparamevent: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Bookmark(::core::mem::transmute_copy(&options), ::core::mem::transmute_copy(&ullstreamposition), ::core::mem::transmute_copy(&lparamevent)).into()
        }
        unsafe extern "system" fn SetAdaptationData<Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padaptationdata: super::super::Foundation::PWSTR, cch: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAdaptationData(::core::mem::transmute_copy(&padaptationdata), ::core::mem::transmute_copy(&cch)).into()
        }
        unsafe extern "system" fn Pause<Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Pause(::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn Resume<Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Resume(::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn SetVoice<Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvoice: ::windows::core::RawPtr, fallowformatchanges: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVoice(::core::mem::transmute(&pvoice), ::core::mem::transmute_copy(&fallowformatchanges)).into()
        }
        unsafe extern "system" fn GetVoice<Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvoice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVoice() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvoice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVoicePurgeEvent<Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulleventinterest: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVoicePurgeEvent(::core::mem::transmute_copy(&ulleventinterest)).into()
        }
        unsafe extern "system" fn GetVoicePurgeEvent<Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulleventinterest: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetVoicePurgeEvent(::core::mem::transmute_copy(&pulleventinterest)).into()
        }
        unsafe extern "system" fn SetContextState<Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, econtextstate: SPCONTEXTSTATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContextState(::core::mem::transmute_copy(&econtextstate)).into()
        }
        unsafe extern "system" fn GetContextState<Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pecontextstate: *mut SPCONTEXTSTATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetContextState(::core::mem::transmute_copy(&pecontextstate)).into()
        }
        Self {
            base: ISpEventSource_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetRecognizer: GetRecognizer::<Impl, IMPL_OFFSET>,
            CreateGrammar: CreateGrammar::<Impl, IMPL_OFFSET>,
            GetStatus: GetStatus::<Impl, IMPL_OFFSET>,
            GetMaxAlternates: GetMaxAlternates::<Impl, IMPL_OFFSET>,
            SetMaxAlternates: SetMaxAlternates::<Impl, IMPL_OFFSET>,
            SetAudioOptions: SetAudioOptions::<Impl, IMPL_OFFSET>,
            GetAudioOptions: GetAudioOptions::<Impl, IMPL_OFFSET>,
            DeserializeResult: DeserializeResult::<Impl, IMPL_OFFSET>,
            Bookmark: Bookmark::<Impl, IMPL_OFFSET>,
            SetAdaptationData: SetAdaptationData::<Impl, IMPL_OFFSET>,
            Pause: Pause::<Impl, IMPL_OFFSET>,
            Resume: Resume::<Impl, IMPL_OFFSET>,
            SetVoice: SetVoice::<Impl, IMPL_OFFSET>,
            GetVoice: GetVoice::<Impl, IMPL_OFFSET>,
            SetVoicePurgeEvent: SetVoicePurgeEvent::<Impl, IMPL_OFFSET>,
            GetVoicePurgeEvent: GetVoicePurgeEvent::<Impl, IMPL_OFFSET>,
            SetContextState: SetContextState::<Impl, IMPL_OFFSET>,
            GetContextState: GetContextState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpRecoContext as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISpRecoContext2_Impl: Sized {
    fn SetGrammarOptions(&mut self, egrammaroptions: u32) -> ::windows::core::Result<()>;
    fn GetGrammarOptions(&mut self, pegrammaroptions: *mut u32) -> ::windows::core::Result<()>;
    fn SetAdaptationData2(&mut self, padaptationdata: super::super::Foundation::PWSTR, cch: u32, ptopicname: super::super::Foundation::PWSTR, eadaptationsettings: u32, erelevance: SPADAPTATIONRELEVANCE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISpRecoContext2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoContext2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpRecoContext2_Vtbl {
        unsafe extern "system" fn SetGrammarOptions<Impl: ISpRecoContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, egrammaroptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGrammarOptions(::core::mem::transmute_copy(&egrammaroptions)).into()
        }
        unsafe extern "system" fn GetGrammarOptions<Impl: ISpRecoContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pegrammaroptions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetGrammarOptions(::core::mem::transmute_copy(&pegrammaroptions)).into()
        }
        unsafe extern "system" fn SetAdaptationData2<Impl: ISpRecoContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padaptationdata: super::super::Foundation::PWSTR, cch: u32, ptopicname: super::super::Foundation::PWSTR, eadaptationsettings: u32, erelevance: SPADAPTATIONRELEVANCE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAdaptationData2(::core::mem::transmute_copy(&padaptationdata), ::core::mem::transmute_copy(&cch), ::core::mem::transmute_copy(&ptopicname), ::core::mem::transmute_copy(&eadaptationsettings), ::core::mem::transmute_copy(&erelevance)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetGrammarOptions: SetGrammarOptions::<Impl, IMPL_OFFSET>,
            GetGrammarOptions: GetGrammarOptions::<Impl, IMPL_OFFSET>,
            SetAdaptationData2: SetAdaptationData2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpRecoContext2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpRecoGrammar_Impl: Sized + ISpGrammarBuilder_Impl {
    fn GetGrammarId(&mut self, pullgrammarid: *mut u64) -> ::windows::core::Result<()>;
    fn GetRecoContext(&mut self) -> ::windows::core::Result<ISpRecoContext>;
    fn LoadCmdFromFile(&mut self, pszfilename: super::super::Foundation::PWSTR, options: SPLOADOPTIONS) -> ::windows::core::Result<()>;
    fn LoadCmdFromObject(&mut self, rcid: *const ::windows::core::GUID, pszgrammarname: super::super::Foundation::PWSTR, options: SPLOADOPTIONS) -> ::windows::core::Result<()>;
    fn LoadCmdFromResource(&mut self, hmodule: super::super::Foundation::HINSTANCE, pszresourcename: super::super::Foundation::PWSTR, pszresourcetype: super::super::Foundation::PWSTR, wlanguage: u16, options: SPLOADOPTIONS) -> ::windows::core::Result<()>;
    fn LoadCmdFromMemory(&mut self, pgrammar: *const SPBINARYGRAMMAR, options: SPLOADOPTIONS) -> ::windows::core::Result<()>;
    fn LoadCmdFromProprietaryGrammar(&mut self, rguidparam: *const ::windows::core::GUID, pszstringparam: super::super::Foundation::PWSTR, pvdataprarm: *const ::core::ffi::c_void, cbdatasize: u32, options: SPLOADOPTIONS) -> ::windows::core::Result<()>;
    fn SetRuleState(&mut self, pszname: super::super::Foundation::PWSTR, preserved: *mut ::core::ffi::c_void, newstate: SPRULESTATE) -> ::windows::core::Result<()>;
    fn SetRuleIdState(&mut self, ulruleid: u32, newstate: SPRULESTATE) -> ::windows::core::Result<()>;
    fn LoadDictation(&mut self, psztopicname: super::super::Foundation::PWSTR, options: SPLOADOPTIONS) -> ::windows::core::Result<()>;
    fn UnloadDictation(&mut self) -> ::windows::core::Result<()>;
    fn SetDictationState(&mut self, newstate: SPRULESTATE) -> ::windows::core::Result<()>;
    fn SetWordSequenceData(&mut self, ptext: super::super::Foundation::PWSTR, cchtext: u32, pinfo: *const SPTEXTSELECTIONINFO) -> ::windows::core::Result<()>;
    fn SetTextSelection(&mut self, pinfo: *const SPTEXTSELECTIONINFO) -> ::windows::core::Result<()>;
    fn IsPronounceable(&mut self, pszword: super::super::Foundation::PWSTR, pwordpronounceable: *mut SPWORDPRONOUNCEABLE) -> ::windows::core::Result<()>;
    fn SetGrammarState(&mut self, egrammarstate: SPGRAMMARSTATE) -> ::windows::core::Result<()>;
    fn SaveCmd(&mut self, pstream: &::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetGrammarState(&mut self, pegrammarstate: *mut SPGRAMMARSTATE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpRecoGrammar_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoGrammar_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpRecoGrammar_Vtbl {
        unsafe extern "system" fn GetGrammarId<Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pullgrammarid: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetGrammarId(::core::mem::transmute_copy(&pullgrammarid)).into()
        }
        unsafe extern "system" fn GetRecoContext<Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprecoctxt: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRecoContext() {
                ::core::result::Result::Ok(ok__) => {
                    *pprecoctxt = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadCmdFromFile<Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfilename: super::super::Foundation::PWSTR, options: SPLOADOPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LoadCmdFromFile(::core::mem::transmute_copy(&pszfilename), ::core::mem::transmute_copy(&options)).into()
        }
        unsafe extern "system" fn LoadCmdFromObject<Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rcid: *const ::windows::core::GUID, pszgrammarname: super::super::Foundation::PWSTR, options: SPLOADOPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LoadCmdFromObject(::core::mem::transmute_copy(&rcid), ::core::mem::transmute_copy(&pszgrammarname), ::core::mem::transmute_copy(&options)).into()
        }
        unsafe extern "system" fn LoadCmdFromResource<Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmodule: super::super::Foundation::HINSTANCE, pszresourcename: super::super::Foundation::PWSTR, pszresourcetype: super::super::Foundation::PWSTR, wlanguage: u16, options: SPLOADOPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LoadCmdFromResource(::core::mem::transmute_copy(&hmodule), ::core::mem::transmute_copy(&pszresourcename), ::core::mem::transmute_copy(&pszresourcetype), ::core::mem::transmute_copy(&wlanguage), ::core::mem::transmute_copy(&options)).into()
        }
        unsafe extern "system" fn LoadCmdFromMemory<Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgrammar: *const SPBINARYGRAMMAR, options: SPLOADOPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LoadCmdFromMemory(::core::mem::transmute_copy(&pgrammar), ::core::mem::transmute_copy(&options)).into()
        }
        unsafe extern "system" fn LoadCmdFromProprietaryGrammar<Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidparam: *const ::windows::core::GUID, pszstringparam: super::super::Foundation::PWSTR, pvdataprarm: *const ::core::ffi::c_void, cbdatasize: u32, options: SPLOADOPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LoadCmdFromProprietaryGrammar(::core::mem::transmute_copy(&rguidparam), ::core::mem::transmute_copy(&pszstringparam), ::core::mem::transmute_copy(&pvdataprarm), ::core::mem::transmute_copy(&cbdatasize), ::core::mem::transmute_copy(&options)).into()
        }
        unsafe extern "system" fn SetRuleState<Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, preserved: *mut ::core::ffi::c_void, newstate: SPRULESTATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRuleState(::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&preserved), ::core::mem::transmute_copy(&newstate)).into()
        }
        unsafe extern "system" fn SetRuleIdState<Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulruleid: u32, newstate: SPRULESTATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRuleIdState(::core::mem::transmute_copy(&ulruleid), ::core::mem::transmute_copy(&newstate)).into()
        }
        unsafe extern "system" fn LoadDictation<Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztopicname: super::super::Foundation::PWSTR, options: SPLOADOPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LoadDictation(::core::mem::transmute_copy(&psztopicname), ::core::mem::transmute_copy(&options)).into()
        }
        unsafe extern "system" fn UnloadDictation<Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnloadDictation().into()
        }
        unsafe extern "system" fn SetDictationState<Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newstate: SPRULESTATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDictationState(::core::mem::transmute_copy(&newstate)).into()
        }
        unsafe extern "system" fn SetWordSequenceData<Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptext: super::super::Foundation::PWSTR, cchtext: u32, pinfo: *const SPTEXTSELECTIONINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWordSequenceData(::core::mem::transmute_copy(&ptext), ::core::mem::transmute_copy(&cchtext), ::core::mem::transmute_copy(&pinfo)).into()
        }
        unsafe extern "system" fn SetTextSelection<Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const SPTEXTSELECTIONINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTextSelection(::core::mem::transmute_copy(&pinfo)).into()
        }
        unsafe extern "system" fn IsPronounceable<Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszword: super::super::Foundation::PWSTR, pwordpronounceable: *mut SPWORDPRONOUNCEABLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsPronounceable(::core::mem::transmute_copy(&pszword), ::core::mem::transmute_copy(&pwordpronounceable)).into()
        }
        unsafe extern "system" fn SetGrammarState<Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, egrammarstate: SPGRAMMARSTATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGrammarState(::core::mem::transmute_copy(&egrammarstate)).into()
        }
        unsafe extern "system" fn SaveCmd<Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, ppszcomemerrortext: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveCmd(::core::mem::transmute(&pstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppszcomemerrortext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGrammarState<Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pegrammarstate: *mut SPGRAMMARSTATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetGrammarState(::core::mem::transmute_copy(&pegrammarstate)).into()
        }
        Self {
            base: ISpGrammarBuilder_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetGrammarId: GetGrammarId::<Impl, IMPL_OFFSET>,
            GetRecoContext: GetRecoContext::<Impl, IMPL_OFFSET>,
            LoadCmdFromFile: LoadCmdFromFile::<Impl, IMPL_OFFSET>,
            LoadCmdFromObject: LoadCmdFromObject::<Impl, IMPL_OFFSET>,
            LoadCmdFromResource: LoadCmdFromResource::<Impl, IMPL_OFFSET>,
            LoadCmdFromMemory: LoadCmdFromMemory::<Impl, IMPL_OFFSET>,
            LoadCmdFromProprietaryGrammar: LoadCmdFromProprietaryGrammar::<Impl, IMPL_OFFSET>,
            SetRuleState: SetRuleState::<Impl, IMPL_OFFSET>,
            SetRuleIdState: SetRuleIdState::<Impl, IMPL_OFFSET>,
            LoadDictation: LoadDictation::<Impl, IMPL_OFFSET>,
            UnloadDictation: UnloadDictation::<Impl, IMPL_OFFSET>,
            SetDictationState: SetDictationState::<Impl, IMPL_OFFSET>,
            SetWordSequenceData: SetWordSequenceData::<Impl, IMPL_OFFSET>,
            SetTextSelection: SetTextSelection::<Impl, IMPL_OFFSET>,
            IsPronounceable: IsPronounceable::<Impl, IMPL_OFFSET>,
            SetGrammarState: SetGrammarState::<Impl, IMPL_OFFSET>,
            SaveCmd: SaveCmd::<Impl, IMPL_OFFSET>,
            GetGrammarState: GetGrammarState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpRecoGrammar as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_Urlmon"))]
pub trait ISpRecoGrammar2_Impl: Sized {
    fn GetRules(&mut self, ppcomemrules: *mut *mut SPRULE, punumrules: *mut u32) -> ::windows::core::Result<()>;
    fn LoadCmdFromFile2(&mut self, pszfilename: super::super::Foundation::PWSTR, options: SPLOADOPTIONS, pszsharinguri: super::super::Foundation::PWSTR, pszbaseuri: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn LoadCmdFromMemory2(&mut self, pgrammar: *const SPBINARYGRAMMAR, options: SPLOADOPTIONS, pszsharinguri: super::super::Foundation::PWSTR, pszbaseuri: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetRulePriority(&mut self, pszrulename: super::super::Foundation::PWSTR, ulruleid: u32, nrulepriority: i32) -> ::windows::core::Result<()>;
    fn SetRuleWeight(&mut self, pszrulename: super::super::Foundation::PWSTR, ulruleid: u32, flweight: f32) -> ::windows::core::Result<()>;
    fn SetDictationWeight(&mut self, flweight: f32) -> ::windows::core::Result<()>;
    fn SetGrammarLoader(&mut self, ploader: &::core::option::Option<ISpeechResourceLoader>) -> ::windows::core::Result<()>;
    fn SetSMLSecurityManager(&mut self, psmlsecuritymanager: &::core::option::Option<super::super::System::Com::Urlmon::IInternetSecurityManager>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_Urlmon"))]
impl ISpRecoGrammar2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoGrammar2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpRecoGrammar2_Vtbl {
        unsafe extern "system" fn GetRules<Impl: ISpRecoGrammar2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcomemrules: *mut *mut SPRULE, punumrules: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRules(::core::mem::transmute_copy(&ppcomemrules), ::core::mem::transmute_copy(&punumrules)).into()
        }
        unsafe extern "system" fn LoadCmdFromFile2<Impl: ISpRecoGrammar2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfilename: super::super::Foundation::PWSTR, options: SPLOADOPTIONS, pszsharinguri: super::super::Foundation::PWSTR, pszbaseuri: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LoadCmdFromFile2(::core::mem::transmute_copy(&pszfilename), ::core::mem::transmute_copy(&options), ::core::mem::transmute_copy(&pszsharinguri), ::core::mem::transmute_copy(&pszbaseuri)).into()
        }
        unsafe extern "system" fn LoadCmdFromMemory2<Impl: ISpRecoGrammar2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgrammar: *const SPBINARYGRAMMAR, options: SPLOADOPTIONS, pszsharinguri: super::super::Foundation::PWSTR, pszbaseuri: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LoadCmdFromMemory2(::core::mem::transmute_copy(&pgrammar), ::core::mem::transmute_copy(&options), ::core::mem::transmute_copy(&pszsharinguri), ::core::mem::transmute_copy(&pszbaseuri)).into()
        }
        unsafe extern "system" fn SetRulePriority<Impl: ISpRecoGrammar2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszrulename: super::super::Foundation::PWSTR, ulruleid: u32, nrulepriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRulePriority(::core::mem::transmute_copy(&pszrulename), ::core::mem::transmute_copy(&ulruleid), ::core::mem::transmute_copy(&nrulepriority)).into()
        }
        unsafe extern "system" fn SetRuleWeight<Impl: ISpRecoGrammar2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszrulename: super::super::Foundation::PWSTR, ulruleid: u32, flweight: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRuleWeight(::core::mem::transmute_copy(&pszrulename), ::core::mem::transmute_copy(&ulruleid), ::core::mem::transmute_copy(&flweight)).into()
        }
        unsafe extern "system" fn SetDictationWeight<Impl: ISpRecoGrammar2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flweight: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDictationWeight(::core::mem::transmute_copy(&flweight)).into()
        }
        unsafe extern "system" fn SetGrammarLoader<Impl: ISpRecoGrammar2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ploader: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGrammarLoader(::core::mem::transmute(&ploader)).into()
        }
        unsafe extern "system" fn SetSMLSecurityManager<Impl: ISpRecoGrammar2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psmlsecuritymanager: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSMLSecurityManager(::core::mem::transmute(&psmlsecuritymanager)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetRules: GetRules::<Impl, IMPL_OFFSET>,
            LoadCmdFromFile2: LoadCmdFromFile2::<Impl, IMPL_OFFSET>,
            LoadCmdFromMemory2: LoadCmdFromMemory2::<Impl, IMPL_OFFSET>,
            SetRulePriority: SetRulePriority::<Impl, IMPL_OFFSET>,
            SetRuleWeight: SetRuleWeight::<Impl, IMPL_OFFSET>,
            SetDictationWeight: SetDictationWeight::<Impl, IMPL_OFFSET>,
            SetGrammarLoader: SetGrammarLoader::<Impl, IMPL_OFFSET>,
            SetSMLSecurityManager: SetSMLSecurityManager::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpRecoGrammar2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpRecoResult_Impl: Sized + ISpPhrase_Impl {
    fn GetResultTimes(&mut self, ptimes: *mut SPRECORESULTTIMES) -> ::windows::core::Result<()>;
    fn GetAlternates(&mut self, ulstartelement: u32, celements: u32, ulrequestcount: u32, ppphrases: *mut ::core::option::Option<ISpPhraseAlt>, pcphrasesreturned: *mut u32) -> ::windows::core::Result<()>;
    fn GetAudio(&mut self, ulstartelement: u32, celements: u32) -> ::windows::core::Result<ISpStreamFormat>;
    fn SpeakAudio(&mut self, ulstartelement: u32, celements: u32, dwflags: u32, pulstreamnumber: *mut u32) -> ::windows::core::Result<()>;
    fn Serialize(&mut self, ppcomemserializedresult: *mut *mut SPSERIALIZEDRESULT) -> ::windows::core::Result<()>;
    fn ScaleAudio(&mut self, paudioformatid: *const ::windows::core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> ::windows::core::Result<()>;
    fn GetRecoContext(&mut self) -> ::windows::core::Result<ISpRecoContext>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpRecoResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpRecoResult_Vtbl {
        unsafe extern "system" fn GetResultTimes<Impl: ISpRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptimes: *mut SPRECORESULTTIMES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetResultTimes(::core::mem::transmute_copy(&ptimes)).into()
        }
        unsafe extern "system" fn GetAlternates<Impl: ISpRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulstartelement: u32, celements: u32, ulrequestcount: u32, ppphrases: *mut ::windows::core::RawPtr, pcphrasesreturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAlternates(::core::mem::transmute_copy(&ulstartelement), ::core::mem::transmute_copy(&celements), ::core::mem::transmute_copy(&ulrequestcount), ::core::mem::transmute_copy(&ppphrases), ::core::mem::transmute_copy(&pcphrasesreturned)).into()
        }
        unsafe extern "system" fn GetAudio<Impl: ISpRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulstartelement: u32, celements: u32, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAudio(::core::mem::transmute_copy(&ulstartelement), ::core::mem::transmute_copy(&celements)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpeakAudio<Impl: ISpRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulstartelement: u32, celements: u32, dwflags: u32, pulstreamnumber: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SpeakAudio(::core::mem::transmute_copy(&ulstartelement), ::core::mem::transmute_copy(&celements), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pulstreamnumber)).into()
        }
        unsafe extern "system" fn Serialize<Impl: ISpRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcomemserializedresult: *mut *mut SPSERIALIZEDRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Serialize(::core::mem::transmute_copy(&ppcomemserializedresult)).into()
        }
        unsafe extern "system" fn ScaleAudio<Impl: ISpRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paudioformatid: *const ::windows::core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ScaleAudio(::core::mem::transmute_copy(&paudioformatid), ::core::mem::transmute_copy(&pwaveformatex)).into()
        }
        unsafe extern "system" fn GetRecoContext<Impl: ISpRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprecocontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRecoContext() {
                ::core::result::Result::Ok(ok__) => {
                    *pprecocontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ISpPhrase_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetResultTimes: GetResultTimes::<Impl, IMPL_OFFSET>,
            GetAlternates: GetAlternates::<Impl, IMPL_OFFSET>,
            GetAudio: GetAudio::<Impl, IMPL_OFFSET>,
            SpeakAudio: SpeakAudio::<Impl, IMPL_OFFSET>,
            Serialize: Serialize::<Impl, IMPL_OFFSET>,
            ScaleAudio: ScaleAudio::<Impl, IMPL_OFFSET>,
            GetRecoContext: GetRecoContext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpRecoResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpRecoResult2_Impl: Sized + ISpPhrase_Impl + ISpRecoResult_Impl {
    fn CommitAlternate(&mut self, pphrasealt: &::core::option::Option<ISpPhraseAlt>) -> ::windows::core::Result<ISpRecoResult>;
    fn CommitText(&mut self, ulstartelement: u32, celements: u32, pszcorrecteddata: super::super::Foundation::PWSTR, ecommitflags: u32) -> ::windows::core::Result<()>;
    fn SetTextFeedback(&mut self, pszfeedback: super::super::Foundation::PWSTR, fsuccessful: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpRecoResult2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecoResult2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpRecoResult2_Vtbl {
        unsafe extern "system" fn CommitAlternate<Impl: ISpRecoResult2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphrasealt: ::windows::core::RawPtr, ppnewresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommitAlternate(::core::mem::transmute(&pphrasealt)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppnewresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommitText<Impl: ISpRecoResult2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulstartelement: u32, celements: u32, pszcorrecteddata: super::super::Foundation::PWSTR, ecommitflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CommitText(::core::mem::transmute_copy(&ulstartelement), ::core::mem::transmute_copy(&celements), ::core::mem::transmute_copy(&pszcorrecteddata), ::core::mem::transmute_copy(&ecommitflags)).into()
        }
        unsafe extern "system" fn SetTextFeedback<Impl: ISpRecoResult2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfeedback: super::super::Foundation::PWSTR, fsuccessful: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTextFeedback(::core::mem::transmute_copy(&pszfeedback), ::core::mem::transmute_copy(&fsuccessful)).into()
        }
        Self {
            base: ISpRecoResult_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CommitAlternate: CommitAlternate::<Impl, IMPL_OFFSET>,
            CommitText: CommitText::<Impl, IMPL_OFFSET>,
            SetTextFeedback: SetTextFeedback::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpRecoResult2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
pub trait ISpRecognizer_Impl: Sized + ISpProperties_Impl {
    fn SetRecognizer(&mut self, precognizer: &::core::option::Option<ISpObjectToken>) -> ::windows::core::Result<()>;
    fn GetRecognizer(&mut self) -> ::windows::core::Result<ISpObjectToken>;
    fn SetInput(&mut self, punkinput: &::core::option::Option<::windows::core::IUnknown>, fallowformatchanges: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetInputObjectToken(&mut self) -> ::windows::core::Result<ISpObjectToken>;
    fn GetInputStream(&mut self) -> ::windows::core::Result<ISpStreamFormat>;
    fn CreateRecoContext(&mut self) -> ::windows::core::Result<ISpRecoContext>;
    fn GetRecoProfile(&mut self) -> ::windows::core::Result<ISpObjectToken>;
    fn SetRecoProfile(&mut self, ptoken: &::core::option::Option<ISpObjectToken>) -> ::windows::core::Result<()>;
    fn IsSharedInstance(&mut self) -> ::windows::core::Result<()>;
    fn GetRecoState(&mut self, pstate: *mut SPRECOSTATE) -> ::windows::core::Result<()>;
    fn SetRecoState(&mut self, newstate: SPRECOSTATE) -> ::windows::core::Result<()>;
    fn GetStatus(&mut self, pstatus: *mut SPRECOGNIZERSTATUS) -> ::windows::core::Result<()>;
    fn GetFormat(&mut self, waveformattype: SPWAVEFORMATTYPE, pformatid: *mut ::windows::core::GUID, ppcomemwfex: *mut *mut super::Audio::WAVEFORMATEX) -> ::windows::core::Result<()>;
    fn IsUISupported(&mut self, psztypeofui: super::super::Foundation::PWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32, pfsupported: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn DisplayUI(&mut self, hwndparent: super::super::Foundation::HWND, psztitle: super::super::Foundation::PWSTR, psztypeofui: super::super::Foundation::PWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32) -> ::windows::core::Result<()>;
    fn EmulateRecognition(&mut self, pphrase: &::core::option::Option<ISpPhrase>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
impl ISpRecognizer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecognizer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpRecognizer_Vtbl {
        unsafe extern "system" fn SetRecognizer<Impl: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, precognizer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRecognizer(::core::mem::transmute(&precognizer)).into()
        }
        unsafe extern "system" fn GetRecognizer<Impl: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprecognizer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRecognizer() {
                ::core::result::Result::Ok(ok__) => {
                    *pprecognizer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInput<Impl: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkinput: *mut ::core::ffi::c_void, fallowformatchanges: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInput(::core::mem::transmute(&punkinput), ::core::mem::transmute_copy(&fallowformatchanges)).into()
        }
        unsafe extern "system" fn GetInputObjectToken<Impl: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptoken: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputObjectToken() {
                ::core::result::Result::Ok(ok__) => {
                    *pptoken = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputStream<Impl: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputStream() {
                ::core::result::Result::Ok(ok__) => {
                    *ppstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRecoContext<Impl: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnewctxt: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRecoContext() {
                ::core::result::Result::Ok(ok__) => {
                    *ppnewctxt = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecoProfile<Impl: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptoken: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRecoProfile() {
                ::core::result::Result::Ok(ok__) => {
                    *pptoken = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRecoProfile<Impl: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptoken: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRecoProfile(::core::mem::transmute(&ptoken)).into()
        }
        unsafe extern "system" fn IsSharedInstance<Impl: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsSharedInstance().into()
        }
        unsafe extern "system" fn GetRecoState<Impl: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut SPRECOSTATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRecoState(::core::mem::transmute_copy(&pstate)).into()
        }
        unsafe extern "system" fn SetRecoState<Impl: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newstate: SPRECOSTATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRecoState(::core::mem::transmute_copy(&newstate)).into()
        }
        unsafe extern "system" fn GetStatus<Impl: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut SPRECOGNIZERSTATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStatus(::core::mem::transmute_copy(&pstatus)).into()
        }
        unsafe extern "system" fn GetFormat<Impl: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, waveformattype: SPWAVEFORMATTYPE, pformatid: *mut ::windows::core::GUID, ppcomemwfex: *mut *mut super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFormat(::core::mem::transmute_copy(&waveformattype), ::core::mem::transmute_copy(&pformatid), ::core::mem::transmute_copy(&ppcomemwfex)).into()
        }
        unsafe extern "system" fn IsUISupported<Impl: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztypeofui: super::super::Foundation::PWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32, pfsupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsUISupported(::core::mem::transmute_copy(&psztypeofui), ::core::mem::transmute_copy(&pvextradata), ::core::mem::transmute_copy(&cbextradata), ::core::mem::transmute_copy(&pfsupported)).into()
        }
        unsafe extern "system" fn DisplayUI<Impl: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, psztitle: super::super::Foundation::PWSTR, psztypeofui: super::super::Foundation::PWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisplayUI(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&psztitle), ::core::mem::transmute_copy(&psztypeofui), ::core::mem::transmute_copy(&pvextradata), ::core::mem::transmute_copy(&cbextradata)).into()
        }
        unsafe extern "system" fn EmulateRecognition<Impl: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphrase: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EmulateRecognition(::core::mem::transmute(&pphrase)).into()
        }
        Self {
            base: ISpProperties_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetRecognizer: SetRecognizer::<Impl, IMPL_OFFSET>,
            GetRecognizer: GetRecognizer::<Impl, IMPL_OFFSET>,
            SetInput: SetInput::<Impl, IMPL_OFFSET>,
            GetInputObjectToken: GetInputObjectToken::<Impl, IMPL_OFFSET>,
            GetInputStream: GetInputStream::<Impl, IMPL_OFFSET>,
            CreateRecoContext: CreateRecoContext::<Impl, IMPL_OFFSET>,
            GetRecoProfile: GetRecoProfile::<Impl, IMPL_OFFSET>,
            SetRecoProfile: SetRecoProfile::<Impl, IMPL_OFFSET>,
            IsSharedInstance: IsSharedInstance::<Impl, IMPL_OFFSET>,
            GetRecoState: GetRecoState::<Impl, IMPL_OFFSET>,
            SetRecoState: SetRecoState::<Impl, IMPL_OFFSET>,
            GetStatus: GetStatus::<Impl, IMPL_OFFSET>,
            GetFormat: GetFormat::<Impl, IMPL_OFFSET>,
            IsUISupported: IsUISupported::<Impl, IMPL_OFFSET>,
            DisplayUI: DisplayUI::<Impl, IMPL_OFFSET>,
            EmulateRecognition: EmulateRecognition::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpRecognizer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISpRecognizer2_Impl: Sized {
    fn EmulateRecognitionEx(&mut self, pphrase: &::core::option::Option<ISpPhrase>, dwcompareflags: u32) -> ::windows::core::Result<()>;
    fn SetTrainingState(&mut self, fdoingtraining: super::super::Foundation::BOOL, fadaptfromtrainingdata: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn ResetAcousticModelAdaptation(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISpRecognizer2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpRecognizer2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpRecognizer2_Vtbl {
        unsafe extern "system" fn EmulateRecognitionEx<Impl: ISpRecognizer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphrase: ::windows::core::RawPtr, dwcompareflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EmulateRecognitionEx(::core::mem::transmute(&pphrase), ::core::mem::transmute_copy(&dwcompareflags)).into()
        }
        unsafe extern "system" fn SetTrainingState<Impl: ISpRecognizer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fdoingtraining: super::super::Foundation::BOOL, fadaptfromtrainingdata: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTrainingState(::core::mem::transmute_copy(&fdoingtraining), ::core::mem::transmute_copy(&fadaptfromtrainingdata)).into()
        }
        unsafe extern "system" fn ResetAcousticModelAdaptation<Impl: ISpRecognizer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResetAcousticModelAdaptation().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            EmulateRecognitionEx: EmulateRecognitionEx::<Impl, IMPL_OFFSET>,
            SetTrainingState: SetTrainingState::<Impl, IMPL_OFFSET>,
            ResetAcousticModelAdaptation: ResetAcousticModelAdaptation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpRecognizer2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub trait ISpRegDataKey_Impl: Sized + ISpDataKey_Impl {
    fn SetKey(&mut self, hkey: super::super::System::Registry::HKEY, freadonly: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ISpRegDataKey_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpRegDataKey_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpRegDataKey_Vtbl {
        unsafe extern "system" fn SetKey<Impl: ISpRegDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkey: super::super::System::Registry::HKEY, freadonly: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKey(::core::mem::transmute_copy(&hkey), ::core::mem::transmute_copy(&freadonly)).into()
        }
        Self { base: ISpDataKey_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), SetKey: SetKey::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpRegDataKey as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ISpResourceManager_Impl: Sized + super::super::System::Com::IServiceProvider_Impl {
    fn SetObject(&mut self, guidserviceid: *const ::windows::core::GUID, punkobject: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetObject(&mut self, guidserviceid: *const ::windows::core::GUID, objectclsid: *const ::windows::core::GUID, objectiid: *const ::windows::core::GUID, freleasewhenlastexternalrefreleased: super::super::Foundation::BOOL, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ISpResourceManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpResourceManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpResourceManager_Vtbl {
        unsafe extern "system" fn SetObject<Impl: ISpResourceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidserviceid: *const ::windows::core::GUID, punkobject: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetObject(::core::mem::transmute_copy(&guidserviceid), ::core::mem::transmute(&punkobject)).into()
        }
        unsafe extern "system" fn GetObject<Impl: ISpResourceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidserviceid: *const ::windows::core::GUID, objectclsid: *const ::windows::core::GUID, objectiid: *const ::windows::core::GUID, freleasewhenlastexternalrefreleased: super::super::Foundation::BOOL, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetObject(::core::mem::transmute_copy(&guidserviceid), ::core::mem::transmute_copy(&objectclsid), ::core::mem::transmute_copy(&objectiid), ::core::mem::transmute_copy(&freleasewhenlastexternalrefreleased), ::core::mem::transmute_copy(&ppobject)).into()
        }
        Self {
            base: super::super::System::Com::IServiceProvider_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetObject: SetObject::<Impl, IMPL_OFFSET>,
            GetObject: GetObject::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpResourceManager as ::windows::core::Interface>::IID
    }
}
pub trait ISpSerializeState_Impl: Sized {
    fn GetSerializedState(&mut self, ppbdata: *mut *mut u8, pulsize: *mut u32, dwreserved: u32) -> ::windows::core::Result<()>;
    fn SetSerializedState(&mut self, pbdata: *const u8, ulsize: u32, dwreserved: u32) -> ::windows::core::Result<()>;
}
impl ISpSerializeState_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpSerializeState_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpSerializeState_Vtbl {
        unsafe extern "system" fn GetSerializedState<Impl: ISpSerializeState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbdata: *mut *mut u8, pulsize: *mut u32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSerializedState(::core::mem::transmute_copy(&ppbdata), ::core::mem::transmute_copy(&pulsize), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn SetSerializedState<Impl: ISpSerializeState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdata: *const u8, ulsize: u32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSerializedState(::core::mem::transmute_copy(&pbdata), ::core::mem::transmute_copy(&ulsize), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetSerializedState: GetSerializedState::<Impl, IMPL_OFFSET>,
            SetSerializedState: SetSerializedState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpSerializeState as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISpShortcut_Impl: Sized {
    fn AddShortcut(&mut self, pszdisplay: super::super::Foundation::PWSTR, langid: u16, pszspoken: super::super::Foundation::PWSTR, shtype: SPSHORTCUTTYPE) -> ::windows::core::Result<()>;
    fn RemoveShortcut(&mut self, pszdisplay: super::super::Foundation::PWSTR, langid: u16, pszspoken: super::super::Foundation::PWSTR, shtype: SPSHORTCUTTYPE) -> ::windows::core::Result<()>;
    fn GetShortcuts(&mut self, langid: u16, pshortcutpairlist: *mut SPSHORTCUTPAIRLIST) -> ::windows::core::Result<()>;
    fn GetGeneration(&mut self) -> ::windows::core::Result<u32>;
    fn GetWordsFromGenerationChange(&mut self, pdwgeneration: *mut u32, pwordlist: *mut SPWORDLIST) -> ::windows::core::Result<()>;
    fn GetWords(&mut self, pdwgeneration: *mut u32, pdwcookie: *mut u32, pwordlist: *mut SPWORDLIST) -> ::windows::core::Result<()>;
    fn GetShortcutsForGeneration(&mut self, pdwgeneration: *mut u32, pdwcookie: *mut u32, pshortcutpairlist: *mut SPSHORTCUTPAIRLIST) -> ::windows::core::Result<()>;
    fn GetGenerationChange(&mut self, pdwgeneration: *mut u32, pshortcutpairlist: *mut SPSHORTCUTPAIRLIST) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISpShortcut_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpShortcut_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpShortcut_Vtbl {
        unsafe extern "system" fn AddShortcut<Impl: ISpShortcut_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdisplay: super::super::Foundation::PWSTR, langid: u16, pszspoken: super::super::Foundation::PWSTR, shtype: SPSHORTCUTTYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddShortcut(::core::mem::transmute_copy(&pszdisplay), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&pszspoken), ::core::mem::transmute_copy(&shtype)).into()
        }
        unsafe extern "system" fn RemoveShortcut<Impl: ISpShortcut_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdisplay: super::super::Foundation::PWSTR, langid: u16, pszspoken: super::super::Foundation::PWSTR, shtype: SPSHORTCUTTYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveShortcut(::core::mem::transmute_copy(&pszdisplay), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&pszspoken), ::core::mem::transmute_copy(&shtype)).into()
        }
        unsafe extern "system" fn GetShortcuts<Impl: ISpShortcut_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16, pshortcutpairlist: *mut SPSHORTCUTPAIRLIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetShortcuts(::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&pshortcutpairlist)).into()
        }
        unsafe extern "system" fn GetGeneration<Impl: ISpShortcut_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwgeneration: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGeneration() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwgeneration = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWordsFromGenerationChange<Impl: ISpShortcut_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwgeneration: *mut u32, pwordlist: *mut SPWORDLIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetWordsFromGenerationChange(::core::mem::transmute_copy(&pdwgeneration), ::core::mem::transmute_copy(&pwordlist)).into()
        }
        unsafe extern "system" fn GetWords<Impl: ISpShortcut_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwgeneration: *mut u32, pdwcookie: *mut u32, pwordlist: *mut SPWORDLIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetWords(::core::mem::transmute_copy(&pdwgeneration), ::core::mem::transmute_copy(&pdwcookie), ::core::mem::transmute_copy(&pwordlist)).into()
        }
        unsafe extern "system" fn GetShortcutsForGeneration<Impl: ISpShortcut_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwgeneration: *mut u32, pdwcookie: *mut u32, pshortcutpairlist: *mut SPSHORTCUTPAIRLIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetShortcutsForGeneration(::core::mem::transmute_copy(&pdwgeneration), ::core::mem::transmute_copy(&pdwcookie), ::core::mem::transmute_copy(&pshortcutpairlist)).into()
        }
        unsafe extern "system" fn GetGenerationChange<Impl: ISpShortcut_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwgeneration: *mut u32, pshortcutpairlist: *mut SPSHORTCUTPAIRLIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetGenerationChange(::core::mem::transmute_copy(&pdwgeneration), ::core::mem::transmute_copy(&pshortcutpairlist)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddShortcut: AddShortcut::<Impl, IMPL_OFFSET>,
            RemoveShortcut: RemoveShortcut::<Impl, IMPL_OFFSET>,
            GetShortcuts: GetShortcuts::<Impl, IMPL_OFFSET>,
            GetGeneration: GetGeneration::<Impl, IMPL_OFFSET>,
            GetWordsFromGenerationChange: GetWordsFromGenerationChange::<Impl, IMPL_OFFSET>,
            GetWords: GetWords::<Impl, IMPL_OFFSET>,
            GetShortcutsForGeneration: GetShortcutsForGeneration::<Impl, IMPL_OFFSET>,
            GetGenerationChange: GetGenerationChange::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpShortcut as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait ISpStream_Impl: Sized + super::super::System::Com::ISequentialStream_Impl + super::super::System::Com::IStream_Impl + ISpStreamFormat_Impl {
    fn SetBaseStream(&mut self, pstream: &::core::option::Option<super::super::System::Com::IStream>, rguidformat: *const ::windows::core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> ::windows::core::Result<()>;
    fn GetBaseStream(&mut self) -> ::windows::core::Result<super::super::System::Com::IStream>;
    fn BindToFile(&mut self, pszfilename: super::super::Foundation::PWSTR, emode: SPFILEMODE, pformatid: *const ::windows::core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX, ulleventinterest: u64) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl ISpStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpStream_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpStream_Vtbl {
        unsafe extern "system" fn SetBaseStream<Impl: ISpStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, rguidformat: *const ::windows::core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBaseStream(::core::mem::transmute(&pstream), ::core::mem::transmute_copy(&rguidformat), ::core::mem::transmute_copy(&pwaveformatex)).into()
        }
        unsafe extern "system" fn GetBaseStream<Impl: ISpStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBaseStream() {
                ::core::result::Result::Ok(ok__) => {
                    *ppstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BindToFile<Impl: ISpStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfilename: super::super::Foundation::PWSTR, emode: SPFILEMODE, pformatid: *const ::windows::core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX, ulleventinterest: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BindToFile(::core::mem::transmute_copy(&pszfilename), ::core::mem::transmute_copy(&emode), ::core::mem::transmute_copy(&pformatid), ::core::mem::transmute_copy(&pwaveformatex), ::core::mem::transmute_copy(&ulleventinterest)).into()
        }
        unsafe extern "system" fn Close<Impl: ISpStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        Self {
            base: ISpStreamFormat_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetBaseStream: SetBaseStream::<Impl, IMPL_OFFSET>,
            GetBaseStream: GetBaseStream::<Impl, IMPL_OFFSET>,
            BindToFile: BindToFile::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpStream as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait ISpStreamFormat_Impl: Sized + super::super::System::Com::ISequentialStream_Impl + super::super::System::Com::IStream_Impl {
    fn GetFormat(&mut self, pguidformatid: *const ::windows::core::GUID) -> ::windows::core::Result<*mut super::Audio::WAVEFORMATEX>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl ISpStreamFormat_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpStreamFormat_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpStreamFormat_Vtbl {
        unsafe extern "system" fn GetFormat<Impl: ISpStreamFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidformatid: *const ::windows::core::GUID, ppcomemwaveformatex: *mut *mut super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFormat(::core::mem::transmute_copy(&pguidformatid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcomemwaveformatex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: super::super::System::Com::IStream_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetFormat: GetFormat::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpStreamFormat as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait ISpStreamFormatConverter_Impl: Sized + super::super::System::Com::ISequentialStream_Impl + super::super::System::Com::IStream_Impl + ISpStreamFormat_Impl {
    fn SetBaseStream(&mut self, pstream: &::core::option::Option<ISpStreamFormat>, fsetformattobasestreamformat: super::super::Foundation::BOOL, fwritetobasestream: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetBaseStream(&mut self) -> ::windows::core::Result<ISpStreamFormat>;
    fn SetFormat(&mut self, rguidformatidofconvertedstream: *const ::windows::core::GUID, pwaveformatexofconvertedstream: *const super::Audio::WAVEFORMATEX) -> ::windows::core::Result<()>;
    fn ResetSeekPosition(&mut self) -> ::windows::core::Result<()>;
    fn ScaleConvertedToBaseOffset(&mut self, ulloffsetconvertedstream: u64) -> ::windows::core::Result<u64>;
    fn ScaleBaseToConvertedOffset(&mut self, ulloffsetbasestream: u64) -> ::windows::core::Result<u64>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl ISpStreamFormatConverter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpStreamFormatConverter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpStreamFormatConverter_Vtbl {
        unsafe extern "system" fn SetBaseStream<Impl: ISpStreamFormatConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, fsetformattobasestreamformat: super::super::Foundation::BOOL, fwritetobasestream: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBaseStream(::core::mem::transmute(&pstream), ::core::mem::transmute_copy(&fsetformattobasestreamformat), ::core::mem::transmute_copy(&fwritetobasestream)).into()
        }
        unsafe extern "system" fn GetBaseStream<Impl: ISpStreamFormatConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBaseStream() {
                ::core::result::Result::Ok(ok__) => {
                    *ppstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormat<Impl: ISpStreamFormatConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidformatidofconvertedstream: *const ::windows::core::GUID, pwaveformatexofconvertedstream: *const super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFormat(::core::mem::transmute_copy(&rguidformatidofconvertedstream), ::core::mem::transmute_copy(&pwaveformatexofconvertedstream)).into()
        }
        unsafe extern "system" fn ResetSeekPosition<Impl: ISpStreamFormatConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResetSeekPosition().into()
        }
        unsafe extern "system" fn ScaleConvertedToBaseOffset<Impl: ISpStreamFormatConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulloffsetconvertedstream: u64, pulloffsetbasestream: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScaleConvertedToBaseOffset(::core::mem::transmute_copy(&ulloffsetconvertedstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *pulloffsetbasestream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScaleBaseToConvertedOffset<Impl: ISpStreamFormatConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulloffsetbasestream: u64, pulloffsetconvertedstream: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScaleBaseToConvertedOffset(::core::mem::transmute_copy(&ulloffsetbasestream)) {
                ::core::result::Result::Ok(ok__) => {
                    *pulloffsetconvertedstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ISpStreamFormat_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetBaseStream: SetBaseStream::<Impl, IMPL_OFFSET>,
            GetBaseStream: GetBaseStream::<Impl, IMPL_OFFSET>,
            SetFormat: SetFormat::<Impl, IMPL_OFFSET>,
            ResetSeekPosition: ResetSeekPosition::<Impl, IMPL_OFFSET>,
            ScaleConvertedToBaseOffset: ScaleConvertedToBaseOffset::<Impl, IMPL_OFFSET>,
            ScaleBaseToConvertedOffset: ScaleBaseToConvertedOffset::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpStreamFormatConverter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISpTranscript_Impl: Sized {
    fn GetTranscript(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn AppendTranscript(&mut self, psztranscript: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISpTranscript_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpTranscript_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpTranscript_Vtbl {
        unsafe extern "system" fn GetTranscript<Impl: ISpTranscript_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsztranscript: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTranscript() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsztranscript = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppendTranscript<Impl: ISpTranscript_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztranscript: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AppendTranscript(::core::mem::transmute_copy(&psztranscript)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetTranscript: GetTranscript::<Impl, IMPL_OFFSET>,
            AppendTranscript: AppendTranscript::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpTranscript as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ISpVoice_Impl: Sized + ISpNotifySource_Impl + ISpEventSource_Impl {
    fn SetOutput(&mut self, punkoutput: &::core::option::Option<::windows::core::IUnknown>, fallowformatchanges: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetOutputObjectToken(&mut self) -> ::windows::core::Result<ISpObjectToken>;
    fn GetOutputStream(&mut self) -> ::windows::core::Result<ISpStreamFormat>;
    fn Pause(&mut self) -> ::windows::core::Result<()>;
    fn Resume(&mut self) -> ::windows::core::Result<()>;
    fn SetVoice(&mut self, ptoken: &::core::option::Option<ISpObjectToken>) -> ::windows::core::Result<()>;
    fn GetVoice(&mut self) -> ::windows::core::Result<ISpObjectToken>;
    fn Speak(&mut self, pwcs: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::Result<u32>;
    fn SpeakStream(&mut self, pstream: &::core::option::Option<super::super::System::Com::IStream>, dwflags: u32) -> ::windows::core::Result<u32>;
    fn GetStatus(&mut self, pstatus: *mut SPVOICESTATUS, ppszlastbookmark: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Skip(&mut self, pitemtype: super::super::Foundation::PWSTR, lnumitems: i32, pulnumskipped: *mut u32) -> ::windows::core::Result<()>;
    fn SetPriority(&mut self, epriority: SPVPRIORITY) -> ::windows::core::Result<()>;
    fn GetPriority(&mut self, pepriority: *mut SPVPRIORITY) -> ::windows::core::Result<()>;
    fn SetAlertBoundary(&mut self, eboundary: SPEVENTENUM) -> ::windows::core::Result<()>;
    fn GetAlertBoundary(&mut self, peboundary: *mut SPEVENTENUM) -> ::windows::core::Result<()>;
    fn SetRate(&mut self, rateadjust: i32) -> ::windows::core::Result<()>;
    fn GetRate(&mut self, prateadjust: *mut i32) -> ::windows::core::Result<()>;
    fn SetVolume(&mut self, usvolume: u16) -> ::windows::core::Result<()>;
    fn GetVolume(&mut self, pusvolume: *mut u16) -> ::windows::core::Result<()>;
    fn WaitUntilDone(&mut self, mstimeout: u32) -> ::windows::core::Result<()>;
    fn SetSyncSpeakTimeout(&mut self, mstimeout: u32) -> ::windows::core::Result<()>;
    fn GetSyncSpeakTimeout(&mut self, pmstimeout: *mut u32) -> ::windows::core::Result<()>;
    fn SpeakCompleteEvent(&mut self) -> super::super::Foundation::HANDLE;
    fn IsUISupported(&mut self, psztypeofui: super::super::Foundation::PWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32, pfsupported: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn DisplayUI(&mut self, hwndparent: super::super::Foundation::HWND, psztitle: super::super::Foundation::PWSTR, psztypeofui: super::super::Foundation::PWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ISpVoice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpVoice_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpVoice_Vtbl {
        unsafe extern "system" fn SetOutput<Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkoutput: *mut ::core::ffi::c_void, fallowformatchanges: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutput(::core::mem::transmute(&punkoutput), ::core::mem::transmute_copy(&fallowformatchanges)).into()
        }
        unsafe extern "system" fn GetOutputObjectToken<Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppobjecttoken: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputObjectToken() {
                ::core::result::Result::Ok(ok__) => {
                    *ppobjecttoken = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputStream<Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputStream() {
                ::core::result::Result::Ok(ok__) => {
                    *ppstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pause<Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn Resume<Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Resume().into()
        }
        unsafe extern "system" fn SetVoice<Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptoken: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVoice(::core::mem::transmute(&ptoken)).into()
        }
        unsafe extern "system" fn GetVoice<Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptoken: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVoice() {
                ::core::result::Result::Ok(ok__) => {
                    *pptoken = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Speak<Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcs: super::super::Foundation::PWSTR, dwflags: u32, pulstreamnumber: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Speak(::core::mem::transmute_copy(&pwcs), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pulstreamnumber = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpeakStream<Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, dwflags: u32, pulstreamnumber: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SpeakStream(::core::mem::transmute(&pstream), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pulstreamnumber = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut SPVOICESTATUS, ppszlastbookmark: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStatus(::core::mem::transmute_copy(&pstatus), ::core::mem::transmute_copy(&ppszlastbookmark)).into()
        }
        unsafe extern "system" fn Skip<Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitemtype: super::super::Foundation::PWSTR, lnumitems: i32, pulnumskipped: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&pitemtype), ::core::mem::transmute_copy(&lnumitems), ::core::mem::transmute_copy(&pulnumskipped)).into()
        }
        unsafe extern "system" fn SetPriority<Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, epriority: SPVPRIORITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPriority(::core::mem::transmute_copy(&epriority)).into()
        }
        unsafe extern "system" fn GetPriority<Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pepriority: *mut SPVPRIORITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPriority(::core::mem::transmute_copy(&pepriority)).into()
        }
        unsafe extern "system" fn SetAlertBoundary<Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eboundary: SPEVENTENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlertBoundary(::core::mem::transmute_copy(&eboundary)).into()
        }
        unsafe extern "system" fn GetAlertBoundary<Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peboundary: *mut SPEVENTENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAlertBoundary(::core::mem::transmute_copy(&peboundary)).into()
        }
        unsafe extern "system" fn SetRate<Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rateadjust: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRate(::core::mem::transmute_copy(&rateadjust)).into()
        }
        unsafe extern "system" fn GetRate<Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prateadjust: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRate(::core::mem::transmute_copy(&prateadjust)).into()
        }
        unsafe extern "system" fn SetVolume<Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usvolume: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVolume(::core::mem::transmute_copy(&usvolume)).into()
        }
        unsafe extern "system" fn GetVolume<Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pusvolume: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetVolume(::core::mem::transmute_copy(&pusvolume)).into()
        }
        unsafe extern "system" fn WaitUntilDone<Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mstimeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WaitUntilDone(::core::mem::transmute_copy(&mstimeout)).into()
        }
        unsafe extern "system" fn SetSyncSpeakTimeout<Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mstimeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSyncSpeakTimeout(::core::mem::transmute_copy(&mstimeout)).into()
        }
        unsafe extern "system" fn GetSyncSpeakTimeout<Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmstimeout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSyncSpeakTimeout(::core::mem::transmute_copy(&pmstimeout)).into()
        }
        unsafe extern "system" fn SpeakCompleteEvent<Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SpeakCompleteEvent()
        }
        unsafe extern "system" fn IsUISupported<Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztypeofui: super::super::Foundation::PWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32, pfsupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsUISupported(::core::mem::transmute_copy(&psztypeofui), ::core::mem::transmute_copy(&pvextradata), ::core::mem::transmute_copy(&cbextradata), ::core::mem::transmute_copy(&pfsupported)).into()
        }
        unsafe extern "system" fn DisplayUI<Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, psztitle: super::super::Foundation::PWSTR, psztypeofui: super::super::Foundation::PWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisplayUI(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&psztitle), ::core::mem::transmute_copy(&psztypeofui), ::core::mem::transmute_copy(&pvextradata), ::core::mem::transmute_copy(&cbextradata)).into()
        }
        Self {
            base: ISpEventSource_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetOutput: SetOutput::<Impl, IMPL_OFFSET>,
            GetOutputObjectToken: GetOutputObjectToken::<Impl, IMPL_OFFSET>,
            GetOutputStream: GetOutputStream::<Impl, IMPL_OFFSET>,
            Pause: Pause::<Impl, IMPL_OFFSET>,
            Resume: Resume::<Impl, IMPL_OFFSET>,
            SetVoice: SetVoice::<Impl, IMPL_OFFSET>,
            GetVoice: GetVoice::<Impl, IMPL_OFFSET>,
            Speak: Speak::<Impl, IMPL_OFFSET>,
            SpeakStream: SpeakStream::<Impl, IMPL_OFFSET>,
            GetStatus: GetStatus::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            SetPriority: SetPriority::<Impl, IMPL_OFFSET>,
            GetPriority: GetPriority::<Impl, IMPL_OFFSET>,
            SetAlertBoundary: SetAlertBoundary::<Impl, IMPL_OFFSET>,
            GetAlertBoundary: GetAlertBoundary::<Impl, IMPL_OFFSET>,
            SetRate: SetRate::<Impl, IMPL_OFFSET>,
            GetRate: GetRate::<Impl, IMPL_OFFSET>,
            SetVolume: SetVolume::<Impl, IMPL_OFFSET>,
            GetVolume: GetVolume::<Impl, IMPL_OFFSET>,
            WaitUntilDone: WaitUntilDone::<Impl, IMPL_OFFSET>,
            SetSyncSpeakTimeout: SetSyncSpeakTimeout::<Impl, IMPL_OFFSET>,
            GetSyncSpeakTimeout: GetSyncSpeakTimeout::<Impl, IMPL_OFFSET>,
            SpeakCompleteEvent: SpeakCompleteEvent::<Impl, IMPL_OFFSET>,
            IsUISupported: IsUISupported::<Impl, IMPL_OFFSET>,
            DisplayUI: DisplayUI::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpVoice as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpXMLRecoResult_Impl: Sized + ISpPhrase_Impl + ISpRecoResult_Impl {
    fn GetXMLResult(&mut self, ppszcomemxmlresult: *mut super::super::Foundation::PWSTR, options: SPXMLRESULTOPTIONS) -> ::windows::core::Result<()>;
    fn GetXMLErrorInfo(&mut self, psemanticerrorinfo: *mut SPSEMANTICERRORINFO) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpXMLRecoResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpXMLRecoResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpXMLRecoResult_Vtbl {
        unsafe extern "system" fn GetXMLResult<Impl: ISpXMLRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszcomemxmlresult: *mut super::super::Foundation::PWSTR, options: SPXMLRESULTOPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetXMLResult(::core::mem::transmute_copy(&ppszcomemxmlresult), ::core::mem::transmute_copy(&options)).into()
        }
        unsafe extern "system" fn GetXMLErrorInfo<Impl: ISpXMLRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psemanticerrorinfo: *mut SPSEMANTICERRORINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetXMLErrorInfo(::core::mem::transmute_copy(&psemanticerrorinfo)).into()
        }
        Self {
            base: ISpRecoResult_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetXMLResult: GetXMLResult::<Impl, IMPL_OFFSET>,
            GetXMLErrorInfo: GetXMLErrorInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpXMLRecoResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechAudio_Impl: Sized + super::super::System::Com::IDispatch_Impl + ISpeechBaseStream_Impl {
    fn Status(&mut self) -> ::windows::core::Result<ISpeechAudioStatus>;
    fn BufferInfo(&mut self) -> ::windows::core::Result<ISpeechAudioBufferInfo>;
    fn DefaultFormat(&mut self) -> ::windows::core::Result<ISpeechAudioFormat>;
    fn Volume(&mut self) -> ::windows::core::Result<i32>;
    fn SetVolume(&mut self, volume: i32) -> ::windows::core::Result<()>;
    fn BufferNotifySize(&mut self) -> ::windows::core::Result<i32>;
    fn SetBufferNotifySize(&mut self, buffernotifysize: i32) -> ::windows::core::Result<()>;
    fn EventHandle(&mut self) -> ::windows::core::Result<i32>;
    fn SetState(&mut self, state: SpeechAudioState) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechAudio_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechAudio_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechAudio_Vtbl {
        unsafe extern "system" fn Status<Impl: ISpeechAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *status = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BufferInfo<Impl: ISpeechAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bufferinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BufferInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *bufferinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultFormat<Impl: ISpeechAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamformat: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *streamformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Volume<Impl: ISpeechAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, volume: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Volume() {
                ::core::result::Result::Ok(ok__) => {
                    *volume = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVolume<Impl: ISpeechAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, volume: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVolume(::core::mem::transmute_copy(&volume)).into()
        }
        unsafe extern "system" fn BufferNotifySize<Impl: ISpeechAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffernotifysize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BufferNotifySize() {
                ::core::result::Result::Ok(ok__) => {
                    *buffernotifysize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferNotifySize<Impl: ISpeechAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffernotifysize: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBufferNotifySize(::core::mem::transmute_copy(&buffernotifysize)).into()
        }
        unsafe extern "system" fn EventHandle<Impl: ISpeechAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandle: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventHandle() {
                ::core::result::Result::Ok(ok__) => {
                    *eventhandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Impl: ISpeechAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: SpeechAudioState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetState(::core::mem::transmute_copy(&state)).into()
        }
        Self {
            base: ISpeechBaseStream_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            BufferInfo: BufferInfo::<Impl, IMPL_OFFSET>,
            DefaultFormat: DefaultFormat::<Impl, IMPL_OFFSET>,
            Volume: Volume::<Impl, IMPL_OFFSET>,
            SetVolume: SetVolume::<Impl, IMPL_OFFSET>,
            BufferNotifySize: BufferNotifySize::<Impl, IMPL_OFFSET>,
            SetBufferNotifySize: SetBufferNotifySize::<Impl, IMPL_OFFSET>,
            EventHandle: EventHandle::<Impl, IMPL_OFFSET>,
            SetState: SetState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechAudio as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechAudioBufferInfo_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn MinNotification(&mut self) -> ::windows::core::Result<i32>;
    fn SetMinNotification(&mut self, minnotification: i32) -> ::windows::core::Result<()>;
    fn BufferSize(&mut self) -> ::windows::core::Result<i32>;
    fn SetBufferSize(&mut self, buffersize: i32) -> ::windows::core::Result<()>;
    fn EventBias(&mut self) -> ::windows::core::Result<i32>;
    fn SetEventBias(&mut self, eventbias: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechAudioBufferInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechAudioBufferInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechAudioBufferInfo_Vtbl {
        unsafe extern "system" fn MinNotification<Impl: ISpeechAudioBufferInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minnotification: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinNotification() {
                ::core::result::Result::Ok(ok__) => {
                    *minnotification = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinNotification<Impl: ISpeechAudioBufferInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minnotification: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinNotification(::core::mem::transmute_copy(&minnotification)).into()
        }
        unsafe extern "system" fn BufferSize<Impl: ISpeechAudioBufferInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffersize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BufferSize() {
                ::core::result::Result::Ok(ok__) => {
                    *buffersize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferSize<Impl: ISpeechAudioBufferInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffersize: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBufferSize(::core::mem::transmute_copy(&buffersize)).into()
        }
        unsafe extern "system" fn EventBias<Impl: ISpeechAudioBufferInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventbias: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventBias() {
                ::core::result::Result::Ok(ok__) => {
                    *eventbias = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventBias<Impl: ISpeechAudioBufferInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventbias: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEventBias(::core::mem::transmute_copy(&eventbias)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            MinNotification: MinNotification::<Impl, IMPL_OFFSET>,
            SetMinNotification: SetMinNotification::<Impl, IMPL_OFFSET>,
            BufferSize: BufferSize::<Impl, IMPL_OFFSET>,
            SetBufferSize: SetBufferSize::<Impl, IMPL_OFFSET>,
            EventBias: EventBias::<Impl, IMPL_OFFSET>,
            SetEventBias: SetEventBias::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechAudioBufferInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechAudioFormat_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Type(&mut self) -> ::windows::core::Result<SpeechAudioFormatType>;
    fn SetType(&mut self, audioformat: SpeechAudioFormatType) -> ::windows::core::Result<()>;
    fn Guid(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetGuid(&mut self, guid: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetWaveFormatEx(&mut self) -> ::windows::core::Result<ISpeechWaveFormatEx>;
    fn SetWaveFormatEx(&mut self, speechwaveformatex: &::core::option::Option<ISpeechWaveFormatEx>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechAudioFormat_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechAudioFormat_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechAudioFormat_Vtbl {
        unsafe extern "system" fn Type<Impl: ISpeechAudioFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audioformat: *mut SpeechAudioFormatType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *audioformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Impl: ISpeechAudioFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audioformat: SpeechAudioFormatType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetType(::core::mem::transmute_copy(&audioformat)).into()
        }
        unsafe extern "system" fn Guid<Impl: ISpeechAudioFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Guid() {
                ::core::result::Result::Ok(ok__) => {
                    *guid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGuid<Impl: ISpeechAudioFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGuid(::core::mem::transmute_copy(&guid)).into()
        }
        unsafe extern "system" fn GetWaveFormatEx<Impl: ISpeechAudioFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, speechwaveformatex: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWaveFormatEx() {
                ::core::result::Result::Ok(ok__) => {
                    *speechwaveformatex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWaveFormatEx<Impl: ISpeechAudioFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, speechwaveformatex: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWaveFormatEx(::core::mem::transmute(&speechwaveformatex)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Type: Type::<Impl, IMPL_OFFSET>,
            SetType: SetType::<Impl, IMPL_OFFSET>,
            Guid: Guid::<Impl, IMPL_OFFSET>,
            SetGuid: SetGuid::<Impl, IMPL_OFFSET>,
            GetWaveFormatEx: GetWaveFormatEx::<Impl, IMPL_OFFSET>,
            SetWaveFormatEx: SetWaveFormatEx::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechAudioFormat as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechAudioStatus_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn FreeBufferSpace(&mut self) -> ::windows::core::Result<i32>;
    fn NonBlockingIO(&mut self) -> ::windows::core::Result<i32>;
    fn State(&mut self) -> ::windows::core::Result<SpeechAudioState>;
    fn CurrentSeekPosition(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn CurrentDevicePosition(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechAudioStatus_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechAudioStatus_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechAudioStatus_Vtbl {
        unsafe extern "system" fn FreeBufferSpace<Impl: ISpeechAudioStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, freebufferspace: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FreeBufferSpace() {
                ::core::result::Result::Ok(ok__) => {
                    *freebufferspace = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NonBlockingIO<Impl: ISpeechAudioStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nonblockingio: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NonBlockingIO() {
                ::core::result::Result::Ok(ok__) => {
                    *nonblockingio = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: ISpeechAudioStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: *mut SpeechAudioState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *state = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentSeekPosition<Impl: ISpeechAudioStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentseekposition: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentSeekPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *currentseekposition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentDevicePosition<Impl: ISpeechAudioStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentdeviceposition: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentDevicePosition() {
                ::core::result::Result::Ok(ok__) => {
                    *currentdeviceposition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            FreeBufferSpace: FreeBufferSpace::<Impl, IMPL_OFFSET>,
            NonBlockingIO: NonBlockingIO::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
            CurrentSeekPosition: CurrentSeekPosition::<Impl, IMPL_OFFSET>,
            CurrentDevicePosition: CurrentDevicePosition::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechAudioStatus as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechBaseStream_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Format(&mut self) -> ::windows::core::Result<ISpeechAudioFormat>;
    fn putref_Format(&mut self, audioformat: &::core::option::Option<ISpeechAudioFormat>) -> ::windows::core::Result<()>;
    fn Read(&mut self, buffer: *mut super::super::System::Com::VARIANT, numberofbytes: i32, bytesread: *mut i32) -> ::windows::core::Result<()>;
    fn Write(&mut self, buffer: &super::super::System::Com::VARIANT) -> ::windows::core::Result<i32>;
    fn Seek(&mut self, position: &super::super::System::Com::VARIANT, origin: SpeechStreamSeekPositionType) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechBaseStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechBaseStream_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechBaseStream_Vtbl {
        unsafe extern "system" fn Format<Impl: ISpeechBaseStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audioformat: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Format() {
                ::core::result::Result::Ok(ok__) => {
                    *audioformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Format<Impl: ISpeechBaseStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audioformat: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_Format(::core::mem::transmute(&audioformat)).into()
        }
        unsafe extern "system" fn Read<Impl: ISpeechBaseStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer: *mut super::super::System::Com::VARIANT, numberofbytes: i32, bytesread: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Read(::core::mem::transmute_copy(&buffer), ::core::mem::transmute_copy(&numberofbytes), ::core::mem::transmute_copy(&bytesread)).into()
        }
        unsafe extern "system" fn Write<Impl: ISpeechBaseStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, byteswritten: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Write(::core::mem::transmute_copy(&buffer)) {
                ::core::result::Result::Ok(ok__) => {
                    *byteswritten = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Seek<Impl: ISpeechBaseStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, origin: SpeechStreamSeekPositionType, newposition: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Seek(::core::mem::transmute_copy(&position), ::core::mem::transmute_copy(&origin)) {
                ::core::result::Result::Ok(ok__) => {
                    *newposition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Format: Format::<Impl, IMPL_OFFSET>,
            putref_Format: putref_Format::<Impl, IMPL_OFFSET>,
            Read: Read::<Impl, IMPL_OFFSET>,
            Write: Write::<Impl, IMPL_OFFSET>,
            Seek: Seek::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechBaseStream as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechCustomStream_Impl: Sized + super::super::System::Com::IDispatch_Impl + ISpeechBaseStream_Impl {
    fn BaseStream(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn putref_BaseStream(&mut self, punkstream: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechCustomStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechCustomStream_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechCustomStream_Vtbl {
        unsafe extern "system" fn BaseStream<Impl: ISpeechCustomStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunkstream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BaseStream() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunkstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_BaseStream<Impl: ISpeechCustomStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkstream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_BaseStream(::core::mem::transmute(&punkstream)).into()
        }
        Self {
            base: ISpeechBaseStream_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            BaseStream: BaseStream::<Impl, IMPL_OFFSET>,
            putref_BaseStream: putref_BaseStream::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechCustomStream as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechDataKey_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SetBinaryValue(&mut self, valuename: &super::super::Foundation::BSTR, value: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GetBinaryValue(&mut self, valuename: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetStringValue(&mut self, valuename: &super::super::Foundation::BSTR, value: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetStringValue(&mut self, valuename: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLongValue(&mut self, valuename: &super::super::Foundation::BSTR, value: i32) -> ::windows::core::Result<()>;
    fn GetLongValue(&mut self, valuename: &super::super::Foundation::BSTR) -> ::windows::core::Result<i32>;
    fn OpenKey(&mut self, subkeyname: &super::super::Foundation::BSTR) -> ::windows::core::Result<ISpeechDataKey>;
    fn CreateKey(&mut self, subkeyname: &super::super::Foundation::BSTR) -> ::windows::core::Result<ISpeechDataKey>;
    fn DeleteKey(&mut self, subkeyname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DeleteValue(&mut self, valuename: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn EnumKeys(&mut self, index: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn EnumValues(&mut self, index: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechDataKey_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechDataKey_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechDataKey_Vtbl {
        unsafe extern "system" fn SetBinaryValue<Impl: ISpeechDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, valuename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBinaryValue(::core::mem::transmute_copy(&valuename), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetBinaryValue<Impl: ISpeechDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, valuename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBinaryValue(::core::mem::transmute_copy(&valuename)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStringValue<Impl: ISpeechDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, valuename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStringValue(::core::mem::transmute_copy(&valuename), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetStringValue<Impl: ISpeechDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, valuename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStringValue(::core::mem::transmute_copy(&valuename)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLongValue<Impl: ISpeechDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, valuename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLongValue(::core::mem::transmute_copy(&valuename), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetLongValue<Impl: ISpeechDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, valuename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLongValue(::core::mem::transmute_copy(&valuename)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenKey<Impl: ISpeechDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subkeyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, subkey: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenKey(::core::mem::transmute_copy(&subkeyname)) {
                ::core::result::Result::Ok(ok__) => {
                    *subkey = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateKey<Impl: ISpeechDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subkeyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, subkey: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateKey(::core::mem::transmute_copy(&subkeyname)) {
                ::core::result::Result::Ok(ok__) => {
                    *subkey = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteKey<Impl: ISpeechDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subkeyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteKey(::core::mem::transmute_copy(&subkeyname)).into()
        }
        unsafe extern "system" fn DeleteValue<Impl: ISpeechDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, valuename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteValue(::core::mem::transmute_copy(&valuename)).into()
        }
        unsafe extern "system" fn EnumKeys<Impl: ISpeechDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, subkeyname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumKeys(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *subkeyname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumValues<Impl: ISpeechDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, valuename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumValues(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *valuename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetBinaryValue: SetBinaryValue::<Impl, IMPL_OFFSET>,
            GetBinaryValue: GetBinaryValue::<Impl, IMPL_OFFSET>,
            SetStringValue: SetStringValue::<Impl, IMPL_OFFSET>,
            GetStringValue: GetStringValue::<Impl, IMPL_OFFSET>,
            SetLongValue: SetLongValue::<Impl, IMPL_OFFSET>,
            GetLongValue: GetLongValue::<Impl, IMPL_OFFSET>,
            OpenKey: OpenKey::<Impl, IMPL_OFFSET>,
            CreateKey: CreateKey::<Impl, IMPL_OFFSET>,
            DeleteKey: DeleteKey::<Impl, IMPL_OFFSET>,
            DeleteValue: DeleteValue::<Impl, IMPL_OFFSET>,
            EnumKeys: EnumKeys::<Impl, IMPL_OFFSET>,
            EnumValues: EnumValues::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechDataKey as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechFileStream_Impl: Sized + super::super::System::Com::IDispatch_Impl + ISpeechBaseStream_Impl {
    fn Open(&mut self, filename: &super::super::Foundation::BSTR, filemode: SpeechStreamFileMode, doevents: i16) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechFileStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechFileStream_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechFileStream_Vtbl {
        unsafe extern "system" fn Open<Impl: ISpeechFileStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filemode: SpeechStreamFileMode, doevents: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Open(::core::mem::transmute_copy(&filename), ::core::mem::transmute_copy(&filemode), ::core::mem::transmute_copy(&doevents)).into()
        }
        unsafe extern "system" fn Close<Impl: ISpeechFileStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        Self {
            base: ISpeechBaseStream_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Open: Open::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechFileStream as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechGrammarRule_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Attributes(&mut self) -> ::windows::core::Result<SpeechRuleAttributes>;
    fn InitialState(&mut self) -> ::windows::core::Result<ISpeechGrammarRuleState>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Id(&mut self) -> ::windows::core::Result<i32>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
    fn AddResource(&mut self, resourcename: &super::super::Foundation::BSTR, resourcevalue: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AddState(&mut self) -> ::windows::core::Result<ISpeechGrammarRuleState>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechGrammarRule_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechGrammarRule_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechGrammarRule_Vtbl {
        unsafe extern "system" fn Attributes<Impl: ISpeechGrammarRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributes: *mut SpeechRuleAttributes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Attributes() {
                ::core::result::Result::Ok(ok__) => {
                    *attributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitialState<Impl: ISpeechGrammarRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitialState() {
                ::core::result::Result::Ok(ok__) => {
                    *state = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: ISpeechGrammarRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: ISpeechGrammarRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *id = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Impl: ISpeechGrammarRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn AddResource<Impl: ISpeechGrammarRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, resourcevalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddResource(::core::mem::transmute_copy(&resourcename), ::core::mem::transmute_copy(&resourcevalue)).into()
        }
        unsafe extern "system" fn AddState<Impl: ISpeechGrammarRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddState() {
                ::core::result::Result::Ok(ok__) => {
                    *state = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Attributes: Attributes::<Impl, IMPL_OFFSET>,
            InitialState: InitialState::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
            AddResource: AddResource::<Impl, IMPL_OFFSET>,
            AddState: AddState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechGrammarRule as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechGrammarRuleState_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Rule(&mut self) -> ::windows::core::Result<ISpeechGrammarRule>;
    fn Transitions(&mut self) -> ::windows::core::Result<ISpeechGrammarRuleStateTransitions>;
    fn AddWordTransition(&mut self, deststate: &::core::option::Option<ISpeechGrammarRuleState>, words: &super::super::Foundation::BSTR, separators: &super::super::Foundation::BSTR, r#type: SpeechGrammarWordType, propertyname: &super::super::Foundation::BSTR, propertyid: i32, propertyvalue: *const super::super::System::Com::VARIANT, weight: f32) -> ::windows::core::Result<()>;
    fn AddRuleTransition(&mut self, destinationstate: &::core::option::Option<ISpeechGrammarRuleState>, rule: &::core::option::Option<ISpeechGrammarRule>, propertyname: &super::super::Foundation::BSTR, propertyid: i32, propertyvalue: *const super::super::System::Com::VARIANT, weight: f32) -> ::windows::core::Result<()>;
    fn AddSpecialTransition(&mut self, destinationstate: &::core::option::Option<ISpeechGrammarRuleState>, r#type: SpeechSpecialTransitionType, propertyname: &super::super::Foundation::BSTR, propertyid: i32, propertyvalue: *const super::super::System::Com::VARIANT, weight: f32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechGrammarRuleState_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechGrammarRuleState_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechGrammarRuleState_Vtbl {
        unsafe extern "system" fn Rule<Impl: ISpeechGrammarRuleState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rule() {
                ::core::result::Result::Ok(ok__) => {
                    *rule = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Transitions<Impl: ISpeechGrammarRuleState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transitions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Transitions() {
                ::core::result::Result::Ok(ok__) => {
                    *transitions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddWordTransition<Impl: ISpeechGrammarRuleState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deststate: ::windows::core::RawPtr, words: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, separators: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, r#type: SpeechGrammarWordType, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyid: i32, propertyvalue: *const super::super::System::Com::VARIANT, weight: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddWordTransition(::core::mem::transmute(&deststate), ::core::mem::transmute_copy(&words), ::core::mem::transmute_copy(&separators), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&propertyname), ::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&propertyvalue), ::core::mem::transmute_copy(&weight)).into()
        }
        unsafe extern "system" fn AddRuleTransition<Impl: ISpeechGrammarRuleState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationstate: ::windows::core::RawPtr, rule: ::windows::core::RawPtr, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyid: i32, propertyvalue: *const super::super::System::Com::VARIANT, weight: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddRuleTransition(::core::mem::transmute(&destinationstate), ::core::mem::transmute(&rule), ::core::mem::transmute_copy(&propertyname), ::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&propertyvalue), ::core::mem::transmute_copy(&weight)).into()
        }
        unsafe extern "system" fn AddSpecialTransition<Impl: ISpeechGrammarRuleState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationstate: ::windows::core::RawPtr, r#type: SpeechSpecialTransitionType, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyid: i32, propertyvalue: *const super::super::System::Com::VARIANT, weight: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddSpecialTransition(::core::mem::transmute(&destinationstate), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&propertyname), ::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&propertyvalue), ::core::mem::transmute_copy(&weight)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Rule: Rule::<Impl, IMPL_OFFSET>,
            Transitions: Transitions::<Impl, IMPL_OFFSET>,
            AddWordTransition: AddWordTransition::<Impl, IMPL_OFFSET>,
            AddRuleTransition: AddRuleTransition::<Impl, IMPL_OFFSET>,
            AddSpecialTransition: AddSpecialTransition::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechGrammarRuleState as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechGrammarRuleStateTransition_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Type(&mut self) -> ::windows::core::Result<SpeechGrammarRuleStateTransitionType>;
    fn Text(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Rule(&mut self) -> ::windows::core::Result<ISpeechGrammarRule>;
    fn Weight(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn PropertyName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn PropertyId(&mut self) -> ::windows::core::Result<i32>;
    fn PropertyValue(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn NextState(&mut self) -> ::windows::core::Result<ISpeechGrammarRuleState>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechGrammarRuleStateTransition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechGrammarRuleStateTransition_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechGrammarRuleStateTransition_Vtbl {
        unsafe extern "system" fn Type<Impl: ISpeechGrammarRuleStateTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut SpeechGrammarRuleStateTransitionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *r#type = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Text<Impl: ISpeechGrammarRuleStateTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Text() {
                ::core::result::Result::Ok(ok__) => {
                    *text = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rule<Impl: ISpeechGrammarRuleStateTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rule() {
                ::core::result::Result::Ok(ok__) => {
                    *rule = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Weight<Impl: ISpeechGrammarRuleStateTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, weight: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Weight() {
                ::core::result::Result::Ok(ok__) => {
                    *weight = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PropertyName<Impl: ISpeechGrammarRuleStateTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PropertyName() {
                ::core::result::Result::Ok(ok__) => {
                    *propertyname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PropertyId<Impl: ISpeechGrammarRuleStateTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PropertyId() {
                ::core::result::Result::Ok(ok__) => {
                    *propertyid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PropertyValue<Impl: ISpeechGrammarRuleStateTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PropertyValue() {
                ::core::result::Result::Ok(ok__) => {
                    *propertyvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextState<Impl: ISpeechGrammarRuleStateTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nextstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NextState() {
                ::core::result::Result::Ok(ok__) => {
                    *nextstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Type: Type::<Impl, IMPL_OFFSET>,
            Text: Text::<Impl, IMPL_OFFSET>,
            Rule: Rule::<Impl, IMPL_OFFSET>,
            Weight: Weight::<Impl, IMPL_OFFSET>,
            PropertyName: PropertyName::<Impl, IMPL_OFFSET>,
            PropertyId: PropertyId::<Impl, IMPL_OFFSET>,
            PropertyValue: PropertyValue::<Impl, IMPL_OFFSET>,
            NextState: NextState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechGrammarRuleStateTransition as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechGrammarRuleStateTransitions_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Item(&mut self, index: i32) -> ::windows::core::Result<ISpeechGrammarRuleStateTransition>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechGrammarRuleStateTransitions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechGrammarRuleStateTransitions_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechGrammarRuleStateTransitions_Vtbl {
        unsafe extern "system" fn Count<Impl: ISpeechGrammarRuleStateTransitions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISpeechGrammarRuleStateTransitions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, transition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *transition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISpeechGrammarRuleStateTransitions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *enumvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechGrammarRuleStateTransitions as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechGrammarRules_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn FindRule(&mut self, rulenameorid: &super::super::System::Com::VARIANT) -> ::windows::core::Result<ISpeechGrammarRule>;
    fn Item(&mut self, index: i32) -> ::windows::core::Result<ISpeechGrammarRule>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Dynamic(&mut self) -> ::windows::core::Result<i16>;
    fn Add(&mut self, rulename: &super::super::Foundation::BSTR, attributes: SpeechRuleAttributes, ruleid: i32) -> ::windows::core::Result<ISpeechGrammarRule>;
    fn Commit(&mut self) -> ::windows::core::Result<()>;
    fn CommitAndSave(&mut self, errortext: *mut super::super::Foundation::BSTR, savestream: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechGrammarRules_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechGrammarRules_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechGrammarRules_Vtbl {
        unsafe extern "system" fn Count<Impl: ISpeechGrammarRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindRule<Impl: ISpeechGrammarRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rulenameorid: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, rule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindRule(::core::mem::transmute_copy(&rulenameorid)) {
                ::core::result::Result::Ok(ok__) => {
                    *rule = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISpeechGrammarRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, rule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *rule = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISpeechGrammarRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *enumvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Dynamic<Impl: ISpeechGrammarRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dynamic: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Dynamic() {
                ::core::result::Result::Ok(ok__) => {
                    *dynamic = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: ISpeechGrammarRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rulename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, attributes: SpeechRuleAttributes, ruleid: i32, rule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(::core::mem::transmute_copy(&rulename), ::core::mem::transmute_copy(&attributes), ::core::mem::transmute_copy(&ruleid)) {
                ::core::result::Result::Ok(ok__) => {
                    *rule = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Commit<Impl: ISpeechGrammarRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Commit().into()
        }
        unsafe extern "system" fn CommitAndSave<Impl: ISpeechGrammarRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errortext: *mut super::super::Foundation::BSTR, savestream: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CommitAndSave(::core::mem::transmute_copy(&errortext), ::core::mem::transmute_copy(&savestream)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            FindRule: FindRule::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Dynamic: Dynamic::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Commit: Commit::<Impl, IMPL_OFFSET>,
            CommitAndSave: CommitAndSave::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechGrammarRules as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechLexicon_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GenerationId(&mut self) -> ::windows::core::Result<i32>;
    fn GetWords(&mut self, flags: SpeechLexiconType, generationid: *mut i32, words: *mut ::core::option::Option<ISpeechLexiconWords>) -> ::windows::core::Result<()>;
    fn AddPronunciation(&mut self, bstrword: &super::super::Foundation::BSTR, langid: i32, partofspeech: SpeechPartOfSpeech, bstrpronunciation: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AddPronunciationByPhoneIds(&mut self, bstrword: &super::super::Foundation::BSTR, langid: i32, partofspeech: SpeechPartOfSpeech, phoneids: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn RemovePronunciation(&mut self, bstrword: &super::super::Foundation::BSTR, langid: i32, partofspeech: SpeechPartOfSpeech, bstrpronunciation: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RemovePronunciationByPhoneIds(&mut self, bstrword: &super::super::Foundation::BSTR, langid: i32, partofspeech: SpeechPartOfSpeech, phoneids: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GetPronunciations(&mut self, bstrword: &super::super::Foundation::BSTR, langid: i32, typeflags: SpeechLexiconType) -> ::windows::core::Result<ISpeechLexiconPronunciations>;
    fn GetGenerationChange(&mut self, generationid: *mut i32, ppwords: *mut ::core::option::Option<ISpeechLexiconWords>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechLexicon_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechLexicon_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechLexicon_Vtbl {
        unsafe extern "system" fn GenerationId<Impl: ISpeechLexicon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, generationid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GenerationId() {
                ::core::result::Result::Ok(ok__) => {
                    *generationid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWords<Impl: ISpeechLexicon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: SpeechLexiconType, generationid: *mut i32, words: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetWords(::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&generationid), ::core::mem::transmute_copy(&words)).into()
        }
        unsafe extern "system" fn AddPronunciation<Impl: ISpeechLexicon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, langid: i32, partofspeech: SpeechPartOfSpeech, bstrpronunciation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPronunciation(::core::mem::transmute_copy(&bstrword), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&partofspeech), ::core::mem::transmute_copy(&bstrpronunciation)).into()
        }
        unsafe extern "system" fn AddPronunciationByPhoneIds<Impl: ISpeechLexicon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, langid: i32, partofspeech: SpeechPartOfSpeech, phoneids: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPronunciationByPhoneIds(::core::mem::transmute_copy(&bstrword), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&partofspeech), ::core::mem::transmute_copy(&phoneids)).into()
        }
        unsafe extern "system" fn RemovePronunciation<Impl: ISpeechLexicon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, langid: i32, partofspeech: SpeechPartOfSpeech, bstrpronunciation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePronunciation(::core::mem::transmute_copy(&bstrword), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&partofspeech), ::core::mem::transmute_copy(&bstrpronunciation)).into()
        }
        unsafe extern "system" fn RemovePronunciationByPhoneIds<Impl: ISpeechLexicon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, langid: i32, partofspeech: SpeechPartOfSpeech, phoneids: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePronunciationByPhoneIds(::core::mem::transmute_copy(&bstrword), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&partofspeech), ::core::mem::transmute_copy(&phoneids)).into()
        }
        unsafe extern "system" fn GetPronunciations<Impl: ISpeechLexicon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, langid: i32, typeflags: SpeechLexiconType, pppronunciations: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPronunciations(::core::mem::transmute_copy(&bstrword), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&typeflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pppronunciations = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGenerationChange<Impl: ISpeechLexicon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, generationid: *mut i32, ppwords: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetGenerationChange(::core::mem::transmute_copy(&generationid), ::core::mem::transmute_copy(&ppwords)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GenerationId: GenerationId::<Impl, IMPL_OFFSET>,
            GetWords: GetWords::<Impl, IMPL_OFFSET>,
            AddPronunciation: AddPronunciation::<Impl, IMPL_OFFSET>,
            AddPronunciationByPhoneIds: AddPronunciationByPhoneIds::<Impl, IMPL_OFFSET>,
            RemovePronunciation: RemovePronunciation::<Impl, IMPL_OFFSET>,
            RemovePronunciationByPhoneIds: RemovePronunciationByPhoneIds::<Impl, IMPL_OFFSET>,
            GetPronunciations: GetPronunciations::<Impl, IMPL_OFFSET>,
            GetGenerationChange: GetGenerationChange::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechLexicon as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechLexiconPronunciation_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Type(&mut self) -> ::windows::core::Result<SpeechLexiconType>;
    fn LangId(&mut self) -> ::windows::core::Result<i32>;
    fn PartOfSpeech(&mut self) -> ::windows::core::Result<SpeechPartOfSpeech>;
    fn PhoneIds(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Symbolic(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechLexiconPronunciation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechLexiconPronunciation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechLexiconPronunciation_Vtbl {
        unsafe extern "system" fn Type<Impl: ISpeechLexiconPronunciation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lexicontype: *mut SpeechLexiconType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *lexicontype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LangId<Impl: ISpeechLexiconPronunciation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LangId() {
                ::core::result::Result::Ok(ok__) => {
                    *langid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PartOfSpeech<Impl: ISpeechLexiconPronunciation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partofspeech: *mut SpeechPartOfSpeech) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PartOfSpeech() {
                ::core::result::Result::Ok(ok__) => {
                    *partofspeech = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhoneIds<Impl: ISpeechLexiconPronunciation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phoneids: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhoneIds() {
                ::core::result::Result::Ok(ok__) => {
                    *phoneids = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Symbolic<Impl: ISpeechLexiconPronunciation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, symbolic: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Symbolic() {
                ::core::result::Result::Ok(ok__) => {
                    *symbolic = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Type: Type::<Impl, IMPL_OFFSET>,
            LangId: LangId::<Impl, IMPL_OFFSET>,
            PartOfSpeech: PartOfSpeech::<Impl, IMPL_OFFSET>,
            PhoneIds: PhoneIds::<Impl, IMPL_OFFSET>,
            Symbolic: Symbolic::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechLexiconPronunciation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechLexiconPronunciations_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Item(&mut self, index: i32) -> ::windows::core::Result<ISpeechLexiconPronunciation>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechLexiconPronunciations_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechLexiconPronunciations_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechLexiconPronunciations_Vtbl {
        unsafe extern "system" fn Count<Impl: ISpeechLexiconPronunciations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISpeechLexiconPronunciations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pronunciation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pronunciation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISpeechLexiconPronunciations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *enumvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechLexiconPronunciations as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechLexiconWord_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn LangId(&mut self) -> ::windows::core::Result<i32>;
    fn Type(&mut self) -> ::windows::core::Result<SpeechWordType>;
    fn Word(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Pronunciations(&mut self) -> ::windows::core::Result<ISpeechLexiconPronunciations>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechLexiconWord_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechLexiconWord_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechLexiconWord_Vtbl {
        unsafe extern "system" fn LangId<Impl: ISpeechLexiconWord_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LangId() {
                ::core::result::Result::Ok(ok__) => {
                    *langid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: ISpeechLexiconWord_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wordtype: *mut SpeechWordType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *wordtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Word<Impl: ISpeechLexiconWord_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, word: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Word() {
                ::core::result::Result::Ok(ok__) => {
                    *word = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pronunciations<Impl: ISpeechLexiconWord_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pronunciations: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pronunciations() {
                ::core::result::Result::Ok(ok__) => {
                    *pronunciations = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            LangId: LangId::<Impl, IMPL_OFFSET>,
            Type: Type::<Impl, IMPL_OFFSET>,
            Word: Word::<Impl, IMPL_OFFSET>,
            Pronunciations: Pronunciations::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechLexiconWord as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechLexiconWords_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Item(&mut self, index: i32) -> ::windows::core::Result<ISpeechLexiconWord>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechLexiconWords_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechLexiconWords_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechLexiconWords_Vtbl {
        unsafe extern "system" fn Count<Impl: ISpeechLexiconWords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISpeechLexiconWords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, word: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *word = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISpeechLexiconWords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *enumvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechLexiconWords as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechMMSysAudio_Impl: Sized + super::super::System::Com::IDispatch_Impl + ISpeechBaseStream_Impl + ISpeechAudio_Impl {
    fn DeviceId(&mut self) -> ::windows::core::Result<i32>;
    fn SetDeviceId(&mut self, deviceid: i32) -> ::windows::core::Result<()>;
    fn LineId(&mut self) -> ::windows::core::Result<i32>;
    fn SetLineId(&mut self, lineid: i32) -> ::windows::core::Result<()>;
    fn MMHandle(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechMMSysAudio_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechMMSysAudio_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechMMSysAudio_Vtbl {
        unsafe extern "system" fn DeviceId<Impl: ISpeechMMSysAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *deviceid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDeviceId<Impl: ISpeechMMSysAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDeviceId(::core::mem::transmute_copy(&deviceid)).into()
        }
        unsafe extern "system" fn LineId<Impl: ISpeechMMSysAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lineid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LineId() {
                ::core::result::Result::Ok(ok__) => {
                    *lineid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLineId<Impl: ISpeechMMSysAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lineid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLineId(::core::mem::transmute_copy(&lineid)).into()
        }
        unsafe extern "system" fn MMHandle<Impl: ISpeechMMSysAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handle: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MMHandle() {
                ::core::result::Result::Ok(ok__) => {
                    *handle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ISpeechAudio_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
            SetDeviceId: SetDeviceId::<Impl, IMPL_OFFSET>,
            LineId: LineId::<Impl, IMPL_OFFSET>,
            SetLineId: SetLineId::<Impl, IMPL_OFFSET>,
            MMHandle: MMHandle::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechMMSysAudio as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechMemoryStream_Impl: Sized + super::super::System::Com::IDispatch_Impl + ISpeechBaseStream_Impl {
    fn SetData(&mut self, data: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GetData(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechMemoryStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechMemoryStream_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechMemoryStream_Vtbl {
        unsafe extern "system" fn SetData<Impl: ISpeechMemoryStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetData(::core::mem::transmute_copy(&data)).into()
        }
        unsafe extern "system" fn GetData<Impl: ISpeechMemoryStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetData() {
                ::core::result::Result::Ok(ok__) => {
                    *pdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ISpeechBaseStream_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetData: SetData::<Impl, IMPL_OFFSET>,
            GetData: GetData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechMemoryStream as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechObjectToken_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Id(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DataKey(&mut self) -> ::windows::core::Result<ISpeechDataKey>;
    fn Category(&mut self) -> ::windows::core::Result<ISpeechObjectTokenCategory>;
    fn GetDescription(&mut self, locale: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetId(&mut self, id: &super::super::Foundation::BSTR, categoryid: &super::super::Foundation::BSTR, createifnotexist: i16) -> ::windows::core::Result<()>;
    fn GetAttribute(&mut self, attributename: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CreateInstance(&mut self, punkouter: &::core::option::Option<::windows::core::IUnknown>, clscontext: SpeechTokenContext) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Remove(&mut self, objectstorageclsid: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetStorageFileName(&mut self, objectstorageclsid: &super::super::Foundation::BSTR, keyname: &super::super::Foundation::BSTR, filename: &super::super::Foundation::BSTR, folder: SpeechTokenShellFolder) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn RemoveStorageFileName(&mut self, objectstorageclsid: &super::super::Foundation::BSTR, keyname: &super::super::Foundation::BSTR, deletefilea: i16) -> ::windows::core::Result<()>;
    fn IsUISupported(&mut self, typeofui: &super::super::Foundation::BSTR, extradata: *const super::super::System::Com::VARIANT, object: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<i16>;
    fn DisplayUI(&mut self, hwnd: i32, title: &super::super::Foundation::BSTR, typeofui: &super::super::Foundation::BSTR, extradata: *const super::super::System::Com::VARIANT, object: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn MatchesAttributes(&mut self, attributes: &super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechObjectToken_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechObjectToken_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechObjectToken_Vtbl {
        unsafe extern "system" fn Id<Impl: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objectid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *objectid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataKey<Impl: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datakey: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataKey() {
                ::core::result::Result::Ok(ok__) => {
                    *datakey = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Category<Impl: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Category() {
                ::core::result::Result::Ok(ok__) => {
                    *category = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescription<Impl: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, locale: i32, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDescription(::core::mem::transmute_copy(&locale)) {
                ::core::result::Result::Ok(ok__) => {
                    *description = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetId<Impl: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, categoryid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, createifnotexist: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetId(::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&categoryid), ::core::mem::transmute_copy(&createifnotexist)).into()
        }
        unsafe extern "system" fn GetAttribute<Impl: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, attributevalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAttribute(::core::mem::transmute_copy(&attributename)) {
                ::core::result::Result::Ok(ok__) => {
                    *attributevalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstance<Impl: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, clscontext: SpeechTokenContext, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(::core::mem::transmute(&punkouter), ::core::mem::transmute_copy(&clscontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *object = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objectstorageclsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&objectstorageclsid)).into()
        }
        unsafe extern "system" fn GetStorageFileName<Impl: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objectstorageclsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, keyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, folder: SpeechTokenShellFolder, filepath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStorageFileName(::core::mem::transmute_copy(&objectstorageclsid), ::core::mem::transmute_copy(&keyname), ::core::mem::transmute_copy(&filename), ::core::mem::transmute_copy(&folder)) {
                ::core::result::Result::Ok(ok__) => {
                    *filepath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStorageFileName<Impl: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objectstorageclsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, keyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, deletefilea: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStorageFileName(::core::mem::transmute_copy(&objectstorageclsid), ::core::mem::transmute_copy(&keyname), ::core::mem::transmute_copy(&deletefilea)).into()
        }
        unsafe extern "system" fn IsUISupported<Impl: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, typeofui: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, extradata: *const super::super::System::Com::VARIANT, object: *mut ::core::ffi::c_void, supported: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsUISupported(::core::mem::transmute_copy(&typeofui), ::core::mem::transmute_copy(&extradata), ::core::mem::transmute(&object)) {
                ::core::result::Result::Ok(ok__) => {
                    *supported = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayUI<Impl: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: i32, title: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, typeofui: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, extradata: *const super::super::System::Com::VARIANT, object: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisplayUI(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&title), ::core::mem::transmute_copy(&typeofui), ::core::mem::transmute_copy(&extradata), ::core::mem::transmute(&object)).into()
        }
        unsafe extern "system" fn MatchesAttributes<Impl: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, matches: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MatchesAttributes(::core::mem::transmute_copy(&attributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *matches = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            DataKey: DataKey::<Impl, IMPL_OFFSET>,
            Category: Category::<Impl, IMPL_OFFSET>,
            GetDescription: GetDescription::<Impl, IMPL_OFFSET>,
            SetId: SetId::<Impl, IMPL_OFFSET>,
            GetAttribute: GetAttribute::<Impl, IMPL_OFFSET>,
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            GetStorageFileName: GetStorageFileName::<Impl, IMPL_OFFSET>,
            RemoveStorageFileName: RemoveStorageFileName::<Impl, IMPL_OFFSET>,
            IsUISupported: IsUISupported::<Impl, IMPL_OFFSET>,
            DisplayUI: DisplayUI::<Impl, IMPL_OFFSET>,
            MatchesAttributes: MatchesAttributes::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechObjectToken as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechObjectTokenCategory_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Id(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDefault(&mut self, tokenid: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Default(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetId(&mut self, id: &super::super::Foundation::BSTR, createifnotexist: i16) -> ::windows::core::Result<()>;
    fn GetDataKey(&mut self, location: SpeechDataKeyLocation) -> ::windows::core::Result<ISpeechDataKey>;
    fn EnumerateTokens(&mut self, requiredattributes: &super::super::Foundation::BSTR, optionalattributes: &super::super::Foundation::BSTR) -> ::windows::core::Result<ISpeechObjectTokens>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechObjectTokenCategory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechObjectTokenCategory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechObjectTokenCategory_Vtbl {
        unsafe extern "system" fn Id<Impl: ISpeechObjectTokenCategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *id = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefault<Impl: ISpeechObjectTokenCategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tokenid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefault(::core::mem::transmute_copy(&tokenid)).into()
        }
        unsafe extern "system" fn Default<Impl: ISpeechObjectTokenCategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tokenid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Default() {
                ::core::result::Result::Ok(ok__) => {
                    *tokenid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetId<Impl: ISpeechObjectTokenCategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, createifnotexist: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetId(::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&createifnotexist)).into()
        }
        unsafe extern "system" fn GetDataKey<Impl: ISpeechObjectTokenCategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, location: SpeechDataKeyLocation, datakey: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDataKey(::core::mem::transmute_copy(&location)) {
                ::core::result::Result::Ok(ok__) => {
                    *datakey = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateTokens<Impl: ISpeechObjectTokenCategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requiredattributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, optionalattributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, tokens: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateTokens(::core::mem::transmute_copy(&requiredattributes), ::core::mem::transmute_copy(&optionalattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *tokens = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            SetDefault: SetDefault::<Impl, IMPL_OFFSET>,
            Default: Default::<Impl, IMPL_OFFSET>,
            SetId: SetId::<Impl, IMPL_OFFSET>,
            GetDataKey: GetDataKey::<Impl, IMPL_OFFSET>,
            EnumerateTokens: EnumerateTokens::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechObjectTokenCategory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechObjectTokens_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Item(&mut self, index: i32) -> ::windows::core::Result<ISpeechObjectToken>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechObjectTokens_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechObjectTokens_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechObjectTokens_Vtbl {
        unsafe extern "system" fn Count<Impl: ISpeechObjectTokens_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISpeechObjectTokens_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, token: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *token = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISpeechObjectTokens_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechObjectTokens as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechPhoneConverter_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn LanguageId(&mut self) -> ::windows::core::Result<i32>;
    fn SetLanguageId(&mut self, languageid: i32) -> ::windows::core::Result<()>;
    fn PhoneToId(&mut self, phonemes: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn IdToPhone(&mut self, idarray: &super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechPhoneConverter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhoneConverter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechPhoneConverter_Vtbl {
        unsafe extern "system" fn LanguageId<Impl: ISpeechPhoneConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languageid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LanguageId() {
                ::core::result::Result::Ok(ok__) => {
                    *languageid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguageId<Impl: ISpeechPhoneConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languageid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLanguageId(::core::mem::transmute_copy(&languageid)).into()
        }
        unsafe extern "system" fn PhoneToId<Impl: ISpeechPhoneConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phonemes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, idarray: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhoneToId(::core::mem::transmute_copy(&phonemes)) {
                ::core::result::Result::Ok(ok__) => {
                    *idarray = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IdToPhone<Impl: ISpeechPhoneConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idarray: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, phonemes: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IdToPhone(::core::mem::transmute_copy(&idarray)) {
                ::core::result::Result::Ok(ok__) => {
                    *phonemes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            LanguageId: LanguageId::<Impl, IMPL_OFFSET>,
            SetLanguageId: SetLanguageId::<Impl, IMPL_OFFSET>,
            PhoneToId: PhoneToId::<Impl, IMPL_OFFSET>,
            IdToPhone: IdToPhone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechPhoneConverter as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechPhraseAlternate_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn RecoResult(&mut self) -> ::windows::core::Result<ISpeechRecoResult>;
    fn StartElementInResult(&mut self) -> ::windows::core::Result<i32>;
    fn NumberOfElementsInResult(&mut self) -> ::windows::core::Result<i32>;
    fn PhraseInfo(&mut self) -> ::windows::core::Result<ISpeechPhraseInfo>;
    fn Commit(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechPhraseAlternate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseAlternate_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechPhraseAlternate_Vtbl {
        unsafe extern "system" fn RecoResult<Impl: ISpeechPhraseAlternate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recoresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecoResult() {
                ::core::result::Result::Ok(ok__) => {
                    *recoresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartElementInResult<Impl: ISpeechPhraseAlternate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startelement: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartElementInResult() {
                ::core::result::Result::Ok(ok__) => {
                    *startelement = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfElementsInResult<Impl: ISpeechPhraseAlternate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numberofelements: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NumberOfElementsInResult() {
                ::core::result::Result::Ok(ok__) => {
                    *numberofelements = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhraseInfo<Impl: ISpeechPhraseAlternate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phraseinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhraseInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *phraseinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Commit<Impl: ISpeechPhraseAlternate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Commit().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            RecoResult: RecoResult::<Impl, IMPL_OFFSET>,
            StartElementInResult: StartElementInResult::<Impl, IMPL_OFFSET>,
            NumberOfElementsInResult: NumberOfElementsInResult::<Impl, IMPL_OFFSET>,
            PhraseInfo: PhraseInfo::<Impl, IMPL_OFFSET>,
            Commit: Commit::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechPhraseAlternate as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechPhraseAlternates_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Item(&mut self, index: i32) -> ::windows::core::Result<ISpeechPhraseAlternate>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechPhraseAlternates_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseAlternates_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechPhraseAlternates_Vtbl {
        unsafe extern "system" fn Count<Impl: ISpeechPhraseAlternates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISpeechPhraseAlternates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, phrasealternate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *phrasealternate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISpeechPhraseAlternates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *enumvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechPhraseAlternates as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechPhraseElement_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn AudioTimeOffset(&mut self) -> ::windows::core::Result<i32>;
    fn AudioSizeTime(&mut self) -> ::windows::core::Result<i32>;
    fn AudioStreamOffset(&mut self) -> ::windows::core::Result<i32>;
    fn AudioSizeBytes(&mut self) -> ::windows::core::Result<i32>;
    fn RetainedStreamOffset(&mut self) -> ::windows::core::Result<i32>;
    fn RetainedSizeBytes(&mut self) -> ::windows::core::Result<i32>;
    fn DisplayText(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn LexicalForm(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Pronunciation(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn DisplayAttributes(&mut self) -> ::windows::core::Result<SpeechDisplayAttributes>;
    fn RequiredConfidence(&mut self) -> ::windows::core::Result<SpeechEngineConfidence>;
    fn ActualConfidence(&mut self) -> ::windows::core::Result<SpeechEngineConfidence>;
    fn EngineConfidence(&mut self) -> ::windows::core::Result<f32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechPhraseElement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseElement_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechPhraseElement_Vtbl {
        unsafe extern "system" fn AudioTimeOffset<Impl: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiotimeoffset: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioTimeOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *audiotimeoffset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioSizeTime<Impl: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiosizetime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioSizeTime() {
                ::core::result::Result::Ok(ok__) => {
                    *audiosizetime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioStreamOffset<Impl: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiostreamoffset: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioStreamOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *audiostreamoffset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioSizeBytes<Impl: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiosizebytes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioSizeBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *audiosizebytes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetainedStreamOffset<Impl: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retainedstreamoffset: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetainedStreamOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *retainedstreamoffset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetainedSizeBytes<Impl: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retainedsizebytes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetainedSizeBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *retainedsizebytes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayText<Impl: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displaytext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayText() {
                ::core::result::Result::Ok(ok__) => {
                    *displaytext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LexicalForm<Impl: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lexicalform: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LexicalForm() {
                ::core::result::Result::Ok(ok__) => {
                    *lexicalform = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pronunciation<Impl: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pronunciation: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pronunciation() {
                ::core::result::Result::Ok(ok__) => {
                    *pronunciation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayAttributes<Impl: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayattributes: *mut SpeechDisplayAttributes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayAttributes() {
                ::core::result::Result::Ok(ok__) => {
                    *displayattributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequiredConfidence<Impl: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requiredconfidence: *mut SpeechEngineConfidence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequiredConfidence() {
                ::core::result::Result::Ok(ok__) => {
                    *requiredconfidence = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActualConfidence<Impl: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, actualconfidence: *mut SpeechEngineConfidence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActualConfidence() {
                ::core::result::Result::Ok(ok__) => {
                    *actualconfidence = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EngineConfidence<Impl: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, engineconfidence: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EngineConfidence() {
                ::core::result::Result::Ok(ok__) => {
                    *engineconfidence = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AudioTimeOffset: AudioTimeOffset::<Impl, IMPL_OFFSET>,
            AudioSizeTime: AudioSizeTime::<Impl, IMPL_OFFSET>,
            AudioStreamOffset: AudioStreamOffset::<Impl, IMPL_OFFSET>,
            AudioSizeBytes: AudioSizeBytes::<Impl, IMPL_OFFSET>,
            RetainedStreamOffset: RetainedStreamOffset::<Impl, IMPL_OFFSET>,
            RetainedSizeBytes: RetainedSizeBytes::<Impl, IMPL_OFFSET>,
            DisplayText: DisplayText::<Impl, IMPL_OFFSET>,
            LexicalForm: LexicalForm::<Impl, IMPL_OFFSET>,
            Pronunciation: Pronunciation::<Impl, IMPL_OFFSET>,
            DisplayAttributes: DisplayAttributes::<Impl, IMPL_OFFSET>,
            RequiredConfidence: RequiredConfidence::<Impl, IMPL_OFFSET>,
            ActualConfidence: ActualConfidence::<Impl, IMPL_OFFSET>,
            EngineConfidence: EngineConfidence::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechPhraseElement as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechPhraseElements_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Item(&mut self, index: i32) -> ::windows::core::Result<ISpeechPhraseElement>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechPhraseElements_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseElements_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechPhraseElements_Vtbl {
        unsafe extern "system" fn Count<Impl: ISpeechPhraseElements_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISpeechPhraseElements_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *element = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISpeechPhraseElements_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *enumvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechPhraseElements as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechPhraseInfo_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn LanguageId(&mut self) -> ::windows::core::Result<i32>;
    fn GrammarId(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn StartTime(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn AudioStreamPosition(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn AudioSizeBytes(&mut self) -> ::windows::core::Result<i32>;
    fn RetainedSizeBytes(&mut self) -> ::windows::core::Result<i32>;
    fn AudioSizeTime(&mut self) -> ::windows::core::Result<i32>;
    fn Rule(&mut self) -> ::windows::core::Result<ISpeechPhraseRule>;
    fn Properties(&mut self) -> ::windows::core::Result<ISpeechPhraseProperties>;
    fn Elements(&mut self) -> ::windows::core::Result<ISpeechPhraseElements>;
    fn Replacements(&mut self) -> ::windows::core::Result<ISpeechPhraseReplacements>;
    fn EngineId(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn EnginePrivateData(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SaveToMemory(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn GetText(&mut self, startelement: i32, elements: i32, usereplacements: i16) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetDisplayAttributes(&mut self, startelement: i32, elements: i32, usereplacements: i16) -> ::windows::core::Result<SpeechDisplayAttributes>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechPhraseInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechPhraseInfo_Vtbl {
        unsafe extern "system" fn LanguageId<Impl: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languageid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LanguageId() {
                ::core::result::Result::Ok(ok__) => {
                    *languageid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GrammarId<Impl: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grammarid: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GrammarId() {
                ::core::result::Result::Ok(ok__) => {
                    *grammarid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartTime<Impl: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starttime: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *starttime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioStreamPosition<Impl: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiostreamposition: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioStreamPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *audiostreamposition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioSizeBytes<Impl: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paudiosizebytes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioSizeBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *paudiosizebytes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetainedSizeBytes<Impl: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retainedsizebytes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetainedSizeBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *retainedsizebytes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioSizeTime<Impl: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiosizetime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioSizeTime() {
                ::core::result::Result::Ok(ok__) => {
                    *audiosizetime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rule<Impl: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rule() {
                ::core::result::Result::Ok(ok__) => {
                    *rule = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, properties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *properties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Elements<Impl: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, elements: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Elements() {
                ::core::result::Result::Ok(ok__) => {
                    *elements = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Replacements<Impl: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, replacements: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Replacements() {
                ::core::result::Result::Ok(ok__) => {
                    *replacements = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EngineId<Impl: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, engineidguid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EngineId() {
                ::core::result::Result::Ok(ok__) => {
                    *engineidguid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnginePrivateData<Impl: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, privatedata: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnginePrivateData() {
                ::core::result::Result::Ok(ok__) => {
                    *privatedata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveToMemory<Impl: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phraseblock: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveToMemory() {
                ::core::result::Result::Ok(ok__) => {
                    *phraseblock = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetText<Impl: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startelement: i32, elements: i32, usereplacements: i16, text: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetText(::core::mem::transmute_copy(&startelement), ::core::mem::transmute_copy(&elements), ::core::mem::transmute_copy(&usereplacements)) {
                ::core::result::Result::Ok(ok__) => {
                    *text = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayAttributes<Impl: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startelement: i32, elements: i32, usereplacements: i16, displayattributes: *mut SpeechDisplayAttributes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDisplayAttributes(::core::mem::transmute_copy(&startelement), ::core::mem::transmute_copy(&elements), ::core::mem::transmute_copy(&usereplacements)) {
                ::core::result::Result::Ok(ok__) => {
                    *displayattributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            LanguageId: LanguageId::<Impl, IMPL_OFFSET>,
            GrammarId: GrammarId::<Impl, IMPL_OFFSET>,
            StartTime: StartTime::<Impl, IMPL_OFFSET>,
            AudioStreamPosition: AudioStreamPosition::<Impl, IMPL_OFFSET>,
            AudioSizeBytes: AudioSizeBytes::<Impl, IMPL_OFFSET>,
            RetainedSizeBytes: RetainedSizeBytes::<Impl, IMPL_OFFSET>,
            AudioSizeTime: AudioSizeTime::<Impl, IMPL_OFFSET>,
            Rule: Rule::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
            Elements: Elements::<Impl, IMPL_OFFSET>,
            Replacements: Replacements::<Impl, IMPL_OFFSET>,
            EngineId: EngineId::<Impl, IMPL_OFFSET>,
            EnginePrivateData: EnginePrivateData::<Impl, IMPL_OFFSET>,
            SaveToMemory: SaveToMemory::<Impl, IMPL_OFFSET>,
            GetText: GetText::<Impl, IMPL_OFFSET>,
            GetDisplayAttributes: GetDisplayAttributes::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechPhraseInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechPhraseInfoBuilder_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn RestorePhraseFromMemory(&mut self, phraseinmemory: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<ISpeechPhraseInfo>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechPhraseInfoBuilder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseInfoBuilder_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechPhraseInfoBuilder_Vtbl {
        unsafe extern "system" fn RestorePhraseFromMemory<Impl: ISpeechPhraseInfoBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phraseinmemory: *const super::super::System::Com::VARIANT, phraseinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RestorePhraseFromMemory(::core::mem::transmute_copy(&phraseinmemory)) {
                ::core::result::Result::Ok(ok__) => {
                    *phraseinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            RestorePhraseFromMemory: RestorePhraseFromMemory::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechPhraseInfoBuilder as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechPhraseProperties_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Item(&mut self, index: i32) -> ::windows::core::Result<ISpeechPhraseProperty>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechPhraseProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseProperties_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechPhraseProperties_Vtbl {
        unsafe extern "system" fn Count<Impl: ISpeechPhraseProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISpeechPhraseProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, property: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *property = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISpeechPhraseProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *enumvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechPhraseProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechPhraseProperty_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Id(&mut self) -> ::windows::core::Result<i32>;
    fn Value(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn FirstElement(&mut self) -> ::windows::core::Result<i32>;
    fn NumberOfElements(&mut self) -> ::windows::core::Result<i32>;
    fn EngineConfidence(&mut self) -> ::windows::core::Result<f32>;
    fn Confidence(&mut self) -> ::windows::core::Result<SpeechEngineConfidence>;
    fn Parent(&mut self) -> ::windows::core::Result<ISpeechPhraseProperty>;
    fn Children(&mut self) -> ::windows::core::Result<ISpeechPhraseProperties>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechPhraseProperty_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseProperty_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechPhraseProperty_Vtbl {
        unsafe extern "system" fn Name<Impl: ISpeechPhraseProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: ISpeechPhraseProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *id = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: ISpeechPhraseProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirstElement<Impl: ISpeechPhraseProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, firstelement: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FirstElement() {
                ::core::result::Result::Ok(ok__) => {
                    *firstelement = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfElements<Impl: ISpeechPhraseProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numberofelements: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NumberOfElements() {
                ::core::result::Result::Ok(ok__) => {
                    *numberofelements = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EngineConfidence<Impl: ISpeechPhraseProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, confidence: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EngineConfidence() {
                ::core::result::Result::Ok(ok__) => {
                    *confidence = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Confidence<Impl: ISpeechPhraseProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, confidence: *mut SpeechEngineConfidence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Confidence() {
                ::core::result::Result::Ok(ok__) => {
                    *confidence = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Impl: ISpeechPhraseProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parentproperty: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parent() {
                ::core::result::Result::Ok(ok__) => {
                    *parentproperty = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Children<Impl: ISpeechPhraseProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, children: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Children() {
                ::core::result::Result::Ok(ok__) => {
                    *children = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
            FirstElement: FirstElement::<Impl, IMPL_OFFSET>,
            NumberOfElements: NumberOfElements::<Impl, IMPL_OFFSET>,
            EngineConfidence: EngineConfidence::<Impl, IMPL_OFFSET>,
            Confidence: Confidence::<Impl, IMPL_OFFSET>,
            Parent: Parent::<Impl, IMPL_OFFSET>,
            Children: Children::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechPhraseProperty as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechPhraseReplacement_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn DisplayAttributes(&mut self) -> ::windows::core::Result<SpeechDisplayAttributes>;
    fn Text(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn FirstElement(&mut self) -> ::windows::core::Result<i32>;
    fn NumberOfElements(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechPhraseReplacement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseReplacement_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechPhraseReplacement_Vtbl {
        unsafe extern "system" fn DisplayAttributes<Impl: ISpeechPhraseReplacement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayattributes: *mut SpeechDisplayAttributes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayAttributes() {
                ::core::result::Result::Ok(ok__) => {
                    *displayattributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Text<Impl: ISpeechPhraseReplacement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Text() {
                ::core::result::Result::Ok(ok__) => {
                    *text = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirstElement<Impl: ISpeechPhraseReplacement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, firstelement: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FirstElement() {
                ::core::result::Result::Ok(ok__) => {
                    *firstelement = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfElements<Impl: ISpeechPhraseReplacement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numberofelements: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NumberOfElements() {
                ::core::result::Result::Ok(ok__) => {
                    *numberofelements = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            DisplayAttributes: DisplayAttributes::<Impl, IMPL_OFFSET>,
            Text: Text::<Impl, IMPL_OFFSET>,
            FirstElement: FirstElement::<Impl, IMPL_OFFSET>,
            NumberOfElements: NumberOfElements::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechPhraseReplacement as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechPhraseReplacements_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Item(&mut self, index: i32) -> ::windows::core::Result<ISpeechPhraseReplacement>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechPhraseReplacements_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseReplacements_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechPhraseReplacements_Vtbl {
        unsafe extern "system" fn Count<Impl: ISpeechPhraseReplacements_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISpeechPhraseReplacements_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, reps: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *reps = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISpeechPhraseReplacements_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *enumvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechPhraseReplacements as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechPhraseRule_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Id(&mut self) -> ::windows::core::Result<i32>;
    fn FirstElement(&mut self) -> ::windows::core::Result<i32>;
    fn NumberOfElements(&mut self) -> ::windows::core::Result<i32>;
    fn Parent(&mut self) -> ::windows::core::Result<ISpeechPhraseRule>;
    fn Children(&mut self) -> ::windows::core::Result<ISpeechPhraseRules>;
    fn Confidence(&mut self) -> ::windows::core::Result<SpeechEngineConfidence>;
    fn EngineConfidence(&mut self) -> ::windows::core::Result<f32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechPhraseRule_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseRule_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechPhraseRule_Vtbl {
        unsafe extern "system" fn Name<Impl: ISpeechPhraseRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: ISpeechPhraseRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *id = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirstElement<Impl: ISpeechPhraseRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, firstelement: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FirstElement() {
                ::core::result::Result::Ok(ok__) => {
                    *firstelement = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfElements<Impl: ISpeechPhraseRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numberofelements: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NumberOfElements() {
                ::core::result::Result::Ok(ok__) => {
                    *numberofelements = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Impl: ISpeechPhraseRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parent() {
                ::core::result::Result::Ok(ok__) => {
                    *parent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Children<Impl: ISpeechPhraseRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, children: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Children() {
                ::core::result::Result::Ok(ok__) => {
                    *children = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Confidence<Impl: ISpeechPhraseRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, actualconfidence: *mut SpeechEngineConfidence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Confidence() {
                ::core::result::Result::Ok(ok__) => {
                    *actualconfidence = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EngineConfidence<Impl: ISpeechPhraseRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, engineconfidence: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EngineConfidence() {
                ::core::result::Result::Ok(ok__) => {
                    *engineconfidence = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            FirstElement: FirstElement::<Impl, IMPL_OFFSET>,
            NumberOfElements: NumberOfElements::<Impl, IMPL_OFFSET>,
            Parent: Parent::<Impl, IMPL_OFFSET>,
            Children: Children::<Impl, IMPL_OFFSET>,
            Confidence: Confidence::<Impl, IMPL_OFFSET>,
            EngineConfidence: EngineConfidence::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechPhraseRule as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechPhraseRules_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Item(&mut self, index: i32) -> ::windows::core::Result<ISpeechPhraseRule>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechPhraseRules_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechPhraseRules_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechPhraseRules_Vtbl {
        unsafe extern "system" fn Count<Impl: ISpeechPhraseRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISpeechPhraseRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, rule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *rule = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISpeechPhraseRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *enumvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechPhraseRules as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechRecoContext_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Recognizer(&mut self) -> ::windows::core::Result<ISpeechRecognizer>;
    fn AudioInputInterferenceStatus(&mut self) -> ::windows::core::Result<SpeechInterference>;
    fn RequestedUIType(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn putref_Voice(&mut self, voice: &::core::option::Option<ISpeechVoice>) -> ::windows::core::Result<()>;
    fn Voice(&mut self) -> ::windows::core::Result<ISpeechVoice>;
    fn SetAllowVoiceFormatMatchingOnNextSet(&mut self, allow: i16) -> ::windows::core::Result<()>;
    fn AllowVoiceFormatMatchingOnNextSet(&mut self) -> ::windows::core::Result<i16>;
    fn SetVoicePurgeEvent(&mut self, eventinterest: SpeechRecoEvents) -> ::windows::core::Result<()>;
    fn VoicePurgeEvent(&mut self) -> ::windows::core::Result<SpeechRecoEvents>;
    fn SetEventInterests(&mut self, eventinterest: SpeechRecoEvents) -> ::windows::core::Result<()>;
    fn EventInterests(&mut self) -> ::windows::core::Result<SpeechRecoEvents>;
    fn SetCmdMaxAlternates(&mut self, maxalternates: i32) -> ::windows::core::Result<()>;
    fn CmdMaxAlternates(&mut self) -> ::windows::core::Result<i32>;
    fn SetState(&mut self, state: SpeechRecoContextState) -> ::windows::core::Result<()>;
    fn State(&mut self) -> ::windows::core::Result<SpeechRecoContextState>;
    fn SetRetainedAudio(&mut self, option: SpeechRetainedAudioOptions) -> ::windows::core::Result<()>;
    fn RetainedAudio(&mut self) -> ::windows::core::Result<SpeechRetainedAudioOptions>;
    fn putref_RetainedAudioFormat(&mut self, format: &::core::option::Option<ISpeechAudioFormat>) -> ::windows::core::Result<()>;
    fn RetainedAudioFormat(&mut self) -> ::windows::core::Result<ISpeechAudioFormat>;
    fn Pause(&mut self) -> ::windows::core::Result<()>;
    fn Resume(&mut self) -> ::windows::core::Result<()>;
    fn CreateGrammar(&mut self, grammarid: &super::super::System::Com::VARIANT) -> ::windows::core::Result<ISpeechRecoGrammar>;
    fn CreateResultFromMemory(&mut self, resultblock: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<ISpeechRecoResult>;
    fn Bookmark(&mut self, options: SpeechBookmarkOptions, streampos: &super::super::System::Com::VARIANT, bookmarkid: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SetAdaptationData(&mut self, adaptationstring: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechRecoContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoContext_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechRecoContext_Vtbl {
        unsafe extern "system" fn Recognizer<Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recognizer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Recognizer() {
                ::core::result::Result::Ok(ok__) => {
                    *recognizer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioInputInterferenceStatus<Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interference: *mut SpeechInterference) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioInputInterferenceStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *interference = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestedUIType<Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uitype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestedUIType() {
                ::core::result::Result::Ok(ok__) => {
                    *uitype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Voice<Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, voice: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_Voice(::core::mem::transmute(&voice)).into()
        }
        unsafe extern "system" fn Voice<Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, voice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Voice() {
                ::core::result::Result::Ok(ok__) => {
                    *voice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowVoiceFormatMatchingOnNextSet<Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowVoiceFormatMatchingOnNextSet(::core::mem::transmute_copy(&allow)).into()
        }
        unsafe extern "system" fn AllowVoiceFormatMatchingOnNextSet<Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pallow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowVoiceFormatMatchingOnNextSet() {
                ::core::result::Result::Ok(ok__) => {
                    *pallow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVoicePurgeEvent<Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventinterest: SpeechRecoEvents) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVoicePurgeEvent(::core::mem::transmute_copy(&eventinterest)).into()
        }
        unsafe extern "system" fn VoicePurgeEvent<Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventinterest: *mut SpeechRecoEvents) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VoicePurgeEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *eventinterest = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventInterests<Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventinterest: SpeechRecoEvents) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEventInterests(::core::mem::transmute_copy(&eventinterest)).into()
        }
        unsafe extern "system" fn EventInterests<Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventinterest: *mut SpeechRecoEvents) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventInterests() {
                ::core::result::Result::Ok(ok__) => {
                    *eventinterest = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCmdMaxAlternates<Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxalternates: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCmdMaxAlternates(::core::mem::transmute_copy(&maxalternates)).into()
        }
        unsafe extern "system" fn CmdMaxAlternates<Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxalternates: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CmdMaxAlternates() {
                ::core::result::Result::Ok(ok__) => {
                    *maxalternates = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: SpeechRecoContextState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetState(::core::mem::transmute_copy(&state)).into()
        }
        unsafe extern "system" fn State<Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: *mut SpeechRecoContextState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *state = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRetainedAudio<Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, option: SpeechRetainedAudioOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRetainedAudio(::core::mem::transmute_copy(&option)).into()
        }
        unsafe extern "system" fn RetainedAudio<Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, option: *mut SpeechRetainedAudioOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetainedAudio() {
                ::core::result::Result::Ok(ok__) => {
                    *option = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_RetainedAudioFormat<Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_RetainedAudioFormat(::core::mem::transmute(&format)).into()
        }
        unsafe extern "system" fn RetainedAudioFormat<Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetainedAudioFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *format = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pause<Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn Resume<Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Resume().into()
        }
        unsafe extern "system" fn CreateGrammar<Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grammarid: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, grammar: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateGrammar(::core::mem::transmute_copy(&grammarid)) {
                ::core::result::Result::Ok(ok__) => {
                    *grammar = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateResultFromMemory<Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resultblock: *const super::super::System::Com::VARIANT, result: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateResultFromMemory(::core::mem::transmute_copy(&resultblock)) {
                ::core::result::Result::Ok(ok__) => {
                    *result = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Bookmark<Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: SpeechBookmarkOptions, streampos: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, bookmarkid: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Bookmark(::core::mem::transmute_copy(&options), ::core::mem::transmute_copy(&streampos), ::core::mem::transmute_copy(&bookmarkid)).into()
        }
        unsafe extern "system" fn SetAdaptationData<Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adaptationstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAdaptationData(::core::mem::transmute_copy(&adaptationstring)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Recognizer: Recognizer::<Impl, IMPL_OFFSET>,
            AudioInputInterferenceStatus: AudioInputInterferenceStatus::<Impl, IMPL_OFFSET>,
            RequestedUIType: RequestedUIType::<Impl, IMPL_OFFSET>,
            putref_Voice: putref_Voice::<Impl, IMPL_OFFSET>,
            Voice: Voice::<Impl, IMPL_OFFSET>,
            SetAllowVoiceFormatMatchingOnNextSet: SetAllowVoiceFormatMatchingOnNextSet::<Impl, IMPL_OFFSET>,
            AllowVoiceFormatMatchingOnNextSet: AllowVoiceFormatMatchingOnNextSet::<Impl, IMPL_OFFSET>,
            SetVoicePurgeEvent: SetVoicePurgeEvent::<Impl, IMPL_OFFSET>,
            VoicePurgeEvent: VoicePurgeEvent::<Impl, IMPL_OFFSET>,
            SetEventInterests: SetEventInterests::<Impl, IMPL_OFFSET>,
            EventInterests: EventInterests::<Impl, IMPL_OFFSET>,
            SetCmdMaxAlternates: SetCmdMaxAlternates::<Impl, IMPL_OFFSET>,
            CmdMaxAlternates: CmdMaxAlternates::<Impl, IMPL_OFFSET>,
            SetState: SetState::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
            SetRetainedAudio: SetRetainedAudio::<Impl, IMPL_OFFSET>,
            RetainedAudio: RetainedAudio::<Impl, IMPL_OFFSET>,
            putref_RetainedAudioFormat: putref_RetainedAudioFormat::<Impl, IMPL_OFFSET>,
            RetainedAudioFormat: RetainedAudioFormat::<Impl, IMPL_OFFSET>,
            Pause: Pause::<Impl, IMPL_OFFSET>,
            Resume: Resume::<Impl, IMPL_OFFSET>,
            CreateGrammar: CreateGrammar::<Impl, IMPL_OFFSET>,
            CreateResultFromMemory: CreateResultFromMemory::<Impl, IMPL_OFFSET>,
            Bookmark: Bookmark::<Impl, IMPL_OFFSET>,
            SetAdaptationData: SetAdaptationData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechRecoContext as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechRecoGrammar_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Id(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn RecoContext(&mut self) -> ::windows::core::Result<ISpeechRecoContext>;
    fn SetState(&mut self, state: SpeechGrammarState) -> ::windows::core::Result<()>;
    fn State(&mut self) -> ::windows::core::Result<SpeechGrammarState>;
    fn Rules(&mut self) -> ::windows::core::Result<ISpeechGrammarRules>;
    fn Reset(&mut self, newlanguage: i32) -> ::windows::core::Result<()>;
    fn CmdLoadFromFile(&mut self, filename: &super::super::Foundation::BSTR, loadoption: SpeechLoadOption) -> ::windows::core::Result<()>;
    fn CmdLoadFromObject(&mut self, classid: &super::super::Foundation::BSTR, grammarname: &super::super::Foundation::BSTR, loadoption: SpeechLoadOption) -> ::windows::core::Result<()>;
    fn CmdLoadFromResource(&mut self, hmodule: i32, resourcename: &super::super::System::Com::VARIANT, resourcetype: &super::super::System::Com::VARIANT, languageid: i32, loadoption: SpeechLoadOption) -> ::windows::core::Result<()>;
    fn CmdLoadFromMemory(&mut self, grammardata: &super::super::System::Com::VARIANT, loadoption: SpeechLoadOption) -> ::windows::core::Result<()>;
    fn CmdLoadFromProprietaryGrammar(&mut self, proprietaryguid: &super::super::Foundation::BSTR, proprietarystring: &super::super::Foundation::BSTR, proprietarydata: &super::super::System::Com::VARIANT, loadoption: SpeechLoadOption) -> ::windows::core::Result<()>;
    fn CmdSetRuleState(&mut self, name: &super::super::Foundation::BSTR, state: SpeechRuleState) -> ::windows::core::Result<()>;
    fn CmdSetRuleIdState(&mut self, ruleid: i32, state: SpeechRuleState) -> ::windows::core::Result<()>;
    fn DictationLoad(&mut self, topicname: &super::super::Foundation::BSTR, loadoption: SpeechLoadOption) -> ::windows::core::Result<()>;
    fn DictationUnload(&mut self) -> ::windows::core::Result<()>;
    fn DictationSetState(&mut self, state: SpeechRuleState) -> ::windows::core::Result<()>;
    fn SetWordSequenceData(&mut self, text: &super::super::Foundation::BSTR, textlength: i32, info: &::core::option::Option<ISpeechTextSelectionInformation>) -> ::windows::core::Result<()>;
    fn SetTextSelection(&mut self, info: &::core::option::Option<ISpeechTextSelectionInformation>) -> ::windows::core::Result<()>;
    fn IsPronounceable(&mut self, word: &super::super::Foundation::BSTR) -> ::windows::core::Result<SpeechWordPronounceable>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechRecoGrammar_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoGrammar_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechRecoGrammar_Vtbl {
        unsafe extern "system" fn Id<Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *id = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecoContext<Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recocontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecoContext() {
                ::core::result::Result::Ok(ok__) => {
                    *recocontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: SpeechGrammarState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetState(::core::mem::transmute_copy(&state)).into()
        }
        unsafe extern "system" fn State<Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: *mut SpeechGrammarState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *state = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rules<Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rules: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rules() {
                ::core::result::Result::Ok(ok__) => {
                    *rules = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newlanguage: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset(::core::mem::transmute_copy(&newlanguage)).into()
        }
        unsafe extern "system" fn CmdLoadFromFile<Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loadoption: SpeechLoadOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CmdLoadFromFile(::core::mem::transmute_copy(&filename), ::core::mem::transmute_copy(&loadoption)).into()
        }
        unsafe extern "system" fn CmdLoadFromObject<Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, classid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, grammarname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loadoption: SpeechLoadOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CmdLoadFromObject(::core::mem::transmute_copy(&classid), ::core::mem::transmute_copy(&grammarname), ::core::mem::transmute_copy(&loadoption)).into()
        }
        unsafe extern "system" fn CmdLoadFromResource<Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmodule: i32, resourcename: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, resourcetype: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, languageid: i32, loadoption: SpeechLoadOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CmdLoadFromResource(::core::mem::transmute_copy(&hmodule), ::core::mem::transmute_copy(&resourcename), ::core::mem::transmute_copy(&resourcetype), ::core::mem::transmute_copy(&languageid), ::core::mem::transmute_copy(&loadoption)).into()
        }
        unsafe extern "system" fn CmdLoadFromMemory<Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grammardata: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, loadoption: SpeechLoadOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CmdLoadFromMemory(::core::mem::transmute_copy(&grammardata), ::core::mem::transmute_copy(&loadoption)).into()
        }
        unsafe extern "system" fn CmdLoadFromProprietaryGrammar<Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, proprietaryguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, proprietarystring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, proprietarydata: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, loadoption: SpeechLoadOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CmdLoadFromProprietaryGrammar(::core::mem::transmute_copy(&proprietaryguid), ::core::mem::transmute_copy(&proprietarystring), ::core::mem::transmute_copy(&proprietarydata), ::core::mem::transmute_copy(&loadoption)).into()
        }
        unsafe extern "system" fn CmdSetRuleState<Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, state: SpeechRuleState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CmdSetRuleState(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&state)).into()
        }
        unsafe extern "system" fn CmdSetRuleIdState<Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ruleid: i32, state: SpeechRuleState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CmdSetRuleIdState(::core::mem::transmute_copy(&ruleid), ::core::mem::transmute_copy(&state)).into()
        }
        unsafe extern "system" fn DictationLoad<Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, topicname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loadoption: SpeechLoadOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DictationLoad(::core::mem::transmute_copy(&topicname), ::core::mem::transmute_copy(&loadoption)).into()
        }
        unsafe extern "system" fn DictationUnload<Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DictationUnload().into()
        }
        unsafe extern "system" fn DictationSetState<Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: SpeechRuleState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DictationSetState(::core::mem::transmute_copy(&state)).into()
        }
        unsafe extern "system" fn SetWordSequenceData<Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, textlength: i32, info: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWordSequenceData(::core::mem::transmute_copy(&text), ::core::mem::transmute_copy(&textlength), ::core::mem::transmute(&info)).into()
        }
        unsafe extern "system" fn SetTextSelection<Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, info: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTextSelection(::core::mem::transmute(&info)).into()
        }
        unsafe extern "system" fn IsPronounceable<Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, word: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, wordpronounceable: *mut SpeechWordPronounceable) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPronounceable(::core::mem::transmute_copy(&word)) {
                ::core::result::Result::Ok(ok__) => {
                    *wordpronounceable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            RecoContext: RecoContext::<Impl, IMPL_OFFSET>,
            SetState: SetState::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
            Rules: Rules::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            CmdLoadFromFile: CmdLoadFromFile::<Impl, IMPL_OFFSET>,
            CmdLoadFromObject: CmdLoadFromObject::<Impl, IMPL_OFFSET>,
            CmdLoadFromResource: CmdLoadFromResource::<Impl, IMPL_OFFSET>,
            CmdLoadFromMemory: CmdLoadFromMemory::<Impl, IMPL_OFFSET>,
            CmdLoadFromProprietaryGrammar: CmdLoadFromProprietaryGrammar::<Impl, IMPL_OFFSET>,
            CmdSetRuleState: CmdSetRuleState::<Impl, IMPL_OFFSET>,
            CmdSetRuleIdState: CmdSetRuleIdState::<Impl, IMPL_OFFSET>,
            DictationLoad: DictationLoad::<Impl, IMPL_OFFSET>,
            DictationUnload: DictationUnload::<Impl, IMPL_OFFSET>,
            DictationSetState: DictationSetState::<Impl, IMPL_OFFSET>,
            SetWordSequenceData: SetWordSequenceData::<Impl, IMPL_OFFSET>,
            SetTextSelection: SetTextSelection::<Impl, IMPL_OFFSET>,
            IsPronounceable: IsPronounceable::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechRecoGrammar as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechRecoResult_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn RecoContext(&mut self) -> ::windows::core::Result<ISpeechRecoContext>;
    fn Times(&mut self) -> ::windows::core::Result<ISpeechRecoResultTimes>;
    fn putref_AudioFormat(&mut self, format: &::core::option::Option<ISpeechAudioFormat>) -> ::windows::core::Result<()>;
    fn AudioFormat(&mut self) -> ::windows::core::Result<ISpeechAudioFormat>;
    fn PhraseInfo(&mut self) -> ::windows::core::Result<ISpeechPhraseInfo>;
    fn Alternates(&mut self, requestcount: i32, startelement: i32, elements: i32) -> ::windows::core::Result<ISpeechPhraseAlternates>;
    fn Audio(&mut self, startelement: i32, elements: i32) -> ::windows::core::Result<ISpeechMemoryStream>;
    fn SpeakAudio(&mut self, startelement: i32, elements: i32, flags: SpeechVoiceSpeakFlags) -> ::windows::core::Result<i32>;
    fn SaveToMemory(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn DiscardResultInfo(&mut self, valuetypes: SpeechDiscardType) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechRecoResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechRecoResult_Vtbl {
        unsafe extern "system" fn RecoContext<Impl: ISpeechRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recocontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecoContext() {
                ::core::result::Result::Ok(ok__) => {
                    *recocontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Times<Impl: ISpeechRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, times: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Times() {
                ::core::result::Result::Ok(ok__) => {
                    *times = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AudioFormat<Impl: ISpeechRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_AudioFormat(::core::mem::transmute(&format)).into()
        }
        unsafe extern "system" fn AudioFormat<Impl: ISpeechRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *format = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhraseInfo<Impl: ISpeechRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phraseinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhraseInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *phraseinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Alternates<Impl: ISpeechRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestcount: i32, startelement: i32, elements: i32, alternates: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Alternates(::core::mem::transmute_copy(&requestcount), ::core::mem::transmute_copy(&startelement), ::core::mem::transmute_copy(&elements)) {
                ::core::result::Result::Ok(ok__) => {
                    *alternates = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Audio<Impl: ISpeechRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startelement: i32, elements: i32, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Audio(::core::mem::transmute_copy(&startelement), ::core::mem::transmute_copy(&elements)) {
                ::core::result::Result::Ok(ok__) => {
                    *stream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpeakAudio<Impl: ISpeechRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startelement: i32, elements: i32, flags: SpeechVoiceSpeakFlags, streamnumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SpeakAudio(::core::mem::transmute_copy(&startelement), ::core::mem::transmute_copy(&elements), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *streamnumber = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveToMemory<Impl: ISpeechRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resultblock: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveToMemory() {
                ::core::result::Result::Ok(ok__) => {
                    *resultblock = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DiscardResultInfo<Impl: ISpeechRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, valuetypes: SpeechDiscardType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DiscardResultInfo(::core::mem::transmute_copy(&valuetypes)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            RecoContext: RecoContext::<Impl, IMPL_OFFSET>,
            Times: Times::<Impl, IMPL_OFFSET>,
            putref_AudioFormat: putref_AudioFormat::<Impl, IMPL_OFFSET>,
            AudioFormat: AudioFormat::<Impl, IMPL_OFFSET>,
            PhraseInfo: PhraseInfo::<Impl, IMPL_OFFSET>,
            Alternates: Alternates::<Impl, IMPL_OFFSET>,
            Audio: Audio::<Impl, IMPL_OFFSET>,
            SpeakAudio: SpeakAudio::<Impl, IMPL_OFFSET>,
            SaveToMemory: SaveToMemory::<Impl, IMPL_OFFSET>,
            DiscardResultInfo: DiscardResultInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechRecoResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechRecoResult2_Impl: Sized + super::super::System::Com::IDispatch_Impl + ISpeechRecoResult_Impl {
    fn SetTextFeedback(&mut self, feedback: &super::super::Foundation::BSTR, wassuccessful: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechRecoResult2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoResult2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechRecoResult2_Vtbl {
        unsafe extern "system" fn SetTextFeedback<Impl: ISpeechRecoResult2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedback: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, wassuccessful: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTextFeedback(::core::mem::transmute_copy(&feedback), ::core::mem::transmute_copy(&wassuccessful)).into()
        }
        Self { base: ISpeechRecoResult_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), SetTextFeedback: SetTextFeedback::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechRecoResult2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechRecoResultDispatch_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn RecoContext(&mut self) -> ::windows::core::Result<ISpeechRecoContext>;
    fn Times(&mut self) -> ::windows::core::Result<ISpeechRecoResultTimes>;
    fn putref_AudioFormat(&mut self, format: &::core::option::Option<ISpeechAudioFormat>) -> ::windows::core::Result<()>;
    fn AudioFormat(&mut self) -> ::windows::core::Result<ISpeechAudioFormat>;
    fn PhraseInfo(&mut self) -> ::windows::core::Result<ISpeechPhraseInfo>;
    fn Alternates(&mut self, requestcount: i32, startelement: i32, elements: i32) -> ::windows::core::Result<ISpeechPhraseAlternates>;
    fn Audio(&mut self, startelement: i32, elements: i32) -> ::windows::core::Result<ISpeechMemoryStream>;
    fn SpeakAudio(&mut self, startelement: i32, elements: i32, flags: SpeechVoiceSpeakFlags) -> ::windows::core::Result<i32>;
    fn SaveToMemory(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn DiscardResultInfo(&mut self, valuetypes: SpeechDiscardType) -> ::windows::core::Result<()>;
    fn GetXMLResult(&mut self, options: SPXMLRESULTOPTIONS) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetXMLErrorInfo(&mut self, linenumber: *mut i32, scriptline: *mut super::super::Foundation::BSTR, source: *mut super::super::Foundation::BSTR, description: *mut super::super::Foundation::BSTR, resultcode: *mut ::windows::core::HRESULT, iserror: *mut i16) -> ::windows::core::Result<()>;
    fn SetTextFeedback(&mut self, feedback: &super::super::Foundation::BSTR, wassuccessful: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechRecoResultDispatch_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoResultDispatch_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechRecoResultDispatch_Vtbl {
        unsafe extern "system" fn RecoContext<Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recocontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecoContext() {
                ::core::result::Result::Ok(ok__) => {
                    *recocontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Times<Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, times: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Times() {
                ::core::result::Result::Ok(ok__) => {
                    *times = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AudioFormat<Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_AudioFormat(::core::mem::transmute(&format)).into()
        }
        unsafe extern "system" fn AudioFormat<Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *format = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhraseInfo<Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phraseinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhraseInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *phraseinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Alternates<Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestcount: i32, startelement: i32, elements: i32, alternates: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Alternates(::core::mem::transmute_copy(&requestcount), ::core::mem::transmute_copy(&startelement), ::core::mem::transmute_copy(&elements)) {
                ::core::result::Result::Ok(ok__) => {
                    *alternates = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Audio<Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startelement: i32, elements: i32, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Audio(::core::mem::transmute_copy(&startelement), ::core::mem::transmute_copy(&elements)) {
                ::core::result::Result::Ok(ok__) => {
                    *stream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpeakAudio<Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startelement: i32, elements: i32, flags: SpeechVoiceSpeakFlags, streamnumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SpeakAudio(::core::mem::transmute_copy(&startelement), ::core::mem::transmute_copy(&elements), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *streamnumber = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveToMemory<Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resultblock: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveToMemory() {
                ::core::result::Result::Ok(ok__) => {
                    *resultblock = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DiscardResultInfo<Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, valuetypes: SpeechDiscardType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DiscardResultInfo(::core::mem::transmute_copy(&valuetypes)).into()
        }
        unsafe extern "system" fn GetXMLResult<Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: SPXMLRESULTOPTIONS, presult: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetXMLResult(::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    *presult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXMLErrorInfo<Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linenumber: *mut i32, scriptline: *mut super::super::Foundation::BSTR, source: *mut super::super::Foundation::BSTR, description: *mut super::super::Foundation::BSTR, resultcode: *mut ::windows::core::HRESULT, iserror: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetXMLErrorInfo(::core::mem::transmute_copy(&linenumber), ::core::mem::transmute_copy(&scriptline), ::core::mem::transmute_copy(&source), ::core::mem::transmute_copy(&description), ::core::mem::transmute_copy(&resultcode), ::core::mem::transmute_copy(&iserror)).into()
        }
        unsafe extern "system" fn SetTextFeedback<Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedback: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, wassuccessful: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTextFeedback(::core::mem::transmute_copy(&feedback), ::core::mem::transmute_copy(&wassuccessful)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            RecoContext: RecoContext::<Impl, IMPL_OFFSET>,
            Times: Times::<Impl, IMPL_OFFSET>,
            putref_AudioFormat: putref_AudioFormat::<Impl, IMPL_OFFSET>,
            AudioFormat: AudioFormat::<Impl, IMPL_OFFSET>,
            PhraseInfo: PhraseInfo::<Impl, IMPL_OFFSET>,
            Alternates: Alternates::<Impl, IMPL_OFFSET>,
            Audio: Audio::<Impl, IMPL_OFFSET>,
            SpeakAudio: SpeakAudio::<Impl, IMPL_OFFSET>,
            SaveToMemory: SaveToMemory::<Impl, IMPL_OFFSET>,
            DiscardResultInfo: DiscardResultInfo::<Impl, IMPL_OFFSET>,
            GetXMLResult: GetXMLResult::<Impl, IMPL_OFFSET>,
            GetXMLErrorInfo: GetXMLErrorInfo::<Impl, IMPL_OFFSET>,
            SetTextFeedback: SetTextFeedback::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechRecoResultDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechRecoResultTimes_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn StreamTime(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Length(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn TickCount(&mut self) -> ::windows::core::Result<i32>;
    fn OffsetFromStart(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechRecoResultTimes_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecoResultTimes_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechRecoResultTimes_Vtbl {
        unsafe extern "system" fn StreamTime<Impl: ISpeechRecoResultTimes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, time: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StreamTime() {
                ::core::result::Result::Ok(ok__) => {
                    *time = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Length<Impl: ISpeechRecoResultTimes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Length() {
                ::core::result::Result::Ok(ok__) => {
                    *length = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TickCount<Impl: ISpeechRecoResultTimes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tickcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TickCount() {
                ::core::result::Result::Ok(ok__) => {
                    *tickcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OffsetFromStart<Impl: ISpeechRecoResultTimes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsetfromstart: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OffsetFromStart() {
                ::core::result::Result::Ok(ok__) => {
                    *offsetfromstart = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            StreamTime: StreamTime::<Impl, IMPL_OFFSET>,
            Length: Length::<Impl, IMPL_OFFSET>,
            TickCount: TickCount::<Impl, IMPL_OFFSET>,
            OffsetFromStart: OffsetFromStart::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechRecoResultTimes as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechRecognizer_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn putref_Recognizer(&mut self, recognizer: &::core::option::Option<ISpeechObjectToken>) -> ::windows::core::Result<()>;
    fn Recognizer(&mut self) -> ::windows::core::Result<ISpeechObjectToken>;
    fn SetAllowAudioInputFormatChangesOnNextSet(&mut self, allow: i16) -> ::windows::core::Result<()>;
    fn AllowAudioInputFormatChangesOnNextSet(&mut self) -> ::windows::core::Result<i16>;
    fn putref_AudioInput(&mut self, audioinput: &::core::option::Option<ISpeechObjectToken>) -> ::windows::core::Result<()>;
    fn AudioInput(&mut self) -> ::windows::core::Result<ISpeechObjectToken>;
    fn putref_AudioInputStream(&mut self, audioinputstream: &::core::option::Option<ISpeechBaseStream>) -> ::windows::core::Result<()>;
    fn AudioInputStream(&mut self) -> ::windows::core::Result<ISpeechBaseStream>;
    fn IsShared(&mut self) -> ::windows::core::Result<i16>;
    fn SetState(&mut self, state: SpeechRecognizerState) -> ::windows::core::Result<()>;
    fn State(&mut self) -> ::windows::core::Result<SpeechRecognizerState>;
    fn Status(&mut self) -> ::windows::core::Result<ISpeechRecognizerStatus>;
    fn putref_Profile(&mut self, profile: &::core::option::Option<ISpeechObjectToken>) -> ::windows::core::Result<()>;
    fn Profile(&mut self) -> ::windows::core::Result<ISpeechObjectToken>;
    fn EmulateRecognition(&mut self, textelements: &super::super::System::Com::VARIANT, elementdisplayattributes: *const super::super::System::Com::VARIANT, languageid: i32) -> ::windows::core::Result<()>;
    fn CreateRecoContext(&mut self) -> ::windows::core::Result<ISpeechRecoContext>;
    fn GetFormat(&mut self, r#type: SpeechFormatType) -> ::windows::core::Result<ISpeechAudioFormat>;
    fn SetPropertyNumber(&mut self, name: &super::super::Foundation::BSTR, value: i32) -> ::windows::core::Result<i16>;
    fn GetPropertyNumber(&mut self, name: &super::super::Foundation::BSTR, value: *mut i32, supported: *mut i16) -> ::windows::core::Result<()>;
    fn SetPropertyString(&mut self, name: &super::super::Foundation::BSTR, value: &super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
    fn GetPropertyString(&mut self, name: &super::super::Foundation::BSTR, value: *mut super::super::Foundation::BSTR, supported: *mut i16) -> ::windows::core::Result<()>;
    fn IsUISupported(&mut self, typeofui: &super::super::Foundation::BSTR, extradata: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<i16>;
    fn DisplayUI(&mut self, hwndparent: i32, title: &super::super::Foundation::BSTR, typeofui: &super::super::Foundation::BSTR, extradata: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GetRecognizers(&mut self, requiredattributes: &super::super::Foundation::BSTR, optionalattributes: &super::super::Foundation::BSTR) -> ::windows::core::Result<ISpeechObjectTokens>;
    fn GetAudioInputs(&mut self, requiredattributes: &super::super::Foundation::BSTR, optionalattributes: &super::super::Foundation::BSTR) -> ::windows::core::Result<ISpeechObjectTokens>;
    fn GetProfiles(&mut self, requiredattributes: &super::super::Foundation::BSTR, optionalattributes: &super::super::Foundation::BSTR) -> ::windows::core::Result<ISpeechObjectTokens>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechRecognizer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecognizer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechRecognizer_Vtbl {
        unsafe extern "system" fn putref_Recognizer<Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recognizer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_Recognizer(::core::mem::transmute(&recognizer)).into()
        }
        unsafe extern "system" fn Recognizer<Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recognizer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Recognizer() {
                ::core::result::Result::Ok(ok__) => {
                    *recognizer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowAudioInputFormatChangesOnNextSet<Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowAudioInputFormatChangesOnNextSet(::core::mem::transmute_copy(&allow)).into()
        }
        unsafe extern "system" fn AllowAudioInputFormatChangesOnNextSet<Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowAudioInputFormatChangesOnNextSet() {
                ::core::result::Result::Ok(ok__) => {
                    *allow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AudioInput<Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audioinput: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_AudioInput(::core::mem::transmute(&audioinput)).into()
        }
        unsafe extern "system" fn AudioInput<Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audioinput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioInput() {
                ::core::result::Result::Ok(ok__) => {
                    *audioinput = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AudioInputStream<Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audioinputstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_AudioInputStream(::core::mem::transmute(&audioinputstream)).into()
        }
        unsafe extern "system" fn AudioInputStream<Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audioinputstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioInputStream() {
                ::core::result::Result::Ok(ok__) => {
                    *audioinputstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsShared<Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shared: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsShared() {
                ::core::result::Result::Ok(ok__) => {
                    *shared = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: SpeechRecognizerState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetState(::core::mem::transmute_copy(&state)).into()
        }
        unsafe extern "system" fn State<Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: *mut SpeechRecognizerState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *state = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *status = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Profile<Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profile: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_Profile(::core::mem::transmute(&profile)).into()
        }
        unsafe extern "system" fn Profile<Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Profile() {
                ::core::result::Result::Ok(ok__) => {
                    *profile = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EmulateRecognition<Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textelements: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, elementdisplayattributes: *const super::super::System::Com::VARIANT, languageid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EmulateRecognition(::core::mem::transmute_copy(&textelements), ::core::mem::transmute_copy(&elementdisplayattributes), ::core::mem::transmute_copy(&languageid)).into()
        }
        unsafe extern "system" fn CreateRecoContext<Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRecoContext() {
                ::core::result::Result::Ok(ok__) => {
                    *newcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormat<Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: SpeechFormatType, format: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFormat(::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *format = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPropertyNumber<Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: i32, supported: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPropertyNumber(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *supported = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyNumber<Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: *mut i32, supported: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPropertyNumber(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&supported)).into()
        }
        unsafe extern "system" fn SetPropertyString<Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, supported: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPropertyString(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *supported = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyString<Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: *mut super::super::Foundation::BSTR, supported: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPropertyString(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&supported)).into()
        }
        unsafe extern "system" fn IsUISupported<Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, typeofui: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, extradata: *const super::super::System::Com::VARIANT, supported: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsUISupported(::core::mem::transmute_copy(&typeofui), ::core::mem::transmute_copy(&extradata)) {
                ::core::result::Result::Ok(ok__) => {
                    *supported = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayUI<Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: i32, title: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, typeofui: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, extradata: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisplayUI(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&title), ::core::mem::transmute_copy(&typeofui), ::core::mem::transmute_copy(&extradata)).into()
        }
        unsafe extern "system" fn GetRecognizers<Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requiredattributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, optionalattributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, objecttokens: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRecognizers(::core::mem::transmute_copy(&requiredattributes), ::core::mem::transmute_copy(&optionalattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *objecttokens = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAudioInputs<Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requiredattributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, optionalattributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, objecttokens: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAudioInputs(::core::mem::transmute_copy(&requiredattributes), ::core::mem::transmute_copy(&optionalattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *objecttokens = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProfiles<Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requiredattributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, optionalattributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, objecttokens: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProfiles(::core::mem::transmute_copy(&requiredattributes), ::core::mem::transmute_copy(&optionalattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *objecttokens = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            putref_Recognizer: putref_Recognizer::<Impl, IMPL_OFFSET>,
            Recognizer: Recognizer::<Impl, IMPL_OFFSET>,
            SetAllowAudioInputFormatChangesOnNextSet: SetAllowAudioInputFormatChangesOnNextSet::<Impl, IMPL_OFFSET>,
            AllowAudioInputFormatChangesOnNextSet: AllowAudioInputFormatChangesOnNextSet::<Impl, IMPL_OFFSET>,
            putref_AudioInput: putref_AudioInput::<Impl, IMPL_OFFSET>,
            AudioInput: AudioInput::<Impl, IMPL_OFFSET>,
            putref_AudioInputStream: putref_AudioInputStream::<Impl, IMPL_OFFSET>,
            AudioInputStream: AudioInputStream::<Impl, IMPL_OFFSET>,
            IsShared: IsShared::<Impl, IMPL_OFFSET>,
            SetState: SetState::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
            putref_Profile: putref_Profile::<Impl, IMPL_OFFSET>,
            Profile: Profile::<Impl, IMPL_OFFSET>,
            EmulateRecognition: EmulateRecognition::<Impl, IMPL_OFFSET>,
            CreateRecoContext: CreateRecoContext::<Impl, IMPL_OFFSET>,
            GetFormat: GetFormat::<Impl, IMPL_OFFSET>,
            SetPropertyNumber: SetPropertyNumber::<Impl, IMPL_OFFSET>,
            GetPropertyNumber: GetPropertyNumber::<Impl, IMPL_OFFSET>,
            SetPropertyString: SetPropertyString::<Impl, IMPL_OFFSET>,
            GetPropertyString: GetPropertyString::<Impl, IMPL_OFFSET>,
            IsUISupported: IsUISupported::<Impl, IMPL_OFFSET>,
            DisplayUI: DisplayUI::<Impl, IMPL_OFFSET>,
            GetRecognizers: GetRecognizers::<Impl, IMPL_OFFSET>,
            GetAudioInputs: GetAudioInputs::<Impl, IMPL_OFFSET>,
            GetProfiles: GetProfiles::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechRecognizer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechRecognizerStatus_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn AudioStatus(&mut self) -> ::windows::core::Result<ISpeechAudioStatus>;
    fn CurrentStreamPosition(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn CurrentStreamNumber(&mut self) -> ::windows::core::Result<i32>;
    fn NumberOfActiveRules(&mut self) -> ::windows::core::Result<i32>;
    fn ClsidEngine(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SupportedLanguages(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechRecognizerStatus_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecognizerStatus_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechRecognizerStatus_Vtbl {
        unsafe extern "system" fn AudioStatus<Impl: ISpeechRecognizerStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiostatus: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *audiostatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentStreamPosition<Impl: ISpeechRecognizerStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcurrentstreampos: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentStreamPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *pcurrentstreampos = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentStreamNumber<Impl: ISpeechRecognizerStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamnumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentStreamNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *streamnumber = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfActiveRules<Impl: ISpeechRecognizerStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numberofactiverules: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NumberOfActiveRules() {
                ::core::result::Result::Ok(ok__) => {
                    *numberofactiverules = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClsidEngine<Impl: ISpeechRecognizerStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsidengine: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClsidEngine() {
                ::core::result::Result::Ok(ok__) => {
                    *clsidengine = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedLanguages<Impl: ISpeechRecognizerStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedlanguages: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedLanguages() {
                ::core::result::Result::Ok(ok__) => {
                    *supportedlanguages = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AudioStatus: AudioStatus::<Impl, IMPL_OFFSET>,
            CurrentStreamPosition: CurrentStreamPosition::<Impl, IMPL_OFFSET>,
            CurrentStreamNumber: CurrentStreamNumber::<Impl, IMPL_OFFSET>,
            NumberOfActiveRules: NumberOfActiveRules::<Impl, IMPL_OFFSET>,
            ClsidEngine: ClsidEngine::<Impl, IMPL_OFFSET>,
            SupportedLanguages: SupportedLanguages::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechRecognizerStatus as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechResourceLoader_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn LoadResource(&mut self, bstrresourceuri: &super::super::Foundation::BSTR, falwaysreload: i16, pstream: *mut ::core::option::Option<::windows::core::IUnknown>, pbstrmimetype: *mut super::super::Foundation::BSTR, pfmodified: *mut i16, pbstrredirecturl: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetLocalCopy(&mut self, bstrresourceuri: &super::super::Foundation::BSTR, pbstrlocalpath: *mut super::super::Foundation::BSTR, pbstrmimetype: *mut super::super::Foundation::BSTR, pbstrredirecturl: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ReleaseLocalCopy(&mut self, pbstrlocalpath: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechResourceLoader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechResourceLoader_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechResourceLoader_Vtbl {
        unsafe extern "system" fn LoadResource<Impl: ISpeechResourceLoader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrresourceuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, falwaysreload: i16, pstream: *mut *mut ::core::ffi::c_void, pbstrmimetype: *mut super::super::Foundation::BSTR, pfmodified: *mut i16, pbstrredirecturl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LoadResource(::core::mem::transmute_copy(&bstrresourceuri), ::core::mem::transmute_copy(&falwaysreload), ::core::mem::transmute_copy(&pstream), ::core::mem::transmute_copy(&pbstrmimetype), ::core::mem::transmute_copy(&pfmodified), ::core::mem::transmute_copy(&pbstrredirecturl)).into()
        }
        unsafe extern "system" fn GetLocalCopy<Impl: ISpeechResourceLoader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrresourceuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrlocalpath: *mut super::super::Foundation::BSTR, pbstrmimetype: *mut super::super::Foundation::BSTR, pbstrredirecturl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLocalCopy(::core::mem::transmute_copy(&bstrresourceuri), ::core::mem::transmute_copy(&pbstrlocalpath), ::core::mem::transmute_copy(&pbstrmimetype), ::core::mem::transmute_copy(&pbstrredirecturl)).into()
        }
        unsafe extern "system" fn ReleaseLocalCopy<Impl: ISpeechResourceLoader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlocalpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseLocalCopy(::core::mem::transmute_copy(&pbstrlocalpath)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            LoadResource: LoadResource::<Impl, IMPL_OFFSET>,
            GetLocalCopy: GetLocalCopy::<Impl, IMPL_OFFSET>,
            ReleaseLocalCopy: ReleaseLocalCopy::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechResourceLoader as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechTextSelectionInformation_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SetActiveOffset(&mut self, activeoffset: i32) -> ::windows::core::Result<()>;
    fn ActiveOffset(&mut self) -> ::windows::core::Result<i32>;
    fn SetActiveLength(&mut self, activelength: i32) -> ::windows::core::Result<()>;
    fn ActiveLength(&mut self) -> ::windows::core::Result<i32>;
    fn SetSelectionOffset(&mut self, selectionoffset: i32) -> ::windows::core::Result<()>;
    fn SelectionOffset(&mut self) -> ::windows::core::Result<i32>;
    fn SetSelectionLength(&mut self, selectionlength: i32) -> ::windows::core::Result<()>;
    fn SelectionLength(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechTextSelectionInformation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechTextSelectionInformation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechTextSelectionInformation_Vtbl {
        unsafe extern "system" fn SetActiveOffset<Impl: ISpeechTextSelectionInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activeoffset: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetActiveOffset(::core::mem::transmute_copy(&activeoffset)).into()
        }
        unsafe extern "system" fn ActiveOffset<Impl: ISpeechTextSelectionInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activeoffset: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActiveOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *activeoffset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActiveLength<Impl: ISpeechTextSelectionInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activelength: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetActiveLength(::core::mem::transmute_copy(&activelength)).into()
        }
        unsafe extern "system" fn ActiveLength<Impl: ISpeechTextSelectionInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activelength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActiveLength() {
                ::core::result::Result::Ok(ok__) => {
                    *activelength = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectionOffset<Impl: ISpeechTextSelectionInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selectionoffset: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectionOffset(::core::mem::transmute_copy(&selectionoffset)).into()
        }
        unsafe extern "system" fn SelectionOffset<Impl: ISpeechTextSelectionInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selectionoffset: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectionOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *selectionoffset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectionLength<Impl: ISpeechTextSelectionInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selectionlength: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectionLength(::core::mem::transmute_copy(&selectionlength)).into()
        }
        unsafe extern "system" fn SelectionLength<Impl: ISpeechTextSelectionInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selectionlength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectionLength() {
                ::core::result::Result::Ok(ok__) => {
                    *selectionlength = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetActiveOffset: SetActiveOffset::<Impl, IMPL_OFFSET>,
            ActiveOffset: ActiveOffset::<Impl, IMPL_OFFSET>,
            SetActiveLength: SetActiveLength::<Impl, IMPL_OFFSET>,
            ActiveLength: ActiveLength::<Impl, IMPL_OFFSET>,
            SetSelectionOffset: SetSelectionOffset::<Impl, IMPL_OFFSET>,
            SelectionOffset: SelectionOffset::<Impl, IMPL_OFFSET>,
            SetSelectionLength: SetSelectionLength::<Impl, IMPL_OFFSET>,
            SelectionLength: SelectionLength::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechTextSelectionInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechVoice_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Status(&mut self) -> ::windows::core::Result<ISpeechVoiceStatus>;
    fn Voice(&mut self) -> ::windows::core::Result<ISpeechObjectToken>;
    fn putref_Voice(&mut self, voice: &::core::option::Option<ISpeechObjectToken>) -> ::windows::core::Result<()>;
    fn AudioOutput(&mut self) -> ::windows::core::Result<ISpeechObjectToken>;
    fn putref_AudioOutput(&mut self, audiooutput: &::core::option::Option<ISpeechObjectToken>) -> ::windows::core::Result<()>;
    fn AudioOutputStream(&mut self) -> ::windows::core::Result<ISpeechBaseStream>;
    fn putref_AudioOutputStream(&mut self, audiooutputstream: &::core::option::Option<ISpeechBaseStream>) -> ::windows::core::Result<()>;
    fn Rate(&mut self) -> ::windows::core::Result<i32>;
    fn SetRate(&mut self, rate: i32) -> ::windows::core::Result<()>;
    fn Volume(&mut self) -> ::windows::core::Result<i32>;
    fn SetVolume(&mut self, volume: i32) -> ::windows::core::Result<()>;
    fn SetAllowAudioOutputFormatChangesOnNextSet(&mut self, allow: i16) -> ::windows::core::Result<()>;
    fn AllowAudioOutputFormatChangesOnNextSet(&mut self) -> ::windows::core::Result<i16>;
    fn EventInterests(&mut self) -> ::windows::core::Result<SpeechVoiceEvents>;
    fn SetEventInterests(&mut self, eventinterestflags: SpeechVoiceEvents) -> ::windows::core::Result<()>;
    fn SetPriority(&mut self, priority: SpeechVoicePriority) -> ::windows::core::Result<()>;
    fn Priority(&mut self) -> ::windows::core::Result<SpeechVoicePriority>;
    fn SetAlertBoundary(&mut self, boundary: SpeechVoiceEvents) -> ::windows::core::Result<()>;
    fn AlertBoundary(&mut self) -> ::windows::core::Result<SpeechVoiceEvents>;
    fn SetSynchronousSpeakTimeout(&mut self, mstimeout: i32) -> ::windows::core::Result<()>;
    fn SynchronousSpeakTimeout(&mut self) -> ::windows::core::Result<i32>;
    fn Speak(&mut self, text: &super::super::Foundation::BSTR, flags: SpeechVoiceSpeakFlags) -> ::windows::core::Result<i32>;
    fn SpeakStream(&mut self, stream: &::core::option::Option<ISpeechBaseStream>, flags: SpeechVoiceSpeakFlags) -> ::windows::core::Result<i32>;
    fn Pause(&mut self) -> ::windows::core::Result<()>;
    fn Resume(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, r#type: &super::super::Foundation::BSTR, numitems: i32) -> ::windows::core::Result<i32>;
    fn GetVoices(&mut self, requiredattributes: &super::super::Foundation::BSTR, optionalattributes: &super::super::Foundation::BSTR) -> ::windows::core::Result<ISpeechObjectTokens>;
    fn GetAudioOutputs(&mut self, requiredattributes: &super::super::Foundation::BSTR, optionalattributes: &super::super::Foundation::BSTR) -> ::windows::core::Result<ISpeechObjectTokens>;
    fn WaitUntilDone(&mut self, mstimeout: i32) -> ::windows::core::Result<i16>;
    fn SpeakCompleteEvent(&mut self) -> ::windows::core::Result<i32>;
    fn IsUISupported(&mut self, typeofui: &super::super::Foundation::BSTR, extradata: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<i16>;
    fn DisplayUI(&mut self, hwndparent: i32, title: &super::super::Foundation::BSTR, typeofui: &super::super::Foundation::BSTR, extradata: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechVoice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechVoice_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechVoice_Vtbl {
        unsafe extern "system" fn Status<Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *status = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Voice<Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, voice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Voice() {
                ::core::result::Result::Ok(ok__) => {
                    *voice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Voice<Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, voice: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_Voice(::core::mem::transmute(&voice)).into()
        }
        unsafe extern "system" fn AudioOutput<Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiooutput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioOutput() {
                ::core::result::Result::Ok(ok__) => {
                    *audiooutput = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AudioOutput<Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiooutput: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_AudioOutput(::core::mem::transmute(&audiooutput)).into()
        }
        unsafe extern "system" fn AudioOutputStream<Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiooutputstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioOutputStream() {
                ::core::result::Result::Ok(ok__) => {
                    *audiooutputstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AudioOutputStream<Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiooutputstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_AudioOutputStream(::core::mem::transmute(&audiooutputstream)).into()
        }
        unsafe extern "system" fn Rate<Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rate() {
                ::core::result::Result::Ok(ok__) => {
                    *rate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRate<Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rate: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRate(::core::mem::transmute_copy(&rate)).into()
        }
        unsafe extern "system" fn Volume<Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, volume: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Volume() {
                ::core::result::Result::Ok(ok__) => {
                    *volume = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVolume<Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, volume: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVolume(::core::mem::transmute_copy(&volume)).into()
        }
        unsafe extern "system" fn SetAllowAudioOutputFormatChangesOnNextSet<Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowAudioOutputFormatChangesOnNextSet(::core::mem::transmute_copy(&allow)).into()
        }
        unsafe extern "system" fn AllowAudioOutputFormatChangesOnNextSet<Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowAudioOutputFormatChangesOnNextSet() {
                ::core::result::Result::Ok(ok__) => {
                    *allow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventInterests<Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventinterestflags: *mut SpeechVoiceEvents) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventInterests() {
                ::core::result::Result::Ok(ok__) => {
                    *eventinterestflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventInterests<Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventinterestflags: SpeechVoiceEvents) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEventInterests(::core::mem::transmute_copy(&eventinterestflags)).into()
        }
        unsafe extern "system" fn SetPriority<Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, priority: SpeechVoicePriority) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPriority(::core::mem::transmute_copy(&priority)).into()
        }
        unsafe extern "system" fn Priority<Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, priority: *mut SpeechVoicePriority) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Priority() {
                ::core::result::Result::Ok(ok__) => {
                    *priority = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlertBoundary<Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, boundary: SpeechVoiceEvents) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlertBoundary(::core::mem::transmute_copy(&boundary)).into()
        }
        unsafe extern "system" fn AlertBoundary<Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, boundary: *mut SpeechVoiceEvents) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlertBoundary() {
                ::core::result::Result::Ok(ok__) => {
                    *boundary = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSynchronousSpeakTimeout<Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mstimeout: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSynchronousSpeakTimeout(::core::mem::transmute_copy(&mstimeout)).into()
        }
        unsafe extern "system" fn SynchronousSpeakTimeout<Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mstimeout: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SynchronousSpeakTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    *mstimeout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Speak<Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: SpeechVoiceSpeakFlags, streamnumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Speak(::core::mem::transmute_copy(&text), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *streamnumber = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpeakStream<Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, flags: SpeechVoiceSpeakFlags, streamnumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SpeakStream(::core::mem::transmute(&stream), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *streamnumber = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pause<Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn Resume<Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Resume().into()
        }
        unsafe extern "system" fn Skip<Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, numitems: i32, numskipped: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&numitems)) {
                ::core::result::Result::Ok(ok__) => {
                    *numskipped = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVoices<Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requiredattributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, optionalattributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, objecttokens: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVoices(::core::mem::transmute_copy(&requiredattributes), ::core::mem::transmute_copy(&optionalattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *objecttokens = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAudioOutputs<Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requiredattributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, optionalattributes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, objecttokens: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAudioOutputs(::core::mem::transmute_copy(&requiredattributes), ::core::mem::transmute_copy(&optionalattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *objecttokens = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WaitUntilDone<Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mstimeout: i32, done: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WaitUntilDone(::core::mem::transmute_copy(&mstimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *done = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpeakCompleteEvent<Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handle: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SpeakCompleteEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *handle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUISupported<Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, typeofui: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, extradata: *const super::super::System::Com::VARIANT, supported: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsUISupported(::core::mem::transmute_copy(&typeofui), ::core::mem::transmute_copy(&extradata)) {
                ::core::result::Result::Ok(ok__) => {
                    *supported = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayUI<Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: i32, title: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, typeofui: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, extradata: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisplayUI(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&title), ::core::mem::transmute_copy(&typeofui), ::core::mem::transmute_copy(&extradata)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            Voice: Voice::<Impl, IMPL_OFFSET>,
            putref_Voice: putref_Voice::<Impl, IMPL_OFFSET>,
            AudioOutput: AudioOutput::<Impl, IMPL_OFFSET>,
            putref_AudioOutput: putref_AudioOutput::<Impl, IMPL_OFFSET>,
            AudioOutputStream: AudioOutputStream::<Impl, IMPL_OFFSET>,
            putref_AudioOutputStream: putref_AudioOutputStream::<Impl, IMPL_OFFSET>,
            Rate: Rate::<Impl, IMPL_OFFSET>,
            SetRate: SetRate::<Impl, IMPL_OFFSET>,
            Volume: Volume::<Impl, IMPL_OFFSET>,
            SetVolume: SetVolume::<Impl, IMPL_OFFSET>,
            SetAllowAudioOutputFormatChangesOnNextSet: SetAllowAudioOutputFormatChangesOnNextSet::<Impl, IMPL_OFFSET>,
            AllowAudioOutputFormatChangesOnNextSet: AllowAudioOutputFormatChangesOnNextSet::<Impl, IMPL_OFFSET>,
            EventInterests: EventInterests::<Impl, IMPL_OFFSET>,
            SetEventInterests: SetEventInterests::<Impl, IMPL_OFFSET>,
            SetPriority: SetPriority::<Impl, IMPL_OFFSET>,
            Priority: Priority::<Impl, IMPL_OFFSET>,
            SetAlertBoundary: SetAlertBoundary::<Impl, IMPL_OFFSET>,
            AlertBoundary: AlertBoundary::<Impl, IMPL_OFFSET>,
            SetSynchronousSpeakTimeout: SetSynchronousSpeakTimeout::<Impl, IMPL_OFFSET>,
            SynchronousSpeakTimeout: SynchronousSpeakTimeout::<Impl, IMPL_OFFSET>,
            Speak: Speak::<Impl, IMPL_OFFSET>,
            SpeakStream: SpeakStream::<Impl, IMPL_OFFSET>,
            Pause: Pause::<Impl, IMPL_OFFSET>,
            Resume: Resume::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            GetVoices: GetVoices::<Impl, IMPL_OFFSET>,
            GetAudioOutputs: GetAudioOutputs::<Impl, IMPL_OFFSET>,
            WaitUntilDone: WaitUntilDone::<Impl, IMPL_OFFSET>,
            SpeakCompleteEvent: SpeakCompleteEvent::<Impl, IMPL_OFFSET>,
            IsUISupported: IsUISupported::<Impl, IMPL_OFFSET>,
            DisplayUI: DisplayUI::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechVoice as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechVoiceStatus_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CurrentStreamNumber(&mut self) -> ::windows::core::Result<i32>;
    fn LastStreamNumberQueued(&mut self) -> ::windows::core::Result<i32>;
    fn LastHResult(&mut self) -> ::windows::core::Result<i32>;
    fn RunningState(&mut self) -> ::windows::core::Result<SpeechRunState>;
    fn InputWordPosition(&mut self) -> ::windows::core::Result<i32>;
    fn InputWordLength(&mut self) -> ::windows::core::Result<i32>;
    fn InputSentencePosition(&mut self) -> ::windows::core::Result<i32>;
    fn InputSentenceLength(&mut self) -> ::windows::core::Result<i32>;
    fn LastBookmark(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn LastBookmarkId(&mut self) -> ::windows::core::Result<i32>;
    fn PhonemeId(&mut self) -> ::windows::core::Result<i16>;
    fn VisemeId(&mut self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechVoiceStatus_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechVoiceStatus_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechVoiceStatus_Vtbl {
        unsafe extern "system" fn CurrentStreamNumber<Impl: ISpeechVoiceStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamnumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentStreamNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *streamnumber = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastStreamNumberQueued<Impl: ISpeechVoiceStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamnumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastStreamNumberQueued() {
                ::core::result::Result::Ok(ok__) => {
                    *streamnumber = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastHResult<Impl: ISpeechVoiceStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresult: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastHResult() {
                ::core::result::Result::Ok(ok__) => {
                    *hresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RunningState<Impl: ISpeechVoiceStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: *mut SpeechRunState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RunningState() {
                ::core::result::Result::Ok(ok__) => {
                    *state = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InputWordPosition<Impl: ISpeechVoiceStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InputWordPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *position = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InputWordLength<Impl: ISpeechVoiceStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InputWordLength() {
                ::core::result::Result::Ok(ok__) => {
                    *length = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InputSentencePosition<Impl: ISpeechVoiceStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InputSentencePosition() {
                ::core::result::Result::Ok(ok__) => {
                    *position = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InputSentenceLength<Impl: ISpeechVoiceStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InputSentenceLength() {
                ::core::result::Result::Ok(ok__) => {
                    *length = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastBookmark<Impl: ISpeechVoiceStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bookmark: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastBookmark() {
                ::core::result::Result::Ok(ok__) => {
                    *bookmark = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastBookmarkId<Impl: ISpeechVoiceStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bookmarkid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastBookmarkId() {
                ::core::result::Result::Ok(ok__) => {
                    *bookmarkid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhonemeId<Impl: ISpeechVoiceStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phoneid: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhonemeId() {
                ::core::result::Result::Ok(ok__) => {
                    *phoneid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VisemeId<Impl: ISpeechVoiceStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visemeid: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VisemeId() {
                ::core::result::Result::Ok(ok__) => {
                    *visemeid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CurrentStreamNumber: CurrentStreamNumber::<Impl, IMPL_OFFSET>,
            LastStreamNumberQueued: LastStreamNumberQueued::<Impl, IMPL_OFFSET>,
            LastHResult: LastHResult::<Impl, IMPL_OFFSET>,
            RunningState: RunningState::<Impl, IMPL_OFFSET>,
            InputWordPosition: InputWordPosition::<Impl, IMPL_OFFSET>,
            InputWordLength: InputWordLength::<Impl, IMPL_OFFSET>,
            InputSentencePosition: InputSentencePosition::<Impl, IMPL_OFFSET>,
            InputSentenceLength: InputSentenceLength::<Impl, IMPL_OFFSET>,
            LastBookmark: LastBookmark::<Impl, IMPL_OFFSET>,
            LastBookmarkId: LastBookmarkId::<Impl, IMPL_OFFSET>,
            PhonemeId: PhonemeId::<Impl, IMPL_OFFSET>,
            VisemeId: VisemeId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechVoiceStatus as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechWaveFormatEx_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn FormatTag(&mut self) -> ::windows::core::Result<i16>;
    fn SetFormatTag(&mut self, formattag: i16) -> ::windows::core::Result<()>;
    fn Channels(&mut self) -> ::windows::core::Result<i16>;
    fn SetChannels(&mut self, channels: i16) -> ::windows::core::Result<()>;
    fn SamplesPerSec(&mut self) -> ::windows::core::Result<i32>;
    fn SetSamplesPerSec(&mut self, samplespersec: i32) -> ::windows::core::Result<()>;
    fn AvgBytesPerSec(&mut self) -> ::windows::core::Result<i32>;
    fn SetAvgBytesPerSec(&mut self, avgbytespersec: i32) -> ::windows::core::Result<()>;
    fn BlockAlign(&mut self) -> ::windows::core::Result<i16>;
    fn SetBlockAlign(&mut self, blockalign: i16) -> ::windows::core::Result<()>;
    fn BitsPerSample(&mut self) -> ::windows::core::Result<i16>;
    fn SetBitsPerSample(&mut self, bitspersample: i16) -> ::windows::core::Result<()>;
    fn ExtraData(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetExtraData(&mut self, extradata: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechWaveFormatEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechWaveFormatEx_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechWaveFormatEx_Vtbl {
        unsafe extern "system" fn FormatTag<Impl: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, formattag: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FormatTag() {
                ::core::result::Result::Ok(ok__) => {
                    *formattag = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormatTag<Impl: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, formattag: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFormatTag(::core::mem::transmute_copy(&formattag)).into()
        }
        unsafe extern "system" fn Channels<Impl: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channels: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Channels() {
                ::core::result::Result::Ok(ok__) => {
                    *channels = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChannels<Impl: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channels: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetChannels(::core::mem::transmute_copy(&channels)).into()
        }
        unsafe extern "system" fn SamplesPerSec<Impl: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, samplespersec: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SamplesPerSec() {
                ::core::result::Result::Ok(ok__) => {
                    *samplespersec = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSamplesPerSec<Impl: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, samplespersec: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSamplesPerSec(::core::mem::transmute_copy(&samplespersec)).into()
        }
        unsafe extern "system" fn AvgBytesPerSec<Impl: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, avgbytespersec: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AvgBytesPerSec() {
                ::core::result::Result::Ok(ok__) => {
                    *avgbytespersec = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAvgBytesPerSec<Impl: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, avgbytespersec: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAvgBytesPerSec(::core::mem::transmute_copy(&avgbytespersec)).into()
        }
        unsafe extern "system" fn BlockAlign<Impl: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blockalign: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BlockAlign() {
                ::core::result::Result::Ok(ok__) => {
                    *blockalign = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBlockAlign<Impl: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blockalign: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBlockAlign(::core::mem::transmute_copy(&blockalign)).into()
        }
        unsafe extern "system" fn BitsPerSample<Impl: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitspersample: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BitsPerSample() {
                ::core::result::Result::Ok(ok__) => {
                    *bitspersample = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBitsPerSample<Impl: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitspersample: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBitsPerSample(::core::mem::transmute_copy(&bitspersample)).into()
        }
        unsafe extern "system" fn ExtraData<Impl: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extradata: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtraData() {
                ::core::result::Result::Ok(ok__) => {
                    *extradata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtraData<Impl: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extradata: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExtraData(::core::mem::transmute_copy(&extradata)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            FormatTag: FormatTag::<Impl, IMPL_OFFSET>,
            SetFormatTag: SetFormatTag::<Impl, IMPL_OFFSET>,
            Channels: Channels::<Impl, IMPL_OFFSET>,
            SetChannels: SetChannels::<Impl, IMPL_OFFSET>,
            SamplesPerSec: SamplesPerSec::<Impl, IMPL_OFFSET>,
            SetSamplesPerSec: SetSamplesPerSec::<Impl, IMPL_OFFSET>,
            AvgBytesPerSec: AvgBytesPerSec::<Impl, IMPL_OFFSET>,
            SetAvgBytesPerSec: SetAvgBytesPerSec::<Impl, IMPL_OFFSET>,
            BlockAlign: BlockAlign::<Impl, IMPL_OFFSET>,
            SetBlockAlign: SetBlockAlign::<Impl, IMPL_OFFSET>,
            BitsPerSample: BitsPerSample::<Impl, IMPL_OFFSET>,
            SetBitsPerSample: SetBitsPerSample::<Impl, IMPL_OFFSET>,
            ExtraData: ExtraData::<Impl, IMPL_OFFSET>,
            SetExtraData: SetExtraData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechWaveFormatEx as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISpeechXMLRecoResult_Impl: Sized + super::super::System::Com::IDispatch_Impl + ISpeechRecoResult_Impl {
    fn GetXMLResult(&mut self, options: SPXMLRESULTOPTIONS) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetXMLErrorInfo(&mut self, linenumber: *mut i32, scriptline: *mut super::super::Foundation::BSTR, source: *mut super::super::Foundation::BSTR, description: *mut super::super::Foundation::BSTR, resultcode: *mut i32, iserror: *mut i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISpeechXMLRecoResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechXMLRecoResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechXMLRecoResult_Vtbl {
        unsafe extern "system" fn GetXMLResult<Impl: ISpeechXMLRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: SPXMLRESULTOPTIONS, presult: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetXMLResult(::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    *presult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXMLErrorInfo<Impl: ISpeechXMLRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linenumber: *mut i32, scriptline: *mut super::super::Foundation::BSTR, source: *mut super::super::Foundation::BSTR, description: *mut super::super::Foundation::BSTR, resultcode: *mut i32, iserror: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetXMLErrorInfo(::core::mem::transmute_copy(&linenumber), ::core::mem::transmute_copy(&scriptline), ::core::mem::transmute_copy(&source), ::core::mem::transmute_copy(&description), ::core::mem::transmute_copy(&resultcode), ::core::mem::transmute_copy(&iserror)).into()
        }
        Self {
            base: ISpeechRecoResult_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetXMLResult: GetXMLResult::<Impl, IMPL_OFFSET>,
            GetXMLErrorInfo: GetXMLErrorInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechXMLRecoResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _ISpeechRecoContextEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _ISpeechRecoContextEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _ISpeechRecoContextEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> _ISpeechRecoContextEvents_Vtbl {
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_ISpeechRecoContextEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _ISpeechVoiceEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _ISpeechVoiceEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _ISpeechVoiceEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> _ISpeechVoiceEvents_Vtbl {
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_ISpeechVoiceEvents as ::windows::core::Interface>::IID
    }
}
